# Macro Gap-Closure Audit

**Status:** Phase F4 audit record. This is a planning and readiness document;
it does not promote a system or authorize runtime work.

## Method

For each canonical system, this audit asks four questions before a proof harness
is created:

1. What is the stable input/output boundary shared with its direct neighbours?
2. Which invariant would make a locally passing implementation unsafe or
   useless to the rest of the system?
3. What evidence and inspection must exist for a solo owner to detect failure?
4. What is the smallest engine-neutral proof that retires the highest-risk
   unknown without committing to a runtime engine?

`Missing` below means the requirement is not yet recorded as a reference proof;
it does **not** mean that an older prototype never explored it.

## Cross-cutting closure rules

- Every canonical artifact needs a versioned identity, deterministic input
  record, provenance/evidence link, validation result, and explicit disposal or
  recovery story. Cached bytes alone are never canonical.
- Cross-system values must use named, versioned contracts. No downstream module
  may infer semantics from an incidental field layout or engine object.
- Every proof harness must emit a compact receipt containing input seed or
  fixture ID, generator/contract version, output hash or semantic equivalence,
  cost measurements, and failure classification.
- The Reference Studio must be able to inspect a proof receipt and its input
  artifact without executing arbitrary generated code.
- Each promotion boundary requires a negative test: malformed input, version
  drift, incompatible neighbour contract, and recovery/fallback behavior.

## Per-system closure register

| System | Neighbour contract to freeze | Highest-risk gap | Required observability | Smallest proof before advancement |
|---|---|---|---|---|
| Forge truth kernel | Versioned evidence/event/candidate envelope | Recovery may preserve bytes but lose authority or ordering meaning | Replay receipt, event graph, rejected-transition reason | Replay and corruption fixture that proves forged authority cannot survive recovery |
| Forge context compiler | Source manifest and message-order/correction envelope | Long corpus, duplicate, truncation, and source-gap behavior is not reference-proven | Source health and gap receipt linked to exact source spans | Versioned long-corpus fixture with replay, dedupe, correction, and incomplete-source cases |
| Forge research | ResearchBrief, Claim, Source, Contradiction, and Experiment records | Unbounded search or untraceable summaries could become pseudo-authority | Claim-to-source graph, freshness/cache status, contradiction receipt | Offline fixture proving citation traceability, conflicting sources, retry/cache, and source-gap output |
| Forge control plane | WorkPackage lifecycle and authority-lane transitions | A valid artifact could bypass readiness or leave no rollback route | State machine timeline and failed-gate reason | Lifecycle matrix exercising dependency failure, authority refusal, rollback, and owner-brief projection |
| Forge Reference Studio | Read-only Inspector/ProofReceipt projection | Visual dashboard could become a hidden control path or omit decisive evidence | Inspector shows seed, version, evidence, cost, failure, and state | Render fixture for one proof receipt plus a negative case proving UI cannot mutate authority |
| Universe identity | Address grammar, seed derivation, generator version, random-stream partition | Cross-platform/version drift could change an address's meaning | Address reconstruction trace and identity collision report | Fixed vectors for address reconstruction, stream partitioning, collision, and version migration |
| Field basis | FieldPacket schema, numeric policy, transform/composition rules | Numeric drift or hidden global state breaks repeatability and cache validity | Packet inspector, range/frequency report, output fingerprint | Fixed-vector composition, poison-input, cache-key, and cross-domain determinism tests |
| Derived world rules | FieldPacket-to-WorldConditions schema and allowed ranges | Attractive outputs may violate causal or physical constraints | Causal chain and range-violation report | Cross-seed and regional fixtures proving bounded causal consistency and variation |
| Lazy universe hierarchy | Addressable descriptor versus materialised-residency contract | Eager expansion, unstable descriptor identity, or cache state leaking into canon | Residency/eviction trace and materialisation counters | Observation-window fixture proving deterministic descriptor output and bounded residency |
| World history ledger | Baseline descriptor plus sparse DeltaEnvelope and migration policy | Generator upgrade or corrupt delta could silently rewrite player/world history | Save lineage, delta replay, collision, and recovery receipt | Save/reload, migration, sparse-delta, collision, and corruption-recovery fixture |
| Significance system | Shared ImportancePacket and hysteresis rules | Private LOD/priority decisions could conflict across render, AI, physics, and streaming | Per-consumer reason codes, focus/threat trace, starvation report | Focus, combat, hysteresis, and independent-fidelity tests on one shared packet |
| Streaming scheduler | Work request, deadline, cancellation, pin, fallback, and budget telemetry contract | Local scheduling wins may cause frame spikes or cache thrash globally | Queue timeline, budget ledger, cancellation/fallback cause | Route-change, combat-pressure, deadline, cancellation, and thrash simulation |
| Semantic emergence | Pressure-to-role-to-capability causal graph with alternatives | Word association could masquerade as causality or erase design diversity | Causal explanation, rejected alternatives, contradiction/poison report | Poison-word, synonym, contradiction, diversity, and explanation fixtures |
| Construction language | Typed part-role graph, sockets, transforms, material regions, validators | Recipes may be unreplayable or physically/interface-invalid | Recipe graph inspector and validator failure locations | Recipe replay plus connectivity, support, collision, and socket-contract fixtures |
| Representation selector | Functional requirement and cost profile to representation decision record | Selection could hide cost/fidelity assumptions or depend on runtime details | Decision rationale, compared alternatives, fidelity/cost report | Category matrix comparing deformation, recombination, sharpness, and cost without engine objects |
| Asset factory | ArtifactManifest containing recipe, representation, materials, LODs, validation, and review images | Generated output could be visually unreviewable, non-replayable, or not repairable | Artifact/LOD inspector and structured visual-review receipt | One end-to-end neutral prop fixture before category expansion; retain failed repair candidate |
| Procedural animation | Articulation/deformation/contact request and temporal-cadence contract | Motion may be stable numerically but fail contact, purpose, or recognisability | Contact/error trace, cadence/LOD report, visual proof receipt | Neutral articulated fixture exercising contact, interpolation, topology, and temporal LOD |
| Runtime adapter | Runtime-neutral promoted ArtifactManifest import boundary | Early engine objects could contaminate canonical state | Clean-project import receipt and identifier preservation report | Gated until all adapter dependencies are reference-proven and an owner approves a scored runtime decision |

## Dependency-level findings

1. **The first reusable contract is not an asset contract.** It is a
   deterministic `ProofReceipt` envelope, because every future proof needs the
   same provenance, version, cost, and failure routing.
2. **Universe identity and field basis are the first game-canonical proof pair.**
   They establish deterministic addressing and deterministic values without
   requiring a renderer, asset pipeline, or engine choice.
3. **The Reference Studio can begin as a read-only receipt inspector.** It must
   not wait for 3D generation, and it must not acquire control-plane authority.
4. **Semantic, construction, representation, asset, and animation work must
   share versioned recipe artifacts.** Otherwise each layer will invent an
   incompatible notion of identity, repair, LOD, and provenance.
5. **Significance and scheduling must be proved together at their boundary.**
   A scheduler-only proof cannot expose incompatible consumer priority rules.
6. **History must be paired with hierarchy before mutable gameplay evidence is
   trusted.** A delta without a stable reconstructable baseline is not
   recoverable.

## Ordered F5 candidate packages (not yet authorized for implementation)

| Order | Candidate package | Why it comes next | Required inputs | Exit evidence |
|---:|---|---|---|---|
| 1 | ProofReceipt and read-only inspector contract | Reusable evidence/observability foundation for every later proof | Truth kernel, control plane, Reference Studio boundary | Receipt schema, validator, fixture, inspector projection, mutation-negative test |
| 2 | Universe identity fixed-vector harness | Lowest engine-neutral generator root | Address/seed/version design evidence | Reconstructable addresses, streams, collision/migration fixtures |
| 3 | Field basis fixed-vector harness | Builds directly on stable identity and validates cacheable field recipes | Identity vectors, field prototype evidence | Composition/determinism/range/cache/poison fixtures |
| 4 | Hierarchy and history paired harness | Establishes lazy materialisation and mutation recovery | Identity, field/world conditions, kernel | Residency/eviction and sparse-delta/migration/corruption receipts |
| 5 | Significance and scheduler paired simulation | Shared performance semantics before visual/game work | Hierarchy descriptors and shared ImportancePacket | Pressure, cancellation, starvation, deadline, and thrash traces |
| 6 | Semantic-to-construction recipe proof | Prevents semantic/geometry identity drift | World conditions/history and causal graph schema | Causal, diversity, replay, socket, support, collision fixtures |
| 7 | Representation, asset, and animation neutral atlas | Makes output inspectable before runtime selection | Valid recipes, selector decision record, Reference Studio | One category at a time, visual/cost/contact/LOD proof packs |

## F4 readiness result

The canonical registry now has a per-system boundary, risk, observability
requirement, and smallest proof path. F4 can be considered *audit-complete*
once this document is validated as part of the canonical-system gate and the
owner accepts the first bounded F5 package. It does not make F5 implementation
active by itself.
