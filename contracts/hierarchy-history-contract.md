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

`BaselineManifest` commits to target logical identity, descriptor fingerprint,
and a sorted unique set of output-affecting dependency fingerprints. Incidental
compiler, build, hardware, storage, cache, and runtime versions are forbidden.
This allows old and new reconstruction baselines to coexist without changing
logical identity or forcing harmless optimizations through migration.

`DeltaEnvelope` belongs to exactly one target stream and baseline. It contains
an exact sequence, expected parent content ID, stable command ID, operation-
schema fingerprint, bounded operation bytes, and content-derived ID. Equal
retry is idempotent. Stale head, gap, duplicate command with changed content,
fork, wrong target/baseline, unknown reducer, and unsupported cross-target work
fail visibly. No wall clock, global simulation clock, merge, transaction-group,
or authority semantics are inferred.

The reference reducer is an explicitly test-only integer key/value state used
to prove replay. Game operations remain owned by later domain contracts.
Snapshots are additive and bind baseline, covered head/sequence, reducer,
builder, canonical state bytes, and state hash. Verification replays source
deltas and compares a retained expected fixture. P4 never deletes source deltas.

Migration receipts bind exact old/new baseline keys, source head, adapter,
input/output state hashes, and result. Equal retry is idempotent; unsupported
semantic changes reject without mutating the source lineage. Multi-hop limits
and long-term compatibility policy remain future production decisions.

Required proof covers strict bytes and corruption, descriptor reconstruction,
dynamic/procedural identity separation, cold/warm/evicted equivalence, bounded
paging and stale cursors, old/new generator coexistence, replay/idempotency/
fork/gap rejection, unsupported cross-target work, snapshot poisoning,
migration rollback, increasing bounded cost measurements, fresh-process replay,
ProofReceipt integration, and authority-negative behavior. Second-platform
exact bytes and the real derived-world contract remain required before
`reference_proven`.
