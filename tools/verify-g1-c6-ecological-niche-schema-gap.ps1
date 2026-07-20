param(
  [string]$AuditPath,
  [string]$CheckpointPath,
  [string]$ProgramPath,
  [string]$RootPath
)
$ErrorActionPreference='Stop'
$repoRoot=Split-Path -Parent $PSScriptRoot
$root=if($RootPath){$RootPath}else{$repoRoot}
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
if(!$AuditPath){$AuditPath=Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_SCHEMA_GAP_AUDIT.md'}
if(!$CheckpointPath){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if(!$ProgramPath){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
$audit=Get-Content -Raw -LiteralPath $AuditPath
$normalizedAudit=$audit-replace'\s+',' '
$checkpoint=Get-Content -Raw -LiteralPath $CheckpointPath|ConvertFrom-Json
$program=Get-Content -Raw -LiteralPath $ProgramPath|ConvertFrom-Json
if(!(Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $checkpoint)){throw 'C6 ecological-niche schema-gap route is not exact.'}
foreach($receipt in @(
  'receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded',
  'receipt:G1-C6-ORGANISM-SUBJECT-IDENTITY-V1:recorded',
  'registered-full-gate:run-500f816d66e94a359e9cf8617982bf49:passed',
  'owner-route:c6-ecological-niche-semantics-schema-gap-audit:authorized'
)){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 ecology predecessor or route receipt missing: $receipt"}}
if(@($checkpoint.verification_receipts|Where-Object{$_-match'^(owner-authorization|source-authorization):c6-ecological-niche'}).Count-ne0){throw 'C6 ecology source authorization exists during schema-gap audit.'}
$c4=@($program.items|Where-Object id -eq C4);$c5=@($program.items|Where-Object id -eq C5);$c6=@($program.items|Where-Object id -eq C6)
if($c4.Count-ne1-or$c5.Count-ne1-or$c6.Count-ne1){throw 'C4-C6 program items are missing or ambiguous.'}
if($c4[0].state-ne'verified'-or$c4[0].status-ne'complete'-or$c5[0].state-ne'verified'-or$c5[0].status-ne'complete'){throw 'C6 ecology prerequisites are not verified and complete.'}
if($c6[0].state-ne'executing'-or$c6[0].status-ne'active'-or$c6[0].gate-ne'design'-or(@($c6[0].depends_on)-join',')-ne'C4,C5'){throw 'C6 ecology master-program state, dependencies or design gate drifted.'}
$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
if($active.Count-ne1-or$active[0].id-ne'C6'){throw 'C6 is not the sole active cursor.'}
$ecologySources=@($c6[0].sources|Where-Object{$_-like'*ecological*niche*'})
if($ecologySources.Count-ne1-or$ecologySources[0]-ne'G1_C6_ECOLOGICAL_NICHE_SEMANTICS_SCHEMA_GAP_AUDIT.md'){throw 'C6 ecology schema-gap source registration drifted.'}
foreach($token in @(
  'source-negative schema-gap audit',
  'cannot automatically derive ecological truth from physical opportunity',
  'Exact upstream authority inventory',
  'spatial-domain','regional-environment-state','physical-region-partition',
  'niche-graph-binding','organism-subject-identity',
  'local accessibility, resource yield, replenishment or organism use',
  'Claim disposition','bounded-satisfied','design-ready','evidence-blocked',
  'consumer-owned-downstream','Versioned authored candidate assertions',
  'Non-cyclic ownership and one-consumer boundary',
  'Source-negative hostile obligations',
  'C6-H100','C6-H101','C6-H102','C6-H103','C6-H104','C6-H105',
  'prospective association as realized occupancy',
  'C6-H1100','H1101','H1102','H1103','H1105','H1106','H1108',
  'H1104 scale generalization','H1107 real C5 consumption',
  'Requirements before implementation readiness',
  'Numerical ceilings are not invented by this audit',
  'Do not create the contract or crate'
)){if(!$normalizedAudit.Contains($token)){throw "C6 ecology schema-gap audit token missing: $token"}}
foreach($pattern in @(
  '(?i)normative override\s*:',
  '(?i)physical opportunity automatically proves',
  '(?i)caller labels? (?:or edges? )?(?:are|prove) ecological facts?',
  '(?i)prospective association proves realized occupancy',
  '(?i)create the ecological contract and source now',
  '(?i)missing evidence (?:is|means) (?:zero|false|invalid|impossible)'
)){if($audit-match$pattern){throw "C6 ecology contradictory normative claim detected: $pattern"}}
$hostileSection=[regex]::Match($audit,'(?s)## Source-negative hostile obligations\s+(.*?)\s+## Requirements before implementation readiness')
if(!$hostileSection.Success){throw 'C6 ecology direct hostile section is missing or relocated.'}
$direct=@([regex]::Matches($hostileSection.Groups[1].Value,'C6-H10[0-5](?![0-9])')|ForEach-Object Value|Sort-Object -Unique)
if($direct.Count-ne6){throw "C6 ecology direct hostile section must contain exact H100..H105, found $($direct.Count)."}
$forbidden=@(
  (Join-Path $root 'crates\ecological-niche-semantics'),
  (Join-Path $root 'contracts\ecological-niche-semantics-contract.md'),
  (Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md'),
  (Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_RESULT.md')
)
foreach($path in $forbidden){if(Test-Path -LiteralPath $path){throw "C6 ecology schema-gap audit crossed into gated output: $path"}}
$cargoFiles=@(Get-ChildItem -LiteralPath $root -Filter Cargo.toml -File -Recurse -ErrorAction SilentlyContinue)
foreach($cargo in $cargoFiles){if((Get-Content -Raw -LiteralPath $cargo.FullName)-match'(?i)ecological-niche-semantics'){throw "C6 ecology Cargo member or dependency exists during schema-gap audit: $($cargo.FullName)"}}
$nicheModule=Get-Content -Raw (Join-Path $root 'crates\niche-graph-binding\MODULE.md')
foreach($token in @('complete ecological niche graph','habitat suitability hazards trophic roles or competition','occupancy')){if(!$nicheModule.Contains($token)){throw "C3 physical-opportunity boundary drifted: $token"}}
Write-Output 'G1 C6 ecological-niche schema gap verified: exact design gate, evidence inventory, hypothesis-only seam, six anchored hostiles and no source/readiness crossing.'
