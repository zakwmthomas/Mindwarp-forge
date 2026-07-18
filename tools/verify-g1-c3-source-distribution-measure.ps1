Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_DISTRIBUTION_MEASURE_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_DISTRIBUTION_MEASURE_ORACLE_RESULT.md'
$oraclePath = Join-Path $root 'tools\prove-g1-c3-source-distribution-measure.py'
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$closure = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_CLOSURE_REGISTER.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach ($required in @(
  'abstract additive source-quantity measure',
  'nu(parent) = nu(lower_child) + nu(upper_child)',
  'projected physical area',
  'solid angle',
  'coordinate-rescaling',
  'unproved double counting',
  '4, 16 and 64',
  'quantity-basis-and-schema gap audit',
  'Do not claim watts, radiance, received power',
  'C3 closure'
)) {
  if ($design -notlike "*$required*") { throw "Source-distribution design is missing: $required" }
}

foreach ($required in @(
  'passed as an abstract additive source-quantity measure',
  'byte-identical',
  'a1eea7cead874d74d7c2bcd87f7020577bbe58469dd9d899ab61cc8aec73090a',
  '8bbde156aefe052dd473a149fc897da5595b7674e942b8126b102360b0570a25',
  'Hostile rejections:',
  'code-facing source-quantity-basis',
  'gap audit',
  'Do not proceed directly to implementation',
  'Nothing broader is locked in. One consumer first, reassess before expanding.'
)) {
  if ($result -notlike "*$required*") { throw "Source-distribution result is missing: $required" }
}

$hash = (Get-FileHash -LiteralPath $oraclePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($hash -ne 'a1eea7cead874d74d7c2bcd87f7020577bbe58469dd9d899ab61cc8aec73090a') {
  throw "Source-distribution oracle source hash drifted: $hash"
}

$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (!(Test-Path -LiteralPath $python)) { $python = 'python' }
$first = & $python $oraclePath
if ($LASTEXITCODE -ne 0) { throw 'Source-distribution oracle first run failed.' }
$second = & $python $oraclePath
if ($LASTEXITCODE -ne 0) { throw 'Source-distribution oracle second run failed.' }
if ($first -cne $second) { throw 'Source-distribution oracle receipt is nondeterministic.' }
$receipt = $first | ConvertFrom-Json
if ($receipt.status -ne 'pass' -or $receipt.checksum -ne '8bbde156aefe052dd473a149fc897da5595b7674e942b8126b102360b0570a25' -or
    $receipt.hostile_rejections -ne 21 -or $receipt.geometric_conservation_checks -ne 3 -or
    $receipt.quantity_conservation_checks -ne 3 -or
    (@($receipt.subdivision_children) -join ',') -ne '4,16,64') {
  throw 'Source-distribution oracle receipt does not match the pinned result.'
}

$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or
    ($c3[0].next_action -notlike '*code-facing source-quantity-basis*schema gap audit*' -and
     $c3[0].next_action -notlike '*source-quantity-basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated spectral/time basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated-basis*transport-applicability schema gap audit*' -and
     $c3[0].next_action -notlike '*implementation-readiness audit*source-calibration sibling*' -and
     $c3[0].next_action -notlike '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
     $c3[0].next_action -notlike '*explicit owner decision*calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*owner-approved bounded calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -or
    $c3[0].sources -notcontains 'G1_C3_SOURCE_DISTRIBUTION_MEASURE_MATHEMATICAL_DESIGN_AUDIT.md' -or
    $c3[0].sources -notcontains 'G1_C3_SOURCE_DISTRIBUTION_MEASURE_ORACLE_RESULT.md') {
  throw 'C3 does not retain the source-distribution result and next bounded gap audit.'
}
if ($closure -notlike '*additive source-quantity measure*quantity-basis and schema gap audit*' -and
    $closure -notlike '*additive abstract source-quantity representation*source-quantity basis/schema audit*mathematical design audit*' -and
    $closure -notlike '*source-quantity oracle selects band/time-integrated radiant energy*calibrated spectral/time basis mathematical design audit*' -and
    $closure -notlike '*source quantity is band/time-integrated radiant energy*calibration witness*transport-applicability schema gap audit*' -and
    $closure -notlike '*source quantity and additive calibration witness*source-calibration sibling*implementation readiness*' -and
    $closure -notlike '*calibrated transport-applicability schema-gap audit*scale ambiguity*spectral/time ambiguity*' -and
    $closure -notlike '*code-free acquisition protocol*both required local physical-evidence families unavailable*material owner decision*' -and
    $closure -notlike '*residual-obligation audit*physical applicability remains evidence-blocked*ecotone*') {
  throw 'Master closure register does not retain the source-distribution result and next gap.'
}
$authorityBoundary = $checkpoint.authority_lane -like '*No crate*production source*watts*radiance*detector*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*No crate*contract schema*production source*RGB boundaries*tick duration*detector*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*No crate*contract schema*production source*normative wavelength*tick duration*spatial scale*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*' -or
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$authorityBoundary = $authorityBoundary -or $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$authorityBoundary = $authorityBoundary -or $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$designContinuation = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if (($checkpoint.batch_id -notin @('G1-C3-SOURCE-DISTRIBUTION-MEASURE-MATHEMATICAL-DESIGN-V1','G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1') -or
    $checkpoint.substage_id -notin @('source-distribution-measure-design-and-oracle','source-distribution-measure-oracle-result','source-quantity-basis-schema-gap-audit','source-quantity-basis-schema-gap-result','source-quantity-basis-design-and-oracle','source-quantity-basis-oracle-result','calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result','calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result','source-calibration-owner-gate','source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result','calibrated-source-energy-distribution-oracle-result','calibrated-source-energy-distribution-owner-gate','calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result','calibrated-transport-applicability-witness-schema-gap-result') -or
    -not $authorityBoundary) -and !$designContinuation -and !$c3InterruptionRoute) {
  throw 'Active checkpoint does not preserve the source-distribution authority boundary.'
}

Write-Output "Source-distribution measure verified: exact additive conservation survives; physical quantity basis and implementation remain gated. receipt=$first"
