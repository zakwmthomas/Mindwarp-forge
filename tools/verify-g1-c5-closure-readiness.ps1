param([string]$ProgramPath,[string]$CheckpointPath,[string]$ReadinessPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c5-successor-route.ps1')
if([string]::IsNullOrWhiteSpace($ProgramPath)){$ProgramPath=Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'}
if([string]::IsNullOrWhiteSpace($CheckpointPath)){$CheckpointPath=Join-Path $root 'context\active\WORKER_BATCH_STATE.json'}
if([string]::IsNullOrWhiteSpace($ReadinessPath)){$ReadinessPath=Join-Path $root 'docs\canonical-system\G1_C5_CLOSURE_READINESS.md'}
$program=Get-Content -LiteralPath $ProgramPath -Raw|ConvertFrom-Json
$checkpoint=Get-Content -LiteralPath $CheckpointPath -Raw|ConvertFrom-Json
$readiness=Get-Content -LiteralPath $ReadinessPath -Raw
$c4=@($program.items|Where-Object id -eq 'C4')
$c5=@($program.items|Where-Object id -eq 'C5')
$active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
$readinessAuthority='Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.'
$implementationAuthority='Owner-authorized bounded C5 significance/scheduler implementation and capability-free closure proof only. Exact dependency C4. Frozen candidate G1_C5_CLOSURE_READINESS.md. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.'
if($c4.Count-ne1-or$c4[0].state-ne'verified'-or$c4[0].status-ne'complete'-or(@($c4[0].depends_on)-join',')-ne'C2,C3A'){throw 'C5 readiness requires exact verified C4.'}
if($c5.Count-ne1-or$c5[0].state-ne'executing'-or$c5[0].status-ne'active'-or(@($c5[0].depends_on)-join',')-ne'C4'-or@($c5[0].sources)-notcontains'G1_C5_CLOSURE_READINESS.md'){throw 'C5 readiness is not bound to exact active C5.'}
$routeAuthorityOk=($checkpoint.substage_id-in@('c5-reconciliation-readiness','c5-implementation-owner-gate')-and$checkpoint.authority_lane-eq$readinessAuthority)-or($checkpoint.substage_id-eq'c5-implementation'-and$checkpoint.authority_lane-eq$implementationAuthority-and@($checkpoint.verification_receipts)-contains'owner-authorization:c5-frozen-implementation-candidate:released')-or((Test-G1C5FullGateReconciliationRoute -Checkpoint $checkpoint)-and@($checkpoint.verification_receipts)-contains'owner-authorization:c5-frozen-implementation-candidate:released')
if($active.Count-ne1-or$active[0].id-ne'C5'-or$checkpoint.batch_id-ne'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1'-or$checkpoint.master_program_item-ne'C5'-or!$routeAuthorityOk){throw 'C5 checkpoint route or authority is not exact.'}
$domains=@('generation','simulation','ai','physics','animation','audio','rendering','streaming')
foreach($domain in $domains){if(!$readiness.Contains("| ``$domain`` |")){throw "C5 domain missing: $domain"}}
foreach($token in @('One ticket names exactly one Main, CPU, GPU or I/O resource','Continuity and biological age are not shared packet signals','Cache is not an execution-budget pool','Stable decision codes, not prose reason strings','implementation source not yet authorized','C6 is not activated automatically')){if(!$readiness.Contains($token)){throw "C5 reconciliation decision missing: $token"}}
$hostiles=@(
'domain.unknown-code','domain.zero-code','domain.missing-required','domain.duplicate-required','domain.swapped-map','domain.map-nonmonotone','domain.private-score',
'truth.packet-zero','truth.packet-mismatch','truth.packet-tier-forged','truth.packet-epoch-mismatch','truth.packet-target-mismatch','truth.policy-mismatch','truth.domain-map-set-mismatch','truth.protection-erased','truth.cross-domain-interference',
'ticket.unknown-domain','ticket.zero-id','ticket.duplicate-id','ticket.conflicting-id','ticket.unknown-work-class','ticket.unknown-dependency','ticket.self-dependency','ticket.dependency-cycle','ticket.cancellation-cycle','ticket.oversized-graph','ticket.oversized-dependencies',
'fallback.missing','fallback.same-cost','fallback.more-expensive','fallback.cross-target','fallback.cross-epoch','fallback.cross-domain','fallback.cross-work-class','fallback.cross-resource','fallback.nested',
'admission.zero-budget','admission.reserve-over-budget','admission.budget-epoch-mismatch','admission.impossible-safety','admission.deadline-zero','admission.cost-overflow','admission.rejection-unreceipted','budget.noncanonical','budget.fingerprint-mismatch',
'dispatch.nondeterministic-tie','dispatch.dependency-before-ready','dispatch.donation-persisted','dispatch.donation-after-cancel','dispatch.resource-cross-charge',
'fairness.background-starved','fairness.debt-overflow','fairness.domain-monopoly','fairness.diagnosis-missing','thrash.focus-oscillation','thrash.route-reversal-stale-work',
'cancel.stale-epoch','cancel.child-cancels-parent','cancel.missing-acknowledgement','cancel.settle-before-acknowledge','cancel.epoch-advance-untraced',
'completion.pending-accepted','completion.inactive-fallback-accepted','completion.rejected-accepted','completion.cancelled-accepted','completion.stale-epoch-accepted','completion.duplicate-accepted','completion.terminal-rewrite','completion.partial-output-accepted',
'residency.zero-target','residency.zero-lease','residency.stale-epoch','residency.unbounded-lease','residency.expired-retained','residency.bypass-mutates','residency.thrash-untraced',
'trace.unknown-decision-code','trace.missing-domain','trace.missing-work-class','trace.packet-mismatch','trace.budget-mismatch','trace.reordered-decision','trace.trailing-bytes','trace.replay-drift',
'authority.runtime-controller','authority.runtime-executor','authority.cache-mutation','authority.storage-mutation','authority.product-weight','authority.ai-generation','authority.rendering-implementation','authority.kernel-mutation')
if($hostiles.Count-ne92){throw 'C5 hostile registry implementation count changed.'}
foreach($id in $hostiles){if(!$readiness.Contains("``$id``")){throw "C5 hostile ID missing: $id"}}
Write-Output 'G1 C5 closure readiness verified: exact C4 dependency, eight domains, supersessions, 92 hostiles and authority-negative source gate are frozen.'
