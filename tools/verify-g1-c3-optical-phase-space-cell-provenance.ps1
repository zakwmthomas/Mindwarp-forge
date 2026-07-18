Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-optical-phase-space-cell-provenance.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_CELL_PROVENANCE_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_CELL_PROVENANCE_ORACLE_RESULT.md'
foreach ($path in @($sourcePath,$designPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing optical phase-space provenance artifact: $path" }
}
if ((Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne '7740595a08656d616f714bc3e1f249acd0a9b0fe95b486736f0227158981d5f6') {
  throw 'Optical phase-space provenance oracle source drifted.'
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*f9b354164a13bdaa312af6c8711915f661fea4a9abd7fe5ba097f872afb297e6*') {
  throw 'Optical phase-space provenance oracle receipt drifted.'
}
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'exact positive reduced rational parent measure',
  'correlation-preserving output forms',
  'Canonical binary refinement',
  'u-u=0',
  '4, 16 and 64-way measure conservation',
  'Do not add a crate'
)) {
  if ($design -notlike "*$required*") { throw "Optical phase-space provenance design drift: $required" }
}
foreach ($required in @(
  'abstract prerequisite survives',
  'Positive portfolios: **20**',
  'Hostile rejections: **33**',
  'full-tree cells: **127**',
  'split receipts: **63**',
  '[-2/1, 2/1]',
  '1,145 bytes',
  'code-facing implementation-readiness audit only'
)) {
  if ($result -notlike "*$required*") { throw "Optical phase-space provenance result drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$designCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-cell-provenance-design-and-oracle' -and
  $checkpoint.authority_lane -like '*Mathematical design and disposable exact-rational oracle only*Do not add or modify a crate, dependency, schema, production test or production source*'
$readinessCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-implementation-readiness','optical-phase-space-cell-provenance-owner-gate') -and
  $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Do not add or modify a crate, dependency, contract schema, production test or production source*'
$implementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and
  $checkpoint.authority_lane -like '*Owner-approved exact additive package only*no coupling consumer*'
$transportImplementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not ($designCheckpoint -or $readinessCheckpoint -or $implementationCheckpoint -or $transportImplementationCheckpoint -or $federatedContinuity)) {
  throw 'Optical phase-space provenance proof checkpoint drifted.'
}
if (-not $implementationCheckpoint -and -not $transportImplementationCheckpoint -and -not $federatedContinuity) {
  foreach ($forbidden in @('crates\optical-phase-space-cell-binding','contracts\optical-phase-space-cell-binding-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized optical phase-space schema appeared: $forbidden" }
  }
}
Write-Output 'Optical phase-space provenance verified: 20 positive portfolios and 33 hostile rejections preserve exact identity, measure, binary ancestry and affine correlation without authorizing a schema.'
