# G1 / C3 whole-cell optical coupling oracle result

Date: 2026-07-16

Status: **the abstract full/zero/unresolved classifier survives exact-rational
counterexamples, but implementation remains blocked because no current owner
binds a source phase-space cell or preserves its correlations.**

## Deterministic receipt

Oracle source SHA-256:
`5a6502863850d68c42d8cd5719455a937cb85690db14330ecc5a9624c0bf7bd2`.

Canonical receipt SHA-256:
`0899edb94e23d4d4aff684bbe4b44a11432a1cd1e26c9e394b8e1c4fd6a913b1`.

Sixteen exact portfolios and twenty-four hostile rejection families pass.

## Surviving semantics

- Strict whole-cell inclusion is `certified_full_cell_arrival` only when every
  conservative image bound is strictly inside the open receiver.
- Whole exclusion is `certified_zero_cell_arrival` only when one conservative
  axis is closed-disjoint from the receiver.
- Boundary equality, partial overlap, changed topology/branch and a possible
  fold remain `unresolved_cell_coupling`.
- Exact 4-, 16- and 64-child partitions each sum to the unchanged parent
  measure.
- Accepted, zero and unresolved measures remain an exact partition; the
  retained mixed portfolio is `1/4 + 1/4 + 1/2 = 1`.
- Correlation erasure is conservative: true `u-u=0` is full in the correlated
  form, while its independent box `[-2,2]` becomes unresolved rather than a
  false full or false zero classification.
- A derivative-sign fold, topology change or branch change cannot be promoted.

## Hostile shields

The oracle rejects boundary promotion, estimated partial fractions, dropping
unresolved measure, copying a parent measure into every child, majority vote,
sample averaging, correlation-erasure false classifications, ignored topology
or folds, partition overlap/gaps, foreign parent identity, missing band/time
basis, contact promotion and all source/detector/visibility/authority claims.

## Disposition

This is a mathematical classifier result, not an implementation readiness
result. Current lineage evidence does not name a source phase-space parent
cell, its projected-area/angular basis, exact child partition identities or
correlation-preserving transport forms. Current receiver arrival also
intentionally rejects conditional boxes.

The next bounded action is a code-facing provenance/correlation gap audit only:
determine whether those subjects can be added independently without modifying
the physical, interface, lineage, cumulative or receiver owners. No crate,
dependency, schema, test or production source is authorized.

