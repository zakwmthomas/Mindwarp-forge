param(
  [string]$DesignPath,
  [string]$ContractPath,
  [string]$ReadinessPath,
  [string]$CheckpointPath,
  [string]$ProgramPath,
  [string]$RootPath
)
$ErrorActionPreference='Stop'
$repoRoot=Split-Path -Parent $PSScriptRoot
$root=if($RootPath){$RootPath}else{$repoRoot}
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
if(!$DesignPath){$DesignPath=Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_MATHEMATICAL_DESIGN_AUDIT.md'}
if(!$ContractPath){$ContractPath=Join-Path $root 'contracts\ecological-niche-semantics-contract.md'}
if(!$ReadinessPath){$ReadinessPath=Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md'}
if(!$CheckpointPath){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if(!$ProgramPath){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
foreach($path in @($DesignPath,$ContractPath,$ReadinessPath,$CheckpointPath,$ProgramPath)){if(!(Test-Path -LiteralPath $path -PathType Leaf)){throw "C6 ecology design/readiness artifact missing: $path"}}
$design=Get-Content -Raw -LiteralPath $DesignPath
$contract=Get-Content -Raw -LiteralPath $ContractPath
$readiness=Get-Content -Raw -LiteralPath $ReadinessPath
$all=($design+' '+$contract+' '+$readiness)-replace'\s+',' '
$checkpoint=Get-Content -Raw -LiteralPath $CheckpointPath|ConvertFrom-Json
$program=Get-Content -Raw -LiteralPath $ProgramPath|ConvertFrom-Json
if(!(Test-G1C6EcologicalNicheSemanticsDesignReadinessRoute -Checkpoint $checkpoint)){throw 'C6 ecological-niche semantics design/readiness route is not exact.'}
foreach($receipt in @(
  'receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded',
  'receipt:G1-C6-ORGANISM-SUBJECT-IDENTITY-V1:recorded',
  'receipt:G1-C6-ECOLOGICAL-NICHE-SCHEMA-GAP-AUDIT-V1:recorded',
  'registered-full-gate:run-c0712471f015435abbaa71ac001ef975:passed',
  'owner-route:c6-ecological-niche-semantics-design-readiness:authorized'
)){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 ecology design/readiness predecessor or route receipt missing: $receipt"}}
if(@($checkpoint.verification_receipts|Where-Object{$_-match'^(owner-authorization|source-authorization):c6-ecological-niche'}).Count-ne0){throw 'C6 ecology source authorization exists during design/readiness.'}
$c4=@($program.items|Where-Object id -eq C4);$c5=@($program.items|Where-Object id -eq C5);$c6=@($program.items|Where-Object id -eq C6)
if($c4.Count-ne1-or$c5.Count-ne1-or$c6.Count-ne1){throw 'C4-C6 program items are missing or ambiguous.'}
if($c4[0].state-ne'verified'-or$c4[0].status-ne'complete'-or$c5[0].state-ne'verified'-or$c5[0].status-ne'complete'){throw 'C6 ecology prerequisites are not verified and complete.'}
if($c6[0].state-ne'executing'-or$c6[0].status-ne'active'-or$c6[0].gate-ne'design'-or(@($c6[0].depends_on)-join',')-ne'C4,C5'){throw 'C6 ecology master-program state, dependencies or design gate drifted.'}
$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
if($active.Count-ne1-or$active[0].id-ne'C6'){throw 'C6 is not the sole active cursor.'}
$expectedSources=@(
 'G1_C6_ECOLOGICAL_NICHE_SEMANTICS_SCHEMA_GAP_AUDIT.md',
 'G1_C6_ECOLOGICAL_NICHE_SEMANTICS_MATHEMATICAL_DESIGN_AUDIT.md',
 'ecological-niche-semantics-contract.md',
 'G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md'
)
$actualSources=@($c6[0].sources|Where-Object{$_-match'(?i)ecological.?niche'})
if(($actualSources-join'|')-ne($expectedSources-join'|')){throw 'C6 ecology design/readiness source registration drifted.'}
foreach($token in @(
 'one atomic authored-hypothesis model','AuthoredEcologicalHypothesisV1','LineageSubjectRefV1','SpeciesCandidateSubjectRefV1',
 'prospective_opportunity_association','habitat_requirement_candidate','resource_requirement_candidate','hazard_exposure_candidate',
 'trophic_relation_candidate','competition_relation_candidate','transition_candidate','opportunity_scalar_inclusive',
 'Competition endpoints may be equal','Detrital pathways remain unavailable','equal-endpoint competition receipt',
 'necessary_evidence_supported','necessary_evidence_unavailable','necessary_evidence_contradictory','indeterminate_budget',
 'ecological_truth_status: unresolved','Missing evidence is not false or zero','C6-H100','C6-H101','C6-H102','C6-H103','C6-H104','C6-H105',
 'C6-H1100','C6-H1101','C6-H1102','C6-H1103','C6-H1105','C6-H1106','C6-H1108',
 '16,384','32,768','maximum predicates: `12`','maximum distinct subjects: `2`','maximum authored referents: `1`','maximum examinations: `128`',
 'required-minus-one','No failure may produce a partial receipt','one additive','organism-niche-binding','deletion-only rollback',
 'Package 9 may define dimorphism','package 10 explicit species membership','does not create a crate, Cargo member/dependency, production source'
)){if(!$all.Contains($token)){throw "C6 ecology design/readiness token missing: $token"}}
foreach($pattern in @(
 '(?i)normative override\s*:',
 '(?i)necessary_evidence_supported`? (?:means|proves) (?:ecologically |habitat )?suitable',
 '(?i)physical opportunity (?:means|proves|is) (?:a )?complete niche',
 '(?i)prospective (?:association|candidate) (?:means|proves) realized occupancy',
 '(?i)species.?candidate (?:means|proves) species membership',
 '(?i)body.?plan expression (?:means|proves) (?:sex|dimorphism)',
 '(?i)competition endpoints must be distinct',
 '(?i)missing evidence (?:is|means) (?:zero|false|invalid|impossible)',
 '(?i)implement (?:the )?production source now'
)){if($all-match$pattern){throw "C6 ecology contradictory normative claim detected: $pattern"}}
$hostileSection=[regex]::Match($design,'(?s)## Frozen hostile mapping\s+(.*?)\s+## Ordering and dimorphism boundary')
if(!$hostileSection.Success){throw 'C6 ecology hostile section is missing or relocated.'}
$direct=@([regex]::Matches($hostileSection.Groups[1].Value,'C6-H10[0-5](?![0-9])')|ForEach-Object Value|Sort-Object -Unique)
if($direct.Count-ne6){throw "C6 ecology hostile section must contain exact H100..H105, found $($direct.Count)."}
foreach($forbidden in @(
 (Join-Path $root 'crates\ecological-niche-semantics'),
 (Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_RESULT.md')
)){if(Test-Path -LiteralPath $forbidden){throw "C6 ecology design/readiness crossed into implementation: $forbidden"}}
foreach($cargo in @(Get-ChildItem -LiteralPath $root -Filter Cargo.toml -File -Recurse -ErrorAction SilentlyContinue)){
 if((Get-Content -Raw -LiteralPath $cargo.FullName)-match'(?i)ecological-niche-semantics'){throw "C6 ecology Cargo member or dependency exists before authorization: $($cargo.FullName)"}
}
$consumerRoot=Join-Path $root 'crates\organism-niche-binding'
if(Test-Path -LiteralPath $consumerRoot){
 foreach($file in @(Get-ChildItem -LiteralPath $consumerRoot -File -Recurse -ErrorAction SilentlyContinue)){
  if($file.Name-ne'MODULE.md'-and(Get-Content -Raw -LiteralPath $file.FullName)-match'(?i)ecological.?niche.?semantics'){throw "C6 ecology consumer source changed before authorization: $($file.FullName)"}
 }
}
Write-Output 'G1 C6 ecological-niche semantics design/readiness verified: atomic authored hypotheses, typed indeterminacy, exact bounds, one-consumer design and no source crossing.'
