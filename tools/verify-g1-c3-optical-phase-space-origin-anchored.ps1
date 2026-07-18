Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-optical-phase-space-origin-anchored.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_ORIGIN_ANCHORED_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_ORIGIN_ANCHORED_ORACLE_RESULT.md'
foreach ($path in @($sourcePath,$designPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing origin-anchored transport artifact: $path" }
}
if ((Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne '97b287ec78d2d8f5031a3c7fbddbcd435db77a649e63ee4697519e1d6f66c156') {
  throw 'Origin-anchored transport oracle source drifted.'
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*bbedc5a632b112b6eb633af57830034dfb99f98881f4f8968fbd44a42be93e76*') {
  throw 'Origin-anchored transport oracle receipt drifted.'
}
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('immutable-origin free-space run','caller supplies no plane, face, output form, topology token or branch result','Optimized common-denominator algebra','caps around 64 to 70 bits','Add no crate')) {
  if ($design -notlike "*$required*") { throw "Origin-anchored design drift: $required" }
}
foreach ($required in @('64-bit input subdomain','4B + 234','B=70','**514 bits**','B=64','**490 bits**','ordered faces','**408 bits**','Hostile rejections and typed stops: **15**','code-facing readiness obligations')) {
  if ($result -notlike "*$required*") { throw "Origin-anchored result drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$oracleCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-transport-origin-anchored-design-and-oracle' -and
  $checkpoint.authority_lane -like '*Origin-anchored mathematical design and disposable exact-rational oracle only*add no transport crate, contract schema, dependency, production test or production source*'
$readinessCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and
  $checkpoint.authority_lane -like '*64-bit immutable-origin*no transport crate*production source*'
$implementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not ($oracleCheckpoint -or $readinessCheckpoint -or $implementationCheckpoint -or $federatedContinuity)) { throw 'Origin-anchored transport checkpoint drifted.' }
if (-not $implementationCheckpoint -and -not $federatedContinuity) {
  foreach ($forbidden in @('crates\optical-phase-space-transport-certificate','contracts\optical-phase-space-transport-certificate-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized origin-anchored transport artifact appeared: $forbidden" }
  }
}
Write-Output 'Origin-anchored transport verified: exact optimized algebra, 64-bit/490-bit conservative shield, three ordered faces, 18 equivalence falsifiers and 15 hostile stops pass without authorizing source.'
