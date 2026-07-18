[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet('route','claim','assert','release')]
    [string]$Mode,
    [string]$DatabasePath = (Join-Path ([Environment]::GetFolderPath('ApplicationData')) 'com.mindwarp.forge\forge.sqlite3'),
    [string]$WorkstreamId = 'forge-live-mainline',
    [string]$ProjectId = 'project-33deae303ed7d669d97c7fe3ab4507c15dc4e7aae54e3ac328b23e79f6a2f0fe',
    [string]$SessionId = $env:CODEX_THREAD_ID,
    [string]$CheckpointPath,
    [ValidateRange(1,1800)]
    [int]$LeaseSeconds = 1200,
    [string]$BinaryPath,
    [switch]$AllowLiveDatabaseMutation
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) {
    $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
}
if ([string]::IsNullOrWhiteSpace($BinaryPath)) {
    $BinaryPath = Join-Path $root 'target\debug\forge-federate.exe'
}
$liveDatabase = Join-Path ([Environment]::GetFolderPath('ApplicationData')) 'com.mindwarp.forge\forge.sqlite3'

if ([string]::IsNullOrWhiteSpace($SessionId)) {
    throw 'Writer lease requires CODEX_THREAD_ID or an explicit -SessionId.'
}
if (!(Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "Forge federation CLI is missing: $BinaryPath"
}
if (!(Test-Path -LiteralPath $DatabasePath -PathType Leaf)) {
    throw "Forge database is missing: $DatabasePath"
}
if (!(Test-Path -LiteralPath $CheckpointPath -PathType Leaf)) {
    throw "Forge checkpoint is missing: $CheckpointPath"
}

$databaseFull = [IO.Path]::GetFullPath($DatabasePath)
$liveFull = [IO.Path]::GetFullPath($liveDatabase)
if ($Mode -in @('route','claim','release') -and
    $databaseFull.Equals($liveFull,[StringComparison]::OrdinalIgnoreCase) -and
    !$AllowLiveDatabaseMutation) {
    throw 'Live Forge lease mutation requires the separately verified -AllowLiveDatabaseMutation gate.'
}

$checkpointHash = (Get-FileHash -LiteralPath $CheckpointPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($Mode -eq 'route') {
    $routePath = Join-Path ([IO.Path]::GetTempPath()) ('forge-session-route-' + [guid]::NewGuid().ToString('N') + '.json')
    try {
        [ordered]@{
            schema_version = 1
            session_id = $SessionId
            revision = 1
            state = 'routed'
            candidate_project_ids = @($ProjectId)
            project_id = $ProjectId
            workstream_id = $WorkstreamId
            method = 'registered_repository_root'
            confidence = 100
            evidence_ids = @('repository-root:mindwarp-forge')
        } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $routePath
        $output = & $BinaryPath route-session $DatabasePath $routePath 2>&1
        if ($LASTEXITCODE -ne 0) { throw ($output -join "`n") }
        $output
        return
    } finally {
        if (Test-Path -LiteralPath $routePath) { Remove-Item -LiteralPath $routePath -Force }
    }
}
$arguments = switch ($Mode) {
    'claim' { @('claim-workstream-writer',$DatabasePath,$WorkstreamId,$SessionId,$checkpointHash,[string]$LeaseSeconds) }
    'assert' { @('assert-workstream-writer',$DatabasePath,$WorkstreamId,$SessionId,$checkpointHash) }
    'release' { @('release-workstream-writer',$DatabasePath,$WorkstreamId,$SessionId) }
}

$output = & $BinaryPath @arguments 2>&1
if ($LASTEXITCODE -ne 0) {
    throw ($output -join "`n")
}
$output
