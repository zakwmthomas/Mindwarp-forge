param([string]$Root = '')
$ErrorActionPreference = 'Stop'
$forgeRoot = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($Root)) { $Root = $forgeRoot }
$binary = Join-Path $forgeRoot 'target\debug\forge-storage.exe'
if (Test-Path -LiteralPath $binary) {
  & $binary cache-plan $Root
} else {
  Push-Location $forgeRoot
  try { & cargo run --quiet -p forge-kernel --bin forge-storage -- cache-plan $Root }
  finally { Pop-Location }
}
if ($LASTEXITCODE -ne 0) { throw 'Cache cleanup preview failed.' }
