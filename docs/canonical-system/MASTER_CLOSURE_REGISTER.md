# Master Closure Register

This is the single queue for substantive fixes. An item is not closed by a plan;
it needs retained evidence and its stated test gate.

## A. Trust, continuity, and recovery

| ID | Fix | Closure gate | State |
|---|---|---|---|
| A1 | Source completeness/format drift | Long corpus, gap/reorder/migration receipts and tests | Verified: versioned envelopes retain raw bytes/hash and parsed child evidence; conflict, ordering, legacy coexistence, replay, and authority-negative tests pass |
| A2 | Durable source manifest/gap history | Persistence/replay/idempotency tests | Verified: append-only deterministic projections, manifest-version isolation, equal-retry idempotency, interrupted-history repair, replay, and authority-negative tests pass the full Forge gate |
| A3 | Controlled application hostile-path boundary | Path, symlink, env, network, process, crash, rollback tests | Verified: traversal/absolute and existing targets fail closed; live junction ancestors are rejected; hostile env/network/process text is never executed; pre/post-rename failures clean artifacts; new-file rollback and replay pass the full Forge gate |
| A4 | Backup/recovery drill | Corrupt/partial restore and fixity tests | Verified: retained receipts recheck byte count, SHA-256, reopen/replay, and object/event/candidate counts; altered, truncated, and wrong-count artifacts fail closed without mutating the live Forge |

## B. Evidence and orchestration

| ID | Fix | Closure gate | State |
|---|---|---|---|
| B1 | Research source/claim/contradiction records | Traceability, contradiction, cache, authority-negative tests | Verified: immutable source/claim/contradiction records preserve exact spans, freshness/availability, confidence, limitations, scope mismatches, unresolved questions, replay, idempotent cache reuse, and hostile-text authority isolation |
| B2 | Control-plane gate/blocker/rollback records | Lifecycle/authority/rollback adversarial tests | Verified: immutable append-ordered work-package, gate, blocker, and rollback records reject stage skips, stale retries, forged rollback, conflicting IDs, and authority escalation; reopen/replay and the full Forge gate pass |
| B3 | Read-only Reference Studio proof inspector | Mutation-negative, source-gap, version/failure view tests | Verified: schema-versioned verified-local projection exposes empty, failure, blocker, rollback, source-gap, and version-mismatch states; hostile text is inert, full record/kernel counts remain unchanged, UI has refresh only, and the full Forge gate passes |
| B4 | Worker telemetry and efficiency module | Append-only traced events, metric registry, recomputable projections, privacy/cardinality/Goodhart guards, read-only trends | Complete; v1 SQLite events, bounded registered metrics, deterministic advisory projection, replay/privacy/cardinality/sample/Goodhart fixtures, and full gate pass |
| B5 | Federated Universal Improvement Kernel | Shared observation/hypothesis/experiment/result/rollback protocol; replay, local isolation, aggregate-masking, transfer-isolation, outage, and authority-negative tests | Complete; append-only local experiment/result/decision and transfer-gate records pass replay, isolation, semantic mismatch, aggregate masking, rollback, outage, schema, and authority-negative fixtures |
| B6 | Federated project routing, indexed retrieval and lossless storage V1 | Project/workstream/session isolation, V4 preservation and indexed query, verified archive replay, managed inventory and preview-only cleanup tests | Complete: revisioned projects/workstreams, closed session routing, normalized V4 references, FTS5 filters/ranking, generation receipts, verified archives, Git-bound inventory, compact bootstrap and approval-negative cleanup plans pass 102 Kernel tests and the complete Forge gate; the owner accepted the independently reproduced V4 receipt as the live search baseline while V1-V3 retention and cleanup/deletion shields remain enforced |

## C. Canonical game foundations

| ID | Fix | Closure gate | State |
|---|---|---|---|
| C1 | ProofReceipt storage binding and exact H7 dependency consumption | F5 provenance comparison, contract/recovery evidence, exact/generic/stale/superseded consumer tests | Verified: inherited owner gate consolidated into the implemented versioned projection; typed read-only H7 consumer fails closed and retains all non-claims |
| C2 | Universe identity policy | Fixed-vector/migration/collision tests | Verified: approved logical/reconstruction split retained; seven focused tests and complete Forge gate pass; cross-platform and performance limits remain explicit |
| C3 | Field numerical policy and causal derived-world contract | Determinism/range/cache/poison/causality/candidate-baseline tests | Executing. The residual-obligation audit confirms physical applicability remains evidence-blocked. The owner-approved result proves the selected evidence-preserving typed-boundary witness through one disposable independent ecotone oracle twice byte-identically: exact integer/Fraction arithmetic, separate semantic/audit digests, dimension-local outcomes, aligned refinement, seven real cell/edge enumeration modes, nineteen executable hostile families and the 65,536-cell/130,560-edge ceilings survive isolated stdlib proof. This is proof-tool evidence only. No canonical 2D material-interface join, blend scale, rendered-seam claim, schema, crate, dependency, production source, downstream consumer, runtime, promotion or C3 closure is authorized. |
| C4 | Hierarchy/history semantics | Residency/delta/migration/corruption tests | Dependency-gated by C3: bounded addressable-world and lifecycle/history components retained; multi-batch replay and hostile stored-value defects repaired; full C4 closure remains open |
| C5 | Significance/scheduler | Pressure/cancellation/starvation/thrash tests | Dependency-gated by C4: scheduler reference and four-domain fidelity fixture retained; full named-domain dispatch/budget proof remains open |
| C6 | Semantic/construction and organism ecology | Causal/diversity/replay/validator tests | Dependency-gated by C4-C5: repaired prototypes now reject forged causal inputs and report person-form structural bindings only; full niches, body plans, physiology, species/ecomorphs, comparison, dimorphism and aesthetic grammar remain open |
| C7 | Representation/assets/animation | Structural/temporal/perception/repair tests | Depends on C5-C6 |
| G1-CLOSEOUT | Whole-chain promotion readiness | Replay/integration/recovery/quality-diversity/visual/simulated-fidelity proof | Depends on C3-C7 |

## Operating rule

Work from A1 downward by dependency and risk. Reassess the whole register,
then its affected group, then the individual contract before every advance.
Do not activate F5 until A1-A4/B1-B3 are closed or explicitly owner-gated.
# 2026-07-18 product and C3 dependency rebaseline

- C3A is the promoted dependency-sufficient seam: validated `WorldGenerationInput` -> replayed `CausalWorldPacket` v1 with nested identity and provenance.
- C3B remains evidence-blocked on physical scale, coefficients, applicability, visibility and presentation fidelity; it is not full C3 closure and does not block GP0-GP4.
- C4 depends on C2 and C3A only. GP0 is structurally verified without gameplay-system promotion; GP1 is independently and completely verified as a capability-free fixed-loop proof and remains the sole active owner-review checkpoint; GP2-GP4 remain dependency-gated.
- Yard Atlas is evidence-linked but independent; no repository, database, UI, authentication, billing or release authority transfers.
