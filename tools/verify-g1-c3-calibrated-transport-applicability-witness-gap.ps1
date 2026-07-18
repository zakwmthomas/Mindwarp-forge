Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_WITNESS_SCHEMA_GAP_AUDIT.md'
if(!(Test-Path -LiteralPath $auditPath)){throw 'Calibrated transport-applicability audit is missing'}
$audit = Get-Content -LiteralPath $auditPath -Raw
$physical = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\src\lib.rs') -Raw
$physicalModule = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\MODULE.md') -Raw
$bulk = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs') -Raw
$bulkModule = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-bulk-transfer\MODULE.md') -Raw
$bulkContract = Get-Content -LiteralPath (Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md') -Raw
$dimensionless = Get-Content -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\src\lib.rs') -Raw
$dimensionlessModule = Get-Content -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\MODULE.md') -Raw
$source = Get-Content -LiteralPath (Join-Path $root 'crates\calibrated-source-energy-distribution\src\lib.rs') -Raw
$sourceModule = Get-Content -LiteralPath (Join-Path $root 'crates\calibrated-source-energy-distribution\MODULE.md') -Raw
$sourceContract = Get-Content -LiteralPath (Join-Path $root 'contracts\calibrated-source-energy-distribution-contract.md') -Raw
$transport = Get-Content -LiteralPath (Join-Path $root 'crates\optical-phase-space-transport-certificate\src\lib.rs') -Raw
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @(
  'Delta from the earlier audit','Current identity inventory','Physical spatial unit',
  'Coefficient and spectral/time applicability','Exact subject and path',
  'Applicability disposition','Correction and validity','Scale ambiguity',
  'Spectral/time ambiguity','Subject mismatch','Add an adapter over only opaque IDs - reject',
  'Future separate applicability sibling - only coherent boundary',
  'certified_everywhere','conservatively_unresolved','A record shape without the physical evidence',
  'Same cell identity is necessary but insufficient','no within-cell energy uniformity',
  'no current owner allocates source joules','not a physical coefficient enclosure',
  'separate joint source/coupling integration proof','not a present validation defect',
  'Stop before schema or mathematical implementation readiness','Nothing broader is locked in'
)) {
  if($audit -notlike "*$required*"){throw "Transport-applicability audit missing: $required"}
}
foreach($required in @('pub enum CoordinateFrameV1','CartesianQ32_32Volume3dV1','pub scope_id: Id','pub reconstruction_id: Id')) {
  if(!$physical.Contains($required)){throw "Physical identity inventory drift: $required"}
}
if($physicalModule -notlike '*opacity propagation visibility detectability or signal range*'){throw 'Physical owner boundary drift'}
foreach($required in @('extinction_q16_48_per_coordinate_unit','pub scope_id: Id','pub reconstruction_id: Id','if input.reconstruction_id != input.physical_volume_recipe_input.reconstruction_id')) {
  if(!$bulk.Contains($required)){throw "Bulk identity inventory drift: $required"}
}
if($bulkModule -notlike '*coefficient discovery catalogues SI calibration or metre mapping*'){throw 'Bulk calibration nonownership drift'}
if($bulkContract -notlike '*no real-world coefficient validity, metre mapping*'){throw 'Bulk physical-calibration exclusion drift'}
foreach($required in @('pub visible_radiance_bulk_profile_id: [u8; 32]','pub band_time_id: [u8; 32]','input.band_time_binding.band_time_id != transport.band_time_id','input.band_time_binding.band_time_id != certificate.band_time_id')) {
  if(!$dimensionless.Contains($required)){throw "Dimensionless identity inventory drift: $required"}
}
if($dimensionlessModule -notlike '*source magnitude radiance emission inverse-square spreading or energy transport*'){throw 'Dimensionless owner boundary drift'}
foreach($required in @('pub subject_id: [u8; 32]','calibrated_basis_id','band_time_id','reconstruction_id','pub joules: ExactRadiantEnergyV1')) {
  if(!$source.Contains($required)){throw "Source identity inventory drift: $required"}
}
if($sourceModule -notlike '*transport applicability attenuation receiver coupling detector response or visibility*'){throw 'Source owner boundary drift'}
if($sourceContract -notlike '*neither stored in identity nor evidence of within-cell*uniformity*'){throw 'Source nonuniformity boundary drift'}
foreach($required in @('pub accepted_measure: ExactMeasureV1','pub zero_measure: ExactMeasureV1','pub unresolved_measure: ExactMeasureV1')) {
  if(!$dimensionless.Contains($required)){throw "Dimensionless measure partition drift: $required"}
}
foreach($required in @('pub cell: OpticalPhaseSpaceCellV1','pub physical_volume_recipe: PhysicalVolumeRecipeV1','pub band_time_id: [u8; 32]','input.cell.scope_id != input.physical_volume_recipe.input.scope_id','input.cell.reconstruction_id != input.physical_volume_recipe.input.reconstruction_id')) {
  if(!$transport.Contains($required)){throw "Transport subject join drift: $required"}
}

if($cargo -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   $boundaries -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-transport-applicability-witness'))) {
  throw 'Transport-applicability owner appeared before authorization'
}
$sourceManifest = Join-Path $root 'crates\calibrated-source-energy-distribution\Cargo.toml'
$manifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object FullName -ne $sourceManifest
foreach($manifest in $manifests) {
  $production = ((Get-Content -LiteralPath $manifest.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-source-energy-distribution\s*='){throw "Unauthorized downstream source-distribution consumer: $($manifest.FullName)"}
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$selectedRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-authorized code-free primary-evidence*mathematical-design audit*' -and
  $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*subject mismatch*separate capability-free applicability sibling*'
$evidenceRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*code-free physical-evidence acquisition protocol*spatial calibration*coefficient evidence*stop before schema*' -and
  $c3[0].proof -like '*mathematical-design audit passes*Project-specific*implementation readiness remains blocked*'
$protocolResultRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*material owner-direction gate*authoritative project-specific spatial and material evidence*leave physical applicability*blocked*different dependency-ready C3 route*' -and
  $c3[0].proof -like '*physical-evidence acquisition protocol passes*unavailable_evidence*schema*implementation readiness*remain blocked*'
$residualRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*' -and $c3[0].proof -like '*unavailable_evidence*physical calibration*received energy*visibility*remain blocked*'
$priorRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*calibrated transport-applicability witness schema gap audit*'
if(!$selectedRoute -and !$evidenceRoute -and !$protocolResultRoute -and !$residualRoute -and !$priorRoute){throw 'C3 transport-applicability route drift'}
$resultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and
  $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and
  $checkpoint.state -eq 'checkpoint' -and
  $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$priorCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-result'
$designCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-evidence-and-mathematical-design','calibrated-transport-applicability-witness-mathematical-design-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized code-free primary research and mathematical design only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$protocolCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $checkpoint.substage_id -in @('calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized code-free evidence inventory and acquisition-protocol design only*No crate*contract schema*dependency*production test*production source*downstream consumer*normative spatial scale*coefficient catalogue*received energy*visibility*promotion*C3 closure*'
$residualCheckpoint = $checkpoint.batch_id -in @('G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and ($checkpoint.authority_lane -like '*Owner-authorized code-free*C3*No crate*contract schema*production source*physical calibration*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*')
if(!$resultCheckpoint -and !$designCheckpoint -and !$protocolCheckpoint -and !$residualCheckpoint -and !$priorCheckpoint -and !$c3InterruptionRoute){throw 'Transport-applicability checkpoint authority drift'}
Write-Output 'Calibrated transport-applicability witness gap verified: scale, coefficient, exact-subject and correction joins remain absent; existing owners and zero-consumer boundaries remain unchanged.'
