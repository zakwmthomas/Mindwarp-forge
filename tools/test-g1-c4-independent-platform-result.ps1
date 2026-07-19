$ErrorActionPreference = 'Stop'
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python3' }
& $python (Join-Path $PSScriptRoot 'test-g1-c4-independent-platform-result.py')
if ($LASTEXITCODE -ne 0) { throw 'C4 retained independent-platform hostile fixtures failed.' }
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-c4-replay-scope-' + [guid]::NewGuid().ToString('N'))
try {
    New-Item -ItemType Directory -Path $temporary -Force | Out-Null
    $checkpoint = Join-Path $temporary 'checkpoint.json'
    $missing = Join-Path $temporary 'missing-receipt.json'
    @{batch_id='UNRELATED-V1';master_program_item='B4';state='executing';substage_id='unrelated-stage'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Unrelated checkpoint did not skip C4 replay.' }
    @{batch_id='G1-C4-HIERARCHY-HISTORY-CLOSURE-V1';master_program_item='C4';state='executing';substage_id='c4-independent-platform-gate'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    $saved = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing 2>&1 | Out-Null
    $targetExit = $LASTEXITCODE; $ErrorActionPreference = $saved
    if ($targetExit -eq 0) { throw 'Target C4 checkpoint admitted a missing retained receipt.' }
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}
Write-Output 'C4 independent-platform replay scope verified: unrelated stages skip; target stage fails closed.'
