# G1 / C3 interval optical cell-step implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; no production
source, schema or manifest change has been performed.**

## Decision

The positive 3D oracle candidate fits as one private additive, separately
versioned conditional interval submodule inside `physical-path-substrate`.
Every code-facing seam now has a bounded rule: admitted ranges, arithmetic,
provenance, codecs, allocation, ownership, compatibility, dependency use,
platform gates and rollback.

The surface remains channel-neutral. It proves one local conditional cell step
from declared point/direction boxes. It does not import or validate optical
events, preventing a dependency cycle. A future composer must independently
replay the initial exact face point or prior step and bind the corresponding
direction source.

## Additive public surface

Implementation is confined to a private `interval` submodule with additive
re-exports from the existing crate. It introduces:

- `INTERVAL_CELL_STEP_CONTRACT_VERSION = 1`;
- `INTERVAL_CELL_STEP_FRACTIONAL_BITS = 160`;
- `MAX_INTERVAL_CELL_STEP_INPUT_BYTES = 16 * 1024`;
- `MAX_INTERVAL_CELL_STEP_EVENT_BYTES = 32 * 1024`;
- `INTERVAL_CELL_STEP_DERIVED_MAXIMUM_LIVE_BITS = 414`;
- a strict `ConditionalIntervalCellStepInputV1`;
- a strict `ConditionalIntervalCellStepEventV1`;
- channel-neutral face, signed-decimal interval, evidence-kind, outcome and
  arithmetic-receipt types;
- compiler and full replay validator functions; and
- new domain separators
  `mindwarp.physical-path.interval-cell-step-input.v1` and
  `mindwarp.physical-path.interval-cell-step-event.v1`.

Existing `CONTRACT_VERSION`, physical-volume/path V1 types, functions, bytes,
identities and domain separators remain untouched.

The compiler receives validated `&PhysicalVolumeRecipeV1` and
`&PhysicalVolumeV1` objects separately from the new input. The input contains
their identities but never embeds their potentially 65,536-run payload. The
validator recompiles the event from the same three objects.

## Input and provenance

The input contains exactly:

- schema version `1`;
- nonzero state source, scope and reconstruction identities plus positive
  state revision;
- evidence kind `declared_conditional_point_direction_box`;
- canonical recipe and volume identities;
- one current `CellIndex3V1`;
- three ordered canonical Q160 point intervals; and
- three ordered canonical Q1.62 direction intervals.

The compiler fully validates recipe/volume replay, scope/reconstruction
agreement, the cell index and rebuilt current cell. Every point endpoint must
lie inside that cell's closed bounds after exact Q32.32-to-Q160 lifting.
Direction endpoints must lie in `[-2^62,2^62]`. Reversed intervals fail before
arithmetic.

A direction box may contain the zero vector. That is a valid conditional state
whose result is `no_forward_progress`; it is not silently normalized or
rejected as an optical error. The module makes no unit-vector, ray-history,
path, source-event, endpoint-arrival or optical-channel claim.

## Exact face and outcome semantics

For each axis, a strictly positive upper direction endpoint makes the max face
possible and a strictly negative lower endpoint makes the min face possible.
Point-to-face distance and speed are nonnegative. Directed fixed-160 division
forms each possible face's time lower bound. A finite upper bound exists only
when the whole direction component has the required strict sign.

If any possible face has time lower bound zero, return
`no_forward_progress`. Otherwise select one face only when its finite upper
time is strictly below every competitor's lower time. No winner returns
`ambiguous_next_face`; iteration or axis order cannot break a tie.

After selection, outward multiply/add forms the hit-point box. The hit axis is
the exact selected Q32.32 plane lifted to Q160. Tangential axes intersect with
the closed current-face bounds. Empty intersection is an arithmetic defect.
The selected neighbor is reconstructed from the volume:

- in-domain available/vacuum/substance evidence returns
  `certified_next_face`;
- `Unavailable` returns `unavailable_neighbor` while retaining the certified
  face/time/point evidence; and
- a face outside the bounded volume returns `outer_domain_exit` without a
  fabricated unsigned neighbor index.

Gas, liquid and solid never imply blocking, attenuation, support or passage.
No endpoint relation, bulk transfer or interface event is emitted.

## Arithmetic derivation

The derivation follows the actual source ranges:

1. Physical coordinates are signed Q32.32 `i64`. Lifting to Q160 shifts by
   128; the largest signed magnitude is `2^191` and requires 192 magnitude
   bits only for the exact negative endpoint.
2. Positive `cell_step_q32_32` is at most `i64::MAX`, so a local point-to-face
   distance is strictly below `2^191` in Q160 raw units.
3. Q1.62 direction magnitudes lift into `[0,2^160]`; the smallest positive
   admitted component is `2^98` in Q160.
4. A directed time numerator multiplies local distance by `2^160` and remains
   below `2^351`. Dividing by the smallest positive speed produces a Q160 time
   below `2^253`.
5. Direction-by-time multiplication remains below `2^413`. Projection and
   point addition require at most 254 magnitude bits before the certified-face
   intersection restores coordinate bounds.

The source-level ceiling is therefore 413 magnitude bits, or 414 bits with
explicit signed allowance, inside fixed 512-bit storage. Encoding may expose
only canonical decimal values; native limb count, width, order and endianness
are forbidden.

Overflow, zero division reached contrary to classification, empty
intersection or exceeding the 414-bit shield is an arithmetic defect, never
ordinary ambiguity. The local call has no adaptive precision and no hidden
reference computation.

## Dependency disposition

`physical-path-substrate` currently depends only on `serde`, `serde_json` and
`sha2`, so fixed 512-bit operations are not presently available. Copying a
home-grown wide integer or moving the optical module's private arithmetic
would create duplication or a reverse dependency.

The bounded action adds this exact direct dependency:

`crypto-bigint = { version = "=0.7.5", default-features = false }`

That package and its complete dependency graph are already resolved in the
workspace lockfile and used by `visible-radiance-interface-event`. Therefore
the action adds no new resolved package, version or feature. The retained wide
dependency spike already proves x64/i686 execution, checked 512-bit
operations, target-neutral behavior, licensing and disabled allocation
features. The implementation must use unsigned-magnitude quotient/remainder
for directed division and may not use native limbs in identity or codecs.

Rollback removes the direct manifest line; because the package remains used by
the optical crate, no lockfile reinterpretation or migration occurs.

## Codec, allocation and work ceilings

Input bytes are capped at 16 KiB and event bytes at 32 KiB before decode and
after encode. Unknown fields, wrong scale, noncanonical signed decimals,
leading zeros, plus signs, negative zero, whitespace, object-order drift,
trailing data, foreign identities and limitation drift fail replay.

One call stores six point endpoints, six direction endpoints and at most six
face candidates. Its arithmetic receipt records:

- fixed precision `160` and storage width `512`;
- derived maximum live bits `414`;
- possible-face count in `0..=6`;
- directed-division count in `0..=12`;
- strict face-order comparison count in `0..=30`; and
- propagated tangential axes as zero or two.

No 64-step cost is charged to one call. The implementation tests must measure
maximum input/event bytes, decoded structure sizes, maximum coordinate/time
decimal lengths and the widest admitted arithmetic fixture.

## Compatibility freeze

Before interval source is compiled, capture canonical bytes, byte hashes and
all available public IDs for at least:

1. straight face traversal;
2. exact reversal;
3. simultaneous edge/vertex contact;
4. stationary point evidence; and
5. a negative/near-maximum coordinate volume.

The fixture covers recipe input, recipe, volume, query and witness objects.
It must pass byte-for-byte before and after the additive module. Existing 13
unit tests and downstream bulk, interface and swept-AABB suites must also
remain unchanged. Any V1 byte, identity, dependency behavior or output drift
rolls back the complete package.

## Required hostile and platform tests

Permanent tests must cover normal and reverse faces, one-Q1.62-unit face
reversal, exact edge/corner ambiguity, correlation-erasure ambiguity,
zero-straddling competitors, exact zero direction, prior-face zero progress,
minimum-positive direction, negative and maximum coordinates, empty/invalid
point boxes, scale/decimal poison, forged cells/volume/identities,
unavailable neighbor, all six outer exits, arithmetic-shield enforcement,
maximum codec/allocation fixtures and four 64-step oracle lanes.

Required gates are warnings-denied native Windows tests, executable i686
Windows tests, Android ARM64 compilation, the deterministic Python oracle,
module-context verification and the full Forge gate. Actual mobile-device
execution and profiling remain later promotion evidence.

## Rollback and exclusions

Rollback deletes the private interval submodule, additive re-export, tests,
fixtures, direct dependency line and this additive contract/readiness surface.
It performs no migration and leaves exact path V1 and all consumers intact.

This action does not authorize interval bulk transfer, a composer, endpoint
arrival, coefficients, physical visibility completion, perception, rendering,
collision, passage, navigation, support/force response, organisms, biome or
ecotone meaning, sphere, planet, terrain, persistence, runtime, promotion or C3
closure.

## Exact owner action

Approve one test-first additive package inside
`crates/physical-path-substrate` that:

1. captures and locks the five exact-path V1 byte/ID families before interval
   source;
2. adds only the private conditional interval cell-step submodule and additive
   re-exports described above;
3. adds the exact already-resolved `crypto-bigint = 0.7.5`,
   default-features-disabled direct dependency;
4. implements fixed-160 universal face certification and typed local outcomes
   under the 414/512-bit source shield;
5. enforces 16 KiB input and 32 KiB event pre-decode caps and records maximum
   byte/allocation/work receipts;
6. retains the independent oracle and hostile x64/i686/Android lanes; and
7. stops and rolls back on V1 drift, a new resolved package/feature, native
   limbs/floats, hidden precision, unbounded decode, dependency cycle,
   end-to-end lineage claim or failed platform/full-repository gate.

General continuation does not authorize this source action. Explicit owner
approval of this exact package is required.
