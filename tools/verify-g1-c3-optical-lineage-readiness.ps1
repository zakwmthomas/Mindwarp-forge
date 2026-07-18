Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_READINESS.md'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$checkpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath $checkpointPath -Raw | ConvertFrom-Json
foreach ($required in @('implementation-ready behind one exact owner action','crates/optical-lineage-binding','fixed-interval-arithmetic','v1 owns no numerical fold','exactly ten distinct families','mindwarp.optical-lineage.lane.v1','mindwarp.optical-lineage.transcript.v1','at most 64 steps','1 MiB','16 MiB','384','24 MiB','26 hostile oracle cases','six fully resealed attacker','i686 Windows','Android','ARM64 check','Rollback is deletion-only','Approval of this exact package','Forge heartbeat')) {
  if ($readiness -notlike "*$required*") { throw "Optical-lineage readiness is missing: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1) { throw 'C3 is missing from the master program.' }
$preApproval = $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-READINESS-V1'
$approvedExecution = $checkpoint.batch_id -eq 'G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1'
$completedExecution = $checkpoint.batch_id -in @('G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1')
$federatedContinuity = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not ($preApproval -or $approvedExecution -or $completedExecution -or $federatedContinuity)) { throw 'Optical-lineage readiness has an unknown transition state.' }
if ($preApproval) {
  if ($checkpoint.substage_id -ne 'optical-lineage-binding-owner-gate' -or $checkpoint.state -ne 'checkpoint' -or
      $checkpoint.authority_lane -notlike '*requires explicit owner approval*' -or
      (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lineage-binding'))) {
    throw 'Pre-approval optical-lineage readiness state drifted.'
  }
} elseif ($approvedExecution) {
  if ($checkpoint.state -ne 'executing' -or $checkpoint.authority_lane -notlike '*explicitly approved*' -or
      $c3[0].next_action -notlike '*owner-approved additive optical-lineage-binding*' -or
      -not (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lineage-binding'))) {
    throw 'Approved optical-lineage implementation transition is not aligned.'
  }
} elseif (-not $federatedContinuity -and (($c3[0].proof -notlike '*prerequisite audit*' -and $c3[0].proof -notlike '*optical-lane-transfer-binding*implemented and verified*' -and $c3[0].proof -notlike '*cumulative transfer owner remains implemented and verified*' -and $c3[0].proof -notlike '*cumulative transfer and receiver-arrival owners remain implemented*') -or
         ($checkpoint.authority_lane -notlike '*No crate*schema*source*receiver arrival*visibility*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*explicitly approved*Receiver geometry*arrival*visibility*perception*runtime*promotion*C3 closure*excluded*' -and
          $checkpoint.authority_lane -notlike '*Mathematical design and independent oracle only*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*requires explicit owner approval*No crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*explicitly approved*receiver-arrival*no new crate*schema*source*visibility*perception*runtime*promotion*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*' -and
          $checkpoint.authority_lane -notlike '*Owner-authorized implementation*optical-phase-space-transport-certificate*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*'))) {
  throw 'Completed optical-lineage execution is not preserved by the post-result route.'
}
Write-Output 'Optical-lineage implementation readiness verified: additive dependency boundary, exact domains and sources, ten terminals, 64-step and resource caps, hostile/platform gates, rollback and owner checkpoint are frozen.'
