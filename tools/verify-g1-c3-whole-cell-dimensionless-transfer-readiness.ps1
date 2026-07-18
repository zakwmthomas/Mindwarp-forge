Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_IMPLEMENTATION_READINESS.md'
if (-not (Test-Path -LiteralPath $path)) { throw 'Whole-cell transfer readiness record is missing.' }
$readiness = Get-Content -LiteralPath $path -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json

foreach ($required in @(
  'ready for one exact serious owner implementation decision',
  'Nothing broader is locked in',
  'Implement one consumer first',
  'BulkOpticalDepthEvaluationInputV1',
  'unchanged kernel',
  'optical-phase-space-dimensionless-transfer',
  'mindwarp.optical-phase-space.band-time.v1',
  'CertifiedAcceptedFiniteTransfer',
  'selected-partial opacity',
  'maximum finite summed raw optical-depth bits: 118',
  'complete input bytes before decode: 128 MiB',
  'aggregate live canonical bytes: 192 MiB',
  'i686-pc-windows-msvc',
  'aarch64-linux-android',
  'Android remains compilation evidence only',
  'Rollback is deletion-only',
  'Without explicit approval, add no crate',
  'closure authority'
)) {
  if ($readiness -notlike "*$required*") { throw "Whole-cell transfer readiness drift: $required" }
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$route = $c3.Count -eq 1 -and
  ($c3[0].next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -or
   $c3[0].next_action -like '*approve or reject*optical-phase-space-dimensionless-transfer*' -or
   $c3[0].next_action -like '*owner-authorized*whole-cell dimensionless-transfer*' -or
   $c3[0].next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*' -or
   $c3[0].next_action -like '*code-free source-distribution*phase-space-measure compatibility*' -or
   $c3[0].next_action -like '*code-facing source-quantity-basis*schema gap audit*' -or
   $c3[0].next_action -like '*source-quantity-basis mathematical design audit*' -or
   $c3[0].next_action -like '*calibrated spectral/time basis mathematical design audit*' -or
   $c3[0].next_action -like '*calibrated-basis*transport-applicability schema gap audit*' -or
   $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*' -or
   $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -or
   $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -or
   $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -or
   $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*')
if (-not $route -or
    $c3[0].sources -notcontains 'G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_IMPLEMENTATION_READINESS.md') {
  throw 'C3 does not retain the whole-cell transfer readiness route.'
}
$readinessRoute = $checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.substage_id -in @('whole-cell-dimensionless-transfer-implementation-readiness','whole-cell-dimensionless-transfer-owner-gate') -and
  $checkpoint.authority_lane -like '*Readiness audit and exact owner gate only*No crate*production source*source magnitude*detector response*visibility*C3 closure*'
$implementationRoute = $checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('bulk-optical-depth-evaluation-implementation','whole-cell-dimensionless-transfer-sibling-implementation','whole-cell-dimensionless-transfer-verification','whole-cell-dimensionless-transfer-result') -and
  $checkpoint.authority_lane -like '*Owner-approved additive bulk evaluation*downstream dimensionless-transfer sibling only*No existing V1 migration*source magnitude*detector response*visibility*promotion*C3 closure*'
$postResultRoute = $checkpoint.batch_id -eq 'G1-C3-POST-WHOLE-CELL-DIMENSIONLESS-TRANSFER-CONSUMER-REASSESSMENT-V1' -and
  $checkpoint.substage_id -eq 'post-whole-cell-dimensionless-transfer-consumer-reassessment' -and
  $checkpoint.authority_lane -like '*Static reassessment*No crate*source distribution*detector*visibility*promotion*C3 closure*'
$sourceMeasureRoute = $checkpoint.batch_id -eq 'G1-C3-SOURCE-DISTRIBUTION-MEASURE-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -in @('source-distribution-measure-design-and-oracle','source-distribution-measure-oracle-result') -and
  $checkpoint.authority_lane -like '*exact-rational oracle*No crate*production source*watts*radiance*detector*visibility*promotion*C3 closure*'
$sourceQuantityGapRoute = $checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1' -and
  $checkpoint.substage_id -in @('source-quantity-basis-schema-gap-audit','source-quantity-basis-schema-gap-result') -and
  $checkpoint.authority_lane -like '*read-only gap audit only*No crate*contract schema*production source*unit selection*watts*joules*radiance*detector*visibility*promotion*C3 closure*'
$sourceQuantityBasisRoute = $checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -in @('source-quantity-basis-design-and-oracle','source-quantity-basis-oracle-result') -and
  $checkpoint.authority_lane -like '*mathematical design*exact-rational oracle only*No crate*contract schema*production source*RGB boundaries*tick duration*detector*visibility*promotion*C3 closure*'
$calibratedBasisRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -in @('calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result') -and
  $checkpoint.authority_lane -like '*Code-free mathematical design*exact-rational oracle only*No crate*contract schema*production source*normative wavelength*tick duration*spatial scale*visibility*promotion*C3 closure*'
$calibratedGapRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1' -and
  $checkpoint.substage_id -in @('calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result') -and
  $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*'
$sourceCalibrationRoute = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1' -and
  $checkpoint.substage_id -eq 'source-calibration-owner-gate' -and
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$sourceCalibrationImplementationRoute = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
  $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$distributionResultRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapRoute = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignRoute = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$checkpointRoute = $readinessRoute -or $implementationRoute -or $postResultRoute -or $sourceMeasureRoute -or $sourceQuantityGapRoute -or $sourceQuantityBasisRoute -or $calibratedBasisRoute -or $calibratedGapRoute -or $sourceCalibrationRoute -or $sourceCalibrationImplementationRoute -or $distributionResultRoute -or $distributionReadinessRoute -or $distributionImplementationRoute -or $transportGapRoute -or $transportDesignRoute
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
$checkpointRoute = $checkpointRoute -or $c3InterruptionRoute
if (-not $checkpointRoute) { throw 'Whole-cell transfer readiness checkpoint or authority boundary drifted.' }
if (-not ($implementationRoute -or $postResultRoute -or $sourceMeasureRoute -or $sourceQuantityGapRoute -or $sourceQuantityBasisRoute -or $calibratedBasisRoute -or $calibratedGapRoute -or $sourceCalibrationRoute -or $sourceCalibrationImplementationRoute -or $distributionResultRoute -or $distributionReadinessRoute -or $distributionImplementationRoute -or $transportGapRoute -or $transportDesignRoute -or $c3InterruptionRoute) -and (Test-Path -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer'))) {
  throw 'Whole-cell transfer source appeared before exact approval.'
}

Write-Output 'Whole-cell dimensionless-transfer readiness verified: additive bulk-owned evaluation, downstream binding/composition, 118-bit arithmetic, caps, platforms and deletion rollback are frozen behind the exact owner gate.'
