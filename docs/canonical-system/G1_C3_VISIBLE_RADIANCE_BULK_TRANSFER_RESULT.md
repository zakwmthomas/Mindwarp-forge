# G1 C3 Visible-Radiance Bulk-Transfer Result

Date: 2026-07-16

Status: **owner-authorized bounded reference implemented and prototype-tested;
C3 remains executing.**

## Result

The isolated `visible-radiance-bulk-transfer` crate now reconstructs exact
physical-volume profiles and path witnesses into three-band observer-independent
direct-beam bulk-transfer evidence.

Known v1 transfer is deliberately limited to vacuum or one exact substance.
Positive-length material transitions return `interface_model_required`,
unavailable cells return `unavailable_evidence`, and paths coincident with
multiple closed cells return `ambiguous_boundary_lane`. Point-only contacts do
not attenuate or invent interfaces. Stationary paths are exact identity unless
their containing evidence is unavailable.

This boundary avoids the oracle's 337-bit mixed-medium accumulator and adds no
bespoke wide-integer subsystem. Admitted known transfer uses checked `u128`
after an explicit three-square length ceiling, directed integer square-root
length bounds, two-limb Q64.64 optical-depth bounds and directed Q0.48
transmission bounds.

## Proof

Twelve warnings-denied focused tests pass:

- strict input/profile/query/transfer replay;
- complete canonical substance coverage and identity failures;
- vacuum, finite-zero, positive finite and per-band opaque evidence;
- same-substance subdivision and forward/reverse invariance;
- unavailable, ambiguous-lane and interface-required outcomes;
- stationary unavailable and tangent point-contact behavior;
- perfect-square/nonsquare length bounds and squared-length ceiling;
- coefficient/length monotonicity and seven exact Python-oracle vectors;
- forged state, unknown fields and noncanonical bytes;
- maximum-profile cost and authority-negative claims.

The retained independent Python oracle also passes 1,570 checks: 512 random
exponential enclosures, 768 random transfer enclosures and 256 subdivision-
invariance cases. Rust's seven frozen Q0.48 vectors match it exactly.

## Cost receipt

The maximum profile contains 65,536 exact substance entries and reconstructs a
65,536-cell volume. In a warnings-denied debug test on the current PC it
produced 29,041,272 canonical profile bytes in 38,955 ms. The full 12-test
focused package completed in 39.14 seconds. These are bounded proof costs, not
runtime or production-performance claims.

## Integration

The crate depends only on `physical-path-substrate`, `serde`, `serde_json` and
`sha2`. Module governance forbids Forge Kernel, Tauri/UI, network, filesystem
and process imports. Its generated `MODULE.md`, canonical contract, system
registry entry and retained C3 verifier make the boundary permanent. The C3
gate runs both the Rust tests and independent Python oracle.

## Failure containment

- Callers submit no cells, spans, optical depth, classification or output.
- Profile compilation rebuilds the volume and requires exact canonical
  coverage of every non-vacuum substance.
- Transfer compilation rebuilds the profile, volume, query and path witness.
- Phase and coarse surface reflectance supply no opacity or interface defaults.
- Same-substance subdivision cannot add rounding steps.
- All arithmetic is checked; no float, epsilon, clamp or partial result exists.
- Strict decoders and validators reject plausible forged state and authority
  drift by complete reconstruction.

## Nonclaims and next route

This result supplies no real coefficient catalogue or scientific validation,
SI/metre mapping, interface reflection/refraction, scattering, emission,
perception, rendering, gameplay line of sight, passage, navigation, organism,
biome, planet, terrain, storage or runtime behavior. It neither closes C3 nor
promotes the prototype.

Biome continuity is unchanged: categorical cells and regions cannot paint
visible biome seams. Continuous causes must produce deterministic ecotones;
sharp transitions remain sharp only where supported by sharp physical causes.

The next safe C3 action is a post-transfer reassessment of the separately open
interface-optics and generic-probe passage routes. No implementation, closure
or promotion follows automatically.
