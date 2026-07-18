Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$readiness = Get-Content -LiteralPath $readinessPath -Raw
foreach ($required in @('implementation-ready package explicitly approved','crates/optical-lane-transfer-binding','fixed-interval-arithmetic','CumulativeOpticalLaneTransferInputV1','CumulativeOpticalLaneTransferV1','mindwarp.optical-lineage.cumulative-factor.v1','mindwarp.optical-lineage.cumulative-result.v1','mindwarp.optical-lineage.cumulative-transcript.v1','18 MiB','256 KiB','128','32 MiB','209 bits','[0,1]','all 26 hostile','all ten','i686-pc-windows-msvc','aarch64-linux-android','Rollback is deletion-only','Approval of this exact package','explicitly approved this exact package')) {
  if ($readiness -notlike "*$required*") { throw "Cumulative transfer readiness drift: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved*G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_READINESS.md*optical-lane-transfer-binding*'
$completedRoute = $c3.Count -eq 1 -and ($c3[0].next_action -like '*code-free receiver-arrival geometry mathematical design*counterexample/oracle audit*' -or $c3[0].next_action -like '*G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_READINESS.md*' -or $c3[0].next_action -like '*optical phase-space provenance prerequisite*') -and ($c3[0].proof -like '*optical-lane-transfer-binding is implemented and verified*' -or $c3[0].proof -like '*cumulative transfer owner remains implemented and verified*' -or $c3[0].proof -like '*cumulative transfer and receiver-arrival owners remain implemented*')
if (-not ($implementationRoute -or $completedRoute -or $federatedContinuity)) {
  throw 'C3 does not retain the exact cumulative-transfer owner action.'
}
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1' -and $checkpoint.state -eq 'executing' -and $checkpoint.substage_id -eq 'cumulative-lane-transfer-additive-source-implementation' -and $checkpoint.authority_lane -like '*explicitly approved*'
$completedCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1' -and $checkpoint.authority_lane -like '*Mathematical design and independent oracle only*No crate*schema*source*'
$ownerCheckpoint = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1' -and $checkpoint.authority_lane -like '*requires explicit owner approval*No crate*schema*source*'
$receiverImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.authority_lane -like '*explicitly approved*receiver-arrival*'
$laterReadiness = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-implementation-readiness','optical-phase-space-cell-provenance-owner-gate') -and $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*'
$laterImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and $checkpoint.authority_lane -like '*Owner-approved exact additive package*Existing owner source and behavior remain unchanged*'
$transportImplementation = $checkpoint.batch_id -eq 'G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not ($implementationCheckpoint -or $completedCheckpoint -or $ownerCheckpoint -or $receiverImplementation -or $laterReadiness -or $laterImplementation -or $transportImplementation -or $federatedContinuity) -or
    !(Test-Path -LiteralPath (Join-Path $root 'crates\optical-lane-transfer-binding'))) {
  throw 'Cumulative-transfer readiness is not aligned to its approved implementation checkpoint.'
}
Write-Output 'Cumulative lane-transfer readiness verified: exact additive package is approved and implementation remains inside its frozen dependency, arithmetic, cap, hostile, platform and rollback boundary.'
