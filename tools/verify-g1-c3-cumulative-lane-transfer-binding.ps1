Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\optical-lane-transfer-binding'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\cumulative.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\optical-lane-transfer-binding-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CUMULATIVE_LANE_TRANSFER_IMPLEMENTATION_RESULT.md'
foreach ($path in @($sourcePath,$testPath,$manifestPath,$contractPath,$resultPath,(Join-Path $crate 'MODULE.md'))) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing cumulative lane-transfer artifact: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'MAX_CUMULATIVE_INPUT_BYTES: usize = 18 * 1024 * 1024',
  'MAX_CUMULATIVE_OUTPUT_BYTES: usize = 256 * 1024',
  'MAX_VALIDATION_LIVE_CANONICAL_BYTES: usize = 32 * 1024 * 1024',
  'MAX_CUMULATIVE_FACTORS: usize = 128','MAXIMUM_LIVE_BITS: u16 = 209',
  'mindwarp.optical-lineage.cumulative-factor.v1',
  'mindwarp.optical-lineage.cumulative-result.v1',
  'mindwarp.optical-lineage.cumulative-transcript.v1',
  'pub fn compile_cumulative_optical_lane_transfer',
  'pub fn validate_cumulative_optical_lane_transfer',
  'ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer',
  'OpticalLineageDispositionV1::ContinueAfterInterface',
  '.div_floor(&scale)','.div_ceil(&scale)','none_evidence_only'
)) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Cumulative lane-transfer source drift: $required" }
}
foreach ($dependency in @('optical-lineage-binding','visible-radiance-bulk-transfer','visible-radiance-interface-event','fixed-interval-arithmetic','serde','serde_json','sha2')) {
  if ($manifest -notmatch [regex]::Escape($dependency)) { throw "Missing cumulative lane-transfer dependency: $dependency" }
}
foreach ($forbidden in @('forge-kernel','tauri','reqwest','ureq','hyper','tokio','std::fs','std::net','std::process','Command::new','f32','f64')) {
  if (($manifest + $source) -match [regex]::Escape($forbidden)) { throw "Forbidden cumulative lane-transfer mechanism: $forbidden" }
}
foreach ($requiredTest in @(
  'derives_vacuum_finite_opaque_and_unavailable_without_factor_injection',
  'strict_codecs_replay_and_reject_output_or_authority_forgery',
  'followed_interface_factor_is_selected_but_terminal_interfaces_inject_none',
  'directed_q160_fold_preserves_sub_q48_products_until_final_projection',
  'one_hundred_twenty_eight_factors_are_bounded_and_costed_exactly'
)) {
  if (($tests + $source) -notmatch [regex]::Escape($requiredTest)) { throw "Missing cumulative lane-transfer test shield: $requiredTest" }
}
foreach ($requiredContract in @('known_current_cell_transfer','no evaluated current-cell transfer','209 bits','128 factors','18 MiB','256 KiB','32 MiB','[0,1]','ten-family','none_evidence_only')) {
  if ($contract -notlike "*$requiredContract*") { throw "Cumulative lane-transfer contract drift: $requiredContract" }
}
foreach ($requiredResult in @('implemented and verified','optical-lane-transfer-binding','Q0.160','128-factor','209-bit shield','no fabricated factor','Five focused Rust tests','i686-pc-windows-msvc','aarch64-linux-android','234.0 seconds','Actual mobile-device performance remains unmeasured')) {
  if ($result -notlike "*$requiredResult*") { throw "Cumulative lane-transfer result drift: $requiredResult" }
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$oracleOutput = & $python (Join-Path $root 'tools\prove-g1-c3-cumulative-lane-transfer.py')
if ($LASTEXITCODE -ne 0 -or ($oracleOutput -join "`n") -notlike '*ee5f237fe1c8b7581372646e01ab12c7ddedfa1707d1b0e5dbf199e81b2ba09d*') {
  throw 'Pinned cumulative lane-transfer oracle receipt drifted.'
}
Write-Output 'Cumulative lane-transfer binding verified: owner-derived ordered factors, directed Q0.160 fold, single Q0.48 projection, hard caps, strict replay, authority exclusions and pinned oracle are retained.'
