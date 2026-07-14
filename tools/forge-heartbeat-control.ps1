[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('status', 'pause', 'resume')]
    [string]$Mode,

    [string]$AutomationPath = (Join-Path $env:USERPROFILE '.codex\automations\forge-heartbeat\automation.toml')
)

$ErrorActionPreference = 'Stop'

if (!(Test-Path -LiteralPath $AutomationPath)) {
    throw "Forge heartbeat automation is missing: $AutomationPath"
}

$text = Get-Content -LiteralPath $AutomationPath -Raw
if ($text -notmatch '(?m)^id\s*=\s*"forge-heartbeat"\s*$') {
    throw 'Refusing to control a non-Forge automation record.'
}

$statusMatch = [regex]::Match($text, '(?m)^status\s*=\s*"(?<value>[^"]+)"\s*$')
if (!$statusMatch.Success) {
    throw 'Forge heartbeat automation has no readable status.'
}

if ($Mode -eq 'status') {
    Write-Output $statusMatch.Groups['value'].Value
    exit 0
}

$target = if ($Mode -eq 'pause') { 'PAUSED' } else { 'ACTIVE' }
$updated = [regex]::Replace(
    $text,
    '(?m)^status\s*=\s*"[^"]+"\s*$',
    "status = `"$target`"",
    1
)
$timestamp = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
if ($updated -match '(?m)^updated_at\s*=') {
    $updated = [regex]::Replace($updated, '(?m)^updated_at\s*=\s*\d+\s*$', "updated_at = $timestamp", 1)
}

$directory = Split-Path -Parent $AutomationPath
$temporary = Join-Path $directory ('.automation-' + [guid]::NewGuid().ToString('N') + '.tmp')
try {
    [System.IO.File]::WriteAllText($temporary, $updated, [System.Text.UTF8Encoding]::new($false))
    Move-Item -LiteralPath $temporary -Destination $AutomationPath -Force
} finally {
    if (Test-Path -LiteralPath $temporary) {
        Remove-Item -LiteralPath $temporary -Force
    }
}

$verified = Get-Content -LiteralPath $AutomationPath -Raw
if ($verified -notmatch "(?m)^status\s*=\s*`"$target`"\s*$") {
    throw "Forge heartbeat did not enter $target state."
}

Write-Output "Forge heartbeat: $target"
