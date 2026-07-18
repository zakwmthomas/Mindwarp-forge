Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_WITNESS_MATHEMATICAL_DESIGN_AUDIT.md'
if(!(Test-Path -LiteralPath $designPath)){throw 'Calibrated transport-applicability mathematical design is missing'}
$design = Get-Content -LiteralPath $designPath -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw

foreach($required in @(
  'capability-free applicability sibling','may not compute received energy',
  'BIPM SI Brochure','metrological traceability','coverage interval',
  'probabilistic','deterministic hard enclosure',
  'absorption, scattering or total extinction','complete calibrated wavelength, time',
  'alpha_coordinate = k * alpha_m','tau = integral(alpha_m ds)',
  'T in [exp(-tau_upper), exp(-tau_lower)]','physical optical-depth enclosure',
  'Minimum exact replay graph','Spatial calibration evidence','Coefficient applicability evidence',
  'Opaque-ID proximity','Whole-cell source theorem',
  'certified_everywhere_finite','certified_everywhere_opaque',
  'certified_everywhere_zero_coupling','certified_everywhere_vacuum_identity',
  'conservatively_unresolved','unavailable_evidence','Underflow remains finite',
  'Scale substitution','Spectral/time substitution','Opaque subject mismatch',
  'Source concentration','Endpoint escape','Correction replay',
  'at most 64 transport steps','64 coefficient','sub-512-bit live-arithmetic shield',
  'Do not invent byte caps','192 MiB aggregate-live ceiling',
  'Implementation readiness','code-free physical-evidence acquisition protocol',
  'Nothing broader is locked in','One consumer first, reassess before expanding'
)) {
  if($design -notlike "*$required*"){throw "Transport-applicability mathematical design missing: $required"}
}
foreach($url in @(
  'https://www.bipm.org/en/publications/si-brochure',
  'https://www.bipm.org/en/si-base-units/metre',
  'https://jcgm.bipm.org/vim/en/2.39.html',
  'https://jcgm.bipm.org/vim/en/2.41.html',
  'https://jcgm.bipm.org/vim/en/2.36.html',
  'https://goldbook.iupac.org/terms/view/A00037',
  'https://goldbook.iupac.org/terms/view/S05490',
  'https://nvlpubs.nist.gov/nistpubs/TechnicalNotes/NIST.TN.1889v1.pdf',
  'https://dlmf.nist.gov/3.1'
)) {
  if(!$design.Contains($url)){throw "Primary-source receipt missing: $url"}
}

if($cargo -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   $boundaries -match 'calibrated-transport-applicability|transport-applicability-witness' -or
   (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-transport-applicability-witness'))) {
  throw 'Transport-applicability owner appeared without schema or implementation authority'
}
$sourceManifest = Join-Path $root 'crates\calibrated-source-energy-distribution\Cargo.toml'
$manifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object FullName -ne $sourceManifest
foreach($manifest in $manifests) {
  $production = ((Get-Content -LiteralPath $manifest.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-source-energy-distribution\s*='){throw "Unauthorized downstream source-distribution consumer: $($manifest.FullName)"}
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$currentRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-authorized code-free primary-evidence*mathematical-design audit*separate capability-free calibrated transport-applicability witness*'
$resultRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*code-free physical-evidence acquisition protocol*spatial calibration*coefficient evidence*stop before schema*' -and
  $c3[0].proof -like '*mathematical-design audit passes*Project-specific*implementation readiness remains blocked*'
$protocolResultRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*material owner-direction gate*authoritative project-specific spatial and material evidence*leave physical applicability*blocked*different dependency-ready C3 route*' -and
  $c3[0].proof -like '*physical-evidence acquisition protocol passes*unavailable_evidence*schema*implementation readiness*remain blocked*'
$residualRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*' -and $c3[0].proof -like '*unavailable_evidence*physical calibration*received energy*visibility*remain blocked*'
if(!$currentRoute -and !$resultRoute -and !$protocolResultRoute -and !$residualRoute){throw 'C3 applicability mathematical-design route drift'}
if(@($c3[0].sources) -notcontains 'G1_C3_CALIBRATED_TRANSPORT_APPLICABILITY_WITNESS_MATHEMATICAL_DESIGN_AUDIT.md') {
  throw 'C3 master route does not cite the applicability mathematical-design result'
}
$designCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-evidence-and-mathematical-design','calibrated-transport-applicability-witness-mathematical-design-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized code-free primary research and mathematical design only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$protocolCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $checkpoint.substage_id -in @('calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized code-free evidence inventory and acquisition-protocol design only*No crate*contract schema*dependency*production test*production source*downstream consumer*normative spatial scale*coefficient catalogue*received energy*visibility*promotion*C3 closure*'
$residualCheckpoint = $checkpoint.batch_id -in @('G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and ($checkpoint.authority_lane -like '*Owner-authorized code-free*C3*No crate*contract schema*production source*physical calibration*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*')
if(!$designCheckpoint -and !$protocolCheckpoint -and !$residualCheckpoint -and !$c3InterruptionRoute){throw 'Applicability mathematical-design checkpoint authority drift'}

Write-Output 'Calibrated transport-applicability mathematical design verified: the pointwise hard-enclosure theorem survives; schema, implementation and received-energy authority remain blocked pending real evidence.'
