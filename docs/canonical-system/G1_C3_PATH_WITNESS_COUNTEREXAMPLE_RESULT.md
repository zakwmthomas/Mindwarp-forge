# G1 C3 Path Witness Counterexample Result

Date: 2026-07-15

Status: **exact witness semantics selected; disposable proof passed; no crate or
schema implementation authorized.**

## Result

Use one exact record per in-domain cell whose **closed physical cell volume**
intersects the closed segment. Each record contains the cell identity and a
reduced rational entry/exit parameter pair. Its kind is:

- `interval` when a non-stationary segment occupies that closed cell over
  positive geometric length; or
- `point` when the intersection has zero geometric length, including isolated
  face/edge/vertex or endpoint contact and every stationary-path record.

This interval-plus-contact witness is the minimum sufficient output among the
three compared forms. It retains everything in a conservative all-contact cell
set while preventing consumers from mistaking a point touch for traversed
medium. It also retains cells that an ordinary single-owner DDA-like traversal
omits at faces, edges, vertices and boundary endpoints.

The result fits the overall Forge plan. It supplies observer-independent
physical evidence only. A channel consumer still decides transfer or
attenuation; a declared probe consumer still decides collision, clearance or
motion validity; C6 still owns organisms, senses and behavior. It selects no
sphere, planet, global topology, terrain engine, navigation mesh, production
voxel representation, storage/streaming layout or runtime map.

## Exact disposable oracle

The executed oracle used a finite `3 x 3 x 3` unit-cell domain and Python
`Fraction` arithmetic. For every in-domain cell it intersected

`P(t) = P0 + t(P1 - P0), 0 <= t <= 1`

with the cell's closed axis-aligned slabs. Per axis, a zero direction component
either left the parameter interval unchanged when the coordinate lay in the
closed slab or rejected the cell. A nonzero component contributed two exact
rational boundary parameters. The intersection of all three axis intervals and
`[0,1]` determined the record.

This exhaustive bounded-cell method is an oracle, not the required future
runtime algorithm. It deliberately favors obvious correctness over traversal
performance and creates no durable implementation dependency.

## Executed portfolio

All cases passed reversal, duplicate-cell and `witness length <= 27` assertions.
`Single-owner omissions` counts exact witness cells absent from the abstracted
ordinary one-owner-per-open-interval DDA output.

| Fixture | Intervals | Point contacts | All-contact cells | Single-owner omissions |
|---|---:|---:|---:|---:|
| axis | 3 | 0 | 3 | 0 |
| face-aligned | 6 | 0 | 6 | 3 |
| edge-aligned | 12 | 0 | 12 | 9 |
| vertex crossing | 2 | 6 | 8 | 6 |
| internal boundary endpoint | 1 | 7 | 8 | 7 |
| outer-domain corner | 1 | 0 | 1 | 0 |
| zero-length internal vertex | 0 | 8 | 8 | 7 |
| thin-barrier crossing | 2 | 0 | 2 | 0 |
| thin-barrier face alignment | 4 | 0 | 4 | 2 |

Canonical summary SHA-256:
`30b60744aa1d26604b5082d0e251fb0d0bca51e254a15a9658dd8a7bce34fab7`.

The thin-barrier comparison also proved why a bare all-contact set is
insufficient: cell `(1,0,0)` is present as a point-only contact in the vertex
fixture and as a positive-length interval in the crossing fixture. A cell set
without the kind and exact interval cannot distinguish those facts.

## Canonical tie, endpoint and order semantics

1. Inputs and reconstructed cell boundaries remain checked fixed-point values.
   Intersection parameters are exact reduced rational ratios derived from those
   integers. No floating-point epsilon or tolerance participates in identity.
2. The segment and each in-domain cell are geometrically closed for evidence
   collection. Both endpoints are included. No phantom cell exists beyond the
   finite bounded-absent domain.
3. Simultaneous two- or three-axis crossings create one exact parameter group.
   Records in that group are semantically unordered; canonical byte projection
   may sort by entry parameter, exit parameter and cell identity only for stable
   serialization. No intermediate tie order becomes physical truth.
4. Reversing endpoints preserves every cell and kind and maps `[a,b]` to
   `[1-b,1-a]`. The complete fixture portfolio proved that invariant.
5. A stationary segment is a point in physical space even though every `t` in
   `[0,1]` maps to it. Its records are therefore `point`, never `interval`.
6. Each in-domain cell appears at most once. Its closed-cell intersection is one
   convex rational interval, possibly degenerate.

## Size and arithmetic bound

For a finite volume with `N = extent_x * extent_y * extent_z` reconstructed
cells, the witness has at most `N` records because each cell has at most one
convex segment intersection and duplicate cell identities are forbidden. The
existing reconstruction ceiling therefore also bounds a witness; no separate
unbounded path allocation is possible.

A future efficient traversal may use checked integer/rational arithmetic
throughout and must be observationally identical to the exhaustive oracle. It
may derive a tighter complexity bound, but the v1 safety bound remains the total
reconstructed-cell ceiling and must be checked before allocation.

## Failure found and engineered away

The first disposable run failed its zero-length assertion. The initial
classifier treated `t_enter < t_exit` as positive traversal. For `P0 == P1`,
the stationary point satisfies the cell slabs for the entire parameter range,
so that rule falsely labeled all containing cells as intervals.

The correction is structural: interval/contact identity is based on **positive
geometric segment length**, with an explicit stationary-path guard. The
zero-length internal-vertex fixture now permanently requires eight point
contacts and zero intervals. This prevents the same data error from returning
in a faster DDA implementation.

## Failure-point assessment

- **Lossy tie ownership:** rejected. Single-owner traversal omitted 3 face, 9
  edge, 6 vertex and 7 endpoint records in the selected fixtures.
- **Contact double counting:** rejected. Bare all-contact output cannot
  distinguish isolated contact from positive traversal.
- **Floating-point or axis-order drift:** excluded from identity by exact
  rational parameters and unordered simultaneous-crossing groups.
- **Reversal asymmetry:** guarded by the exact `[a,b] -> [1-b,1-a]` invariant.
- **Boundary leakage:** the outer-corner fixture produced one in-domain record
  and no out-of-domain phantom cell.
- **Stationary-path misclassification:** found by the first run and replaced by
  the geometric-length rule and permanent fixture.
- **Unbounded output or duplicates:** excluded by one record per cell and the
  reconstruction-cell ceiling.
- **Consumer-policy leakage:** the witness reports physical evidence only; it
  does not say whether a contact blocks light, sound, a probe or an organism.

## Remaining limits and next gate

The oracle proves semantic sufficiency, not a production traversal algorithm,
memory cost at the maximum Forge proof ceiling, cross-language codec identity,
or integration with generated occupancy. It also does not define material
surface physics at shared boundaries; typed consumers must state how they use
point and interval records.

The refreshed physical-path substrate implementation-readiness audit promised
by the design gate is now recorded in
`G1_C3_PHYSICAL_PATH_SUBSTRATE_IMPLEMENTATION_READINESS.md`. It freezes a
bounded module/contract/test surface and prepares one exact owner action without
implementing a crate/schema, closing or promoting C3, or selecting planet,
terrain, runtime, propagation, biome or organism semantics.
