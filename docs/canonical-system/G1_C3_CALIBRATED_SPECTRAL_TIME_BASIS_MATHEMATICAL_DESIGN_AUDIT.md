# G1 / C3 calibrated spectral/time basis mathematical design audit

Date: 2026-07-17

Status: **oracle-ready as a versioned additive calibration witness beside the
unchanged V1 band/time identity. Physical transfer composition remains
blocked until a transport owner supplies whole-cell spectral/time validity;
no schema or source is authorized.**

## Inherited boundary

The source-quantity oracle selected nonnegative band/time-integrated radiant
energy in joules. The current transport owner instead binds
`VisibleRadianceBandV1::{Red,Green,Blue}` and a nonzero opaque
`time_basis_id`; `band_time_id` hashes only that legacy pair. No current byte
defines wavelength bounds, spectral weighting, time origin, duration or
real-world coefficient validity.

This design therefore cannot reinterpret V1. It may only add a separate,
versioned witness that correlates one exact physical basis with one unchanged
legacy pair. The legacy identity remains a correlation key, not physical
calibration.

## Primary metrology constraints

NIST lists radiant energy in joules and distinguishes spectral radiometric
quantities as densities with respect to wavelength, frequency or wavenumber.
Changing the independent variable follows the ordinary substitution rules;
the density and its Jacobian cannot be silently retained. BIPM also warns that
a unit name alone does not identify the physical quantity.

Primary references:

- NIST, *Introduction to Optical Radiometry / Radiometric Terminology*:
  <https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=104704>
- BIPM, *The International System of Units (SI Brochure), 9th edition*:
  <https://www.bipm.org/documents/20126/41483022/SI-Brochure-9.pdf>

These sources constrain quantity and coordinates. They do not select Forge
RGB boundaries, a tick duration, an epoch, a source spectrum or transport
coefficients.

## Candidate comparison

### 1. Reinterpret the legacy RGB/time bytes - rejected

Assigning wavelengths or seconds directly to an existing enum or opaque ID
would change the meaning of already frozen bytes without changing their
identity. The same historical record could then acquire different physical
meaning under a later table. Identity equality would be mistaken for
calibration equality.

### 2. Overlapping response-weighted RGB channels - rejected for additive energy

Overlapping color-matching or detector-response functions can be useful for a
separately typed response quantity. They do not form a disjoint measure
partition: summing channel values can count the same radiant energy more than
once, and the result depends on the response functions. They cannot be called
an additive partition of radiant energy in joules.

### 3. Band-centre or average-transfer calibration - rejected

A representative wavelength, average coefficient or average transfer loses
source-spectrum correlation. Two sources with the same integrated band energy
can place all energy in different sub-bands. If one sub-band transmits and the
other does not, received energy differs even though the average and total
source energy are identical. The same counterexample applies across time.

### 4. Versioned disjoint spectral/time calibration witness - selected for oracle

The survivor is a separate immutable witness with:

- quantity kind `radiant_energy` and unit `joule`;
- spectral coordinate `vacuum_wavelength_metre`;
- three ordered, contiguous, nonempty half-open intervals forming one declared
  covered domain: blue, then green, then red;
- unit energy integration weighting, not a detector or perceptual response;
- one nonzero time-coordinate origin identity, exact integer start/end ticks,
  positive exact rational seconds per tick and a half-open interval;
- one nonzero calibration provenance identity and positive basis version;
- an explicit mapping from each physical interval/time cell to exactly one
  unchanged legacy `(band,time_basis_id)` pair; and
- a derived calibration-witness identity that includes every field above.

No numerical wavelength endpoint, clock origin, duration or tick scale is
selected here. Exact values in the disposable oracle are hostile fixtures
only and have no canonical or gameplay authority.

## Alias and version rule

Within one calibration authority domain, a legacy `(band,time_basis_id)` pair
may resolve to at most one physical witness identity. A different interval,
tick scale, origin, provenance or version requires a different witness and
cannot coexist as another meaning for the same legacy pair. Historical V1
records remain unchanged; physical composition must name the witness
explicitly and fail on absence or ambiguity.

A new calibration version is not an in-place correction. It is a new physical
basis. Quantities from different versions remain separately typed unless a
later exact conversion proof maps them.

## Additive spectral and temporal mapping

For one source cell and one calibrated time cell, let `Q_b`, `Q_g` and `Q_r`
be integrated radiant energies over the disjoint blue, green and red
intervals. The energy over the declared covered domain is exactly:

`Q_covered = Q_b + Q_g + Q_r`.

Adjacent calibrated time cells with the same spectral basis, time origin and
seconds-per-tick rational add in the same way. Overlap duplicates energy; a
gap invalidates a complete-covered-domain claim; a boundary reversal or
different time scale is not composable. Energy outside the declared covered
domain is unclaimed, not zero.

Vacuum wavelength is fixed as the coordinate so wavelength/frequency
reparameterization cannot reuse density values without the required Jacobian.
The witness stores integrated energy, not spectral density.

## Transport applicability theorem and remaining block

For nonnegative spectral-temporal energy measure `dQ` over calibrated cell
`B x I`, multiplication by one dimensionless interval `[l,u]` is conservative
only if a transport applicability receipt proves, for every wavelength and
instant in that cell:

`l <= T(lambda,t) <= u`.

Then exact monotonicity gives:

`l * Q(B,I) <= integral T(lambda,t) dQ <= u * Q(B,I)`.

This is a whole-cell pointwise enclosure, not a sample, midpoint or
energy-independent average. An effective weighted transfer is admissible only when the identical
source distribution and weighting are owned and proven; the current source
primitive deliberately owns only integrated energy.

The current visible-radiance bulk contract supplies neither pointwise
spectral/time validity nor real-world coefficient calibration, and it
explicitly owns coefficients per abstract volume coordinate unit rather than
an SI metre mapping. Therefore the new basis witness can calibrate the source
subject but cannot by itself turn current dimensionless transport into a
physical received-energy claim. A future applicability owner must bind the
exact calibration witness, transport profile/revision, spatial calibration
and conservative whole-cell validity.

## Exact oracle obligations

The disposable exact-rational oracle must prove:

1. exact RGB partition and adjacent-time addition without overlap or loss;
2. deterministic calibration identity and legacy-pair uniqueness;
3. physical-basis mismatch rejection across interval, coordinate, weighting,
   origin, ticks, tick scale, provenance and version;
4. conservative integrated-energy bounds for multiple nonnegative
   spectral/time allocations when a pointwise receipt exists;
5. equal-total-energy spectral and temporal correlation counterexamples to a
   scalar average or representative sample;
6. rejection of overlapping response-weighted channels as additive joules;
7. absence, ambiguity and sample-only transport applicability rejection; and
8. canonical rational, boundary, duration and alias hostile cases.

## Stop boundary

Run the deterministic exact-rational oracle only. Add no crate, dependency,
contract schema, production test, production source, calibration registry or
runtime path. Select no RGB numbers, tick duration, clock origin, emission
model, coefficient catalogue, spatial scale or detector response. Do not
claim physical received energy, power, irradiance, radiance, visibility,
perception, runtime, promotion or C3 closure.

If the witness survives, the next action is a code-facing calibrated-basis and
transport-applicability schema gap audit. It must identify exact ownership and
whether the source basis can advance independently while physical composition
remains blocked. It does not authorize implementation.

Nothing broader is locked in. One consumer first, reassess before expanding.
