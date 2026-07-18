Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$bulkSource = Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs'
$bulkTests = Join-Path $root 'crates\visible-radiance-bulk-transfer\tests\optical_depth_evaluation.rs'
$source = Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\src\lib.rs'
$tests = Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\tests\dimensionless_transfer.rs'
$manifest = Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\Cargo.toml'
$bulkContract = Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md'
$contract = Join-Path $root 'contracts\optical-phase-space-dimensionless-transfer-contract.md'
$result = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_IMPLEMENTATION_RESULT.md'
foreach ($path in @($bulkSource,$bulkTests,$source,$tests,$manifest,$bulkContract,$contract,$result)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing whole-cell transfer implementation artifact: $path" }
}
if ((Get-FileHash -LiteralPath $bulkSource -Algorithm SHA256).Hash.ToLowerInvariant() -ne 'd318b0919f8f59e64b782c30fb7a6898a8bb2aea092b6015b49372abdbcd4971') { throw 'Bulk evaluation source drifted.' }
if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash.ToLowerInvariant() -ne '0f9b5fb66cafd9c977dde80e37cf228a7cc041c4864e29b62dac20c3d2992d18') { throw 'Whole-cell transfer source drifted.' }
$bulkText = Get-Content -LiteralPath $bulkSource -Raw
$sourceText = Get-Content -LiteralPath $source -Raw
$testText = (Get-Content -LiteralPath $bulkTests -Raw) + (Get-Content -LiteralPath $tests -Raw)
$manifestText = Get-Content -LiteralPath $manifest -Raw
$contractText = (Get-Content -LiteralPath $bulkContract -Raw) + (Get-Content -LiteralPath $contract -Raw)
$resultText = Get-Content -LiteralPath $result -Raw
foreach ($required in @('BulkOpticalDepthEvaluationInputV1','BulkOpticalDepthEvaluationV1','BULK_OPTICAL_DEPTH_MAXIMUM_RAW_BITS: u16 = 118','MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES: usize = 4 * 1024','mindwarp.visible-radiance.bulk-optical-depth-evaluation.input.v1','mindwarp.visible-radiance.bulk-optical-depth-evaluation.result.v1','exp_neg_q0_64_bounds(upper)','exp_neg_q0_64_bounds(lower)')) { if ($bulkText -notlike "*$required*") { throw "Bulk evaluation implementation drift: $required" } }
foreach ($required in @('OpticalBandTimeBindingV1','WholeCellDimensionlessTransferInputV1','WholeCellDimensionlessTransferOutcomeV1','CertifiedAcceptedFiniteTransfer','CertifiedAcceptedOpaqueTransfer','CertifiedAcceptedUnresolvedTransfer','CertifiedZeroCoupling','UnresolvedCoupling','MAXIMUM_STEPS: usize = 64','MAXIMUM_ENDPOINT_ADDITIONS: u16 = 128','MAXIMUM_RAW_OPTICAL_DEPTH_BITS: u16 = 118','MAX_INPUT_BYTES: usize = 128 * 1024 * 1024','MAX_RESULT_BYTES: usize = 256 * 1024','MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 192 * 1024 * 1024','mindwarp.optical-phase-space.band-time.v1','mindwarp.optical-phase-space.dimensionless-transfer.input.v1','mindwarp.optical-phase-space.dimensionless-transfer.result.v1','none_evidence_only')) { if ($sourceText -notlike "*$required*") { throw "Whole-cell transfer implementation drift: $required" } }
foreach ($required in @('projected_zero_lower_is_finite_underflow_not_opacity','band_time_binding_is_deterministic_and_fail_closed','finite_prefix_selected_partial_and_codecs_replay','selected_and_prefix_opacity_remain_distinct','zero_unresolved_and_forgery_preserve_authority_boundary','start_inside_identity_and_cross_owner_mutations_fail_closed')) { if ($testText -notlike "*$required*") { throw "Whole-cell transfer test shield missing: $required" } }
foreach ($required in @('optical-phase-space-receiver-coupling','optical-phase-space-transport-certificate','visible-radiance-bulk-transfer')) { if ($manifestText -notlike "*$required*") { throw "Whole-cell transfer dependency drift: $required" } }
foreach ($forbidden in @('forge-kernel','tauri','tokio','reqwest','crypto-bigint','num-bigint')) { if ($manifestText -like "*$forbidden*") { throw "Whole-cell transfer forbidden dependency: $forbidden" } }
foreach ($required in @('Additive optical-depth evaluation receipt','118-bit raw ceiling','unchanged bulk-owned','mandatory opaque prefix','zero and unresolved coupling')) { if ($contractText -notlike "*$required*") { throw "Whole-cell transfer contract drift: $required" } }
foreach ($required in @('owner-authorized bounded implementation','unchanged private exponential kernel','finite underflow, never opacity','i686-pc-windows-msvc','aarch64-linux-android','mobile-device performance remains unmeasured','deletion-only')) { if ($resultText -notlike "*$required*") { throw "Whole-cell transfer result drift: $required" } }
foreach ($forbidden in @('f32','f64','std::fs','std::net','std::process','Command::new','unwrap(','expect(')) { if ($sourceText -match [regex]::Escape($forbidden)) { throw "Forbidden whole-cell transfer mechanism present: $forbidden" } }
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-dimensionless-transfer.ps1')
if (-not $?) { throw 'Whole-cell transfer mathematical verifier failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-dimensionless-transfer-readiness.ps1')
if (-not $?) { throw 'Whole-cell transfer readiness verifier failed.' }
& cargo test -p visible-radiance-bulk-transfer --all-targets
if ($LASTEXITCODE -ne 0) { throw 'Bulk evaluation native and legacy tests failed.' }
& cargo test -p optical-phase-space-dimensionless-transfer --all-targets
if ($LASTEXITCODE -ne 0) { throw 'Whole-cell transfer native tests failed.' }
& cargo test -p visible-radiance-bulk-transfer --target i686-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'Bulk evaluation executable i686 tests failed.' }
& cargo test -p optical-phase-space-dimensionless-transfer --target i686-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'Whole-cell transfer executable i686 tests failed.' }
& cargo check -p visible-radiance-bulk-transfer --target aarch64-linux-android
if ($LASTEXITCODE -ne 0) { throw 'Bulk evaluation Android compilation failed.' }
& cargo check -p optical-phase-space-dimensionless-transfer --target aarch64-linux-android
if ($LASTEXITCODE -ne 0) { throw 'Whole-cell transfer Android compilation failed.' }
& (Join-Path $PSScriptRoot 'verify-module-context.ps1')
if (-not $?) { throw 'Whole-cell transfer module context failed.' }
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or -not (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])) { throw 'Whole-cell transfer implementation route drifted.' }
Write-Output 'Whole-cell dimensionless-transfer implementation verified: unchanged bulk kernel reuse, owner replay, conservative composition, exact measure, strict identities/codecs and native/i686/Android gates pass without magnitude, detector, visibility, runtime or closure authority.'
