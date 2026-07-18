# G1 / C3 one-band interval bulk-transfer oracle result

Date: 2026-07-16

Status: **bounded candidate selected; arithmetic-consolidation audit required
before code-facing implementation readiness; no production source authorized.**

## Decision

A useful one-band conditional interval bulk transfer can be built from one
validated cell-step input/event without assuming a unit direction and without
inventing a representative ray.

The selected length certificate intersects two independently sound
enclosures:

1. outward `sqrt(dx^2 + dy^2 + dz^2) * face_time`; and
2. the outward Euclidean norm of the input point box to the certified hit-point
   box.

The speed/time certificate respects the cell-step parameterization and can
retain a positive lower bound. The displacement certificate caps the result
by the current cell geometry and repairs widening caused by treating direction
and time as independent. Intersecting sound enclosures is sound and never
chooses a midpoint, corner or normalized representative.

The transfer remains one spectral band per call. Post-interface RGB direction
boxes may follow different cells, so a shared three-band geometry call is
forbidden after dispersion.

## Deterministic oracle receipt

`tools/prove-g1-c3-interval-bulk-transfer.py` ran twice with byte-identical
canonical receipt hash
`94b2fe43260c9a604ec6c22035f28f7026319531c22951a4e8747f8d242713c3`.
The script source SHA-256 is
`f5ed61bbe73a6df645ef31497d6930f12057f5f7346572e9c41b7f55647ecae6`.

The proof contains:

- 11 named finite, vacuum, zero, opaque, unavailable, terminal and ambiguous
  cases;
- 512 exact named corner witnesses;
- 256 deterministic generated boxes: 247 finite and nine typed face
  ambiguities;
- 15,808 generated exact corner witnesses;
- four independent 64-step red, green, blue and widened lanes;
- 16,384 repeated-lane exact corner witnesses; and
- 321 maximum observed live bits in the exercised portfolio.

Every admitted exact correlated witness is contained by the selected length,
optical-depth and Q0.48 transmission bounds. Ordinary red, green and blue
64-step lanes retain at most one Q160 raw length unit and one Q0.48
transmission unit of width per step. The deliberately widened lane's dual
certificate reduces the maximum length width from
`1267651506922594111953188364294` to
`115150275831372401705631800` Q160 raw units while retaining the same one-unit
transmission width.

The broad generated portfolio's maximum transmission width is 116,378 Q0.48
raw units, below `2^-31` in absolute transmission fraction. This is useful
conditional evidence, not a universal production-quality claim; real
coefficient distributions and device profiling remain absent.

## Arithmetic ceiling

The source-range derivation is stricter than the observed portfolio:

1. Q1.62 direction endpoints lift to Q160 with component magnitude at most
   `2^160`.
2. Three squared components sum below `3 * 2^320`, requiring at most 322
   magnitude bits before the directed square root.
3. The speed enclosure is below `2^161` Q160 raw units.
4. The existing certified face time remains below `2^253` Q160 raw units.
5. A deliberately dependency-erased speed/time product remains below `2^414`
   before division by `2^160`.
6. Independently, every start and certified hit point lies in one cell, so
   displacement length remains below the cell diagonal and below `2^192` Q160
   raw units over the admitted Q32.32 volume range.
7. Intersecting the certificates therefore restores a final length below
   `2^192` raw units.
8. Multiplication by the existing unsigned Q16.48 coefficient remains below
   `2^256`; projection to Q64.64 remains below `2^112`, inside the existing
   bulk module's checked `u128` optical-depth and exponential path.

The new wide work is confined to direction norm, displacement norm and their
length intersection. Existing optical-depth and exponential semantics can be
reused inside their current owner without a second kernel.

Production readiness must freeze a 414-magnitude-bit wide intermediate shield
inside 512-bit storage and explicitly define whether its receipt counts a
separate signed allowance. No source may blindly reuse the cell-step's
414-signed-bit label because the vector norm adds a different bound.

## Typed local result

The surviving conditional result should bind one validated bulk profile, one
band, one cell-step input/event and one current-cell evidence value. Its local
outcomes are:

- vacuum or finite-zero identity;
- finite length, optical-depth and transmission bounds;
- opaque zero transmission;
- unavailable current cell; and
- upstream ambiguous/no-progress geometry.

For a certified span followed by `outer_domain_exit` or
`unavailable_neighbor`, the current-cell transfer is retained and the terminal
neighbour disposition is attached afterward. A future composer decides
whether or how that lane terminates. Interval bulk does not invoke an
interface or endpoint test.

## Arithmetic-consolidation disposition

The audit rejects a third private signed-512 implementation.
`physical-path-substrate` and `visible-radiance-interface-event` already own
near-duplicate target-neutral signed-magnitude parsing, checked arithmetic and
directed division. Interval bulk additionally needs the same interval
multiplication and directed square root already present in the interface
arithmetic module.

The next package is therefore a **capability-free shared fixed-interval
arithmetic consolidation design/readiness audit**. It must specify a semantic-
neutral crate owning only:

- checked signed-magnitude 512-bit values;
- canonical decimal parsing and formatting without native limbs;
- checked add, subtract, multiply and shift;
- mathematical floor/ceiling division from unsigned magnitudes;
- fixed-scale ordered interval add, subtract, multiply and intersection;
- directed integer square root; and
- explicit live-bit accounting.

It must not own physical coordinates, optical coefficients, spectral bands,
domain separators, semantic codecs, outcomes or authority. Existing owners
retain those policies.

The audit must compare extraction against retaining two private implementations
plus a third new one, quantify source/maintenance reduction, and freeze a
migration sequence protected by the existing point-interface V1 and exact-path
V1 byte/ID fixtures plus new interval event fixtures. If extraction cannot
preserve bytes, identities, errors, feature sets and x64/i686/Android behavior,
the fallback is explicit private retention—not an unreviewed third copy.

## Failure points engineered out

| Failure | Permanent response |
|---|---|
| treat Q160 face time as Euclidean length | multiply by an outward direction norm |
| allow speed/time dependency erasure to dominate width | intersect with independent start-hit displacement norm |
| normalize a declared box by assertion | no unit-vector premise exists in the candidate |
| recombine dispersed RGB geometry | exactly one band and cell-step receipt per call |
| discard attenuation at an outer or unavailable next cell | retain current-cell transfer before terminal disposition |
| copy the bulk exponential kernel | reuse it inside the existing bulk owner |
| create a third divergent signed-512 implementation | require shared-arithmetic consolidation disposition first |
| let local bulk become a composer | no ordered history, interface call or endpoint claim |

## Authority and next action

This result authorizes no Rust source, crate, dependency, manifest, schema,
arithmetic migration, interval bulk implementation or composer. The next safe
action is the shared fixed-interval arithmetic consolidation design/readiness
audit. After that audit, a separate interval bulk code-facing readiness audit
may prepare an exact owner action if the arithmetic and compatibility seams
are closed.

Perception, rendering, gameplay visibility, collision, navigation, organism,
biome, sphere, planet, terrain, runtime, promotion and C3 closure remain
outside this result.
