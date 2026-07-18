# Swept AABB passage contract v1

## Scope

This additive, capability-free C3 reference asks where a positive-extent,
fixed-orientation Q32.32 AABB translated along one closed segment contacts or
has strict interior overlap with validated physical-volume cells. It uses the
Minkowski-expanded cell bounds and exact reduced unit rationals.

It is evidence, not runtime collision response. It does not own forces,
depenetration, friction, restitution, support, buoyancy, drag, navigation,
walkability, organism capability, planet shape, terrain, biome identity,
ecotones, materials, rendering, authority, or promotion.

## Required input and authority separation

- The physical recipe and volume must replay under `physical-path-substrate`.
- All three probe half-extents are positive Q32.32 values.
- V1 motion is fixed-orientation translation. Rotation is typed unsupported.
- Each touched known subject requires an exact mechanical-profile rule.
- Mechanical subjects are unique and sorted by canonical subject bytes, so
  equivalent rule sets cannot acquire different query identities by reordering.
- Subjects are `vacuum`, `unavailable`, or the exact phase plus nonzero
  substance source identity already present in cell evidence.
- Phase never implies mechanics. Rules say only `blocks_translation` or
  `does_not_block_translation`.
- Unavailable evidence stays unavailable and never becomes vacuum or passage.

## Exact contact semantics

Each cell is expanded by the probe half-extents with checked i128
intermediates and representable i64 results. Closed slab intersection produces
`t_enter` and `t_exit`. A positive-duration interval is strict interior only
when no stationary axis lies on an expanded face. Consequently, sliding along
a face or edge remains contact-only even when `t_enter < t_exit`.

The bounded physical volume is contracted by the probe half-extents to define
the legal centre domain. Initial protrusion and boundary contact are typed
separately from cell evidence. Entry-axis masks are unordered three-bit sets,
so simultaneous face, edge, and corner entry cannot acquire a false order.

## Failure and resource boundaries

- Unknown fields, noncanonical bytes, zero source identities, duplicate
  subjects, forged result identities, arithmetic overflow, and stale replay
  fail closed.
- At most 65,536 cells, 65,536 witnesses, and 32 MiB of canonical result bytes
  are admitted.
- Query and result identifiers are SHA-256 domain-separated canonical-byte
  hashes and validation recompiles the result.
- The permanent Python oracle uses arbitrary-precision critical times and
  midpoints rather than the Rust slab implementation.

## Rollback

Removal is limited to the `swept-aabb-passage` crate, this contract, its oracle,
verifier, generated module context, and result record. The point-path substrate
and all downstream runtime or organism systems remain unchanged.
