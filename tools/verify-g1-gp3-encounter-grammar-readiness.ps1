param([string]$ProgramPath,[string]$CheckpointPath)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($ProgramPath)) { $ProgramPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json' }
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) { $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
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

$checkpoint = Get-Content -LiteralPath $CheckpointPath -Raw | ConvertFrom-Json
$gp3Live = $checkpoint.batch_id -eq 'G1-GP3-ENCOUNTER-GRAMMAR-V1' -and
    $checkpoint.master_program_item -eq 'GP3' -and
    $checkpoint.substage_id -in @(
        'gp3-encounter-grammar-readiness',
        'gp3-encounter-grammar-implementation',
        'gp3-encounter-grammar-verification',
        'gp3-encounter-grammar-recorded'
    )
$gp4Successor = $checkpoint.batch_id -eq 'G1-GP4-SIGNAL-ANCHOR-VERTICAL-V1' -and
    $checkpoint.master_program_item -eq 'GP4' -and
    $checkpoint.substage_id -in @(
        'gp4-signal-anchor-readiness',
        'gp4-signal-anchor-implementation',
        'gp4-signal-anchor-verification',
        'gp4-signal-anchor-recorded'
    )
$closeoutSuccessor = $checkpoint.batch_id -eq 'G1-VERTICAL-CLOSEOUT-V1' -and
    $checkpoint.master_program_item -eq 'G1-VERTICAL-CLOSEOUT' -and
    $checkpoint.substage_id -eq 'g1-vertical-closeout-recorded'
$c4Successor = $checkpoint.batch_id -eq 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1' -and
    $checkpoint.master_program_item -eq 'C4' -and
    $checkpoint.substage_id -in @('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result','c4-independent-platform-gate')
$c5Successor = $checkpoint.batch_id -eq 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' -and
    $checkpoint.master_program_item -eq 'C5' -and $checkpoint.substage_id -eq 'c5-reconciliation-readiness' -and
    $checkpoint.authority_lane -eq 'Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.'
if (!$gp3Live -and !$gp4Successor -and !$closeoutSuccessor -and !$c4Successor -and !$c5Successor) {
    throw 'GP3 readiness is not bound to its canonical route or an admitted authenticated successor.'
}
if ($gp4Successor -or $closeoutSuccessor -or $c4Successor -or $c5Successor) {
    $program = Get-Content -LiteralPath $ProgramPath -Raw | ConvertFrom-Json
    $gp3 = @($program.items | Where-Object id -eq 'GP3')
    $gp4 = @($program.items | Where-Object id -eq 'GP4')
    $closeout = @($program.items | Where-Object id -eq 'G1-VERTICAL-CLOSEOUT')
    $c4 = @($program.items | Where-Object id -eq 'C4')
    $c5 = @($program.items | Where-Object id -eq 'C5')
    $runMatch = if (($closeoutSuccessor -or $c4Successor -or $c5Successor) -and $gp4.Count -eq 1) { [regex]::Match([string]$gp4[0].proof,'run-[0-9a-f]{32}') } else { $null }
    $c4Run = if ($c5Successor -and $c4.Count -eq 1) { [regex]::Match([string]$c4[0].proof,'run-[0-9a-f]{32}') } else { $null }
    $gp4StateValid = if ($closeoutSuccessor -or $c4Successor -or $c5Successor) {
        $gp4.Count -eq 1 -and $gp4[0].state -eq 'verified' -and $gp4[0].status -eq 'complete' -and $runMatch.Success -and @($checkpoint.verification_receipts) -contains "registered-full-gate:$($runMatch.Value):passed" -and
        $closeout.Count -eq 1 -and @($closeout[0].depends_on) -contains 'GP4' -and
        (($closeoutSuccessor -and $closeout[0].state -eq 'executing' -and $closeout[0].status -eq 'active') -or
         ($c4Successor -and $closeout[0].state -eq 'verified' -and $closeout[0].status -eq 'complete' -and $c4.Count -eq 1 -and $c4[0].state -eq 'executing' -and $c4[0].status -eq 'active' -and (@($c4[0].depends_on)-join ',') -eq 'C2,C3A') -or
         ($c5Successor -and $closeout[0].state -eq 'verified' -and $closeout[0].status -eq 'complete' -and $c4.Count -eq 1 -and $c4[0].state -eq 'verified' -and $c4[0].status -eq 'complete' -and (@($c4[0].depends_on)-join ',') -eq 'C2,C3A' -and @($c4[0].sources) -contains 'G1_C4_CLOSURE_RESULT.md' -and $c4Run.Success -and @($checkpoint.verification_receipts) -contains "registered-full-gate:$($c4Run.Value):passed" -and @($checkpoint.verification_receipts) -contains 'receipt:G1-C4-CLOSURE:recorded' -and $c5.Count -eq 1 -and $c5[0].state -eq 'executing' -and $c5[0].status -eq 'active' -and (@($c5[0].depends_on)-join ',') -eq 'C4'))
    } else { $gp4.Count -eq 1 -and $gp4[0].state -eq 'executing' -and $gp4[0].status -eq 'active' }
    if ($gp3.Count -ne 1 -or $gp3[0].state -ne 'promoted' -or $gp3[0].status -ne 'complete' -or
        $gp3[0].proof -notlike '*run-50a8c78043eb46c483f1f655d3793f9b*' -or !$gp4StateValid -or @($gp4[0].depends_on) -notcontains 'GP3') {
        throw 'GP4 or bounded closeout successor does not authenticate recorded GP3 closure.'
    }
}

$route = & (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
if ($route -ne $true) { throw 'GP3 or its authenticated successor is not admitted by the federated interruption route.' }

Write-Output 'G1 GP3 encounter grammar readiness verified: five fixed domain-tagged situations, exact GP0 references, noncombat/retreat/threat boundaries and hostile codec limits are frozen before source.'
