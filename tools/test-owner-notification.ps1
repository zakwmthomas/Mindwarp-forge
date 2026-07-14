$ErrorActionPreference = 'Stop'
$tool = Join-Path $PSScriptRoot 'owner-notification.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-owner-notification-$PID"
$state = Join-Path $temp 'state.json'; $outbox = Join-Path $temp 'outbox.json'; $eventPath = Join-Path $temp 'event.json'
function Event([string]$Id, [string]$Category, [int]$Revision, [string]$Severity='high') {
  [ordered]@{ problem_id=$Id; category=$Category; severity=$Severity; affected_package='W2'; evidence='evidence/receipt'; attempted='bounded repair'; consequence='work remains blocked'; requested_action='Acknowledge the recorded blocker.'; material_revision=$Revision }
}
function Notify([object]$Event, [bool]$Online=$true, [string]$Now='2026-07-13T04:00:00Z') {
  $Event | ConvertTo-Json | Set-Content $eventPath
  (& $tool -Operation Notify -EventPath $eventPath -StatePath $state -OutboxPath $outbox -Online $Online -Now $Now | ConvertFrom-Json)
}
try {
  New-Item -ItemType Directory -Force -Path $temp | Out-Null
  $normal = Notify (Event 'P0' 'normal_progress' 1)
  if ($normal.outcome -ne 'suppressed') { throw 'Normal progress was notified.' }
  $first = Notify (Event 'P1' 'failed_verification' 1)
  if ($first.outcome -ne 'delivered' -or $first.channel -ne 'task_chat') { throw 'Actionable problem was not routed to task chat.' }
  $duplicate = Notify (Event 'P1' 'failed_verification' 1)
  if ($duplicate.reason -ne 'duplicate_revision') { throw 'Duplicate revision was not suppressed.' }
  & $tool -Operation Acknowledge -StatePath $state -OutboxPath $outbox -ProblemId P1 -Revision 1 -Now '2026-07-13T04:01:00Z' | Out-Null
  $acknowledged = Notify (Event 'P1' 'failed_verification' 1)
  if ($acknowledged.reason -ne 'acknowledged_without_material_change') { throw 'Acknowledged unchanged problem was not suppressed.' }
  $changed = Notify (Event 'P1' 'failed_verification' 2)
  if ($changed.outcome -ne 'delivered') { throw 'Material change was not re-notified.' }
  $stale = Notify (Event 'P1' 'failed_verification' 1)
  if ($stale.outcome -ne 'suppressed') { throw 'Stale material revision was re-notified.' }
  $queued = Notify (Event 'P2' 'blocked_package' 1) $false
  if ($queued.outcome -ne 'queued') { throw 'Offline notification was not queued.' }
  $retried = @(& $tool -Operation Retry -StatePath $state -OutboxPath $outbox -Online $true -Now '2026-07-13T04:02:00Z' | ConvertFrom-Json)
  if ($retried.Count -ne 1 -or $retried[0].outcome -ne 'delivered') { throw 'Offline queue retry failed.' }
  $limited = Notify (Event 'P3' 'owner_decision' 1) $true '2026-07-13T04:03:00Z'
  if ($limited.reason -ne 'rate_limit' -or $limited.outcome -ne 'queued') { throw 'Rate-limited problem was not deferred safely.' }
  $critical = Notify (Event 'P4' 'security_recovery_failure' 1 'critical') $true '2026-07-13T04:04:00Z'
  if ($critical.outcome -ne 'delivered') { throw 'Critical security/recovery problem was hidden by rate limiting.' }
  & $tool -Operation Acknowledge -StatePath $state -OutboxPath $outbox -ProblemId P3 -Revision 1 -Now '2026-07-13T04:05:00Z' | Out-Null
  $ackRetry = @(& $tool -Operation Retry -StatePath $state -OutboxPath $outbox -Online $true -Now '2026-07-13T05:05:00Z' | ConvertFrom-Json)
  if ($ackRetry.Count -ne 1 -or $ackRetry[0].reason -ne 'acknowledged_before_retry') { throw 'Acknowledged queued notification was delivered again.' }
  $unsafe = Event 'P5' 'authority_boundary' 1
  $unsafe.authority_grant = $true
  $unsafe | ConvertTo-Json | Set-Content $eventPath
  $rejected = $false
  try { & $tool -Operation Notify -EventPath $eventPath -StatePath $state -OutboxPath $outbox | Out-Null } catch { $rejected = $true }
  if (!$rejected) { throw 'Authority-bearing notification was accepted.' }
  $history = Get-Content $state -Raw | ConvertFrom-Json
  if (@($history.history | Where-Object { $_.authority_effect -ne 'none' -or $_.protected_kernel_mutation }).Count -gt 0) { throw 'Notification mutated authority or protected Kernel state.' }
  Write-Output 'Owner notification fixtures verified: classification, routing, deduplication, acknowledgement, material change, offline retry, rate limiting, and authority rejection.'
} finally { if (Test-Path $temp) { Remove-Item -LiteralPath $temp -Recurse -Force } }
