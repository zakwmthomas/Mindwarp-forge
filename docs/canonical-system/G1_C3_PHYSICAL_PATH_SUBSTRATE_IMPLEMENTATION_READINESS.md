# G1 C3 Physical Path Substrate Implementation Readiness

Date: 2026-07-15

Status: **whole-plan aligned; exact owner action released; bounded
capability-free reference implemented and recorded separately.**

## Readiness decision

The finite 3D occupancy design and exact interval-plus-contact witness result
are implementation-ready for one small reference package. The audit found no
remaining behavior that must be invented in code after the corrections below.

The future reference must be a new `physical-path-substrate` crate. It is a
proof implementation of bounded semantics, not a production voxel engine. Its
first traversal algorithm must be the exhaustive exact closed-cell slab oracle
that passed the disposable portfolio. A faster DDA is deliberately deferred;
it may replace the internal algorithm only after exact oracle-equivalence tests
over the full hostile fixture portfolio.

The owner subsequently released the exact bounded action by instructing Forge
to continue. `G1_C3_PHYSICAL_PATH_SUBSTRATE_RESULT.md` records the implementation
and verification. This readiness document itself grants no approval, promotion
or runtime authority.

## Whole-plan alignment

The package fits `MASTER_PLAN_V2.md`, `DEPENDENCY_MAP.md`, `PROOF_MATRIX.md` and
the active C3 route because it supplies observer-independent physical occupancy
and path evidence before channel, probe, ecology, representation and runtime
meaning. It does not collapse those later consumers into one path score.

The owner's biome-fade correction is compatible and now a permanent shield:

- physical region identities and volume cells may index exact evidence but may
  not directly paint biome, palette, material or population seams;
- continuous climate, moisture, substrate and biological causes must produce
  deterministic ecotones at their own scales;
- a sharp transition remains sharp only when supported by sharp physical
  evidence and must not be blurred through a barrier or material interface; and
- the path substrate emits none of those biome or presentation decisions. A
  future biome consumer must retain the underlying continuous fields rather
  than infer fade weights from categorical region IDs or dominant cell phase.

The package leaves C4, C6, C7 and R1 gated and makes no C3 closure claim.

## Integration defects closed before code

### 1. Do not extend the 2D `SpatialDomain`

The existing `spatial-domain` crate is a bounded Q32.32 **2D sampling lattice**.
Changing it to 3D would mutate promoted identity, codec and downstream partition
semantics. The new volume therefore owns a separate fixed
`CartesianQ32_32Volume3dV1` coordinate-frame tag and has no dependency on
`spatial-domain` or `physical-region-partition`.

The shared Q32.32 convention is compatible arithmetic, not shared schema
identity. No automatic projection between a 2D sample cell and a 3D occupancy
cell exists in v1.

### 2. Remove impossible departure semantics

V1 accepts only endpoints inside the volume's closed rectangular domain. A
straight segment between two admitted endpoints cannot leave that convex
domain, so a `departure` witness state would be unreachable and ambiguous.
Outside endpoints fail before traversal. A future clipped ray or multi-volume
query requires a new versioned contract; it cannot overload v1.

### 3. Make stationary geometry authoritative

For `P0 == P1`, the geometric path has zero length although every parameter in
`[0,1]` maps to the same point. Every containing-cell record is `point` while
retaining the exact parameter preimage `[0/1,1/1]`. Record kind is authoritative
and cannot be inferred from parameter width. This preserves endpoint reversal
and permanently prevents the defect found by the first oracle run.

### 4. Keep simultaneous crossings unordered

Equal reduced rational parameters define one simultaneous event group. No
axis-priority, temporary DDA cell or insertion sequence becomes physical truth.
Stable bytes sort records by exact entry, exact exit and `(x,y,z)` cell index;
equal-parameter records are semantically unordered despite that serialization
order.

## Exact v1 input seam

### Physical volume recipe

`PhysicalVolumeRecipeInputV1` must contain only:

- `schema_version = 1`;
- nonzero 32-byte `recipe_source_id`, `scope_id` and `reconstruction_id`;
- positive `recipe_revision`;
- the fixed `CartesianQ32_32Volume3dV1` frame tag;
- signed Q32.32 `origin[3]`;
- one positive signed-Q32.32 `cell_step` for cubic v1 cells;
- positive `extent[3]` cell counts;
- fixed bounded-absent boundary mode and six-face adjacency tags;
- one default `CellEvidenceV1`; and
- canonical sparse `ColumnRunV1` overrides.

The cubic step is an explicit v1 simplification. An anisotropic, rotated,
curved, wrapped, joined or hierarchical volume requires a new schema and
migration evidence.

`CellEvidenceV1` is a closed enum:

- `unavailable`;
- `vacuum`; or
- `gas`, `liquid` or `solid`, each with one nonzero 32-byte
  `substance_source_id`.

Unavailable is not vacuum. Phase and substance identity do not imply opacity,
permeability, acoustic impedance, chemical transfer, toxicity, strength,
support, friction, passability, biome or visual material.

### Canonical sparse columns

Each run contains `(x_index, y_index, z_start, length, evidence)`. Runs are
sorted by `(x,y,z_start)`, positive-length, in range, non-overlapping and
different from the default. Adjacent runs with equal evidence must be merged;
an override cannot cross a column. Empty override lists are valid. Any other
syntax is noncanonical even if it reconstructs the same cells.

Recipe validation checks `extent_x * extent_y * extent_z` before allocation,
then checks `origin + cell_step * extent_axis` in `i128` and requires every
closed outer bound to fit signed Q32.32 `i64`.

### Path query

`PhysicalPathQueryV1` binds:

- `schema_version = 1`;
- the exact validated volume identity; and
- two signed Q32.32 endpoint triples.

Both endpoints must lie within the volume's closed outer bounds. Internal grid
planes and the maximum outer faces are valid endpoint locations. No caller may
submit cells, crossings, intervals, contacts or a traversal order.

## Exact reconstructed output

### Volume result

`PhysicalVolumeV1` contains the exact recipe identity, reconstruction identity,
checked cell count, provenance-bound volume identity, provenance-independent
occupancy fingerprint, fixed limitations and authority-negative claims. The
occupancy fingerprint includes frame, origin, step, extent, boundary mode and
every reconstructed cell evidence value; it excludes recipe source, scope and
revision. Equality does not imply interchangeable planets, biomes or runtime
chunks.

Cells are reconstructed in `(x,y,z)` lexicographic order. `PhysicalCellV1`
contains its exact index, closed Q32.32 bounds, evidence and content-derived
identity. Caller-supplied reconstructed cells are never trusted.

### Exact rational parameters

`UnitRationalV1` stores a reduced `u64 numerator / u64 denominator` with a
nonzero denominator, numerator not greater than denominator, greatest common
divisor one and canonical zero `0/1`. Differences between two admitted `i64`
coordinates are computed in `i128`, have magnitude at most `u64::MAX`, and fit
the record. Rational comparison uses checked `u128` cross-products of the two
`u64` pairs; no float, epsilon or rounding enters identity.

### Path witness

`PhysicalPathWitnessV1` binds the exact query and volume identities and a
duplicate-free ordered list of `CellPathRecordV1` values. Each record contains:

- exact cell identity and `(x,y,z)` index;
- reduced rational `t_enter` and `t_exit`; and
- closed `PathIntersectionKindV1::{Interval, Point}`.

For non-stationary paths, `Interval` requires `t_enter < t_exit`; isolated
contacts require equality. For stationary paths, all containing cells are
`Point` with `[0/1,1/1]`. Equal rational endpoints expose simultaneous face,
edge and vertex groups without a stored group ID. Cell bounds plus `P(t)` make
the contact axes exactly reconstructible, so a second contact-class field would
be duplicate truth.

Reversing a non-stationary query preserves cell and kind and maps `[a,b]` to
`[1-b,1-a]`. Reversing a stationary query is the identical canonical query and
retains `[0/1,1/1]` point records.

The public witness validator recompiles the volume and exhaustive oracle from
the exact recipe and query, then compares the complete result. Forged record
lists cannot obtain canonical bytes through the public API.

## Bounded reference algorithm and cost

V1 freezes:

- `MAX_PHYSICAL_VOLUME_PROOF_CELLS = 65_536`;
- `MAX_PHYSICAL_VOLUME_RUNS = 65_536`; and
- `MAX_PATH_WITNESS_RECORDS = 65_536`.

The first reference reconstructs at most `N` cells and applies exact closed-slab
intersection to every cell. A cell is convex, so it contributes at most one
record. Reconstruction, validation and one query are `O(N)`; witness length is
at most `N`. Sparse source size is `O(R)` with `R <= N`. No recursion or
allocation occurs before checked counts and coordinate bounds pass.

This deliberately pays bounded proof cost to avoid DDA tie ambiguity. The
implementation receipt must report maximum-fixture runtime, peak or estimated
resident bytes, recipe/result/witness canonical byte counts and test time on
the 65,536-cell ceiling. If the receipt is excessive, reduce the proof ceiling
instead of weakening validation, returning partial evidence or installing a
lossy optimized traversal.

A future optimized traversal may retain the v1 schema only if exhaustive-oracle
equivalence passes for every permanent fixture plus deterministic generated
small-volume comparisons. Otherwise it requires a new candidate and cannot
silently replace the reference.

## Mandatory public surface

The smallest permitted surface is:

- strict `to_bytes` and `from_bytes` for recipe, query, volume and witness;
- `compile_physical_volume` and `validate_physical_volume`;
- `compile_path_witness` and `validate_path_witness`; and
- read-only cell reconstruction helpers required by tests.

Strict decoders reject unknown fields, duplicate JSON keys where the chosen
codec exposes them, trailing bytes, whitespace variants, reordered object
fields, noncanonical run ordering, non-reduced rationals and any bytes that do
not exactly equal re-encoding. There is no mutation, partial update, cache
acceptance, ambient latest recipe or external capability.

## Required hostile proof portfolio

The implementation package must independently prove:

1. strict deterministic recipe, query, volume and witness replay;
2. default-only, one-cell, one-row, one-column, one-layer and uniform volumes;
3. hollow, stacked, overhang, cave, checkerboard and maximum-cell volumes;
4. unavailable distinct from vacuum and non-vacuum substance identity required;
5. zero identity, revision, step or extent and unknown enum/schema rejection;
6. extent multiplication, coordinate, count, rational comparison and allocation
   overflow before output;
7. unsorted, overlapping, mergeable, zero-length, default-equivalent,
   cross-column and out-of-range runs;
8. recipe source, scope, revision, reconstruction, transform, extent, step,
   boundary or evidence changes rekeying the correct identities;
9. semantic occupancy equality across provenance-only recipe changes;
10. omitted, duplicate, reordered, foreign or forged reconstructed cells;
11. outside endpoints rejected and closed minimum/maximum endpoints admitted;
12. axis, face-aligned, edge-aligned and through-vertex paths;
13. internal boundary starts/ends and simultaneous two/three-axis crossings;
14. forward/reverse equivalence with exact rational mapping;
15. zero-length interior, face, edge and vertex queries producing only points;
16. thin-barrier crossing versus face/edge/vertex-only contact;
17. no out-of-domain phantom cells at outer boundaries;
18. no duplicate witness cell and witness count at or below cell count;
19. bare single-owner and undifferentiated all-contact negative controls;
20. exhaustive oracle equivalence for deterministic generated small volumes;
21. forged query, interval, kind, order, identity, limitation and authority data;
22. unknown fields, noncanonical ordering, whitespace, trailing bytes and
    unreduced or zero-denominator rationals;
23. measured maximum-ceiling reconstruction, witness and canonical-byte cost;
24. absence of opacity, transfer, visibility, detectability, passability,
    clearance, route, biome, ecotone, organism, planet, terrain, storage,
    runtime, approval or promotion claims; and
25. a later biome-consumer negative fixture proving categorical region or cell
    identity alone cannot produce palette/material weights or a visible seam.

## Permanent integration shield

An authorized implementation must:

- add only `crates/physical-path-substrate` to the workspace;
- use local `serde`, `serde_json` and `sha2` patterns with no local crate
  dependency in v1;
- add capability prohibitions and an empty dependency list to
  `governance/module-boundaries.json`;
- add a generated first-read `MODULE.md` through
  `governance/module-context-registry.json`;
- add `contracts/physical-path-substrate-contract.md` and a bounded result
  record;
- extend the C3 verifier to require the contract, result, failure-correction
  tokens, hostile test names and `cargo test -p physical-path-substrate`;
- run warnings-denied focused tests plus module context, modularity, master,
  canonical, record-role, bootstrap and complete repository verification; and
- preserve the inherited dirty tree and use the established isolated desktop
  build route if a live executable lock blocks the ordinary build.

No focused test substitutes for the complete Forge gate. No module may import
Forge Kernel, Tauri/UI, filesystem, process, network, persistence, runtime,
ecology, representation or engine capabilities.

## Rollback and migration

The implementation is additive. Removing the new crate, contract/result,
workspace/governance declarations and verifier additions restores the safe
fallback: no canonical 3D occupancy volume and no path claim. Existing 2D
spatial, physical-region and derived-world identities remain byte-identical.

Recipe or evidence changes rebuild new identities; caller patches do not
migrate them. Anisotropic, rotated, curved, wrapped, multi-volume, hierarchical,
mesh-derived, runtime-streamed or clipped-ray forms need new schemas and
explicit migration/equivalence evidence.

## Exact owner action

This audit prepared the following bounded action, which the owner subsequently
released through a direct continuation instruction:

> Authorize the capability-free `physical-path-substrate` v1 reference exactly
> as bounded by `G1_C3_PHYSICAL_PATH_SUBSTRATE_IMPLEMENTATION_READINESS.md`,
> with exhaustive exact traversal, all hostile and cost receipts, and no C3
> closure, promotion, planet, biome, consumer or runtime authority.

The action produced the additive bounded reference recorded in
`G1_C3_PHYSICAL_PATH_SUBSTRATE_RESULT.md`. Verification, promotion, C3 closure
and all excluded downstream work remain separate gates.
