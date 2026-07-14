# Hierarchy and History: Whole-System Alignment Audit

**State:** researched architecture reconciliation; no implementation or runtime
authority. **Date:** 2026-07-13.

## Result

The accepted direction is sound: one addressable universe, immutable logical
identity, deterministic baseline reconstruction, finite observation, disposable
materialisation, and explicit mutable deltas fit the complete canonical system.
The original P4 gate should not be implemented unchanged, however. Five seams
need to be made explicit in the reference contract so a locally passing harness
does not create an incompatible persistence standard.

## Dependency walk

| Boundary | Alignment result | Required repair or retained limit |
|---|---|---|
| Universe identity -> hierarchy | Stable logical IDs plus separate reconstruction fingerprints correctly keep place identity distinct from generator output. | Dynamic/player-created instances need an explicit stable instance segment/ID fixture; absence, tombstone, and never-observed are distinct states. |
| Field basis -> derived rules -> hierarchy | Field recipes are cache-independent and versioned, but `derived-world-rules` is still only specified even though the registry makes it a direct hierarchy dependency. | Add an opaque, versioned `WorldConditionsRef`/derived-input seam and a fake deterministic fixture. P4 must not claim derived-world integration or `reference_proven` status until that separate contract passes. |
| Hierarchy -> history | Descriptor identity, materialisation, and mutable facts are correctly separated. | A baseline must bind the full reconstruction dependency closure, not just the universe reconstruction fingerprint. Use a canonical `BaselineManifest` root containing descriptor, field/derived-rule, generator, and reducer contract versions. |
| History internal | Parent-linked compare-and-append correctly detects stale writers and forks. | Make the single writer **per target stream**, add a stable command/idempotency ID and causation/group reference, and reject unsupported cross-stream atomic actions visibly. Do not imply one global serial chain. |
| History -> significance/scheduler | Descriptor windows and disposable residency telemetry are the right inputs. | Scheduler demand may request materialisation but cannot create identity, history, or canonical population claims. Cursor, filter, version, and work budget must be bound so paging/cancellation cannot skip, duplicate, or enumerate without bound. |
| History -> semantics/construction | World pressures need both reconstructed baseline and explicit changes. | Name an `EffectiveWorldView` read projection. Downstream semantics consumes that projection plus lineage evidence, never raw cache state or an arbitrary subset of deltas. |
| Construction/assets -> history | Player structures and phenotypes belong in deltas, while reusable recipes/artifacts remain separate canonical objects. | Delta operations reference stable instance plus recipe/artifact versions; they do not embed engine objects or make an asset manifest mutable. |
| History -> runtime adapter | Engine-neutral IDs and data-only envelopes preserve the future adapter boundary. | Storage engine, file layout, network authority, replication, and runtime object IDs remain outside P4. Import must later prove identifier and lineage preservation. |
| History -> Forge/Reference Studio | ProofReceipt linkage and read-only inspection fit the existing authority boundary. | The capability-free harness must not mutate the protected Kernel. A future production ledger needs its own authority/transaction decision rather than inheriting ProofReceipt storage semantics. |

## Repairs to the P4 contract before implementation

1. **Complete baseline binding.** Define `BaselineManifest` and make
   `BaselineKey` commit to the logical target, descriptor fingerprint, exact
   generator/field/derived-rule versions and recipes, hierarchy contract, and
   state-reducer version. Unknown or unavailable dependencies block replay.
2. **Version-bound paging.** Replace a free unsigned cursor with an opaque
   canonical continuation token that binds parent, child kind, descriptor
   version, filter/order contract, and next position. Prove `has_more` with
   bounded work, cancellation, stale-token rejection, and no eager scan.
3. **Stream and command semantics.** Treat the linear chain as one target
   stream. Bind a stable command/idempotency ID, operation schema, causation or
   transaction-group reference, sequence, expected parent, and canonical
   simulation time/epoch where time matters. Wall-clock time is informational.
   Cross-target atomicity and multiplayer authority are explicit unsupported
   cases, not accidental partial success.
4. **Safe evolution.** Retain immutable old envelopes and versioned reducers or
   upcasters. Test bounded multi-hop migrations, missing intermediate adapters,
   hierarchy reparent/split/merge, deleted baseline objects, dynamic instances,
   and rollback to the original readable lineage. No ambient "latest" logic.
5. **Independent snapshots and scale evidence.** A snapshot binds the baseline,
   covered head/sequence, reducer version, state hash, and builder version. It
   is checked against replay and a retained expected fixture, never solely by
   the same implementation that built it. Measure replay and storage growth at
   increasing stream/window sizes; snapshots remain optional and source deltas
   remain intact in P4.

## External practice reconciled

- Hello Games' GDC sessions describe a staged continuous pipeline from world
  generation through polygonisation, texturing, population and simulation, and
  emphasize testing an effectively infinite environment. This supports staged,
  finite materialisation but does not prove save or conflict semantics:
  <https://www.gdcvault.com/play/1024265/Continuous_World_Generation_in__No_Man_s_Sky_>
  and <https://www.gdcvault.com/play/1024514/Building-Worlds-Using%3E>.
- Orleans keeps stable logical grain identity separate from temporary in-memory
  activation and can shed idle activations under memory pressure. This supports
  identity/residency separation, not adoption of actors or Orleans storage:
  <https://learn.microsoft.com/en-us/dotnet/orleans/host/configuration-guide/activation-collection>.
- Microsoft's event-sourcing guidance says the pattern is costly and should be
  selective; it requires per-entity ordered streams, optimistic concurrency,
  idempotency, schema evolution, projections, snapshots, and explicit handling
  of cross-entity conflicts. This supports using the ledger for mutable facts,
  not every datum: <https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing>.
- Factorio's developers report that deterministic save/load depends on stable
  ID mapping and that generator/prototype changes, removal order, corruption,
  and staged version migration make loading substantially harder than saving.
  Their ARM/x86 port used per-tick state CRC comparison across 2,417 tests to
  find cross-platform nondeterminism:
  <https://www.factorio.com/blog/post/fff-270> and
  <https://www.factorio.com/blog/post/fff-370>.
- IPFS Merkle-DAG guidance confirms that content changes create new immutable
  nodes/ancestors and permit deduplication and branch-conflict detection. It
  supports content IDs and retained forks, not IPFS networking:
  <https://docs.ipfs.tech/concepts/merkle-dag/>.
- CRDT research distinguishes convergence from preservation of application
  invariants. Deferring generic multi-writer merging until game operations and
  authority rules are known is therefore correct:
  <https://doi.org/10.4230/LIPIcs.ECOOP.2025.4>.
- SQLite WAL can be useful for a later local prototype, but its own guidance
  warns about single-host scope, one-writer behavior, checkpoint starvation,
  unbounded WAL growth, and large transactions. P4 must remain storage-neutral:
  <https://www.sqlite.org/wal.html>.
- EVE's Time Dilation postmortem shows that graceful overload behavior must be
  based on measured whole-system load: reducing a seemingly central physics
  update accounted for only 5-10% of load. P4 should expose materialisation and
  replay cost to the later shared scheduler rather than embed a private policy:
  <https://www.eveonline.com/news/view/introducing-time-dilation-tidi>.

## Failure forecast

Without the repairs above, the likely expensive failures are orphaned deltas
after a generator/rule change; paging duplicates or omissions after version
drift; a "single writer" accidentally interpreted as global architecture;
half-applied inventory/ownership/relationship changes across entities;
snapshots that validate their own reducer bug; replay time and storage growing
without an oracle; and runtime/storage choices leaking into canonical IDs.

## Reconciled implementation plan

1. Amend the P4 data contract with `BaselineManifest`, version-bound cursor,
   per-target stream/command semantics, reducer-aware snapshots, and explicit
   unsupported cross-stream behavior.
2. Add a minimal deterministic `WorldConditionsRef` fixture seam without
   implementing derived-world physics or changing its registry status.
3. Implement the capability-free hierarchy/history crate and strict codec.
4. Run descriptor/window/cache, replay/idempotency/fork, dynamic-instance,
   migration/corruption, snapshot-poison, growth, fresh-process, and
   authority-negative fixtures.
5. Integrate read-only ProofReceipts; do not select a database or mutate the
   Kernel.
6. Keep P4 at `prototype_tested`. Before `reference_proven`, require the real
   derived-world contract, second-platform bytes, and measured scale evidence.
7. Continue to P5 significance/scheduler only through descriptor windows,
   effective-world projections, typed work tickets, and disposable telemetry.

## Decision boundary

The next owner decision is whether to build this **repaired test-only P4
protocol harness**. Approval would not authorize a save database, multiplayer,
cross-entity transaction policy, runtime residency, engine work, destructive
compaction, or protected-Kernel changes.

## Conditional-approval critical revalidation

The owner approved implementation only after a second pass checked permanent
quality, simplification, and project-philosophy alignment. That pass confirmed
the direction but deliberately reduced the proposed permanent surface:

- A baseline binds only **output-affecting canonical contract fingerprints**.
  It does not bind compiler, build, storage, hardware, cache, or other incidental
  versions. This preserves exact reconstruction without turning harmless
  optimization into save migration.
- The P4 cursor is an explicit canonical tuple of parent descriptor fingerprint,
  child kind, and next index. P4 has no configurable filter/order language, so
  an opaque extensible token and unused policy fields would add complexity
  without proof value. A later version can add a separately named query contract.
- The v1 delta keeps target stream, expected parent/sequence, stable command ID,
  operation schema fingerprint, and operation bytes. A global simulation clock
  and transaction-group field were removed: their semantics belong to later
  gameplay/network design and premature placeholders would look authoritative.
- Cross-target operations remain explicitly unsupported. Orleans demonstrates
  that correct multi-entity ACID behavior is a distinct opt-in subsystem with
  retry and unknown-outcome handling, not a free property of per-entity logs:
  <https://learn.microsoft.com/en-us/dotnet/orleans/grains/transactions>.
- KurrentDB independently validates the minimal stream rule: stable event ID
  plus expected stream version provides retry idempotency and optimistic
  concurrency; disabling the expected-version check weakens that guarantee:
  <https://docs.kurrent.io/clients/python/v1.2/appending-events>.
- Generator improvement must not require either freezing quality or resetting
  player work. No Man's Sky has shipped new planets and rewritten generation
  and meshing while explicitly preserving existing terrain and bases. P4 must
  prove old/new baseline coexistence and pinning rather than one universe-wide
  destructive upgrade: <https://www.nomanssky.com/origins-update/>,
  <https://www.nomanssky.com/worlds-part-i-update/>, and
  <https://www.nomanssky.com/worlds-part-ii-update/>.
- Factorio simplified its stable-ID loading design by separating removed IDs
  and migrating old IDs to current ones instead of restoring save-local ID
  layouts. This supports Mind Warp's stable logical identity plus explicit
  baseline mapping and warns against retaining incidental internal layouts:
  <https://www.factorio.com/blog/post/fff-259>.
- Factorio also makes migration application ordered and idempotent and records
  which migrations ran. P4 therefore proves exact source/target/adapter
  identity and equal-retry behavior, but does not promise unlimited historical
  compatibility or implement reparent/split/merge policy before game semantics:
  <https://lua-api.factorio.com/latest/auxiliary/migrations.html>.
- Git pack files show that immutable objects can later be compressed or stored
  as deltas without changing their logical history surface. P4 snapshots stay
  a replay optimization; storage compression remains a later representation
  concern: <https://git-scm.com/docs/git-pack-objects>.
- No Man's Sky fixed a case where repeated portal visits enlarged saves enough
  to impede saving. The reference ledger therefore records semantic state
  changes, not every observation, cache access, render event, or transient
  simulation tick, and includes bounded growth measurements:
  <https://www.nomanssky.com/2017/08/atlas-rises-patch-1-34/>.

### Philosophy check

| Project principle | P4 preservation rule |
|---|---|
| Minimal foundations, emergent richness | Freeze envelopes and invariants, not hierarchy vocabulary, world rules, or gameplay operations. |
| Marginal-return optimization | Stop at discriminating fixtures; no database, CRDT, distributed transaction, or runtime framework without measured need. |
| Prevention before repair | Strict canonical bytes, expected-head append, dependency fixity, and fail-closed replay prevent silent damage; migration remains available. |
| Expensive detail earns persistence | Persist semantic deltas only; observation, cache, residency, rendering, and transient ticks remain disposable. |
| Graceful degradation | Corrupt/newer history leaves its baseline and last verified head readable. |
| Player-visible quality first | Old and new generator baselines coexist; player-modified places are never silently regenerated for technical convenience. |
| Compression by representation | Snapshots and future packing optimize storage/replay without becoming identity or deleting source lineage in P4. |
| Unification without false generality | One envelope mechanism is shared, while operation meaning, authority, scheduling, and cross-target consistency remain owned by their later systems. |

### Revalidation result

The condition is satisfied. The smaller contract preserves the entire accepted
architecture while leaving higher-quality generators, derived rules, gameplay
semantics, multiplayer authority, storage, and runtime implementations open.
Implementation may proceed only in this reduced capability-free lane and must
remain `prototype_tested`.
