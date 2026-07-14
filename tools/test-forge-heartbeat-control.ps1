$ErrorActionPreference = 'Stop'
$tool = Join-Path $PSScriptRoot 'forge-heartbeat-control.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-heartbeat-control-$PID-$([guid]::NewGuid().ToString('N'))"
$automation = Join-Path $temp 'automation.toml'

try {
    New-Item -ItemType Directory -Path $temp -Force | Out-Null
    @'
version = 1
id = "forge-heartbeat"
kind = "heartbeat"
name = "Forge heartbeat"
prompt = "retain this prompt exactly"
status = "ACTIVE"
updated_at = 1
'@ | Set-Content -LiteralPath $automation -Encoding utf8

    $initial = & $tool -Mode status -AutomationPath $automation
    if ($initial -ne 'ACTIVE') { throw 'Initial heartbeat status was not read.' }

    & $tool -Mode pause -AutomationPath $automation | Out-Null
    if ((& $tool -Mode status -AutomationPath $automation) -ne 'PAUSED') { throw 'Heartbeat did not pause.' }

    & $tool -Mode pause -AutomationPath $automation | Out-Null
    if ((& $tool -Mode status -AutomationPath $automation) -ne 'PAUSED') { throw 'Heartbeat pause was not idempotent.' }

    & $tool -Mode resume -AutomationPath $automation | Out-Null
    if ((& $tool -Mode status -AutomationPath $automation) -ne 'ACTIVE') { throw 'Heartbeat did not resume.' }

    $final = Get-Content -LiteralPath $automation -Raw
    if ($final -notmatch 'retain this prompt exactly') { throw 'Heartbeat control rewrote the automation prompt.' }
    if ($final -notmatch '(?m)^updated_at\s*=\s*\d{10,}$') { throw 'Heartbeat control did not refresh updated_at.' }

    $foreign = Join-Path $temp 'foreign.toml'
    $final.Replace('id = "forge-heartbeat"', 'id = "other"') | Set-Content -LiteralPath $foreign -Encoding utf8
    $rejected = $false
    try { & $tool -Mode pause -AutomationPath $foreign | Out-Null } catch { $rejected = $true }
    if (!$rejected) { throw 'Heartbeat control accepted a foreign automation.' }

    Write-Output 'Forge heartbeat control verified: status, atomic pause, idempotency, resume, prompt retention, timestamp, and foreign-record rejection.'
} finally {
    if (Test-Path -LiteralPath $temp) { Remove-Item -LiteralPath $temp -Recurse -Force }
}
