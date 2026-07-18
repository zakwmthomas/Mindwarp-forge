# Hydrological State Contract v1

This is a capability-free causal seam between exact geological/atmospheric
evidence and later climate or derived-world rules. It is a bounded inventory
contract, not a phase-equilibrium solver, hydrologic-cycle simulation,
scientific-validation claim or runtime system.

`HydrologicalInput` binds a nonzero reconstruction and hydrological-source
identity, the exact replayed `GeologicalAtmosphericContract`, total water
column in grams per square metre, a declared solid/liquid/vapor partition in
permille, and the declared surface-accessible fraction of the liquid
reservoir. A nonzero inventory must partition to exactly 1000 permille. A dry
inventory must have zero partition and surface access. Surface access without
a liquid reservoir fails closed.

The compiler retains exact scaled numerators rather than rounding reservoir
quantities:

- each phase column is stored in thousandths of grams per square metre as
  `total_water_column_g_m2 * phase_partition_permille`;
- surface-accessible liquid is stored in millionths of grams per square metre
  as `liquid_phase_column_thousandths * accessible_fraction_permille`; and
- liquid-medium availability is true only when that exact numerator is
  nonzero.

The partition and accessibility are declared evidence. The contract does not
derive them from temperature, pressure, terrain or chemistry. NASA describes
water as stored in solid, liquid and vapor reservoirs and moving among surface,
atmosphere and subsurface stores, while phase changes and transport participate
in weather and climate. This contract retains the reservoir distinction while
leaving those coupled processes open:

- NASA Earth Observatory, *The Water Cycle*:
  <https://science.nasa.gov/earth/earth-observatory/the-water-cycle/>
- NASA Goddard, *Water Cycle and Precipitation*:
  <https://science.gsfc.nasa.gov/earth/climate/researchareas/155/>

Temperature-derived phase stability, evaporation, condensation,
precipitation, runoff, infiltration, groundwater dynamics, ocean geometry,
terrain flow, salinity, climate, materials, habitability, biomes, niches,
visibility, traversability and runtime simulation remain outside this
contract.
