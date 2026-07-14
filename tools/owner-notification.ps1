param(
  [ValidateSet('Notify','Acknowledge','Retry')][string]$Operation = 'Notify',
  [string]$EventPath,
  [Parameter(Mandatory=$true)][string]$StatePath,
  [Parameter(Mandatory=$true)][string]$OutboxPath,
  [bool]$Online = $true,
  [string]$Now,
  [string]$ProblemId,
  [int]$Revision
)
$ErrorActionPreference = 'Stop'
$timestamp = if ($Now) { [DateTimeOffset]::Parse($Now) } else { [DateTimeOffset]::UtcNow }

function Read-JsonState([string]$Path, [object]$Default) {
  if (!(Test-Path -LiteralPath $Path)) { return $Default }
  return Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
}
function Write-JsonState([string]$Path, [object]$Value) {
  $parent = Split-Path -Parent $Path
  if ($parent) { New-Item -ItemType Directory -Force -Path $parent | Out-Null }
  $temporary = "$Path.$PID.tmp"
  ConvertTo-Json -InputObject $Value -Depth 10 | Set-Content -LiteralPath $temporary
  Move-Item -LiteralPath $temporary -Destination $Path -Force
}
function Receipt([string]$Outcome, [string]$Reason, [object]$Event) {
  [ordered]@{
    schema_version = 1
    outcome = $Outcome
    reason = $Reason
    problem_id = if ($Event) { $Event.problem_id } else { $ProblemId }
    material_revision = if ($Event) { $Event.material_revision } else { $Revision }
    channel = 'task_chat'
    timestamp = $timestamp.ToString('o')
    authority_effect = 'none'
    protected_kernel_mutation = $false
  }
}

$state = Read-JsonState $StatePath ([pscustomobject]@{ schema_version=1; history=@(); acknowledgements=@() })
$outbox = @(Read-JsonState $OutboxPath @())
while ($outbox.Count -eq 1 -and $outbox[0] -is [System.Array]) { $outbox = @($outbox[0]) }
if ($state.schema_version -ne 1) { throw 'Unsupported owner-notification state schema.' }

if ($Operation -eq 'Acknowledge') {
  if ([string]::IsNullOrWhiteSpace($ProblemId) -or $Revision -lt 1) { throw 'Acknowledgement requires problem ID and positive revision.' }
  $acks = @($state.acknowledgements) + [pscustomobject]@{ problem_id=$ProblemId; material_revision=$Revision; timestamp=$timestamp.ToString('o') }
  $state.acknowledgements = $acks
  Write-JsonState $StatePath $state
  Receipt 'acknowledged' 'owner_acknowledgement_recorded_without_authority' $null | ConvertTo-Json -Depth 5
  exit
}

if ($Operation -eq 'Retry') {
  $receipts = @()
  if (!$Online) { (Receipt 'queued' 'transport_offline' $null) | ConvertTo-Json -Depth 5; exit }
  $remaining = @()
  foreach ($queued in $outbox) {
    $ack = @($state.acknowledgements | Where-Object { $_.problem_id -eq $queued.problem_id } | Sort-Object material_revision | Select-Object -Last 1)
    if ($ack.Count -gt 0 -and [int]$ack[0].material_revision -ge [int]$queued.material_revision) {
      $receipts += Receipt 'suppressed' 'acknowledged_before_retry' $queued
      continue
    }
    $windowStart = $timestamp.AddHours(-1)
    $recent = @($state.history | Where-Object { $_.status -eq 'delivered' -and [DateTimeOffset]::Parse($_.delivered_at) -ge $windowStart })
    $critical = $queued.severity -eq 'critical' -and $queued.category -in @('security_recovery_failure','authority_boundary')
    if ($recent.Count -ge 3 -and !$critical) {
      $remaining += $queued
      $receipts += Receipt 'queued' 'rate_limit_retry_deferred' $queued
      continue
    }
    $delivered = [pscustomobject]@{
      problem_id=$queued.problem_id; material_revision=[int]$queued.material_revision
      category=$queued.category; severity=$queued.severity; affected_package=$queued.affected_package
      evidence=$queued.evidence; attempted=$queued.attempted; consequence=$queued.consequence
      requested_action=$queued.requested_action; channel='task_chat'; queued_at=$queued.queued_at
      status='delivered'; delivered_at=$timestamp.ToString('o')
      authority_effect='none'; protected_kernel_mutation=$false
    }
    $state.history = @($state.history) + $delivered
    $receipts += Receipt 'delivered' 'offline_retry_succeeded' $delivered
  }
  Write-JsonState $OutboxPath $remaining
  Write-JsonState $StatePath $state
  $receipts | ConvertTo-Json -Depth 5
  exit
}

if ([string]::IsNullOrWhiteSpace($EventPath) -or !(Test-Path -LiteralPath $EventPath)) { throw 'Notification event missing.' }
$event = Get-Content -LiteralPath $EventPath -Raw | ConvertFrom-Json
foreach ($field in @('problem_id','category','severity','affected_package','evidence','attempted','consequence','requested_action','material_revision')) {
  if ($null -eq $event.$field -or ($event.$field -is [string] -and [string]::IsNullOrWhiteSpace($event.$field))) { throw "Notification event lacks required field: $field" }
}
if ($event.severity -notin @('low','medium','high','critical')) { throw 'Notification severity is invalid.' }
foreach ($field in @('authority_grant','approved','promoted','protected_kernel_mutation')) {
  if ($event.PSObject.Properties.Name -contains $field) { throw "Notification event contains forbidden authority field: $field" }
}
if ($event.requested_action -match '(?i)diagnos(e|is).*code') { throw 'Notification cannot ask the owner to diagnose code.' }
if ([int]$event.material_revision -lt 1) { throw 'Material revision must be positive.' }

$notifyCategories = @('owner_decision','design_gate','authority_boundary','security_recovery_failure','failed_verification','efficiency_escalation','blocked_package','dependency_change')
if ($notifyCategories -notcontains $event.category) {
  Receipt 'suppressed' 'non_actionable_category' $event | ConvertTo-Json -Depth 5
  exit
}
$ack = @($state.acknowledgements | Where-Object { $_.problem_id -eq $event.problem_id } | Sort-Object material_revision | Select-Object -Last 1)
if ($ack.Count -gt 0 -and [int]$ack[0].material_revision -ge [int]$event.material_revision) {
  Receipt 'suppressed' 'acknowledged_without_material_change' $event | ConvertTo-Json -Depth 5
  exit
}
$existing = @(@($state.history) + $outbox | Where-Object { $_.problem_id -eq $event.problem_id -and [int]$_.material_revision -eq [int]$event.material_revision })
if ($existing.Count -gt 0) {
  Receipt 'suppressed' 'duplicate_revision' $event | ConvertTo-Json -Depth 5
  exit
}
$newer = @(@($state.history) + $outbox | Where-Object { $_.problem_id -eq $event.problem_id -and [int]$_.material_revision -gt [int]$event.material_revision })
if ($newer.Count -gt 0) {
  Receipt 'suppressed' 'stale_revision' $event | ConvertTo-Json -Depth 5
  exit
}

$windowStart = $timestamp.AddHours(-1)
$recent = @($state.history | Where-Object { $_.status -eq 'delivered' -and [DateTimeOffset]::Parse($_.delivered_at) -ge $windowStart })
$critical = $event.severity -eq 'critical' -and $event.category -in @('security_recovery_failure','authority_boundary')
if ($recent.Count -ge 3 -and !$critical) {
  $envelope = [pscustomobject]@{
    problem_id=$event.problem_id; material_revision=[int]$event.material_revision
    category=$event.category; severity=$event.severity; affected_package=$event.affected_package
    evidence=$event.evidence; attempted=$event.attempted; consequence=$event.consequence
    requested_action=$event.requested_action; channel='task_chat'; queued_at=$timestamp.ToString('o')
    status='queued'; delivered_at=$null; authority_effect='none'; protected_kernel_mutation=$false
  }
  $outbox += $envelope
  Write-JsonState $OutboxPath $outbox
  Receipt 'queued' 'rate_limit' $event | ConvertTo-Json -Depth 5
  exit
}

$envelope = [pscustomobject]@{
  problem_id=$event.problem_id; material_revision=[int]$event.material_revision
  category=$event.category; severity=$event.severity; affected_package=$event.affected_package
  evidence=$event.evidence; attempted=$event.attempted; consequence=$event.consequence
  requested_action=$event.requested_action; channel='task_chat'; queued_at=$timestamp.ToString('o')
  status=if ($Online) { 'delivered' } else { 'queued' }
  delivered_at=if ($Online) { $timestamp.ToString('o') } else { $null }
  authority_effect='none'; protected_kernel_mutation=$false
}
if ($Online) { $state.history = @($state.history) + $envelope; Write-JsonState $StatePath $state }
else { $outbox += $envelope; Write-JsonState $OutboxPath $outbox }
$outcome = if ($Online) { 'delivered' } else { 'queued' }
$reason = if ($Online) { 'actionable_problem_routed' } else { 'transport_offline' }
Receipt $outcome $reason $event | ConvertTo-Json -Depth 5
