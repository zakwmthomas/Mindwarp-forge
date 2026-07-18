$ErrorActionPreference = 'Stop'
$fixture = Join-Path ([IO.Path]::GetTempPath()) ("forge-proof-system-projection-" + [guid]::NewGuid().ToString('N'))
try {
  New-Item -ItemType Directory -Force -Path (Join-Path $fixture 'docs\canonical-system') | Out-Null
  New-Item -ItemType Directory -Force -Path (Join-Path $fixture 'crates\forge-kernel\src') | Out-Null
  $registryPath = Join-Path $fixture 'docs\canonical-system\system-registry.json'
  '{"systems":[{"id":"alpha","layer":"game-canonical"},{"id":"forge-only","layer":"forge"}]}' | Set-Content -LiteralPath $registryPath -NoNewline
  & (Join-Path $PSScriptRoot 'refresh-proof-receipt-system-ids.ps1') -Root $fixture
  & (Join-Path $PSScriptRoot 'refresh-proof-receipt-system-ids.ps1') -Root $fixture -Check
  '{"systems":[{"id":"alpha","layer":"game-canonical"},{"id":"beta","layer":"game-canonical"}]}' | Set-Content -LiteralPath $registryPath -NoNewline
  $failedClosed = $false
  try { & (Join-Path $PSScriptRoot 'refresh-proof-receipt-system-ids.ps1') -Root $fixture -Check }
  catch { $failedClosed = $_.Exception.Message -like '*stale*' }
  if (!$failedClosed) { throw 'ProofReceipt projection did not reject registry drift.' }
  & (Join-Path $PSScriptRoot 'refresh-proof-receipt-system-ids.ps1') -Root $fixture
  & (Join-Path $PSScriptRoot 'refresh-proof-receipt-system-ids.ps1') -Root $fixture -Check
  Write-Output 'ProofReceipt system-ID projection fixtures verified: forge filtering, registry drift rejection, and refresh propagation pass.'
} finally {
  if (Test-Path $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
}
