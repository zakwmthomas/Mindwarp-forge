# G1 C3 stellar/orbital foundation result

Status: **bounded stellar/orbital prototype tested; C3 remains active and incomplete.**

## Implemented boundary

`crates/stellar-orbital` adds the first explicit C3 causal layer above
deterministic identity. A strict `StellarOrbitalInput` records stellar mass,
luminosity, a bounded three-band spectrum, semi-major axis and eccentricity
using named integer scales. Compilation produces:

- periapsis and apoapsis in milli-AU;
- inverse-square irradiation bounds in millionths of Earth flux;
- mean-distance irradiation;
- normalized orbital-period-squared evidence; and
- an explicitly saturated three-band input for the existing synthetic palette
  seam, while retaining the unsaturated scalar flux separately.

The reference uses the JPL/NASA elliptical relationships
`periapsis = a(1-e)` and `apoapsis = a(1+e)`, Kepler's normalized third-law
relationship, and NASA's inverse-square irradiation relationship:

- https://spsweb.fltops.jpl.nasa.gov/portaldataops/mpg/MPG_Docs/MPG%20Book/Release/Chapter7-OrbitalMechanics.pdf
- https://www.jpl.nasa.gov/edu/resources/lesson-plan/pi-in-the-sky-11/
- https://www.nasa.gov/stem-content/the-inverse-square-law-of-light/

These sources establish the bounded relations, not scientific sufficiency for
the eventual universe generator.

## Adversarial proof

Seven focused module tests pass:

1. deterministic strict encode/decode and exact replay;
2. circular Earth-normalized distance, flux and period-squared vectors;
3. eccentric distance and inverse-square flux ordering;
4. luminosity changes flux without fabricating orbit changes;
5. distance reduces flux while increasing period-squared evidence;
6. invalid ranges, unknown fields, noncanonical bytes, claim drift and
   identity drift fail closed; and
7. a plausible fabricated public state with a recomputed state identity is
   rejected by exact input replay.

`derived-world-rules` now embeds the exact stellar/orbital input/state contract,
requires its reconstruction identity to match the world input, and derives the
stellar palette cause from validated state. Its nine focused tests, every
affected downstream binding test and all 39 desktop tests pass, including a
read-only `stellar-orbital` ProofReceipt/Reference Studio fixture. The disposable
125-case range portfolio and 32-reconstruction provenance portfolio also pass
with zero unexpected failures.

## Explicit non-claims and next layer

This result does not implement multi-star dynamics, perturbations, phase,
precession, resonances, stellar evolution, production ephemerides or
scientific validation. It also does not implement geology, atmosphere,
hydrology, climate, materials, biomes, niches, visibility or traversability.

C3 therefore remains active. The next bounded proof is a strict geological and
atmospheric input seam that consumes exact stellar/orbital evidence while
keeping hydrology, climate and later layers explicitly open.
