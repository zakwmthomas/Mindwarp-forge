$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = 'C:\Users\zakwm\.cargo\bin\cargo.exe'
if (!(Test-Path -LiteralPath $cargo -PathType Leaf)) { throw 'Cargo is required for H7 readiness verification.' }

$output = & $cargo run -q -p humanoid-proof-chain --example h7_readiness 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H7 readiness example failed.' }
$receipt = $output -join "`n"
foreach ($required in @(
  'package_id=7b01d650258fe50b7cd59290a4a56e6df3a17271991dba313e29b6c0cf607619',
  'simulated_candidate_id=2d93d3ae31de08754852e27a3a04332d009b456141565be8e805a98eed8d6222',
  'candidate_name=engine-neutral-humanoid-proof-baseline-v1',
  'claim_count=6', 'non_claim_count=8',
  'authority=evidence_only_no_promotion', 'kernel_candidate_created=false'
)) {
  if (!$receipt.Contains($required)) { throw "H7 readiness receipt drifted or is incomplete: $required" }
}

& $cargo test -q -p humanoid-proof-chain
if ($LASTEXITCODE -ne 0) { throw 'H7 scope and lifecycle simulations failed.' }

$kernelOutput = & $cargo run -q -p humanoid-proof-chain-integration --example h7_kernel_candidate 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H7 disposable Kernel candidate proof failed.' }
$kernelReceipt = $kernelOutput -join "`n"
foreach ($required in @(
  'package_bytes=1512',
  'kernel_evidence_id=f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c',
  'kernel_candidate_id=c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433',
  'kernel_candidate_state=proposed', 'kernel_event_count=2',
  'authority=evidence_only_no_approval_or_promotion', 'persistent_forge_mutated=false'
)) {
  if (!$kernelReceipt.Contains($required)) { throw "H7 disposable Kernel receipt drifted or is incomplete: $required" }
}

& $cargo test -q -p humanoid-proof-chain-integration --test h7_candidate_package
if ($LASTEXITCODE -ne 0) { throw 'H7 candidate persistence and authority-negative integration failed.' }

& $cargo test -q -p humanoid-proof-chain-integration --example h7_candidate_admission
if ($LASTEXITCODE -ne 0) { throw 'H7 bounded candidate admission idempotency and state-drift shields failed.' }

& $cargo test -q -p humanoid-proof-chain-integration --example h7_candidate_approval
if ($LASTEXITCODE -ne 0) { throw 'H7 bounded owner-approved transition backup, idempotency, and no-promotion shields failed.' }

& $cargo test -q -p humanoid-proof-chain-integration --example h7_candidate_promotion
if ($LASTEXITCODE -ne 0) { throw 'H7 bounded owner-authorized promotion backup, idempotency, state, and no-application shields failed.' }

$source = Get-Content -LiteralPath (Join-Path $root 'crates\humanoid-proof-chain\src\promotion.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\humanoid-promotion-readiness-contract.md') -Raw
$readiness = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\H7_HUMANOID_PROMOTION_READINESS.md') -Raw
foreach ($required in @(
  'EvidencePackageOnlyNotKernelCandidate', 'no_generated_or_imported_surface_asset',
  'simulated_only_no_kernel_state_change', 'no_promoted_humanoid_proof_baseline',
  'separate owner actions', 'CandidateSuperseded', 'does not authorize'
)) {
  if (!$source.Contains($required) -and !$contract.Contains($required) -and !$readiness.Contains($required)) {
    throw "H7 scope, rollback, or authority boundary is missing: $required"
  }
}
Write-Output 'H7 readiness verified: narrow proof-baseline scope, exact H6/H5 binding, stable package/evidence/candidate IDs, hostile overclaim rejection, disposable proposal/approval/promotion integration, persistence recovery, no-application shield, and protected supersession pass.'
