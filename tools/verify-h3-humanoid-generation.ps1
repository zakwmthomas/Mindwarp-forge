$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = 'C:\Users\zakwm\.cargo\bin\cargo.exe'
if (!(Test-Path -LiteralPath $cargo -PathType Leaf)) { throw 'Cargo is required for H3 verification.' }

$output = & $cargo run -q -p humanoid-generation --example h3_receipt 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H3 receipt example failed.' }
$receipt = $output -join "`n"
foreach ($required in @(
  'input_fingerprint=5667d387e4f7a0159fee99bab584c9481cc42b549535eaaec78de3a7b5796adf',
  'candidate_fingerprint=4d04df0dd58cdd8ecdb7c41e9dbde2dec1910b36b7d5643b2d254ef4b3c707fa',
  'replay_candidate_fingerprint=4d04df0dd58cdd8ecdb7c41e9dbde2dec1910b36b7d5643b2d254ef4b3c707fa',
  'joints=17', 'links=16', 'deterministic_replay=true', 'inputs_unchanged=true', 'capability_free=true'
)) {
  if (!$receipt.Contains($required)) { throw "H3 receipt drifted or is incomplete: $required" }
}

$source = Get-Content -LiteralPath (Join-Path $root 'crates\humanoid-generation\src\lib.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\humanoid-generation-contract.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\H3_NEUTRAL_HUMANOID_GENERATION_RESULT.md') -Raw
foreach ($required in @(
  'UnapprovedStructuralCandidate', 'ProtectedKernelMutation', 'ExternalExecutable',
  'declared output budget exhausted', 'pure-structural-projection-v1',
  'actual-pixel fitness receipts', 'does not replace integration proof'
)) {
  if (!$source.Contains($required) -and !$contract.Contains($required) -and !$result.Contains($required)) {
    throw "H3 proof or authority boundary is missing: $required"
  }
}
Write-Output 'H3 humanoid generation verified: exact P6/H2 input binding, deterministic 17-joint/16-link replay, unchanged inputs, bounded output, and capability-negative authority retained.'
