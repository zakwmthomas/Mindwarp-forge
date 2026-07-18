# G1 C3 physical partition implementation readiness

Date: 2026-07-15

Status: **whole-plan aligned and owner-authorized for the corrected bounded
capability-free v1 implementation; verification and promotion remain separate**.

## Readiness decision

The total-signature-then-connectivity design in
`G1_C3_PHYSICAL_PARTITION_POLICY_DESIGN_AUDIT.md` is exact enough for one small
reference implementation. The package can be implemented without inventing a
distance metric, clustering objective, planet topology, biome vocabulary or
caller-authored region data.

The implementation must be a new `physical-region-partition` crate. It may
depend only on:

- `spatial-domain` for exact cells and shared-edge neighbours;
- `regional-environment-state` for coordinate-bound exposure and moisture
  potential reconstruction; and
- `climate-state` for exact absorbed-energy availability and its nested
  validated surface-accessible-liquid evidence.

It must not depend on the Forge Kernel, desktop/Tauri, niche or organism
modules, filesystem, process, network, persistence, runtime or engine code.

## Whole-plan alignment result

The owner required this candidate to be checked against the overall plan and
stated that it is okay if it fits. The comparison covered `MASTER_PLAN_V2.md`,
`DEPENDENCY_MAP.md`, `PROOF_MATRIX.md`, `system-registry.json`, C3/C4/C6 and the
gated R1 runtime adapter.

The corrected candidate fits because:

- G1 requires physical world evidence before C6 biome ecomorph, niche,
  organism and representation meaning;
- C3 owns causal physical worlds and physical biome structure, while C6 owns
  ecological roles, occupancy, ecomorphs and organisms;
- deterministic physical components give later ecology spatially distinct
  evidence without naming or inferring that ecology;
- canonical recipes remain engine-neutral while runtime materialisation stays
  downstream;
- C4 remains gated on complete C3 rather than this one component; and
- R1 receives no topology, terrain, storage, streaming or engine decision.

The audit also found and corrected two integration defects before code:

1. `RegionalFieldBindingV1` belongs to `regional-environment-state`, which
   already owns field-source and recipe semantics. The partition crate consumes
   that upstream type rather than duplicating it.
2. A direct hydrology-only gate is insufficient for the causal plan. The exact
   `ClimateContract` gates exposure availability from absorbed shortwave energy
   and moisture availability from its nested hydrological evidence.

This authorizes a bounded C3 reference only. It does not establish scientific
biome sufficiency, close C3, unlock C4, enter C6 or select R1.

## Exact v1 input seam

### Coordinate-free regional binding

The crate must not accept an existing `RegionalEnvironmentInput` as a template
because its coordinate would be irrelevant to the partition yet would remain
identity-bearing. That alias is engineered out with an upstream
`regional_environment_state::RegionalFieldBindingV1` containing only:

- `schema_version = 1`;
- exact `reconstruction_id`;
- nonzero `regional_source_id` and `moisture_source_id`; and
- strict canonical exposure and moisture field-recipe bytes.

For every reconstructed spatial cell, the compiler constructs a fresh
`RegionalEnvironmentInput` with that cell's exact Q32.32 coordinate and calls
the existing regional compiler. No caller-supplied sampled value is admitted.

### Closed physical dimensions

`PartitionDimensionV1` is a closed enum with exactly:

- `regional_exposure_permille`; and
- `regional_moisture_potential_permille`.

The dimensions remain separate. Moisture produces the distinct signature value
`unavailable` whenever the nested exact hydrological evidence says there is no
surface-accessible liquid. Exposure is likewise `unavailable` when the exact
validated `ClimateContract` reports zero absorbed shortwave energy. Numeric
zero is never an alias for unavailable. All domain, field-binding and climate
reconstruction identities must match before the first cell is classified.

### Closed classifiers

Each unique ordered dimension has exactly one classifier:

- `exact_value`; or
- `lower_bound_cuts(Vec<u16>)`.

Cuts must be non-empty, strictly increasing and inside `1..=1000`. The bin is
the number of cuts less than or equal to the value. Empty, duplicate,
descending, out-of-range or unknown classifier data fails closed. There is no
local tolerance, centroid, mean, weighted score, distance, seed, region count,
merge callback or executable expression.

### Recipe identity and provenance

`PhysicalPartitionRecipeInputV1` contains:

- `schema_version = 1`;
- nonzero `recipe_source_id` and `scope_id`;
- positive authored `recipe_revision`;
- the non-empty duplicate-free ordered dimension rules; and
- fixed applicability versions for spatial domain, regional state and
  climate evidence.

Strict canonical recipe bytes produce `physical_partition_recipe_id`. Human
labels are not canonical semantics. The source and scope IDs must resolve to
content provenance outside this capability-free crate; they grant no approval.
Changing source, scope, revision, dimension order, classifier or cuts rekeys the
recipe.

`PhysicalPartitionInputV1` binds the exact validated spatial domain,
coordinate-free regional binding, validated climate contract and exact recipe.
Its identity changes with any of them.

## Exact output and reconstruction

The output owns only:

- one canonical physical signature per domain cell;
- exhaustive connected physical components;
- each component's signature, sorted member cell identities and boundary
  neighbour component identities;
- exact source/domain/recipe identities;
- fixed limitations and authority-negative claims; and
- content-derived input, component and partition identities.

Cells are enumerated in ascending `(x_index, y_index)` order. An edge exists
only between exact reconstructed `shared_edge_4` neighbours with byte-identical
signatures. Components are sorted by their smallest member index; members and
boundary references are sorted and duplicate-free. Every domain cell must
appear exactly once. Disconnected equal signatures remain different component
identities.

The public validator must recompile the complete result from the exact input
and compare it. Serialization must require that exact input validation so a
forged result cannot produce canonical bytes through the public API.

## Bounded cost

V1 sets `MAX_PARTITION_PROOF_CELLS = 65_536`. This is a prototype memory and
test-cost guard, not a production world-size, storage or geometry claim. It is
stricter than the spatial-domain ceiling because a checkerboard can create one
component per cell and retain membership plus boundary evidence.

For `N` admitted cells, the implementation must use checked accounting and
remain `O(N)` in reconstructed cells plus shared edges. A bounded rectified grid
has fewer than `2N` undirected shared edges. Component count is at most `N`.
Any count conversion, allocation estimate, cell total, edge total or identity
buffer overflow fails before output. No recursion proportional to component
size is permitted; use bounded iterative traversal or deterministic union-find.

An implementation receipt must report the measured test cost at the ceiling.
If that cost is excessive, reduce the proof ceiling rather than weakening
validation or silently using lazy partial membership.

## Mandatory public surface

The smallest permitted public surface is:

- strict `to_bytes` / `from_bytes` for recipe input, partition input and
  partition result;
- `compile_physical_partition`;
- `validate_physical_partition`; and
- read-only reconstruction helpers only where tests require them.

There is no mutation, incremental patch, cached acceptance, caller label,
external capability or ambient latest recipe in v1.

## Required hostile proof portfolio

The implementation package must pass at least these independent failures:

1. deterministic strict recipe/input/result replay;
2. exact-value uniform, checkerboard and disconnected-island components;
3. authored cut values immediately below, at and above every boundary;
4. explicit `unavailable` moisture versus numeric zero;
5. reconstruction mismatch across domain, field and hydrology;
6. missing, duplicate, reordered and unsupported dimensions;
7. empty, duplicate, descending and out-of-range cuts;
8. proof-ceiling, checked cell/edge accounting and worst-case component count;
9. no diagonal or cross-boundary wrap connectivity;
10. source-order and traversal-order invariant canonical output;
11. omitted, duplicate, foreign and cross-signature member forgery;
12. forged component boundary, signature, identity, limitations and authority;
13. domain, evidence, availability and recipe changes rekeying exact outputs;
14. unknown fields, whitespace, noncanonical ordering and schema drift; and
15. absence of biome, habitat, organism, planet, runtime or authority claims.

The `0,5,10` tolerance-chain case remains a negative API test: no v1 type or
codec may express the rejected local-tolerance mode.

## Permanent integration shield

Implementation must also:

- add the crate to the workspace;
- declare the three dependencies and capability prohibitions in
  `governance/module-boundaries.json`;
- add a generated `MODULE.md` through
  `governance/module-context-registry.json`;
- update `regional-environment-state` module context for its new upstream-owned
  coordinate-free binding without changing existing point-input bytes;
- add a strict canonical contract and bounded result record;
- extend `tools/verify-g1-c3-derived-world.ps1` to require the contract, result,
  source tokens and hostile-test names and to run
  `cargo test -p physical-region-partition`;
- pass module context, modularity, master program, canonical, record-role and
  complete repository verification; and
- use an isolated warnings-denied desktop build if the known live executable
  lock blocks the ordinary final build.

No focused test can substitute for the full integration shield.

## Rollback and migration

The implementation is additive. Reverting the new crate, its workspace and
governance declarations, contract/result records and verifier additions returns
Forge to the permanent no-partition fallback without rewriting spatial-domain
or regional identities.

There is no migration from a recipe change in v1. A changed recipe or source
rebuilds a new partition identity. A future wrapped, hierarchical, spherical or
irregular domain requires a new spatial schema and explicit migration evidence;
v1 component identities cannot transfer silently.

## Explicit nonclaims

The output is not a sphere, planet surface, continent, terrain, watershed,
weather cell, biome, habitat, hazard, resource score, vegetation, niche,
organism, lineage, aesthetic region, visibility field, traversability map,
storage shard, stream chunk, runtime simulation, approval or promotion.

Passing this readiness gate proves only that a bounded deterministic reference
can be implemented without unresolved semantic invention.

## Owner authorization receipt

The owner instructed Forge to double-check this work against the overall plan
and stated that if it fits, it should be okay. The whole-plan audit above finds
that the corrected candidate fits. The selected action is therefore the
bounded v1 implementation below. The no-partition path remains its rollback;
verification, promotion and every excluded scope remain unauthorized.

## Exact bounded decision

- **Selected - bounded v1 implementation:** create only the capability-free
  `physical-region-partition` crate and shields specified above, with no named
  recipe content or biome semantics.
- **Rollback - retain no partition:** remove the additive crate and its
  declarations while retaining the audited design as future evidence.

This authorization does not select authored cut values, promote the prototype,
close C3 or authorize later ecology/runtime work.
