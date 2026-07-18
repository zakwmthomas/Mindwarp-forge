param([switch]$Check, [string]$Root)
$ErrorActionPreference = 'Stop'
$root = if ($Root) { (Resolve-Path $Root).Path } else { Split-Path -Parent $PSScriptRoot }
$registryPath = Join-Path $root 'docs\canonical-system\system-registry.json'
$target = Join-Path $root 'crates\forge-kernel\src\generated_proof_receipt_system_ids.rs'
$registry = Get-Content $registryPath -Raw | ConvertFrom-Json
$ids = @($registry.systems | Where-Object layer -eq 'game-canonical' | ForEach-Object id)
if ($ids.Count -eq 0 -or @($ids | Sort-Object -Unique).Count -ne $ids.Count) {
  throw 'Canonical game-system IDs are empty or duplicated.'
}
$builder = [Text.StringBuilder]::new()
[void]$builder.AppendLine('// Generated from docs/canonical-system/system-registry.json. Do not edit.')
[void]$builder.AppendLine('const PROOF_RECEIPT_SYSTEM_IDS: &[&str] = &[')
foreach ($id in $ids) { [void]$builder.AppendLine(('    "{0}",' -f $id)) }
[void]$builder.AppendLine('];')
$expected = $builder.ToString().Replace("`r`n", "`n")
if ($Check) {
  if (!(Test-Path $target) -or (Get-Content $target -Raw).Replace("`r`n", "`n") -ne $expected) {
    throw 'Generated ProofReceipt system IDs are stale. Run tools\refresh-proof-receipt-system-ids.ps1.'
  }
  Write-Output "ProofReceipt system-ID projection verified: $($ids.Count) canonical game systems."
} else {
  [IO.File]::WriteAllText($target, $expected, [Text.UTF8Encoding]::new($false))
  Write-Output "ProofReceipt system-ID projection refreshed: $($ids.Count) canonical game systems."
}
