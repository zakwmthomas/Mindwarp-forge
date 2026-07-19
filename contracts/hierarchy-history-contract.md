# Hierarchy and History Contract v1

This is a capability-free, engine-neutral reference contract. It proves the
separation of immutable hierarchy description, bounded observation,
non-canonical residency, deterministic baseline reconstruction, and explicit
mutable history. It is not a save system, database, multiplayer authority,
runtime residency manager, or production world generator.

The two covered registry systems are `prototype_tested`; this contract does not
claim `reference_proven` or production readiness.

`HierarchyDescriptor` binds a stable logical universe identity, optional parent
identity, reconstruction fingerprint, an opaque versioned world-conditions
reference, origin class, and exact bounded recipe bytes. Descriptor fingerprints
are content-derived. Materialisation state never participates in descriptor or
baseline identity.

`ChildCursor` is the smallest version-bound paging contract: parent descriptor
fingerprint, child kind, and next unsigned index. `ChildWindow` is capped at 256
items for the prototype only. Observation reports exact scope and `has_more`
without enumerating beyond the requested page. P4 defines no filtering or
custom-order language.

C4 adds a single exact zero-request case: `limit=0`, zero logical children and
zero work budget returns an empty 43-byte canonical observation. A zero request
against nonempty scope or nonzero work budget rejects. Canonical requested,
returned, examined and byte rows are `(0,0,0,43)`, `(1,1,1,113)`,
`(16,16,16,1163)` and `(256,256,256,18201)`.

Dynamic instances derive stable domain-separated logical IDs from nonzero
parent and stable-instance IDs. Address evidence has four strict states:
never observed, absent, present and tombstoned. Those states are evidence and
never perform deletion. The lifecycle binding separately commits a nonzero
entity ID, assignment-contract fingerprint and exact ambient age cohort;
reload validates that retained binding and never rerolls it.

`BaselineManifest` commits to target logical identity, descriptor fingerprint,
and a sorted unique set of output-affecting dependency fingerprints. Incidental
compiler, build, hardware, storage, cache, and runtime versions are forbidden.
This allows old and new reconstruction baselines to coexist without changing
logical identity or forcing harmless optimizations through migration.
Availability validation requires the exact sorted dependency kinds and
fingerprints: missing, mismatched, duplicate, zero, unsorted, incidental and
extra C3B dependencies all reject.

`DeltaEnvelope` belongs to exactly one target stream and baseline. It contains
an exact sequence, expected parent content ID, stable command ID, operation-
schema fingerprint, bounded operation bytes, and content-derived ID. Equal
retry is idempotent. Stale head, gap, duplicate command with changed content,
fork, wrong target/baseline, unknown reducer, and unsupported cross-target work
fail visibly. No wall clock, global simulation clock, merge, transaction-group,
or authority semantics are inferred.

Known-good recovery preflights at most 1,024 records and 16 MiB, then commits
each strict-decode, append and semantic-replay candidate atomically. It stops
at the first typed failure and never examines or admits a later tail record.
Reparent, split and merge remain typed unsupported topology operations.

The reference reducer is an explicitly test-only integer key/value state used
to prove replay. Game operations remain owned by later domain contracts.
Snapshots are additive and bind baseline, covered head/sequence, reducer,
builder, canonical state bytes, and state hash. Verification replays source
deltas and compares a retained expected fixture. P4 never deletes source deltas.

Migration receipts bind exact old/new baseline keys, source head, adapter,
input/output state hashes, and result. Equal retry is idempotent; unsupported
semantic changes reject without mutating the source lineage. The C4 reference
receipt proves at most two adjacent identity-only hops with every intermediate
validated and readable-source rollback. Broader migration and long-term
compatibility remain future production decisions.

Required proof covers strict bytes and corruption, descriptor reconstruction,
dynamic/procedural identity separation, cold/warm/evicted equivalence, bounded
paging and stale cursors, old/new generator coexistence, replay/idempotency/
fork/gap rejection, unsupported cross-target work, snapshot poisoning,
migration rollback, increasing bounded cost measurements, fresh-process replay,
ProofReceipt integration, and authority-negative behavior. Exact promoted C3A
replay is retained by `addressable-world-binding`; independent second-platform
exact bytes remain open before `reference_proven`.

The promoted C3A `WorldGenerationInput`/`CausalWorldPacket` seam is the exact
derived-world dependency for C4. Fresh-process evidence means two separately
launched processes with byte-identical canonical semantic receipts. A second
architecture on the same Windows host is labelled
`same_host_second_architecture`; an Android build without execution is
`compile_only`. Neither satisfies independent second-platform proof. Promotion
requires the same semantic receipt to execute twice with both independent
execution provenance and platform diversity (a different OS family or actual
target architecture/device class) at the same source manifest. A remote runner
using the same Windows x64 platform does not satisfy platform diversity, and a
same-host second architecture does not satisfy independent execution.
Platform timings and toolchain metadata remain observations outside canonical
semantic equality.

The pinned history cost rows `(events, delta bytes, full-stream bytes, decoded
operations, snapshot bytes)` are `(0,0,148,0,179)`, `(1,182,332,1,215)`,
`(16,3407,3587,16,261)`, `(64,13848,14125,64,489)` and
`(256,55705,56367,256,1451)`. They are deterministic byte/work evidence, not
latency, storage-engine, cache, memory-residency or runtime performance claims.
