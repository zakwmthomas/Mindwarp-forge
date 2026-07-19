# G1 C4 age/lifecycle, recovery and ProofReceipt result

Status: **bounded lifecycle components proven at in-memory-fixture tier; C4 is not closed out.**

## 2026-07-15 adversarial repair

The original history adapter reset delta sequence and command identity on every
`drive` call, so a second ordinary update batch failed with `CommandConflict`.
It also cast stored signed values directly to `u16`. The repaired adapter
continues the existing stream sequence, omits reload and zero-change deltas,
validates stored mode/progress/lock values before conversion, and rejects
ambient appearance-lock state. Multi-batch continuation, hostile stored values,
zero-change suppression, public-state validation, and ambient-lock rejection
are retained as regression tests.

## What this closes

C4's next action named four remaining parts. This result closes the last
three:

1. **Stable age cohorts and selected-entity lifecycle deltas** —
   `crates/entity-lifecycle` implements
   `SELECTIVE_LIVING_ENTITY_AGING_DESIGN.md` steps 1-3 of its own "cheap
   proof plan": strict bounded `AgeCohort`/`LifecycleMode`/
   `LifecycleState` records using permille fixed-point progress only, a
   pure state-transition table (`apply`), and a presentation projection
   (`present`). 11 focused tests pass: deterministic replay, monotonic
   progress, juveniles unaffected by the appearance lock, adult
   presentation clamping while locked, hidden elder progress revealed
   exactly (no rejuvenation) after unlock, ambient entities never ticking,
   adoption preserving cohort/progress without reroll, elder progress
   requiring completed maturity, saturation at the permille bound, reload
   as a true no-op, and closed-failure overflow.
2. **Recovery without continuous ambient simulation** —
   `crates/entity-lifecycle-history-binding` maps lifecycle-state-changing
   events onto `hierarchy-history`'s already-proven generic
   `ReferenceOperation`/`HistoryStream` delta and replay machinery (no new
   operation schema, no reducer change). 6 focused tests pass: storage
   round-trip, reload suppression, gap rejection, multi-batch continuation,
   hostile stored-value rejection, and zero-change suppression. The storage
   proof includes a full encode-decode round-trip of every delta and
   reconstructs the exact
   same `LifecycleState` by pure replay (never re-running
   `entity_lifecycle::apply`); `Reload` produces no delta and never
   pollutes the ledger; and a deliberately gapped lifecycle delta is
   rejected by hierarchy-history's existing, unmodified `Gap` check. This
   is the direct evidence that a lifecycle entity's exact state can be
   recovered purely by replaying stored deltas rather than simulating
   forward from any clock.
3. **Read-only ProofReceipt integration for the addressable-world-binding
   seam** — `forge-desktop`'s
   `addressable_world_binding_vector_persists_as_read_only_proof_receipt`
   test (mirroring the existing derived-world-rules and hierarchy-history
   fixtures) binds a real `CausalWorldPacket` into a `HierarchyDescriptor`,
   persists both as plain Kernel objects, and records a `ProofReceiptRecord`
   that carries forward the addressable-world-binding's own recorded
   provenance-sensitive-fingerprint limitation. Kernel object/event/candidate
   counts are asserted unchanged by the receipt recording step itself.

## Explicit non-claims

- No population sampler, `PresentationProfile` visual curves, mesh/shader
  work, or "one humanoid plus one structurally different creature" visual
  comparison (item 4 of the design document's cheap proof plan) is
  implemented. That remains open and requires visual assets, which are out
  of scope for this typed-model/replay-tier proof.
- No mortality path exists anywhere in `entity-lifecycle`; this is a type-
  level guarantee (`PresentedStage`/`LifecycleState` have no such variant),
  not a tested behavior.
- `entity-lifecycle-history-binding`'s key mapping (mode/maturity/elder/
  appearance-lock to four fixed `u16` keys) is a disposable reference
  mapping for this proof; it is not proposed as the production save-record
  schema.
- The addressable-world-binding provenance-sensitive-fingerprint limitation
  recorded in `G1_C4_ADDRESSABLE_WORLD_BINDING_RESULT.md` is unchanged and
  is now also carried into its ProofReceipt's `limitations` field rather
  than only living in a markdown result document.

## Remaining work and corrected ownership

- C4 still owns canonical lifecycle state plus its addressable history. Broad
  hierarchy/history proof remains open for stable dynamic/presence evidence,
  exact dependency availability, corrupt-tail recovery, bounded migration
  chains, deterministic snapshot/log/replay cost curves and independent
  second-platform receipts.
- Population distributions and species-authored `PresentationProfile`
  semantics belong to C6, which consumes C4's stable lifecycle state.
- Rendered two-creature and phone-legible comparison belongs to C7 and its
  bounded visual-observation gate.
- A physical-only, provenance-independent world-conditions fingerprint is an
  optional consumer-driven gap, not a current C4 closure requirement.

The owner chose to continue the canonical broad route. C4 is active; this
result remains a bounded input and is not itself a C4 closure receipt.
