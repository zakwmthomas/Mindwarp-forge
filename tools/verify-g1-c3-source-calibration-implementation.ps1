Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\calibrated-spectral-time-basis'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\calibration.rs'
$fixturePath = Join-Path $crate 'fixtures\calibration_v1_identity_lock.json'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\calibrated-spectral-time-basis-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_CALIBRATION_IMPLEMENTATION_RESULT.md'
foreach($path in @($sourcePath,$testPath,$fixturePath,$manifestPath,$contractPath,$resultPath)) {
  if(!(Test-Path -LiteralPath $path)){throw "Source-calibration implementation artifact missing: $path"}
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$fixture = Get-Content -LiteralPath $fixturePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach($required in @('ExactUnsignedRationalV1','CalibratedSpectralTimeBasisInputV1','CalibratedSpectralTimeBasisV1','DerivedLegacyBandTimeIdsV1','MAX_DECIMAL_DIGITS: usize = 39','MAX_INPUT_BYTES: usize = 16 * 1024','MAX_RESULT_BYTES: usize = 32 * 1024','MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 64 * 1024','mindwarp.calibrated-spectral-time-basis.basis.v1','mindwarp.calibrated-spectral-time-basis.legacy-time-commitment.v1','mindwarp.optical-phase-space.band-time.v1','compare_rationals','deny_unknown_fields','none_evidence_only')) {
  if($source -notlike "*$required*"){throw "Source-calibration implementation drift: $required"}
}
foreach($required in @('exact_identity_codec_and_legacy_owner_lock','substitutions_change_basis_time_and_band_identities','rational_interval_time_and_identity_hostiles_fail_typed','strict_raw_codec_ceiling_and_forgery_shields','calibration_v1_identity_lock.json')) {
  if($tests -notlike "*$required*"){throw "Source-calibration test shield missing: $required"}
}
foreach($required in @('c28511856b8f17725deed954f9617a9097342716f8a2cae9349dd1e05ebeb727','ad6c05e1c5ec1c80060b58b989a6290cec342e50815eddeb271ca23ed54a821f','a9913e0d498c2e686574b1a755675d32ce0be3bdc59bf3335cb8d40716684a22','d70f6c4760dbcbd7be0e091c6473082b3b88f11dc9920940bd91c4d8e8c96a79')) {
  if($fixture -notlike "*$required*"){throw "Source-calibration identity fixture drift: $required"}
}
foreach($required in @('serde = { version = "1", features = ["derive"] }','serde_json = "1"','sha2 = "0.10"')) {
  if($manifest -notmatch [regex]::Escape($required)){throw "Source-calibration dependency drift: $required"}
}
$productionManifest = ($manifest -split '(?m)^\[dev-dependencies\]\s*$')[0]
foreach($forbidden in @('optical-phase-space','visible-radiance','physical-path','forge-kernel','tauri','reqwest','tokio')) {
  if($productionManifest -like "*$forbidden*"){throw "Forbidden source-calibration production dependency: $forbidden"}
}
foreach($required in @('capability-free evidence owner','byte-compatible legacy','No float conversion','16 KiB input, 32 KiB','deletion-only')) {
  if($contract -notlike "*$required*"){throw "Source-calibration contract drift: $required"}
}
foreach($required in @('implemented and fully verified as an owner-approved zero-consumer','896','1,786','six provenance/version/spectral/time substitutions','Transport applicability remains blocked','deletion-only','417.9 seconds','2,283','810','51')) {
  if($result -notlike "*$required*"){throw "Source-calibration result drift: $required"}
}
$otherManifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object FullName -ne $manifestPath
foreach($other in $otherManifests) {
  $production = ((Get-Content -LiteralPath $other.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-spectral-time-basis\s*=' -and
     $other.Directory.Name -ne 'calibrated-source-energy-distribution') {
    throw "Unapproved calibration consumer appeared: $($other.FullName)"
  }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
   $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$distributionResultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and
   $checkpoint.authority_lane -like '*verified zero-consumer source-calibration owner remains frozen*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(-not ($implementationCheckpoint -or $distributionResultCheckpoint -or $distributionReadinessCheckpoint -or $distributionImplementationCheckpoint -or $transportGapCheckpoint -or $transportDesignCheckpoint -or $c3InterruptionRoute)) {
  throw 'Source-calibration implementation checkpoint drift'
}
Push-Location $root
try {
  & cargo test -p calibrated-spectral-time-basis --all-targets
  if($LASTEXITCODE -ne 0){throw 'Native source-calibration tests failed'}
  & cargo test -p calibrated-spectral-time-basis --target i686-pc-windows-msvc
  if($LASTEXITCODE -ne 0){throw 'Executable i686 source-calibration tests failed'}
  & cargo check -p calibrated-spectral-time-basis --target aarch64-linux-android
  if($LASTEXITCODE -ne 0){throw 'Android ARM64 source-calibration check failed'}
} finally { Pop-Location }
Write-Output 'Source-calibration implementation verified: exact schema, stateless identities, strict codecs, the sole approved additive consumer and native/i686/Android gates pass.'
