# G1 C1 ProofReceipt and H7 consumer result

Status: **verified and consolidated; no repeated owner decision required**.

## Provenance finding

The inherited C1 owner gate was stale. `F5_PROOF_RECEIPT_DECISION.md` already
records the owner-selected versioned SQLite projection, its authority boundary,
failure matrix, rollback, and exit proof. The current Kernel persistence layer
implements that exact additive projection with canonical receipt IDs, atomic
receipt/link storage, evidence foreign keys, strict validation, version-aware
read-only projection, Reference Studio inspection, idempotent retry, corruption
failure, SQLite reopen, and verified backup recovery. C1 found no novel storage
choice and therefore does not ask the owner to repeat one.

The canonical binding remains:

- protected Kernel objects/events/candidate lifecycle are the sole authority
  and provenance boundary;
- immutable ProofReceipt rows are a separately versioned, authority-negative
  SQLite projection linked to retained Kernel evidence;
- Reference Studio reads the projection and never promotes from it; and
- the additive projection can be ignored by older builds without rewriting the
  protected journal.

## Residual dependency gap and implementation

H7 promotion deliberately established a narrow proof baseline, not an asset.
The remaining C1 gap was that future G1 code could have checked only generic
`Promoted` state. `humanoid-proof-chain-integration` now exposes a read-only
`verify_h7_dependency` boundary that requires:

1. exact Kernel candidate
   `c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433`;
2. exact evidence
   `f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c`;
3. strict canonical decoding of the exact H7 package and candidate type;
4. `Promoted` and therefore non-superseded state;
5. the exact direct-owner, explicit-authorization promotion event; and
6. all six claims plus all eight non-claims, returned to the consumer as
   retained blockers.

The validator is read-only. It changes no object, event, candidate, receipt,
asset, file, runtime, or application state.

## Cheap proof result

The focused crate test passes all existing H6/H7 integration cases plus three
new consumer cases:

- the exact promoted package passes and retains six claims/eight blockers;
- a different generic promoted candidate cannot substitute for H7; and
- Proposed, Approved, and Superseded forms of the exact H7 candidate fail
  closed.

This closes C1. C2 may now reassess the already prototype-tested universe
identity policy against the unified G1 route; it must not assume that C1 or H7
grants asset, runtime, production, or engine authority.
