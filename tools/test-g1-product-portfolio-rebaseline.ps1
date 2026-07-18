$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $root 'tools\verify-g1-product-portfolio-rebaseline.ps1'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-portfolio-transition-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temporary | Out-Null
try {
    function Write-Fixture($programValue, $checkpointValue, [string]$name) {
        $programPath = Join-Path $temporary ($name + '-program.json')
        $checkpointPath = Join-Path $temporary ($name + '-checkpoint.json')
        $programValue | ConvertTo-Json -Depth 20 | Set-Content -LiteralPath $programPath
        $checkpointValue | ConvertTo-Json -Depth 20 | Set-Content -LiteralPath $checkpointPath
        @($programPath, $checkpointPath)
    }

    $current = Write-Fixture $program $checkpoint 'current-route'
    & $verifier -ProgramPath $current[0] -CheckpointPath $current[1] -SkipFixtureTests | Out-Null

    $futureProgram = $program | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $futureCheckpoint = $checkpoint | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $active = @($futureProgram.items | Where-Object state -eq 'executing')
    if ($active.Count -ne 1) { throw 'Fixture requires one current executing item.' }
    $active[0].state = 'promoted'; $active[0].status = 'complete'
    $alternate = @($futureProgram.items | Where-Object { $_.id -ne $active[0].id -and $_.state -eq 'proposed' })[0]
    $alternate.state = 'executing'; $alternate.status = 'active'
    $futureCheckpoint.master_program_item = $alternate.id
    $future = Write-Fixture $futureProgram $futureCheckpoint 'future-alternate'
    & $verifier -ProgramPath $future[0] -CheckpointPath $future[1] -SkipFixtureTests | Out-Null

    $forgedCheckpoint = $checkpoint | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $currentExecuting = @($program.items | Where-Object state -eq 'executing')[0].id
    $forgedCheckpoint.master_program_item = @($program.items.id | Where-Object { $_ -ne $currentExecuting })[0]
    $forged = Write-Fixture $program $forgedCheckpoint 'forged-mismatch'
    $priorErrorAction = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    $output = & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $verifier -ProgramPath $forged[0] -CheckpointPath $forged[1] -SkipFixtureTests 2>&1
    $forgedExit = $LASTEXITCODE
    $ErrorActionPreference = $priorErrorAction
    if ($forgedExit -eq 0 -or ($output -join "`n") -notlike '*does not align with the canonical worker checkpoint*') {
        throw 'Forged checkpoint mismatch did not fail closed.'
    }
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}
Write-Output 'G1 product-portfolio transition fixtures verified: current route, alternate aligned execution and forged checkpoint mismatch behave exactly.'
