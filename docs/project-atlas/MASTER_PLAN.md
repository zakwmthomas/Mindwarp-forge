# Forge Master Plan

> GENERATED from `docs/canonical-system/MASTER_PROGRAM.json`. Do not edit this view directly.

Canonical lifecycle: schema v2; executing item: `F5`.
Planning doctrine: `docs/canonical-system/MASTER_PLAN_V2.md`.

## F4

- **F4-MODULARITY** - promoted; gate: hard; depends on: none.
  Next: Maintain the declared module graph and static boundary fixtures.
- **W1** - promoted; gate: hard; depends on: F4-MODULARITY.
  Next: Build and run the worker proof harness before further autonomous package rollout.
- **W2** - promoted; gate: hard; depends on: W1.
  Next: Implement owner-notification routing for actionable Forge problems.
- **A1** - promoted; gate: hard; depends on: F4-MODULARITY, W1.
  Next: Implement versioned SourceChunkEnvelope persistence and replay.
- **A2** - promoted; gate: hard; depends on: A1.
  Next: Persist source manifest/gap history.
- **A3** - promoted; gate: hard; depends on: F4-MODULARITY.
  Next: Build hostile controlled-application fixture pack.
- **A4** - promoted; gate: hard; depends on: A3.
  Next: Run backup/recovery corruption drill.
- **B4** - promoted; gate: hard; depends on: F4-MODULARITY.
  Next: Maintain append-only BatchEvent and advisory projection fixtures.
- **B5** - promoted; gate: hard; depends on: B4.
  Next: Maintain federated local-isolation and transfer-gate fixtures.
- **F4-CLOSEOUT** - promoted; gate: hard; depends on: F4-MODULARITY, W1, W2, A1, A2, A3, A4, B1, B2, B3, B4, B5.
  Next: Maintain the mechanical F4 exit audit and owner-gate evidence.
- **F5-OWNER-GATE** - promoted; gate: owner; depends on: F4-CLOSEOUT.
  Next: Retain the owner-delegated versioned-projection decision and its narrow authority boundary.
- **B1** - promoted; gate: hard; depends on: A1.
  Next: Implement research source/claim/contradiction records.
- **B2** - promoted; gate: hard; depends on: B1.
  Next: Persist control-plane gate/blocker/rollback receipts.
- **B3** - promoted; gate: hard; depends on: B2.
  Next: Build read-only Reference Studio proof inspector.

## F5

- **F5** - executing; gate: hard; depends on: F5-OWNER-GATE.
  Next: Execute H4 functional-control calibration: bind the verified H3 candidate to the existing broken-connection, silhouette-collapse, and articulation-drift controls; derive the smallest orthogonal integer metrics in memory; record exactly which control each metric distinguishes and cannot distinguish; and reject stale fingerprints, missing controls, cross-sensitive claims, post-hoc thresholds, visual-quality inference, and authority escalation before any H5 observation.
- **F5-COHERENCE** - promoted; gate: hard; depends on: F4-CLOSEOUT.
  Next: Implement the owner-approved typed knowledge intake, canonical role registry, recoverability baseline, unified snapshot, and generated knowledge views without crossing the independent F5 owner-observation gate.

## G1

- **C1** - ready_for_owner; gate: owner; depends on: F5.
  Next: Choose ProofReceipt storage boundary.
- **C2** - designed; gate: design; depends on: C1.
  Next: Resolve universe identity policy.
- **C3-C7** - proposed; gate: design; depends on: C2.
  Next: Advance canonical modules bottom-up with local proof packs.

## R1

- **R1** - proposed; gate: owner; depends on: C3-C7.
  Next: Prepare runtime-adapter selection package.
