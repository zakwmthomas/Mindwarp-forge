# Lazy Hierarchy and World History: Readiness Package

**Status:** discovery plus researched design recommendation. The exact gate is
retained in `HIERARCHY_HISTORY_DESIGN_GATE.md`. This package does not
materialise a universe, create saves, or implement a game/runtime database.

## Source evidence and limits

The recovered master specification requires one addressable rather than fully
enumerated universe, a hierarchy from universe through entities, lazy
materialisation, disposable caches, and a save model of baseline reconstruction
key + generator version + explicit world/player delta ledger. It specifically
warns that coordinates alone are insufficient for mutable state, collision
resistance, or generator-version changes. The evidence is retained with fixity
in `evidence/handover-manifest.json`; it is not a passing persistence system.

## Boundary to establish

The paired harness should distinguish immutable description, disposable
materialisation, and mutable history:

| Record | Canonical role | Must not contain |
|---|---|---|
| `HierarchyDescriptor` | Typed address, identity/version context, parent relation, deterministic child descriptor recipe | Loaded object pointers, cache state, renderer/runtime objects |
| `ObservationWindow` | Explicit finite request for a descriptor subset and detail band | Claim that observed set is total universe contents |
| `ResidencyRecord` | Non-canonical cache/materialisation telemetry | Identity or mutable-world authority |
| `BaselineKey` | Reconstructable descriptor/version reference for mutable state | Mutable delta contents |
| `DeltaEnvelope` | Typed explicit change against one baseline and lineage | Ambient coordinates as sole identity, generator defaults |
| `HistoryReceipt` | Replay, migration, collision, recovery result | Owner approval/promotion authority |

## Invariants

- A descriptor is reconstructable from its address, versioned identity context,
  and declared parent/recipe inputs; materialising it does not alter it.
- Observation is finite and explicit. A sample is never interpreted as total
  population or complete world state.
- Cache warm/cold/evicted state cannot affect descriptor identity or a baseline
  reconstruction.
- A delta applies only to an exact typed baseline/lineage. Incompatible or
  unknown versions must produce a migration/rejection receipt.
- Replaying the same ordered delta set yields the same effective state;
  collision/conflict handling is explicit rather than last-write-wins by
  accident.
- Corrupt, partial, duplicate, or unrelated deltas cannot silently rewrite
  canonical baseline state.

## Paired fixture matrix

| Fixture | Required observation |
|---|---|
| Reconstruct same descriptor twice | Equal descriptor fingerprint with zero dependence on residency |
| Cold/warm/evicted cache | Equivalent canonical descriptor; telemetry may differ only in labelled cost fields |
| Small observation window | Bounded materialisation count and explicit sampled scope |
| Parent/child descriptor pair | Child relation is deterministic but not a direct topology/cache copy |
| Empty delta replay | Reconstructs declared baseline only |
| Ordered sparse deltas | Same ordered set produces identical effective state and lineage receipt |
| Duplicate/conflicting delta | Idempotency or conflict result is explicit and testable |
| Generator-version mismatch | Explicit migration path or visible rejection; never newest-generator fallback |
| Corrupt/truncated delta | Failure with retained known-good baseline and recovery receipt |

## Neighbour contracts

| Neighbour | Provides | Receives |
|---|---|---|
| Universe identity | Typed address, generator version, partitioned stream context | Descriptor reconstruction reference only |
| Field/derived-world rules | Versioned recipe/conditions references | No mutable history or cache state as field input |
| Significance/scheduler | Descriptor metadata and explicit observation windows | No authority to alter descriptor/history canon |
| World history ledger | BaselineKey and descriptor lineage | Explicit DeltaEnvelope and replay/recovery status |
| ProofReceipt/Reference Studio | Fixtures, fingerprints, costs, failure/recovery evidence | Read-only inspection only |

## Readiness gaps deliberately left open

The evidence does not select the production derived-world rule contract,
canonical hierarchy vocabulary, production observation policy, multiplayer or
cross-target transaction semantics, retention/compaction, or recovery storage
boundary. The alignment audit permits only an opaque deterministic
`WorldConditionsRef` fixture seam and makes the remaining choices explicit;
they cannot be inferred from cache or runtime behavior.

## Entry criteria for a future implementation package

- ProofReceipt, identity, and field numerical-policy decisions are resolved.
- Descriptor, baseline, and delta contracts are separately versioned and have
  explicit compatibility/rejection behavior.
- `BaselineKey` commits to the full descriptor/field/derived-rule/reducer
  dependency manifest; the derived-world fixture cannot be mistaken for proof.
- Fixture matrix covers reconstructability, lazy residency, sparse replay,
  conflict, migration, and corruption recovery before any promotion claim.
- History work is data-only and engine-neutral; cache and runtime services stay
  outside canonical descriptor/delta semantics.
