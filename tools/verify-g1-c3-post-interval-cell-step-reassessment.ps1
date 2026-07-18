Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_INTERVAL_CELL_STEP_CONSUMER_REASSESSMENT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @('one-band conditional','must be one band','normalization cannot be assumed','copying a third implementation','authorizes only the selected mathematical/oracle audit')) {
  if ($audit -notlike "*$required*") { throw "Post-cell-step reassessment is missing: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1) { throw 'C3 master item is missing or duplicated.' }
$ownerGateRoute = $c3[0].next_action -like '*fixed-interval arithmetic*' -and
  $c3[0].next_action -like '*owner approval*' -and
  $checkpoint.batch_id -eq 'G1-C3-FIXED-INTERVAL-ARITHMETIC-CONSOLIDATION-OWNER-GATE-V1'
$postConsolidationRoute = $c3[0].next_action -like '*post-consolidation consumer reassessment*' -and
  $c3[0].next_action -like '*one-band interval bulk*' -and
  $c3[0].next_action -like '*optical arithmetic migration*' -and
  $checkpoint.batch_id -eq 'G1-C3-POST-FIXED-INTERVAL-ARITHMETIC-CONSOLIDATION-REASSESSMENT-V1' -and
  $checkpoint.next_action -like '*one-band interval bulk*' -and
  $checkpoint.next_action -like '*optical arithmetic migration*'
$intervalBulkOwnerGate = $c3[0].next_action -like '*G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_READINESS.md*' -and
  $c3[0].next_action -like '*owner approval*' -and
  $c3[0].proof -like '*one additive bulk-owned query and transfer*' -and
  $checkpoint.batch_id -eq 'G1-C3-INTERVAL-BULK-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.substage_id -eq 'interval-bulk-transfer-owner-gate' -and
  $checkpoint.authority_lane -like '*composition remain excluded*'
$intervalBulkImplementation = $c3[0].next_action -like '*owner-approved additive one-band interval bulk package*' -and
  $checkpoint.batch_id -eq 'G1-C3-INTERVAL-BULK-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*Optical migration*composition remain excluded*'
$postIntervalBulkRoute = ($c3[0].next_action -like '*bounded optical-lineage composition mathematical design reassessment*' -or $c3[0].next_action -like '*optical-lineage counterexample/oracle audit*') -and
  $checkpoint.batch_id -in @('G1-C3-POST-INTERVAL-BULK-TRANSFER-CONSUMER-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COMPOSITION-DESIGN-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COUNTEREXAMPLE-ORACLE-V1') -and
  $c3[0].sources -contains 'G1_C3_POST_INTERVAL_BULK_TRANSFER_CONSUMER_REASSESSMENT.md'
$lineageOwnerGate = $c3[0].next_action -like '*G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_READINESS.md*' -and
  $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.authority_lane -like '*requires explicit owner approval*'
$lineageImplementation = $c3[0].next_action -like '*owner-approved additive optical-lineage-binding*' -and
  $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*' -and
  $checkpoint.authority_lane -like '*cumulative power*local-owner changes*endpoint claims*C3 closure remain excluded*'
$postLineageRoute = $c3[0].next_action -like '*code-free post-optical-lineage source/receiver prerequisite*' -and
  $checkpoint.batch_id -eq 'G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1' -and
  $checkpoint.authority_lane -like '*No crate*numerical fold*arrival*visibility*C3 closure*'
$cumulativeOwnerRoute = $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*' -and
  $checkpoint.batch_id -eq 'G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.authority_lane -like '*requires explicit owner approval*receiver arrival*visibility*C3 closure*'
$cumulativeImplementationRoute = $c3[0].next_action -like '*owner-approved*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*optical-lane-transfer-binding*' -and
  $checkpoint.batch_id -eq 'G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*'
$receiverDesignRoute = $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.authority_lane -like '*Mathematical design and independent oracle only*No crate*schema*source*visibility*C3 closure*'
$receiverOwnerRoute = $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.authority_lane -like '*requires explicit owner approval*No crate*schema*source*visibility*C3 closure*'
$receiverImplementationRoute = $c3[0].next_action -like '*approved*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*receiver-arrival-geometry-binding*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$phaseSpaceReadinessRoute = $c3[0].next_action -like '*optical phase-space provenance prerequisite*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  ($checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -or
   $checkpoint.authority_lane -like '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*')
$transportOwnerRoute = $c3[0].next_action -like '*Hold the exact serious owner decision*64-bit immutable-origin cap*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -eq 'optical-phase-space-transport-origin-anchored-owner-gate' -and
  $checkpoint.authority_lane -like '*Owner decision only*add no transport crate*production source*visibility*runtime*promotion*C3 closure*'
$transportImplementationRoute = $c3[0].next_action -like '*owner-approved additive optical-phase-space-transport-certificate implementation*post-transport consumer reassessment*' -and
  $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*visibility*runtime*promotion*C3-closure authority*'
$federatedContinuityRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not ($ownerGateRoute -or $postConsolidationRoute -or $intervalBulkOwnerGate -or $intervalBulkImplementation -or $postIntervalBulkRoute -or $lineageOwnerGate -or $lineageImplementation -or $postLineageRoute -or $cumulativeOwnerRoute -or $cumulativeImplementationRoute -or $receiverDesignRoute -or $receiverOwnerRoute -or $receiverImplementationRoute -or $phaseSpaceReadinessRoute -or $transportOwnerRoute -or $transportImplementationRoute -or $federatedContinuityRoute)) {
  throw 'C3 is not routed through the fixed-arithmetic gate or its bounded post-consolidation reassessment.'
}
Write-Output 'Post-cell-step reassessment verified: one-band interval bulk is selected after shared-arithmetic consolidation and composition remains blocked.'
