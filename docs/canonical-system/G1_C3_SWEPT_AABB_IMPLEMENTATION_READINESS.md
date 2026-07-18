# G1 C3 Swept AABB Implementation Readiness

Date: 2026-07-16

Status: **implementation-ready candidate frozen; exact owner gate retained; no
schema, crate or implementation exists.**

## Readiness decision

The fixed-orientation translated AABB remains the minimum candidate, but the
design audit's outcome language required one correction before code: closed
geometric contact is not automatically failed passage. Exact face sliding,
edge/corner tangency and a start-on-boundary motion moving away can touch a
blocking configuration obstacle without entering its interior.

V1 must therefore preserve both closed contact and open-interior penetration.
It may not collapse them into a Boolean collision or traversability result.

## Closed candidate boundary

### Input identities and values

The future additive module consumes, but never modifies:

- one validated `PhysicalVolumeRecipeV1` and exact reconstructed
  `PhysicalVolumeV1`;
- one nonzero probe source ID and one nonzero mechanical-profile source ID;
- one local probe AABB with three strictly positive `i64` Q32.32 half extents;
- one start and end reference point in the existing volume's closed Cartesian
  frame; and
- one complete-on-use mechanical profile keyed by exact evidence subject.

The evidence-subject key is one of `unavailable`, `vacuum`, or exact
`(phase, substance_source_id)` for gas, liquid and solid. Its only v1 response
is `blocks_translation` or `does_not_block_translation`. Missing entries remain
`interaction_model_required`. Phase, names and coarse reflectance never supply
a default.

V1 does not model friction, support, buoyancy, drag, pressure, damage,
depenetration, restitution or force. Contact axes are evidence a later consumer
may interpret; they are not support claims.

### Motion and domain

Motion is one reference-point translation over exact `t in [0,1]`. Orientation
is absent from v1 rather than fixed by a hidden default. Any rotational or
piecewise-motion request is `unsupported_motion`.

Both reference endpoints must be inside the original closed volume. Six
explicit outer-domain configuration obstacles represent the space beyond its
bounds. A probe already protruding beyond the volume is
`initial_outer_domain_overlap`; first boundary entry is
`outer_domain_contact`. Unavailable cells remain distinct from the outer domain.

## Exact construction

For each cell, compute expanded bounds in `i128`:

`expanded_min[axis] = cell_min[axis] - half_extent[axis]`

`expanded_max[axis] = cell_max[axis] + half_extent[axis]`

Reject if either bound is outside `i64`; never clamp. Segment direction is the
checked `i128` difference of Q32.32 endpoints. Raw event numerators are signed
`i128`; nonnegative magnitudes and denominators must fit `u64`. Rational
ordering uses cross-products in `u128`, matching the verified substrate.

Each touched expanded cell produces at most one witness:

- exact reduced `t_enter` and `t_exit` for the closed configuration obstacle;
- a three-bit unordered entry-axis mask;
- `contact_only` when the segment intersects only the boundary; or
- `interior_interval` when some open parameter interval places the probe
  interior inside the obstacle interior.

`t_enter == t_exit` is always `contact_only`, but the converse is not true: a
segment can lie along a face over a positive interval. Interior classification
must therefore prove strict overlap on all three axes over a common open
parameter interval; interval length alone is insufficient.

## Typed result

The candidate result has these mutually exclusive top-level states:

- `stationary_separated`;
- `stationary_contact_only`;
- `stationary_interior_overlap`;
- `sweep_separated`;
- `sweep_contact_only`;
- `sweep_first_interior_entry`;
- `initial_interior_overlap`;
- `outer_domain_contact`;
- `initial_outer_domain_overlap`;
- `unavailable_evidence`;
- `interaction_model_required`; and
- `unsupported_motion`.

Known results retain the earliest exact parameter, every simultaneous cell and
axis witness at that parameter, profile provenance, query/probe identities and
limitations. Contact ordering is canonical bytes only and has no semantic
priority. No result says `passable`, `walkable`, `navigable`, `supported`,
`habitable`, or `destination_reachable`.

## Independent counterexample oracle

The required disposable oracle may not reuse slab interval code. Over a small
integer lattice it must:

1. construct expanded configuration obstacles directly with arbitrary-precision
   integers;
2. enumerate every critical rational time at which a moving reference point
   reaches an expanded min or max plane, plus `0` and `1`;
3. sort and deduplicate exact fractions;
4. evaluate boundary/interior membership at every critical time and at one
   exact rational midpoint of every adjacent interval; and
5. derive the first contact and first interior entry independently from those
   samples.

The production candidate must equal this oracle for exhaustive small-lattice
start/end/extent portfolios and directed hostile fixtures. A checksum over the
canonical vector set is required before implementation readiness can advance.

## Mandatory hostile fixtures

1. point path clears while the extended probe hits a one-cell-thick barrier;
2. face sliding, edge sliding and one-time corner touch remain `contact_only`;
3. start-on-boundary moving away versus moving inward;
4. stationary separated, boundary-only and strict interior cases;
5. initial overlap with one and several cells, without invented MTV;
6. simultaneous faces/cells and canonical-order permutation attacks;
7. exact reversal: contact parameters map by `t -> 1-t`, while initial-state
   classes intentionally do not masquerade as symmetric outcomes;
8. probe extent and expanded-bound overflow;
9. missing, forged and phase-derived interaction entries;
10. unavailable cell versus outer-domain distinction;
11. geometrically equivalent coarse/fine occupied unions: stable first-contact
    geometry, intentionally different provenance; and
12. noncanonical JSON, unknown fields, wrong identities and forged witnesses.

## Resource ceiling

- reconstructed cells examined: at most 65,536;
- expanded-cell witnesses: at most 65,536, one per cell;
- entry axes: one three-bit mask per witness;
- exact axis tests: at most `3 * 65,536` per query, excluding validation replay;
- canonical result bytes: hard cap 32 MiB;
- measured debug test body: must remain below 30 seconds on the current PC;
- retained implementation-readiness target: measure peak memory and require a
  64 MiB ceiling before promotion or mobile execution.

These are proof ceilings, not production performance claims. An optimized
adapter may later use broad-phase acceleration only if it reproduces all
canonical vectors.

## Module, rollback and dependency plan

Any implementation must be a new additive capability-free crate. It may depend
only on `physical-path-substrate` plus already accepted serialization/hash
utilities. It cannot alter the substrate's contract, identities, codecs or
tests. Removing the new crate, its contract/result and verifier must restore the
current workspace without data migration or reinterpretation.

No collision engine, floating-point geometry package, GJK library, runtime,
filesystem, network, process or platform-specific dependency is justified.

## Exact implementation gate

Implementation remains blocked until the owner authorizes exactly:

`APPROVE G1-C3-SWEPT-AABB-REFERENCE-V1`

That phrase authorizes only the additive capability-free crate, strict codecs,
independent disposable oracle vectors, hostile tests, focused integration and
rollback verification described here. It does not authorize runtime collision,
organism binding, navigation, terrain, coefficients, rotation, sphere/capsule,
planet geometry, biome presentation, C3 closure or promotion.

## Stop condition

Readiness is complete. Stop at the owner gate. Do not create the schema, crate,
oracle script or dependency entry without the exact authorization above.
