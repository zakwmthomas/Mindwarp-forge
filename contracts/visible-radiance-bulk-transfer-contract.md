# Visible-Radiance Bulk-Transfer Contract v1

This capability-free contract reconstructs one exact
`physical-path-substrate` volume and witness and returns observer-independent
three-band direct-beam bulk-transfer evidence. It is a bounded procedural
reference, not a coefficient catalogue, interface-optics model, perception or
rendering system.

`VisibleRadianceBulkProfileInputV1` binds nonzero source, scope and
reconstruction identities, a positive revision, one exact physical-volume
recipe and a canonical substance interaction entry for every unique
non-vacuum substance in that volume. Entries are strictly ordered by exact
substance identity. Each red/green/blue interaction is either a finite Q16.48
extinction coefficient per volume coordinate unit or explicit opaque evidence.
Vacuum has implicit zero bulk extinction; unavailable has no interaction.

The profile compiler rebuilds the volume. The transfer compiler then rebuilds
the profile, exact path witness and every cell evidence value. Callers never
submit cells, path spans, optical depth, transmission, classifications or
output state.

For each nonempty open parameter span, exactly one positive-length cell must be
active. Unavailable evidence returns `unavailable_evidence`; overlapping
closed-cell lanes return `ambiguous_boundary_lane`. Point-only contacts add no
bulk length. A positive-length transition between vacuum and a substance or
between distinct substances returns `interface_model_required`. Consequently a
known v1 result contains only vacuum or one exact substance, and homogeneous
cell subdivision cannot change it.

Stationary paths return exact identity unless any containing point record is
unavailable. Other point-only substances neither attenuate nor create an
interface.

Known finite bands carry directed Q64.64 optical-depth bounds encoded as two
u64 limbs and directed Q0.48 transmission bounds. Vacuum is exact identity;
opaque is exact zero. The geometric squared delta is checked in `u128` and
fails when the three-square sum exceeds that v1 ceiling. Directed integer
square root bounds Q32.32 length. Checked `u128` multiplication produces
Q64.64 optical depth. A fixed range-reduced alternating-series and directed
interval-squaring kernel encloses `exp(-tau)` before outward Q0.48 projection.
No returned interval may exceed one Q0.48 unit. Float, epsilon, wrapping,
clamping, partial output and best-effort state are forbidden.

All input, profile, query and transfer codecs are strict reconstruct-and-
compare JSON. Unknown fields, noncanonical ordering or bytes, incomplete or
duplicate substance coverage, foreign identities, output forgery, limitation
drift and authority mutation fail closed. Profile substances, physical cells
and witness records are capped at 65,536.

The independent Python mathematical oracle remains a permanent verification
dependency. The Rust reference must match its fixed vectors and retain hostile
replay, classification, arithmetic, ceiling, forgery and cost tests.

## Additive conditional interval bulk-transfer surface

`ConditionalIntervalBulkQueryV1` binds one validated bulk profile, exactly one
red, green or blue band, and one complete conditional cell-step input/event
pair. The compiler reconstructs the profile volume and revalidates the nested
cell-step event. Callers cannot submit a current-cell classification, face,
length, optical depth, transmission value or terminal disposition.

For a certified face, the compiler derives two outward Q160 length
enclosures: direction norm multiplied by certified face time, and the norm of
the start-box-to-certified-hit-box displacement. Their intersection is the
only admitted current-cell length. It is not a normalized representative ray.
Ambiguous face and no-forward-progress inputs remain typed upstream outcomes;
unavailable current-cell evidence never becomes vacuum. Transfer through the
known current cell is retained before a known, unavailable or outer-domain
terminal disposition is attached.

The additive route uses `fixed-interval-arithmetic` directly. Wide work is
held in signed 512-bit storage with a 414-magnitude-bit intermediate shield;
the intersected length must remain at or below 192 magnitude bits. The receipt
freezes ceilings of seven shifts, eight interval multiplications, four adds,
three subtracts, two directed roots, one intersection, one Q64 projection and
192 exponential terms. Arithmetic defects map into the bulk owner's existing
fail-closed error surface.

Finite Q16.48 coefficients are lifted and multiplied by the intersected Q160
length, then outward-projected to Q64.64 before the existing bulk-owned
exponential kernel is reused. Vacuum is exact identity and opaque is exact
zero. Exactly one spectral band is processed per call because dispersed bands
may follow different cells. Query bytes are capped at 64 KiB and transfer
bytes at 16 KiB before decode; strict reconstruct-and-compare replay rejects
unknown fields, foreign nested evidence, identity forgery and noncanonical
bytes.

This is local one-cell conditional evidence only. It neither invokes interface
optics nor composes ordered cell/interface lineage, and it makes no endpoint,
arrival, visibility, perception, runtime, approval or promotion claim. The
pre-existing three-band exact-path V1 schemas, bytes and identities are
unchanged.

## Additive optical-depth evaluation receipt

`BulkOpticalDepthEvaluationInputV1` admits only ordered nonnegative Q64.64
endpoints below the 118-bit raw ceiling. Its read-only compiler calls the
unchanged bulk-owned `exp(-tau)` kernel exactly twice: upper depth for the
lower Q0.48 transfer endpoint and lower depth for the upper endpoint. It
derives both identities under new domain separators and repeats the unchanged
input endpoints in the result.

Input and result codecs are each capped at 4 KiB before decode and remain
strict reconstruct-and-compare JSON. This surface derives no path, medium,
spectral band, time basis, coefficient, opacity, or physical quantity. A zero
projected lower transfer remains finite underflow and never becomes opacity.
Every legacy exact-path and conditional-interval V1 byte and identity remains
unchanged.

This contract defines no real-world coefficient validity, metre mapping,
surface reflection, refraction, scattering, emission, source power, received
irradiance, inverse-square geometry, biological sight, detectability,
presentation, gameplay line of sight, passage, navigation, biome, ecotone,
organism, sphere, planet, terrain, storage, streaming, runtime, approval or
promotion. Categorical cells or regions cannot create visible biome seams;
continuous biome causes must still produce deterministic ecotones, with sharp
boundaries only for sharp physical causes.
