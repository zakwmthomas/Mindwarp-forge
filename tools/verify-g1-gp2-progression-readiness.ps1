$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_GP2_PROGRESSION_READINESS.md'
$designPath = Join-Path $root 'docs\canonical-system\G1_GP2_PROGRESSION_DESIGN.md'
foreach ($path in @($readinessPath,$designPath)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "GP2 design record is missing: $path" }
}
$joined = (Get-Content -LiteralPath $readinessPath -Raw) + "`n" + (Get-Content -LiteralPath $designPath -Raw)
foreach ($token in @(
    'ProgressionLedgerV1', 'BaseLoopStateV1', 'canonical digest',
    'KnowledgeRecordV1', 'AccessRecordV1', 'RelationshipEventV1',
    'ConstructionRecordV1', 'CapabilityRecordV1', 'NamedAssetV1',
    'full-flow-kit', 'colony-safe-kit', 'timed-controller',
    'steward-builder', 'urgency-discovery', 'cautious-mapper',
    'pairwise-incomparable', 'same-event or same-subject cycle',
    'Recovery emits', 'no durable progress', 'reversible proposed assumptions'
)) {
    if (!$joined.Contains($token)) { throw "GP2 readiness is missing: $token" }
}
foreach ($forbidden in @(
    'universal currency is admitted', 'positive conversion cycle is admitted',
    'runtime implementation is authorized', 'Greenfield dependency is authorized',
    'C3B authority is granted'
)) {
    if ($joined.Contains($forbidden)) { throw "GP2 readiness crosses its authority boundary: $forbidden" }
}
if (Test-Path -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\src\progression.rs')) {
    throw 'GP2 source implementation began before the readiness report.'
}

Write-Output 'G1 GP2 progression readiness verified: five typed lanes, exact GP1 digest authority, explicit flow/exploit rules, S1 capability allowlist and deterministic incomparable strategies are frozen before source.'
