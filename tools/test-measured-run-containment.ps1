$ErrorActionPreference = 'Stop'

$tool = Join-Path $PSScriptRoot 'invoke-measured-run.ps1'
$fixtureParent = Join-Path ([IO.Path]::GetTempPath()) ('forge-measured-containment-' + [guid]::NewGuid().ToString('N'))
$root = Join-Path $fixtureParent 'forge'
$sibling = Join-Path $fixtureParent 'forge-evil'
$outside = Join-Path $fixtureParent 'outside'
$junction = Join-Path $root 'linked-outside'
try {
    New-Item -ItemType Directory -Path (Join-Path $root 'governance'),(Join-Path $root 'context\active'),$sibling,$outside -Force | Out-Null
    @{
        schema_version = 1
        runs = @(@{
            id = 'escape'
            timeout_seconds = 10
            working_directory = '..\forge-evil'
            runner = 'powershell'
            arguments = @('-NoProfile', '-Command', 'exit 0')
            module = 'fixture'
            verification_scope = 'containment'
        })
    } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $root 'governance\routine-run-registry.json') -Encoding utf8
    @{batch_id='fixture'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Encoding utf8

    $rejected = $false
    try { & $tool -RunId escape -ProjectRoot $root | Out-Null } catch {
        $rejected = $_.Exception.Message -eq 'Routine-run working directory escapes the Forge repository.'
    }
    if (!$rejected) { throw 'A separator-confusable sibling working directory escaped the measured-run root.' }
    if (Test-Path -LiteralPath (Join-Path $root '.local\forge-metrics\inbox')) {
        throw 'Rejected measured run emitted a receipt.'
    }

    New-Item -ItemType Junction -Path $junction -Target $outside | Out-Null
    $registry = Get-Content -LiteralPath (Join-Path $root 'governance\routine-run-registry.json') -Raw | ConvertFrom-Json
    $registry.runs[0].working_directory = 'linked-outside'
    $registry | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $root 'governance\routine-run-registry.json') -Encoding utf8
    $junctionRejected = $false
    try { & $tool -RunId escape -ProjectRoot $root | Out-Null } catch {
        $junctionRejected = $_.Exception.Message -eq 'Routine-run working directory crosses a reparse point.'
    }
    if (!$junctionRejected) { throw 'An in-repository junction escaped the measured-run root.' }
    Remove-Item -LiteralPath $junction -Force
    Write-Output 'Measured-run containment verified: sibling-prefix and reparse-point escapes fail before execution or receipt emission.'
} finally {
    if (Test-Path -LiteralPath $junction) { Remove-Item -LiteralPath $junction -Force }
    if (Test-Path -LiteralPath $fixtureParent) { Remove-Item -LiteralPath $fixtureParent -Recurse -Force }
}
