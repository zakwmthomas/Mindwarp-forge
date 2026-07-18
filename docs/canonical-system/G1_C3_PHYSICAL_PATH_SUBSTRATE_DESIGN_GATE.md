# G1 C3 Physical Path Substrate Design Gate

Date: 2026-07-15

Status: **bounded representation selected; exact proof, readiness and
owner-authorized reference implementation recorded.**

## Decision

Select a finite, non-wrapping **three-dimensional Cartesian occupancy evidence
volume** as the semantic reference for the remaining C3 path substrate. Its
canonical source form should use exact sparse column runs, but sparsity is an
encoding choice: the contract means a bounded 3D field of reconstructed cell
evidence.

This is the smallest compared alternative that can represent positive free
space, solids, gas and liquid media, stacked surfaces, overhangs, caves and
volumetric routes without assuming an organism. It can later support:

- ordered segment-through-cell evidence for channel-specific propagation;
- surface and medium boundary evidence for physical visibility queries; and
- clearance/continuity evidence consumed by a separately declared generic
  probe or movement envelope.

The selected representation is not a planet, sphere, projection, global
topology, terrain engine, voxel game, navigation mesh, storage layout,
streaming policy or runtime map. V1 is one finite Cartesian proof volume with
bounded-absent edges. No wrapping or relationship among multiple volumes is
defined.

The exact disposable result now distinguishes positive geometric-length cell
intervals from face/edge/vertex-only contacts, makes simultaneous crossings
unordered, and proves hostile cases without floating-point dependence. See
`G1_C3_PATH_WITNESS_COUNTEREXAMPLE_RESULT.md`. The refreshed readiness audit is
recorded in `G1_C3_PHYSICAL_PATH_SUBSTRATE_IMPLEMENTATION_READINESS.md`; the
owner-authorized bounded result is recorded in
`G1_C3_PHYSICAL_PATH_SUBSTRATE_RESULT.md`.

## Primary-practice constraints

These sources constrain architecture only. Forge does not adopt their runtime,
sensor, probability, rendering, navigation or threshold policies.

- LaValle defines collision-free motion in the configuration space of a
  particular body. This supports keeping world occupancy separate from a body
  or probe-specific validity result. Source: Steven M. LaValle, *Planning
  Algorithms*, sections 4.2-4.3,
  https://lavalle.pl/planning/node144.html and
  https://lavalle.pl/planning/node156.html
- OMPL deliberately leaves state validity and motion validation to the
  problem integration, and warns that discrete motion checks can miss invalid
  states when resolution is too coarse. This supports a generic physical
  substrate plus separately typed probe validation and explicit resolution
  limits. Source: Open Motion Planning Library, *State Validity Checking*,
  https://ompl.kavrakilab.org/stateValidation.html
- PBRT treats surface intersection and participating-medium transmittance as
  separate operations along an explicit ray. This supports recording geometry
  and medium evidence in the substrate while leaving optical transmittance to
  a channel-specific consumer. Source: *Physically Based Rendering*,
  *Media* and *Light Interface*,
  https://www.pbr-book.org/3ed-2018/Volume_Scattering/Media and
  https://pbr-book.org/3ed-2018/Light_Sources/Light_Interface
- OctoMap demonstrates why full 3D occupancy distinguishes free, occupied and
  unknown space and can represent arbitrary multi-level environments. Forge
  rejects its sensor-updated probability semantics and dynamic unbounded
  extent; v1 needs exact deterministic generated evidence and a finite proof
  ceiling. Source: OctoMap project and paper,
  https://octomap.github.io/
- Recast's compact heightfield stores unobstructed spans but also carries
  `walkableHeight` and `walkableClimb`. That is useful consumer evidence and a
  reason not to adopt a navigation heightfield as canonical world truth:
  walkability is already probe-specific. Source: Recast Navigation,
  https://recastnav.com/structrcCompactHeightfield.html
- OpenVDB explicitly separates sparse 3D storage from interpretation and binds
  index space to physical space through a transform. Forge reuses only that
  separation principle, not OpenVDB or its runtime data structure. Source:
  OpenVDB overview,
  https://www.openvdb.org/documentation/doxygen/overview.html
- The original Amanatides-Woo result shows ordered traversal through a uniform
  3D voxel partition is practical, but its floating-point presentation and
  ordinary tie behavior are not automatically Forge-canonical. Source: John
  Amanatides and Andrew Woo, *A Fast Voxel Traversal Algorithm for Ray
  Tracing* (1987),
  https://physique.cmaisonneuve.qc.ca/svezina/projet/ray_tracer/download/A_Fast_Voxel_Traversal_Algorythm_For_Ray_Tracing.pdf

## Alternative comparison

| Alternative | What it can answer | Decisive failure | Decision |
|---|---|---|---|
| enriched 2D cells and edges | bounded surface adjacency and authored edge facts at low cost | cannot represent vertical occupancy, stacked routes, overhangs, caves, volumetric media or honest 3D occlusion | reject as shared substrate; retain existing `SpatialDomain` for sampling and partition proofs |
| single-height 2.5D surface | elevation, local slope and ground-following continuity | one height per horizontal coordinate cannot represent bridges, tunnels, caves, stacked surfaces or general volumetric media; it silently selects terrain semantics | reject as general substrate; a future ground-surface projection may consume 3D evidence |
| bounded 3D occupancy | free/occupied/media evidence, multi-level geometry, segment occupancy and volumetric clearance | highest proof memory and aliasing risk; requires exact resolution and traversal policy | select with strict finite ceiling and sparse canonical source form |
| caller-declared physical graph | arbitrary topology and cheap connectivity queries | edges can hide geometry, clearance and medium assumptions; cannot reconstruct continuous segment intersections or prove caller edges physically | reject as source of truth; derived graphs may be rebuilt from validated volume evidence |

### Why not 2.5D as the initial compromise

A single-valued height field looks cheaper because the current field basis is
2D, but it bakes a ground-surface assumption into the common contract and still
cannot answer the visibility and volumetric-media questions that selected this
package. Multi-level height spans approach sparse 3D occupancy semantics while
retaining column compression. The honest common abstraction is therefore a
bounded 3D occupancy volume whose source encoding may be column-oriented.

### Why this is not a voxel-world decision

The contract selects proof semantics, not a production representation. A
future runtime may reconstruct equivalent physical evidence from meshes,
signed-distance fields, constructive geometry, procedural recipes, sparse
volumes or another promoted form. Cache/storage layout and rendering remain
noncanonical. No volume joins, planetary curvature, global addressing,
streaming or meshing policy enters v1.

## Selected semantic boundary

The implementation candidate should eventually separate four strict records.
Names below are design labels, not authorized schemas.

### 1. Physical volume recipe

The recipe binds:

- schema, recipe source, scope, revision and reconstruction identity;
- a finite 3D Q32.32 Cartesian origin, positive cell step and three checked
  cell counts;
- bounded-absent boundary mode and closed six-face adjacency;
- an explicit default cell evidence value; and
- canonical non-overlapping maximal column runs that override the default.

The product of cell counts must have a small proof ceiling checked before any
allocation or reconstruction. V1 should reuse the current 65,536-cell ceiling
unless a disposable cost receipt justifies a different bound.

### 2. Reconstructed cell evidence

Every cell reconstructs from the recipe and has exact index, bounds and cell
identity. Cell evidence is either unavailable or an available dominant phase:

- vacuum;
- gas;
- liquid; or
- solid.

Available non-vacuum evidence binds a nonzero substance/source identity. Phase
does not imply opacity, permeability, acoustic impedance, chemical diffusion,
toxicity, support strength, friction or passability. One dominant phase per
cell is a v1 discretization limit, not a physical claim that mixtures or
partial cells do not exist.

Unavailable is distinct from vacuum. Unknown evidence must never become free
space by default.

### 3. Physical volume result

The result binds exact recipe identity and reconstruction, exposes a semantic
occupancy-content fingerprint separately from provenance-bound volume
identity, and replays every cell from the recipe. It does not serialize caller
membership, derived visibility, traversal cost or propagation output as trusted
facts.

### 4. Path witness

A query binds exact volume identity and two in-domain Q32.32 endpoints. The
future witness must reconstruct all intersected cells and physical boundary
crossings under one versioned conservative policy. It must expose enough exact
parametric interval/contact evidence for a channel consumer to distinguish:

- positive-length travel through a cell;
- contact with only a face, edge or vertex;
- simultaneous crossings on multiple axes;
- an endpoint exactly on a boundary; and
- departure from the bounded domain.

The witness owns geometry traversal only. It does not calculate attenuation,
signal range, optical visibility, biological detectability, collision-free
motion, clearance, walkability, traversal cost or a route.

## Identity and ownership shields

- Volume provenance identity changes with source, scope, revision,
  reconstruction, transform, extent, resolution, boundary mode or evidence.
- A separate semantic occupancy fingerprint may ignore source provenance but
  must include the exact coordinate frame, bounds, resolution and reconstructed
  cell evidence. Equal fingerprints do not imply interchangeable planets,
  biomes, runtime chunks or storage.
- Sparse-run syntax must be canonical: sorted columns and starts, positive
  lengths, no overlap, no adjacent equal runs that should be merged, no
  out-of-domain cells and no run identical to the declared default.
- The volume module owns physical occupancy evidence and exact segment
  traversal. Channel modules own attenuation/transfer. Probe consumers own
  clearance, collision and motion validity. C6 owns organism bodies, senses and
  behavior.
- A partition component may reference volume scope later, but region
  signatures do not generate occupancy and occupancy does not retroactively
  change partition semantics.

## Adversarial findings and fixtures

Implementation readiness requires permanent failures for:

- zero identity, step or extent; multiplication, coordinate and allocation
  overflow; proof-ceiling breach before allocation;
- reconstruction, scope, recipe, transform, extent or resolution mismatch;
- overlapping, unsorted, mergeable, zero-length, default-equivalent or
  out-of-range column runs;
- unknown phase/schema fields, noncanonical bytes and fabricated content or
  authority claims;
- treating unavailable cells as vacuum or free space;
- one-cell, one-row/column/layer, uniform, hollow, stacked, cave, overhang,
  checkerboard and maximum-proof-size volumes;
- path endpoints outside the domain, zero-length paths and axis-aligned paths;
- paths along a face or edge, through a vertex, starting/ending on a boundary,
  reverse endpoint order and simultaneous two- or three-axis crossings;
- thin barriers and diagonal corner contacts that a nonconservative sampler
  could skip;
- traversal-order or sparse-encoding changes altering semantic cell evidence;
- caller-authored visibility, propagation, passability, navigation, planet,
  runtime, approval or promotion claims.

## Cost, recovery and rollback

- V1 remains bounded by total reconstructed cells and path-witness length.
- Canonical semantics use integer coordinates and exact checked arithmetic;
  sparse runs reduce source bytes but do not change the cell ceiling.
- Rebuild from recipe is the recovery path. Derived cell caches, path witnesses
  and consumer outputs are disposable.
- The previous safe fallback is no physical volume and no propagation,
  visibility or passage claim. Existing 2D spatial and partition evidence
  remains valid and unchanged.
- No filesystem, network, process, runtime, Kernel authority, approval,
  promotion or persistence capability belongs in the future module.

## Resolved path-witness blocker

`G1_C3_PATH_WITNESS_COUNTEREXAMPLE_RESULT.md` records the passed disposable
exact-rational portfolio. It selects one record per closed in-domain cell
intersection, distinguishes positive geometric-length intervals from point-only
contacts, treats simultaneous crossings as unordered exact parameter groups,
includes endpoints without phantom outer cells, proves reversal, and bounds the
witness by the reconstructed-cell ceiling. The first run also found and removed
a stationary-path parameter-width trap through a permanent zero-length fixture.

## Authority gate

This design document alone authorizes no crate or schema implementation. The
separate readiness action was released and produced the bounded reference
result without changing these nonclaims. No sphere, planet,
terrain engine, runtime map, navigation mesh, propagation model, organism
binding, C3 closure or promotion is requested here.
