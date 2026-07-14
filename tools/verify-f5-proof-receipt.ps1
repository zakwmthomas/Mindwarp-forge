$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$atlas = Get-Content (Join-Path $root 'docs\project-atlas\project-model.json') -Raw | ConvertFrom-Json
$registry = Get-Content (Join-Path $root 'docs\canonical-system\system-registry.json') -Raw | ConvertFrom-Json
$source = Get-Content (Join-Path $root 'crates\forge-kernel\src\persistence.rs') -Raw
$contracts = Get-Content (Join-Path $root 'crates\forge-kernel\src\contracts.rs') -Raw
$ui = Get-Content (Join-Path $root 'apps\forge-desktop\ui\src\main.ts') -Raw
$items = @{}; foreach ($item in $program.items) { $items[$item.id] = $item }

if ($items['F5-OWNER-GATE'].status -ne 'complete' -or $items['F5'].status -ne 'active') { throw 'F5 owner transition is not active.' }
$f4Items = @($program.items | Where-Object milestone -eq 'F4')
$f5Items = @($program.items | Where-Object milestone -eq 'F5')
if (@($f4Items | Where-Object status -ne 'complete').Count -gt 0 -or @($f5Items | Where-Object status -eq 'active').Count -ne 1) { throw 'Master-program F5 milestone projection is inconsistent.' }
foreach ($relative in @('contracts\proof-receipt-projection-contract.md','docs\canonical-system\F5_PROOF_RECEIPT_DECISION.md')) {
  if (!(Test-Path (Join-Path $root $relative))) { throw "F5 ProofReceipt evidence missing: $relative" }
}
foreach ($token in @('pub struct ProofReceiptRecord','CREATE TABLE IF NOT EXISTS proof_receipts','proof_receipt_evidence_refs','canonical_proof_receipt_id','proof_receipts_fail_closed_and_survive_backup_reopen')) {
  if (!$source.Contains($token) -and !$contracts.Contains($token)) { throw "ProofReceipt implementation shield missing: $token" }
}
foreach ($system in @($registry.systems | Where-Object layer -eq 'game-canonical')) {
  if (!$source.Contains(('"' + $system.id + '"'))) { throw "ProofReceipt schema lacks canonical system: $($system.id)" }
}
if (!$ui.Contains('records.proof_receipts') -or !$ui.Contains('Equivalence:')) { throw 'Reference Studio does not render ProofReceipt evidence.' }
if ((Get-Content (Join-Path $root 'crates\forge-kernel\src\lib.rs') -Raw).Contains('ProofReceipt')) { throw 'ProofReceipt leaked into the protected Kernel module.' }
Write-Output 'F5 ProofReceipt verified: versioned projection, canonical linkage, recovery, and read-only inspector shields are present.'
