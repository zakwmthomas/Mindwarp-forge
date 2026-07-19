param([string]$ReceiptPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
if([string]::IsNullOrWhiteSpace($ReceiptPath)){$ReceiptPath=Join-Path $root 'docs\canonical-system\G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'}
$receipt=Get-Content -Raw $ReceiptPath|ConvertFrom-Json
$result=Get-Content -Raw (Join-Path $root 'docs\canonical-system\G1_C4_HIERARCHY_HISTORY_LOCAL_RESULT.md')
$batch=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')
foreach($value in @($receipt.semantic_receipt_sha256,$receipt.bounded_source_manifest_sha256,$receipt.source_commit,$receipt.tracked_tree_manifest_sha256)){if(!$result.Contains($value)-or!$batch.Contains($value)){throw "C4 retained records disagree on $value"}}
$state=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json
if($state.state-ne'executing'-or$state.substage_id-ne'c4-verification'-or$state.context_health-notmatch'candidate_verified_local'-or$state.next_action-notmatch'independent.*platform'){throw 'C4 retained checkpoint state changed.'}
Write-Output "C4 retained records agree: commit $($receipt.source_commit), semantic $($receipt.semantic_receipt_sha256)."
