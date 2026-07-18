Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$design = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_FIXED_INTERVAL_ARITHMETIC_CONSOLIDATION_DESIGN_AUDIT.md') -Raw
$readiness = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_FIXED_INTERVAL_ARITHMETIC_CONSOLIDATION_IMPLEMENTATION_READINESS.md') -Raw
$physicalSource = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\src\interval.rs') -Raw
$physicalManifest = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\Cargo.toml') -Raw
foreach ($required in @('semantic-neutral arithmetic crate','migrate physical cell-step first','third private copy','seven permanent conditional cell-step','Explicit owner approval')) {
  if (($design + $readiness) -notlike "*$required*") { throw "Fixed-arithmetic readiness is missing: $required" }
}
if (Test-Path -LiteralPath (Join-Path $root 'crates\fixed-interval-arithmetic')) {
  throw 'Shared fixed-arithmetic production crate appeared before explicit owner approval.'
}
if ($physicalManifest -match 'fixed-interval-arithmetic' -or $physicalSource -notmatch 'struct Signed512') {
  throw 'Physical arithmetic migration occurred before explicit owner approval.'
}
Write-Output 'Fixed-interval arithmetic consolidation readiness verified: staged semantic-neutral extraction, compatibility capture, one-consumer migration and owner stop are retained.'
