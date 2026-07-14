$ErrorActionPreference = 'Stop'
$tool = Join-Path $PSScriptRoot 'verify-bootstrap.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-bootstrap-read-only-$PID-$([guid]::NewGuid().ToString('N'))"
$manifestDirectory = Join-Path $temp '.local\forge-bootstrap'

try {
    New-Item -ItemType Directory -Path $manifestDirectory -Force | Out-Null
    @{
        schema_version = 1
        capture_state = 'paused'
        last_capture_unix = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
        sessions = @()
        events = 0
    } | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $manifestDirectory 'MANIFEST.json') -Encoding utf8

    $sentinels = @(
        'context\active\CURRENT_STATE.md',
        'context\bootstrap\BRIEFING.md',
        'governance\WORKER_FEEDBACK_BRIEF.md'
    )
    foreach ($relative in $sentinels) {
        $path = Join-Path $temp $relative
        New-Item -ItemType Directory -Path (Split-Path -Parent $path) -Force | Out-Null
        Set-Content -LiteralPath $path -Value "unchanged:$relative" -NoNewline
    }
    $before = @{}
    Get-ChildItem -LiteralPath $temp -Recurse -File | ForEach-Object {
        $relative = $_.FullName.Substring($temp.Length).TrimStart('\', '/')
        $before[$relative] = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
    }

    $failure = $null
    try { & $tool -Root $temp | Out-Null } catch { $failure = $_ }
    if ($null -eq $failure) { throw 'Paused bootstrap was accepted.' }
    if ($failure.Exception.Message -ne 'Forge capture is paused; resolve it before mutable work.') {
        throw "Paused bootstrap returned the wrong failure: $($failure.Exception.Message)"
    }

    $afterFiles = @(Get-ChildItem -LiteralPath $temp -Recurse -File)
    if ($afterFiles.Count -ne $before.Count) { throw 'Paused bootstrap changed the fixture file set.' }
    foreach ($file in $afterFiles) {
        $relative = $file.FullName.Substring($temp.Length).TrimStart('\', '/')
        if (-not $before.ContainsKey($relative)) { throw "Paused bootstrap created a file: $relative" }
        $afterHash = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash
        if ($afterHash -ne $before[$relative]) { throw "Paused bootstrap mutated a file: $relative" }
    }

    Write-Output 'Bootstrap pause fixture verified: paused capture fails before generated projections are refreshed.'
}
finally {
    if (Test-Path -LiteralPath $temp) { Remove-Item -LiteralPath $temp -Recurse -Force }
}
