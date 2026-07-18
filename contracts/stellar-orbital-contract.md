# Stellar/Orbital Contract v1

This capability-free C3 contract establishes the first bounded causal layer
after deterministic universe identity. It accepts explicit integer-scaled
stellar and elliptical-orbit inputs and produces strict, replayable distance,
irradiation and normalized-period-squared evidence. It is not a production
ephemeris or a claim of scientific validation.

## Canonical input

`StellarOrbitalInput` binds a reconstruction identity and stellar-source
identity to these explicit units:

- primary mass in milli-solar masses;
- luminosity in millionths of solar luminosity;
- a three-band spectral distribution whose permille components sum to 1000;
- semi-major axis in milli-AU; and
- eccentricity in millionths, strictly below one.

The contract uses no floats, ambient current version, executable expression,
cache state, orbital solver plug-in, runtime object or aesthetic intent.
Unknown fields, zero identities, invalid scales, non-elliptical eccentricity,
noncanonical bytes and a periapsis below the contract's milli-AU resolution
fail closed.

## Bounded derivation

For this two-body reference, periapsis and apoapsis are derived as
`a(1-e)` and `a(1+e)`. Irradiation is proportional to luminosity divided by
distance squared. The period evidence retains Kepler's normalized
`T^2 = a^3 / M` relation as millionths of an Earth-year squared ratio, avoiding
a lossy integer square root.

The state also provides a bounded three-band stellar irradiance input for the
existing synthetic palette seam. Each band combines the declared spectrum
with mean-distance irradiation and saturates explicitly at 1000 permille; the
unsaturated scalar irradiation remains separately available in millionths of
Earth flux.

These relations follow the bounded elliptical-orbit and inverse-square
references retained in `G1_C3_STELLAR_ORBITAL_RESULT.md`. They do not cover
multi-star systems, perturbations, orbital phase, precession, resonances,
stellar evolution or relativistic effects.

## Trust and authority boundary

`StellarOrbitalState` validates schema, ranges, ordering, limitations,
authority effect and content-derived identity before serialization. Plausible
caller-authored state is still insufficient: `StellarOrbitalContract` must
replay the exact input and reproduce the exact state before any downstream
world compiler accepts it. The reconstruction identity must also match the
containing `WorldGenerationInput`.

The contract grants no approval, promotion, runtime, scientific, geological,
atmospheric, hydrological, climate, material, biome, niche, habitability,
visibility, traversability, aesthetic or representation authority.
