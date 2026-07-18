Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_OPTICAL_CELL_STEP_IMPLEMENTATION_READINESS.md'
$sourcePath = Join-Path $root 'crates\physical-path-substrate\src\lib.rs'
$manifestPath = Join-Path $root 'crates\physical-path-substrate\Cargo.toml'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$checkpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$source = Get-Content -LiteralPath $sourcePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath $checkpointPath -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1) { throw 'C3 master item is missing or duplicated.' }

foreach ($required in @(
  'INTERVAL_CELL_STEP_DERIVED_MAXIMUM_LIVE_BITS = 414',
  'MAX_INTERVAL_CELL_STEP_INPUT_BYTES = 16 * 1024',
  'MAX_INTERVAL_CELL_STEP_EVENT_BYTES = 32 * 1024',
  'declared_conditional_point_direction_box',
  'mindwarp.physical-path.interval-cell-step-input.v1',
  'strictly below every competitor',
  '413 magnitude bits',
  'crypto-bigint = { version = "=0.7.5", default-features = false }',
  'adds no new resolved package, version or feature',
  'General continuation does not authorize this source action'
)) {
  if ($readiness -notlike "*$required*") { throw "Interval cell-step readiness is missing: $required" }
}
foreach ($requiredSource in @(
  'pub const CONTRACT_VERSION: u16 = 1;',
  'pub const MAX_PHYSICAL_VOLUME_PROOF_CELLS: u64 = 65_536;',
  'pub cell_step_q32_32: i64',
  'pub origin_q32_32: [i64; 3]',
  'pub fn validate_physical_volume(',
  'pub fn build_physical_cell('
)) {
  if ($source -notmatch [regex]::Escape($requiredSource)) { throw "Physical-path source bound drift: $requiredSource" }
}
if ($source -match 'mod interval|ConditionalIntervalCellStep' -or $manifest -match 'crypto-bigint') {
  throw 'Interval cell-step production source changed before explicit owner approval.'
}
if (($c3[0].next_action -notlike '*Await explicit owner approval*interval cell-step*' -or
    $checkpoint.batch_id -ne 'G1-C3-INTERVAL-OPTICAL-CELL-STEP-OWNER-GATE-V1' -or
    $checkpoint.next_action -notlike '*Await explicit owner approval*') -and !$c3InterruptionRoute) {
  throw 'Canonical route is not stopped at the exact interval cell-step owner gate.'
}
Write-Output 'Interval cell-step readiness verified: 414/512-bit bound, bounded codecs, exact dependency pin, v1 freeze and owner stop are retained.'
