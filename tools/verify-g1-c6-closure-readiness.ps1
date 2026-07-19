param([string]$ProgramPath,[string]$CheckpointPath,[string]$ReadinessPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
if(!$ProgramPath){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
if(!$CheckpointPath){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if(!$ReadinessPath){$ReadinessPath=Join-Path $root 'docs\canonical-system\G1_C6_CLOSURE_READINESS.md'}
$program=Get-Content -LiteralPath $ProgramPath -Raw|ConvertFrom-Json
$checkpoint=Get-Content -LiteralPath $CheckpointPath -Raw|ConvertFrom-Json
$text=Get-Content -LiteralPath $ReadinessPath -Raw
if(!(Test-G1C6ReconciliationReadinessRoute -Checkpoint $checkpoint)){throw 'C6 readiness route tuple drifted.'}
$c4=@($program.items|Where-Object id -eq C4);$c5=@($program.items|Where-Object id -eq C5);$c6=@($program.items|Where-Object id -eq C6)
if($c4.Count-ne1-or$c5.Count-ne1-or$c6.Count-ne1){throw 'C4-C6 program items are missing or ambiguous.'}
if($c4[0].state-ne'verified'-or$c4[0].status-ne'complete'-or$c5[0].state-ne'verified'-or$c5[0].status-ne'complete'){throw 'C6 exact prerequisites are not verified and complete.'}
if($c6[0].state-ne'executing'-or$c6[0].status-ne'active'-or(@($c6[0].depends_on)-join ',')-ne'C4,C5'-or@($c6[0].sources)-notcontains'G1_C6_CLOSURE_READINESS.md'){throw 'C6 program route drifted.'}
$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'});if($active.Count-ne1-or$active[0].id-ne'C6'){throw 'C6 is not the sole active cursor.'}
foreach($token in @('38 tests','`semantic-construction` 14','`organism-niche-binding` 5','`niche-graph-binding` 7','`macro-lineage-binding` 6','`person-form-eligibility` 6','C3 physical opportunity and coavailability precursor','structural binding completeness only','C6-H000..003','C6-H100..105','C6-H200..205','C6-H300..306','C6-H400..405','C6-H500..505','C6-H600..605','C6-H700..705','C6-H800..805','C6-H900..904','C6-H1000..1004','C6-H1100..1108','one humanoid and one structurally distinct nonhuman','This readiness record does not authorize that source','Nothing broader is locked in')){if(!$text.Contains($token)){throw "C6 readiness token missing: $token"}}
if(Test-Path -LiteralPath (Join-Path $root 'crates\body-plan-structure')){throw 'Prospective body-plan source exists before separate authorization.'}
foreach($receipt in @('receipt:G1-C5-CLOSURE:recorded','owner-route:c6-reconciliation-readiness:authorized','independent-review:c5-c6-readiness-transition:accepted','focused:c5-c6-successor-route-hostiles:passed','registered-full-gate:run-f51cc195a0f54ad88beeeb5d809780e9:passed','transition:c5-verified-c6-readiness-activated:recorded')){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 readiness receipt missing: $receipt"}}
Write-Output 'G1 C6 closure readiness verified: exact dependencies, evidence limits, hostile domains and source-negative gate agree.'
