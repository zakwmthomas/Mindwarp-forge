Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scriptPath = Join-Path $root 'tools\prove-g1-c3-interval-bulk-transfer.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_BULK_TRANSFER_ORACLE_RESULT.md'
$expectedSource = 'f5ed61bbe73a6df645ef31497d6930f12057f5f7346572e9c41b7f55647ecae6'
$actualSource = (Get-FileHash -LiteralPath $scriptPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualSource -ne $expectedSource) { throw 'Interval bulk oracle source checksum drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$receipt = (& $python $scriptPath | Out-String | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0) { throw 'Interval bulk oracle failed.' }
if ($receipt.receipt_sha256 -ne '94b2fe43260c9a604ec6c22035f28f7026319531c22951a4e8747f8d242713c3' -or
    $receipt.generated_256.outcomes.finite -ne 247 -or
    $receipt.generated_256.outcomes.ambiguous_next_face -ne 9 -or
    $receipt.generated_256.corner_checks -ne 15808 -or
    $receipt.maximum_observed_live_bits -ne 321 -or
    @($receipt.repeated_64_step_lanes.PSObject.Properties).Count -ne 4) {
  throw 'Interval bulk oracle canonical receipt drifted.'
}
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'one spectral band per call',
  'intersects two independently sound',
  '414-magnitude-bit',
  'shared fixed-interval arithmetic consolidation',
  'rejects a third private signed-512 implementation',
  'authorizes no Rust source'
)) {
  if ($result -notlike "*$required*") { throw "Interval bulk result is missing: $required" }
}
Write-Output 'Interval bulk oracle verified: 247/256 finite, 9 typed ambiguities, 16,320 generated/named corner checks, four 64-step lanes and dual length certificates.'
