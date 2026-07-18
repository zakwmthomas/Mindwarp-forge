Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\calibrated-source-energy-distribution'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\distribution.rs'
$surfacePath = Join-Path $crate 'tests\public_surface.rs'
$fixturePath = Join-Path $crate 'fixtures\distribution_v1_identity_lock.json'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\calibrated-source-energy-distribution-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_IMPLEMENTATION_RESULT.md'
foreach($path in @($sourcePath,$testPath,$surfacePath,$fixturePath,$manifestPath,$contractPath,$resultPath)) {
  if(!(Test-Path -LiteralPath $path)){throw "Calibrated source-energy implementation artifact missing: $path"}
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$fixture = Get-Content -LiteralPath $fixturePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach($required in @('ExactRadiantEnergyV1','SourceEnergyRefinementDirectiveV1','CalibratedSourceEnergyDistributionQueryV1','CalibratedSourceEnergyAllocationV1','CalibratedSourceEnergySplitReceiptV1','CalibratedSourceEnergyDistributionV1','replay_calibrated_source_energy_distribution','split_optical_phase_space_cell','MAX_FRONTIER_ALLOCATIONS: usize = 64','MAX_REFINEMENT_DIRECTIVES: usize = 63','MAX_QUERY_BYTES: usize = 128 * 1024','MAX_RESULT_BYTES: usize = 256 * 1024','MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 4 * 1024 * 1024','MAX_ENERGY_LIVE_BITS: u16 = 385','deny_unknown_fields','none_evidence_only')) {
  if($source -notlike "*$required*"){throw "Calibrated source-energy implementation drift: $required"}
}
foreach($required in @('root_only_is_exact_strict_and_replayable','upstream_axis_split_is_replayed_and_energy_is_conserved','mixed_depth_frontier_has_canonical_path_order','resolved_leaf_cannot_be_refined_again','non_frontier_and_nonconserving_directives_fail_typed','provenance_and_canonical_decimal_shields_fail_closed','result_replay_rejects_identity_tamper_and_codecs_reject_trailing_bytes','directive_count_is_bounded_before_replay','identity_and_codec_fixture_lock','full_sixty_four_leaf_envelope_is_admitted')) {
  if($tests -notlike "*$required*"){throw "Calibrated source-energy test shield missing: $required"}
}
foreach($required in @('547b4f1003e54a37247b1a820ed5f508673026066a9c4f80638129047cc82cc5','0cd4aa695ebfb9eb83525189cfc42f8fb2484c20b848bb255c06c08a7c4a3fda','5204c98619f8c76caa801fa7c252a6444b8552c0664d5727dd05ee4719cc396b','220fc3bb726a3d52ba4fc7eba7912076835ebf3396f236c70e028385967d3fcf')) {
  if($fixture -notlike "*$required*"){throw "Calibrated source-energy identity fixture drift: $required"}
}
foreach($required in @('calibrated-spectral-time-basis','fixed-interval-arithmetic','optical-phase-space-cell-binding','serde = { version = "1", features = ["derive"] }','serde_json = "1"','sha2 = "0.10"')) {
  if($manifest -notmatch [regex]::Escape($required)){throw "Calibrated source-energy dependency drift: $required"}
}
foreach($forbidden in @('forge-kernel','tauri','reqwest','tokio','visible-radiance','optical-lineage','receiver-arrival')) {
  if($manifest -like "*$forbidden*"){throw "Forbidden calibrated source-energy dependency: $forbidden"}
}
foreach($required in @('capability-free evidence owner','385-bit','non-circular','zero downstream','deletion-only')) {
  if($contract -notlike "*$required*"){throw "Calibrated source-energy contract drift: $required"}
}
foreach($required in @('owner-approved bounded V1','64 frontier allocations','zero downstream consumers','deletion-only','Transport and perception remain blocked')) {
  if($result -notlike "*$required*"){throw "Calibrated source-energy result drift: $required"}
}
$otherManifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object FullName -ne $manifestPath
foreach($other in $otherManifests) {
  $production = ((Get-Content -LiteralPath $other.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-source-energy-distribution\s*='){throw "Downstream consumer appeared: $($other.FullName)"}
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$applicabilityGapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and
   $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and
   $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$applicabilityDesignCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
   $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
   $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(!$implementationCheckpoint -and !$applicabilityGapCheckpoint -and !$applicabilityDesignCheckpoint -and !$c3InterruptionRoute) {
  throw 'Calibrated source-energy implementation checkpoint drift'
}
Push-Location $root
try {
  & cargo test -p calibrated-source-energy-distribution --all-targets
  if($LASTEXITCODE -ne 0){throw 'Native calibrated source-energy tests failed'}
  & cargo test -p calibrated-source-energy-distribution --target i686-pc-windows-msvc
  if($LASTEXITCODE -ne 0){throw 'Executable i686 calibrated source-energy tests failed'}
  & cargo check -p calibrated-source-energy-distribution --target aarch64-linux-android
  if($LASTEXITCODE -ne 0){throw 'Android ARM64 calibrated source-energy check failed'}
} finally { Pop-Location }
Write-Output 'Calibrated source-energy distribution implementation verified: upstream axis replay, exact conservation, strict identities/codecs, bounded resources, zero downstream consumers and native/i686/Android gates pass.'
