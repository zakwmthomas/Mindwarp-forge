# G1 C5 Significance and Scheduler Closure Readiness

Status: **reconciled readiness; implementation source not yet authorized.**

## Exact boundary

C5 depends exactly on verified C4. It may prove one capability-free, deterministic, integer-only significance and scheduling reference across exactly eight named consumer domains:

1. `generation`
2. `simulation`
3. `ai`
4. `physics`
5. `animation`
6. `audio`
7. `rendering`
8. `streaming`

Cache is not a ninth significance domain. C5 may emit a typed, bounded, disposable residency lease request and trace lease admission, bypass, expiry and churn; it may not implement cache mutation, eviction, frequency policy or runtime residency.

No fixture value is a product weight. No result grants C3B, C6, C7, broad G1 closure, runtime controller or executor, cache mutation, storage mutation, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual-asset or Kernel authority.

## Eight-domain matrix

| Domain | Illustrative work only | Fidelity axis | Required fallback/non-mutation |
|---|---|---|---|
| `generation` | bounded generation/construction proposal | request detail/cadence | cheaper proposal; never rewrites recipe, artifact or canon |
| `simulation` | declared off-screen or local simulation slice | cadence/detail | cheaper slice; never changes age, scale or gameplay capability truth |
| `ai` | sensing/pathfinding/decision slice | sensing and update cadence | cheaper declared behavior evidence; no AI executor or generation |
| `physics` | collision/constraint slice | collision fidelity | cheaper declared approximation; no engine object or runtime physics |
| `animation` | sampling/interpolation/secondary-motion slice | sampling/detail cadence | declared cheaper sampling; no asset or animation implementation |
| `audio` | source/mix evaluation slice | declared audio fidelity | declared cheaper audio evidence; no device or mixer control |
| `rendering` | geometry/material/shader/texture request | presentation axes | identity-preserving cheaper request; no renderer implementation |
| `streaming` | materialisation/residency/prefetch request | population/impostor/culling detail | cancellable cheaper request; no cache or canonical mutation |

## Superseding reconciliation decisions

- One ticket names exactly one Main, CPU, GPU or I/O resource. Multi-resource work is an admitted acyclic graph of phase tickets; the early multi-resource-vector sketch is superseded.
- `SignificanceState` is derived state separate from strict `ImportancePacket` evidence. Continuity and biological age are not shared packet signals and cannot become hidden product priority.
- Cache is not an execution-budget pool. Only a bounded expiring residency lease request and authority-negative trace may be proved here.
- Stable decision codes, not prose reason strings or debug formatting, are canonical trace identity. Human-readable reasons are projections and must not alter the digest.
- Aperiodic hierarchy containers are optional future local hypotheses, not C5 closure requirements.

## Reconciliation result

The retained crate passes 18 focused tests and remains capability-free. Its packet codec, hysteresis, monotone fidelity maps, bounded DAG, four resource pools, safety admission, dependency donation, service debt, cancellation states, stale-epoch quarantine, validated cheaper fallback and deterministic trace hash are reusable.

The old claim that named fidelity maps were the only gap is contradicted by the current code and locked P5 gate. Current closure gaps are:

- four named domains are absent and none of the eight has end-to-end scheduler evidence;
- arbitrary nonzero `u16` consumer IDs are accepted;
- a ticket supplies a packet fingerprint and tier independently, so forged priority and truth forks are not rejected;
- fallback validation does not require the same consumer and work class;
- external completion can be accepted from a pending, inactive-fallback, rejected, already-complete or otherwise invalid state;
- the budget envelope is not canonically bound to the closure fixture;
- the trace lacks self-contained domain/work-class attribution and strict replay bytes;
- route reversal, partial-slice cancellation, eight-domain fairness, independent resource exhaustion and significance-to-dispatch thrash are not composed;
- no typed residency lease, bypass, expiry or churn proof exists.

## Minimal additive candidate

The smallest credible implementation keeps the existing generic mechanics and adds only:

1. a closed `ConsumerDomainV1` with the eight stable domain codes above;
2. a typed domain fidelity record binding domain, packet fingerprint, derived tier and monotone map;
3. admission input that derives or verifies ticket tier from the exact packet/state receipt instead of trusting an independent tier;
4. strict canonical `BudgetEnvelopeV1` bytes and fingerprint covering epoch, four pools, reserves and debt bound;
5. deterministic `AdmissionReceiptV1` for acceptance and every rejection, binding ticket/graph/budget fingerprints and stable reason code;
6. fallback validation requiring identical target, epoch, domain, work class and resource with strictly lower cost and no nested fallback;
7. state-gated completion accepting output only from `Running`, with every stale, cancelled, inactive, rejected, duplicate or terminal completion discarded or rejected deterministically;
8. strict replayable `PressureTraceV2` carrying domain, work class, packet, budget and stable decision identity, including epoch-advance decisions and starvation diagnosis;
9. a capability-free `ResidencyIntentV1` with bounded target, epoch, lease steps and disposition (`request`, `renew`, `expire`, `bypass`) but no cache implementation;
10. one eight-domain integration fixture spanning Main, CPU, GPU and I/O plus exact hostile tests.

No runtime executor, async runtime, clock, hardware measurement, cache implementation, product vocabulary or final fidelity curve belongs in this candidate.

## Frozen hostile registry

### Domain and shared-truth failures

- `domain.unknown-code`
- `domain.zero-code`
- `domain.missing-required`
- `domain.duplicate-required`
- `domain.swapped-map`
- `domain.map-nonmonotone`
- `domain.private-score`
- `truth.packet-zero`
- `truth.packet-mismatch`
- `truth.packet-tier-forged`
- `truth.packet-epoch-mismatch`
- `truth.packet-target-mismatch`
- `truth.policy-mismatch`
- `truth.domain-map-set-mismatch`
- `truth.protection-erased`
- `truth.cross-domain-interference`

### Ticket, graph and fallback failures

- `ticket.unknown-domain`
- `ticket.zero-id`
- `ticket.duplicate-id`
- `ticket.conflicting-id`
- `ticket.unknown-work-class`
- `ticket.unknown-dependency`
- `ticket.self-dependency`
- `ticket.dependency-cycle`
- `ticket.cancellation-cycle`
- `ticket.oversized-graph`
- `ticket.oversized-dependencies`
- `fallback.missing`
- `fallback.same-cost`
- `fallback.more-expensive`
- `fallback.cross-target`
- `fallback.cross-epoch`
- `fallback.cross-domain`
- `fallback.cross-work-class`
- `fallback.cross-resource`
- `fallback.nested`

### Admission, dispatch and fairness failures

- `admission.zero-budget`
- `admission.reserve-over-budget`
- `admission.budget-epoch-mismatch`
- `admission.impossible-safety`
- `admission.deadline-zero`
- `admission.cost-overflow`
- `admission.rejection-unreceipted`
- `budget.noncanonical`
- `budget.fingerprint-mismatch`
- `dispatch.nondeterministic-tie`
- `dispatch.dependency-before-ready`
- `dispatch.donation-persisted`
- `dispatch.donation-after-cancel`
- `dispatch.resource-cross-charge`
- `fairness.background-starved`
- `fairness.debt-overflow`
- `fairness.domain-monopoly`
- `fairness.diagnosis-missing`
- `thrash.focus-oscillation`
- `thrash.route-reversal-stale-work`

### Cancellation and completion failures

- `cancel.stale-epoch`
- `cancel.child-cancels-parent`
- `cancel.missing-acknowledgement`
- `cancel.settle-before-acknowledge`
- `cancel.epoch-advance-untraced`
- `completion.pending-accepted`
- `completion.inactive-fallback-accepted`
- `completion.rejected-accepted`
- `completion.cancelled-accepted`
- `completion.stale-epoch-accepted`
- `completion.duplicate-accepted`
- `completion.terminal-rewrite`
- `completion.partial-output-accepted`

### Residency, trace and authority failures

- `residency.zero-target`
- `residency.zero-lease`
- `residency.stale-epoch`
- `residency.unbounded-lease`
- `residency.expired-retained`
- `residency.bypass-mutates`
- `residency.thrash-untraced`
- `trace.unknown-decision-code`
- `trace.missing-domain`
- `trace.missing-work-class`
- `trace.packet-mismatch`
- `trace.budget-mismatch`
- `trace.reordered-decision`
- `trace.trailing-bytes`
- `trace.replay-drift`
- `authority.runtime-controller`
- `authority.runtime-executor`
- `authority.cache-mutation`
- `authority.storage-mutation`
- `authority.product-weight`
- `authority.ai-generation`
- `authority.rendering-implementation`
- `authority.kernel-mutation`

## Acceptance ladder

1. Static: exact C4 dependency, eight domains, non-goals and 92 hostile IDs are frozen.
2. Typed fixture: strict codecs and every hostile mutation fail closed without capabilities.
3. In-memory composition: all eight domains share one packet-derived tier, span four resources, preserve distinct monotone fidelity, and replay byte-exact decisions.
4. Pressure simulation: stable focus, oscillation, route reversal, protected interaction, overload, starvation, cancellation, fallback, resource exhaustion and residency churn have pinned deterministic traces.
5. Portability: two fresh native processes, same-host i686 execution, Android ARM64 compile, and genuinely independent hosted execution retain identical semantic evidence with honest classifications.
6. Integration: read-only ProofReceipt persistence changes no canonical object, event or candidate counts.
7. Closure: focused verifiers, dependent regressions, independent review and one registered full Forge gate pass. C6 is not activated automatically.

## Implementation gate

Readiness may advance only after the route verifier proves this exact boundary and hostile registry. Product source remains blocked until a separately recorded owner-authorized C5 implementation lane names this minimal additive candidate and preserves deletion-only rollback for new proof surfaces.
