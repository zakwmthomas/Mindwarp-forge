$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$required = @(
    'contracts\mindwarp-gameplay-foundation-contract.md',
    'docs\canonical-system\G1_GP0_GAMEPLAY_FOUNDATION_RESULT.md',
    'crates\mindwarp-gameplay-foundation\Cargo.toml',
    'crates\mindwarp-gameplay-foundation\src\lib.rs',
    'crates\mindwarp-gameplay-foundation\src\fixtures.rs',
    'crates\mindwarp-gameplay-foundation\tests\gp0_contract.rs',
    'crates\mindwarp-gameplay-foundation\MODULE.md'
)
foreach ($relative in $required) {
    if (!(Test-Path -LiteralPath (Join-Path $root $relative) -PathType Leaf)) {
        throw "GP0 gameplay foundation is missing: $relative"
    }
}

$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\mindwarp-gameplay-foundation-contract.md') -Raw
foreach ($token in @(
    'causal_explorer_maker',
    'repairable-failure model are explicit reversible proposed',
    'Combat may create room to act but never resolves',
    'AuthoredGameplayNonC3B',
    'admits exactly S1 direct, bypass, and ration, rejects S1 retreat'
)) {
    if (!$contract.Contains($token)) { throw "GP0 contract is missing: $token" }
}

$source = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\src\lib.rs') -Raw
$fixtures = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\src\fixtures.rs') -Raw
$tests = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\tests\gp0_contract.rs') -Raw
foreach ($token in @(
    'GameplayConceptRecordV1', 'NonGoal::CombatResolvesCoreTension',
    'C3AWorldReferenceV1', 'bind_validated_c3a_world', 'validate_world_packet',
    'state does not match deterministic replay', 'history predecessor mismatch',
    'session has no threat contribution'
)) {
    if (!$source.Contains($token)) { throw "GP0 implementation is missing: $token" }
}
foreach ($token in @(
    'gp0.s1.colony-conduit', 'gp0.s2.storm-nest', 'gp0.s3.memory-gate',
    'gp0.s4.signal-anchor', 'gp0.s5.afterlight', 'wire-scavengers',
    'keeper-mara', 's1.direct', 's1.bypass', 's1.ration', 's1.retreat'
)) {
    if (!$fixtures.Contains($token)) { throw "GP0 fixed fixtures are missing: $token" }
}
foreach ($token in @(
    'afterlight_replays_distinct_direct_bypass_and_ration_meanings',
    'threat_actions_are_session_specific_and_s4_only_clears_the_work_area',
    'history_codec_rejects_nested_and_predecessor_corruption',
    'c3_a_binding_accepts_exact_pair_and_rejects_foreign_packet'
)) {
    if (!$tests.Contains($token)) { throw "GP0 negative matrix is missing: $token" }
}
foreach ($forbidden in @(
    'std::fs', 'std::process', 'std::net', 'tauri::', 'forge_kernel::',
    'physical_path_substrate::', 'visible_radiance_', 'optical_phase_space_',
    'packet_path', 'packetPath'
)) {
    if ($source.Contains($forbidden) -or $fixtures.Contains($forbidden)) {
        throw "GP0 implementation crosses its capability boundary: $forbidden"
    }
}

$modules = Get-Content -LiteralPath (Join-Path $root 'governance\module-context-registry.json') -Raw | ConvertFrom-Json
$module = @($modules.modules | Where-Object id -eq 'mindwarp-gameplay-foundation')
if ($module.Count -ne 1 -or $module[0].maturity -ne 'prototype_tested') {
    throw 'GP0 module context is missing or has the wrong maturity.'
}
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw | ConvertFrom-Json
$boundary = @($boundaries.modules | Where-Object id -eq 'mindwarp-gameplay-foundation')
if ($boundary.Count -ne 1 -or @($boundary[0].dependencies) -notcontains 'derived-world-rules') {
    throw 'GP0 module boundary lacks the exact C3A owner dependency.'
}

Push-Location $root
try {
    & cargo test -p mindwarp-gameplay-foundation
    if ($LASTEXITCODE -ne 0) { throw 'GP0 focused Rust tests failed.' }
} finally {
    Pop-Location
}

Write-Output 'G1 GP0 gameplay foundation verified: five corrected authored sessions, strict deterministic replay/history, typed C3A seam and authority-negative boundaries pass.'
