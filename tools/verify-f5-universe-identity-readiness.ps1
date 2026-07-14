$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\UNIVERSE_IDENTITY_DESIGN_GATE.md'
$contractPath = Join-Path $root 'contracts\universe-identity-contract.md'
$sourcePath = Join-Path $root 'crates\universe-identity\src\lib.rs'
foreach ($requiredPath in @($path, $contractPath, $sourcePath)) {
  if (!(Test-Path $requiredPath)) { throw "Universe identity proof source is missing: $requiredPath" }
}
$text = Get-Content $path -Raw
foreach ($required in @(
  'approved for bounded reference implementation',
  'Independent revalidation',
  'logical identity is',
  'independent of generator version',
  'RFC 8949 deterministic CBOR',
  'Domain-separated SHA-256',
  'HKDF-SHA-256',
  'HMAC-SHA-256',
  'Fixed-vector manifest',
  'Failure and recovery matrix',
  'ProofReceipt'
)) {
  if (!$text.Contains($required)) { throw "Universe identity readiness evidence missing: $required" }
}
$contract = Get-Content $contractPath -Raw
foreach ($required in @('Universe Identity Contract v1', 'fail closed', 'bulk field-generation performance claim', 'ProofReceipt-compatible')) {
  if (!$contract.Contains($required)) { throw "Universe identity contract evidence missing: $required" }
}
$workspace = Get-Content (Join-Path $root 'Cargo.toml') -Raw
if (!$workspace.Contains('"crates/universe-identity"')) { throw 'Universe identity crate is absent from the Cargo workspace.' }
$boundaries = Get-Content (Join-Path $root 'governance\module-boundaries.json') -Raw | ConvertFrom-Json
$module = @($boundaries.modules | Where-Object id -eq 'universe-identity')
if ($module.Count -ne 1 -or $module[0].root -ne 'crates/universe-identity' -or @($module[0].dependencies).Count -ne 0) {
  throw 'Universe identity is not retained as a dependency-free declared module.'
}
$source = Get-Content $sourcePath -Raw
foreach ($required in @('fixed_vectors_are_byte_exact', 'strict_codec_rejects', 'migration_preserves_logical_identity', 'injected_collision_is_diagnostic', 'proof_evidence_is_bounded_and_authority_negative')) {
  if (!$source.Contains($required)) { throw "Universe identity executable proof missing: $required" }
}
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$active = @($program.items | Where-Object status -eq 'active')
$f5 = @($program.items | Where-Object id -eq 'F5')[0]
if ($active.Count -ne 1 -or ($active[0].id -ne 'F5' -and !($f5.status -eq 'complete' -and $active[0].milestone -in @('G1','R1')))) { throw 'Universe identity readiness is not retained through the F5 or later route.' }
if ((Get-Content (Join-Path $root 'crates\forge-kernel\src\lib.rs') -Raw).Contains('UniverseIdentity')) { throw 'Readiness work leaked into protected Kernel implementation.' }
Write-Output 'F5 universe identity reference verified: approved decision, strict contract, fixed vectors, migration/collision failures, ProofReceipt evidence, and authority boundaries retained.'
