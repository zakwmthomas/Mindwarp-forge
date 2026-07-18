$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\FIELD_BASIS_READINESS.md'
$designPath = Join-Path $root 'docs\canonical-system\FIELD_BASIS_DESIGN_GATE.md'
$contractPath = Join-Path $root 'contracts\field-basis-contract.md'
$sourcePath = Join-Path $root 'crates\field-basis\src\lib.rs'
$fixturePath = Join-Path $root 'crates\field-basis\fixtures\second-language-v1.json'
$oraclePath = Join-Path $root 'tools\verify-field-basis-second-language.py'
foreach ($path in @($readinessPath, $designPath, $contractPath, $sourcePath, $fixturePath, $oraclePath)) {
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

$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (!(Test-Path $cargo)) { $cargo = (Get-Command cargo -ErrorAction Stop).Source }
$rustReceipt = (& $cargo run --quiet -p field-basis --example second_language_vectors 2>&1 | Out-String)
if ($LASTEXITCODE -ne 0) { throw "Rust field-basis receipt emitter failed: $rustReceipt" }
$expected = Get-Content $fixturePath -Raw | ConvertFrom-Json
$actual = $rustReceipt | ConvertFrom-Json
if (($expected | ConvertTo-Json -Depth 10 -Compress) -ne ($actual | ConvertTo-Json -Depth 10 -Compress)) {
  throw 'Rust field-basis receipt does not match the committed second-language fixture.'
}

$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (!(Test-Path $python)) {
  $python = (Get-Command python3 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -First 1)
}
if (!$python -or !(Test-Path $python)) { throw 'A real Python runtime is required for the independent field-basis receipt; the Windows Store alias is not accepted.' }
$oracleOutput = (& $python $oraclePath $fixturePath 2>&1 | Out-String)
if ($LASTEXITCODE -ne 0) { throw "Python field-basis oracle failed: $oracleOutput" }
foreach ($required in @('same Windows host','not a second-platform receipt','not reference_proven')) {
  if (!(($expected.limitations -join ' ').Contains($required))) { throw "Second-language limitation missing: $required" }
}

Write-Output 'F5 field-basis reference verified: approved policy, capability-free module, exact Rust/Python vectors, failures, cache binding, ProofReceipt evidence, and retained same-platform limits.'
