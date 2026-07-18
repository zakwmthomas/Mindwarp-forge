# G1 C3 Visible-Radiance Interface-Event Mathematical Design Audit

Date: 2026-07-16

Status: **local mathematical candidate selected; arbitrary-precision oracle
required before schema or implementation readiness.**

## Decision

The smallest honest interface-optics candidate is one reconstructed **local
smooth-dielectric event** at a uniquely evidenced grid face. It returns
bandwise certified bounds for reflected/transmitted power fractions and local
outgoing direction, or a closed typed unavailable/ambiguous/unsupported result.

It does not extend the current bulk-transfer result and does not claim
end-to-end transfer. General refraction changes the path direction, so every
downstream occupancy witness would have to be rebuilt from the event. The exact
rational crossing point may also be unrepresentable in the current Q32.32 path
endpoint type. Those are later path-composition dependencies, not rounding
details to hide inside this event.

No schema, fixed-width policy, coefficient catalogue or implementation is
ready until an arbitrary-precision disposable oracle falsifies or confirms the
geometry, equations, invariants and bit-width ceilings below.

## Exact event reconstruction

The design consumes a reconstructed `PhysicalVolumeV1` and exact
`PhysicalPathWitnessV1`. It first applies the bulk consumer's permanent rules:

- unavailable positive-length evidence remains unavailable;
- an open span with anything other than one active positive-length cell remains
  `ambiguous_boundary_lane`;
- adjacent spans with identical medium evidence merge before event discovery;
- point-only contacts contribute no interface event; and
- stationary paths contain no interface event.

An interior transition is a candidate interface event only when:

1. two consecutive merged positive-length spans meet at the same exact reduced
   rational path parameter strictly between zero and one;
2. their medium evidence identities differ;
3. their cells differ by exactly one index step on exactly one axis and match
   on the other two axes; and
4. explicit interaction evidence binds that exact shared face, both exact
   medium identities and the current volume reconstruction.

The one-axis cell-index delta reconstructs the oriented unit normal from the
incident cell toward the target cell. The exact path delta must have a positive
component along that normal. No normal is inferred from phase, material name or
substance identity.

A transition whose cells differ on two axes is an edge crossing; one differing
on three axes is a vertex crossing. Both return `ambiguous_interface_geometry`.
So do simultaneous faces, coincident lanes or any record set that cannot prove
one unique oriented shared face. The design never chooses an axis by order,
largest component or epsilon offset.

An endpoint lying on a face is not a crossing unless the query contains
positive-length evidence on both sides. Tangent point contact with another
medium remains a non-event. Equal-medium face subdivision remains a non-event.

## Interaction authority

Occupancy proves medium identity, not optical boundary behavior. The candidate
therefore requires explicit face-bound interaction evidence with:

- nonzero source, scope, reconstruction and revision identities;
- the exact volume identity and canonical shared-face key;
- the exact unordered pair of medium evidence identities on that face;
- model class `smooth_lossless_unpolarized_dielectric`; and
- one positive real refractive-index value for each side and each fixed
  `red`, `green`, `blue` band.

Vacuum may use an explicit exact index of one inside the interaction record; it
is not silently inserted by a query. A later catalogue may generate records,
but the query cannot author coefficients or classifications.

Face binding is deliberate. A pair-wide substance property would falsely make
every boundary between two substances identical and would have no place for a
coating, finish, roughness or different physical interface. Conversely, the
face record does not turn every grid face into a physical surface: only an
explicit record admits that face as the declared interface approximation.

The selected model does not cover conductors, semiconductors, complex index of
refraction, polarization, roughness, microfacets, boundary absorption,
scattering, fluorescence, emission, thin-film interference or layered
interfaces. Declared evidence in those classes returns
`unsupported_interface_model`; missing or mismatched evidence fails closed.

The existing coarse surface-reflectance state is not admissible input. It is
not keyed to path substances or faces and contains no incidence or interface
model.

## Mathematical candidate

For one band, let:

- `d = end - start` be the exact nonzero Q32.32 raw integer path delta;
- `S = dx^2 + dy^2 + dz^2`;
- `a = dot(d, n)` for the reconstructed oriented axis normal, with `a > 0`;
- `eta_i > 0` and `eta_t > 0` be the declared real refractive indices; and
- `q = eta_t / eta_i` be the relative index in the direction of travel.

The incident-angle evidence can avoid premature vector normalization:

`cos_i^2 = a^2 / S`

`sin_i^2 = 1 - cos_i^2`

`sin_t^2 = sin_i^2 / q^2`

If `sin_t^2 >= 1`, the band returns total internal reflection: reflected power
fraction one, transmitted power fraction zero and no transmitted direction.
The exact equality is included in TIR so the undefined grazing transmitted
direction is never fabricated.

Otherwise:

`cos_i = sqrt(cos_i^2)`

`cos_t = sqrt(1 - sin_t^2)`

For the smooth unpolarized dielectric model, the amplitude ratios are:

`r_parallel = (q*cos_i - cos_t) / (q*cos_i + cos_t)`

`r_perpendicular = (cos_i - q*cos_t) / (cos_i + q*cos_t)`

and the reflected power fraction is:

`R = (r_parallel^2 + r_perpendicular^2) / 2`

Inside this explicitly lossless model only, transmitted power fraction is
`T = 1 - R`. This is not a universal interface law or a radiance/presentation
scaling rule.

Let `u = d / sqrt(S)` be the incident unit direction. Its tangential component
is `u_parallel = u - cos_i*n`. The transmitted unit direction is:

`v = u_parallel / q + cos_t*n`

and the reflected unit direction is:

`r = u - 2*cos_i*n`.

The oracle must return directed enclosures for `R`, `T` and every component of
`r` and `v`. The reflected direction is common across bands; transmitted
direction may differ by band because the declared indices may differ. The
event output must state that these are local outgoing directions at the event,
not paths to the original endpoint.

## Numerical policy not yet frozen

The inputs are exact bounded integers and rationals, but the square roots and
Fresnel ratios are generally irrational. A fixed-point format cannot be chosen
honestly from the previous bulk-transfer widths because this calculation adds:

- two square-root layers through path normalization and transmitted angle;
- ratios whose denominators approach zero at hostile grazing configurations;
- squared rational functions for two polarization components; and
- three directed outgoing-vector components per band.

The disposable oracle must use arbitrary-precision integers/rationals plus
directed square-root intervals. It must measure maximum intermediate bit widths
and prove an outward error ceiling before recommending any Q formats, limb
counts, iteration limits or public arithmetic ceiling. Float, epsilon, clamp,
saturation and best-effort output remain forbidden.

If a bounded fixed-width enclosure cannot be made narrow and monotone at the
declared input ceilings, the oracle must recommend either narrower admitted
inputs or retained arbitrary-precision arithmetic. It may not silently reduce
accuracy or substitute Schlick's approximation.

## Closed result classes for the design

The future design may return only one of these semantic classes:

- `no_interface_event`;
- `known_smooth_dielectric_event` with three band results;
- `total_internal_reflection` as a band result;
- `unavailable_evidence`;
- `ambiguous_boundary_lane`;
- `ambiguous_interface_geometry`;
- `missing_interface_evidence`;
- `unsupported_interface_model`; or
- `arithmetic_ceiling` / `nonconvergent_enclosure` after the oracle freezes
  their exact meanings.

Malformed identities, wrong volume, wrong face, reversed media without valid
reconstruction, coefficient forgery and noncanonical state are validation
failures, not physical outcomes.

## Mandatory oracle portfolio

The next package is a disposable arbitrary-precision oracle, not Rust code. It
must cover at least:

1. normal incidence with the analytic reflectance
   `((eta_t-eta_i)/(eta_t+eta_i))^2`;
2. exact index matching with zero reflection and unchanged direction;
3. both directions across the same declared face;
4. above, below and exactly at the critical-angle boundary;
5. grazing incidence approached from admitted exact vectors;
6. band dispersion with three different transmitted directions;
7. power bounds `0 <= R,T <= 1` and outward `R+T = 1` containment;
8. unit-length containment for reflected and transmitted direction bounds;
9. Snell-invariant containment for every transmitted band;
10. reciprocity-compatible forward/reverse fixtures;
11. equal-medium cell subdivision producing no event;
12. face crossing versus edge, vertex, tangent and endpoint-only contact;
13. unavailable, coincident-lane and missing/unsupported profile outcomes;
14. extreme admitted path deltas and refractive-index ratios;
15. deterministic generated cases with retained seeds and worst-width receipt;
16. measured intermediate numerator, denominator and square-root bit widths;
17. proof that no event result claims downstream-cell traversal or endpoint
    arrival; and
18. authority-negative checks for perception, rendering, passage, biome,
    planet, terrain, runtime, approval and promotion.

The oracle should compare interval algorithms, not choose a production
dependency. A passing oracle authorizes only an implementation-readiness
audit, not an interface implementation.

## Adversarial findings

- Multiplying a local interface factor into the old straight witness is invalid
  whenever refraction changes direction.
- Rounding the exact crossing point to Q32.32 before a new query can move the
  origin into the wrong closed cell and invent or omit contacts.
- Treating a diagonal cell transition as one face chooses a false normal.
- Treating all boundaries between a substance pair alike erases face-specific
  interface provenance.
- Reusing bulk extinction or surface reflectance as refractive index conflates
  distinct units and physical processes.
- Returning one RGB direction despite band-varying indices hides dispersion.
- Calling `T = 1-R` universal would overclaim beyond the admitted lossless
  dielectric model.
- A grid interface remains a declared bounded approximation, not proof of
  smooth real-world surface geometry.

## Sources and limits

The mathematical form follows PBRT 4e,
[Specular Reflection and Transmission](https://www.pbr-book.org/4ed/Reflection_Models/Specular_Reflection_and_Transmission)
and [Dielectric BSDF](https://www.pbr-book.org/4ed/Reflection_Models/Dielectric_BSDF).
Those references distinguish bulk transmittance from interface scattering,
derive Snell/Fresnel behavior, identify total internal reflection and separate
smooth dielectric, rough dielectric and conductor cases. They justify the
candidate equations and exclusions only. They do not validate Forge's three
bands, grid faces, coefficients, fixed-point policy or schemas.

Local authority remains the exact `physical-path-substrate` and
`visible-radiance-bulk-transfer` contracts plus the recorded post-transfer
reassessment.

## Exact next action and stop condition

Implement only the disposable arbitrary-precision oracle portfolio above and
record its counterexamples, bit-width maxima and enclosure results. Stop before
creating a Rust crate, interface schema, production coefficient source,
downstream refractive path composer or generic probe system. Do not add
perception, rendering, biome presentation, sphere, planet, terrain, runtime,
C3 closure or promotion authority.

Biome continuity remains unchanged: categorical cells, regions or interface
records cannot paint biome seams. Continuous physical causes must produce
deterministic ecotones; sharp transitions are valid only where sharp physical
causes are evidenced.
