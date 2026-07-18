# G1 / C3 interval optical cell-step implementation result

Date: 2026-07-16

Status: **implemented and verified as one additive channel-neutral local proof;
downstream interval bulk and composition remain unapproved.**

## Result

`physical-path-substrate` now exposes a separately versioned conditional
interval cell-step surface from a private `interval` module. It accepts one
validated physical volume, current cell, declared Q160 point box and declared
Q1.62 direction box. Fixed-160 checked signed-magnitude arithmetic certifies a
face only by strict whole-box ordering and returns typed ambiguity,
no-forward-progress, outer-exit and unavailable-neighbour outcomes.

The implementation retains the 414-bit signed live-value shield inside
512-bit storage, six-face/12-division/30-comparison work ceilings and 16/32 KiB
strict codec caps. It adds only the exact already-resolved
`crypto-bigint = 0.7.5` default-features-disabled direct dependency. No native
limb codec, float, adaptive precision, filesystem, network, process, optical
reverse dependency or end-to-end lineage claim exists.

## Compatibility and hostile evidence

Five exact-path V1 families permanently recompute and lock canonical byte
lengths, SHA-256 values and all public IDs for straight traversal, reversal,
simultaneous contact, stationary evidence and negative/near-maximum
coordinates. Hostile tests cover face direction reversal, one-Q1.62-unit
ordering, ties, correlation erasure, zero and zero-straddling directions,
prior-face zero progress, minimum direction, all six outer exits, unavailable
evidence, invalid boxes, decimal/scale/provenance poison, codec caps and the
near-maximum arithmetic fixture.

## Verification receipt

- Native Windows warnings-denied: 13 legacy + 1 V1 freeze + 5 interval tests.
- i686 Windows execution: the same 19 tests passed.
- Android ARM64: `cargo check -p physical-path-substrate --target aarch64-linux-android` passed.
- Downstream unchanged suites: swept AABB 9, radiance bulk 12 and interface 20 tests passed.
- Independent oracle: 248/256 certified, eight typed ambiguities, four 64-step lanes and 321 observed live bits.
- Permanent source shield: `tools/verify-g1-c3-interval-cell-step-implementation.ps1` passed.

Actual device profiling remains later promotion evidence. This result does not
authorize interval bulk transfer, path composition, endpoint arrival,
visibility completion, collision response, navigation, organism behavior,
biome meaning, sphere, planet, terrain, runtime or C3 closure.
