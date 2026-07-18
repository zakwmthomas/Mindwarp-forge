Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md'
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @('implementation-ready behind one exact owner action','crates/receiver-arrival-geometry-binding','optical-lineage-binding','physical-path-substrate','fixed-interval-arithmetic','must not depend on the cumulative-transfer crate','ReceiverArrivalGeometryInputV1','ReceiverAabbV1','mindwarp.receiver-arrival.aabb.v1','mindwarp.receiver-arrival.result.v1','mindwarp.receiver-arrival.transcript.v1','unsupported_conditional_evidence','arrival_at_start','certified_strict_interior_arrival','0 <= t < t_face','414-bit shield','384 divisions','768 bound comparisons','18 MiB','256 KiB','32 MiB','all 18','26','i686-pc-windows-msvc','aarch64-linux-android','Rollback is deletion-only','Approval of this exact package','heartbeat paused')) {
  if ($readiness -notlike "*$required*") { throw "Receiver-arrival readiness drift: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$approvedRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*approved*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*test-first*receiver-arrival-geometry-binding*' -and $c3[0].proof -like '*18*26*'
$completedRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*' -and $c3[0].proof -like '*receiver-arrival owners remain implemented*'
if (-not ($approvedRoute -or $completedRoute -or $federatedContinuity)) {
  throw 'C3 does not retain the approved exact receiver-arrival geometry action.'
}
$approved = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and
  $checkpoint.state -eq 'executing' -and
  $checkpoint.substage_id -in @('receiver-arrival-geometry-additive-source-implementation','receiver-arrival-geometry-post-result-consumer-reassessment') -and
  $checkpoint.authority_lane -like '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*'
$laterReadiness = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-implementation-readiness','optical-phase-space-cell-provenance-owner-gate') -and $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*'
$laterImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and $checkpoint.authority_lane -like '*Owner-approved exact additive package*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*'
$transportImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*visibility*runtime*promotion*C3-closure authority*'
if (-not ($approved -or $laterReadiness -or $laterImplementation -or $transportImplementation -or $federatedContinuity) -or -not (Test-Path -LiteralPath (Join-Path $root 'crates\receiver-arrival-geometry-binding'))) {
  throw 'Receiver-arrival geometry readiness is not aligned to approved implementation.'
}
Write-Output 'Receiver-arrival geometry readiness verified: exact-ray AABB semantics, 414-bit arithmetic, caps, hostile/platform gates, deletion rollback and explicit owner approval are frozen.'
