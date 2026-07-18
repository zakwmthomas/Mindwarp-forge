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

    $current = Write-Fixture $program $checkpoint 'current-c4v'
    & $verifier -ProgramPath $current[0] -CheckpointPath $current[1] -SkipFixtureTests | Out-Null

    $futureProgram = $program | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $futureCheckpoint = $checkpoint | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $c4v = @($futureProgram.items | Where-Object id -eq 'C4V')[0]
    $c4v.state = 'promoted'; $c4v.status = 'complete'
    $gp3 = @($futureProgram.items | Where-Object id -eq 'GP3')[0]
    $gp3.state = 'executing'; $gp3.status = 'active'
    $futureCheckpoint.master_program_item = 'GP3'
    $future = Write-Fixture $futureProgram $futureCheckpoint 'future-gp3'
    & $verifier -ProgramPath $future[0] -CheckpointPath $future[1] -SkipFixtureTests | Out-Null

    $forgedCheckpoint = $checkpoint | ConvertTo-Json -Depth 20 | ConvertFrom-Json
    $forgedCheckpoint.master_program_item = 'GP4'
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
Write-Output 'G1 product-portfolio transition fixtures verified: current C4V, future aligned execution and forged checkpoint mismatch behave exactly.'
