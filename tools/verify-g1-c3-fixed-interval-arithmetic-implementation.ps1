Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\fixed-interval-arithmetic'
$sourcePath = Join-Path $crate 'src\lib.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$physicalPath = Join-Path $root 'crates\physical-path-substrate\src\interval.rs'
$physicalManifestPath = Join-Path $root 'crates\physical-path-substrate\Cargo.toml'
$fixturePath = Join-Path $root 'crates\physical-path-substrate\fixtures\interval_cell_step_identity_lock.json'
$contractPath = Join-Path $root 'contracts\fixed-interval-arithmetic-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_FIXED_INTERVAL_ARITHMETIC_CONSOLIDATION_RESULT.md'
foreach ($path in @($sourcePath,$manifestPath,$physicalPath,$physicalManifestPath,$fixturePath,$contractPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Shared arithmetic implementation artifact missing: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$physical = Get-Content -LiteralPath $physicalPath -Raw
$physicalManifest = Get-Content -LiteralPath $physicalManifestPath -Raw
$fixture = Get-Content -LiteralPath $fixturePath -Raw | ConvertFrom-Json
foreach ($required in @('pub struct Signed512','pub struct FixedInterval','from_canonical_decimal','canonical_decimal','div_floor','div_ceil','pub fn sqrt','pub fn project','maximum_magnitude_bits')) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Shared arithmetic API drift: $required" }
}
if ($manifest -notmatch 'crypto-bigint = \{ version = "=0\.7\.5", default-features = false \}' -or
    $manifest -match '(?m)^serde|(?m)^serde_json|(?m)^sha2') {
  throw 'Shared arithmetic dependency surface drifted.'
}
foreach ($forbidden in @('to_words','as_words','std::fs','std::net','std::process','forge_kernel','tauri::')) {
  if ($source -match [regex]::Escape($forbidden)) { throw "Forbidden shared arithmetic mechanism present: $forbidden" }
}
if ($physical -match '(?m)^struct Signed512' -or
    $physical -notmatch 'use fixed_interval_arithmetic::\{FixedArithmeticError, Signed512\};' -or
    $physicalManifest -notmatch 'fixed-interval-arithmetic = \{ path = "\.\./fixed-interval-arithmetic" \}') {
  throw 'Physical cell-step did not migrate only its private signed-512 primitive.'
}
if (@($fixture).Count -ne 7) { throw 'Physical interval identity fixture must contain seven families.' }
foreach ($family in @('normal_certified_face','reverse_outer_face','exact_ambiguity','no_forward_progress','unavailable_neighbor','near_parallel_transfer_ready','negative_near_maximum')) {
  if (@($fixture | Where-Object name -eq $family).Count -ne 1) { throw "Missing interval identity family: $family" }
}
$lock = Get-Content -LiteralPath (Join-Path $root 'Cargo.lock') -Raw
if ([regex]::Matches($lock, '(?m)^name = "crypto-bigint"$').Count -ne 1) {
  throw 'The resolved crypto-bigint package count drifted.'
}
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('capability-free','Native limbs','owns no physical coordinates')) {
  if ($contract -notlike "*$required*") { throw "Shared arithmetic contract drift: $required" }
}
foreach ($required in @('reversible experiment passed','only the physical cell-step migrated','visible-radiance-interface-event','was deliberately not migrated','does not itself authorize')) {
  if ($result -notlike "*$required*") { throw "Shared arithmetic result drift: $required" }
}
Write-Output 'Fixed-interval arithmetic implementation verified: semantic-neutral API, exact dependency, physical-only migration, seven interval identities and native-limb/capability shields are retained.'
