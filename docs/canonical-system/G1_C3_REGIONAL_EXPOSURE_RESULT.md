# G1 C3 Regional Exposure Result

## Result

The loose `field_recipe_id` in `WorldGenerationInput` has been replaced by an
exact `RegionalEnvironmentContract`. Canonical recipe bytes, reconstruction
identity and Q32.32 coordinates now cause one bounded regional exposure
fraction before derived-world compilation.

## Bounded proof

- Seven regional-state tests cover strict replay, exact endpoint
  normalization, coordinate-caused variation, range rejection, invalid recipe
  and identity rejection, hostile bytes, fabricated state and claim drift.
- Ten derived-world tests prove that coordinates can change physical palette
  and visible-radiance strength while leaving an unrelated pressure-wave
  channel unchanged.
- Twenty-one affected downstream tests and all 41 desktop tests pass through
  the exact regional contract.
- The disposable portfolio passes 125/125 range cases, preserves 32/32
  reconstruction provenance identities, and produces 31 distinct exposures
  plus 30 distinct palettes across 32 coordinates.
- The retained whole-chain reconstruction helper now rebuilds the regional
  contract too, extending the `WL-032` regression guard.
- The complete repository gate passes governance, canonical coherence, all 35
  module fronts, UI build and workspace tests. Its ordinary final desktop
  build reaches only the known live-executable lock; an isolated desktop build
  with warnings denied passes.

## Evidence boundary

USGS records that terrain aspect and its angular relation to the Sun can
materially affect radiometric response. NASA technical evidence describes
using terrain slope, azimuth, illumination angle, horizons and view factors to
determine incoming radiation. These sources justify only spatially varying
exposure as a useful abstraction; the Forge field is procedural and is not a
scientific terrain or radiation model.

- https://www.usgs.gov/publications/digital-elevation-data-aid-land-use-and-land-cover-classification
- https://ntrs.nasa.gov/citations/19910030222

## Retained limitations

No terrain geometry, slope, aspect, shadow, cloud, weather, radiative transfer,
physical visibility distance, biome, traversability, habitability or runtime
claim is made.
