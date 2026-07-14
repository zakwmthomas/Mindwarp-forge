# Hierarchy and History Design Gate

**State:** researched recommendation; explicit design confirmation required
before implementation. **Date:** 2026-07-13.

## Evidence and decision

### Recovered project evidence reconciliation

Before asking the owner to decide, the authoritative survival pack was checked
directly at SHA-256
`f0f01b7469226d3d5c77780c23e97a96342b517ece536bc0351e9486117b251b`:

- `01_CURRENT_TRUTH/MASTER_DEVLOG_V0.2.0/FORGE_MASTER_SPEC.md`, sections 6,
  7, 20, 22 and 24, already decides that there is exactly one addressable—not
  fully enumerated—universe; finite tests are sampled observations; only
  relevant systems are materialized; lazy generation and finite observation
  windows are mandatory; caches are disposable; and saves are baseline
  reconstruction key + generator version + explicit world/player deltas.
- The same source names destruction, ownership, inventory, discoveries,
  structures, relationships, economy, politics and player-created phenotypes
  as delta families; says coordinates alone are insufficient; requires bounded
  deterministic save migration/recovery; lists the complete save/ledger as
  unresolved work; and says never to overwrite history silently.
- `04_FORGE_AND_GAME/MIND_WARP_GAME_CONTEXT.md` independently repeats the
  baseline/version/delta save model and prohibits runtime repair from freely
  rewriting progression, economy, network truth or repair logic.
- `06_SOURCE_PACKAGES/mindwarp_parent_child_field_inheritance.zip` verifies
  deterministic parent/child variation experiments, but it is field-generation
  evidence—not a save, conflict, or persistence implementation. Its Python
  timings and provisional thresholds are therefore not promoted here.

So the large product architecture is **not** being referred back to the owner.
It is already accepted project truth. The remaining decisions below are small,
versioned safety defaults for the first engine-neutral reference harness.

Official guidance supports three separations needed by Mind Warp:

- IPFS Merkle-DAG nodes are content-addressed and immutable; changing content
  produces a different node/ancestor graph: <https://docs.ipfs.tech/concepts/merkle-dag/>.
- Microsoft Orleans treats an activation as a temporary in-memory embodiment of
  a durable virtual identity, supporting separation of existence from residency:
  <https://learn.microsoft.com/en-us/dotnet/orleans/host/configuration-guide/activation-collection>.
- Azure's event-sourcing guidance reconstructs state by ordered replay, warns
  about ordering/duplicates, and treats snapshots as replay optimizations rather
  than replacements for history:
  <https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing>.

The recommendation uses these properties without adopting IPFS networking,
Orleans actors, Azure services, distributed eventual consistency, or a runtime.

## Recommended bounded lane

The whole-system dependency and external-practice review in
`HIERARCHY_HISTORY_SYSTEM_ALIGNMENT_AUDIT.md` found the direction compatible
but repaired five seams before implementation: full baseline dependency
binding, version-bound paging, per-target stream/idempotency semantics,
bounded staged migration, and reducer-independent snapshot/scale evidence.

1. `HierarchyDescriptor` is immutable strict-CBOR data keyed by the existing
   logical universe identity. Its separate reconstruction fingerprint binds
   identity, generator/field contract versions, parent logical identity, and
   exact descriptor recipe bytes. Content change never rewrites logical identity.
   A versioned opaque `WorldConditionsRef` is accepted as a dependency seam;
   P4 does not implement or claim proof of derived-world rules.
2. Children are requested through a finite `ChildWindow` containing parent,
   child kind, version-bound canonical cursor (parent descriptor fingerprint,
   child kind, next index), and bounded limit
   (prototype maximum 256, not a permanent gameplay/population cap). The cursor
   is only the explicit tuple above; P4 defines no configurable filtering or
   ordering language. Results state
   their exact window and `has_more`; no sampled window claims a total
   population and no `has_more` check may trigger unbounded enumeration.
3. `MaterializationReceipt` records warm/cold/evicted cost only. Cache keys and
   loaded objects never participate in descriptors, baselines, or history.
4. A strict `BaselineManifest` binds logical target, descriptor fingerprint,
   and the sorted set of **output-affecting** generator/field/derived-rule,
   hierarchy, and state-reducer contract fingerprints. Compiler, build,
   storage, hardware, cache, and other incidental versions are excluded.
   `BaselineKey` commits to that complete canonical manifest.
   `DeltaEnvelope` binds schema, baseline, target logical identity, stream scope,
   stable command/idempotency ID, monotonically increasing sequence, expected
   parent delta ID, operation-schema fingerprint, operation bytes, and content
   ID. Global time and transaction-group semantics are deliberately absent.
   V1 is a single-writer linear chain per
   target stream with compare-and-append head; this is a conservative offline
   reference default, not a global serialization or multiplayer policy.
5. Equal retry is idempotent. A different delta at the same sequence/parent is
   a visible fork conflict; no timestamp or last-write-wins fallback exists.
   Multi-writer merge/CRDT and cross-target atomic semantics are deferred until
   gameplay and authority requirements can define intent-preserving operations.
   Unsupported cross-target operations fail visibly rather than partially apply.
6. Snapshots are optional additive replay checkpoints binding baseline, exact
   covered head/sequence, reducer version, resulting state hash, and builder
   version. They are accepted only after replay equivalence plus a retained
   independent expected fixture and never delete source deltas in the reference
   package. Replay/storage growth is measured over increasing bounded sizes.
7. Generator, descriptor, reducer, or delta-schema migration produces
   append-only migration receipts linking old/new baselines, exact adapter
   versions, input/output lineage, and proof. Multi-hop migration is bounded and
   every intermediate is validated. Failure leaves the original chain readable
   and blocks use of newer semantics.
8. Old and new generator baselines may coexist under stable logical identity.
   The reference proves an old touched target remains pinned while a separately
   versioned sibling/target can use improved generation. It does not select the
   future product policy for when a place becomes pinned.

All canonical envelopes use strict deterministic CBOR. The first implementation
is capability-free and in-memory with serialization/replay fixtures; choosing a
save database, filesystem layout, runtime residency manager, or networking is
outside this gate.

## Alternatives challenged

| Alternative | Rejection/defer reason |
|---|---|
| Coordinates as mutable identity | Generator/version changes and collisions can orphan history |
| Eager child arrays | Contradicts addressable infinite-scale hierarchy and creates unbounded work |
| Cache object as descriptor | Eviction or runtime change would rewrite canon |
| Current-state CRUD only | Loses intent, replay, migration, corruption boundary, and audit lineage |
| Timestamp/last-write-wins | Hides forks and depends on clock/order semantics unrelated to player intent |
| CRDT merge now | Operation semantics are not yet defined; generic convergence can preserve the wrong game meaning |
| Destructive snapshot compaction | A bad snapshot could erase the only recoverable lineage |

## Required proof

- Descriptor reconstruction, parent/child binding, changed-version separation,
  dynamic-instance identity, absence/tombstone separation, and collision rejection.
- Empty/small/boundary windows, maximum limit, stable paging, cancellation, and
  stale-token rejection plus proof that observation/`has_more` never triggers
  unbounded enumeration.
- Cold/warm/evicted materialization with identical canonical descriptors.
- Empty replay, ordered sparse deltas, equal retry, stale head, gap, duplicate,
  fork, unrelated target/baseline, stable command deduplication, unsupported
  cross-target operation, unknown version, and authority-negative cases.
- Truncated/corrupt envelope quarantine, known-good head recovery, additive
  snapshot replay equivalence, poisoned reducer/snapshot rejection, bounded
  staged migration, migration rollback, and visible rejection of undefined
  reparent/split/merge semantics.
- Full baseline-dependency closure, deterministic fake `WorldConditionsRef`,
  increasing replay/window/storage cost curves, and no derived-world proof claim.
- Fresh-process and second-platform strict-byte receipts before
  `reference_proven`; measured replay/materialization cost labelled locally.
- Read-only ProofReceipt/Reference Studio integration with no Kernel mutation.

## Exact confirmation gate

Confirm proceeding with the repaired test-only P4 protocol harness for the
already accepted lazy-universe and baseline-plus-delta architecture. The
256-item window and per-target single-writer chain are versioned prototype
bounds, not permanent gameplay or multiplayer decisions. Confirmation
authorizes only capability-free contracts, codecs, dependency fixtures, replay,
recovery/migration/scale evidence, and ProofReceipt integration. It does not
authorize saves, a storage engine, cross-target transactions, runtime residency,
multiplayer merging, destructive compaction, engine work, or protected-Kernel
mutation.
