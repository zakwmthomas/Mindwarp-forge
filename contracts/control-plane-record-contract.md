# Control Plane Record Contract v0.1

The control plane persists immutable, data-only `WorkPackage`, `GateReceipt`,
`BlockerRecord`, and `RollbackRecord` values. These records make lifecycle
history inspectable; none of them approves, promotes, applies, executes, or
grants authority.

## Lifecycle rules

- A work package fixes its initial stage, dependencies, risk, evidence and
  verification requirements, authority lane, and next action.
- A passed gate advances exactly one stage and carries evidence with no failure
  reason. Failed or blocked gates remain at the current stage and retain a
  non-empty failure reason.
- The current stage is reconstructed from append order. A stale retry, skipped
  stage, unknown package, empty evidence list, or conflicting immutable ID
  fails closed.
- A blocker names an existing package, affected stage, requirement, evidence,
  and open/resolved state. Recording it does not guess or satisfy the missing
  dependency, decision, verification, or authority.
- A rollback references an existing failed or blocked gate for the same work
  package and retains the previous standard, affected artifact, restore
  evidence, reason, and follow-up. It cannot erase the failed candidate.

## Required proof

Fixtures must cover ordered lifecycle replay, stage skipping, stale/repeated
transitions, immutable-ID conflicts, unknown references, failed gate
visibility, blocker persistence, rollback from a failed gate, rejection of
rollback from a passed gate, reopen/replay, and hostile authority language
remaining inert in the Forge kernel.
