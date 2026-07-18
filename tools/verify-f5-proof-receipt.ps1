$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$atlas = Get-Content (Join-Path $root 'docs\project-atlas\project-model.json') -Raw | ConvertFrom-Json
$registry = Get-Content (Join-Path $root 'docs\canonical-system\system-registry.json') -Raw | ConvertFrom-Json
$source = Get-Content (Join-Path $root 'crates\forge-kernel\src\persistence.rs') -Raw
$contracts = Get-Content (Join-Path $root 'crates\forge-kernel\src\contracts.rs') -Raw
$ui = Get-Content (Join-Path $root 'apps\forge-desktop\ui\src\main.ts') -Raw
& (Join-Path $root 'tools\refresh-proof-receipt-system-ids.ps1') -Check
$items = @{}; foreach ($item in $program.items) { $items[$item.id] = $item }

$f4Items = @($program.items | Where-Object milestone -eq 'F4')
$f5Items = @($program.items | Where-Object milestone -eq 'F5')
$g1Items = @($program.items | Where-Object milestone -eq 'G1')
$f5Active = $items['F5-OWNER-GATE'].status -eq 'complete' -and $items['F5'].status -eq 'active' -and @($f5Items | Where-Object status -eq 'active').Count -eq 1
$f5Complete = $items['F5-OWNER-GATE'].status -eq 'complete' -and $items['F5'].status -eq 'complete' -and @($f5Items | Where-Object status -ne 'complete').Count -eq 0 -and @($g1Items | Where-Object status -eq 'active').Count -eq 1
if (@($f4Items | Where-Object status -ne 'complete').Count -gt 0 -or (!$f5Active -and !$f5Complete)) { throw 'Master-program F5 milestone projection is inconsistent.' }
foreach ($relative in @('contracts\proof-receipt-projection-contract.md','docs\canonical-system\F5_PROOF_RECEIPT_DECISION.md')) {
  if (!(Test-Path (Join-Path $root $relative))) { throw "F5 ProofReceipt evidence missing: $relative" }
}
foreach ($token in @('pub struct ProofReceiptRecord','CREATE TABLE IF NOT EXISTS proof_receipts','proof_receipt_evidence_refs','canonical_proof_receipt_id','proof_receipts_fail_closed_and_survive_backup_reopen')) {
  if (!$source.Contains($token) -and !$contracts.Contains($token)) { throw "ProofReceipt implementation shield missing: $token" }
}
if (!$ui.Contains('records.proof_receipts') -or !$ui.Contains('Equivalence:')) { throw 'Reference Studio does not render ProofReceipt evidence.' }
if ((Get-Content (Join-Path $root 'crates\forge-kernel\src\lib.rs') -Raw).Contains('ProofReceipt')) { throw 'ProofReceipt leaked into the protected Kernel module.' }
Write-Output 'F5 ProofReceipt verified: versioned projection, canonical linkage, recovery, and read-only inspector shields are present.'
