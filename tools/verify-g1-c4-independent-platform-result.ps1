param([string]$CheckpointPath,[string]$ReceiptPath)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) { $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
if ([string]::IsNullOrWhiteSpace($ReceiptPath)) { $ReceiptPath = Join-Path $root 'docs\canonical-system\G1_C4_INDEPENDENT_PLATFORM_EXECUTION.json' }
$checkpoint = Get-Content -LiteralPath $CheckpointPath -Raw | ConvertFrom-Json
foreach ($property in @('batch_id','master_program_item','state','substage_id')) { if ($checkpoint.PSObject.Properties.Name -notcontains $property) { throw "C4 replay checkpoint is missing: $property" } }
$c4Batch='G1-C4-HIERARCHY-HISTORY-CLOSURE-V1';$c5Batch='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1'
if($checkpoint.batch_id-eq$c4Batch){if($checkpoint.master_program_item-ne'C4'-or$checkpoint.state-ne'executing'){throw 'C4 replay checkpoint route is malformed.'};if($checkpoint.substage_id-notin@('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result','c4-independent-platform-gate')){throw 'C4 replay checkpoint substage is unrecognized.'}}
if($checkpoint.batch_id-eq$c5Batch){if($checkpoint.master_program_item-ne'C5'-or$checkpoint.state-ne'executing'-or$checkpoint.substage_id-ne'c5-reconciliation-readiness'){throw 'C5 successor replay checkpoint route is malformed.'}}
$replayRequired=($checkpoint.batch_id-eq$c4Batch-and$checkpoint.substage_id-eq'c4-independent-platform-gate')-or($checkpoint.batch_id-eq$c5Batch)
if(!$replayRequired){Write-Output 'C4 independent-platform replay not applicable at this substage.';return}
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python3' }
& $python (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.py') --receipt $ReceiptPath
if ($LASTEXITCODE -ne 0) { throw 'C4 retained independent-platform attestation replay failed.' }
