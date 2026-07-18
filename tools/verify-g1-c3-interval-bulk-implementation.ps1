Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'crates\visible-radiance-bulk-transfer\src\interval.rs'
$libraryPath = Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs'
$manifestPath = Join-Path $root 'crates\visible-radiance-bulk-transfer\Cargo.toml'
$fixturePath = Join-Path $root 'crates\visible-radiance-bulk-transfer\fixtures\bulk_v1_identity_lock.json'
$lockTestPath = Join-Path $root 'crates\visible-radiance-bulk-transfer\tests\bulk_v1_identity_lock.rs'
$intervalTestPath = Join-Path $root 'crates\visible-radiance-bulk-transfer\tests\interval_bulk.rs'
$contractPath = Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_BULK_TRANSFER_IMPLEMENTATION_RESULT.md'
foreach ($path in @($sourcePath,$libraryPath,$manifestPath,$fixturePath,$lockTestPath,$intervalTestPath,$contractPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing interval bulk implementation artifact: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$library = Get-Content -LiteralPath $libraryPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$fixture = Get-Content -LiteralPath $fixturePath -Raw | ConvertFrom-Json
$lockTests = Get-Content -LiteralPath $lockTestPath -Raw
$intervalTests = Get-Content -LiteralPath $intervalTestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw

foreach ($required in @(
  'MAX_INTERVAL_BULK_QUERY_BYTES: usize = 64 * 1024',
  'MAX_INTERVAL_BULK_TRANSFER_BYTES: usize = 16 * 1024',
  'INTERVAL_BULK_FRACTIONAL_BITS: u16 = 160',
  'INTERVAL_BULK_STORAGE_BITS: u16 = 512',
  'INTERVAL_BULK_DERIVED_MAXIMUM_MAGNITUDE_BITS: u16 = 414',
  'INTERVAL_BULK_FINAL_LENGTH_MAXIMUM_MAGNITUDE_BITS: u16 = 192',
  'mindwarp.visible-radiance.interval-bulk-query.v1',
  'mindwarp.visible-radiance.interval-bulk-transfer.v1',
  'pub struct ConditionalIntervalBulkQueryV1',
  'pub struct ConditionalIntervalBulkTransferV1',
  'pub struct IntervalBulkLengthCertificateV1',
  'pub fn compile_conditional_interval_bulk_transfer',
  'pub fn validate_conditional_interval_bulk_transfer',
  'length.checked_mul(&coefficient)',
  'optical_q160.project(64)',
  'exp_neg_q0_64_bounds'
)) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Interval bulk implementation bound drift: $required" }
}
if ($library -notmatch 'mod interval;' -or $library -notmatch 'pub use interval::\*;') {
  throw 'The additive interval bulk implementation is not privately housed and re-exported.'
}
if ($manifest -notmatch 'fixed-interval-arithmetic = \{ path = "\.\./fixed-interval-arithmetic" \}') {
  throw 'The direct shared fixed-interval arithmetic dependency drifted.'
}
if (@($fixture.families).Count -ne 8) { throw 'The bulk V1 compatibility fixture must contain eight families.' }
foreach ($family in @('vacuum_identity','finite_zero_identity','finite_positive_attenuation','opaque_termination','unavailable_evidence','ambiguous_boundary_lane','interface_model_required','stationary_point_behavior')) {
  if (@($fixture.families | Where-Object name -eq $family).Count -ne 1) { throw "Missing bulk V1 identity family: $family" }
}
foreach ($requiredTest in @('one_band_exact_length_three_interactions','current_cell_transfer_precedes_outer','unavailable_current_ambiguity_no_progress','hostile_codecs_caps_and_forged_nested_evidence','bulk_v1_bytes_and_identities_remain_locked')) {
  if (($intervalTests + $lockTests) -notmatch [regex]::Escape($requiredTest)) { throw "Missing interval bulk test shield: $requiredTest" }
}
foreach ($requiredContract in @('Additive conditional interval bulk-transfer surface','exactly one','414-magnitude-bit','64 KiB and transfer','local one-cell conditional evidence only')) {
  if ($contract -notlike "*$requiredContract*") { throw "Interval bulk contract drift: $requiredContract" }
}
foreach ($requiredResult in @('implemented and verified','67783f4eae5f737979580fbddd6725d4faaa556fb031b90730cf7359ba27fce2','94b2fe43260c9a604ec6c22035f28f7026319531c22951a4e8747f8d242713c3','i686 Windows','Android ARM64','passed in 232.8 seconds','Actual mobile-device performance remains unmeasured','optical arithmetic migration')) {
  if ($result -notlike "*$requiredResult*") { throw "Interval bulk implementation result drift: $requiredResult" }
}
foreach ($forbidden in @('f32','f64','struct Signed512','visible_radiance_interface_event','std::fs','std::net','std::process','Command::new')) {
  if ($source -match [regex]::Escape($forbidden)) { throw "Forbidden interval bulk implementation mechanism present: $forbidden" }
}
Write-Output 'Interval bulk implementation verified: additive one-band schema, dual Q160 certificate, shared 512-bit arithmetic, 414/192-bit shields, bounded codecs, existing exponential reuse and eight-family V1 freeze are retained.'
