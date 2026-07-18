$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$verifyPath = Join-Path $PSScriptRoot 'verify.ps1'
$launcherPath = Join-Path $PSScriptRoot 'invoke-measured-run.ps1'
$verifySource = Get-Content -LiteralPath $verifyPath -Raw
$launcherSource = Get-Content -LiteralPath $launcherPath -Raw

foreach ($token in @('RegisteredRunId','RegisteredInvocationId','forge-full-gate-v1','Full Forge verification must use the registered launcher')) {
    if (!$verifySource.Contains($token)) { throw "Full-gate entry guard is missing: $token" }
}
foreach ($token in @('-RegisteredRunId','-RegisteredInvocationId')) {
    if (!$launcherSource.Contains($token)) { throw "Registered launcher binding is missing: $token" }
}

$savedPreference = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
$directOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $verifyPath 2>&1
$directExit = $LASTEXITCODE
$ErrorActionPreference = $savedPreference
if ($directExit -eq 0 -or ($directOutput -join "`n") -notlike '*Full Forge verification must use the registered launcher*') {
    throw 'Direct production verify.ps1 invocation did not fail before expensive verification.'
}
if (($directOutput -join "`n") -like '*Atlas verified*') {
    throw 'Direct production verify.ps1 invocation began expensive verification before rejection.'
}

$fixture = Join-Path ([IO.Path]::GetTempPath()) ('forge-registered-full-gate-' + [guid]::NewGuid().ToString('N'))
try {
    New-Item -ItemType Directory -Path (Join-Path $fixture 'governance'),(Join-Path $fixture 'context\active'),(Join-Path $fixture 'tools') -Force | Out-Null
    @{
        schema_version = 1
        runs = @(@{
            id = 'forge-full-gate-v1'
            timeout_seconds = 20
            working_directory = '.'
            runner = 'powershell'
            arguments = @('-NoProfile','-ExecutionPolicy','Bypass','-File','tools/verify.ps1')
            module = 'forge-dashboard'
            verification_scope = 'complete'
        })
    } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $fixture 'governance\routine-run-registry.json') -Encoding utf8
    @{batch_id='fixture-full-gate'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $fixture 'context\active\WORKER_BATCH_STATE.json') -Encoding utf8
    @'
param([string]$RegisteredRunId,[string]$RegisteredInvocationId)
if ($RegisteredRunId -ne 'forge-full-gate-v1' -or $RegisteredInvocationId -notmatch '^run-[0-9a-f]{32}$') { exit 31 }
exit 0
'@ | Set-Content -LiteralPath (Join-Path $fixture 'tools\verify.ps1') -Encoding utf8

    & $launcherPath -RunId forge-full-gate-v1 -ProjectRoot $fixture | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Canonical registered full-gate launcher did not pass its invocation identity.' }
    $receipts = @(Get-ChildItem -LiteralPath (Join-Path $fixture '.local\forge-metrics\inbox') -Filter 'run-*.json')
    if ($receipts.Count -ne 1) { throw 'Canonical registered full-gate launcher did not emit exactly one receipt.' }
    $receipt = Get-Content -LiteralPath $receipts[0].FullName -Raw | ConvertFrom-Json
    if ($receipt.outcome -ne 'passed' -or @($receipt.evidence_ids) -notcontains 'run-definition:forge-full-gate-v1') {
        throw 'Canonical registered full-gate receipt lost its public run-definition binding.'
    }
} finally {
    if (Test-Path -LiteralPath $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
}

Write-Output 'Registered full-gate launcher verified: direct production invocation fails before work; canonical launch passes with one non-secret receipt.'
