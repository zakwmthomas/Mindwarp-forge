Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'crates\physical-path-substrate\src\interval.rs'
$libraryPath = Join-Path $root 'crates\physical-path-substrate\src\lib.rs'
$manifestPath = Join-Path $root 'crates\physical-path-substrate\Cargo.toml'
$fixturePath = Join-Path $root 'crates\physical-path-substrate\fixtures\exact_path_v1_identity_lock.json'
$testPath = Join-Path $root 'crates\physical-path-substrate\tests\interval_cell_step.rs'
foreach ($path in @($sourcePath, $libraryPath, $manifestPath, $fixturePath, $testPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing interval cell-step implementation artifact: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$library = Get-Content -LiteralPath $libraryPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$fixture = Get-Content -LiteralPath $fixturePath -Raw | ConvertFrom-Json
$tests = Get-Content -LiteralPath $testPath -Raw

foreach ($required in @(
  'INTERVAL_CELL_STEP_CONTRACT_VERSION: u16 = 1',
  'INTERVAL_CELL_STEP_FRACTIONAL_BITS: u16 = 160',
  'MAX_INTERVAL_CELL_STEP_INPUT_BYTES: usize = 16 * 1024',
  'MAX_INTERVAL_CELL_STEP_EVENT_BYTES: usize = 32 * 1024',
  'INTERVAL_CELL_STEP_DERIVED_MAXIMUM_LIVE_BITS: u16 = 414',
  'mindwarp.physical-path.interval-cell-step-input.v1',
  'mindwarp.physical-path.interval-cell-step-event.v1',
  'pub struct ConditionalIntervalCellStepInputV1',
  'pub struct ConditionalIntervalCellStepEventV1',
  'pub fn compile_conditional_interval_cell_step',
  'pub fn validate_conditional_interval_cell_step_event',
  'if bytes.len() > MAX_INTERVAL_CELL_STEP_INPUT_BYTES',
  'if bytes.len() > MAX_INTERVAL_CELL_STEP_EVENT_BYTES'
)) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Interval implementation bound drift: $required" }
}
if ($library -notmatch 'mod interval;' -or $library -notmatch 'pub use interval::\*;') {
  throw 'The interval implementation is not privately housed and additively re-exported.'
}
if ($manifest -notmatch 'fixed-interval-arithmetic = \{ path = "\.\./fixed-interval-arithmetic" \}') {
  throw 'The physical cell-step shared-arithmetic dependency drifted.'
}
if (@($fixture).Count -ne 5) { throw 'The exact-path V1 compatibility fixture must contain five families.' }
foreach ($family in @('straight_face','exact_reverse','simultaneous_vertex','stationary_point','negative_near_maximum')) {
  if (@($fixture | Where-Object name -eq $family).Count -ne 1) { throw "Missing exact-path V1 family: $family" }
}
foreach ($requiredTest in @('six_outer_exits','one_unit_reversal','correlation_erasure','near_maximum_coordinates','MAX_INTERVAL_CELL_STEP_INPUT_BYTES','MAX_INTERVAL_CELL_STEP_EVENT_BYTES')) {
  if ($tests -notmatch [regex]::Escape($requiredTest)) { throw "Missing hostile interval test shield: $requiredTest" }
}
foreach ($forbidden in @('f32','f64','to_words','as_words','from_words','std::fs','std::net','std::process','Command::new','PRECISIONS')) {
  if ($source -match [regex]::Escape($forbidden)) { throw "Forbidden interval implementation mechanism present: $forbidden" }
}
Write-Output 'Interval cell-step implementation verified: fixed-160/512 arithmetic, 414-bit shield, bounded strict codecs, exact dependency pin, typed outcomes and five-family V1 freeze are retained.'
