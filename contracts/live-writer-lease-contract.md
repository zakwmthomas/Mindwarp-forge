# Live writer lease contract v1

Status: owner-authorized bounded Forge coordination component.

## Purpose and boundary

Every Mindwarp Forge session is read-only by default. Repository mutation
requires one unexpired project-wide workstream lease held by the exact
`CODEX_THREAD_ID`, routed through the registered Mindwarp Forge project and
`forge-live-mainline` workstream, and bound to the current canonical Worker
Batch State SHA-256.

The lease coordinates writers only. It does not approve work, select a master
program item, grant owner authority, make captured chat authoritative, promote
code, publish, spend, weaken security, or mutate the live database merely
because a test passes.

## Invariants

- One project-wide workstream coordinates every mutation in the shared Forge
  worktree, even when agents research different packages or modules.
- A session route grants no write authority. Only the matching live lease does.
- A lease lasts at most 1,800 seconds and must be reasserted before each
  material mutation or long verification run.
- The claim records exactly one current checkpoint SHA-256. Checkpoint drift
  invalidates the claim and requires a fresh successor revision.
- A second holder, an unrouted session, an expired lease, an inactive
  workstream, a stale revision, a missing thread ID, or a mismatched project,
  workstream, or checkpoint fails before mutation.
- Release expires the current holder's lease. Another routed session may claim
  only afterward or after natural expiry.
- Greenfield and other repositories retain independent project/workstream
  leases; their writers do not consume the Forge writer lease.
- Live database route, claim, and release operations require the explicit
  `-AllowLiveDatabaseMutation` switch and remain separately gated until source,
  disposable, focused, and complete Forge verification pass.

## Verification and rollback

Disposable fixtures must prove routed claim, idempotent routing, sequential and
simultaneous second-writer rejection, read-only assertion failure, checkpoint drift, bounded TTL,
release/takeover, missing-route rejection, and complete Forge Kernel recovery.

Rollback removes the wrapper, focused verifier, CLI verbs, and writer-claim
helpers while retaining prior federated project, workstream, route, evidence,
and checkpoint history. It never deletes the live database or captured source.
