# Control Plane Lifecycle and Owner-Brief: Readiness Package

**Status:** B2 verified. Immutable work-package, gate, blocker, and rollback
records plus focused adversarial fixtures and the full Forge gate pass. This
package does not grant authority, approve/promote candidates, or alter the Atlas.

## Boundary to establish

| Record | Required role | Must not imply |
|---|---|---|
| `WorkPackage` | Scope, dependencies, risk, evidence, verification, authority lane, next action | Approval to cross a missing gate |
| `GateReceipt` | Passed/failed/blocked stage, evidence, failure reason, rollback target | Automatic promotion |
| `BlockerRecord` | Missing authority/dependency/decision and affected work | Silent retry, hidden work, or a guessed decision |
| `RollbackRecord` | Previous standard, affected artifact, restore evidence, follow-up | Deletion of historical evidence |
| `OwnerBriefItem` | At most five material batched decisions with options/consequences/evidence | Inclusion of routine automatic work |
| `ImmediateItem` | Spending, credentials, publishing, security, deletion, or other immediate authority need | Hiding behind the batched brief limit |

## Invariants and adversarial fixtures

- Lifecycle cannot skip stages; failed or blocked gates remain visible.
- Dependencies, authority lanes, and verification evidence are checked before
  advancement; stale or unrelated evidence is rejected.
- Automatic work is logged but cannot self-promote into owner decisions.
- Owner brief ordering is risk-based and bounded; immediate items remain
  separately visible.
- Rollback retains prior standard, evidence, and reason; recovery cannot erase
  a failed candidate.
- Fixtures cover missing dependency, forged/stale gate receipt, blocked item,
  invalid authority lane, overflowed decision queue, immediate-item visibility,
  failed verification, rollback, and repeated transition attempts.

## Readiness gaps deliberately left open

The record model does not select risk scoring, escalation thresholds, rollback
retention, owner-brief wording, or automatic-monitoring cadence. Those are
operating-policy choices that must remain explicit and reviewable.

## Entry criteria

- WorkPackage/GateReceipt/Blocker/Rollback schemas and validator tests exist.
- All adversarial fixtures prove no lifecycle or authority bypass.
- Owner Brief is read-only and linked to exact evidence; it cannot perform an
  approval, promotion, or application action.

## Implemented B2 slice

- SQLite stores immutable work packages and append-ordered gate, blocker, and
  rollback records; reopen reproduces the same lifecycle timeline.
- Passed gates advance exactly one stage. Failed/blocked gates stay at the
  current stage with a retained reason. Skips, stale retries, unknown package
  references, empty evidence, and conflicting immutable IDs fail closed.
- Rollback requires a failed or blocked gate for the same work package and
  retains the failed history plus exact restore evidence.
- Kernel verification passes 56 tests, including authority-negative
  fixtures proving hostile approval/promotion text changes neither kernel
  events nor candidate state.
- Owner Brief remains the existing bounded read-only projection. The full Forge
  gate passes with 12 desktop and 56 kernel tests; B3 is now active.
