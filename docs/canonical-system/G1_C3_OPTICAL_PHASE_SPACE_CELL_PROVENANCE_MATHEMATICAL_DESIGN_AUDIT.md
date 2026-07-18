# G1 / C3 optical phase-space cell provenance mathematical design audit

Date: 2026-07-16

Status: **oracle-ready as a capability-free provenance model; not schema-ready,
implementation-ready or a source/radiance model.**

## Inputs from the completed gap audit

The surviving whole-cell coupling classifier needs an independently replayable
subject with exact measure and retained correlation. Current physical and
interface inputs expose independent axis boxes, optical lineage begins from an
already-compiled box, receiver arrival is exact-ray only, and cumulative
transfer owns no source geometry. Those owners remain unchanged.

This design therefore tests a separate abstract record. The provisional name
`optical-phase-space-cell-binding` is only a discussion handle. It does not
authorize a schema, crate, dependency, production test or source file.

## Capability-free root subject

One root record contains:

- nonzero 256-bit source, scope and reconstruction identities;
- a positive source revision;
- a parameter dimension `d`, provisionally bounded to `1 <= d <= 4` for the
  disposable oracle only;
- one exact positive reduced rational parent measure; and
- six correlation-preserving output forms, corresponding only to a future
  conservative three-position / three-direction projection seam.

Each output form is

`y_j(u) = c_j + sum_i(a_j_i * u_i) + r_j`,

where every shared symbol has `u_i in [-1, 1]`, every coefficient and centre is
an exact reduced rational, and the outward remainder is an exact interval
`r_j in [r_lo_j, r_hi_j]`. Symbol order is canonical parameter-axis order. The
model carries no probability, density, radiance, source power or detector
meaning.

The root identity hashes every field above with a domain separator. It excludes
receiver identity, cumulative transfer, lineage transcript, classification and
runtime state so the same source cell may be replayed independently against
multiple consumers.

## Canonical binary refinement

Refinement selects one parameter axis and produces exactly two ordered children:
`lower` then `upper`. No arbitrary child geometry is caller supplied.

For the selected parent symbol `u` and a child-local `v in [-1, 1]`:

- lower child substitutes `u = (v - 1) / 2`;
- upper child substitutes `u = (v + 1) / 2`;
- the selected coefficient becomes `a / 2`;
- the centre becomes `c - a / 2` or `c + a / 2`;
- all other coefficients and the outward remainder are unchanged; and
- each child measure is exactly one half of the parent measure.

The split receipt commits to parent ID, selected axis, ordered child IDs and
exact conservation. The children commit to root ID, parent ID, depth, complete
ordered `(axis, side)` path, derived form and derived measure. Shared split
boundaries have measure zero; child interiors are disjoint and the pair covers
the complete parent parameter cell.

Arbitrary split pivots and caller-supplied child measures are rejected from this
smallest proof. They add codec, canonicalization and measure-authority choices
without increasing the information needed to falsify correlation preservation.

## Conservative projection seam

For each output form, the exact independent interval projection is:

- lower: `c - sum_i(abs(a_i)) + r_lo`; and
- upper: `c + sum_i(abs(a_i)) + r_hi`.

This projection may later populate existing physical/interface interval inputs,
but those boxes remain conservative projections rather than provenance
authority. The correlated form must remain replayable by the independent owner.

Correlation is observable before projection. If two forms share identical
coefficients and remainders, their exact difference can be zero even when the
difference of their independently projected boxes is wide. The oracle must
retain `u-u=0` while showing the box-erased difference `[-2,2]`; it must never
upgrade the latter to a correlated proof.

## Canonical rational rules

Every rational must have a positive denominator, greatest common divisor one,
and zero encoded only as `0/1`. The disposable oracle provisionally rejects a
numerator or denominator wider than 256 bits. These are oracle bounds, not
implementation caps; readiness must derive fixed-width or arbitrary-precision
needs from the retained portfolio.

Root measure is strictly positive. Binary refinement preserves positivity and
exactly halves measure. Zero or negative measure is invalid, not an empty or
dark cell.

## Typed rejection obligations

The independent oracle must reject:

- zero/malformed identities, zero revision and dimension outside `1..=4`;
- zero/negative/noncanonical or over-wide rationals;
- wrong output count, coefficient arity or reversed remainder;
- invalid split axis or depth beyond the provisional six-level portfolio;
- forged root/parent identity, depth, path, measure, centre, coefficient or
  remainder;
- swapped, duplicate or missing split children;
- extra receiver, topology, branch, radiance or authority fields;
- correlation-symbol permutation presented under an unchanged identity; and
- any 4, 16 or 64-way refinement whose child measures do not sum exactly to
  the root.

The model has no repair-by-guessing path. Invalid evidence fails replay. A
future bounded refinement consumer must type work exhaustion and retain the
entire unprocessed measure as unresolved; this oracle does not silently discard
cells.

## Provisional resource variables

The oracle records, but does not promote:

- maximum depth `6`;
- maximum leaf count `64` and full binary-tree cell count `127`;
- maximum observed numerator and denominator bit lengths;
- canonical byte lengths for the root, cells and split receipts; and
- counts of identities, splits, rational transforms and projections.

Any later readiness package must derive hard byte, allocation, arithmetic and
work ceilings, then prove hostile cap behavior. Copying these provisional
portfolio sizes into production would be unsupported.

## Authority and rollback

The candidate owns provenance evidence only. It does not own emission law,
radiance, power, inverse-square spreading, spectral catalogues, physical
traversal, interface arithmetic, lineage composition, cumulative transfer,
receiver acceptance, partial coverage, detector response, visibility,
perception, runtime, persistence, promotion or C3 closure.

No current crate imports this model. A future implementation must be additive
and deletion-only rollback; existing physical, interface, lineage, cumulative
and receiver V1 bytes, identities, fixtures and behavior remain unchanged.

## Oracle decision rule

The abstract model may advance only if exact-rational portfolios prove:

1. domain-separated deterministic identity and authority-negative fields;
2. exact 4, 16 and 64-way measure conservation;
3. algebraically exact child reparameterization;
4. retained shared-symbol cancellation and conservative independent projection;
5. strict rejection of every named forgery; and
6. deterministic receipt reproduction under a pinned oracle source hash.

Passing does not authorize schema or source. It permits only a later code-facing
implementation-readiness audit. **Do not add a crate, dependency, schema,
production test or production source.**

