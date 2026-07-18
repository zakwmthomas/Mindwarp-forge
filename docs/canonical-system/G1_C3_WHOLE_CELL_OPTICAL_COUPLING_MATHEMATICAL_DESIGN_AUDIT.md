# G1 / C3 whole-cell optical coupling mathematical design audit

Date: 2026-07-16

Status: **oracle-ready as an abstract conservative classifier; not
implementation-ready because current owners do not bind a source phase-space
cell or preserve its correlations.**

## Subject and invariants

The candidate subject is one bounded same-band, same-time-basis source
phase-space cell with exact projected-area/angular parameter bounds and one
exact nonnegative parent measure. Children must form a nonoverlapping exact
partition whose measures sum to the parent.

Transport evidence is a correlation-preserving affine enclosure plus an
outward remainder interval for each receiver-plane coordinate. Axis-aligned
boxes may be consumed as a conservative fallback, but correlation erasure may
only widen the result to unresolved; it must never manufacture full or zero
coverage.

Every admitted cell must retain one uniform ordered lineage topology and
interface branch. Any topology/branch ambiguity, conditional lineage,
derivative sign change, fold/caustic possibility or unbounded remainder is
typed unresolved before receiver classification.

## Frozen three-way classification

For an open receiver interior `(r_min, r_max)` and a conservative whole-cell
image enclosure `[x_min, x_max]` on every required axis:

- `certified_full_cell_arrival` requires strict `r_min < x_min` and
  `x_max < r_max` on every axis plus uniform topology/branch evidence;
- `certified_zero_cell_arrival` requires at least one axis with
  `x_max <= r_min` or `x_min >= r_max`; and
- every overlap, boundary equality, partial inclusion, correlation-only
  uncertainty or nonuniform topology is `unresolved_cell_coupling`.

Boundary contact is therefore never full arrival. Partial overlap never
produces an estimated accepted fraction.

## Measure accounting

The result carries three exact measures whose sum equals the source measure:

- fully accepted measure;
- certified-zero measure; and
- unresolved measure.

Subdivision moves measure between these categories only by proving a child's
classification. It cannot delete unresolved measure, copy the parent into
each child, average sample outcomes or promote a majority vote. A zero-measure
cell remains typed `zero_measure` and contributes to none of the three finite
categories.

Dimensionless cumulative lane transfer may later enclose loss on fully
accepted measure, but this design defines no radiance, source emission or
received power and does not multiply those quantities.

## Arithmetic and cost questions for the oracle

Use exact rational affine forms over a normalized parameter box. Compare a
correlation-preserving form against its axis-aligned interval fallback. The
oracle must retain:

- strict full, strict zero, boundary equality and partial overlap portfolios;
- exact 1-to-4, 1-to-16 and mixed-depth measure partitions;
- correlated cancellation that becomes unresolved after box erasure but never
  changes to a false full/zero result;
- monotone affine focusing with no derivative sign change;
- a quadratic fold whose derivative interval contains zero;
- uniform versus changed topology and interface branch evidence;
- unresolved-measure conservation under capped subdivision; and
- hostile mutations of measure, partition, topology, enclosure, strictness,
  classification and authority.

No production precision, object identity, byte ceiling or operation cap is
frozen by this abstract oracle. It should report observed rational numerator
and denominator widths and cell counts only to inform a later code-facing
audit.

## Implementation blockers

Current optical lineage binds one point/direction box per step, not a declared
source projected-area/angular cell. It does not preserve correlations back to
source parameters. Receiver arrival intentionally rejects nondegenerate boxes.
Consequently a surviving abstract classifier would still need a separate
code-facing provenance and correlation audit before readiness.

Do not add a crate, dependency, schema, test or production source. Do not
modify conditional receiver policy. Do not claim source emission, received
power, detector response, visibility, perception, runtime, promotion or C3
closure.

