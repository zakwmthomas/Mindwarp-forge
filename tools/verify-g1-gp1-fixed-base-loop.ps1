$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$required = @(
    'contracts\mindwarp-gameplay-foundation-contract.md',
    'docs\canonical-system\G1_GP1_FIXED_BASE_LOOP_READINESS.md',
    'docs\canonical-system\G1_GP1_FIXED_BASE_LOOP_RESULT.md',
    'crates\mindwarp-gameplay-foundation\src\base_loop.rs',
    'crates\mindwarp-gameplay-foundation\tests\gp1_base_loop.rs',
    'crates\mindwarp-gameplay-foundation\MODULE.md'
)
foreach ($relative in $required) {
    if (!(Test-Path -LiteralPath (Join-Path $root $relative) -PathType Leaf)) {
        throw "GP1 fixed base loop is missing: $relative"
    }
}

$source = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\src\base_loop.rs') -Raw
$tests = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\tests\gp1_base_loop.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\mindwarp-gameplay-foundation-contract.md') -Raw
foreach ($token in @(
    'pub const MAX_RECOVERIES: u8 = 3', 'LoopPhaseV1', 'PreparationV1',
    'AuthoredFixture', 'ValidatedC3A', 'BaseLoopLedgerV1', 'CompletedRunReceiptV1',
    'run already completed', 'recovery limit exhausted',
    'afterlight latest predecessor inadmissible', 'preparation session mismatch',
    'world context does not match expected authority'
)) {
    if (!$source.Contains($token)) { throw "GP1 implementation is missing: $token" }
}
foreach ($token in @(
    'all_five_sessions_share_one_six_phase_loop_and_outcome_is_not_prepared',
    'recoverable_failure_preserves_choice_and_rejects_fourth_failure_without_mutation',
    'remembered_response_is_atomic_idempotent_and_distinct_runs_can_repeat',
    's1_remembered_response_materially_changes_later_s5_state',
    'phase_skips_unsafe_stops_and_fabricated_or_noncanonical_state_fail_closed',
    'afterlight_rejects_missing_or_retreat_and_uses_latest_legitimate_repeat'
)) {
    if (!$tests.Contains($token)) { throw "GP1 hostile proof is missing: $token" }
}
foreach ($token in @(
    'Preparation selects', 'Exactly three recoveries are allowed',
    'typed completed run ID', 'GP2 progression is not begun'
)) {
    if (!$contract.Contains($token)) { throw "GP1 contract is missing: $token" }
}
foreach ($forbidden in @(
    'std::fs', 'std::process', 'std::net', 'tauri::', 'forge_kernel::',
    'physical_path_substrate::', 'visible_radiance_', 'optical_phase_space_',
    'greenfield', 'currency', 'procedural'
)) {
    if ($source.Contains($forbidden)) { throw "GP1 implementation crosses its capability boundary: $forbidden" }
}

Push-Location $root
try {
    & cargo test -p mindwarp-gameplay-foundation --test gp1_base_loop
    if ($LASTEXITCODE -ne 0) { throw 'GP1 focused Rust tests failed.' }
} finally {
    Pop-Location
}

Write-Output 'G1 GP1 fixed base loop verified: six phases, repairable failure, strict stable stops, typed idempotent history, exact optional C3A context and S1-to-S5 consequence pass.'
