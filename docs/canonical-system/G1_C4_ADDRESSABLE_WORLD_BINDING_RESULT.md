# G1 C4 addressable-world-binding result

Status: **bounded C4 proof component passed; C4 is incomplete and depends on the promoted C3A seam, not C3B.**

The 2026-07-15 adversarial repair now requires the exact
`WorldGenerationInput` and replays the packet before binding. A schema number
and hexadecimal packet ID alone no longer pass; a packet compiled from another
input is rejected.

## What this closes

The master program's C4 next action begins: "Promote lazy hierarchy and
history semantics, then prove addressable world packages...". `hierarchy-
history`'s `HierarchyDescriptor` already accepted an opaque
`world_conditions_contract`/`world_conditions_fingerprint` pair as a
dependency seam (`HIERARCHY_HISTORY_DESIGN_GATE.md`: "P4 does not implement
or claim proof of derived-world rules"). This result proves a real
`derived_world_rules::CausalWorldPacket` (the promoted C3A dependency seam) can fill
that seam correctly.

The accepted dependency is exact: `WorldGenerationInput`, `CausalWorldPacket`
v1 and `validate_world_packet`, preserving nested identity and provenance. This
crate imports no physical-path, optical-transfer, visibility or presentation
owner. C3B therefore remains a separate evidence-blocked fidelity lane.

## New component

`crates/addressable-world-binding` depends on both `hierarchy-history` and
`derived-world-rules` so that neither of those two modules has to depend on
the other. It exposes `bind_addressable_world_package`, which builds a
`HierarchyDescriptor` whose world-conditions fields come from a real,
compiled `CausalWorldPacket`.

## Proof (5 focused tests, in-memory fixture tier)

1. Binding a real packet is deterministic and the resulting descriptor
   round-trips through strict canonical encode/decode.
2. Changing only the physical drivers (atmosphere transmission) changes
   `world_conditions_fingerprint` and the descriptor fingerprint, while
   `logical_id` and `reconstruction_fingerprint` (place identity) do not
   move. Physical change never silently relocates a place.
3. Two packets built from different `reconstruction_id` values (different
   provenance) but identical physical drivers produce byte-identical
   physical palette and signal content, but still bind to **different**
   `world_conditions_fingerprint` values, because the whole-packet
   fingerprint embeds provenance (`input_id`).
4. An out-of-range packet schema version and an empty descriptor recipe both
   fail closed with a typed error rather than binding silently.
5. A valid packet compiled from a different input is rejected before binding.

## Retained limitation (explicit, not hidden)

Point 3 above is a real, recorded gap: this binding has no physical-only,
provenance-independent fingerprint. Any future system that wants to detect
"these two places have physically identical conditions" (for example, cache
reuse or biome deduplication across regions) cannot use
`world_conditions_fingerprint` for that yet. This is out of scope for the
current C4 proof and is not silently assumed to be solved.

## What C4 still has open

- Stable age cohorts and selected-entity lifecycle deltas
  (`SELECTIVE_LIVING_ENTITY_AGING_DESIGN.md` is design-only; nothing in this
  result implements it).
- Recovery without continuous ambient simulation for addressable world
  packages specifically (existing `hierarchy-history` recovery/migration
  tests are packet-agnostic; they were not re-run against a real bound
  packet here).
- Read-only ProofReceipt/Reference Studio integration for this new binding
  (C1-C3 each added this; C4 has not yet).
- A physical-only fingerprint, if a future consumer needs one.

C4 is not closed by this result. It is the first proof component in the same
pattern C3 used: gap audit against the design gate, smallest real causal
seam, then cheap focused tests before broader proof.

## Dependency-correction isolation audit

Status: **one disposable, source-only correction-isolation fixture passed;
the result is reference evidence only. C3 remains open and C4 remains
dependency-gated.**

The owner authorized one bounded audit of whether two exact validated
`WorldGenerationInput` / `CausalWorldPacket` pairs for the same logical place
but different reconstruction provenance remain isolated without destroying the
older replay path. The fixture ran only in a disposable repository copy. It was
not added to canonical source, changed no public API or module boundary, and is
not production evidence.

### Fixture and bounded result

The disposable test
`corrected_world_dependency_keeps_old_history_replayable_and_rejects_cross_baseline_state`
used two valid inputs and packets with one shared logical ID, descriptor recipe
and fixture-supplied reconstruction fingerprint; distinct nested reconstruction
provenance; equal current physical palette, regional exposure and signals; and
distinct packet IDs because exact `input_id` provenance remained bound.

It established only these reference-fixture statements:

1. Crossed input/packet pairs fail with `InvalidCausalPacket`.
2. Valid pairs produce different world-condition, descriptor and baseline
   fingerprints while retaining the same logical ID.
3. A fixture-local sorted three-entry dependency list succeeds; an unsorted list
   fails construction.
4. A baseline-zero delta offered to the baseline-one stream fails with
   `WrongBaseline` without changing the target head or event count.
5. The original stream retains its correctly parented events and test-only
   replay after the corrected descriptor and baseline exist.
6. An original snapshot fails against the corrected stream with
   `SnapshotMismatch`; an original child cursor fails against the corrected
   descriptor with `StaleCursor`.
7. Cold, warm and evicted `MaterializationReceipt` values retain one descriptor
   fingerprint. This is residency-label noninterference only.

`MigrationReceipt::identity_reference` was deliberately constructed as a
hostile counterexample. It can bind different baseline keys and repeat the
source test-state hash without proving semantic correction migration. The
disposition is
`semantic_migration_not_proved_identity_reference_insufficient`, not migration
success.

### Receipt, resource bounds and execution deviation

The original authorized cold selected-test run emitted:

```text
C4_CORRECTION_ISOLATION_DISPOSABLE receipt_sha256=e134a626977b0ca08dc3b6952b1864e07868bf1aab9a5853cf1e0867b9d6f497 retained_bytes=2620 streams=2 events=3 status=correction_isolated_reference_only migration_disposition=semantic_migration_not_proved_identity_reference_insufficient authority=none_evidence_only
```

It passed 1/1 selected test in 0.01 seconds after a 22.63-second cold compile
and run, using `RUSTFLAGS=-D warnings`, offline locked Cargo, one job, one test
thread and an isolated target. The retained aggregate was 2,620 bytes across two
streams and three events; the isolated target measured 227,252,581 bytes.

Frozen ceilings were at most three inputs/packets/descriptors/baselines, two
streams, four deltas, twelve tests, sixteen binding/validation calls, 64 KiB per
canonical object, 32 KiB receipt, 180 seconds cold, 15 seconds warm and 2 GiB
target; exactly three residency receipts; no random/grid/network/database/UI or
runtime portfolio; and exactly one Cargo/test invocation.

While drafting the static verifier, two later Cargo invocations violated the
one-run ceiling:

| Invocation | Classification and outcome | Tool wall | Cargo time | Test time | Receipt |
|---|---|---:|---:|---:|---|
| 1 | authorized cold run; 1 passed | 22.63 s | included | 0.01 s | `e134a626977b0ca08dc3b6952b1864e07868bf1aab9a5853cf1e0867b9d6f497` |
| 2 | `unauthorized_zero_test_filter_invocation`; 0 ran, 6 filtered | 0.9 s | 0.54 s | 0.00 s | none |
| 3 | `unauthorized_repeated_selected_test_execution`; 1 passed, 5 filtered | 0.6 s | 0.15 s | 0.01 s | same receipt as invocation 1 |

The deviation classification is `one_run_ceiling_violated`. The repeated
receipt, counts, status, migration disposition and authority value were
identical, but repeatability does not repair the deviation, authorize another
run or widen the result. No formal pre-rerun source hash was captured. The
post-rerun disposable source SHA-256 was
`8b1b8262902ab9aa0c6f9b6ccd587119027e92983b7e4bfde778d706ca6ba6fe`;
the retained limitation is `no_formal_pre_post_source_hash_for_reruns`, so
source equality across the reruns is not claimed as cryptographically proved.

The disposable fixture performed no random or grid portfolio, network,
database, Forge Desktop, UI, renderer, runtime or child-process behavior. Its
source was deliberately not imported because the authorized audit froze a
no-production-import boundary; this record and exact hashes retain the result
and its limitation rather than turning it into a canonical executable proof.

### Red-team claim limits

This audit records `status=correction_isolated_reference_only` and
`authority=none_evidence_only`:

- `reconstruction_fingerprint` is caller supplied; the binder does not derive
  or compare it with `WorldGenerationInput.reconstruction_id`.
- `DependencyRef.kind` values are untyped integers. Fixture-local meanings do
  not establish a canonical registry or full dependency closure.
- Old and corrected baselines coexist. Nothing invalidates, retires, deletes,
  supersedes or automatically migrates the old baseline.
- `ReferenceOperation` is a test reducer, not reconstruction of C3 physics,
  production recovery, a save game or gameplay history.
- Materialization receipts contain caller-supplied residency and cost values;
  they prove no cache, storage, eviction, timing or runtime behavior.
- Equal current palette, exposure and signals do not establish byte-identical
  packets or physical-only, provenance-independent equivalence.
- Identity-reference migration is not a semantic correction validator. Any
  future migration requires separately owned semantics, replay equivalence,
  rollback and authority.
- The fixture proves no physical applicability, visibility, ecotone production
  behavior, biome, organism, renderer, runtime or player-facing result.

C3 remains executing with physical-applicability, visibility and ecotone
obligations open. C4 remains gated by C3. This audit does not activate C4,
release C5, close either item, promote a baseline, or grant implementation,
persistence, migration, runtime or protected-Kernel authority.
