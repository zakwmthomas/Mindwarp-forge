Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-optical-phase-space-transport-width.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_WIDTH_SPIKE_RESULT.md'
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_CERTIFICATE_IMPLEMENTATION_READINESS.md'
foreach ($path in @($sourcePath,$resultPath,$readinessPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing transport width artifact: $path" }
}
if ((Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne '112fbb78356b38c0b2fad53a49c07040ea4812e484a54725b823b3f8c011d71d') {
  throw 'Transport width spike source drifted.'
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*f7d2db26715b9a015918e3f48e25da98e9faaab2abc30ada0f4ce3801820c0c9*') {
  throw 'Transport width spike receipt drifted.'
}
$result = Get-Content -LiteralPath $resultPath -Raw
$readiness = Get-Content -LiteralPath $readinessPath -Raw
foreach ($required in @('repeated-relinearization','Signed512','64-bit immutable-origin','This is the serious change gate')) {
  if ($readiness -notlike "*$required*") { throw "Transport readiness drift: $required" }
}
foreach ($required in @('16 | 327 / 233 bits | 513 / 403 bits','24 | 488 / 353 bits | 778 / 613 bits','No constructed two-step cap remains within 512 bits','representation failure','Surviving origin-anchored question','No crate, schema, dependency, production test or production source')) {
  if ($result -notlike "*$required*") { throw "Transport width result drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$proofRoute = $checkpoint.substage_id -in @('optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and
  $checkpoint.authority_lane -like '*add no transport crate, contract schema, dependency, production test or production source*'
$implementationRoute = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not $proofRoute -and -not $implementationRoute -and -not $federatedContinuity) {
  throw 'Transport width checkpoint drifted.'
}
if (-not $implementationRoute -and -not $federatedContinuity) {
  foreach ($forbidden in @('crates\optical-phase-space-transport-certificate','contracts\optical-phase-space-transport-certificate-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized transport artifact appeared: $forbidden" }
  }
}
Write-Output 'Optical phase-space transport width verified: repeated relinearization exceeds the 512-bit shield and remains rejected; only the origin-anchored design question survives.'
