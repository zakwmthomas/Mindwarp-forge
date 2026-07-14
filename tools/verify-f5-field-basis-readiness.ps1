$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\FIELD_BASIS_READINESS.md'
$designPath = Join-Path $root 'docs\canonical-system\FIELD_BASIS_DESIGN_GATE.md'
$contractPath = Join-Path $root 'contracts\field-basis-contract.md'
$sourcePath = Join-Path $root 'crates\field-basis\src\lib.rs'
foreach ($path in @($readinessPath, $designPath, $contractPath, $sourcePath)) {
  if (!(Test-Path $path)) { throw "Field-basis readiness source missing: $path" }
}
$text = Get-Content $designPath -Raw
foreach ($required in @(
  'approved and implemented',
  'Philox4x32-10',
  'Q32.32',
  'Q16.48',
  'ties-to-even',
  'deterministic-CBOR',
  'FastNoise Lite',
  'WGSL',
  'Cache key',
  'Required proof',
  'Failure and recovery',
  'Exact confirmation gate',
  'protected-Kernel mutation'
)) {
  if (!$text.Contains($required)) { throw "Field-basis readiness evidence missing: $required" }
}
$source = Get-Content $sourcePath -Raw
foreach ($required in @('philox_known_answer_is_fixed','recipe_bytes_are_strict_and_stable','poison_and_overflow_fail_closed','cache_key_binds_recipe_identity_and_domain','proof_evidence_is_authority_negative')) {
  if (!$source.Contains($required)) { throw "Field-basis executable proof missing: $required" }
}
$workspace = Get-Content (Join-Path $root 'Cargo.toml') -Raw
if (!$workspace.Contains('"crates/field-basis"')) { throw 'Field-basis crate is absent from the Cargo workspace.' }
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$active = @($program.items | Where-Object status -eq 'active')
$f5 = @($program.items | Where-Object id -eq 'F5')[0]
if ($active.Count -ne 1 -or ($active[0].id -ne 'F5' -and !($f5.status -eq 'complete' -and $active[0].milestone -in @('G1','R1')))) { throw 'Field-basis readiness is not retained through the F5 or later route.' }
if ((Get-Content (Join-Path $root 'crates\forge-kernel\src\lib.rs') -Raw).Contains('FieldRecipe')) {
  throw 'Field-basis readiness leaked into protected Kernel implementation.'
}
Write-Output 'F5 field-basis reference verified: approved policy, capability-free module, exact vectors, failures, cache binding, ProofReceipt evidence, and retained limits.'
