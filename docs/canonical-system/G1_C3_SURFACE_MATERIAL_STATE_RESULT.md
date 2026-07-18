# G1 C3 Surface Material Plausibility Result

## Result

The loose dominant-surface reflectance vector has been replaced by an exact
`SurfaceMaterialContract` nested after climate and before derived-world rules.
The contract retains only three bounded reflectance fractions because no other
material property has a justified current consumer.

## Bounded proof

- Seven tests cover strict replay, reflectance causality, climate provenance
  separation, hostile ranges and bytes, fabricated upstream state, public-state
  drift and claim drift.
- Derived-world palette computation consumes the validated material state.
- The 125-case range and 32-reconstruction provenance portfolios pass with zero
  unexpected failures.
- The portfolio exposed a partial reconstruction helper that did not rebuild
  newer climate and material bindings. It now rebuilds the whole causal chain,
  and a retained regression test checks every nested reconstruction binding
  before compilation (`WL-032`).
- The complete repository gate passes governance, canonical coherence, all 34
  module fronts, UI build and workspace tests. Its ordinary final desktop build
  reaches only the known live-executable lock; an isolated desktop build with
  warnings denied passes.

## Retained limitations

This is not a composition model, spectral library, BRDF, chemistry or material
simulation. Materials remain coarse plausibility evidence; biomes, niches,
visibility, traversability, habitability and runtime behavior remain open.
