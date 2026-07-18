Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_RECEIVER_OPTICAL_PREREQUISITE_COUNTEREXAMPLE_AUDIT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @('arrival-only and combined source-to-receiver schemas are rejected','cumulative dimensionless lane-transfer mathematical','A terminal face is not a receiver','Cross-band multiplication fabricates a path','Repeated Q0.48 projection can invent avoidable zero','Interface terminal power is not a followed factor','Zero transfer is not visibility or darkness','exact rational multiplication','independent counterexample/oracle audit','Do not add a crate')) {
  if ($audit -notlike "*$required*") { throw "Source-receiver prerequisite audit drift: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$selectedRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*cumulative dimensionless lane-transfer mathematical design and independent counterexample/oracle audit*'
$ownerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*explicit owner approval*'
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*optical-lane-transfer-binding*'
$receiverDesignRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*counterexample/oracle audit*'
$receiverOwnerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*explicit owner approval*'
$receiverImplementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*approved*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*receiver-arrival-geometry-binding*'
$downstreamRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*'
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
if (-not ($selectedRoute -or $ownerRoute -or $implementationRoute -or $receiverDesignRoute -or $receiverOwnerRoute -or $receiverImplementationRoute -or $downstreamRoute -or $federatedContinuity) -or
    ($c3[0].proof -notlike '*prerequisite audit*' -and $c3[0].proof -notlike '*optical-lane-transfer-binding*implemented and verified*' -and $c3[0].proof -notlike '*cumulative transfer owner remains implemented and verified*' -and $c3[0].proof -notlike '*cumulative transfer and receiver-arrival owners remain implemented*' -and $c3[0].proof -notlike '*exact-lineage cumulative transfer*complete-cell result*' -and $c3[0].proof -notlike '*whole-cell dimensionless-transfer result*source-distribution*' -and $c3[0].proof -notlike '*source-quantity basis mathematical design*band/time-integrated radiant energy*' -and $c3[0].proof -notlike '*calibrated spectral/time mathematical design*pointwise*' -and $c3[0].proof -notlike '*calibrated-basis and transport-applicability*Transport applicability remains blocked*' -and $c3[0].proof -notlike '*closed-frontier additive calibrated radiant-energy measure*Transport applicability remains blocked*' -and $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and $c3[0].proof -notlike '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*')) {
  throw 'C3 does not retain the selected cumulative-transfer design audit and stop condition.'
}
if (-not $federatedContinuity -and ($checkpoint.batch_id -notin @('G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1') -or
    ($checkpoint.authority_lane -notlike '*Design and oracle only*No crate*schema*production source*receiver arrival*visibility*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*requires explicit owner approval*No crate*schema*source*receiver arrival*visibility*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*' -and
     $checkpoint.authority_lane -notlike '*Mathematical design and independent oracle only*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*requires explicit owner approval*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Owner-authorized implementation*optical-phase-space-transport-certificate*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*'))) {
  throw 'Cumulative-transfer design checkpoint or authority boundary drifted.'
}
Write-Output 'Source-receiver prerequisite audit verified: premature arrival is rejected and only a cumulative dimensionless lane-transfer mathematical/oracle audit is selected.'
