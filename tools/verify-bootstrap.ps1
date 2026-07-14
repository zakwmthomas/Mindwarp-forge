param([string]$Root)

$ErrorActionPreference = 'Stop'

$root = if ($Root) { [System.IO.Path]::GetFullPath($Root) } else { Split-Path -Parent $PSScriptRoot }
$manifestPath = Join-Path $root '.local\forge-bootstrap\MANIFEST.json'
if (Test-Path -LiteralPath $manifestPath) {
    $manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
    if ($manifest.schema_version -ne 1) { throw 'Unsupported Forge bootstrap manifest schema.' }
    if ($manifest.capture_state -ne 'running') { throw "Forge capture is $($manifest.capture_state); resolve it before mutable work." }
}

$required = @(
    'AGENTS.md',
    'context\bootstrap\START_HERE.md',
    'context\active\CURRENT_STATE.md',
    '.local\forge-bootstrap\START_HERE.md',
    '.local\forge-bootstrap\INDEX.md',
    '.local\forge-bootstrap\LEDGER_STATE.md',
    '.local\forge-bootstrap\OWNER_BRIEF.md',
    '.local\forge-bootstrap\MANIFEST.json',
    'context\bootstrap\BRIEFING.md',
    'governance\WORKER_FEEDBACK_BRIEF.md'
)
foreach ($relative in $required) {
    if (!(Test-Path -LiteralPath (Join-Path $root $relative))) {
        throw "Bootstrap is incomplete: missing $relative. Start or resume Forge capture, then retry."
    }
}

$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
$age = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() - [int64]$manifest.last_capture_unix
if ($age -lt 0 -or $age -gt 86400) { throw "Forge bootstrap is stale ($age seconds since capture). Refresh Forge capture before mutable work." }
foreach ($session in @($manifest.sessions)) {
    $path = Join-Path (Join-Path $root '.local\forge-bootstrap') $session.path
    if (!(Test-Path -LiteralPath $path)) { throw "Bootstrap transcript is missing: $($session.path)" }
    $actual = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne [string]$session.sha256) { throw "Bootstrap transcript hash mismatch: $($session.path)" }
}

& (Join-Path $PSScriptRoot 'verify-worker-batch-state.ps1')
if (!$?) { throw 'Canonical active checkpoint validation failed.' }
& (Join-Path $PSScriptRoot 'refresh-active-context.ps1')
if (!$?) { throw 'Active-context projection refresh failed.' }
& (Join-Path $PSScriptRoot 'refresh-worker-feedback.ps1')
if (!$?) { throw 'Worker feedback refresh failed.' }

$work = Get-Content (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$briefing = Get-Content (Join-Path $root 'context\bootstrap\BRIEFING.md') -Raw
if (!$briefing.Contains("Work package: **$($work.batch_id)**") -or !$briefing.Contains("Atlas milestone: **$($work.atlas_route.milestone) -")) {
    throw 'Task briefing route is stale relative to the canonical active checkpoint.'
}
& (Join-Path $PSScriptRoot 'refresh-active-context.ps1') -Check
if (!$?) { throw 'Generated active-context projection verification failed.' }

Write-Output "Bootstrap verified: $($manifest.sessions.Count) session(s), $($manifest.events) event(s), capture updated $age second(s) ago."
