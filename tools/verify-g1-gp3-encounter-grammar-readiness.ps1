$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$readinessPath = Join-Path $root 'docs\canonical-system\G1_GP3_ENCOUNTER_GRAMMAR_READINESS.md'
$designPath = Join-Path $root 'docs\canonical-system\G1_GP3_ENCOUNTER_GRAMMAR_DESIGN.md'
$registryPath = Join-Path $root 'docs\canonical-system\G1_GP3_ENCOUNTER_GRAMMAR_FIXED_REGISTRY.md'
foreach ($path in @($readinessPath,$designPath,$registryPath)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "GP3 design record is missing: $path"
    }
}

$joined = (Get-Content -LiteralPath $readinessPath -Raw) + "`n" +
    (Get-Content -LiteralPath $designPath -Raw) + "`n" +
    (Get-Content -LiteralPath $registryPath -Raw)
foreach ($token in @(
    'exactly these five situations', 'environment', 'creature', 'society',
    'Only S4 has an anomaly facet', 'construction', 'domain_facets',
    'evidence_refs', 'risk_refs',
    'all 18 GP0 outcomes', 'at least two noncombat, threat-free',
    'exactly one retreat', 's3.force', 'force_partial',
    'S2 `predator`', 'S4 `wire-scavengers`', 'S5 `food-scavengers`',
    'optional and nonterminal',
    '131,072 bytes', '32,768 bytes', 'deny_unknown_fields',
    'u32_be(domain UTF-8 byte length)', 'mindwarp.gp3.fixed-situation.v1',
    'mindwarp.gp3.fixed-grammar.v1', 'AuthoredGameplayNonC3B',
    'observed_fact', 'available_inference', 'prepared_tool',
    'authored_state', 'exact_predecessor', 'subject_ids',
    'resolved', 'mitigated', 'accepted', 'transferred', 'unchanged',
    'threat_contribution', 'world-contribution-only',
    'no GP2 record or mapping', 'no procedural generator',
    'GP4 and runtime remain forbidden'
)) {
    if (!$joined.Contains($token)) { throw "GP3 readiness is missing: $token" }
}

foreach ($id in @(
    'gp3.s1.colony-conduit', 'gp3.s2.storm-nest',
    'gp3.s3.memory-gate', 'gp3.s4.signal-anchor', 'gp3.s5.afterlight',
    's1.flow-loss', 's2.exposure', 's3.ledger', 's4.timing', 's5.history',
    'conduit-failure', 'storm-arrival', 'channel-harm',
    'anchor-collapse', 'buffer-violation',
    'predator', 'wire-scavengers', 'food-scavengers'
)) {
    if (!$joined.Contains($id)) { throw "GP3 fixed registry is missing: $id" }
}

foreach ($forbidden in @(
    'procedural generation is authorized', 'combat resolves core tension',
    'automatic progression is authorized', 'runtime authority is granted',
    'C3B authority is granted', 'GP4 implementation is authorized'
)) {
    if ($joined.Contains($forbidden)) {
        throw "GP3 readiness crosses its authority boundary: $forbidden"
    }
}

$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
if ($checkpoint.batch_id -ne 'G1-GP3-ENCOUNTER-GRAMMAR-V1' -or
    $checkpoint.master_program_item -ne 'GP3' -or
    $checkpoint.substage_id -notin @(
        'gp3-encounter-grammar-readiness',
        'gp3-encounter-grammar-implementation',
        'gp3-encounter-grammar-verification',
        'gp3-encounter-grammar-recorded'
    )) {
    throw 'GP3 readiness is not bound to the canonical checkpoint route.'
}

$route = & (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
if ($route -ne $true) { throw 'GP3 is not admitted by the federated interruption route.' }

Write-Output 'G1 GP3 encounter grammar readiness verified: five fixed domain-tagged situations, exact GP0 references, noncombat/retreat/threat boundaries and hostile codec limits are frozen before source.'
