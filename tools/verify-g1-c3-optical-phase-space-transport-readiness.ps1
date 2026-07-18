Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_CERTIFICATE_IMPLEMENTATION_READINESS.md'
if (-not (Test-Path -LiteralPath $path)) { throw 'Transport implementation readiness is missing.' }
$text = Get-Content -LiteralPath $path -Raw
foreach ($required in @(
  'ready for one explicit additive implementation decision',
  'optical-phase-space-transport-certificate',
  '64-bit immutable-origin',
  'live shield is **490 bits**',
  '4B+234',
  'InterfaceRequired',
  'maximum steps: 64',
  'input bytes: 16 MiB',
  'certificate output: 20 MiB',
  'Any change to the 64-bit cap',
  'This is the serious change gate'
)) {
  if ($text -notlike "*$required*") { throw "Transport implementation readiness drift: $required" }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$ownerGate = $checkpoint.state -eq 'checkpoint' -and
  $checkpoint.substage_id -eq 'optical-phase-space-transport-origin-anchored-owner-gate' -and
  $checkpoint.authority_lane -like '*Owner decision only*64-bit immutable-origin*add no transport crate*production source*'
$implementationRoute = $checkpoint.state -in @('executing','verifying','recorded') -and
  $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not $ownerGate -and -not $implementationRoute -and -not $federatedContinuity) {
  throw 'Transport readiness owner gate drifted.'
}
if ($ownerGate) {
  foreach ($forbidden in @('crates\optical-phase-space-transport-certificate','contracts\optical-phase-space-transport-certificate-contract.md')) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized transport implementation appeared: $forbidden" }
  }
}
foreach ($verifier in @(
  'tools\verify-g1-c3-optical-phase-space-transport-certificate.ps1',
  'tools\verify-g1-c3-optical-phase-space-transport-width.ps1',
  'tools\verify-g1-c3-optical-phase-space-origin-anchored.ps1'
)) {
  & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $root $verifier)
  if ($LASTEXITCODE -ne 0) { throw "Nested transport verifier failed: $verifier" }
}
Write-Output 'Optical phase-space transport readiness verified: the exact 64-bit immutable-origin additive boundary remains frozen through its owner gate or authorized implementation route.'
