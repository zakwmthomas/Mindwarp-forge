# G1 / C3 source-quantity basis oracle result

Date: 2026-07-17

Status: **band/time-integrated radiant energy in joules survives as the
physical quantity primitive; physical composition remains blocked on a
calibrated spectral/time basis. No schema or source is authorized.**

## Result

The exact-rational oracle passed twice with byte-identical output. A
nonnegative radiant-energy quantity integrated over one exact spectral basis
and half-open time interval conserves exactly through nonuniform subdivision
to 4, 16 and 64 leaves, adds across disjoint time cells, and composes with an
exactly matching dimensionless transfer enclosure without sampling.

The alternatives failed for distinct reasons:

- average radiant power loses temporal correlation unless emission and
  transfer share a proven piecewise-constant partition;
- normalized non-SI quantity retains arbitrary scale and cannot close physical
  C3 claims; and
- radiance density still lacks projected physical area, solid angle and the
  phase-space Jacobian.

The oracle rejected spectral, temporal and calibration identity mismatches.
This exposes the remaining blocker: current RGB enum and opaque time identity
cannot prove that source energy and transfer integrate the same physical
subject.

## Pinned receipt

- Oracle: `tools/prove-g1-c3-source-quantity-basis.py`
- Oracle SHA-256:
  `a17f636c3387824680b767d89845fe7b2a97b1a7fe33979f497fa09a9b74cd90`
- Receipt checksum:
  `f3fe98d6e80f670859c32a42d24636027385d31cecde081c03dcd6e6ce305b59`
- Hostile rejections: `21`
- Conservation checks: `3`
- Leaf counts: `4`, `16`, `64`

## Complete Forge verification

The complete canonical gate passed after the current calibrated-basis route
was added to the historical C3 reassessment shields:

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `438.1 seconds`
- Output lines: `2,208`
- Durable files classified: `793`
- Declared modules verified: `50`

Zero source, zero coupling and unresolved transfer remain distinct. Exact
positive transfer products that later project to zero remain numerical
underflow, not physical zero.

## Decision

Advance one bounded step to a code-free **calibrated spectral/time basis
mathematical design audit**. It must define exact spectral intervals,
integration weighting, physical time intervals/duration, calibration
provenance, and an additive mapping to the existing transport band/time
identity without mutating V1 bytes.

Do not proceed to source-quantity schema readiness yet. Add no crate,
dependency, contract schema, production test or production source. Select no
universal RGB boundaries, tick duration or source emission model. Do not claim
received power, irradiance, radiance, detector response, visibility, runtime,
promotion or C3 closure.

Nothing broader is locked in. One consumer first, reassess before expanding.
