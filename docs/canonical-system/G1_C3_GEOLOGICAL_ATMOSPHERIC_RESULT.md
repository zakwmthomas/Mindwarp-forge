# G1 C3 Geological/Atmospheric Foundation Result

## Result

C3 now has a strict bounded geological/atmospheric input-state seam between
stellar/orbital evidence and derived-world rules. The new
`geological-atmospheric` crate requires exact nested stellar/orbital replay and
matching reconstruction identity before deriving surface gravity, atmospheric
column pressure and three-band direct transmission. It retains declared
internal heat flux and solid-surface fraction with explicit units and
provenance.

`WorldGenerationInput` no longer accepts caller-authored pressure, substrate or
atmospheric transmission as independent trusted values. It consumes the exact
`GeologicalAtmosphericContract`, which nests the exact
`StellarOrbitalContract`. Palette and signal-medium checks use only the
validated derived state.

## Bounded proof

- Seven geological/atmospheric tests cover deterministic canonical replay,
  Earth-normalized gravity and column pressure, independent mass/radius/column
  causality, bandwise gas/aerosol attenuation, foreign stellar evidence,
  hostile ranges and bytes, attenuation without an atmospheric column,
  plausible fabricated public state and claim drift.
- Nine derived-world tests remain green after rebinding pressure, substrate,
  transmission and stellar evidence through the new contract.
- Addressable-world, organism-support, opportunity-graph and macro-lineage
  downstream binding suites remain green.
- Forge Desktop retains a read-only geological/atmospheric ProofReceipt without
  object, event or candidate authority mutation.
- The 125-case causal-range and 32-reconstruction provenance portfolios pass;
  all repository gates, UI build and workspace tests pass. The ordinary final
  desktop build reaches only the running executable's Windows file lock, while
  the warnings-denied isolated-target desktop build passes.

## Source boundary

The implemented relations are deliberately small primary-source-backed
references:

- NASA Goddard gives surface gravitational acceleration as `g = GM/R^2`:
  <https://imagine.gsfc.nasa.gov/observatories/learning/swift/classroom/law_grav_guide.html>
- NASA GISS describes atmospheric pressure as the weight of the air column
  under gravity:
  <https://www.giss.nasa.gov/edu/icp/education/cloudintro/pressure.html>
- NASA Goddard derives sequential Beer-Bouguer-Lambert attenuation:
  <https://acd-ext.gsfc.nasa.gov/anonftp/acd/daac_ozone/Lecture4/Text/Lecture_4/beerslaw.html>

These sources support the bounded relations only. They do not validate the
Forge scaling limits as a scientific planet model.

## Retained limitations

C3 remains active. Atmospheric composition and vertical structure, weather,
climate, hydrology, material/mineral state, tectonics, habitability, biomes,
niches, visibility and traversability remain open. The integer spherical and
column relations are not a production atmosphere, geophysics solver or
scientific validation portfolio. Second-platform deterministic evidence also
remains absent. C4-C6 stay dependency-gated.
