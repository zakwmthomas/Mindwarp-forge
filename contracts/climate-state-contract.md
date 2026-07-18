# Climate State Contract v1

This is a capability-free scalar plausibility seam, not a climate or universe
simulation. It binds the exact hydrological contract and its complete upstream
chain, a declared Bond-albedo fraction, and a declared outgoing-longwave
fraction of incident irradiance.

The compiler retains checked integer numerators in explicit normalized units:

- incident shortwave is the stellar/orbital mean-distance irradiance numerator
  over four million Earth-normalized irradiance units;
- absorbed shortwave multiplies incident irradiance by `1000 - albedo` and is
  stored over four billion units;
- outgoing longwave multiplies the same incident irradiance by its declared
  permille fraction; and
- net radiation is the signed difference between those two numerators.

The factor of four represents global spherical averaging only. NASA describes
the radiation budget as incoming shortwave balanced by reflected shortwave and
outgoing longwave, with global mean incoming sunlight one quarter of the
top-of-atmosphere irradiance:

- <https://science.nasa.gov/ems/13_radiationbudget/>
- <https://ceres.larc.nasa.gov/science/>

Albedo and outgoing-longwave fractions are declared procedural descriptors.
The contract does not solve temperature, greenhouse behavior, weather,
circulation, clouds, hydrological transport, phase equilibrium, materials,
habitability, biomes, niches, visual quality or runtime evolution.
