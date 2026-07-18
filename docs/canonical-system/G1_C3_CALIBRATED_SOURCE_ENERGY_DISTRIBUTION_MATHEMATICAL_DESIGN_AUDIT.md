# G1 / C3 calibrated source-energy distribution mathematical design audit

Date: 2026-07-17

Status: **oracle-ready as a closed-frontier additive radiant-energy measure
bound to one verified calibration and one exact phase-space root. No schema,
consumer or production source is authorized.**

## Inherited owners and exact subject

The verified `calibrated-spectral-time-basis` owner supplies one complete
physical spectral/time basis identity and three derived legacy band/time
identities. The existing `optical-phase-space-cell-binding` owner supplies one
exact root, deterministic binary ancestry, cell identities and positive
abstract measure. Neither owner allocates source energy, and neither is
modified by this design.

The mathematical subject binds:

- one nonzero source identity, scope, source provenance and positive source
  revision;
- one complete calibrated-basis identity, selected RGB band and its exactly
  derived legacy band/time identity;
- calibration provenance distinct from source provenance;
- one phase-space root and reconstruction identity;
- exact cell paths, cell identities, ancestry and positive measures; and
- nonnegative reduced rational radiant energy in joules for that calibrated
  band and time cell.

Identity equality is correlation evidence only. The disposable fixture basis,
source and cell values do not select a canonical emission model or calibration.

## Representation comparison

### Independent leaf-energy records - rejected

A bag of leaf records can add numerically, but it cannot prove that the leaves
cover the root exactly once. Omitting a cell deletes energy; duplicating a cell
double-counts it; including both an ancestor and descendant overlaps the
subject. A root total added after the fact detects some arithmetic errors but
does not prove geometric coverage or ancestry.

### Root total without a distribution - retained only as unresolved

One exact root energy is a valid statement that the calibrated source emitted
that quantity into the root. It says nothing about the distribution within the
root. It is therefore the coarsest valid unresolved frontier, not a complete
fine-grained allocation and not a uniform-density assertion.

### Density with respect to abstract cell measure - derived view only

For a validated cell `C`, `rho(C) = Q(C) / mu(C)` is an exact average density
relative to that particular parameterization. It is useful for comparison but
is not the canonical quantity:

- a coordinate reparameterization changes `mu` and therefore `rho` unless a
  proven Jacobian transforms it;
- equal parent density can hide opposite child distributions; and
- the abstract measure is not projected SI area, solid angle or radiance.

Density must remain derived from the same exact cell and distribution version.
It cannot be transplanted, summed as energy or called radiance.

### Closed-frontier additive radiant-energy measure - selected

The survivor is a finite prefix-free frontier of exact phase-space cells. The
frontier must cover the root exactly once, its cell measures must sum to the
root measure, and its energies must sum to the separately bound root energy:

`Q(root) = sum Q(C) for C in frontier`.

Every refinement replaces one frontier parent atomically with both owned
children and proves:

`Q(parent) = Q(lower) + Q(upper)`.

No caller-authored unrelated leaf is admissible. A zero-energy child is an
explicit exact allocation. If finer allocation is unknown, the energy remains
on a coarser frontier cell with `unresolved_within_cell`; absence never means
zero. Mixed-depth frontiers are valid only when they remain prefix-free and
cover the root exactly once.

## Identity and provenance rule

The distribution identity must bind the complete subject, exact root energy,
the ordered frontier, every cell identity/path/measure, every exact energy and
each resolution state. Changing source revision, source provenance,
calibration, band/time, root, reconstruction, cell ancestry, energy or
resolution creates a different subject or fails validation.

Source provenance cannot alias calibration provenance. Calibration says what
the spectral/time coordinates mean; source provenance says who or what
asserted this emission allocation. Conflating them would let a calibration
revision silently rewrite source magnitude.

## Conservation and counterexamples

The disposable oracle must prove:

1. exact cell-measure and radiant-energy conservation at 4, 16 and 64 leaves,
   including zero-energy children;
2. root-only and mixed-depth unresolved frontiers retain the entire energy;
3. independent leaf omission, duplication, ancestor overlap and incomplete
   coverage fail closed;
4. equal calibrated bases can carry different root energies;
5. equal root energy can carry different phase-space distributions;
6. density changes under unproved measure reparameterization and equal parent
   density does not identify child allocation;
7. foreign basis, band/time, source, provenance, revision, root,
   reconstruction, cell identity, path, measure and resolution fail closed;
8. negative, noncanonical and non-reduced rational quantities fail closed;
9. atomic split duplication, deletion and parent-total mismatch fail closed;
   and
10. zero source with positive transfer remains typed differently from positive
    source with zero dimensionless coupling even though both algebraic products
    are zero.

The last check is a separation control only. It grants no transport
applicability or received-energy claim.

## Ownership and readiness consequence

The candidate cannot live inside the calibration owner because source
magnitude and revision are independent of calibration. It cannot live inside
the cell owner because that owner is channel-neutral and does not own energy.
It cannot live inside dimensionless transfer or bulk transport because those
owners expressly exclude source magnitude.

If the oracle survives, the next step is one code-facing ownership and
implementation-readiness audit for a separate calibrated source-energy
distribution owner. That audit must freeze exact records, codecs, identity
domains, resource ceilings, dependency direction, hostile fixtures, platform
gates and deletion-only rollback. It does not authorize implementation.

## Stop boundary

Run one deterministic exact-rational oracle only. Add no crate, contract
schema, dependency, production test, production source or consumer. Modify no
current owner. Select no emission spectrum, source model, normative energy,
spatial scale, coefficient catalogue, transport applicability, aperture,
detector response, visibility, runtime, promotion or C3 closure.

Nothing broader is locked in. One consumer first, reassess before expanding.
