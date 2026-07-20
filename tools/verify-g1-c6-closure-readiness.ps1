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
$normalizedText=$text-replace'\s+',' '
$readinessRoute=Test-G1C6ReconciliationReadinessRoute -Checkpoint $checkpoint
$implementationRoute=Test-G1C6BodyPlanStructureImplementationRoute -Checkpoint $checkpoint
$identityReadinessRoute=Test-G1C6OrganismIdentityReadinessRoute -Checkpoint $checkpoint
$identityImplementationRoute=Test-G1C6OrganismSubjectIdentityImplementationRoute -Checkpoint $checkpoint
$ecologySchemaGapRoute=Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $checkpoint
if(!$readinessRoute-and!$implementationRoute-and!$identityReadinessRoute-and!$identityImplementationRoute-and!$ecologySchemaGapRoute){throw 'C6 authorized current route tuple drifted.'}
$c4=@($program.items|Where-Object id -eq C4);$c5=@($program.items|Where-Object id -eq C5);$c6=@($program.items|Where-Object id -eq C6)
if($c4.Count-ne1-or$c5.Count-ne1-or$c6.Count-ne1){throw 'C4-C6 program items are missing or ambiguous.'}
if($c4[0].state-ne'verified'-or$c4[0].status-ne'complete'-or$c5[0].state-ne'verified'-or$c5[0].status-ne'complete'){throw 'C6 exact prerequisites are not verified and complete.'}
if($c6[0].state-ne'executing'-or$c6[0].status-ne'active'-or(@($c6[0].depends_on)-join ',')-ne'C4,C5'-or@($c6[0].sources)-notcontains'G1_C6_CLOSURE_READINESS.md'){throw 'C6 program route drifted.'}
$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'});if($active.Count-ne1-or$active[0].id-ne'C6'){throw 'C6 is not the sole active cursor.'}
foreach($token in @('38 tests','`semantic-construction` 14','`organism-niche-binding` 5','`niche-graph-binding` 7','`macro-lineage-binding` 6','`person-form-eligibility` 6','C3 physical opportunity and coavailability precursor','structural binding completeness only','C6-H000..003','C6-H100..105','C6-H200..205','C6-H300..306','C6-H400..405','C6-H500..505','C6-H600..605','C6-H700..705','C6-H800..805','C6-H900..904','C6-H1000..1004','C6-H1100..1108','C6-H1200..1209','exactly 82 IDs','Biological sex, reproductive role','third structurally withheld morphology','approved paired humanoid model','This readiness record does not authorize source','Nothing broader is locked in')){if(!$normalizedText.Contains($token)){throw "C6 readiness token missing: $token"}}
$hostileIds=@([regex]::Matches($text,'C6-H(?:[0-9]{3,4})')|ForEach-Object Value|Sort-Object -Unique)
if($hostileIds.Count-ne82){throw "C6 readiness hostile registry must contain exactly 82 unique IDs, found $($hostileIds.Count)."}
if($readinessRoute-and(Test-Path -LiteralPath (Join-Path $root 'crates\body-plan-structure'))){throw 'Prospective body-plan source exists before separate authorization.'}
if($identityReadinessRoute-and(Test-Path -LiteralPath (Join-Path $root 'crates\organism-subject-identity'))){throw 'Prospective organism identity source exists before separate authorization.'}
foreach($receipt in @('receipt:G1-C5-CLOSURE:recorded','owner-route:c6-reconciliation-readiness:authorized','independent-review:c5-c6-readiness-transition:accepted','focused:c5-c6-successor-route-hostiles:passed','registered-full-gate:run-f51cc195a0f54ad88beeeb5d809780e9:passed','registered-full-gate:run-fcbfd6e3df844cd2b3ece02c48ba9e5c:passed','transition:c5-verified-c6-readiness-activated:recorded')){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 readiness receipt missing: $receipt"}}
if($implementationRoute-and@($checkpoint.verification_receipts)-notcontains'owner-authorization:c6-body-plan-structure-v1:released'){throw 'C6 body-plan implementation authorization receipt missing.'}
if($identityReadinessRoute-and@($checkpoint.verification_receipts)-notcontains'receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded'){throw 'C6 identity readiness requires the recorded body-plan V1 receipt.'}
if($identityImplementationRoute-and@($checkpoint.verification_receipts)-notcontains'owner-authorization:c6-organism-subject-identity-v1:released'){throw 'C6 identity implementation authorization receipt missing.'}
if($ecologySchemaGapRoute-and@($checkpoint.verification_receipts)-notcontains'receipt:G1-C6-ORGANISM-SUBJECT-IDENTITY-V1:recorded'){throw 'C6 ecology schema-gap route requires the recorded identity V1 receipt.'}
Write-Output 'G1 C6 closure readiness verified: exact dependencies, evidence limits, 82 hostile IDs, variation-capable topology and source-negative gate agree.'
