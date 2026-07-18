# G1 C3 Visible-Radiance Path-Transfer Mathematical Design Audit

Date: 2026-07-15

Status: **design selected; disposable exact-arithmetic proof required before
schema or implementation readiness.**

## Decision

The smallest honest first visible-radiance consumer is a bounded direct-beam
**bulk-extinction transfer evidence** calculation over the exact physical path
witness. It must accumulate dimensionless optical depth from declared,
substance-bound three-band extinction coefficients and physical path length,
then return certified transmission bounds rather than one falsely exact scalar.

This first package does not infer opacity from phase and does not reuse the
existing coarse surface-reflectance vector. It also does not yet claim transfer
through material interfaces. A path that requires an undeclared interface
model returns typed unavailable evidence instead of silently treating the
interface as transparent or opaque.

No schema or implementation is ready until a disposable arbitrary-precision
oracle falsifies or confirms the arithmetic and boundary rules below.

## Physical basis and bounded claim

For one homogeneous direct-beam interval and spectral band, the candidate uses

`T = exp(-tau)` and `tau = kappa * length`,

where `kappa` is an extinction coefficient per declared coordinate-length unit.
For piecewise homogeneous intervals, optical depths add before one exponential
evaluation. NIST defines transmittance as transmitted divided by incident
intensity and relates Beer-Lambert absorbance to path length; its attenuation
references use exponential decay with coefficients that depend on photon energy
or spectral conditions:

- <https://nvlpubs.nist.gov/nistpubs/ir/2007/ir7457.pdf>
- <https://physics.nist.gov/PhysRefData/XrayMassCoef/chap2.html>
- <https://www.nist.gov/publications/theoretical-concepts-spectrophotometric-measurements>

The PBRT reference independently treats homogeneous-medium transmittance as
Beer-law extinction over geometric ray length and distinguishes volume
transmittance from the wider scattering problem:

- <https://www.pbr-book.org/4ed/Volume_Scattering/Transmittance>

These sources justify the mathematical shape, not Forge coefficient values,
three-band scientific fidelity, or a complete radiative-transfer claim.

## Typed authority boundary

The future query may bind only exact validated artifacts:

- the exact `PhysicalVolumeV1` and `PhysicalPathWitnessV1` identities;
- a versioned visible-radiance bulk-interaction profile identity;
- exactly three declared visible-band identifiers; and
- for every encountered non-vacuum substance, a profile-owned extinction
  coefficient or an explicit `opaque` marker.

The interaction profile is authoritative evidence keyed by exact
`substance_source_id`. It is not an ad hoc query multiplier. Vacuum has exact
zero bulk extinction. Phase supplies no default. Missing, conflicting, zero-ID,
wrong-volume or wrong-band evidence fails closed.

Coefficient units are inverse **volume coordinate units**. The package does
not call a coordinate unit a metre without a separately validated metric
binding. This keeps the calculation dimensionally closed without fabricating
planet scale. A later SI mapping must be versioned and provenance-bound.

## Canonical geometric reduction

Only positive-length `Interval` records contribute bulk optical depth. Isolated
`Point` contacts have zero bulk length. The consumer reconstructs the open
parameter spans and requires exactly one active positive-length cell over every
span. It then merges adjacent spans with the same interaction evidence before
any rounding.

That merge is a permanent resolution shield: splitting one homogeneous region
into more same-substance cells cannot add rounding operations or alter the
candidate transfer. If more than one positive-length cell is active over an
open span, as for a path lying exactly on a closed cell face or edge, the result
is `ambiguous_boundary_lane`, not an axis-priority choice. Unavailable active
evidence remains unavailable.

Stationary paths contain no bulk distance, but they return unavailable if any
containing cell is unavailable; otherwise their bulk transfer is identity.
Tangent point-only contact with a foreign substance does not create bulk
attenuation or an interface crossing.

## Interface boundary

Existing surface reflectance cannot stand in for interface transmission:
reflectance and transmittance are distinct quantities, the current material
state is not keyed to path substances, and it has no incidence, refraction,
roughness or scattering semantics.

The bulk v1 candidate therefore returns `interface_model_required` whenever
the ordered positive-length medium sequence changes between distinct
substances, including vacuum-to-substance and substance-to-vacuum. It does not
multiply an assumed interface coefficient. A later interface design must own
ordered material pairs, incidence geometry, face/edge/vertex ambiguity,
reflection/refraction and energy bounds. An explicit opaque bulk marker may
terminate a path without pretending to model its surface.

## Deterministic numerical candidate

The implementation candidate uses no floating point:

1. compute the checked squared endpoint delta in raw Q32.32 coordinates;
2. use directed integer square root to bound the full geometric length;
3. form maximal same-profile parameter spans from exact witness rationals;
4. accumulate lower and upper optical-depth bounds with directed integer
   arithmetic, applying rounding only per maximal physical span;
5. sum all bulk optical depth before conversion; and
6. evaluate `exp(-tau)` with a fixed, versioned, directed interval algorithm
   that proves `T_lower <= true T <= T_upper`.

The output retains optical-depth and transmission bounds in declared fixed
formats plus an explicit maximum bound width. It never labels a rounded value
exact. An `opaque` interval yields exact zero transmission in its affected band.

The current path contract permits endpoint deltas whose three squared terms may
exceed `u128` when summed. The consumer may not wrap or silently clamp this
case. The disposable proof must compare a small local wide-integer routine with
an arbitrary-precision oracle and decide whether the reference uses bounded
wide arithmetic or a separately declared query ceiling. No such ceiling may be
smuggled into code after readiness.

## Rejected alternatives

- **one transmission multiplier per cell:** resolution-dependent and repeats
  the removed caller-authored universal-transmission defect;
- **solid means opaque:** contradicted by transparent solids and typed substance
  evidence;
- **reuse surface reflectance as transmission:** wrong quantity and wrong
  provenance;
- **multiply rounded segment transmissions:** order and subdivision can alter
  results; optical depth must accumulate first;
- **ordinary floating point plus epsilon:** unsuitable for canonical identity
  and cross-implementation replay;
- **one rounded fixed transmission scalar:** hides approximation error;
- **choose one cell on face/edge overlap:** turns serialization or axis priority
  into physical truth;
- **full scattering, emission or rendering now:** exceeds the smallest direct
  transfer obligation and mixes physical evidence with presentation.

## Required disposable counterexample proof

Before an implementation-readiness audit, an arbitrary-precision oracle must
test at least:

1. axis and diagonal Q32.32 lengths, perfect squares and adjacent non-squares;
2. maximum admitted coordinate deltas and checked wide-sum overflow;
3. homogeneous-cell subdivision invariance after canonical span merging;
4. two and three different bulk coefficients with exact rational boundaries;
5. forward/reverse optical-depth equivalence;
6. vacuum identity, explicit opaque termination and absent coefficient failure;
7. unavailable intervals and unavailable stationary contacts;
8. tangent point contact versus positive-length thin material;
9. face/edge-aligned ambiguous lanes and vertex crossings;
10. required substance-interface transitions;
11. monotonicity in coefficient and length;
12. certified exponential enclosures at zero, small, ordinary, saturation and
    maximum optical depth; and
13. a hostile search for a rounded result outside its declared interval or a
    result changed solely by same-substance cell subdivision.

The proof must report worst bound width, arithmetic size, canonical byte cost
and a safe fixed ceiling. A failed enclosure, invariance or overflow test stops
the package; it cannot be repaired with an epsilon or a wider claim.

## Output and nonclaims

The future output is observer-independent direct-beam transfer evidence for a
declared path and three visible bands. It is not emitted radiance, received
irradiance, inverse-square spreading, diffuse light, scattering, reflection,
refraction, color appearance, perception, detectability, line-of-sight gameplay
policy, rendering or runtime visibility.

It defines no sphere, planet, terrain, navigation, organism, biome or material
palette. Categorical cells and physical regions still cannot paint visible
biome seams; continuous causal fields must create deterministic ecotones, and
sharp transitions remain sharp only when a sharp physical cause supports them.

## Exact next action

Build only the disposable arbitrary-precision bulk-transfer oracle and hostile
portfolio specified above. Use it to select the wide-integer, directed-square-
root and certified-exponential bounds before an implementation-readiness audit.
Do not create a Rust crate, durable schema, coefficients, interface model or C3
closure from this design record.
