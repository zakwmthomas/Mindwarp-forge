Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$oracle = & $python (Join-Path $PSScriptRoot 'prove-g1-c3-swept-aabb.py') | Out-String
if ($LASTEXITCODE -ne 0) { throw 'Swept-AABB oracle failed.' }
if ($oracle -notmatch '7d34a367c5cbfb418bf0caca084f1757e9c2539bc110d252c9e7ebf20ab4d43e') {
    throw 'Swept-AABB oracle checksum drift.'
}
Push-Location $root
try {
    cargo test -p swept-aabb-passage
    if ($LASTEXITCODE -ne 0) { throw 'Swept-AABB Rust tests failed.' }
} finally { Pop-Location }
Write-Host 'G1/C3 swept-AABB reference verified.'
