# Physical Path Substrate Contract v1

This capability-free contract reconstructs one finite, cubic, non-wrapping
Q32.32 Cartesian 3D occupancy evidence volume and exact conservative segment
witnesses. It is independent of the existing 2D `SpatialDomain`; no projection
or identity migration between those schemas is implied.

The strict recipe binds source, scope, reconstruction, revision, origin,
positive cell step, positive three-axis extent, bounded-absent faces, six-face
adjacency, one default cell value and canonical sparse `(x,y,z)` column runs.
The proof caps cells, runs and witness records at 65,536. Coordinate products
and outer bounds are checked before allocation.

Cell evidence is exactly unavailable, vacuum, gas, liquid or solid. Gas, liquid
and solid require a nonzero substance-source identity. Unavailable never means
vacuum. Dominant phase implies no opacity, transfer, toxicity, support,
friction, passability, biome or visual material.

Both path endpoints must lie inside the closed convex volume. The exhaustive
reference intersects the closed segment with every closed in-domain cell using
checked integer differences, reduced `u64/u64` unit rationals and `u128`
comparisons. Each touched cell appears once with exact entry/exit parameters
and a closed kind: positive geometric-length `interval`, or zero-length
`point`. Stationary queries always emit point records even though their exact
parameter preimage is `[0/1,1/1]`. Simultaneous crossings are semantically
unordered; stable bytes sort only by exact entry, exit and cell index.

Recipe, volume, query and witness codecs are strict and canonical. Volume and
witness validators rebuild complete expected results rather than trusting
serialized cells or crossings. Provenance-bound volume identity remains
separate from a semantic occupancy fingerprint.

## Additive conditional interval cell-step v1

The substrate also owns one separately versioned, channel-neutral conditional
cell-step proof. A caller supplies a validated recipe and volume plus one
current cell, a declared Q160 point box and a declared Q1.62 direction box.
The proof neither imports optical records nor claims ray, endpoint or source
lineage.

The implementation uses checked signed-magnitude arithmetic in fixed 512-bit
storage at exactly 160 fractional bits. A face is certified only when its
finite outward time upper bound is strictly below every competing possible
face lower bound. Zero-time candidates return `no_forward_progress`; an absent
unique winner returns `ambiguous_next_face`. Certified evidence distinguishes
an available neighbour, an unavailable neighbour and a bounded outer-domain
exit without fabricating an unsigned neighbour index.

Input and event codecs reject unknown or noncanonical data and enforce 16 KiB
and 32 KiB pre-decode and post-encode ceilings. The arithmetic receipt caps
live signed values at 414 bits, candidates at six, directed divisions at 12,
strict comparisons at 30 and propagated tangential axes at zero or two.
Validation fully recompiles the event. Existing exact-path V1 canonical bytes
and identities are protected by five permanent compatibility families.

This contract does not define a sphere, planet, terrain, global topology,
production voxel store, streaming, meshing, runtime map, opacity, propagation,
visibility, detectability, collision, clearance, passability, navigation,
route cost, biome, ecotone, organism, palette, presentation, approval,
promotion, persistence or external capability. Continuous biome causes must
still fade in later consumers; categorical cells or regions cannot create a
visible seam, and sharp blending boundaries require sharp physical causes.
