param([string]$ProgramPath,[string]$CheckpointPath,[string]$ResultPath,[switch]$RouteOnly)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c5-successor-route.ps1')
if ([string]::IsNullOrWhiteSpace($ProgramPath)) { $ProgramPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json' }
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) { $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
if ([string]::IsNullOrWhiteSpace($ResultPath)) { $ResultPath = Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_RESULT.md' }
$readinessPath = Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_READINESS.md'
$designPath = Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_DESIGN.md'
$registryPath = Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_FIXED_REGISTRY.md'
foreach ($path in @($readinessPath,$designPath,$registryPath)) {
  if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "GP4 readiness artifact missing: $path" }
}
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$design = Get-Content -LiteralPath $designPath -Raw
$registry = Get-Content -LiteralPath $registryPath -Raw
$joined = $readiness + "`n" + $design + "`n" + $registry

foreach ($token in @(
  'fixed hub frame', 'gp4.signal-anchor.vertical-1', 'gp0.s4.signal-anchor',
  'gp3.s4.signal-anchor', 's4.approach.temporary', 's4.temporary-rescue',
  'temporary-brace-kit', 'wire-scavengers', 'four C4V command batches containing five GP1 actions',
  'all thirteen fields other than `world_context`', 'No `..` remainder',
  'ledger_before', 'ledger_after', 'revision 3', 'revision-4',
  'GP2 accepts only the terminal authored shadow', 'No invented `gp0_contract_digest`',
  'twenty-five semantic slots', 'twenty-nine-row hard/compare matrix',
  '8,388,608 bytes', 'after parse but before dependency traversal',
  'broad_g1=false', 'runtime_containment_pending=true',
  'runtime_selected=false', 'promotion_authority=false'
)) { if (!$joined.Contains($token)) { throw "GP4 readiness missing exact token: $token" } }

$digests = @(
  'e8865e011d8b7ada0787303d49e4c769ff19164dc7a51f52d396e80b2c408b44',
  'c258b1b83e86cc52f30502c8e8d29d7bbda161ce7abc1031d5612a65c84d5328',
  'b6428f7febdfab0560b975f02b47bb3dcbd7e940a4a88fd65028aa6c685a4033',
  '9d9e3507f19953aef3c7a2013fac50c370d15e94013318c45d5b31fad33aa248',
  '21b33ac9883afa4df0667bd720af3c7f12b956726e8f2841f1eed1fd9fd0a24f',
  '5e60a42c78bc085662fe8264fb362ecd9af141b5aa874d0ef8013254fcf3a735',
  '5f54137fa9de4b06514dbfde509ef5faf65a23b885a24288ed5cb51bbcee07ca',
  'e3479b36a3e7085ae892a358ba7e5e6415688ef0d82e0338b9226ae71c46576f',
  'd7f9437bd750f5a85660e2a604fa894be22c7689f9983f565be64bb653ac6867',
  '5ef8f69963d20b11bb57b99250ed0e934dd998b081b60085fdf2b18200722884',
  '7034df7aa827fd3c6f24a2d9a5113c7b9b0d6415bf0ca44e300ead40cfe1282f',
  'daaac2a5e65ad645f6ce7319f1429d577947a6bc760c6a5748ba267fa1603684',
  'ef4cd659fcfc59290288babb82935cc081c9a53a2796d0030a4a269496c62c07',
  '1caea41670d9f63b3f454219a0b995d69ae35ebd800c4d6dd5f608bf3523f0d5',
  'febd62dc54cecd95ab7c7de6c95597b8880da7f756f9355997bbcf36ff369b87'
)
foreach ($digest in $digests) { if (!$registry.Contains($digest)) { throw "GP4 fixed digest missing: $digest" } }

$commandIds = @(
  '6fa6a6d429003d91fb4f577486a34ff4bf174e16e659c966ccf3327e8dd2cc15',
  '287ccf55549997ecd90f3fd8fc202bd7c92be386923e1954a9324554b343fda1',
  '42a8db36764d01b533e7adf62c87fb226685471ca2b2f23ac3253cec330b9da1',
  'd3dd8df1f02284cebeb677d1136ebaa1ceb095edb3704aedf7e6fe9a57344fff'
)
foreach ($id in $commandIds) { if (!$registry.Contains($id)) { throw "GP4 command ID missing: $id" } }
foreach ($label in @('`prepare`','`depart-and-choose-outcome`','`begin-return`','`record-remembered-response`')) {
  if (!$registry.Contains($label)) { throw "GP4 command label missing: $label" }
}
if (!$registry.Contains('exactly one `0x00` byte') -or !$registry.Contains('`B` is exactly the UTF-8 bytes of fixed `bundle_id`')) {
  throw 'GP4 command framing does not define NUL or bundle ID bytes.'
}

$slotIds = @(
 'hub-status','player-actor','iven-absent','signal-anchor-opportunity','anchor-broken-state',
 'signal-window-evidence','wire-scavenger-threat','anchor-collapse-risk','temporary-brace-tool',
 'temporary-rescue-choice','temporary-brace-intervention','work-area-safe','anchor-brace-temporary',
 'temporary-crossing','iven-returned','signal-coordinate-recorded','caravan-delayed','brace-expired',
 'permanent-repair-incomplete','remembered-response','next-decision','rev1-prepared-stop',
 'rev2-consequence-stop','rev3-return-prefix','rev4-terminal'
)
foreach ($id in $slotIds) {
  if (($registry | Select-String -Pattern ([regex]::Escape("| ``$id`` |")) -AllMatches).Matches.Count -ne 1) {
    throw "GP4 semantic slot row missing or duplicated: $id"
  }
}

$hard = @('strict-bundle-roundtrip','exact-dependency-digests','c2-c3a-identity','gp1-action-stable-order','gp3-approach-evidence-risk','c4v-append-restart','gp2-authored-shadow-isolation','no-duplicate-memory-progression','semantic-slot-coverage','accessibility-equivalence','no-canonical-mutation','no-ambient-authority','headless-deterministic-tests','clean-target-build','runtime-provenance-licensing','containment-teardown')
$compare = @('cold-build-import','incremental-iteration','bundle-validation-restart-latency','input-semantic-feedback-latency','cpu-gpu-frame-pacing','peak-steady-memory','binary-asset-project-size','mobile-battery-thermal','adapter-dependency-surface','debugging-profiling','platform-export-coverage','upgrade-maintenance-risk','owner-play-comprehension')
foreach ($id in $hard) { if (!$registry.Contains("``hard.$id``")) { throw "GP4 hard requirement missing: $id" } }
foreach ($id in $compare) { if (!$registry.Contains("``compare.$id``")) { throw "GP4 compare requirement missing: $id" } }

foreach ($field in @('schema_version','bundle_id','session_bytes','c3a_input_bytes','c3a_packet_bytes','c4v_log_bytes','return_prefix_snapshot_bytes','final_snapshot_bytes','persistence_receipt_bytes','command_ids','authored_shadow_state_bytes','common_semantic_digest','gp3_situation_bytes','gp4_approach_ref_digest','gp3_threat_digest','gp4_threat_ref_digest','threat_selected','progression_ledger_bytes','presentation_slots','adapter_requirements','bundle_digest')) {
  if (!$design.Contains("``$field")) { throw "GP4 bundle field missing: $field" }
}

$program = Get-Content -LiteralPath $ProgramPath -Raw | ConvertFrom-Json
$gp4 = @($program.items | Where-Object id -eq 'GP4')
$closeout = @($program.items | Where-Object id -eq 'G1-VERTICAL-CLOSEOUT')
$c4 = @($program.items | Where-Object id -eq 'C4')
$c5 = @($program.items | Where-Object id -eq 'C5')
$expectedDependencies = @('C3A','C4V','GP0','GP1','GP2','GP3','GP4')
$broad = @($program.items | Where-Object id -eq 'G1-CLOSEOUT')
if ($broad.Count -ne 1 -or $broad[0].state -eq 'promoted') { throw 'Broad G1 closeout was changed by GP4 readiness.' }

$checkpoint = Get-Content -LiteralPath $CheckpointPath -Raw | ConvertFrom-Json
$gp4Live = $checkpoint.batch_id -eq 'G1-GP4-SIGNAL-ANCHOR-VERTICAL-V1' -and $checkpoint.master_program_item -eq 'GP4' -and
    $checkpoint.substage_id -in @('gp4-signal-anchor-readiness','gp4-signal-anchor-implementation','gp4-signal-anchor-verification','gp4-signal-anchor-recorded') -and
    $gp4.Count -eq 1 -and $gp4[0].state -eq 'executing' -and $gp4[0].status -eq 'active' -and
    $closeout.Count -eq 1 -and $closeout[0].state -eq 'proposed' -and $closeout[0].status -eq 'gated'
$closeoutSuccessor = $checkpoint.batch_id -eq 'G1-VERTICAL-CLOSEOUT-V1' -and $checkpoint.master_program_item -eq 'G1-VERTICAL-CLOSEOUT' -and
    $checkpoint.substage_id -eq 'g1-vertical-closeout-recorded' -and
    $gp4.Count -eq 1 -and $gp4[0].state -eq 'verified' -and $gp4[0].status -eq 'complete' -and $gp4[0].proof -match 'run-[0-9a-f]{32}' -and
    $closeout.Count -eq 1 -and $closeout[0].state -eq 'executing' -and $closeout[0].status -eq 'active'
$c4Successor = $checkpoint.batch_id -eq 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1' -and $checkpoint.master_program_item -eq 'C4' -and
    $checkpoint.substage_id -in @('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result','c4-independent-platform-gate') -and
    $gp4.Count -eq 1 -and $gp4[0].state -eq 'verified' -and $gp4[0].status -eq 'complete' -and $gp4[0].proof -match 'run-[0-9a-f]{32}' -and
    $closeout.Count -eq 1 -and $closeout[0].state -eq 'verified' -and $closeout[0].status -eq 'complete' -and
    $c4.Count -eq 1 -and $c4[0].state -eq 'executing' -and $c4[0].status -eq 'active' -and (@($c4[0].depends_on)-join ',') -eq 'C2,C3A'
$c4Run = if($c4.Count-eq1){[regex]::Match([string]$c4[0].proof,'run-[0-9a-f]{32}')}else{$null}
$c5Route = ($checkpoint.batch_id -eq 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' -and $checkpoint.master_program_item -eq 'C5' -and $checkpoint.substage_id -eq 'c5-reconciliation-readiness' -and
    $checkpoint.authority_lane -eq 'Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.') -or
    (Test-G1C5FullGateReconciliationRoute -Checkpoint $checkpoint) -or
    (Test-G1C5RecordedClosureRoute -Checkpoint $checkpoint)
$c5Successor = $c5Route -and
    $gp4.Count -eq 1 -and $gp4[0].state -eq 'verified' -and $gp4[0].status -eq 'complete' -and $gp4[0].proof -match 'run-[0-9a-f]{32}' -and
    $closeout.Count -eq 1 -and $closeout[0].state -eq 'verified' -and $closeout[0].status -eq 'complete' -and
    $c4.Count -eq 1 -and $c4[0].state -eq 'verified' -and $c4[0].status -eq 'complete' -and @($c4[0].sources) -contains 'G1_C4_CLOSURE_RESULT.md' -and $c4Run.Success -and
    @($checkpoint.verification_receipts) -contains "registered-full-gate:$($c4Run.Value):passed" -and @($checkpoint.verification_receipts) -contains 'receipt:G1-C4-CLOSURE:recorded' -and
    $c5.Count -eq 1 -and $c5[0].state -eq 'executing' -and $c5[0].status -eq 'active' -and (@($c5[0].depends_on)-join ',') -eq 'C4'
if ((!$gp4Live -and !$closeoutSuccessor -and !$c4Successor -and !$c5Successor) -or $closeout.Count -ne 1 -or (Compare-Object @($closeout[0].depends_on) $expectedDependencies)) { throw 'GP4 readiness checkpoint or authenticated successor drifted.' }
$runMatch = if ($closeoutSuccessor -or $c4Successor -or $c5Successor) { [regex]::Match([string]$gp4[0].proof,'run-[0-9a-f]{32}') } else { $null }
if (($closeoutSuccessor -or $c4Successor -or $c5Successor) -and (!$runMatch.Success -or @($checkpoint.verification_receipts) -notcontains "registered-full-gate:$($runMatch.Value):passed" -or !(Get-Content -LiteralPath $ResultPath -Raw).Contains($runMatch.Value))) { throw 'GP4 successor lost exact successful-run consistency.' }
$route = & (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
if ($route -ne $true) { throw 'GP4 or its bounded closeout successor is not admitted by the interruption route.' }
if ($checkpoint.substage_id -eq 'gp4-signal-anchor-readiness' -and (Test-Path -LiteralPath (Join-Path $root 'crates\mindwarp-signal-anchor-vertical'))) {
  throw 'GP4 source exists before readiness acceptance.'
}

if (!$RouteOnly) {
  & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-computation.ps1')
  if (!$?) { throw 'GP4 computational readiness fixture failed.' }
  & (Join-Path $root 'tools\test-g1-gp4-signal-anchor-readiness-hostile.ps1')
  if (!$?) { throw 'GP4 hostile readiness fixture failed.' }
}

Write-Output 'G1 GP4 Signal Anchor readiness verified: exact bundle schema, dual-world semantics, dependency receipts, 25 semantic slots, 29 unmeasured adapter rows and bounded closeout are frozen before source.'
