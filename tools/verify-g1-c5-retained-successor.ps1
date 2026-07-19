param([string]$Root)
$ErrorActionPreference = 'Stop'
if ([string]::IsNullOrWhiteSpace($Root)) { $Root = Split-Path -Parent $PSScriptRoot }
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python' }
& $python (Join-Path $PSScriptRoot 'verify-g1-c5-retained-successor.py') --root $Root
if ($LASTEXITCODE -ne 0) { throw 'C5 retained-successor classification failed.' }
