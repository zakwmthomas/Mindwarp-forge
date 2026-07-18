# G1 C3 spatial-domain design

Date: 2026-07-15

Status: **implemented as bounded capability-free prototype evidence; promotion remains separate**.

## Decision

The smallest spatial seam compatible with current C3 evidence is a finite,
single-resolution, two-dimensional rectified sampling grid. Version 1 has:

- one explicit Q32.32 cell-center origin;
- one positive Q32.32 step per ordered axis;
- finite positive cell counts;
- canonical zero-based integer cell indices;
- edge-sharing four-neighbour adjacency inside the finite bounds; and
- an explicit `bounded_absent` boundary mode: outside cells do not exist and
  opposite edges never become neighbours.

This is a sampling-domain contract, not a model of a planet's shape. It binds
finite cells to the coordinate system that `field-basis` already samples. It
does not authorize a wrapped surface, sphere, projection, terrain mesh,
physical-region partition, biome, simulation or runtime representation.

## Local constraints

- `field-basis` samples an unbounded signed Q32.32 two-dimensional Cartesian
  plane. Its canonical policy rejects implicit numeric wrapping.
- `regional-environment-state` consumes one coordinate pair and has no domain,
  extent, level, cell or adjacency semantics.
- `universe-identity` owns typed hierarchical identity, including opaque
  Region and Site address payloads, but explicitly excludes coordinates from
  identity. Those address kinds cannot be reinterpreted as geometry.
- `addressable-world-binding` binds exact world-condition evidence to logical
  and reconstruction identity but supplies no spatial topology.

## Primary-standard reconciliation

The standards below constrain structure only. Their Earth reference systems,
global coverage rules, encodings and floating-point types do not transfer into
Forge's canonical contract.

- OGC Coverage Implementation Schema makes the domain set mandatory, defines
  its extent through ordered axes and lower/upper bounds, and distinguishes
  the domain's direct positions from its range values. This supports an
  explicit finite domain and ordered axes rather than an inferred array shape.
  Source: OGC 09-146r8, *Coverage Implementation Schema 1.1*, accessed
  2026-07-15, https://docs.ogc.org/is/09-146r8/09-146r8.html
- OGC's rectified-grid model uses an origin and offset vectors, and treats a
  grid point representing a sample space as the sample-space centre. This
  supports explicit cell-centre origin and step vectors without claiming
  physical polygon geometry. Source: OGC 08-085r4, *GML in JPEG 2000*,
  accessed 2026-07-15, https://docs.ogc.org/is/08-085r4/08-085r4.html
- OGC DGGS makes cell geometry/topology, identifiers, representative
  positions and refinement levels explicit. Its global model also demonstrates
  why Forge cannot silently adopt it: a globe geometry and global-domain
  reference system are prerequisites that C3 does not possess. Source: OGC
  20-040r3, *Discrete Global Grid Systems Part 1*, accessed 2026-07-15,
  https://docs.ogc.org/as/20-040r3/20-040r3.html

## Candidate comparison

| Candidate | Fit with current fields | Failure point | Decision |
|---|---|---|---|
| no domain | point sampling remains exact | no finite cells, adjacency or complete iteration | baseline only |
| bounded rectified grid | direct origin-plus-step mapping into Q32.32 | can be overclaimed as world geometry | select with strict nonclaims |
| wrapped rectified grid | coordinate mapping remains possible | invents cross-edge adjacency and can make field values discontinuous at the seam | reject from v1 |
| topology-only declared graph | arbitrary worlds are expressible | adjacency can contradict coordinate distance; field interpolation has no topology-aware meaning | reject from v1 |
| coordinate-anchored arbitrary graph | potentially compatible | larger codec, validation and consumer surface with no current need | defer until a consumer requires it |
| global/spherical tessellation | appropriate for some planet surfaces | requires a globe model, projection/refinement policy and new sampling semantics | dependency-blocked |

## Candidate contract

`SpatialDomainInputV1` contains only:

- `schema_version = 1`;
- exact `logical_world_id` and `reconstruction_id` references;
- fixed `coordinate_frame = field_q32_32_cartesian_2d_v1`;
- `cell_center_origin_q32_32: [i64; 2]`;
- `cell_step_q32_32: [u64; 2]`, both nonzero and no greater than `i64::MAX`;
- `cell_count: [u32; 2]`, both nonzero and bounded by a declared proof-resource
  ceiling;
- fixed `adjacency = shared_edge_4`; and
- fixed `boundary_mode = bounded_absent`.

The exact canonical descriptor bytes produce a domain-separated
`spatial_domain_id`. A cell is addressed only by `(x_index, y_index)` within
the descriptor bounds. Its domain-separated `cell_id` binds the
`spatial_domain_id` and canonical indices.

The sample coordinate for an index is calculated independently per axis:

`origin + index * step`

using checked `i128` intermediates and requiring the final value to fit `i64`.
Every descriptor must prove the maximum index on both axes is representable
before admission. No clamping, saturation, modulo or implicit wrap is allowed.

Neighbour enumeration is the canonical ordered subset of negative-x,
positive-x, negative-y and positive-y cells that remain in bounds. Diagonal
corner contact is not adjacency in v1. Enumeration and serialization order are
normative; caller-submitted cell or edge lists are never canonical facts.

## Identity and migration

Any change to origin, either step, either count, axis order, coordinate-frame
version, adjacency or boundary semantics changes `spatial_domain_id` and every
derived `cell_id`. Logical world identity does not change merely because a
sampling domain changes.

There is no ambient latest domain. A future wrapped, hierarchical, spherical,
irregular or coordinate-anchored graph domain requires a new schema and an
explicit migration/reconstruction receipt. V1 cell identities cannot be
silently reused across that change.

## Failure engineering

The validator and fixtures must reject:

- zero counts or steps, unsupported versions and unknown fields;
- coordinate overflow at the furthest cell before any cell is emitted;
- out-of-range, negative, aliased or swapped indices;
- self, diagonal, cross-boundary, wrapped, duplicate or asymmetric neighbour
  claims;
- caller-authored cell IDs, coordinates, cell lists or edge lists that differ
  from deterministic reconstruction;
- world/reconstruction mismatches with a consuming regional input;
- noncanonical bytes, reordered axes and identity drift; and
- attempts to label the descriptor as planet geometry, terrain, region
  membership, biome evidence or runtime authority.

The finite proof-resource ceiling is a validation and test-cost guard, not a
claim about production world size. A future lazy iterator may remove material
enumeration cost without changing descriptor or cell identity.

## Cheapest sufficient proof

The first capability-free implementation should prove:

1. strict descriptor and cell-index codecs;
2. exact origin and positive/negative-coordinate sample vectors;
3. corner, edge, interior and one-cell-domain neighbour vectors;
4. traversal-order-independent domain/cell identity;
5. every failure class above, including near-`i64` overflow;
6. exact binding into reconstructed `RegionalEnvironmentInput` coordinates;
7. no Kernel event, candidate, approval, promotion, runtime or filesystem
   authority; and
8. focused downstream tests before the complete Forge integration gate.

No visual or expensive simulation tier is justified. Typed in-memory fixtures
are sufficient until a later partition consumer exists.

## Readiness and next gate

The owner authorized the bounded implementation after its spherical nonclaim
was explained. `spatial-domain` and its regional-coordinate binding now pass
focused hostile tests. The retained evidence and nonclaims are recorded in
`G1_C3_SPATIAL_DOMAIN_RESULT.md`; no promotion or physical partition policy is
implied.
