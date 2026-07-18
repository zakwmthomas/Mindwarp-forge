Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-optical-phase-space-transport-certificate.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_CERTIFICATE_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_CERTIFICATE_ORACLE_RESULT.md'
foreach ($path in @($sourcePath,$designPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing optical phase-space transport certificate artifact: $path" }
}
if ((Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne 'a678220e7aebd3ec9e71c4df3a2a791d323848e81b3255da7f66e077aac185b5') {
  throw 'Optical phase-space transport certificate oracle source drifted.'
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*17b2edb4757b852470bdd9fab8d813b3b184605b4c555edecb55c18ce8fb197f*') {
  throw 'Optical phase-space transport certificate oracle receipt drifted.'
}
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'caller never supplies the claimed output forms',
  'Fixed affine advance',
  'Axis-plane intersection enclosure',
  'unsupported_nonlinear_interface',
  '4/16/64 child-measure conservation',
  'Add no crate, contract schema, dependency, production test or source'
)) {
  if ($design -notlike "*$required*") { throw "Optical phase-space transport design drift: $required" }
}
foreach ($required in @(
  'Positive portfolios: **24**',
  'Hostile rejections: **33**',
  'Maximum observed certificate size: **3,105 bytes**',
  '[30/17, 34/15]',
  '[-1/2, 1/2]',
  'unsupported_parallel_or_reversed_plane',
  'implementation-readiness',
  'separate serious owner decision'
)) {
  if ($result -notlike "*$required*") { throw "Optical phase-space transport result drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$oracleCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-certificate-design-and-oracle' -and
  $checkpoint.authority_lane -like '*Mathematical design and disposable exact-rational oracle only*add no coupling consumer, crate, contract schema, dependency, production test or source*'
$readinessCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-certificate-implementation-readiness' -and
  $checkpoint.authority_lane -like '*Code-facing readiness audit only*no crate, contract schema, dependency, production test or production source*'
$widthCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-certificate-arithmetic-width-spike' -and
  $checkpoint.authority_lane -like '*Disposable exact-rational width and utility spike only*add no transport crate, contract schema, dependency, production test or production source*'
$anchoredCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-origin-anchored-design-and-oracle' -and
  $checkpoint.authority_lane -like '*Origin-anchored mathematical design and disposable exact-rational oracle only*add no transport crate, contract schema, dependency, production test or production source*'
$ownerCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-origin-anchored-owner-gate' -and
  $checkpoint.authority_lane -like '*Owner decision only*64-bit immutable-origin*add no transport crate*production source*'
$implementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not ($oracleCheckpoint -or $readinessCheckpoint -or $widthCheckpoint -or $anchoredCheckpoint -or $ownerCheckpoint -or $implementationCheckpoint -or $federatedContinuity)) {
  throw 'Optical phase-space transport certificate checkpoint drifted.'
}
if (-not $implementationCheckpoint -and -not $federatedContinuity) {
  foreach ($forbidden in @('crates\optical-phase-space-transport-certificate','contracts\optical-phase-space-transport-certificate-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized optical phase-space transport schema appeared: $forbidden" }
  }
}
Write-Output 'Optical phase-space transport certificate verified: 24 positive portfolios and 33 hostile rejections preserve exact free-space derivation, correlation, topology order and typed nonlinear stops without authorizing source.'
