# G1 / C3 optical lane coupling-measure oracle result

Date: 2026-07-16

Status: **finite correlated boundary lineages are rejected as sufficient
coupling evidence; corners/boundaries do not certify the phase-space-cell
interior. No schema or implementation candidate is authorized.**

## Deterministic receipt

The exact-rational oracle source is pinned at SHA-256
`368064c569fee8da613b0463c5322e93d3bd2870c4fc06aac68d5b720f8dab87`.

Its canonical receipt is pinned at SHA-256
`19e9b252a965e5a154d6864a4a426d47015b987a4932d3694ffc04a15d722d84`.

Twelve portfolios and twenty hostile rejection families pass.

## Surviving facts

- Exact rectangular phase-space measure is invariant when one parent is
  partitioned into 4, 16 or 64 nonoverlapping children.
- The NIST spherical-area free-space special case reduces exactly to
  `Omega = A / r^2`; the retained portfolio is `3/2 / 3^2 = 1/6 sr`.
- A passive dimensionless loss factor in `[0,1]` cannot increase an admitted
  radiance enclosure.
- Zero projected source area yields zero finite measure.
- Band, time, source-area and angular bases remain mandatory independent facts.

## Decisive counterexamples

Two affine neighboring-ray maps share the identical central ray but produce
different exact footprints, proving that central arrival plus cumulative
transfer is insufficient.

A central ray can be strictly inside the receiver while both admitted boundary
rays miss, so central arrival is not full receiver acceptance.

Most importantly, all four corners of a parameter cell can share one topology
while an interior point crosses another topology. The exact portfolio uses
`x^2 + y^2 < 1/4`: corners of `[-1,1]^2` are outside and the centre is inside.
Therefore even independently replayed corner/boundary lineages cannot certify
the whole phase-space cell without an interior enclosure theorem.

The fold portfolio `y = u^2` also proves that equal boundary images can hide a
caustic/fold. Such evidence must remain `unsupported_caustic_or_fold` rather
than receive a favourable Jacobian or footprint.

## Disposition

The finite boundary-lineage candidate is rejected, not implementation-ready.
The smallest honest next question is whether whole-cell interval topology
certification or adaptive subdivision with a proved interior certificate can
survive the same topology/fold counterexamples. Ray differentials remain a
comparator only; they are neither a source measure nor a whole-cell proof.

No crate, dependency, schema, test or production source is authorized. No
source emission, received power, receiver acceptance, detector response,
visibility, perception, runtime, promotion or C3 closure is claimed.

