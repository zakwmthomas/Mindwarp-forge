# Worker <-> Forge Integration Plan

## Current truth

The worker governance system is durable repository knowledge and the automation
reads it. It is **not yet** a first-class Kernel ledger object or a visible
desktop dashboard projection. Do not claim bidirectional integration until the
phases below are verified.

## Target loop

`Forge state/evidence -> worker navigation and batch selection -> worker action
-> test/verification receipts -> learning observation -> governance change ->
Forge validation/projection -> next worker batch`

## Phased implementation

| Phase | Deliverable | Verification | Authority boundary |
|---|---|---|---|
| I1 | Canonical governance records, learning ledger, batch state, and validation tool | Required records, policy references, batch/ledger schema checks | Files are policy/evidence; no authority grant |
| I2 | Deterministic governance snapshot generated from records | Stable snapshot and change detection tests | Read-only projection only |
| I3 | Kernel-linked evidence references for approved governance changes | Replay/provenance and authority-negative tests | No policy change from captured text |
| I4 | Desktop read-only governance dashboard | Mutation-negative UI tests | No edit/approve/promote controls |
| I5 | Worker effectiveness metrics and three-batch audit projection | Metrics/threshold provenance and regression tests | Metrics advise; they do not self-authorize |

## I1 closure batch

1. Define a machine-readable governance manifest that names the canonical
   prompt, protocol, governance system, learning ledger, policy IDs, batch
   state, closure register, and integration plan.
2. Add a verifier that fails closed on missing records, malformed manifest,
   missing policies, or a worker prompt that is not linked to the canonical
   specification.
3. Add the verifier to the full Forge gate.
4. Record the result in the active batch state and learning ledger.

## Required future safeguards

- Governance changes need provenance, rationale, expected benefit, and
  regression signal before promotion into a universal policy.
- Worker metrics must distinguish planned work, partial work, verified work,
  blocked work, and narration; they must never reward superficial activity.
- The worker may propose improvements but cannot approve its own authority,
  security, spending, publishing, engine, or code-promotion changes.
