$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$ensureSource = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'ensure-context-current.ps1') -Raw
$verifySource = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'verify-bootstrap.ps1') -Raw

foreach ($required in @(
    "[ValidateSet('Check', 'Refresh')]",
    "[string]`$Mode = 'Check'",
    "if (`$Mode -eq 'Refresh')",
    "verify-bootstrap.ps1') -Mode `$Mode"
)) {
    if (!$ensureSource.Contains($required)) { throw "Context gate lacks explicit mode boundary: $required" }
}
if (!$ensureSource.Contains("if (!`$forgeProcess -and `$Mode -eq 'Refresh')")) {
    throw 'Read-only check mode can still launch Forge.'
}

foreach ($required in @(
    "[ValidateSet('Check', 'Refresh')][string]`$Mode = 'Check'",
    "if (`$Mode -eq 'Refresh')",
    "verify-worker-feedback-freshness.ps1"
)) {
    if (!$verifySource.Contains($required)) { throw "Bootstrap verifier lacks explicit mode boundary: $required" }
}

if (Test-Path -LiteralPath (Join-Path $root '.local\forge-bootstrap\MANIFEST.json')) {
    $sentinels = @(
        'context\active\CURRENT_STATE.md',
        'context\bootstrap\BRIEFING.md',
        'governance\WORKER_FEEDBACK_BRIEF.md'
    )
    $before = @{}
    foreach ($relative in $sentinels) {
        $before[$relative] = (Get-FileHash -LiteralPath (Join-Path $root $relative) -Algorithm SHA256).Hash
    }
    & (Join-Path $PSScriptRoot 'verify-bootstrap.ps1') -Mode Check | Out-Null
    foreach ($relative in $sentinels) {
        $after = (Get-FileHash -LiteralPath (Join-Path $root $relative) -Algorithm SHA256).Hash
        if ($after -ne $before[$relative]) { throw "Active bootstrap Check mutated generated projection: $relative" }
    }
}

Write-Output 'Bootstrap modes verified: Check is the non-mutating default and Refresh is explicit.'
