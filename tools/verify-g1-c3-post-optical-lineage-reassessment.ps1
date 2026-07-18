Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\G1_C3_POST_OPTICAL_LINEAGE_BINDING_CONSUMER_REASSESSMENT.md'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$audit = Get-Content -LiteralPath $auditPath -Raw
foreach ($required in @('receiver arrival, cumulative power and physical visibility remain open','source/receiver optical prerequisite','counterexample','arrival-only binding','cumulative dimensionless direct-beam transfer','source-to-receiver opportunity record','underflow-to-darkness','Do not add cumulative arithmetic','separate exact','implementation-readiness package and explicit owner approval')) {
  if ($audit -notlike "*$required*") { throw "Post-lineage reassessment drift: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$postRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-free post-optical-lineage source/receiver prerequisite and counterexample audit*'
$cumulativeRoute = $c3.Count -eq 1 -and ($c3[0].next_action -like '*cumulative dimensionless lane-transfer mathematical design*' -or $c3[0].next_action -like '*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*' -or $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*optical phase-space provenance prerequisite*')
if (-not ($postRoute -or $cumulativeRoute -or $federatedContinuity) -or
    $c3[0].sources -notcontains 'G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_RESULT.md' -or
    $c3[0].sources -notcontains 'G1_C3_POST_OPTICAL_LINEAGE_BINDING_CONSUMER_REASSESSMENT.md') {
  throw 'C3 does not retain the bounded post-lineage reassessment route.'
}
if (-not $federatedContinuity -and ($checkpoint.batch_id -notin @('G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1') -or
    ($checkpoint.authority_lane -notlike '*No crate*schema*source*receiver arrival*visibility*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*' -and
     $checkpoint.authority_lane -notlike '*Mathematical design and independent oracle only*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*requires explicit owner approval*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*' -and
     $checkpoint.authority_lane -notlike '*Owner-authorized implementation*optical-phase-space-transport-certificate*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*'))) {
  throw 'Post-lineage reassessment checkpoint or authority boundary drifted.'
}
Write-Output 'Post-optical-lineage reassessment verified: ordered opportunity is retained while source, receiver, cumulative transfer, arrival, visibility and implementation remain separately gated.'
