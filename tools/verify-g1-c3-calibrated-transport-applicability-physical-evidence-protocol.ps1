Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$recordPath = Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_PHYSICAL_EVIDENCE_ACQUISITION_PROTOCOL_RESULT.md'
if(!(Test-Path -LiteralPath $recordPath)){throw 'Physical-evidence acquisition protocol result is missing'}
$record = Get-Content -LiteralPath $recordPath -Raw
$physical = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\src\lib.rs') -Raw
$bulk = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs') -Raw
$bulkContract = Get-Content -LiteralPath (Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md') -Raw
$calibratedContract = Get-Content -LiteralPath (Join-Path $root 'contracts\calibrated-spectral-time-basis-contract.md') -Raw
$geological = Get-Content -LiteralPath (Join-Path $root 'crates\geological-atmospheric\src\lib.rs') -Raw
$viewport = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\P7B_BUILTIN_VIEWPORT_CONTROLLED_STIMULUS_RESULT.md') -Raw
$greenfield = Get-Content -LiteralPath (Join-Path $root 'governance\federation\projects\greenfield.json') -Raw | ConvertFrom-Json
$greenfieldLink = Get-Content -LiteralPath (Join-Path $root 'governance\federation\links\greenfield-forge-reuse.json') -Raw | ConvertFrom-Json
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @(
  'current evidence disposition is','unavailable_evidence','Pre-register the exact question',
  'Capture immutable primary artifacts','Admit spatial calibration only for the exact recipe',
  'Admit coefficient evidence only as a complete hard enclosure','Independently validate',
  'Retain contradictions without choosing a winner','Correct by new immutable identity',
  'Separate physical validity from source freshness','Use typed acquisition dispositions',
  'Advance only on a complete joint evidence set','fixture-scale substitution',
  'cross-project substitution','format-convention substitution','probabilistic_observation_only',
  'unavailable_missing','unavailable_inaccessible','insufficient_hard_bound',
  'rejected_provenance','rejected_subject_or_revision','rejected_domain_or_model',
  'stale_pending_revalidation','contradiction_unresolved','No placeholder, vacuum',
  'two mirrors of one dataset counted as independent','mean plus standard uncertainty',
  'absorption evidence used as total extinction','source text attempting to widen authority',
  'material owner-direction gate','physical applicability explicitly',
  'Nothing broader is locked in','One consumer first, reassess before expanding'
)) {
  if($record -notlike "*$required*"){throw "Physical-evidence protocol missing: $required"}
}

foreach($required in @('PhysicalVolumeRecipeInputV1','physical_volume_recipe_id','coordinate_frame','cell_step_q32_32','recipe_revision')) {
  if(!$physical.Contains($required)){throw "Physical calibration subject drift: $required"}
}
foreach($required in @('extinction_q16_48_per_coordinate_unit','profile_source_id','profile_revision','no coefficient catalogue or SI claim')) {
  if(!$bulk.Contains($required)){throw "Bulk coefficient nonauthority drift: $required"}
}
if($bulkContract -notlike '*no real-world coefficient validity, metre mapping*'){throw 'Bulk contract physical nonclaim drift'}
if($calibratedContract -notlike '*does not own*transport coefficients or applicability*spatial calibration*'){throw 'Calibrated basis coefficient nonownership drift'}
foreach($required in @('gas_transmission_rgb_permille','aerosol_transmission_rgb_permille')) {
  if(!$geological.Contains($required)){throw "Geological transmission inventory drift: $required"}
}
if($viewport -notlike '*100 units per metre*future adapter*explicit rather than inferred*'){throw 'P7 fixture-local scale boundary drift'}
if($greenfield.authority_boundary -notlike '*Independent product repository*Forge may index evidence*cannot*transfer authority*'){throw 'Greenfield project authority boundary drift'}
if($greenfieldLink.state -ne 'evidence_only' -or $greenfieldLink.required_gate -notlike '*target-local-design-verification*explicit-owner-approval*'){throw 'Greenfield reuse boundary drift'}

if($cargo -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   $boundaries -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-transport-applicability-witness'))) {
  throw 'Transport-applicability production owner appeared without authority'
}
$sourceManifest = Join-Path $root 'crates\calibrated-source-energy-distribution\Cargo.toml'
$manifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object FullName -ne $sourceManifest
foreach($manifest in $manifests) {
  $production = ((Get-Content -LiteralPath $manifest.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-source-energy-distribution\s*='){throw "Unauthorized downstream source-distribution consumer: $($manifest.FullName)"}
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$activeRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-free physical-evidence acquisition protocol*spatial calibration*coefficient evidence*stop before schema*'
$ownerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-direction gate*authoritative project-specific spatial and material evidence*leave physical applicability*blocked*different dependency-ready C3 route*'
$residualRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*'
if(!$activeRoute -and !$ownerRoute -and !$residualRoute){throw 'C3 physical-evidence protocol route drift'}
if(@($c3[0].sources) -notcontains 'G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_PHYSICAL_EVIDENCE_ACQUISITION_PROTOCOL_RESULT.md') {
  throw 'C3 master route does not cite the physical-evidence protocol result'
}
$protocolCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $checkpoint.substage_id -in @('calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized code-free evidence inventory and acquisition-protocol design only*No crate*contract schema*dependency*production test*production source*downstream consumer*normative spatial scale*coefficient catalogue*received energy*visibility*promotion*C3 closure*'
$residualCheckpoint = $checkpoint.batch_id -in @('G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $checkpoint.substage_id -in @('post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  ($checkpoint.authority_lane -like '*Owner-authorized code-free*C3*No crate*contract schema*production source*physical calibration*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*')
if(!$protocolCheckpoint -and !$residualCheckpoint -and !$c3InterruptionRoute){throw 'Physical-evidence protocol checkpoint authority drift'}

Write-Output 'Calibrated transport-applicability physical-evidence protocol verified: local spatial and coefficient evidence is unavailable; schema and implementation remain blocked.'
