$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$script = Join-Path $PSScriptRoot 'prove-g1-c3-interval-numerical-certificate.py'
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (!(Test-Path $python)) { $python = 'python' }

$source = Get-Content $script -Raw
$production = [regex]::Match($source, '(?s)def production_decision\(.*?\n\n(?=def main)').Value
if ([string]::IsNullOrWhiteSpace($production)) { throw 'Production decision function is missing.' }
foreach ($forbidden in @('REFERENCE_PRECISION','numerical_excess','384')) {
  if ($production.Contains($forbidden)) { throw "Production decision reads external oracle truth: $forbidden" }
}

$first = (& $python $script | Out-String).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Interval numerical-certificate oracle failed.' }
$second = (& $python $script | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Interval numerical-certificate oracle is nondeterministic.' }
$receipt = $first | ConvertFrom-Json
if ($receipt.status -ne 'pass' -or $receipt.total_cases -ne 265 -or $receipt.bounded_enclosures -ne 263 -or $receipt.ambiguous_interface_branches -ne 2) {
  throw 'Interval numerical-certificate portfolio receipt drifted.'
}
if ($receipt.production_precision -ne 160 -or $receipt.reference_precision_test_only -ne 384 -or $receipt.production_nonconvergent -ne 0) {
  throw 'Interval numerical-certificate precision or outcome receipt drifted.'
}
if ($receipt.forced_80_raw_evaluator_outcome -ne 'all_transmit' -or $receipt.forced_80_reference_excess_target_units -ne 2991) {
  throw 'Forced-cap evaluator/oracle distinction drifted.'
}
if ($receipt.max_160_vs_384_numerical_excess_target_units -ne 0 -or $receipt.derived_maximum_live_bits -ne 452 -or $receipt.storage_bits -ne 512) {
  throw 'Interval numerical-certificate containment or storage receipt drifted.'
}
if ($receipt.receipt_sha256 -ne '6f4c5997d23bd6a463ccc7e7d0d3a843f52f453425b4eea5566721f4535dc082') {
  throw 'Interval numerical-certificate checksum drifted.'
}
Write-Output "Interval numerical certificate verified: $($receipt.checks) checks, checksum $($receipt.receipt_sha256)."

