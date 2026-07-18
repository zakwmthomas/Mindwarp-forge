$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'crates\visible-radiance-interface-event\src\interval.rs'
$lockPath = Join-Path $root 'crates\visible-radiance-interface-event\fixtures\point_v1_identity_lock.json'
$testPath = Join-Path $root 'crates\visible-radiance-interface-event\tests\interval_interface.rs'
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_INCIDENT_FIXED160_IMPLEMENTATION_READINESS.md'

foreach ($path in @($sourcePath, $lockPath, $testPath, $readinessPath)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "Interval implementation artifact missing: $path" }
}

$source = Get-Content -LiteralPath $sourcePath -Raw
foreach ($required in @(
    'INTERVAL_FRACTIONAL_BITS: u16 = 160',
    'MAX_INTERVAL_INPUT_BYTES: usize = 16 * 1024',
    'MAX_INTERVAL_EVENT_BYTES: usize = 64 * 1024',
    'INTERVAL_DERIVED_MAXIMUM_LIVE_BITS: u16 = 452',
    'forge-visible-radiance-interval-interface-input-v1',
    'forge-visible-radiance-interval-interface-event-v1',
    'DeclaredConditionalDirectionBox',
    'AmbiguousInterfaceBranch'
)) {
    if (!$source.Contains($required)) { throw "Interval implementation invariant missing: $required" }
}
foreach ($forbidden in @('REFERENCE_PRECISION','384','adaptive_outcome','f32','f64','std::net','std::fs','std::process')) {
    if ($source.Contains($forbidden)) { throw "Interval production source contains forbidden capability or oracle dependency: $forbidden" }
}

$lock = Get-Content -LiteralPath $lockPath -Raw | ConvertFrom-Json
if (@($lock).Count -ne 5) { throw 'Point-v1 identity lock must contain five pre-interval vectors.' }
foreach ($name in @('normal_incidence','index_match','reverse_direction','critical_tir','unsupported_model')) {
    $row = @($lock | Where-Object name -eq $name)
    if ($row.Count -ne 1 -or $row[0].input_bytes_sha256 -notmatch '^[0-9a-f]{64}$' -or $row[0].event_bytes_sha256 -notmatch '^[0-9a-f]{64}$') {
        throw "Point-v1 identity lock is missing or malformed: $name"
    }
}

$tests = Get-Content -LiteralPath $testPath -Raw
foreach ($required in @('invalid_boxes_fail_before_arithmetic','rgb_branches_are_independent','reverse_orientation_is_world_signed','deterministic_hostile_box_portfolio','MAX_INTERVAL_INPUT_BYTES + 1','MAX_INTERVAL_EVENT_BYTES + 1')) {
    if (!$tests.Contains($required)) { throw "Interval implementation fixture missing: $required" }
}

Write-Output 'Fixed-160 interval interface implementation shield verified: production-only precision, byte caps, identities, hostile fixtures, and point-v1 lock are present.'
