# Geological/Atmospheric Contract v1

This is a capability-free causal seam between exact stellar/orbital evidence
and later derived-world rules. It is a bounded integer reference, not a
scientifically complete planet, atmosphere, geology, climate or habitability
model.

`GeologicalAtmosphericInput` binds a nonzero reconstruction and planetary-body
identity, the exact replayed `StellarOrbitalContract`, planet mass and radius in
milli-Earth units, internal heat flux in milli-watts per square metre,
solid-surface fraction in permille, atmospheric column mass in grams per square
metre, and independent three-band gas and aerosol direct transmissions in
permille. Unknown fields, implicit units, floats, runtime objects and
caller-authored output state are rejected. A zero atmospheric column requires
identity gas and aerosol transmission; attenuation without an atmospheric
column is contradictory and fails closed.

The v1 compiler produces only:

- earth-normalized spherical surface gravity from mass divided by radius
  squared;
- surface pressure from atmospheric column mass multiplied by derived surface
  gravity;
- three-band direct transmission from sequential gas and aerosol attenuation;
- exact retention of declared internal heat flux and solid-surface fraction;
  and
- content-derived identities, limitations and authority-negative state.

The exact input must replay to the exact public state before downstream use.
The geological/atmospheric and nested stellar/orbital reconstruction identities
must match. A plausible public state with a recomputed state identifier is not
sufficient if it does not equal compilation from the retained input.

The surface-gravity relation follows NASA's `g = GM/R^2` reference. The
pressure relation is the bounded column-weight consequence of atmospheric
pressure being force per unit area from the air column under gravity. The
transmission seam follows the Beer-Bouguer-Lambert sequential-attenuation
relation; it does not claim diffuse radiative transfer, wavelength-resolved
chemistry, clouds or multiple scattering.

Primary references:

- NASA Goddard, *The Gravity of the Situation*:
  <https://imagine.gsfc.nasa.gov/observatories/learning/swift/classroom/law_grav_guide.html>
- NASA GISS, *Atmospheric Pressure*:
  <https://www.giss.nasa.gov/edu/icp/education/cloudintro/pressure.html>
- NASA Goddard, *The Beer-Bouger-Lambert Law*:
  <https://acd-ext.gsfc.nasa.gov/anonftp/acd/daac_ozone/Lecture4/Text/Lecture_4/beerslaw.html>

Composition, vertical structure, phase state, weather, climate, hydrology,
materials, mineralogy, tectonics, habitability, biomes, niches, visibility,
traversability, scientific validation and runtime simulation remain outside
this contract.
