Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$recordPath = Join-Path $root 'docs\canonical-system\G1_C3_POST_PHYSICAL_EVIDENCE_RESIDUAL_OBLIGATION_AND_CLOSURE_ADMISSIBILITY_AUDIT.md'
if(!(Test-Path -LiteralPath $recordPath)){throw 'C3 residual-obligation audit is missing'}
$record = Get-Content -LiteralPath $recordPath -Raw
$plan = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PLAN_V2.md') -Raw
$protocol = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_PHYSICAL_EVIDENCE_ACQUISITION_PROTOCOL_RESULT.md') -Raw
$partition = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_PHYSICAL_REGION_PARTITION_RESULT.md') -Raw
$passage = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_SWEPT_AABB_REFERENCE_RESULT.md') -Raw
$opportunity = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_ENVIRONMENTAL_OPPORTUNITY_RESULT.md') -Raw
$sourceContract = Get-Content -LiteralPath (Join-Path $root 'contracts\calibrated-source-energy-distribution-contract.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$closure = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_CLOSURE_REGISTER.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @(
  'owner delegated the material choice','leave calibrated','physical applicability explicitly blocked',
  'bounded_satisfied','c3_owned_open','evidence_blocked','consumer_owned_downstream',
  'design_ready','optional_maintenance','Residual-obligation inventory',
  'C3 cannot honestly close','unavailable_evidence','Q32.32','per-coordinate-unit',
  'joint source/coupling integration proof','zero downstream consumers',
  'cross-boundary/ecotone fixture mathematical design audit','select',
  'continuous causal field','categorical physical-region boundary','reversal and sampling-order invariance',
  'sharp material/interface cause','direct categorical painting','fixed-width','blending',
  'cause-scaled mixing','evidence-preserving typed boundaries','not add a crate',
  'physical applicability stays explicitly blocked','Nothing broader is locked in',
  'One consumer first, reassess before expanding'
)) {
  if($record -notlike "*$required*"){throw "Residual-obligation audit missing: $required"}
}

foreach($required in @('signals, visibility, traversability, biomes, niches','cross-boundary gradient','no-visible-seam','sharp-cause retention','reversal')) {
  if($plan -notlike "*$required*"){throw "Master C3 obligation drift: $required"}
}
if($protocol -notlike '*current local evidence result is*unavailable_evidence*both required evidence families*'){throw 'Physical-evidence result drift'}
if($partition -notlike '*not biome semantics*' -and $partition -notlike '*does not implement*biomes*'){throw 'Physical-region nonclaim drift'}
if($passage -notlike '*walkability*organisms*terrain*biomes*'){throw 'Generic-passage nonclaim drift'}
if($opportunity -notlike '*habitat*organism*' -and $opportunity -notlike '*does not*ecological*'){throw 'Opportunity downstream boundary drift'}
if($sourceContract -notlike '*zero downstream*consumers*' -and $sourceContract -notlike '*No downstream owner imports*'){throw 'Source-distribution zero-consumer boundary drift'}

$c3 = @($program.items | Where-Object id -eq 'C3')
$route = $c3.Count -eq 1 -and
  (($c3[0].next_action -like '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*' -and
    $c3[0].proof -like '*residual-obligation*physical visibility*evidence-blocked*cross-boundary*no-visible-seam*') -or
   ($c3[0].next_action -like '*physical applicability*blocked*code-free*implementation-readiness*ecotone*oracle*' -and
    $c3[0].proof -like '*ecotone mathematical-design*evidence-preserving typed-boundary*') -or
   ($c3[0].next_action -like '*physical applicability*blocked*ecotone-oracle implementation decision*' -and
    $c3[0].proof -like '*ecotone oracle implementation-readiness*evidence-preserving typed-boundary*') -or
   ($c3[0].next_action -like '*physical applicability*blocked*ecotone oracle implementation verification*' -and
    $c3[0].proof -like '*passes twice*evidence-preserving typed-boundary*'))
if(!$route){throw 'C3 residual-obligation route drift'}
if(@($c3[0].sources) -notcontains 'G1_C3_POST_PHYSICAL_EVIDENCE_RESIDUAL_OBLIGATION_AND_CLOSURE_ADMISSIBILITY_AUDIT.md'){throw 'C3 route omits residual-obligation audit'}
if($closure -notlike '*physical applicability remains evidence-blocked*' -or $closure -notlike '*ecotone*'){throw 'Closure register residual route drift'}

$active = (($checkpoint.batch_id -eq 'G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1' -and
    $checkpoint.substage_id -eq 'post-physical-evidence-residual-obligation-audit-result' -and
    $checkpoint.authority_lane -like '*Owner-authorized code-free*C3 residual-obligation*No crate*contract schema*production source*physical calibration*promotion*C3 closure*') -or
  ($checkpoint.batch_id -in @('G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
    $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
    ($checkpoint.authority_lane -like '*code-free*C3 cross-boundary ecotone mathematical design*No crate*contract schema*production*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Owner-approved disposable C3 ecotone oracle implementation only*No crate*contract schema*dependency*production*promotion*C3 closure*')))
if(!$active -and !$c3InterruptionRoute){throw 'Residual-obligation checkpoint authority drift'}

Write-Output 'C3 residual-obligation audit verified: physical applicability stays blocked and the code-free ecotone fixture design is the sole next route.'
