Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LINEAGE_COMPOSITION_DESIGN_REASSESSMENT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @('thin immutable lane manifest','explicitly supplied bounded object bundle','IDs-only thin manifest with ambient lookup','reject as canonical v1 candidate','lossless adapter rule','next Q160 point box equals','Cumulative power is deliberately excluded','Endpoint arrival is also excluded','one, three, 64 and 192-step','no schema, crate or composer implementation authorized')) {
  if ($audit -notlike "*$required*") { throw "Optical lineage design is missing: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$oracleRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*optical-lineage counterexample/oracle audit*' -and
  $c3[0].next_action -like '*do not add composer source*cumulative-power arithmetic*receiver semantics*' -and
  $c3[0].proof -like '*ambient lookup*destructive streaming*fully nested canonical objects*'
$ownerGateRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_READINESS.md*' -and
  $c3[0].proof -like '*26 hostile cases*six fully resealed attacks*ten typed terminals*'
$implementationRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-approved additive optical-lineage-binding*' -and
  $c3[0].proof -like '*owner explicitly approved*four fixture hashes*'
$completedRoute = $c3.Count -eq 1 -and
  ($c3[0].next_action -like '*code-free post-optical-lineage source/receiver prerequisite*' -or
   $c3[0].next_action -like '*cumulative dimensionless lane-transfer mathematical design*' -or
   $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*' -or
   $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*' -or
   $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -or
   $c3[0].next_action -like '*optical phase-space provenance prerequisite*') -and
  ($c3[0].proof -like '*prerequisite audit*' -or $c3[0].proof -like '*optical-lane-transfer-binding*implemented and verified*' -or $c3[0].proof -like '*cumulative transfer owner remains implemented and verified*' -or $c3[0].proof -like '*cumulative transfer and receiver-arrival owners remain implemented*')
if (-not ($oracleRoute -or $ownerGateRoute -or $implementationRoute -or $completedRoute -or $federatedContinuity)) {
  throw 'C3 does not retain the selected optical-lineage oracle route and stop condition.'
}
$oracleCheckpoint = $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-COUNTEREXAMPLE-ORACLE-V1' -and
  $checkpoint.substage_id -eq 'optical-lineage-counterexample-oracle' -and
  $checkpoint.authority_lane -like '*No Rust schema*composer*cumulative-power fold*receiver semantics*'
$ownerCheckpoint = $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.substage_id -eq 'optical-lineage-binding-owner-gate' -and
  $checkpoint.authority_lane -like '*requires explicit owner approval*'
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*' -and
  $checkpoint.authority_lane -like '*cumulative power*endpoint claims*C3 closure remain excluded*'
$completedCheckpoint = $checkpoint.batch_id -in @('G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1') -and
  $checkpoint.authority_lane -like '*No crate*schema*source*receiver arrival*visibility*C3 closure*'
$cumulativeImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*'
$receiverDesignCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.authority_lane -like '*Mathematical design and independent oracle only*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$receiverOwnerCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.authority_lane -like '*requires explicit owner approval*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$receiverImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$phaseSpaceReadinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*'
$phaseSpaceImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.authority_lane -like '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*'
$transportImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*'
if (-not ($oracleCheckpoint -or $ownerCheckpoint -or $implementationCheckpoint -or $completedCheckpoint -or $cumulativeImplementationCheckpoint -or $receiverDesignCheckpoint -or $receiverOwnerCheckpoint -or $receiverImplementationCheckpoint -or $phaseSpaceReadinessCheckpoint -or $phaseSpaceImplementationCheckpoint -or $transportImplementationCheckpoint -or $federatedContinuity)) {
  throw 'The optical-lineage oracle checkpoint or authority boundary drifted.'
}
Write-Output 'Optical-lineage design verified: thin per-band manifest plus explicit replayed bundle is selected for hostile oracle proof; ambient lookup, cumulative power, receiver arrival and source remain excluded.'
