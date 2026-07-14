$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$readinessPath=Join-Path $root 'docs\canonical-system\SEMANTIC_CONSTRUCTION_READINESS.md'
$gatePath=Join-Path $root 'docs\canonical-system\SEMANTIC_CONSTRUCTION_DESIGN_GATE.md'
$contractPath=Join-Path $root 'contracts\semantic-construction-contract.md'
$cratePath=Join-Path $root 'crates\semantic-construction'
foreach($path in @($readinessPath,$gatePath,$contractPath,$cratePath)){if(!(Test-Path $path)){throw "P6 artifact missing: $path"}}
$readiness=Get-Content $readinessPath -Raw
$gate=Get-Content $gatePath -Raw
$contract=Get-Content $contractPath -Raw
foreach($required in @('PressureContext','RoleSet','SolutionFamilySet','CapabilityGraph','PartRoleGraph','ConstructionRecipe','Poison-word input','Contradictory pressures','Readiness gaps deliberately left open')){if(!$readiness.Contains($required)){throw "P6 readiness missing: $required"}}
foreach($required in @('Recovered evidence audit','66 passing tests','symbol-grounding problem','ConceptId','JustificationGraph','single_feasible_family','no private global scalar','closed, versioned registry','Atomic recipe rejection','Whole-system alignment','indeterminate_budget','Exact confirmation')){if(!$gate.Contains($required)){throw "P6 design gate missing: $required"}}
foreach($required in @('P2 identity','P4 hierarchy/history','P5 significance/scheduler','P7 representation/assets/animation','read-only','protected-Kernel')){if(!$gate.Contains($required)){throw "P6 neighbour boundary missing: $required"}}
foreach($forbidden in @('implementation authorized','runtime selected','engine approved','product ontology selected','protected Kernel mutation approved')){if($gate.Contains($forbidden)){throw "P6 design gate contains forbidden authority: $forbidden"}}
foreach($required in @('Stable `ConceptId`','no universal scalar score','closed versioned registry','no partial result','canonical JSON','indeterminate_budget','read-only `ProofReceipt`','not Mind Warp content grammar')){if(!$contract.Contains($required)){throw "P6 contract missing: $required"}}
$sources=(Get-ChildItem (Join-Path $cratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('SemanticConstructionPackage','ClaimClass','JustificationEdge','SolutionFamilySet','CapabilityRegistry','PartRoleGraph','ConstructionRecipe','ValidationReport','IndeterminateBudget','reference_proof_evidence')){if(!$sources.Contains($required)){throw "P6 crate missing proof surface: $required"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($sources.Contains($forbidden)){throw "P6 crate crosses capability boundary: $forbidden"}}
$manifest=Get-Content (Join-Path $cratePath 'Cargo.toml') -Raw
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($manifest.Contains($forbidden)){throw "P6 manifest crosses capability boundary: $forbidden"}}
$program=Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw|ConvertFrom-Json
$active=@($program.items|Where-Object status -eq 'active')
$f5=@($program.items|Where-Object id -eq 'F5')[0]
if($active.Count-ne 1-or($active[0].id-ne'F5'-and!($f5.status-eq'complete'-and$active[0].milestone-in@('G1','R1')))){throw 'P6 design gate is not retained through the F5 or later route.'}
Write-Output 'F5 P6 semantic/construction verified: research, strict contract, canonical reference, causal/lexical separation, alternatives, closed validation, atomic graph replay, adversarial fixtures, and capability boundaries retained.'
