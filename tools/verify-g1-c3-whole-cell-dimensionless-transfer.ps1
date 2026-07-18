Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$design = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_MATHEMATICAL_DESIGN_AUDIT.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_ORACLE_RESULT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json

foreach ($required in @(
  'sum(a_i for i < k)',
  'sum(b_i for i <= k)',
  'selected partial step is opaque',
  'StartInside',
  'single attenuation evaluation',
  'numerical underflow, never opaque evidence',
  'band/time binding',
  'kernel is private',
  'Add no crate, contract schema, dependency'
)) {
  if ($design -notlike "*$required*") { throw "Whole-cell transfer design drift: $required" }
}
foreach ($required in @(
  '16 finite, vacuum, opaque',
  'nine exact sample-containment checks',
  'ten hostile subject/authority rejections',
  '4-, 16- and 64-child measure conservation',
  'central-lane substitution counterexample',
  '2^-96',
  'no schema or crate is authorized'
)) {
  if ($result -notlike "*$required*") { throw "Whole-cell transfer result drift: $required" }
}

$script = Join-Path $root 'tools\prove-g1-c3-whole-cell-dimensionless-transfer.py'
$sourceHash = (Get-FileHash -LiteralPath $script -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sourceHash -ne '486495cdee971e000cf8f326aa2b895247ce9f4b8ed35f5e5832723e516450c1') {
  throw 'Whole-cell transfer oracle source hash drifted.'
}
$bundledPython = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (Test-Path -LiteralPath $bundledPython) {
  $pythonPath = $bundledPython
} else {
  $python = Get-Command python -ErrorAction SilentlyContinue
  if ($null -eq $python) { throw 'Python runtime is unavailable.' }
  $pythonPath = $python.Source
}
$first = (& $pythonPath $script | Out-String).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Whole-cell transfer oracle failed.' }
$second = (& $pythonPath $script | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Whole-cell transfer oracle is nondeterministic.' }
$receipt = $first | ConvertFrom-Json
if ($receipt.status -ne 'pass' -or $receipt.portfolios -ne 16 -or
    $receipt.sample_containment_checks -ne 9 -or $receipt.hostile_rejections -ne 10 -or
    $receipt.checksum -ne 'b6cac30017fa55bee50aff399d61027b91c8696d96d4236508aaef612ca4dc91' -or
    $receipt.repeated_q0_48_underflow -ne 'typed_not_opaque' -or
    $receipt.maximum_finite_prefix_raw_bits -ne 118) {
  throw 'Whole-cell transfer oracle receipt drifted.'
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$c3Route =
  $c3.Count -eq 1 -and
  (($c3[0].next_action -like '*code-facing readiness/gap audit*band/time identity*exponential-kernel ownership*') -or
   ($c3[0].next_action -like '*Owner: approve or reject*bulk-owned optical-depth evaluation receipt*optical-phase-space-dimensionless-transfer sibling*') -or
   ($c3[0].next_action -like '*owner-authorized whole-cell dimensionless-transfer implementation*') -or
   ($c3[0].next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*') -or
   ($c3[0].next_action -like '*code-free source-distribution*phase-space-measure compatibility*') -or
   ($c3[0].next_action -like '*code-facing source-quantity-basis*schema gap audit*') -or
   ($c3[0].next_action -like '*source-quantity-basis mathematical design audit*') -or
   ($c3[0].next_action -like '*calibrated spectral/time basis mathematical design audit*') -or
   ($c3[0].next_action -like '*calibrated-basis*transport-applicability schema gap audit*') -or
   ($c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*') -or
   ($c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*') -or
   ($c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*') -or
   ($c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*') -or
   ($c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*'))
if ($c3.Count -ne 1 -or
    -not $c3Route -or
    $c3[0].sources -notcontains 'G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_MATHEMATICAL_DESIGN_AUDIT.md' -or
    $c3[0].sources -notcontains 'G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_ORACLE_RESULT.md') {
  throw 'C3 does not retain the verified whole-cell transfer route.'
}
$checkpointRoute =
  ($checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -in @('whole-cell-dimensionless-transfer-oracle-verification','whole-cell-dimensionless-transfer-oracle-result') -and
   $checkpoint.authority_lane -like '*Mathematical design and disposable exact-rational oracle only*No crate*source magnitude*detector response*visibility*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
   $checkpoint.substage_id -in @('whole-cell-dimensionless-transfer-implementation-readiness','whole-cell-dimensionless-transfer-owner-gate') -and
   $checkpoint.authority_lane -like '*Readiness audit and exact owner gate only*No crate*production source*source magnitude*detector response*visibility*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('bulk-optical-depth-evaluation-implementation','whole-cell-dimensionless-transfer-sibling-implementation','whole-cell-dimensionless-transfer-verification','whole-cell-dimensionless-transfer-result') -and
   $checkpoint.authority_lane -like '*Owner-approved additive bulk evaluation*downstream dimensionless-transfer sibling only*No existing V1 migration*source magnitude*detector response*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-POST-WHOLE-CELL-DIMENSIONLESS-TRANSFER-CONSUMER-REASSESSMENT-V1' -and
   $checkpoint.substage_id -eq 'post-whole-cell-dimensionless-transfer-consumer-reassessment' -and
   $checkpoint.authority_lane -like '*Static reassessment*No crate*source distribution*detector*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-SOURCE-DISTRIBUTION-MEASURE-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -in @('source-distribution-measure-design-and-oracle','source-distribution-measure-oracle-result') -and
   $checkpoint.authority_lane -like '*exact-rational oracle*No crate*production source*watts*radiance*detector*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1' -and
   $checkpoint.substage_id -in @('source-quantity-basis-schema-gap-audit','source-quantity-basis-schema-gap-result') -and
   $checkpoint.authority_lane -like '*read-only gap audit only*No crate*contract schema*production source*unit selection*watts*joules*radiance*detector*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -in @('source-quantity-basis-design-and-oracle','source-quantity-basis-oracle-result') -and
   $checkpoint.authority_lane -like '*mathematical design*exact-rational oracle only*No crate*contract schema*production source*RGB boundaries*tick duration*detector*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -in @('calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result') -and
   $checkpoint.authority_lane -like '*Code-free mathematical design*exact-rational oracle only*No crate*contract schema*production source*normative wavelength*tick duration*spatial scale*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1' -and
   $checkpoint.substage_id -in @('calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result') -and
   $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1' -and
   $checkpoint.substage_id -eq 'source-calibration-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
   $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and
   $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and
   $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and
   $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and
   $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*') -or
  ($checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
   $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
   $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*')
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
$checkpointRoute = $checkpointRoute -or $c3InterruptionRoute
if (-not $checkpointRoute) { throw 'Whole-cell transfer checkpoint or authority boundary drifted.' }
if ($checkpoint.batch_id -notin @('G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-V1','G1-C3-POST-WHOLE-CELL-DIMENSIONLESS-TRANSFER-CONSUMER-REASSESSMENT-V1','G1-C3-SOURCE-DISTRIBUTION-MEASURE-MATHEMATICAL-DESIGN-V1','G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1','G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1','G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
    -not $c3InterruptionRoute -and (Test-Path -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer'))) {
  throw 'Unauthorized whole-cell transfer crate appeared.'
}

Write-Output 'Whole-cell dimensionless transfer verified: exact optical-depth composition, receiver truncation, opacity, underflow, subject and measure shields pass without production authority.'
