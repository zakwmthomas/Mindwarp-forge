# G1 / C3 source-quantity basis mathematical design audit

Date: 2026-07-17

Status: **oracle-ready for band/time-integrated radiant energy as the physical
quantity primitive; existing RGB/time identity is not calibrated enough for
physical composition, so schema readiness remains blocked.**

## Metrology boundary

NIST optical-radiometry definitions distinguish radiant energy, radiant flux
or power, irradiance and radiance. BIPM defines the joule as energy and the
watt as joule per second. Radiance additionally requires projected area and
solid angle. These definitions constrain the candidates but do not provide a
source emission model, spectral bands or time intervals for Forge.

Primary references:

- NIST, *Introduction to Optical Radiometry / Radiometric Terminology*:
  <https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=104704>
- BIPM, *The International System of Units (SI Brochure), 9th edition*:
  <https://www.bipm.org/documents/d/guest/si-brochure-9-en-pdf>

## Common subject

Every candidate concerns one exact source, scope, reconstruction, revision,
phase-space root/cell ancestry, spectral basis and time interval. It must
preserve the already proven independent additive quantity measure:

`Q(parent) = Q(lower child) + Q(upper child)`.

The physical basis cannot be inferred from the abstract geometric measure.
Zero quantity is canonical and unresolved allocation is retained explicitly.

## Candidate comparison

### 1. Band/time-integrated radiant energy — selected primitive

`Q_e(C,B,I)` is radiant energy in joules emitted into cell `C`, within exact
spectral basis `B`, during exact half-open time interval `I=[t0,t1)`. It is a
nonnegative exact rational. Disjoint phase-space cells, disjoint spectral
intervals and disjoint time intervals add without dividing by abstract cell
measure.

For an exactly matching dimensionless transfer enclosure `T=[l,u]`, algebraic
composition is:

`Q_received in [Q_e*l, Q_e*u] joules`.

Zero transfer retains the source energy in a typed zero-coupled bucket;
unresolved transfer retains it unresolved. Exact positive products that later
project to zero remain numerical underflow.

The quantity basis must name joules explicitly. Spectral definitions use exact
half-open wavelength intervals with an explicit SI scale and integration
weighting; time definitions use exact half-open intervals and an explicit
seconds-per-tick rational. This design selects no universal RGB boundaries or
simulation tick duration.

### 2. Radiant power — rejected as the primary stored quantity

Power in watts is energy per second. A constant power plus exact duration can
derive energy, but average power over a coarse interval loses correlation with
time-varying emission and transfer. Two sources can have equal average power
while emitting in opposite halves of an interval; if transfer differs between
the halves, received energy differs.

Power is therefore a derived view only when a later owner proves the required
steady or piecewise-constant assumptions and binds the identical time
partition. It is not the canonical source-cell quantity.

### 3. Explicitly normalized non-SI quantity — rejected for C3 physics

A normalized quantity can preserve addition and deterministic gameplay ratios,
but its scale is arbitrary. Changing the reference scale changes the inferred
joules without changing stored values. It may support a separately named
synthetic system, but cannot close the physical visible-radiance gap and cannot
be called energy, power or radiance.

### 4. Radiance density — rejected at the current boundary

Radiance requires radiant flux per projected physical area and solid angle.
The existing phase-space cell measure owns neither and has no physical
Jacobian. A density over that measure changes meaning under coordinate
reparameterization. Radiance remains a falsification control, not a survivor.

## Spectral and temporal compatibility blocker

The selected energy primitive requires physical basis equality, not merely
opaque identity equality. A basis identity must cover at least:

- quantity kind `radiant_energy` and unit `joule`;
- exact wavelength lower/upper endpoints, SI scale, half-open convention and
  integration weighting;
- exact time start/end ticks, seconds-per-tick rational and half-open
  convention;
- calibration/provenance identity and basis version; and
- the transport band mapping whose coefficients are valid for that exact
  spectral basis.

Current `VisibleRadianceBandV1::{Red,Green,Blue}` has no canonical wavelength
boundaries or integration weighting. Current `time_basis_id` has no duration
or interval. Consequently the existing `band_time_id` can correlate records
but cannot prove that transfer and energy integrate the same physical subject.
The oracle must reject composition across either mismatch.

This is the next blocker after the quantity candidate survives. It must be
resolved additively; existing optical V1 bytes and identities cannot be
reinterpreted.

## Exact arithmetic model

The disposable oracle uses reduced nonnegative rational joules. Canonical zero
is `0/1`; denominators are positive; negative, zero-denominator, leading-zero,
signed-zero and non-reduced forms fail. It proves:

1. nonuniform 4-, 16- and 64-leaf radiant-energy conservation, including zero
   children;
2. exact addition over disjoint time cells;
3. matching-basis transfer enclosure composition;
4. zero source, zero coupling, unresolved transfer and exact-positive
   projection underflow as distinct cases;
5. equal-average-power temporal-correlation failure;
6. normalized-scale ambiguity and radiance reparameterization failure;
7. spectral/time/basis/root/cell/ancestry substitution rejection; and
8. atomic split duplication, deletion and parent-total mismatch rejection.

## Stop boundary

Run the exact-rational oracle only. Add no crate, dependency, contract schema,
production test or production source. Do not select RGB wavelength boundaries,
tick duration, emission model or calibration source. Do not claim received
power, irradiance, radiance, detector response, visibility, runtime, promotion
or C3 closure.

If radiant energy survives, advance to one code-free calibrated spectral/time
basis design audit. That design must decide how exact physical band/time bases
map additively to the existing transport enum without mutating V1 identities.

Nothing broader is locked in. One consumer first, reassess before expanding.
