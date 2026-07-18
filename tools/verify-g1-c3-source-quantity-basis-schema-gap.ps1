Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_QUANTITY_BASIS_SCHEMA_GAP_AUDIT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$closure = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_CLOSURE_REGISTER.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach ($required in @(
  'gap confirmed',
  'irradiance_*_millionths_earth',
  'normalized potential',
  'Identity correlation only',
  'validator rejects zero',
  'Projection DTO',
  'No repository declaration of',
  'receiver-plane',
  'Semantic reuse and implementation reuse are separate decisions',
  'Canonical zero',
  'Missing physical design decisions',
  '4,096 cells',
  'source-quantity-basis',
  'mathematical design audit',
  'Add no crate, dependency, contract schema',
  'Nothing broader is locked in. One consumer first, reassess before expanding.'
)) {
  if ($audit -notlike "*$required*") { throw "Source-quantity basis gap audit is missing: $required" }
}

$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or
    ($c3[0].next_action -notlike '*source-quantity-basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated spectral/time basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated-basis*transport-applicability schema gap audit*' -and
     $c3[0].next_action -notlike '*implementation-readiness audit*source-calibration sibling*' -and
     $c3[0].next_action -notlike '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
     $c3[0].next_action -notlike '*explicit owner decision*calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*owner-approved bounded calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -or
    ($c3[0].proof -notlike '*no current owner*physical source quantity*' -and
     $c3[0].proof -notlike '*source-quantity basis mathematical design*band/time-integrated radiant energy*' -and
     $c3[0].proof -notlike '*calibrated spectral/time mathematical design*versioned witness*' -and
     $c3[0].proof -notlike '*calibrated-basis and transport-applicability*stateless derived commitment*' -and
     $c3[0].proof -notlike '*closed-frontier additive calibrated radiant-energy measure*exact phase-space root*' -and
     $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and
     $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and
     $c3[0].proof -notlike '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*')) {
  throw 'C3 does not retain the confirmed source-quantity basis gap and mathematical-design route.'
}
if ($closure -notlike '*source-quantity basis*mathematical design audit*' -and
    $closure -notlike '*source-quantity oracle selects band/time-integrated radiant energy*calibrated spectral/time basis mathematical design audit*' -and
    $closure -notlike '*source quantity is band/time-integrated radiant energy*calibration witness*transport-applicability schema gap audit*' -and
    $closure -notlike '*source quantity and additive calibration witness*source-calibration sibling*implementation readiness*' -and
    $closure -notlike '*calibrated transport-applicability schema-gap audit*scale ambiguity*spectral/time ambiguity*' -and
    $closure -notlike '*code-free acquisition protocol*both required local physical-evidence families unavailable*material owner decision*' -and
    $closure -notlike '*residual-obligation audit*physical applicability remains evidence-blocked*ecotone*') {
  throw 'Master closure register does not retain the physical source-quantity gap.'
}
$authorityBoundary = $checkpoint.authority_lane -like '*No crate*contract schema*production source*unit selection*watts*joules*radiance*detector*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*No crate*contract schema*production source*RGB boundaries*tick duration*detector*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*No crate*contract schema*production source*normative wavelength*tick duration*spatial scale*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$authorityBoundary = $authorityBoundary -or ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
  $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*')
$authorityBoundary = $authorityBoundary -or $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$designContinuation = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if (($checkpoint.batch_id -notin @('G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1') -or
    $checkpoint.substage_id -notin @('source-quantity-basis-schema-gap-audit','source-quantity-basis-schema-gap-result','source-quantity-basis-design-and-oracle','source-quantity-basis-oracle-result','calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result','calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result','source-calibration-owner-gate','source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result','calibrated-source-energy-distribution-oracle-result','calibrated-source-energy-distribution-owner-gate','calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result','calibrated-transport-applicability-witness-schema-gap-result') -or
    -not $authorityBoundary) -and !$designContinuation -and !$c3InterruptionRoute) {
  throw 'Active checkpoint does not preserve the source-quantity gap-audit boundary.'
}

Write-Output 'Source-quantity basis/schema gap verified: existing normalized and exact owners cannot supply physical source semantics; separate mathematical design remains code-free.'
