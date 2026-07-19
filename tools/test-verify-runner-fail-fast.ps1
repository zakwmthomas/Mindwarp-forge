$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$verifyPath = Join-Path $PSScriptRoot 'verify.ps1'
$runnerPath = Join-Path $PSScriptRoot 'verification-runner.ps1'

if (!(Test-Path -LiteralPath $runnerPath -PathType Leaf)) {
    throw 'The shared verification runner is missing.'
}

. $runnerPath

$fixture = Join-Path ([IO.Path]::GetTempPath()) ('forge-verifier-runner-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $fixture | Out-Null
try {
    Set-Content -LiteralPath (Join-Path $fixture 'pass.ps1') -Encoding utf8 -Value "Write-Output 'pass'; exit 0"
    Set-Content -LiteralPath (Join-Path $fixture 'fail.ps1') -Encoding utf8 -Value "Write-Output 'fail'; exit 23"

    Invoke-ForgeVerifier -ScriptRoot $fixture -ScriptName 'pass.ps1'
    $failedClosed = $false
    try {
        Invoke-ForgeVerifier -ScriptRoot $fixture -ScriptName 'fail.ps1'
    } catch {
        $failedClosed = $_.Exception.Message -match 'fail\.ps1' -and $_.Exception.Message -match '23'
    }
    if (!$failedClosed) { throw 'A non-zero child verifier did not fail closed with its script name and exit code.' }
} finally {
    Remove-Item -LiteralPath $fixture -Recurse -Force -ErrorAction SilentlyContinue
}

$source = Get-Content -LiteralPath $verifyPath -Raw
if (!$source.Contains(". (Join-Path `$PSScriptRoot 'verification-runner.ps1')")) {
    throw 'verify.ps1 does not load the shared verification runner.'
}
$rawInvocations = [regex]::Matches($source, '(?m)^& \(Join-Path \$PSScriptRoot ''[^'']+\.ps1''\)')
if ($rawInvocations.Count -ne 0) {
    throw "verify.ps1 retains $($rawInvocations.Count) unchecked child-script invocation(s)."
}

& (Join-Path $PSScriptRoot 'test-g1-c4-retained-successor-adapter.ps1')
if (!$?) { throw 'Retained C4 successor adapter fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-g1-c5-current-successor-route.ps1')
if (!$?) { throw 'C5 current successor route fixtures failed.' }

Write-Output 'Verification runner verified: child scripts fail immediately with exact exit evidence.'
