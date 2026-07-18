Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\G1_C3_POST_OPTICAL_PHASE_SPACE_PROVENANCE_CONSUMER_REASSESSMENT.md'
if (-not (Test-Path -LiteralPath $path)) { throw 'Post phase-space provenance reassessment is missing.' }
$audit = Get-Content -LiteralPath $path -Raw
foreach ($required in @(
  'cell owner closes source measure, ancestry and affine',
  'correlation, but a coupling consumer is not implementation-ready',
  'forms are the transported image',
  'makes no claim that its',
  'forms are physically correct',
  'Treat projection receipt as transported truth',
  'Import the cell into current lineage/receiver owners',
  'optical-phase-space-transport-certificate',
  'same-band and time basis',
  'ordered topology/branch family',
  'forged-form',
  'space affine family should be exact',
  'Add no crate, contract schema, dependency, production test or source',
  'separate serious owner decision'
)) {
  if ($audit -notlike "*$required*") { throw "Post phase-space provenance reassessment drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
if (-not $federatedContinuity -and ($checkpoint.substage_id -notin @('optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate','optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -or
    ($checkpoint.authority_lane -notlike '*Audit and comparison only*add no coupling consumer*production test or source*' -and
     $checkpoint.authority_lane -notlike '*Mathematical design and disposable exact-rational oracle only*add no coupling consumer*production test or source*' -and
     $checkpoint.authority_lane -notlike '*Owner decision only*add no transport crate*production source*' -and
     $checkpoint.authority_lane -notlike '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'))) {
  throw 'Post phase-space provenance reassessment checkpoint or authority boundary drifted.'
}
if (-not $federatedContinuity -and $checkpoint.substage_id -notin @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result')) {
  foreach ($forbidden in @('crates\optical-phase-space-transport-certificate','contracts\optical-phase-space-transport-certificate-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized transport certificate source appeared: $forbidden" }
  }
}
Write-Output 'Post phase-space provenance reassessment verified: measure/correlation provenance is closed, transported-form authority remains open, and only a code-free transport-certificate design/oracle is selected.'
