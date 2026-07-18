Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$first = & $python (Join-Path $PSScriptRoot 'prove-g1-c3-interval-optical-continuation.py') | Out-String
if ($LASTEXITCODE -ne 0) { throw 'Interval optical continuation counterexample oracle failed.' }
$second = & $python (Join-Path $PSScriptRoot 'prove-g1-c3-interval-optical-continuation.py') | Out-String
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Interval optical continuation oracle is nondeterministic.' }
if ($first -notmatch '91cc00cbcb97e9a8b8157edbaacc21fd05826bf0213d1515d126c702b9b5d6fd') {
    throw 'Interval optical continuation oracle checksum drift.'
}
Write-Host 'G1/C3 interval optical continuation counterexamples verified.'
