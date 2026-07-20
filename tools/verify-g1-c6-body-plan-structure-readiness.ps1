param([string]$ProgramPath,[string]$CheckpointPath,[string]$ReadinessPath,[string]$ContractPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
if(!$ProgramPath){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
if(!$CheckpointPath){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if(!$ReadinessPath){$ReadinessPath=Join-Path $root 'docs\canonical-system\G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md'}
if(!$ContractPath){$ContractPath=Join-Path $root 'contracts\body-plan-structure-contract.md'}
$program=Get-Content -Raw -LiteralPath $ProgramPath|ConvertFrom-Json
$checkpoint=Get-Content -Raw -LiteralPath $CheckpointPath|ConvertFrom-Json
$readiness=Get-Content -Raw -LiteralPath $ReadinessPath
$contract=Get-Content -Raw -LiteralPath $ContractPath
$bodyPlanRoute=Test-G1C6BodyPlanStructureImplementationRoute -Checkpoint $checkpoint
$identitySuccessorRoute=Test-G1C6OrganismIdentityReadinessRoute -Checkpoint $checkpoint
$identityImplementationRoute=Test-G1C6OrganismSubjectIdentityImplementationRoute -Checkpoint $checkpoint
$ecologySchemaGapRoute=Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $checkpoint
$ecologyDesignReadinessRoute=Test-G1C6EcologicalNicheSemanticsDesignReadinessRoute -Checkpoint $checkpoint
if(!$bodyPlanRoute-and!$identitySuccessorRoute-and!$identityImplementationRoute-and!$ecologySchemaGapRoute-and!$ecologyDesignReadinessRoute){throw 'C6 body-plan implementation or exact recorded successor route is not exact.'}
$c6=@($program.items|Where-Object id -eq C6)
$expectedGate=if($identitySuccessorRoute-or$ecologySchemaGapRoute-or$ecologyDesignReadinessRoute){'design'}else{'implementation'}
if($c6.Count-ne1-or$c6[0].state-ne'executing'-or$c6[0].status-ne'active'-or$c6[0].gate-ne$expectedGate-or(@($c6[0].depends_on)-join ',')-ne'C4,C5'){throw 'C6 body-plan master-program or recorded successor route is not exact.'}
foreach($source in @('G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md','body-plan-structure-contract.md')){if(@($c6[0].sources)-notcontains$source){throw "C6 body-plan source record missing: $source"}}
foreach($receipt in @('owner-authorization:c6-body-plan-structure-v1:released','registered-full-gate:run-818500121b3f4958ac093b105a7fe61b:passed')){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 body-plan readiness receipt missing: $receipt"}}
if(($identitySuccessorRoute-or$identityImplementationRoute-or$ecologySchemaGapRoute-or$ecologyDesignReadinessRoute)-and@($checkpoint.verification_receipts)-notcontains'receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded'){throw 'C6 body-plan recorded successor receipt missing.'}
foreach($token in @('body_plan_ref` means family identity, not expression identity','Exactly 18 test groups','HUMANOID_BILATERAL_V1','RADIAL_POLYRAY_V1','WITHHELD_SERIAL_V1','macro-lineage-binding','Stop after the body-plan result is recorded')){if(!$readiness.Contains($token)){throw "C6 body-plan readiness token missing: $token"}}
foreach($token in @('coordinate-free body-plan family','content-derived family','multiple containment roots','No universal pelvis','part templates | 64','relation instances | 512','262,144','4,096','V1 owns no sex or dimorphism applicability','One consumer')){if(!$contract.Contains($token)){throw "C6 body-plan contract token missing: $token"}}
Write-Output 'G1 C6 body-plan structure implementation readiness verified: exact family/expression identity, 18 test groups, resource envelope, one consumer and authority-negative scope are frozen.'
