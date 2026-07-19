$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
$canonical=Get-Content -Raw (Join-Path $root 'context/active/WORKER_BATCH_STATE.json')|ConvertFrom-Json
if(!(Test-G1C6AuthorizedCurrentRoute -Checkpoint $canonical)){throw 'Canonical C6 route rejected.'}
$readiness=$canonical|ConvertTo-Json -Depth 100|ConvertFrom-Json
$readiness.batch_id='G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1';$readiness.substage_id='c6-reconciliation-readiness'
$readiness.authority_lane='Owner-authorized C6 semantic/construction and organism-ecology reconciliation and capability-free readiness only. Exact dependencies verified C4 and C5. Retain corrected C6 prerequisite evidence as non-closure evidence. No C6 implementation source, C3B, C7, broad G1 closure, runtime, product ontology or vocabulary, solver or AI generation, geometry, assets, animation, renderer, visual-quality claim, physiology or content constants, filesystem, network, process, Companion, Greenfield, promotion authority or Kernel mutation.'
if(!(Test-G1C6ReconciliationReadinessRoute -Checkpoint $readiness)){throw 'Synthetic C6 readiness route rejected.'}
$implementation=$canonical|ConvertTo-Json -Depth 100|ConvertFrom-Json
$implementation.batch_id='G1-C6-BODY-PLAN-STRUCTURE-IMPLEMENTATION-V1';$implementation.substage_id='c6-body-plan-structure-test-first-implementation'
$implementation.authority_lane='Owner-authorized capability-free C6 body-plan family/topology V1 test-first implementation only. Exact dependencies verified C4 and C5. Authorizes the new body-plan-structure crate, one additive macro-lineage-binding family-reference validator, exact tests, governance projections and verification for this package. No ecology realization, physiology, reproduction, heredity, development, sex or dimorphism applicability, caste, species, individual or population semantics, personhood, product ontology, solver or AI generation, geometry, proportions, pose, assets, animation, renderer, visual-quality claim, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation.'
if(!(Test-G1C6BodyPlanStructureImplementationRoute -Checkpoint $implementation)){throw 'Synthetic C6 body-plan implementation route rejected.'}
foreach($field in @('batch_id','master_program_item','state','substage_id','authority_lane')){$x=$implementation|ConvertTo-Json -Depth 100|ConvertFrom-Json;$x.$field='FORGED';if(Test-G1C6AuthorizedCurrentRoute -Checkpoint $x){throw "Forged C6 field admitted: $field"}}
Write-Output 'G1 C6 authorized successor routes verified: readiness and exact body-plan implementation tuples pass; forged fields fail closed.'

