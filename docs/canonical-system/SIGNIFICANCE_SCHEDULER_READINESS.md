# Significance and Streaming Scheduler: Readiness Package

**Status:** discovery and readiness only. This package does not schedule a game
frame, start a runtime, or claim engine/hardware performance.

## Purpose

Mind Warp requires one shared significance model for rendering, simulation, AI,
physics, animation, audio, streaming, cache, and generation. The scheduler then
turns declared demand into bounded work under CPU/GPU/I/O/main-thread budgets.
The two systems are paired because a scheduler cannot correct incompatible
private priority models created by its consumers.

## Boundary to establish

| Record | Required role | Must not contain |
|---|---|---|
| `ImportancePacket` | Versioned, explainable demand signals and hysteresis state | Direct engine component references or authority to mutate canon |
| `ConsumerRequest` | Consumer, work class, required fidelity, deadline, and fallback | Private substitute significance score |
| `BudgetEnvelope` | Named CPU/GPU/I/O/main-thread/cache budgets and measurement labels | One undifferentiated global budget |
| `WorkTicket` | Cancellable, deduplicated, traceable unit of scheduled work | Unbounded execution capability |
| `SchedulerDecision` | Admit/defer/cancel/pin/fallback result with reason codes | Hidden priority changes or policy authority |
| `PressureTrace` | Reproducible timeline of focus, threat, route, workload, and outcomes | Claim of runtime profiling without measured environment |

## Core invariants

- Every consumer derives priority from the same versioned ImportancePacket; a
  private LOD score is an integration failure unless explicitly mapped and
  compared.
- Hysteresis is explicit: small signal fluctuation cannot repeatedly admit and
  cancel the same work.
- Threat/interaction protection can reserve resources, but reservations are
  visible and bounded so they cannot silently starve unrelated safety work.
- Every ticket has a deadline, cancellation condition, fallback, and owner
  budget classification before it is admitted.
- Route/focus changes invalidate stale work predictably; cancellation and
  cache pinning are observable rather than accidental side effects.
- A simulated result is labelled simulated. No engine-specific FPS, memory, or
  GPU conclusion is inferred before the final runtime-adapter phase.

## Paired fixture matrix

| Fixture | Required observation |
|---|---|
| Stable focus | Deterministic admission order and bounded steady-state residency |
| Oscillating focus | Hysteresis prevents admission/cancellation thrash |
| Route reversal | Stale prefetch cancels; useful pinned work is retained only by declared policy |
| Combat/interaction spike | Protected work receives its declared reservation with visible displaced work/fallbacks |
| Competing consumers | Shared packet produces explainable cross-consumer fidelity differences |
| Starvation pressure | Deferred work either progresses within stated policy or emits a starvation diagnosis |
| Deadline overload | Tickets degrade/fallback/cancel by declared rule rather than silently miss indefinitely |
| Cache thrash | Pin/admission/bypass decisions are traced and canonical descriptors remain unchanged |
| Poison ticket | Unknown consumer, invalid budget, missing fallback, impossible deadline, duplicate ID, and stale version fail visibly |

## Observability requirements

The Reference Studio inspector must be able to show a sampled PressureTrace,
BudgetEnvelope, packet version, ticket decision reason, fallback, cancellation
cause, residency effect, and measurement label. It may not manipulate the
queue, inspect private runtime memory, or convert a scheduler result into
authority over generation/history state.

## Neighbour contracts

| Neighbour | Provides | Receives |
|---|---|---|
| Hierarchy/history | Descriptor metadata and explicit observation windows | No permission to materialise or mutate canonical state |
| Asset/animation/AI/audio/physics | ConsumerRequest and declared fidelity/fallback vocabulary | Shared packet/decision only, not private global priority |
| Streaming/cache | Budgeted tickets, cancellation, pins, and fallback actions | Disposable residency telemetry |
| ProofReceipt | Fixture/trace inputs, outputs, measurements, warnings | No pass/fail authority into scheduling |

## Readiness gaps deliberately left open

The evidence does not select the importance dimensions, weights, hysteresis
function, consumer fidelity vocabulary, budget units, deadline policy,
reservation policy, cache admission policy, or simulation model. These are
interdependent player-experience and cost choices; selecting them automatically
would be a hidden product-direction decision.

## Entry criteria for a future implementation package

- Identity/hierarchy descriptors and observation-window semantics are settled.
- A decision record defines the shared packet, units, hysteresis, and budget
  semantics together with a simulated-versus-measured policy.
- Paired fixture matrix covers combat, route change, starvation, cancellation,
  deadline, and cache-thrash before any reference-proof claim.
- Scheduler module remains engine-neutral and receives work only through typed
  tickets, never arbitrary code or mutable-world authority.
