Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_READINESS.md'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$checkpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$bulkSourcePath = Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs'

$readiness = Get-Content -LiteralPath $readinessPath -Raw
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath $checkpointPath -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1) { throw 'C3 master item is missing or duplicated.' }

foreach ($required in @(
  'implementation-ready behind one exact owner action',
  'ConditionalIntervalBulkQueryV1',
  'IntervalBulkLengthCertificateV1',
  'mindwarp.visible-radiance.interval-bulk-query.v1',
  'mindwarp.visible-radiance.interval-bulk-transfer.v1',
  '414 **magnitude** bits',
  'eight permanent existing V1 families',
  '64 KiB and 16 KiB',
  'i686-pc-windows-msvc',
  'aarch64-linux-android',
  'Rollback is deletion-only',
  'approval of this exact package is required'
)) {
  if ($readiness -notlike "*$required*") { throw "Interval bulk readiness is missing: $required" }
}

$ownerGate = $checkpoint.batch_id -eq 'G1-C3-INTERVAL-BULK-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.substage_id -eq 'interval-bulk-transfer-owner-gate' -and
  $checkpoint.state -eq 'checkpoint' -and
  $checkpoint.next_action -like '*explicit owner approval or rejection*'
$approvedImplementation = $checkpoint.batch_id -eq 'G1-C3-INTERVAL-BULK-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*owner explicitly approved the exact readiness package*' -and
  $checkpoint.authority_lane -like '*Optical migration*composition remain excluded*' -and
  $c3[0].next_action -like '*owner-approved additive one-band interval bulk package*' -and
  $c3[0].proof -like '*owner explicitly approved the exact package*'
$completedImplementation = $checkpoint.batch_id -in @('G1-C3-POST-INTERVAL-BULK-TRANSFER-CONSUMER-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COMPOSITION-DESIGN-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COUNTEREXAMPLE-ORACLE-V1','G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-READINESS-V1','G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1','G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1') -and
  ($c3[0].next_action -like '*bounded optical-lineage composition mathematical design reassessment*' -or $c3[0].next_action -like '*optical-lineage counterexample/oracle audit*' -or $c3[0].next_action -like '*G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*owner-approved additive optical-lineage-binding*' -or $c3[0].next_action -like '*code-free post-optical-lineage source/receiver prerequisite*' -or $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*' -or $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*optical phase-space provenance prerequisite*' -or $c3[0].next_action -like '*Hold the exact serious owner decision*64-bit immutable-origin cap*') -and
  $c3[0].sources -contains 'G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_RESULT.md'
$federatedContinuity = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not ($ownerGate -or $approvedImplementation -or $completedImplementation -or $federatedContinuity)) {
  throw 'Interval bulk readiness is not aligned to its owner gate or exact approved implementation batch.'
}
if (-not ($approvedImplementation -or $completedImplementation -or $federatedContinuity) -and
    ($c3[0].next_action -notlike '*G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_READINESS.md*' -or
     $c3[0].next_action -notlike '*eight bulk V1 compatibility families before source*')) {
  throw 'C3 does not retain the exact interval bulk owner action.'
}
$bulkSource = Get-Content -LiteralPath $bulkSourcePath -Raw
if ($ownerGate) {
  foreach ($forbidden in @('ConditionalIntervalBulkQueryV1','ConditionalIntervalBulkTransferV1')) {
    if ($bulkSource -like "*$forbidden*") { throw "Interval bulk implementation appeared before owner approval: $forbidden" }
  }
}
Write-Output 'Interval bulk implementation readiness verified: additive schema, dual certificate, 414-bit shield, V1 compatibility, platform gates, rollback and owner boundary are frozen or explicitly released.'
