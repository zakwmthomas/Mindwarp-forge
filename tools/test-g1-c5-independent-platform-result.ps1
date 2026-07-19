$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python3' }
& $python (Join-Path $PSScriptRoot 'test-g1-c5-independent-platform-result.py')
if ($LASTEXITCODE -ne 0) { throw 'C5 retained independent-platform hostile fixtures failed.' }
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-c5-replay-scope-' + [guid]::NewGuid().ToString('N'))
try {
    New-Item -ItemType Directory -Path $temporary -Force | Out-Null
    $checkpoint = Join-Path $temporary 'checkpoint.json'
    $missing = Join-Path $temporary 'missing-receipt.json'

    @{batch_id='UNRELATED-V1';master_program_item='B4';state='executing';substage_id='unrelated-stage'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c5-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Unrelated checkpoint did not skip C5 replay.' }

    foreach ($substage in @('c5-reconciliation-readiness','c5-portability-classification','c5-verification','c5-verified-result')) {
        @{batch_id='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1';master_program_item='C5';state='executing';substage_id=$substage} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
        & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c5-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "Unrelated C5 substage $substage did not skip replay." }
    }

    @{batch_id='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1';master_program_item='C5';state='executing';substage_id='c5-independent-platform-gate'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    $saved = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c5-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing 2>&1 | Out-Null
    $targetExit = $LASTEXITCODE
    $ErrorActionPreference = $saved
    if ($targetExit -eq 0) { throw 'Exact C5 independent-platform gate admitted a missing retained receipt.' }
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}
Write-Output 'C5 independent-platform replay scope verified: unrelated substages skip; exact gate fails closed.'
