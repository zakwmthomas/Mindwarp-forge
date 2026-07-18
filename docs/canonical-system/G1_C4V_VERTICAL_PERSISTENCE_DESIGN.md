# G1 C4V vertical persistence design

Status: frozen pre-source design receipt.

## Canonical records

- `VerticalIdentityV1`: exact hub/place/player/encounter/session/run identity.
- `VerticalCommandBatchV1`: immutable actor, command ID, expected revision,
  sequence and one or two ordered GP1 actions.
- `VerticalEventV1`: canonical command bytes plus exact resulting GP1 bytes and
  domain-separated hash, wrapped by one canonical hierarchy `DeltaEnvelope`.
- `VerticalLogV1`: baseline/identity/initial-state bytes and ordered canonical
  delta bytes with strict codec and bounded vectors.
- `VerticalSnapshotV1`: optional replay-checked accelerator with no deletion.
- `VerticalMigrationReceiptV1`: exact one-way V1-to-V2 codec adapter evidence.
- `VerticalPersistenceReceiptV1`: read-only `world-history-ledger` evidence for
  proof `c4v-vertical-persistence-seam-v1`.

## Reducer and ordering

The reducer accepts only externally replay-validated C3A context and a
strictly decoded GP1 state. It verifies current stable stop, exact actor and
identity, retry/conflict before stale/gap, then applies the complete batch to a
temporary GP1 state. No event becomes visible unless every action succeeds and
the resulting state is the next stable stop. Append uses exact
`DeltaEnvelope` baseline, target, sequence and parent authority.

Restart is not deserialization trust. It reconstructs all upstream bindings,
decodes each delta strictly, replays each canonical command, compares the exact
stored consequence, and then re-encodes the entire log.

## Reversible boundaries

Snapshot and migration are optional proof fixtures, not architecture choices.
There is no production storage or runtime adapter. The V2 fixture may change
only codec dependency identity; gameplay semantics, commands and final
GP1 bytes are invariant. V1 remains independently reopenable after migration.

## Verification order

1. Contract/readiness and dependency registration.
2. Failing identity, batch, retry, restart, corruption, snapshot and migration
   tests.
3. Smallest isolated source implementation.
4. Focused C4V plus retained GP1/GP0 tests.
5. Independent review, workspace and one registered complete gate.

No GP3, GP4, runtime or broad C4 implementation is authorized.
