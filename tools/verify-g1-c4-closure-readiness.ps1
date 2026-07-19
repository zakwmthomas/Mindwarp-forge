param([switch]$RouteOnly,[string]$ProgramPath,[string]$CheckpointPath)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if([string]::IsNullOrWhiteSpace($ProgramPath)){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
if([string]::IsNullOrWhiteSpace($CheckpointPath)){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
$program = Get-Content -LiteralPath $ProgramPath -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath $CheckpointPath -Raw | ConvertFrom-Json
$c4 = @($program.items | Where-Object id -eq 'C4')
$closeout = @($program.items | Where-Object id -eq 'G1-VERTICAL-CLOSEOUT')
$active = @($program.items | Where-Object { $_.state -eq 'executing' -and $_.status -eq 'active' })
$authority = [string]$checkpoint.authority_lane
$requiredAuthority = @('Owner-authorized broad C4 hierarchy/history reconciliation and capability-free closure proof only','Exact dependencies C2 and C3A','No C3B','C5','C6','C7','broad G1 closure','runtime','storage engine','filesystem','network','multiplayer','cross-target transactions','Companion','Greenfield','visual assets','Kernel mutation')
$c4Live = $checkpoint.batch_id -eq 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1' -and $checkpoint.master_program_item -eq 'C4' -and
    $checkpoint.substage_id -in @('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result','c4-independent-platform-gate') -and
    $c4.Count -eq 1 -and $c4[0].state -eq 'executing' -and $c4[0].status -eq 'active' -and $active.Count -eq 1 -and $active[0].id -eq 'C4'
$retainedActive = if($active.Count-eq 1){[string]$active[0].id}else{''}
$c4Run=if($c4.Count-eq 1){[regex]::Match([string]$c4[0].proof,'run-[0-9a-f]{32}')}else{$null}
function Complete-Item([string]$id){$item=@($program.items|Where-Object id -eq $id);return $item.Count-eq 1-and$item[0].status-eq'complete'-and$item[0].state-in@('promoted','verified')}
$successorClosure = switch($retainedActive){'C5'{Complete-Item 'C4'};'C6'{(Complete-Item 'C4')-and(Complete-Item 'C5')};'C7'{(Complete-Item 'C4')-and(Complete-Item 'C5')-and(Complete-Item 'C6')};'G1-CLOSEOUT'{(Complete-Item 'C4')-and(Complete-Item 'C5')-and(Complete-Item 'C6')-and(Complete-Item 'C7')};'R1'{Complete-Item 'G1-CLOSEOUT'};default{$false}}
$hasClosureSource = @($c4[0].sources) -contains 'G1_C4_CLOSURE_RESULT.md'
$hasFullGateReceipt = $c4Run.Success -and (@($checkpoint.verification_receipts) -contains "registered-full-gate:$($c4Run.Value):passed")
$hasClosureReceipt = @($checkpoint.verification_receipts) -contains 'receipt:G1-C4-CLOSURE:recorded'
$c4Retained = $c4.Count -eq 1 -and $c4[0].state -eq 'verified' -and $c4[0].status -eq 'complete' -and
    $hasClosureSource -and $hasFullGateReceipt -and $hasClosureReceipt -and
    $retainedActive -in @('C5','C6','C7','G1-CLOSEOUT','R1') -and $checkpoint.master_program_item -eq $retainedActive -and $successorClosure
if (!$c4Live -and !$c4Retained) { throw "C4 readiness route invalid: active=$retainedActive source=$hasClosureSource full_gate=$hasFullGateReceipt closure=$hasClosureReceipt successor=$successorClosure checkpoint=$($checkpoint.master_program_item)." }
if ((@($c4[0].depends_on)-join ',') -ne 'C2,C3A' -or $closeout.Count -ne 1 -or $closeout[0].state -ne 'verified' -or $closeout[0].status -ne 'complete') { throw 'C4 readiness is not bound to exact C2+C3A and verified vertical closeout evidence.' }
if($c4Live){foreach ($token in $requiredAuthority) { if (!$authority.Contains($token)) { throw "C4 authority boundary missing: $token" } }}
if ($c4Live -and ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -ne $true)) { throw 'C4 route is not admitted by the federated interruption guard.' }
$pins = [ordered]@{
  'C2'='bbd80968996612cca154ad29e324d9fdeb50072f38fd41d2c43699bacdb2da3d'; 'C3A'='61e1fe3bdea3f000cd6f08cf51bbe7f4d3e2baaa506700e495b07a35f8bbec9b'; 'C4V'='eec2ea37577af35147ac85b3082f85a98c58e1acbbcd7dd85d4377c595e676a0'; 'G1-VERTICAL-CLOSEOUT'='2963d0c1b7e679b88e4db273fe24fbcdf9de09894fb0a5c5e8e905c5c73966d6'
}
if($c4Live){$pins['C5']='c6cadcfff651ee48e46214c9b7e94554dc546e97a3df56c459bd5ef843bed24b';$pins['C6']='00fefa9ada5f788ab1c470b9abed83b2c85c2529b870c9aff1be6c12af477511';$pins['C7']='bddc31b69fe9a9c15805bcd8f52c1edcd1868bd55d38192cfc117b1582bc1131';$pins['G1-CLOSEOUT']='0f568deda22a0901030948cb9c9355916cbfe2319e943267f1cd6135c5fe39a5';$pins['R1']='87ef69f50f4d4cd7fa297d19cae56e77db1cac53605f12849c3cc380156b9869'}
$sha = [Security.Cryptography.SHA256]::Create()
try { foreach ($entry in $pins.GetEnumerator()) { $item=@($program.items|Where-Object id -eq $entry.Key); if($item.Count-ne 1){throw "Missing pinned successor item: $($entry.Key)"}; $bytes=[Text.Encoding]::UTF8.GetBytes(($item[0]|ConvertTo-Json -Depth 20 -Compress)); $actual=(($sha.ComputeHash($bytes)|ForEach-Object{$_.ToString('x2')})-join ''); if($actual-ne$entry.Value){throw "$($entry.Key) changed during C4 readiness: $actual"} } } finally { $sha.Dispose() }
if (!$RouteOnly) {
  $readiness=Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C4_CLOSURE_READINESS.md') -Raw
  $design=Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C4_CLOSURE_DESIGN.md') -Raw
  $contract=Get-Content -LiteralPath (Join-Path $root 'contracts\hierarchy-history-contract.md') -Raw
  foreach($token in @('0`, `1`, `16`, `256','0`, `1`, `16`, `64`, `256','1024','16 MiB','at most `2` hops','candidate_verified_local','same_host_second_architecture','compile_only')){if(!$readiness.Contains($token)){throw "C4 readiness bound missing: $token"}}
  foreach($token in @('dynamic_instance_logical_id','AddressPresence','NeverObserved','Absent','AmbientCohortBindingV1','verify_available_dependencies','recover_known_good_prefix','RecoveryFailureKind','UnsupportedTopology','C4SemanticReceiptV1','input_sha256','packet_id','packet_sha256','world_conditions_fingerprint','window_rows','history_rows','full_stream_bytes','snapshot_bytes','production_storage=false','runtime_residency=false','cross_target_transactions=false','c3b=false','promotion_authority=false')){if(!$design.Contains($token)){throw "C4 design field missing: $token"}}
  foreach($token in @('execution provenance and platform diversity','same_host_second_architecture','compile_only')){if(!$contract.Contains($token)){throw "C4 portability contract missing: $token"}}
  $hostiles=@('identity.dynamic-zero-parent','identity.dynamic-zero-instance','identity.dynamic-domain-drift','identity.dynamic-vector-drift','presence.unknown-tag','presence.state-substitution','presence.zero-fingerprint','presence.trailing-bytes','cohort.zero-entity','cohort.zero-contract','cohort.entity-drift','cohort.contract-drift','cohort.value-drift','cohort.reroll','cohort.trailing-bytes','dependency.manifest-invalid','dependency.missing','dependency.fingerprint-mismatch','dependency.extra','dependency.c3b-extra','dependency.unsorted','dependency.duplicate','dependency.zero-kind','history.wrong-baseline','history.wrong-target','history.gap','history.stale-head','history.fork','history.command-conflict','history.unknown-schema','history.cross-target','history.reparent','history.split','history.merge','history.corrupt-envelope','history.truncated-envelope','history.trailing-envelope','history.recovery-past-prefix','history.recovery-bound-overflow','snapshot.wrong-baseline','snapshot.wrong-head','snapshot.wrong-sequence','snapshot.wrong-reducer','snapshot.wrong-builder','snapshot.wrong-state','snapshot.wrong-hash','migration.missing-adapter','migration.zero-adapter','migration.duplicate-adapter','migration.wrong-logical-id','migration.same-baseline','migration.reordered-hop','migration.noncontiguous-hop','migration.failed-hop','migration.overbound','migration.altered-source','migration.changed-retry','migration.receipt-tamper','receipt.unknown-field','receipt.missing-field','receipt.dependency-reorder','receipt.type-coercion','receipt.proof-drift','receipt.source-drift','receipt.authority-flip','receipt.hash-drift','portability.single-process','portability.stdout-mismatch','portability.source-mismatch','portability.compile-as-execution','portability.same-host-as-independent','portability.same-platform-remote','portability.target-drift','portability.runner-drift')
  if($hostiles.Count-ne 74){throw 'C4 hostile registry count changed.'};foreach($id in $hostiles){if(!$readiness.Contains("``$id``")){throw "C4 hostile ID missing: $id"}}
  $hsha=[Security.Cryptography.SHA256]::Create();try{$body='mindwarp/c4-hostile-registry/v1'+[char]0+($hostiles-join "`n");$digest=(($hsha.ComputeHash([Text.Encoding]::UTF8.GetBytes($body))|ForEach-Object{$_.ToString('x2')})-join '')}finally{$hsha.Dispose()};if($digest-ne'4d4b7cb792f5b410092d247354bac62a5b8f3dc880fcb2a6ad61ffafadff127c'-or!$readiness.Contains($digest)){throw 'C4 hostile registry digest changed.'}
  if($checkpoint.substage_id-eq'c4-reconciliation-readiness'-and(Test-Path -LiteralPath (Join-Path $root 'crates\hierarchy-history\src\closure.rs'))){throw 'C4 hardening source exists before readiness transition.'}
  & (Join-Path $root 'tools\test-g1-c4-successor-route.ps1') | Out-Null; if(!$?){throw 'C4 successor hostile fixture failed.'}
}
Write-Output 'G1 C4 closure readiness verified: exact C2+C3A route, predecessor receipt, later-system pins, semantic bounds, portability classifications and authority exclusions are frozen.'
