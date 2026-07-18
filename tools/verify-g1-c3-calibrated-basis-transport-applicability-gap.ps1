Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_BASIS_TRANSPORT_APPLICABILITY_SCHEMA_GAP_AUDIT.md') -Raw
$dimensionless = Get-Content -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\src\lib.rs') -Raw
$dimensionlessModule = Get-Content -LiteralPath (Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\MODULE.md') -Raw
$bulk = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs') -Raw
$bulkModule = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-bulk-transfer\MODULE.md') -Raw
$bulkContract = Get-Content -LiteralPath (Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md') -Raw
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @('no current owner or schema','Current-owner inventory','Mutate','OpticalBandTimeBindingV1','One combined calibration/applicability sibling - reject','Separate source-calibration sibling - select for readiness audit only','Uniqueness without a registry','stateless derived commitment','Existing arbitrary V1 time','Minimum source-calibration schema obligations','No current 4 KiB, 64 KiB or 128 MiB ceiling automatically applies','Transport-applicability gap','deletion-only','implementation-readiness audit','Do not implement now','Nothing broader is locked in')) {
  if($audit -notlike "*$required*"){throw "Calibrated-basis/applicability gap audit missing: $required"}
}
foreach($required in @('pub struct OpticalBandTimeBindingV1','pub band: VisibleRadianceBandV1','pub time_basis_id: [u8; 32]','pub band_time_id: [u8; 32]','MAX_BAND_TIME_BINDING_BYTES: usize = 4 * 1024','json(&(band, time_basis_id))')) {
  if(!$dimensionless.Contains($required)){throw "Dimensionless-transfer inventory drift: $required"}
}
if($dimensionlessModule -notlike '*does not own*source magnitude radiance emission*energy transport*'){throw 'Dimensionless-transfer ownership boundary drift'}
if($bulk -notlike '*extinction_q16_48_per_coordinate_unit*'){throw 'Bulk coordinate-unit coefficient inventory drift'}
if($bulkModule -notlike '*coefficient discovery catalogues SI calibration or metre mapping*'){throw 'Bulk module nonownership drift'}
if($bulkContract -notlike '*no real-world coefficient validity, metre mapping*'){throw 'Bulk contract physical-calibration exclusion drift'}
$preAuthorization = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1'
if($preAuthorization -and ($cargo -like '*calibrated-spectral-time-basis*' -or $boundaries -like '*calibrated-spectral-time-basis*' -or (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-spectral-time-basis')))){throw 'Source-calibration owner appeared before authorization'}
if($dimensionless -match 'wavelength|seconds_per_tick|calibration_provenance|spatial_calibration|transport_applicability'){throw 'Physical calibration fields appeared inside dimensionless V1'}

$c3 = @($program.items | Where-Object id -eq 'C3')
if($c3.Count -ne 1 -or
   ($c3[0].next_action -notlike '*implementation-readiness audit*source-calibration sibling*' -and $c3[0].next_action -notlike '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and $c3[0].next_action -notlike '*explicit owner decision*calibrated-source-energy-distribution*' -and $c3[0].next_action -notlike '*owner-approved bounded calibrated-source-energy-distribution*' -and $c3[0].next_action -notlike '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].next_action -notlike '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*') -or
   ($c3[0].proof -notlike '*stateless derived commitment*Transport applicability remains blocked*' -and
    $c3[0].proof -notlike '*source-calibration implementation-readiness*stateless*Transport applicability remains blocked*' -and
    $c3[0].proof -notlike '*closed-frontier additive calibrated radiant-energy measure*Transport applicability remains blocked*' -and
    $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and
    $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and
    $c3[0].proof -notlike '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*' -and
    $c3[0].proof -notlike '*residual-obligation*physical visibility*C3-owned*evidence-blocked*')) {
  throw 'C3 calibrated-basis/applicability gap route drift'
}
$gapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1' -and
   $checkpoint.substage_id -in @('calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result') -and
   $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*'
$readinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1' -and
   $checkpoint.substage_id -eq 'source-calibration-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
   $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$distributionResultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-mathematical-design-result' -and $checkpoint.authority_lane -like '*Owner-authorized code-free primary research and mathematical design only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$protocolCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1' -and $checkpoint.substage_id -in @('calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$residualCheckpoint = $checkpoint.batch_id -in @('G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and ($checkpoint.authority_lane -like '*Owner-authorized code-free*C3*No crate*contract schema*production source*physical calibration*promotion*C3 closure*' -or $checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*')
if(!$gapCheckpoint -and !$readinessCheckpoint -and !$implementationCheckpoint -and !$distributionResultCheckpoint -and !$distributionReadinessCheckpoint -and !$distributionImplementationCheckpoint -and !$transportGapCheckpoint -and !$transportDesignCheckpoint -and !$protocolCheckpoint -and !$residualCheckpoint -and !$c3InterruptionRoute) {
  throw 'Calibrated-basis/applicability gap checkpoint authority drift'
}
Write-Output 'Calibrated-basis/applicability gap verified: a stateless source-calibration sibling may enter readiness; transport applicability remains blocked and no schema exists.'
