Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_CELL_PROVENANCE_IMPLEMENTATION_READINESS.md'
if (-not (Test-Path -LiteralPath $path)) { throw 'Optical phase-space cell readiness is missing.' }
$readiness = Get-Content -LiteralPath $path -Raw
foreach ($required in @(
  'ready for one explicit additive implementation decision',
  'optical-phase-space-cell-binding',
  'TransverseAreaDirection4d',
  'PositiveRationalV1',
  'CorrelatedAffineOutputV1',
  'OpticalPhaseSpaceSplitReceiptV1',
  'OpticalPhaseSpaceProjectionReceiptV1',
  'single positive collective gcd',
  'maximum live magnitude is **368 bits**',
  'root input: 16 KiB',
  'split receipt: 64 KiB',
  'aggregate live canonical bytes per operation: 160 KiB',
  'depth/path: 12',
  'Q160/Q1.62 outward rounding',
  'i686-pc-windows-msvc',
  'aarch64-linux-android',
  'deletion-only',
  'Exact owner action',
  'Add no coupling consumer and modify no existing owner source or V1 behavior',
  'serious change gate'
)) {
  if ($readiness -notlike "*$required*") { throw "Optical phase-space readiness drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$readinessCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-cell-provenance-implementation-readiness' -and $checkpoint.state -eq 'executing'
$ownerCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-cell-provenance-owner-gate' -and $checkpoint.state -eq 'checkpoint'
$implementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and $checkpoint.state -in @('executing','checkpoint')
$transportImplementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and $checkpoint.state -in @('executing','verifying','recorded') -and $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not ($readinessCheckpoint -or $ownerCheckpoint -or $implementationCheckpoint -or $transportImplementationCheckpoint -or $federatedContinuity) -or
    ($implementationCheckpoint -and $checkpoint.authority_lane -notlike '*Owner-approved exact additive package only*no coupling consumer*') -or
    (-not $implementationCheckpoint -and -not $transportImplementationCheckpoint -and -not $federatedContinuity -and (
      $checkpoint.authority_lane -notlike '*Readiness audit only and explicit owner gate*Do not add or modify a crate, dependency, contract schema, production test or production source*' -or
      ($checkpoint.resume_after -notlike '*Await explicit owner approval or rejection*' -and $checkpoint.resume_after -notlike '*On explicit approval*No source action while waiting*')))) {
  throw 'Optical phase-space readiness checkpoint or owner gate drifted.'
}
if (-not $implementationCheckpoint -and -not $transportImplementationCheckpoint -and -not $federatedContinuity) {
  foreach ($forbidden in @('crates\optical-phase-space-cell-binding','contracts\optical-phase-space-cell-binding-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Optical phase-space source appeared before owner approval: $forbidden" }
  }
}
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$readinessRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-facing implementation-readiness audit*common-denominator rational arithmetic*pause at the owner gate*'
$ownerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*Owner: approve or reject*optical phase-space provenance prerequisite*Without explicit approval add no crate, dependency, contract schema, production test or production source*'
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved optical phase-space provenance prerequisite package*optical-phase-space-cell-binding*Add no coupling consumer*'
$transportDesignRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*optical-phase-space-transport-certificate*Add no coupling consumer*production test or source*'
$transportOwnerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*hold the exact serious owner decision*64-bit immutable-origin cap*Add no coupling consumer or production source*'
$transportImplementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*owner-approved additive optical-phase-space-transport-certificate implementation*post-transport consumer reassessment*'
if (-not ($readinessRoute -or $ownerRoute -or $implementationRoute -or $transportDesignRoute -or $transportOwnerRoute -or $transportImplementationRoute -or $federatedContinuity) -or
    ($c3[0].proof -notlike '*20 positive portfolios and 33 hostile rejections*' -and
     $c3[0].sources -notcontains 'G1_C3_OPTICAL_PHASE_SPACE_CELL_PROVENANCE_IMPLEMENTATION_RESULT.md')) {
  throw 'C3 does not retain the optical phase-space readiness gate.'
}
Write-Output 'Optical phase-space cell readiness verified: exact additive V1 types, 368-bit shield, bounded codecs/tests and deletion-only rollback are frozen behind explicit owner approval.'
