# Significance and Scheduler P5 Design Gate

**Status:** owner-gated design; no implementation authority.  
**Evidence date:** 2026-07-13.

## Decision question

Approve a capability-free, deterministic P5 reference harness that tests the
contracts below using simulated integer cost units. Approval would not select
gameplay weights, a runtime engine, real frame budgets, hardware targets,
production cache policy, or authority to materialise or mutate canonical state.

## Recovered prototype reconciliation

The fixed survival pack contains five bounded Python prototype bundles:

| Nested package | Bytes | Retained tests | Useful evidence |
|---|---:|---:|---|
| `mindwarp_frame_budget_scheduler_test.zip` | 147546 | 4 | Chunking, dependency order, fixed background budget |
| `mindwarp_integrated_perceptual_scheduler_test_v2.zip` | 225103 | 4 | Route change, separate resource budgets, cache events |
| `mindwarp_perceptual_noise_streaming_test_v2.zip` | 273787 | 3 | Predictive streaming and field-driven fixture data |
| `mindwarp_research_consolidated_scheduler.zip` | 333976 | 4 | Arrival-relative deadlines and adaptive budget sketch |
| `mindwarp_temporal_lod_scheduler_test.zip` | 188463 | 4 | Shared temporal cadence and combat protection sketch |

These are evidence, not reusable production modules. They prove that the
problem can be simulated, but their passing reports mostly cover happy paths.
They hard-code floating-point weights, milliseconds, megabytes, category order,
game modes, and one scalar score. Those values are not owner-approved product
semantics and are not cross-platform canonical data.

The following gaps prevent direct reuse:

1. No prototype implements promotion/demotion hysteresis state; oscillating
   inputs can therefore flap.
2. Score sorting has no fair-share debt or bounded aging, so continuously
   urgent work can starve valid background work.
3. Deadline urgency saturates at the deadline and does not distinguish how far
   overdue work is, whether admission was feasible, or whether fallback ran.
4. Cancellation is a boolean. There is no request/acknowledge lifecycle,
   partial-output quarantine, dependent invalidation, or stale completion test.
5. Fallbacks are unvalidated strings and are not demonstrated to be cheaper,
   present, or executed.
6. Dependencies are trusted: unknown nodes and cycles do not have a complete
   fail-closed validation proof.
7. Sequential resource loops create an incidental main/CPU/GPU/I/O order even
   though no global cross-resource order was designed.
8. Cache pins have no separate reservation budget or lease-expiry proof.
9. Duplicate IDs, deterministic tie-breaking, impossible cost vectors,
   overload admission, poison packets, and authority-negative behavior are not
   covered across the bundle set.

## External primary-source reconciliation

- Linux EEVDF separates responsiveness from fairness using eligibility,
  virtual deadlines, and lag (work owed or over-served). P5 should therefore
  keep a bounded fairness debt instead of trying to encode starvation into the
  significance score: <https://www.kernel.org/doc/html/v6.12/scheduler/sched-eevdf.html>.
- Linux deadline scheduling states that deadline guarantees require admission
  control and bounded utilization; overload invalidates the guarantee. P5 must
  reject or degrade infeasible tickets before claiming deadline safety:
  <https://docs.kernel.org/scheduler/sched-deadline.html>.
- Google SRE recommends bounded queues, early load shedding, cheaper degraded
  results, and explicit overload testing; it also warns that complex degradation
  paths can form harmful feedback loops. P5 fallbacks stay typed, visible, and
  deliberately small: <https://sre.google/sre-book/addressing-cascading-failures/>.
- Kubernetes scaling uses tolerance and stabilization windows to prevent
  flapping. This supports explicit asymmetric promotion/demotion policy rather
  than the recovered prototypes' instantaneous score response:
  <https://kubernetes.io/docs/reference/kubernetes-api/autoscaling/horizontal-pod-autoscaler-v2/>.
- Tokio documents that cancellation propagation is not atomic and that a
  wrapped future is cancellation-safe only if the future itself is. P5 must
  treat cancellation as a protocol with acknowledged output disposition, not
  as proof that work stopped instantly:
  <https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html>.
- Unreal's async-loading guidance explicitly warns about races, flushes, global
  interactions, and synchronous loading inside the loader. This supports
  capability-free tickets and a ban on arbitrary callbacks or canonical-state
  mutation in the reference scheduler:
  <https://dev.epicgames.com/documentation/unreal-engine/asynchronous-level-loading-in-unreal-engine>.

These sources are design analogues. P5 does not adopt Linux, Kubernetes, Tokio,
Google infrastructure, or Unreal runtime APIs.

## Recommended minimal contract

### Shared significance

`ImportancePacket` is a versioned, bounded integer signal vector plus reason
codes, protection flags, and a deterministic tier. The test-only vector covers
focus/visibility, interaction or threat, predicted need and confidence, and
continuity. It is not a single universal float.

Every consumer uses the same packet but declares a versioned
`ConsumerFidelityMap`. Rendering, AI, physics, audio, animation, generation,
and streaming may select different fidelity from the same evidence; none may
invent a private global significance score.

`SignificanceState` holds explicit asymmetric enter/leave thresholds and
minimum hold steps. It is derived state, not canonical world history. Protected
interaction/threat flags define a bounded minimum tier, never unlimited budget.

### Scheduler

`WorkTicket` contains a stable ID, target descriptor fingerprint, request epoch,
work class, dependency IDs, bounded multi-resource cost vector, deadline class,
typed fallback, cancellation scope, and output disposition. It contains data,
never code, engine objects, filesystem/network access, or mutation authority.

`BudgetEnvelope` has independently named resource pools with simulated integer
units. P5 does not call these units milliseconds, bytes, FPS, or hardware
capacity. Reservations are bounded and unused reserved capacity is reclaimable.

Scheduling has two stages:

1. Validation/admission rejects malformed graphs, cycles, unknown dependencies,
   duplicate/conflicting IDs, impossible costs, absent fallbacks, stale versions,
   and infeasible hard-deadline sets.
2. Dispatch considers deadline class, shared significance tier, dependency
   readiness, and bounded fair-share debt. A total deterministic tie-breaker is
   explicit. No one scalar is treated as the meaning of all four inputs.

Deadline classes are `interaction_safety`, `visible_minimum`, and
`quality_target`. Only the first can request a bounded reservation. On overload,
quality falls back first, then visible work uses its declared minimum; impossible
safety demand emits a failed admission receipt rather than a false guarantee.

Cancellation states are `requested`, `acknowledged`, and `settled`. Completion
from a cancelled or stale request epoch is retained as trace evidence but its
output is discarded. A parent cancellation cascades to children; cancelling a
child does not cancel its parent.

Cache pinning is a disposable, bounded lease. The scheduler may request
materialisation or residency but cannot create identity, rewrite a descriptor,
append history, or infer a total population from a P4 observation window.

## Required adversarial proof

- Strict packet/ticket bytes, bounded dimensions, version drift, poison values,
  duplicate/conflicting IDs, deterministic ties, unknown dependencies, cycles,
  and oversized graphs.
- Focus oscillation across both thresholds; minimum holds prevent flapping while
  urgent protection promotes immediately and demotion remains conservative.
- Stable focus, route reversal, stale-epoch completion, cancellation during a
  partial slice, parent/child cancellation, fallback activation, and no partial
  output acceptance.
- Deadline admission at, below, and above capacity; quality degradation,
  bounded safety reservation, reclaimed unused reservation, and visible failed
  admission when safety demand is impossible.
- Sustained urgent traffic plus background work: fair-share debt either advances
  the background ticket within its bound or emits a starvation receipt.
- Independent resource exhaustion, cache thrash, lease expiry, cache bypass,
  descriptor immutability, and no eager hierarchy enumeration.
- Exact replay from the same trace, increasing queue/window cost measurements,
  read-only ProofReceipt integration, and absence of approval, promotion,
  execution, publishing, spending, credential, engine, or Kernel authority.

## Alternatives rejected or deferred

| Alternative | Result |
|---|---|
| Reuse recovered Python scheduler directly | Rejected: accidental product values and missing failure protocols |
| One weighted global score | Rejected: hides incompatible meanings and starvation behavior |
| Pure earliest-deadline-first | Rejected: deadlines need admission and do not encode player significance or fairness |
| Fixed category priority | Rejected: can permanently starve lower classes and freezes taxonomy |
| Generic async runtime now | Deferred: engine/runtime selection remains gated |
| CRDT/distributed scheduler | Rejected: no multiplayer/distributed authority requirement exists |
| Real millisecond/FPS targets | Deferred: requires selected runtime and measured hardware |

## Exact confirmation

Approve the bounded capability-free P5 reference harness using shared integer
importance packets, explicit hysteresis, typed admission/fallback/cancellation,
separate simulated resource budgets, deterministic dispatch, and bounded
fairness debt. This approval does not select final weights, game modes, runtime
budgets, cache sizes, an engine, or production scheduling behavior.

## Conditional-approval critical revalidation

The owner approved the bounded harness on condition that P5 receive the same
second-wave research, whole-system, simplification, and permanent-quality audit
used for P4. The condition is satisfied, but the implementation surface is
smaller and the failure protocol is stronger than the first gate.

### Additional primary evidence

- Linux priority-inheritance documentation shows that urgent work can remain
  blocked indefinitely when its lower-priority prerequisite is itself displaced.
  P5 therefore donates an effective urgency/deadline through the validated task
  DAG without rewriting the prerequisite's canonical significance:
  <https://docs.kernel.org/next/locking/rt-mutex-design.html>.
- Deficit Round Robin demonstrates that variable-size work can retain a cheap,
  explicit fairness balance rather than rely on score aging hidden inside a
  global priority formula. P5 uses bounded service debt only as a discriminating
  reference fixture, not as the future runtime algorithm:
  <https://openscholarship.wustl.edu/cse_research/339/>.
- Google's Borg report combines admission control, resource isolation,
  simulation, and declarative work descriptions. Later trace analysis also
  reports that dependencies account for much failure and workloads are heavily
  skewed. P5 must test dependency failure and mixed work sizes rather than only
  uniform happy paths: <https://research.google/pubs/large-scale-cluster-management-at-google-with-borg/>
  and <https://research.google/pubs/borg-the-next-generation/>.
- Unreal's task-system history records deadlocks, latency spikes, and
  unresponsiveness caused by executing unrelated work while waiting. P5 admits
  a complete acyclic dependency graph and never runs arbitrary callbacks:
  <https://dev.epicgames.com/documentation/unreal-engine/tasks-systems-in-unreal-engine>.
- TinyLFU separates cache admission from eviction and shows that cache policy
  quality depends on workload history and object distribution. P5 therefore
  emits bounded residency requests/lease evidence but does not select an
  eviction or frequency policy: <https://arxiv.org/abs/1512.00727>.

### Repairs and simplifications

1. **One resource per ticket.** A job that has I/O, CPU, GPU, and main-thread
   phases is a DAG of small tickets. A multi-resource cost vector would imply
   atomic reservation and head-of-line behavior that P5 has not designed.
2. **Budgets are inputs.** The reference scheduler consumes a versioned integer
   `BudgetEnvelope`; it does not convert frame time, temperature, FPS, or load
   into budgets. That controller belongs to a measured runtime adapter and
   otherwise risks an unstable feedback loop.
3. **Priority donation is derived and bounded.** A prerequisite inherits the
   most urgent blocked dependent's effective deadline/tier for dispatch only.
   The original ImportancePacket and ticket remain unchanged and the donation
   vanishes when the dependency is complete or cancelled.
4. **Fairness is separate from significance.** Continuity/age is removed from
   the shared signal vector. Bounded service debt belongs to dispatch, is reset
   by service, and can never manufacture threat or interaction significance.
5. **Fallback is a ticket reference.** The fallback must exist, use the same
   target/epoch/resource, cost strictly less, contain no fallback of its own,
   and remain inactive until the original is rejected, expires, or settles as
   cancelled. An unvalidated string is not degradation.
6. **Cancellation and freshness are output gates.** `requested`,
   `acknowledged`, and `settled` are distinct. Results from a cancelled or stale
   request epoch are traced and discarded even if the underlying work finishes.
7. **Cache policy remains open.** P5 may issue a bounded, expiring residency
   lease request. It cannot evict, mutate a descriptor, infer population, or
   claim a hit-rate improvement. Cache admission and eviction need real access
   traces at the runtime gate.
8. **No private universal scalar.** The test policy derives a tier from bounded
   monotone integer thresholds and reason flags. Consumer maps are monotone
   tier-to-fidelity tables. Final signal ontology, thresholds, and fidelity
   meanings remain later creative/runtime decisions.

### Whole-system preservation

| Boundary | Preserved rule |
|---|---|
| P4 hierarchy/history | Tickets reference descriptor fingerprints and bounded observation windows; they cannot create identity, append deltas, or claim total population. |
| Derived rules/semantics | Significance consumes declared evidence but never becomes causal world truth or semantic vocabulary. |
| Construction/assets | Expensive work is a typed request with fallback; scheduling never changes recipe or artifact identity. |
| Procedural animation | Consumers share one packet but retain independent monotone fidelity maps and declared interpolation/fallback behavior. |
| Reference Studio | Decision traces and failures are read-only evidence; no queue control, execution, or authority path is added. |
| Runtime adapter | Real budgets, threads, clocks, device queues, cache algorithms, and performance claims remain gated. |

### Revalidation result

The reference can proceed as `prototype_tested` with strict integer data,
bounded in-memory simulation, deterministic traces, and negative authority. It
must stop before a budget controller, cache algorithm, async executor, product
weights, engine objects, or production timing claims. This avoids permanent
quality loss: player-facing mappings and measured optimization remain open,
while the failure contracts needed to compare them become executable.
