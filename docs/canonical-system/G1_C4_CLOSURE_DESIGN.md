# G1 C4 hierarchy/history hardening design

Status: **candidate design; implementation follows only after readiness passes.**

## Minimal additive surface

Core hierarchy/history hardening stays inside `hierarchy-history`; it adds no
dependency and changes no existing descriptor, delta, snapshot or
reference-operation bytes. The cohort binding belongs in
`entity-lifecycle-history-binding`, which already owns the integration between
`entity-lifecycle::AgeCohort` and hierarchy-history. The disposable receipt
composes both modules without reversing their dependency direction.

1. `dynamic_instance_logical_id(parent, stable_instance)` derives a domain-
   separated stable ID and rejects zero inputs.
2. Strict `AddressPresence` evidence distinguishes `NeverObserved`, `Absent`,
   `Present` and `Tombstoned` without performing deletion. It uses fixed
   definite CBOR arrays `[1,tag,...]`, exact tag-specific lengths, nonzero
   hashes, a 256-byte input ceiling and strict decode/re-encode equality. It
   does not change `CONTRACT_VERSION` or existing descriptor bytes/domains.
3. `entity-lifecycle-history-binding::AmbientCohortBindingV1` binds nonzero entity logical ID,
   nonzero assignment-contract/version fingerprint and exact `AgeCohort` into
   strict canonical bytes. Reload/replay must reproduce the same binding;
   changed entity, contract or cohort fails. It selects no species weights and
   makes no population-distribution claim.
4. `BaselineManifest::verify_available_dependencies` first revalidates the
   manifest and then requires an exact, bounded, sorted, unique availability
   set by kind and fingerprint. Extras, incidental kinds and C3B kinds reject.
5. `recover_known_good_prefix` preflights at most 1024 records and checked total
   bytes no greater than 16 MiB before decoding. For each record it clones the
   current stream, strict-decodes, appends and semantically replays the candidate;
   only then does it commit the candidate prefix. It stops at the first failure,
   never examines later records, and reports a stable `RecoveryFailureKind`
   rather than codec/debug text. It returns
   the exact retained stream, accepted count and typed failure under fixed
   record/byte ceilings.
6. Reparent, split and merge requests return `UnsupportedTopology` without
   changing stream head, events or replay state.
7. A bounded identity-only migration-chain builder prevalidates one or two
   adjacent canonical target baselines and equal-length, nonzero, distinct
   adapters before building a local receipt vector. Any failure returns only
   `Err`, exposes no partial vector and never mutates the borrowed source.
   `MigrationReceipt::validate_content_id` authenticates the fixed preimage;
   the existing one-hop content ID is pinned before and after refactoring.
8. Deterministic scale evidence has separate fixed window rows and history
   rows. Window rows record request count, returned count, examined nodes and
   canonical window bytes for `0/1/16/256`. History rows record event count,
   canonical delta bytes, canonical full-stream bytes, strict-decoded/replayed
   operation count and canonical snapshot bytes for `0/1/16/64/256` events,
   using checked accumulation. It makes no timing,
   database, cache or runtime claim.

## Semantic receipt

`C4SemanticReceiptV1` binds the contract and fixture IDs, exact registered C2
proof plus explicit replayed C3A input SHA-256, packet ID, packet SHA-256 and
bound descriptor/world-conditions fingerprint, descriptor/dynamic/presence/cohort digests,
ordered baseline keys and
heads, snapshot and known-good recovery digests, ordered migration receipt IDs,
fixed cost rows, hostile-registry digest/count, and these exact false flags:

- `production_storage=false`
- `runtime_residency=false`
- `cross_target_transactions=false`
- `c3b=false`
- `promotion_authority=false`

The exact ordered body is: `schema_version:u16`, `receipt_id:text<=64`,
`contract_id:text<=64`, `fixture_id:text<=64`, `c2_proof:proof`,
`c3a_proof:{input_sha256:[u8;32],packet_id:[u8;32],packet_sha256:[u8;32],
descriptor_fingerprint:[u8;32],world_conditions_fingerprint:[u8;32]}`,
`descriptor_digest:[u8;32]`, `dynamic_ids:[[u8;32];2]`,
`presence_digests:[[u8;32];4]`, `cohort_binding_digest:[u8;32]`,
`baseline_keys:[[u8;32];3]`, `history_heads:[[u8;32];3]`,
`snapshot_content_id:[u8;32]`, `recovery:{baseline_key,accepted_count:u16,
recovered_head:[u8;32],failure_kind:u8,source_sha256:[u8;32]}`,
`migration_content_ids:[[u8;32];2]`, `migration_final_digest:[u8;32]`,
`rollback_source_sha256:[u8;32]`, `window_rows:[window_row;4]`,
`history_rows:[history_row;5]`,
`hostile_registry_digest:[u8;32]`, `hostile_count:u16`, the five booleans above,
then `receipt_hash:[u8;32]`. The C2 proof is fixed
`{id:u8,kind:u8,reference:[u8;32]}`. Window rows are
`{requested:u16,returned:u16,examined:u16,canonical_bytes:u64}`. History rows
are `{event_count:u16,encoded_delta_bytes:u64,full_stream_bytes:u64,
decoded_operation_count:u16,snapshot_bytes:u64}`.
All arrays have exact counts and no free-form proof strings.

Canonical bytes are strict deterministic CBOR. `receipt_hash` is outside the
hashed body and equals SHA-256 of domain
`mindwarp/c4-semantic-receipt/v1\0` followed by every body field encoded as an
8-byte big-endian length plus exact canonical field bytes. Decode rejects
unknown/trailing/coercible data and must re-encode byte-identically. The fixture
replays the real C3A input/packet through `addressable-world-binding`; constants
alone are not proof. Snapshot
and migration digests are validated `content_id` values. Recovery binds stable
failure kind, never Debug text. Cost-row values and their digest are pinned
after implementation. Every field and nested proof reference is mutated.

Platform observations are separate noncanonical receipts containing the exact
semantic SHA, clean source-tree manifest SHA plus commit, target triple, OS,
architecture, pointer width/endian, rustc/cargo versions, exact command/exit,
executable hash when one exists, and separate stdout/stderr hashes from two
fresh processes. Compile-only targets use typed absence, never zero hashes.
Timing is advisory. Windows i686 is
`same_host_second_architecture`; Android check is `compile_only`; neither may be
labelled independent second-platform execution. Promotion requires both
independent execution provenance and platform diversity; same-target remote
Windows x64 fails the latter.

## Test-first order

1. Add seven failing test families for dynamic identity, presence evidence,
   dependency availability, corrupt-tail recovery, topology refusal, two-hop
   migration and deterministic cost rows.
2. Implement only enough additive source to pass them.
3. Retain the four existing C4 package suites and focused C4V regression.
4. Build the public-API semantic receipt in a disposable offline Cargo project;
   run it in two fresh processes and compare raw output bytes.
5. Run same-host i686 and Android compile checks with honest classifications.
6. Require an actual independent second-platform receipt before promotion.
7. Run the dedicated implementation verifier, independent review and one
   measured registered full Forge gate. C5 remains gated until the result is
   recorded and separately activated.
