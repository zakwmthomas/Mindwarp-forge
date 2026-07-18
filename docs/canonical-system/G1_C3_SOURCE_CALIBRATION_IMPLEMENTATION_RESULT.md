# G1 / C3 source-calibration implementation result

Date: 2026-07-17

Status: **implemented and fully verified as an owner-approved zero-consumer
capability-free module. No consumer or transport authority was added.**

## Implemented owner

`calibrated-spectral-time-basis` implements the exact frozen input and result
records, reduced canonical unsigned `u128` rationals, stateless identity graph,
strict codecs and 16/32/64 KiB resource ceilings. It uses no float, capability
API, registry or stateful migration.

The production dependencies are only `serde`, `serde_json` and `sha2`. The
legacy optical compiler is a test-only cross-check; no production dependency
points into the optical graph and no current crate imports this new owner.

## Identity and hostile receipts

- input bytes: `896`; SHA-256:
  `c28511856b8f17725deed954f9617a9097342716f8a2cae9349dd1e05ebeb727`
- result bytes: `1,786`; SHA-256:
  `ad6c05e1c5ec1c80060b58b989a6290cec342e50815eddeb271ca23ed54a821f`
- calibrated basis ID:
  `a9913e0d498c2e686574b1a755675d32ce0be3bdc59bf3335cb8d40716684a22`
- legacy time ID:
  `d70f6c4760dbcbd7be0e091c6473082b3b88f11dc9920940bd91c4d8e8c96a79`
- blue/green/red legacy band-time IDs match the unchanged V1 compiler.
- six provenance/version/spectral/time substitutions change the basis, time
  and all band-time identities.
- typed and raw tests reject rational aliases/overflow, invalid ordering,
  gaps/overlaps, zero identities, invalid ticks/scales, unknown and duplicate
  fields, invalid UTF-8, trailing bytes, oversize input/result and forged IDs.

## Platform receipts

- `cargo test -p calibrated-spectral-time-basis --all-targets`: pass
- `cargo test -p calibrated-spectral-time-basis --target i686-pc-windows-msvc`:
  pass, executable
- `cargo check -p calibrated-spectral-time-basis --target aarch64-linux-android`:
  pass
- `cargo fmt --all -- --check`: pass

## Complete Forge verification

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `417.9 seconds`
- Output lines: `2,283`
- Durable files classified: `810`
- Declared modules verified: `51`

## Authority and rollback

This evidence does not allocate source energy, calibrate transport, prove
received energy, drive a detector, visibility, rendering or gameplay, or grant
runtime, promotion or C3 closure. Transport applicability remains blocked on
spatial/real-world coefficient calibration and whole-cell pointwise proof.

Rollback remains deletion-only because the new owner has zero consumers and no
legacy data or identity migrated. After complete verification, reassess and
select at most one consumer separately. Nothing broader is locked in.
