$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-writer-lease-test-' + [guid]::NewGuid().ToString('N'))
$target = Join-Path $temporary 'target'
$database = Join-Path $temporary 'fixture.sqlite3'
$checkpoint = Join-Path $temporary 'checkpoint.json'
$projectRecord = Join-Path $root 'governance\federation\projects\mindwarp-forge.json'
$workstreamRecord = Join-Path $temporary 'workstream.json'
$projectId = 'project-33deae303ed7d669d97c7fe3ab4507c15dc4e7aae54e3ac328b23e79f6a2f0fe'

function Expect-Failure([scriptblock]$Action,[string]$Message) {
    $failed = $false
    try { & $Action | Out-Null } catch { $failed = $true }
    if (!$failed) { throw $Message }
}

New-Item -ItemType Directory -Path $temporary | Out-Null
try {
    $env:CARGO_TARGET_DIR = $target
    & $cargo build -p forge-kernel --bin forge-federate
    if ($LASTEXITCODE -ne 0) { throw 'forge-federate build failed.' }
    $binary = Join-Path $target 'debug\forge-federate.exe'

    '{"checkpoint":"fixture-a"}' | Set-Content -LiteralPath $checkpoint -NoNewline
    [ordered]@{
        schema_version=1; id='forge-live-mainline'; project_id=$projectId
        title='Forge live mainline writer'; stage='implementation'; status='active'
        authority_lane='delegated'; dependencies=@(); blockers=@()
        checkpoint_uri='context/active/WORKER_BATCH_STATE.json'
        next_action='Continue only through the sole live Forge writer.'
        revision=1; lease=$null; evidence_ids=@('evidence:writer-lease-fixture')
    } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $workstreamRecord
    & $binary register-project $database $projectRecord | Out-Null
    & $binary register-workstream $database $workstreamRecord | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Disposable federation fixture registration failed.' }
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode route -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a | Out-Null
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode route -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b | Out-Null
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode route -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a | Out-Null

    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode claim -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a -LeaseSeconds 60 | Out-Null
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a | Out-Null
    Expect-Failure { & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode claim -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b -LeaseSeconds 60 } 'A second live writer acquired the lease.'
    Expect-Failure { & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b } 'A read-only session passed writer assertion.'

    '{"checkpoint":"fixture-b"}' | Set-Content -LiteralPath $checkpoint -NoNewline
    Expect-Failure { & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a } 'Checkpoint drift did not invalidate the writer lease.'
    '{"checkpoint":"fixture-a"}' | Set-Content -LiteralPath $checkpoint -NoNewline

    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode release -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-a | Out-Null
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode claim -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b -LeaseSeconds 60 | Out-Null
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b | Out-Null
    Expect-Failure { & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode claim -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-c -LeaseSeconds 60 } 'An unrouted session acquired a writer lease.'
    Expect-Failure { & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode claim -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b -LeaseSeconds 1801 } 'An overlong writer lease was accepted.'

    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode release -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId session-b | Out-Null
    $checkpointHash = (Get-FileHash -LiteralPath $checkpoint -Algorithm SHA256).Hash.ToLowerInvariant()
    $processA = Start-Process -FilePath $binary -ArgumentList @('claim-workstream-writer',$database,'forge-live-mainline','session-a',$checkpointHash,'60') -PassThru -WindowStyle Hidden
    $processB = Start-Process -FilePath $binary -ArgumentList @('claim-workstream-writer',$database,'forge-live-mainline','session-b',$checkpointHash,'60') -PassThru -WindowStyle Hidden
    $processA.WaitForExit()
    $processB.WaitForExit()
    $successfulClaims = @(@($processA.ExitCode,$processB.ExitCode) | Where-Object { $_ -eq 0 })
    if ($successfulClaims.Count -ne 1) { throw "Concurrent claims did not elect exactly one writer: exit codes $($processA.ExitCode), $($processB.ExitCode)." }
    $winner = if ($processA.ExitCode -eq 0) { 'session-a' } else { 'session-b' }
    & (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -DatabasePath $database -CheckpointPath $checkpoint -BinaryPath $binary -SessionId $winner | Out-Null

    Write-Output 'Forge writer lease verified: routed sole-writer claim, simultaneous conflict election, checkpoint drift, release, takeover, missing route and TTL bounds pass.'
} finally {
    Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}
