# Visible-Radiance Interface Event Contract

Version: 1  
Status: additive capability-free reference

## Purpose

This contract reconstructs one exact physical path and one explicit face-bound
interaction record into local three-band smooth-dielectric power and direction
enclosures. It fills the typed `interface_model_required` boundary left by the
bulk-transfer reference without changing that reference or continuing a
refracted path.

## Admitted evidence

- The physical volume recipe and query replay through `physical-path-substrate`.
- Exactly one distinct transition has positive-length evidence on both sides.
- The two cells share exactly one face. Edge, vertex, coincident-lane and
  multiple-transition cases fail as typed non-known outcomes.
- The canonical face record binds nonzero source, scope, reconstruction and
  revision provenance; the exact two reconstructed media; and one explicit
  model.
- V1 computes only `smooth_lossless_unpolarized_dielectric` with positive
  Q16.48 red, green and blue indices in `[1/4, 16]`. Vacuum has no implicit
  index; it must be declared like every other medium.

Occupancy, substance names, phase and coarse reflectance never imply interface
behavior. A mismatched declared face or medium is invalid data, not an optical
outcome.

## Arithmetic

- Exact TIR classification compares `(S-a^2)*eta_i^2` with
  `S*eta_t^2` before any roots or rounded branches.
- General evaluation independently recomputes fixed 512-bit outward enclosures
  at 96, 128 and 160 fractional bits and monotonically intersects retained
  target intervals.
- Geometries whose squared delta exceeds 64 bits conservatively execute through
  160 bits. This permanent guard prevents the extreme coprime portfolio from
  certifying earlier than the independent oracle schedule.
- Known Q0.48 power and Q1.62 direction intervals are ordered and at most one
  target unit wide. Failure at 160 returns `nonconvergent_enclosure`.
- Maximum work is three evaluations and 384 fractional-bit work units. The
  derived live ceiling is 452 bits inside checked 512-bit storage.
- Overflow, zero division, negative roots, empty intersections and branch drift
  are arithmetic defects. They are never ordinary nonconvergence.
- Signed floor and ceiling division use unsigned magnitudes with explicit sign
  adjustment. Native limb access is forbidden from codecs, identities and
  target fixtures.

## Codec and replay

Input and event schemas are version 1, reject unknown fields and require
canonical JSON replay. Wide endpoints use canonical signed decimal strings:
no plus sign, leading zero, negative zero or value outside 512 bits. Scales are
explicit `q0_48` and `q1_62`; arithmetic levels are the strict prefix of
`[96,128,160]` actually attempted. Revalidation recompiles the whole event from
the input.

## Typed outcomes

`known`, `nonconvergent_enclosure`, `no_interface_event`,
`unavailable_evidence`, `ambiguous_boundary_lane`,
`ambiguous_interface_geometry`, `missing_interface_evidence`, and
`unsupported_interface_model` remain distinct. A nonconvergent result is not
darkness, opacity, zero transmission, total reflection or permission to retry
with hidden resources.

## Platform rule

The semantic core is portable checked Rust. Primary evidence is executable
x64 Windows plus executable i686 replay; Android ARM64 compilation is retained
for the PC/mobile priority tier. Actual mobile-device execution and production
profiling remain later promotion gates. Lower-ROI platforms do not block this
reference and may be ported when justified.

## Nonclaims and rollback

The event does not own coefficients, full radiative transfer, downstream path
occupancy, endpoint arrival, perception, rendering, passage, navigation,
biomes or ecotones, spheres, planets, terrain, persistence, runtime, approval,
promotion or C3 closure. Removing this crate and its one exact dependency pin
restores the prior explicit `interface_model_required` behavior without data
migration or reinterpretation.

## Additive conditional interval incident v1

The crate also exposes a separately versioned conditional interval surface.
It does not alter the point-v1 schema, codecs, identities, domain separators,
arithmetic schedule or outcomes described above.

`VisibleRadianceIntervalInterfaceInputV1` binds a declared Q1.62 incident
component box, ordered source and target cells, the canonical face evidence,
and the identities of separately supplied `PhysicalVolumeRecipeV1` and
`PhysicalVolumeV1` objects. The compiler and validator replay those supplied
objects and reconstruct the cells, media and oriented shared face. The input
is conditional local evidence only: it is not proof of a prior ray, a path,
endpoint arrival or point-position lineage.

Before numerical work, exact integer checks reject noncanonical or reversed
component intervals, a box containing the zero vector, a box with no unit
sphere intersection, and a box whose full normal interval does not point
strictly from source to target. Each red, green and blue band is independently
classified as uniformly total-internal-reflecting, uniformly transmitting or
branch-ambiguous. An ambiguous band carries no representative direction or
power.

Uniform bands execute exactly one outward interval evaluation at 160
fractional bits in checked 512-bit storage. The derived maximum live bound is
452 bits. The production interval source does not consult the adaptive point
schedule, 384-bit verification oracle, adjacent-precision agreement, native
floating point or platform-native limb encodings. Failure to form a finite
enclosure at the fixed cap is typed as `nonconvergent_enclosure`; arithmetic
overflow or an invariant breach remains an error.

Interval input bytes are capped at 16 KiB and event bytes at 64 KiB before
decode as well as after encode. Both codecs reject unknown fields and require
canonical replay. The interval input and event use the distinct domain
separators `forge-visible-radiance-interval-interface-input-v1` and
`forge-visible-radiance-interval-interface-event-v1`.

The committed point-v1 identity fixture locks five public cases across the
addition. The retained internal wide-coprime arithmetic case has no public
input/event codec to freeze; it remains protected by the private kernel test
and independent exact portfolio checksum. Rollback removes the additive
interval module, re-export, tests and this section without migration or
reinterpretation of point-v1 data.
