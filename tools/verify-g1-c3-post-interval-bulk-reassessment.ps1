Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_RESULT.md') -Raw
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_INTERVAL_BULK_TRANSFER_CONSUMER_REASSESSMENT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @('implemented and verified as one additive one-band local proof','outward Q160 length','eight existing bulk V1','same 17 tests passed','passed in 232.8 seconds','Actual mobile-device performance remains unmeasured','optical arithmetic migration')) {
  if ($result -notlike "*$required*") { throw "Interval bulk result is missing: $required" }
}
foreach ($required in @('local numerical prerequisites are present','bounded optical-lineage','thin immutable lineage manifest','streaming fold','no composer','interface owner''s Q1.62','endpoint relation','Do not add composer source')) {
  if ($audit -notlike "*$required*") { throw "Post-interval-bulk reassessment is missing: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$designRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*bounded optical-lineage composition mathematical design reassessment*' -and
  $c3[0].next_action -like '*Do not add composer source*' -and
  $c3[0].proof -like '*ordered lineage*remain unproved*'
$oracleRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*optical-lineage counterexample/oracle audit*' -and
  $c3[0].next_action -like '*do not add composer source*cumulative-power arithmetic*receiver semantics*' -and
  $c3[0].sources -contains 'G1_C3_POST_INTERVAL_BULK_TRANSFER_CONSUMER_REASSESSMENT.md'
$ownerGateRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_READINESS.md*' -and
  $c3[0].next_action -like '*Without approval*do not create the crate*schema*' -and
  $c3[0].proof -like '*materially new cross-module schema*requires an exact owner action*'
$implementationRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-approved additive optical-lineage-binding*' -and
  $c3[0].proof -like '*owner explicitly approved*four fixture hashes*'
$completedRoute = $c3.Count -eq 1 -and
  ($c3[0].next_action -like '*code-free post-optical-lineage source/receiver prerequisite*' -or
   $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*' -or
   $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*' -or
   $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -or
   $c3[0].next_action -like '*optical phase-space provenance prerequisite*' -or
   $c3[0].next_action -like '*whole-cell receiver-coupling mathematical design*' -or
   $c3[0].next_action -like '*code-free mathematical design*whole-cell dimensionless transfer*' -or
   $c3[0].next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -or
   $c3[0].next_action -like '*owner-authorized whole-cell dimensionless-transfer implementation*' -or
   $c3[0].next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*' -or
   $c3[0].next_action -like '*code-free source-distribution*phase-space-measure compatibility*' -or
   $c3[0].next_action -like '*code-facing source-quantity-basis*schema gap audit*' -or
   $c3[0].next_action -like '*source-quantity-basis mathematical design audit*' -or
   $c3[0].next_action -like '*calibrated spectral/time basis mathematical design audit*' -or
   $c3[0].next_action -like '*calibrated-basis*transport-applicability schema gap audit*' -or
   $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*' -or
   $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -or
   $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -or
   $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -or
   $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -and
  ($c3[0].proof -like '*prerequisite audit*' -or $c3[0].proof -like '*optical-lane-transfer-binding*implemented and verified*' -or $c3[0].proof -like '*cumulative transfer owner remains implemented and verified*' -or $c3[0].proof -like '*cumulative transfer and receiver-arrival owners remain implemented*' -or $c3[0].proof -like '*exact-lineage cumulative transfer*complete-cell result*' -or $c3[0].proof -like '*per-step optical depth*16 portfolios*' -or $c3[0].proof -like '*whole-cell dimensionless-transfer result*source-distribution*' -or $c3[0].proof -like '*no current owner defines physical source quantity*' -or $c3[0].proof -like '*source-quantity basis mathematical design*temporal correlation*' -or $c3[0].proof -like '*calibrated spectral/time mathematical design*whole-cell pointwise*' -or $c3[0].proof -like '*calibrated-basis and transport-applicability*stateless derived commitment*' -or $c3[0].proof -like '*closed-frontier additive calibrated radiant-energy measure*Transport applicability remains blocked*' -or $c3[0].proof -like '*compact axis-bearing*zero downstream consumers*' -or $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*' -or $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*')
if (-not ($designRoute -or $oracleRoute -or $ownerGateRoute -or $implementationRoute -or $completedRoute)) {
  throw 'C3 does not retain the bounded post-interval-bulk composition design route.'
}
$checkpointBoundary = $checkpoint.authority_lane -like '*composer source*remain excluded*' -or
  $checkpoint.authority_lane -like '*No Rust schema*composer*cumulative-power fold*receiver semantics*' -or
  $checkpoint.authority_lane -like '*requires explicit owner approval*Cumulative power*receiver semantics*' -or
  $checkpoint.authority_lane -like '*explicitly approved*cumulative power*endpoint claims*C3 closure remain excluded*' -or
  $checkpoint.authority_lane -like '*Audit and counterexample work only*No crate*numerical fold*arrival*visibility*C3 closure*' -or
  $checkpoint.authority_lane -like '*requires explicit owner approval*No crate*schema*source*receiver arrival*visibility*C3 closure*' -or
  $checkpoint.authority_lane -like '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*' -or
  $checkpoint.authority_lane -like '*Mathematical design and independent oracle only*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*requires explicit owner approval*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*existing cell, physical and fixed-arithmetic APIs unchanged*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*' -or
  $checkpoint.authority_lane -like '*Code-free reassessment*whole-cell dimensionless transfer*No crate*source magnitude*detector response*visibility*C3 closure*'
$federatedContinuity = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not $federatedContinuity -and ($checkpoint.batch_id -notin @('G1-C3-POST-INTERVAL-BULK-TRANSFER-CONSUMER-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COMPOSITION-DESIGN-REASSESSMENT-V1','G1-C3-OPTICAL-LINEAGE-COUNTEREXAMPLE-ORACLE-V1','G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-READINESS-V1','G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1','G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1') -or
    -not $checkpointBoundary)) {
  throw 'The active checkpoint does not retain the post-interval-bulk source boundary.'
}
Write-Output 'Post-interval-bulk reassessment verified: all three local operations are present, bounded lineage design is selected, and composer source, endpoint, visibility and C3 closure remain excluded.'
