[CmdletBinding()]
param(
    [ValidateRange(1, 30)]
    [int]$WaitSeconds = 4,
    [ValidateRange(1, 60)]
    [int]$MaxCaptureAgeSeconds = 12
)

$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $root '.local\forge-bootstrap\MANIFEST.json'
$workspaceReleaseApp = Join-Path $root 'target\release\forge-desktop.exe'
$workspaceDebugApp = Join-Path $root 'target\debug\forge-desktop.exe'
$legacyReleaseApp = Join-Path $root 'apps\forge-desktop\src-tauri\target\release\forge-desktop.exe'
$legacyDebugApp = Join-Path $root 'apps\forge-desktop\src-tauri\target\debug\forge-desktop.exe'

function Get-CaptureManifest {
    if (!(Test-Path -LiteralPath $manifestPath)) {
        return $null
    }
    return Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
}

function Get-CaptureAgeSeconds($manifest) {
    return [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() - [int64]$manifest.last_capture_unix
}

$manifest = Get-CaptureManifest
if ($manifest -and $manifest.capture_state -eq 'paused') {
    throw 'Forge capture is paused. It must be explicitly resumed; this safety gate will not override that decision.'
}

$forgeProcess = Get-Process -Name 'forge-desktop' -ErrorAction SilentlyContinue | Select-Object -First 1
if (!$forgeProcess) {
    $app = @($workspaceReleaseApp, $workspaceDebugApp, $legacyReleaseApp, $legacyDebugApp) |
        Where-Object { Test-Path -LiteralPath $_ } |
        ForEach-Object { Get-Item -LiteralPath $_ } |
        Sort-Object LastWriteTimeUtc -Descending |
        Select-Object -First 1 -ExpandProperty FullName
    if (!$app) {
        throw 'Forge Desktop is not running and no built Forge executable was found. Build or open Forge Desktop, then retry.'
    }
    Start-Process -FilePath $app | Out-Null
}

# The Forge desktop watcher scans local Codex sessions every two seconds. Waiting
# here is bounded and lets a just-created task become part of generated evidence.
Start-Sleep -Seconds $WaitSeconds

$manifest = Get-CaptureManifest
if (!$manifest) {
    throw 'Forge did not generate a bootstrap manifest after the refresh wait.'
}
if ($manifest.capture_state -ne 'running') {
    throw "Forge capture is $($manifest.capture_state); resolve it before project work."
}

$age = Get-CaptureAgeSeconds $manifest
if ($age -lt 0 -or $age -gt $MaxCaptureAgeSeconds) {
    throw "Forge capture did not become current ($age seconds old). Do not continue with mutable work."
}

& (Join-Path $PSScriptRoot 'verify-bootstrap.ps1')
if (!$?) {
    throw 'Forge bootstrap verification failed after context refresh.'
}

Write-Output "Forge context gate passed: capture is running and $age second(s) old."
