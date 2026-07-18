param([string]$Path)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = if ($Path) { $Path } else { Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
if (!(Test-Path $path)) { throw 'Worker batch state missing.' }
. (Join-Path $PSScriptRoot 'worker-handoff-integrity.ps1')
$state = Get-Content $path -Raw | ConvertFrom-Json
if ($state.schema_version -ne 3 -or $state.canonical_role -ne 'sole-active-work-checkpoint') { throw 'Unsupported worker batch-state schema or role.' }
$valid = @('ready','executing','checkpoint','verifying','recorded','complete','blocked')
if ($valid -notcontains $state.state) { throw 'Invalid worker batch state.' }
if ([string]::IsNullOrWhiteSpace($state.next_action)) { throw 'Worker batch lacks exact next action.' }
if ([string]::IsNullOrWhiteSpace($state.substage_id)) { throw 'Worker batch lacks exact substage.' }
$handoffSections = @(Get-WorkerHandoffSectionNames)
$receipts = $state.handoff_section_receipts
if ($null -eq $receipts) { throw 'Worker batch lacks handoff section integrity receipts.' }
if ($state.PSObject.Properties.Name -contains 'section_stage_ids') { throw 'Worker batch retains superseded section-stage bindings.' }
$receiptNames = @($receipts.PSObject.Properties.Name)
foreach ($section in $handoffSections) {
  if ($receiptNames -notcontains $section) { throw "Worker batch lacks handoff section receipt: $section" }
  $receipt = $receipts.$section
  if ([string]$receipt.stage_id -ne [string]$state.substage_id) {
    throw "Worker batch handoff receipt is stale relative to the active substage: $section"
  }
  if ([string]$receipt.content_sha256 -notmatch '^[0-9a-f]{64}$') {
    throw "Worker batch handoff receipt lacks a canonical content hash: $section"
  }
  $actualHash = Get-WorkerHandoffSectionHash -State $state -Section $section
  if ([string]$receipt.content_sha256 -ne $actualHash) {
    throw "Worker batch handoff section changed after review: $section"
  }
  if ($receipt.review_disposition -notin @('revised','carried_forward') -or [string]::IsNullOrWhiteSpace($receipt.review_note)) {
    throw "Worker batch handoff receipt lacks an explicit review disposition: $section"
  }
}
$unexpectedReceipts = @($receiptNames | Where-Object { $handoffSections -notcontains $_ })
if ($unexpectedReceipts.Count -gt 0) { throw "Worker batch contains unknown handoff section receipt: $($unexpectedReceipts[0])" }
foreach ($field in @('master_program_item','risk_level','research_gate','authority_lane','context_health')) {
  if ([string]::IsNullOrWhiteSpace($state.$field)) { throw "Worker batch lacks canonical field: $field" }
}
if ([string]::IsNullOrWhiteSpace($state.atlas_route.milestone) -or @($state.atlas_route.systems).Count -eq 0) { throw 'Worker batch lacks Atlas route.' }
if (@($state.evidence_requirements).Count -eq 0 -or @($state.verification_plan).Count -eq 0) { throw 'Worker batch lacks bounded evidence or verification plan.' }
if ($state.PSObject.Properties.Name -contains 'exit_criteria') {
  $criteria = @($state.exit_criteria)
  if ($criteria.Count -eq 0) { throw 'Structured exit criteria cannot be empty.' }
  $criterionIds = @{}
  foreach ($criterion in $criteria) {
    if ([string]::IsNullOrWhiteSpace($criterion.id) -or [string]::IsNullOrWhiteSpace($criterion.label)) {
      throw 'Structured exit criterion lacks an id or label.'
    }
    if ($criterionIds.ContainsKey([string]$criterion.id)) { throw "Duplicate structured exit criterion: $($criterion.id)" }
    $criterionIds[[string]$criterion.id] = $true
    if ($criterion.status -notin @('planned','in_progress','completed','verified','blocked')) {
      throw "Structured exit criterion has invalid status: $($criterion.id)"
    }
    if ($criterion.status -eq 'verified' -and @($criterion.evidence_ids).Count -eq 0) {
      throw "Verified exit criterion lacks evidence: $($criterion.id)"
    }
  }
  if ($state.state -in @('recorded','complete') -and @($criteria | Where-Object { $_.status -ne 'verified' }).Count -gt 0) {
    throw 'Completed worker batch retains an unverified structured exit criterion.'
  }
}
if ($null -eq $state.consecutive_no_progress_limit -or $state.consecutive_no_progress_limit -lt 1) { throw 'Worker batch lacks no-progress limit.' }
if ($state.state -in @('recorded','complete','blocked')) {
  if (@($state.evidence).Count -eq 0) { throw 'Terminal worker batch state lacks evidence.' }
}
if ($state.state -in @('recorded','complete')) {
  if (@($state.verification_receipts).Count -eq 0) { throw 'Completed worker batch lacks verification receipts.' }
  if ($null -eq $state.metrics) { throw 'Completed worker batch lacks metric integrity record.' }
  foreach ($field in @('token_cost','elapsed_time')) {
    $metric = $state.metrics.$field
    if ($null -eq $metric -or $metric.status -notin @('measured','unknown')) { throw "Worker metric lacks measured/unknown status: $field" }
    if ($metric.status -eq 'unknown' -and $null -ne $metric.value) { throw "Unknown worker metric has fabricated value: $field" }
  }
  if ([string]::IsNullOrWhiteSpace($state.transition)) { throw 'Completed worker batch lacks recorded transition.' }
}
foreach ($name in @('authority_grant','approved','promoted','protected_kernel_mutation')) {
  if ($state.PSObject.Properties.Name -contains $name) { throw "Worker batch contains forbidden authority field: $name" }
}
$legacy = @('context\active\WORK_PACKAGE.json','context\active\BATCH_STATE.md','context\active\WORKER_HANDOFF.md')
if (!$Path) { foreach ($relative in $legacy) { if (Test-Path (Join-Path $root $relative)) { throw "Redundant authored active-state file remains: $relative" } } }
$allowed = @{
  ready = @('executing','blocked')
  executing = @('checkpoint','verifying','blocked')
  checkpoint = @('executing','verifying','blocked')
  verifying = @('recorded','blocked')
  recorded = @('complete','blocked')
  complete = @()
  blocked = @()
}
if ($null -ne $state.previous_state -and $allowed[$state.previous_state] -notcontains $state.state) { throw 'Illegal worker batch-state transition.' }
& (Join-Path $PSScriptRoot 'verify-stage-quality-gates.ps1') -Path $path
if (!$?) { throw 'Stage-quality gates failed.' }
Write-Output "Worker batch state verified: $($state.state)."
