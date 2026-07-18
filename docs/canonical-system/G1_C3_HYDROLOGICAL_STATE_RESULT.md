# G1 C3 Hydrological State Foundation Result

## Result

C3 now has a strict bounded hydrological input-state seam between exact
geological/atmospheric evidence and derived-world rules. The new
`hydrological-state` crate binds a declared total water column, exact
solid/liquid/vapor partition, and surface-accessible liquid fraction to the
full nested causal chain.

`WorldGenerationInput` no longer accepts a caller-authored
`liquid_medium_permille`. It consumes the exact `HydrologicalContract` and uses
its validated surface-accessible liquid evidence for medium-dependent signal
checks. The contract nests exact geological/atmospheric and stellar/orbital
state, so plausible caller-authored reservoir or planet state cannot bypass
input replay.

## Bounded proof

- Seven hydrological tests cover deterministic canonical replay, exact scaled
  reservoir evidence, partition and surface-access causality, dry-inventory
  contradictions, surface access without liquid, foreign or fabricated planet
  state, hostile ranges and bytes, plausible fabricated public state and claim
  drift.
- Nine derived-world tests remain green through the complete nested chain.
- Addressable-world, organism-support, opportunity-graph and macro-lineage
  downstream binding suites remain green.
- Forge Desktop passes 41 tests including a read-only hydrological ProofReceipt
  without object, event or candidate authority mutation.
- The complete repository gate passes governance, canonical coherence, all 32
  module fronts, the UI build, workspace tests and all 41 desktop tests. Its
  ordinary final desktop build reaches only the expected live
  `target\debug\forge-desktop.exe` lock; the same desktop build passes in an
  isolated target directory with warnings denied.

## Source boundary

NASA records distinct solid, liquid and vapor water reservoirs and surface,
atmospheric and subsurface storage, while also showing that phase change and
transport are parts of the climate-coupled hydrologic cycle:

- <https://science.nasa.gov/earth/earth-observatory/the-water-cycle/>
- <https://science.gsfc.nasa.gov/earth/climate/researchareas/155/>

Those sources support the reservoir distinction only. The Forge partition is
declared evidence, not an inferred temperature-pressure equilibrium or a
scientific hydrology model.

## Retained limitations

C3 remains active. Temperature-derived phase stability, precipitation,
transport, terrain flow, groundwater dynamics, salinity, climate, materials,
biomes, niches, visibility and traversability remain open. Second-platform
deterministic evidence also remains absent. C4-C6 stay dependency-gated.
