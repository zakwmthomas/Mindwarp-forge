param([string]$ReceiptPath,[string]$ResultPath,[string]$CheckpointPath,[string]$ProgramPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
if([string]::IsNullOrWhiteSpace($ReceiptPath)){$ReceiptPath=Join-Path $root 'docs\canonical-system\G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'}
if([string]::IsNullOrWhiteSpace($ResultPath)){$ResultPath=Join-Path $root 'docs\canonical-system\G1_C4_HIERARCHY_HISTORY_LOCAL_RESULT.md'}
if([string]::IsNullOrWhiteSpace($CheckpointPath)){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if([string]::IsNullOrWhiteSpace($ProgramPath)){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
$receipt=Get-Content -Raw $ReceiptPath|ConvertFrom-Json
$result=Get-Content -Raw $ResultPath
$batch=Get-Content -Raw $CheckpointPath
foreach($value in @($receipt.semantic_receipt_sha256,$receipt.bounded_source_manifest_sha256,$receipt.source_commit,$receipt.tracked_tree_manifest_sha256)){if(!$result.Contains($value)-or!$batch.Contains($value)){throw "C4 retained records disagree on $value"}}
$state=$batch|ConvertFrom-Json
$program=Get-Content -Raw $ProgramPath|ConvertFrom-Json
$c4=@($program.items|Where-Object id -eq 'C4');$c5=@($program.items|Where-Object id -eq 'C5');$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
$c4Live=$state.batch_id-eq'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1'-and$state.master_program_item-eq'C4'-and$state.state-eq'executing'-and$state.substage_id-eq'c4-independent-platform-gate'-and$state.context_health-match'candidate_verified_local'-and$state.next_action-match'registered forge-full-gate-v1'-and$state.next_action-match'C5 inactive'-and$c4.Count-eq1-and$c4[0].state-eq'executing'-and$c4[0].status-eq'active'-and$active.Count-eq1-and$active[0].id-eq'C4'
$c4Run=if($c4.Count-eq1){[regex]::Match([string]$c4[0].proof,'run-[0-9a-f]{32}')}else{$null}
$c5Retained=$state.batch_id-eq'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1'-and$state.master_program_item-eq'C5'-and$state.state-eq'executing'-and$state.substage_id-eq'c5-reconciliation-readiness'-and$state.authority_lane-eq'Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.'-and$c4.Count-eq1-and$c4[0].state-eq'verified'-and$c4[0].status-eq'complete'-and((@($c4[0].depends_on)-join',')-eq'C2,C3A')-and$c4Run.Success-and@($c4[0].sources)-contains'G1_C4_CLOSURE_RESULT.md'-and$c5.Count-eq1-and$c5[0].state-eq'executing'-and$c5[0].status-eq'active'-and((@($c5[0].depends_on)-join',')-eq'C4')-and$active.Count-eq1-and$active[0].id-eq'C5'-and@($state.verification_receipts)-contains"registered-full-gate:$($c4Run.Value):passed"-and@($state.verification_receipts)-contains'receipt:G1-C4-CLOSURE:recorded'
if(!$c4Live-and!$c5Retained){throw 'C4 retained records are not bound to the exact live C4 gate or authenticated C5 successor.'}
Write-Output "C4 retained records agree: commit $($receipt.source_commit), semantic $($receipt.semantic_receipt_sha256)."
