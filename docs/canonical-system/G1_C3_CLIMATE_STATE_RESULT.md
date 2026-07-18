# G1 C3 Climate Plausibility Seam Result

## Result

C3 now has a lightweight scalar climate seam between exact hydrological
provenance and derived-world rules. It uses two bounded dimensionless drivers:
Bond albedo and outgoing-longwave fraction. Checked integer arithmetic produces
normalized absorbed-shortwave and signed radiation-imbalance evidence.

This improves causal believability without adding time steps, spatial grids,
temperature solving, weather, fluid dynamics or universe-wide simulation.

## Bounded proof

- Seven climate tests cover deterministic strict replay, separate driver
  causality, stellar scaling, provenance-only hydrology changes, hostile
  ranges/bytes, fabricated upstream state and claim drift.
- Nine derived-world tests pass through the exact climate-to-stellar chain.
- The workspace type-check passes after downstream fixtures adopt the exact
  climate contract.
- The complete repository gate passes through governance, all 33 module fronts,
  UI build and workspace tests. The ordinary final desktop build reaches only
  the running executable lock; an isolated warnings-denied desktop build passes.

## Iterative improvement

Two registration omissions exposed duplicated manual facts. Module-context
refresh now runs Cargo/boundary validation before generating front doors.
ProofReceipt system IDs are now generated from the canonical system registry,
with a freshness gate and a disposable fixture proving registry-drift rejection
and refresh propagation. Future canonical additions therefore propagate or fail
at registration rather than surfacing late in the full gate.

## Retained limitations

The inputs are procedural descriptors rather than observed or solved climate
physics. Temperature, greenhouse behavior, equilibrium, seasons, weather,
circulation, clouds, materials, biomes, niches, visibility, traversability,
habitability, scientific validation and runtime simulation remain open.
