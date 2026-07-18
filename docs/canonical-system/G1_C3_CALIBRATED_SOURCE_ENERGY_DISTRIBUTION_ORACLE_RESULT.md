# G1 / C3 calibrated source-energy distribution oracle result

Date: 2026-07-17

Status: **the closed-frontier additive calibrated radiant-energy measure
survives exact-rational falsification. It remains code-free and has no
transport, detector or implementation authority.**

## Result

The deterministic oracle passed twice with byte-identical output. One exact
root radiant energy can be represented by a prefix-free closed frontier of
phase-space cells whose measures cover the root exactly once and whose
nonnegative energies sum exactly to the root energy.

Nonuniform refinements preserved both geometric measure and radiant energy at
4, 16 and 64 leaves. Sixty-three atomic split receipts bound each parent to
both children, including zero-energy children. A mixed-depth frontier retained
unresolved detail at a coarser cell without treating absence as zero or losing
energy.

Independent leaf records failed because omission, duplication and
ancestor/descendant overlap cannot prove complete single coverage. Density
with respect to abstract cell measure survived only as a derived
coordinate-local average: rescaling the uncalibrated measure changed density,
and equal parent density hid opposite child allocations.

## Identity and semantic separations

The oracle bound the implemented calibrated-basis identity
`a9913e0d498c2e686574b1a755675d32ce0be3bdc59bf3335cb8d40716684a22`,
the selected derived band/time identity, distinct source and calibration
provenance, the known phase-space root identity, reconstruction, cell paths,
cell identities, measures, energy values and resolution states.

Equal calibration with different source energy produced different
distribution identities. Equal source energy with opposite phase-space
allocation also produced different identities. Zero source with positive
transfer and positive source with zero coupling remained distinct typed cases
even though both algebraic products were zero.

## Pinned receipt

- Oracle: `tools/prove-g1-c3-calibrated-source-energy-distribution.py`
- Oracle SHA-256:
  `e76be9bfcdf80543529baea94f70acf3455257e33c1e97871be4b2ecdc018553`
- Receipt checksum:
  `33edaae6b5733b50f8c46592eee80664d5361a3613191844dcbd3ce58ed2edd6`
- Portfolios: `8`
- Hostile rejections: `32`
- Conservation checks: `3`
- Leaf counts: `4`, `16`, `64`
- Atomic split receipts: `63`
- Canonical representation: `prefix_free_closed_frontier`
- Density status: `derived_coordinate_local_average_only`
- Unresolved allocation: `retained_at_coarser_frontier_cell`

The oracle's source identities, energies and allocation ratios are disposable
hostile fixtures. They select no canonical source behavior.

## Complete Forge verification

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `403.1 seconds`
- Output lines: `2,277`
- Durable files classified: `816`
- Declared modules verified: `51`

## Surviving claim

Only this claim survives: exact band/time-integrated radiant energy in joules
can be bound as a finite additive measure over the existing exact cell algebra
when one complete calibrated basis, one derived band/time identity, one source
provenance/revision and one closed frontier are all identity-bound.

The result does not prove within-cell uniformity, physical radiance, transport
applicability, received energy, detector response or visibility. A coarser
frontier retains unresolved detail rather than inventing a distribution.

## Decision and next action

Advance one bounded step to a **code-facing calibrated source-energy
distribution ownership and implementation-readiness audit**. It must decide
the exact additive owner boundary and freeze candidate records, canonical
codecs, resource ceilings, dependency direction, hostile/platform gates and
deletion-only rollback. It must retain one first consumer and zero downstream
consumers.

Do not implement. Add no crate, contract schema, dependency, production test,
production source or consumer without a later exact owner decision. Transport
applicability, received-energy composition, detector response, visibility,
runtime, promotion and C3 closure remain blocked or later.

Nothing broader is locked in. One consumer first, reassess before expanding.
