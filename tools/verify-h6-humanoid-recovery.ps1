$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = 'C:\Users\zakwm\.cargo\bin\cargo.exe'
if (!(Test-Path -LiteralPath $cargo -PathType Leaf)) { throw 'Cargo is required for H6 verification.' }

$output = & $cargo run -q -p humanoid-proof-chain --example h6_receipt 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H6 receipt example failed.' }
$receipt = $output -join "`n"
foreach ($required in @(
  'h5_receipt_id=5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520',
  'manifest_id=a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d',
  'stage_count=5', 'authority=evidence_only_no_promotion',
  'deterministic_replay=true', 'capability_free=true'
)) {
  if (!$receipt.Contains($required)) { throw "H6 receipt drifted or is incomplete: $required" }
}

& $cargo test -q -p humanoid-proof-chain
if ($LASTEXITCODE -ne 0) { throw 'H6 in-memory reconstruction and corruption fixtures failed.' }
& $cargo test -q -p humanoid-proof-chain-integration --test h6_humanoid_recovery
if ($LASTEXITCODE -ne 0) { throw 'H6 durable backup/reopen fixture failed.' }

$source = Get-Content -LiteralPath (Join-Path $root 'crates\humanoid-proof-chain\src\lib.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\humanoid-proof-chain-contract.md') -Raw
$readiness = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\H6_HUMANOID_REPRODUCTION_RECOVERY_READINESS.md') -Raw
foreach ($required in @(
  'CorruptionRejectedAndRebuilt', 'EvidenceOnlyNoPromotion',
  'f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051',
  'preview_target_only_no_asset_import', 'protected_kernel_mutation',
  'Five-claim coverage audit', 'never repaired in place'
)) {
  if (!$source.Contains($required) -and !$contract.Contains($required) -and !$readiness.Contains($required)) {
    throw "H6 recovery or authority boundary is missing: $required"
  }
}
Write-Output 'H6 humanoid recovery verified: exact H1-H5 identities, strict replay, hostile corruption rejection, known-good rebuild, retained Kernel evidence, ProofReceipt, and backup/reopen continuity pass.'
