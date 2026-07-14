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

## C. Canonical game foundations

| ID | Fix | Closure gate | State |
|---|---|---|---|
| C1 | ProofReceipt storage binding | Owner decision plus contract/recovery tests | Owner-gated |
| C2 | Universe identity policy | Fixed-vector/migration/collision tests | Design-gated |
| C3 | Field numerical policy | Determinism/range/cache/poison tests | Depends on C2 |
| C4 | Hierarchy/history semantics | Residency/delta/migration/corruption tests | Depends on C2-C3 |
| C5 | Significance/scheduler | Pressure/cancellation/starvation/thrash tests | Depends on C4 |
| C6 | Semantic/construction | Causal/diversity/replay/validator tests | Depends on C4 |
| C7 | Representation/assets/animation | Structural/temporal/perception/repair tests | Depends on C5-C6 |

## Operating rule

Work from A1 downward by dependency and risk. Reassess the whole register,
then its affected group, then the individual contract before every advance.
Do not activate F5 until A1-A4/B1-B3 are closed or explicitly owner-gated.
