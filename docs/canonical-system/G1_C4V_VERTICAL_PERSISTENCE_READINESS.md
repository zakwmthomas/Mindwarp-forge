# G1 C4V vertical persistence readiness

Status: owner-authorized decision-complete readiness; source implementation is
blocked until this contract and its adversarial matrix are durable.

## Exact scope and dependencies

C4V is one additive, engine-neutral and capability-free sibling depending only
on C2, C3A and GP1. It is not broad C4 closure and does not alter C4, C5, C6,
D4, R1 or C3B. GP4 may depend on C4V plus GP3 instead of broad C4.

The implementation target is an isolated `mindwarp-vertical-persistence`
reference crate. It proves deterministic stable-stop-to-stable-stop command
history, strict restart, one codec-only migration/rollback fixture and one
read-only evidence receipt. It is not a production save engine.

## Frozen authority

`VerticalIdentityV1` binds:

- hub ID to a C2 `UniverseAddress::logical_fingerprint`;
- place ID to a `HierarchyDescriptor.logical_id`;
- player ID to a C2 entity address fingerprint;
- encounter ID to a domain-separated hash of hub, place, player, GP1 session
  and run identities;
- immutable typed session and run IDs.

The identity record retains canonical hub, place and player `UniverseAddress`
bytes plus exact canonical `HierarchyDescriptor` bytes; restart strictly
decodes and re-encodes them before recomputing every fingerprint. Hub and place
addresses end at `NodeKind::Site`, player ends at `NodeKind::Entity`, all share
one universe seed, all logical fingerprints are pairwise distinct, and the
descriptor logical ID equals the place address fingerprint. Session and run
reuse the exact GP1 ID constraint: lowercase ASCII letters, digits, `.`, `_`
or `-`, with length 1-96 bytes.

Encounter identity uses domain `mindwarp/c4v/encounter-id/v1\0` over six
unsigned-big-endian-length-framed fields in this exact order: hub fingerprint,
place fingerprint, player fingerprint, canonical session bytes, canonical run
bytes and GP1 contract fingerprint.

Every C3A-backed start must externally replay and validate the exact
`WorldGenerationInput` and `CausalWorldPacket`, call
`bind_addressable_world_package`, and bind the exact hierarchy descriptor and
baseline. Authored GP1 context is inadmissible for C4V.

The baseline is exact: `BaselineManifest.logical_id = place_id` and
`descriptor_fingerprint = HierarchyDescriptor::fingerprint()`. Its sorted
dependency registry is: kind 1 C3A world-conditions contract fingerprint;
kind 2 exact packet fingerprint; kind 3 GP1 reducer contract fingerprint;
kind 4 vertical identity-manifest fingerprint; kind 5 initial GP1 state hash;
kind 6 C4V codec fingerprint. V1 kind 6 is the domain hash of
`mindwarp/c4v/codec/v1\0`; no caller chooses dependency kinds.

The GP1 state's `world_context` must equal
`bind_validated_c3a_world(exact_input, exact_packet)` for the same input and
packet passed to `bind_addressable_world_package`. Recomputing the addressable
descriptor from those inputs must produce bytes exactly equal to the retained
descriptor bytes. `DeltaEnvelope.target_logical_id` is always `place_id`;
encounter uniqueness remains bound through dependency kind 4.

## Atomic command boundary

`VerticalCommandBatchV1` binds actor/player, command ID, expected revision,
sequence and one or two ordered GP1 actions. It is accepted only when sequence
is current revision plus one, expected revision is current revision,
`DeltaEnvelope.expected_parent` is the current head, and identity/baseline are
exact. The input must be a typed stable stop and the output the next typed
stable stop.

Two-action batches are exactly:

- `Depart` plus `ChooseOutcome`;
- `Depart` plus `FailEncounter`;
- `Recover` plus `ChooseOutcome`;
- `Recover` plus `FailEncounter`.

`Depart` and `Recover` are never persisted alone. All other accepted batches
contain one GP1 action that independently reaches the next stable stop.

Retry is checked before stale: an identical command ID and canonical command
returns its original consequence, a changed command with the same ID is a
conflict, a different command at an old revision is stale, and a skipped
sequence is a gap. Each append atomically stores canonical command bytes and
the exact resulting GP1 bytes and hash inside one canonical `DeltaEnvelope`.

Validation order is fixed: strict static identity, baseline and actor checks;
then existing command lookup and byte-exact canonical retry/conflict; then
expected-revision stale; sequence gap or old-sequence stale; and finally parent
fork. Retry equality covers actor, all identity bytes, expected revision,
sequence and ordered canonical action bytes.

Hard bounds are: 256 events; two actions; 8 KiB per action; 16 KiB canonical
command; 256 KiB initial or consequence state; 4 MiB canonical log; 512 KiB
snapshot; and 64 KiB migration or evidence receipt. Encoding and decoding
enforce identical bounds.

## Restart, snapshot and migration gates

`VerticalLogV1` retains strict baseline and identity bytes, initial GP1 bytes,
and canonical delta bytes. Restart begins fresh, revalidates external C3A and
the canonical session, rebuilds the hierarchy `HistoryStream`, replays every
command, and requires byte-identical final GP1 state, `BaseLoopLedgerV1`,
re-encoded log, head, revision and hashes.

An optional snapshot binds baseline, identity, head, revision, reducer and
codec fingerprints, state bytes and state hash. It is accepted only after
full replay proves equality and never deletes events.

The sole migration is a V1 stream to a V2 baseline that changes only dependency
kind 6 from the codec fingerprint of `mindwarp/c4v/codec/v1\0` to the
fingerprint of `mindwarp/c4v/codec/v2\0`. Adapter ID is the domain hash of
`mindwarp/c4v/v1-to-v2-adapter/v1\0`; it consumes length-framed canonical V1
log bytes and emits length-framed canonical V2 log bytes. One deterministic adapter re-emits the same command IDs
in the same semantic order, recomputes baseline identities, requires identical
final GP1 bytes, and emits a deterministic receipt. Poisoned input rejects;
the untouched V1 stream reopens byte-identically as rollback evidence.

The vertical delta operation schema is the domain hash of
`mindwarp/c4v/vertical-command-operation-schema/v1\0` and never
`reference_operation_schema`. Consequence, identity, log, snapshot, migration
and receipt hashes use, respectively, `mindwarp/c4v/consequence/v1\0`,
`mindwarp/c4v/identity/v1\0`, `mindwarp/c4v/log/v1\0`,
`mindwarp/c4v/snapshot/v1\0`, `mindwarp/c4v/migration/v1\0` and
`mindwarp/c4v/receipt/v1\0`, with explicit length framing.

## Reuse and exclusions

Reuse only `UniverseAddress`, C3A packet validation,
`bind_addressable_world_package`, `HierarchyDescriptor` codec/fingerprint,
`BaselineManifest`, `DeltaEnvelope`, `HistoryStream::append`, GP1 state/action
replay and `ProofReceipt`. Do not reuse `ReferenceOperation`, `ReferenceState`,
`Snapshot::build_reference`, `MigrationReceipt::identity_reference`, or Kernel
SQLite.

Excluded: production filesystem/database/cloud storage, save slots, runtime,
cache, ambient population, multiplayer, merge, CRDT, clocks, compaction,
deletion, generic migration, GP2/GP3 expansion, C3B, Greenfield, authentication
and Kernel mutation.

## Adversarial gate

Red tests must reject crossed hub/place/player/actor identities, foreign C3A or
descriptor evidence, authored context, fabricated GP1 state, unsafe-stop
ending, stale/gap/fork/changed retry/wrong baseline, reorder/truncate/trailing
or noncanonical bytes, altered actions/consequence/hash/ledger, command after
terminal and every bad snapshot. Positive tests cover all five structural
sessions, one full restart, and an S1 ledger that legitimately initializes S5.

Nothing broader is locked in. Stop after one consumer and reassess.
