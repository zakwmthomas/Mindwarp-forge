# G1 C4V vertical persistence result

Status: prototype tested, additive and capability-free.

Recorded closure: registered complete gate
`run-fa6334a300e04d409dd5cddb4f22542e` passed in 511,311 ms. Earlier
fail-closed governance receipts `run-396dae5319cb45fab6b1c136f3dea843`
and `run-43674553ca304e1b898c85449414833f` are retained. GP3 remains queued
and was not started.

## Implemented slice

`mindwarp-vertical-persistence` is the isolated C4V sibling selected by the
master program. It composes C2 canonical universe addresses, the C4
addressable-world descriptor seam, exact C3A world validation and the GP1
stable-stop reducer. Broad C4 remains unchanged. GP4 may consume this seam only
after its separate GP3 dependency is satisfied.

The crate implements strict `VerticalIdentityV1`, atomic
`VerticalCommandBatchV1`, baseline-bound `VerticalLogV1`, replay-verified
`VerticalSnapshotV1`, one length-framed V1-to-V2 codec adapter and an
authority-negative `world-history-ledger` receipt. It imports no Kernel,
desktop, filesystem, network, Greenfield, C3B, GP2, GP3 or GP4 capability.

## Frozen behavior proved

- Hub and place end in C2 Site addresses, player ends in Entity, all share one
  universe seed, and retained address and descriptor bytes reconstruct exactly.
- The exact external C3A input/packet pair binds both the GP1 context and the
  place descriptor. Authored or foreign evidence fails closed.
- Commands persist stable stop to stable stop. Depart and Recover cannot be
  stored alone; each is atomic with ChooseOutcome or FailEncounter.
- Exact command retry returns the original consequence before stale checking.
  Changed retry, stale revision, skipped sequence and crossed parent are
  distinct failures.
- Every append and restart validates the complete retained history both
  structurally and semantically before returning. Rehashed fabricated valid
  consequences, wrong baseline/target, reorder, truncation, altered state and
  corrupt hashes are rejected.
- Snapshots are optional, nondeleting and accepted only by full replay.
- Migration consumes and emits unsigned-big-endian-length-framed canonical log
  bytes, changes only baseline dependency kind 6, preserves command IDs and
  final GP1 bytes, and leaves the original V1 artifact byte-identical for
  rollback.
- Hard bounds are enforced symmetrically: 256 events, two actions, 8 KiB per
  action, 16 KiB per command, 256 KiB state/consequence, 4 MiB log, 512 KiB
  snapshot and 64 KiB migration/evidence receipt. Append checks the whole next
  log before returning; GP1's finite run grammar makes the 4 MiB crossing
  unreachable in current valid fixtures, but the acceptance check is local and
  mandatory.

## Verification

Seven focused adversarial tests cover all five sessions including exact S1 to
S5 predecessor ledger authority, atomic batch/retry ordering, full restart and
terminal behavior, snapshots, migration/rollback, strict receipt codecs,
crossed identities/descriptor/packet/actor, fabricated GP1 state, wrong
baseline/target and retained semantic consequence substitution.

The focused command is:

`cargo test -p mindwarp-vertical-persistence`

The registered verifier is:

`tools/verify-c4v-vertical-persistence.ps1`

## Retained exclusions

This result does not implement production storage, runtime persistence,
cross-target transactions, deletion/compaction, generic migration, GP3/GP4,
C3B, D4, R1, Greenfield integration, publishing, authentication or promotion.
Nothing broader is locked in.
