Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_POST_FIXED_INTERVAL_ARITHMETIC_CONSOLIDATION_REASSESSMENT.md'
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_READINESS.md'
$bulkCargoPath = Join-Path $root 'crates\visible-radiance-bulk-transfer\Cargo.toml'
$opticalCargoPath = Join-Path $root 'crates\visible-radiance-interface-event\Cargo.toml'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$checkpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'

$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'one-band interval bulk implementation readiness selected',
  'is not a prerequisite',
  '429-line private arithmetic module',
  'does not supply a missing interval-bulk prerequisite',
  'code-facing implementation-readiness audit',
  'does not authorize interval bulk source',
  'does not authorize interval bulk source, optical migration'
)) {
  if ($result -notlike "*$required*") { throw "Post-consolidation reassessment is missing: $required" }
}
if (!(Test-Path -LiteralPath $readinessPath)) { throw 'Selected interval bulk readiness record is missing.' }
$bulkCargo = Get-Content -LiteralPath $bulkCargoPath -Raw
$opticalCargo = Get-Content -LiteralPath $opticalCargoPath -Raw
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath $checkpointPath -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$implementationAuthorized = $checkpoint.batch_id -eq 'G1-C3-INTERVAL-BULK-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*owner explicitly approved the exact readiness package*' -and
  $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved additive one-band interval bulk package*'
$implementationCompleted = $c3.Count -eq 1 -and
  $c3[0].sources -contains 'G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_RESULT.md' -and
  $c3[0].sources -contains 'G1_C3_POST_INTERVAL_BULK_TRANSFER_CONSUMER_REASSESSMENT.md'
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
if ($bulkCargo -like '*fixed-interval-arithmetic*' -and -not ($implementationAuthorized -or $implementationCompleted -or $federatedContinuity)) { throw 'Interval bulk source dependency appeared before owner approval.' }
if ($opticalCargo -like '*fixed-interval-arithmetic*' -or $opticalCargo -notlike '*crypto-bigint*') {
  throw 'Optional optical migration occurred before a separate gate.'
}
Write-Output 'Post-consolidation reassessment verified: interval bulk readiness carries closure value, its exact owner gate is respected, and optional optical migration remains excluded.'
