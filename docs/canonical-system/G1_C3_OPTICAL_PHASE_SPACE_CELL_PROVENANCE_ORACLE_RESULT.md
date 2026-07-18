# G1 / C3 optical phase-space cell provenance oracle result

Date: 2026-07-16

Status: **the smallest independent parent/partition/correlation model survives
the disposable exact-rational oracle; implementation remains blocked behind a
separate code-facing readiness audit and explicit owner approval.**

## Reproducible receipt

- Oracle source:
  `tools/prove-g1-c3-optical-phase-space-cell-provenance.py`
- Source SHA-256:
  `7740595a08656d616f714bc3e1f249acd0a9b0fe95b486736f0227158981d5f6`
- Receipt SHA-256:
  `f9b354164a13bdaa312af6c8711915f661fea4a9abd7fe5ba097f872afb297e6`
- Positive portfolios: **20**
- Hostile rejections: **33**
- Authority effect: `none_evidence_only`
- Schema authorized: `false`

The repository-bundled Python runtime reproduced the receipt from the pinned
source.

## Surviving evidence

### Identity is deterministic and consumer-independent

The root identity replays exactly and changes when source, scope,
reconstruction, revision, measure or form changes. Receiver identity is absent
from canonical root and child records. Extra receiver, topology/branch and
radiance fields fail the strict record surface rather than silently joining the
identity.

The retained root ID is:
`857685a9dfcd37439e7d487716032171595a8b53abe2a88916027b1e2af98f72`.

### Binary refinement conserves exact measure

The oracle constructed the complete depth-six binary tree:

- maximum depth: **6**;
- leaf cells: **64**;
- full-tree cells: **127**; and
- split receipts: **63**.

Exact root measure remained `1/1` at 4, 16 and 64 leaves. Every child carried
exactly half its parent measure, and each split receipt committed to the ordered
lower/upper child pair. Swapped, duplicate or missing children, forged measures
and modified receipts were rejected.

### Correlation survives refinement

The exact shared-symbol difference `u-u` projected to `[0/1, 0/1]`. Erasing
that relationship and subtracting the two independent boxes produced
`[-2/1, 2/1]`. The model therefore retains a proof unavailable from current
axis boxes while its conservative projection still widens safely.

The first lower and upper children reparameterized the selected coefficient
from `1` to `1/2` and shifted the centre to `-1/2` and `1/2`. Exact rational
replay retained this transformation through all six refinement levels.

### Canonical rational and structural attacks fail

The 33 hostile cases reject:

- zero or malformed provenance IDs and zero revision;
- dimensions zero and five;
- zero/negative measure;
- output-count, coefficient-arity and remainder-order drift;
- numerator or denominator beyond the provisional 256-bit oracle bound;
- invalid split axes and depth-seven work;
- forged root, parent, depth, path, measure, centre, coefficient and remainder;
- swapped, duplicated and missing children;
- altered split receipts;
- receiver, branch and radiance field injection; and
- unreduced, noncanonical-zero and negative-denominator rationals.

Invalid evidence is rejected; the oracle contains no repair-by-guessing or
drop-unresolved-measure path.

## Observed cost, not promoted caps

The retained portfolio observed:

- maximum canonical cell record: **1,145 bytes**;
- maximum numerator width: **4 bits**; and
- maximum denominator width: **7 bits**.

These small values arise from the selected exact fixtures. They do not justify
production caps, a fixed-width arithmetic choice or a general optical-source
dimension. A readiness audit must derive limits using hostile large-rational,
maximum-form, maximum-depth and codec portfolios.

## What the oracle did not prove

The root measure is declared provenance evidence. The oracle proves canonical
identity and exact conservation, not that a future source recipe chose the
physically correct projected-area/angular measure.

The provisional dimension range `1..=4`, midpoint-only binary refinement,
six-output affine-plus-remainder form, 256-bit rational guard and depth six are
oracle choices. None is an implementation schema or promoted limit.

The proof does not establish source emission, radiance, power, inverse-square
spreading, topology/branch uniformity, interface/bulk composition, receiver
coverage, partial coupling fraction, detector response, visibility, runtime,
persistence, promotion or C3 closure.

## Readiness blockers

Before any source action, a separate code-facing readiness audit must decide
and verify:

1. exact public input/result/receipt types and strict codecs;
2. whether the implementation dimension is fixed at four or admits a bounded
   lower-dimensional degeneracy representation;
3. the source of root-measure authority and its explicit non-radiance meaning;
4. domain separators and canonical identity fields;
5. rational representation and derived bit ceilings;
6. maximum form terms, depth, cells, bytes, allocations and operations;
7. typed work exhaustion that retains all unprocessed measure as unresolved;
8. read-only projection into current interval inputs without modifying their V1
   identities or semantics;
9. exact fixture and platform matrix; and
10. deletion-only rollback with unchanged current owners.

That package would be a serious schema decision. It must present an exact owner
action and pause before a crate, contract, production test or source is added.

## Decision

The abstract prerequisite survives. It is sufficient to justify a
**code-facing implementation-readiness audit only**. It does not authorize
`optical-phase-space-cell-binding`, any dependency, schema, production test or
production source. The coupling consumer remains separate and later; this
result supplies only a potential provenance subject.

