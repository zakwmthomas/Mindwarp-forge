Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_RECEIVER_ARRIVAL_GEOMETRY_CONSUMER_REASSESSMENT.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_RESULT.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
foreach ($required in @('phase-space measure','source emission magnitude only','inverse-square spreading','optical lane coupling measure','receiver aperture and detector response','combined source-to-detector record','central ray is insufficient','discretization-invariant','refraction/focusing counterexamples','Do not add a crate')) {
  if ($audit -notlike "*$required*") { throw "Post-receiver reassessment drift: $required" }
}
if ($result -notlike '*implemented and verified*' -or $result -notlike '*229.3 seconds*') { throw 'Verified receiver implementation result is not retained.' }
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$postRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*post-receiver-arrival consumer reassessment*no new crate*schema*source*'
$downstreamRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*' -and $c3[0].proof -like '*receiver-arrival owners remain implemented*'
if (-not ($postRoute -or $downstreamRoute -or $federatedContinuity)) {
  throw 'C3 does not retain the post-receiver code-free reassessment route.'
}
$postCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -eq 'receiver-arrival-geometry-post-result-consumer-reassessment' -and $checkpoint.authority_lane -like '*Audit and comparison only*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$laterReadiness = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-implementation-readiness','optical-phase-space-cell-provenance-owner-gate') -and $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*'
$laterImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and $checkpoint.authority_lane -like '*Owner-approved exact additive package*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*'
$transportImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*visibility*runtime*promotion*C3-closure authority*'
if (-not ($postCheckpoint -or $laterReadiness -or $laterImplementation -or $transportImplementation -or $federatedContinuity)) {
  throw 'Post-receiver reassessment checkpoint or authority boundary drifted.'
}
Write-Output 'Post-receiver reassessment verified: central-ray arrival and transfer remain separate from lane measure, inverse-square is not universal, and only a code-free coupling-measure audit is selected.'
