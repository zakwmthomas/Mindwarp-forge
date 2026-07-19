param([string]$CheckpointPath,[string]$ReceiptPath)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) { $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
if ([string]::IsNullOrWhiteSpace($ReceiptPath)) { $ReceiptPath = Join-Path $root 'docs\canonical-system\G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json' }
$checkpoint = Get-Content -LiteralPath $CheckpointPath -Raw | ConvertFrom-Json
foreach ($property in @('batch_id','master_program_item','state','substage_id')) {
    if ($checkpoint.PSObject.Properties.Name -notcontains $property) { throw "C5 replay checkpoint is missing: $property" }
}
$c5Batch = 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1'
$replayRequired = $checkpoint.batch_id -eq $c5Batch -and
    $checkpoint.master_program_item -eq 'C5' -and
    $checkpoint.state -eq 'executing' -and
    $checkpoint.substage_id -eq 'c5-independent-platform-gate'
if (!$replayRequired) {
    Write-Output 'C5 independent-platform replay not applicable at this substage.'
    return
}
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python3' }
& $python (Join-Path $PSScriptRoot 'verify-g1-c5-independent-platform-result.py') --receipt $ReceiptPath
if ($LASTEXITCODE -ne 0) { throw 'C5 retained independent-platform attestation replay failed.' }
