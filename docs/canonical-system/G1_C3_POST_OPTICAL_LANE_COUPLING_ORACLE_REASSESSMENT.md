# G1 / C3 post optical-lane coupling oracle reassessment

Date: 2026-07-16

Status: **finite boundary/corner lineages are insufficient. The only surviving
bounded research candidate is whole-phase-space-cell interval certification
with all-or-nothing receiver coverage and typed unresolved boundaries; design
and oracle only.**

## Rejected routes

- A central ray plus scalar weight has no discretization-invariant measure.
- Boundary or corner lineages cannot certify interior topology.
- First-order differentials do not prove behavior across folds, caustics or
  branch changes.
- Adaptive point sampling can find counterexamples but never proves an
  unsampled interior under a finite cap.
- A favourable partial-coverage fraction cannot be inferred from an AABB box
  overlap or from the fraction of samples that arrive.

## Surviving candidate

Evaluate a declared source projected-area/direction phase-space cell only when
owner-produced interval evidence certifies the complete cell through one
ordered topology and into the strict receiver interior. The v1 research
candidate would be deliberately all-or-nothing:

- `certified_full_cell_arrival` may carry the unchanged source measure and
  separately composed dimensionless transfer enclosure;
- `certified_zero_cell_arrival` may carry zero accepted measure only when whole
  exclusion is proved; and
- topology ambiguity, branch ambiguity, caustic/fold possibility, conditional
  widening or partial receiver overlap remains typed unresolved.

This does not estimate a partial coverage fraction. Adaptive subdivision may
partition the source measure exactly and classify children independently, but
unresolved children retain their measure as unknown rather than being dropped,
averaged or promoted from samples.

## Why this is only a design question

The existing physical interval owner can certify a common next face for a
point/direction box, and the interface owner can retain uniform versus
ambiguous branches. But the current lineage and receiver owners intentionally
do not claim whole phase-space-cell measure, preserve all source
area/direction correlations or accept conditional receiver arrival. Reusing
their evidence is a possible composition route, not proof that a schema is
sound.

## Selected next bounded action

Run only a mathematical design and exact-rational counterexample/oracle audit
for whole-cell all-or-nothing coupling. It must test correlation loss,
refinement conservation, strict full receiver inclusion, strict whole
exclusion, partial overlap, branch/topology changes, fold detection, operation
caps and unresolved-measure accounting.

Do not add a crate, dependency, schema, test or production source. Do not
modify conditional receiver-arrival policy. Do not claim source emission,
received power, detector response, visibility, perception, runtime, promotion
or C3 closure.

