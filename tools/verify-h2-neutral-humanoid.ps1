$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = 'C:\Users\zakwm\.cargo\bin\cargo.exe'
if (!(Test-Path -LiteralPath $cargo -PathType Leaf)) { throw 'Cargo is required for H2 verification.' }

$output = & $cargo run -q -p representation-contract --example h2_receipt 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H2 receipt example failed.' }
$receipt = $output -join "`n"
foreach ($required in @(
  'profile_id=forge-neutral-humanoid-structural-v1',
  'profile_fingerprint=c44adba610e2d70361d72cd9f78d1c3b7f56041a5574ef2f795570a72763d6e3',
  'scene_fingerprint=f94ebe29d2d8a5b9abfcd906412db4ad0da0a2e8e0947de7a422f51274ddac82',
  'reference_suite_fingerprint=1a4e25e81bc39327bc95975054846496b88c4510d378c0bef5f3ea1a5281939a',
  'joints=17', 'links=16', 'frames=2'
)) {
  if (!$receipt.Contains($required)) { throw "H2 receipt drifted or is incomplete: $required" }
}

$source = Get-Content -LiteralPath (Join-Path $root 'crates\representation-contract\src\neutral_humanoid.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\representation-contract.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\H2_NEUTRAL_HUMANOID_REPRESENTATION_RESULT.md') -Raw
foreach ($required in @(
  'StructuralProofCandidate', 'fixture_unit_not_metric',
  'structural_rest_equals_frame_zero_no_inverse_bind_matrices',
  'joint hierarchy contains a cycle', 'production_ready',
  'inverse-bind matrices', 'No recovered JSON'
)) {
  if (!$source.Contains($required) -and !$contract.Contains($required) -and !$result.Contains($required)) {
    throw "H2 authority or adversarial boundary is missing: $required"
  }
}
Write-Output 'H2 neutral humanoid verified: fixed profile/scene/suite fingerprints, bounded hierarchy, explicit coordinates and rest rule, and authority-negative claims retained.'
