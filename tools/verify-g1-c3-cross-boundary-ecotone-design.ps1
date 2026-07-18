Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$recordPath = Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_MATHEMATICAL_DESIGN_AUDIT.md'
if(!(Test-Path -LiteralPath $recordPath)){throw 'C3 cross-boundary ecotone design audit is missing'}
$record = Get-Content -LiteralPath $recordPath -Raw
$plan = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PLAN_V2.md') -Raw
$derivedContract = Get-Content -LiteralPath (Join-Path $root 'contracts\derived-world-rules-contract.md') -Raw
$regionalContract = Get-Content -LiteralPath (Join-Path $root 'contracts\regional-environment-state-contract.md') -Raw
$partitionContract = Get-Content -LiteralPath (Join-Path $root 'contracts\physical-region-partition-contract.md') -Raw
$materialContract = Get-Content -LiteralPath (Join-Path $root 'contracts\surface-material-state-contract.md') -Raw
$derivedSource = Get-Content -LiteralPath (Join-Path $root 'crates\derived-world-rules\src\lib.rs') -Raw
$derivedManifest = Get-Content -LiteralPath (Join-Path $root 'crates\derived-world-rules\Cargo.toml') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$closure = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_CLOSURE_REGISTER.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @(
  'evidence-preserving typed-boundary witness selected','not a blending algorithm',
  'physical-region identity cannot become a causal palette operand',
  'Existing owner inventory','regional-environment-state','physical-region-partition',
  'surface-material-state','derived-world-rules','Unavailable regional evidence is distinct from numeric zero',
  'Candidate comparison','direct categorical painting','fixed-width blending','cause-scaled mixing',
  'evidence-preserving typed boundary','reject','defer','select',
  'Exact pointwise causal model','500,000,000','1,000,000,000','round-half-up',
  'categorical-independence law','Typed shared-edge witness','continuous_cause_exact',
  'sharp_cause_exact','unavailable_evidence','contradictory_evidence','provenance_mismatch',
  'noncanonical_input','arithmetic_out_of_range','unsupported_join',
  'does not claim a continuous interpolant','Independent disposable oracle',
  'imports no Forge production crate','arbitrary integers','Fraction','65,536-cell ceiling',
  '257 x 256','Hostile falsifier portfolio','label-only split','disconnected islands',
  'numeric zero exposure','explicit material-interface fixture','steep continuous-source gradient',
  '999 * 1000 * 1000 * 500','narrow-intermediate wrap','Stale partition receipts',
  'contradictory_evidence','bounded domain edges','recursive-blend negative control',
  'fixed-width negative control','Pass rule and claim ceiling','metres-per-coordinate-unit',
  'no contract schema, crate, dependency, production test','Physical applicability remains',
  'Nothing broader is locked in','One consumer first, reassess before expanding'
)) {
  if($record -notlike "*$required*"){throw "Ecotone design audit missing: $required"}
}

foreach($required in @('cross-boundary gradient','no-visible-seam','sharp-cause retention','reversal')) {
  if($plan -notlike "*$required*"){throw "Master C3 ecotone obligation drift: $required"}
}
if($derivedContract -notlike '*palette*stellar*atmospheric*surface-material reflectance*regional exposure*'){throw 'Derived-world causal palette contract drift'}
if($derivedContract -like '*physical-region*' -or $derivedManifest -like '*physical-region-partition*'){throw 'Categorical partition leaked into derived-world causal owner'}
if($regionalContract -notlike '*Q32.32*coordinate*0..1000*' -or $regionalContract -notlike '*not measured*physical visibility*'){throw 'Regional continuous-source boundary drift'}
if($partitionContract -notlike '*Unavailable evidence*distinct*never numeric zero*' -or $partitionContract -notlike '*Disconnected islands*remain distinct*'){throw 'Partition categorical/unavailable boundary drift'}
if($materialContract -notlike '*reflectance*' -or $materialContract -notlike '*integer*permille*' -or $materialContract -notlike '*not a measured*spectrum*biomes*'){throw 'Surface-material evidence boundary drift'}
if($derivedSource -notmatch '\(product \+ 500_000_000\) / 1_000_000_000'){throw 'Derived-world exact palette rounding relation drift'}
if($derivedSource -notlike '*bounded_stellar_irradiance_rgb_permille*atmosphere_transmission_rgb_permille*dominant_surface_reflectance_rgb_permille*exposure_permille*'){throw 'Derived-world exact causal operands drift'}

$c3 = @($program.items | Where-Object id -eq 'C3')
$route = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*ecotone*oracle*implementation*' -and
  $c3[0].proof -like '*evidence-preserving typed-boundary*' -and
  $c3[0].proof -like '*semantic*digest*'
if(!$route){throw 'C3 post-design route drift'}
if($c3[0].proof -notlike '*ecotone*evidence-preserving typed-boundary*'){throw 'C3 route omits ecotone design result'}
if($closure -notlike '*evidence-preserving typed-boundary*independent*oracle*'){throw 'Closure register ecotone route drift'}

$active = (($checkpoint.batch_id -in @('G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
    $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
    $checkpoint.authority_lane -like '*code-free*C3 cross-boundary ecotone mathematical design*No crate*contract schema*production*renderer*biome*organism*runtime*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1' -and
    $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
    ($checkpoint.authority_lane -like '*C3 ecotone-oracle implementation-readiness*No Python oracle*crate*contract schema*dependency*production*renderer*biome*organism*runtime*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production*renderer*biome*organism*runtime*promotion*C3 closure*')) -or
  ($checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1' -and
    $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
    $checkpoint.authority_lane -like '*Owner-approved disposable C3 ecotone oracle implementation only*No crate*contract schema*dependency*production*renderer*biome*organism*runtime*promotion*C3 closure*'))
if(!$active -and !$c3InterruptionRoute){throw 'Ecotone mathematical-design checkpoint authority drift'}

Write-Output 'C3 cross-boundary ecotone design verified: categorical labels are excluded from causal palette output, explicit sharp causes survive, and implementation remains gated.'
