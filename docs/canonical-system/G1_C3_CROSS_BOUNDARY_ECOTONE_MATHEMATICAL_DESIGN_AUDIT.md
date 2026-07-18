# G1 C3 cross-boundary ecotone mathematical-design audit

Date: 2026-07-18

Status: **evidence-preserving typed-boundary witness selected; independent
fixture oracle specified; no implementation authorized.**

## Decision

The smallest admissible C3 result is not a blending algorithm. It is an
evidence-preserving typed-boundary witness which proves that categorical
physical-region identity cannot become a causal palette operand, retains an
explicit sharp physical cause without averaging it, and returns a typed
unresolved result whenever the evidence is unavailable, contradictory or from
the wrong subject.

This is the bounded C3 half of the master cross-boundary rule. It proves causal
label independence in exact fixtures. It does not prove that rendered pixels
are seamless, select a biome transition, or define an ecotone width.

## Existing owner inventory

| Owner | Admissible evidence | Boundary retained here |
|---|---|---|
| `regional-environment-state` | exact Q32.32 sample coordinate, separate exposure and moisture permille, reconstruction and field provenance | a continuous-source causal sample; coordinates are not metres and discrete samples do not prove the field between them |
| `physical-region-partition` | exact cell membership, lower-cut or exact signatures, connected components and shared-edge adjacency over the same spatial domain | categorical annotation and provenance only; region, component, signature and cut are not palette weights |
| `surface-material-state` | exact three-band reflectance permille and material provenance | a causal palette operand, but V1 provides no canonical 2D cell-to-material-interface binding |
| `derived-world-rules` | validated star, atmosphere, material and regional inputs and a deterministic three-band physical palette bound | a pointwise causal consumer; it has no physical-region/component input and makes no rendered-visibility claim |

Unavailable regional evidence is distinct from numeric zero. Equal numeric
values do not repair a reconstruction, domain, recipe, material revision or
cell-identity mismatch. Partition adjacency is not a material interface.

## Candidate comparison

| Candidate | Determinism and reversal | Sharp-cause retention | Disposition |
|---|---|---|---|
| direct categorical painting | deterministic but category-dependent | retains an arbitrary categorical step | **reject**: region identity itself creates the false seam |
| fixed-width blending | only after freezing an unowned kernel and width; recursive variants are order-dependent | averages through a genuine interface | **reject**: invents a scale and can blur a real cause |
| cause-scaled mixing | possible only after every cause owns an exact scale, kernel and join | conditional | **defer**: Forge has no canonical cross-cause strength, distance or 2D material-interface owner |
| evidence-preserving typed boundary | pointwise, deterministic and reversal-safe by construction | preserves two exact sides and cause identity | **select** |

There is no universal similarity score, cause-strength score, biome weight,
beauty score, norm over heterogeneous evidence, fade width or tolerance in the
selected model.

## Exact pointwise causal model

For canonical cell centre `x` and band `b`, let:

- `I_b(x)` be validated stellar irradiance permille;
- `A_b(x)` be validated atmospheric transmission permille;
- `M_b(x)` be validated surface reflectance permille; and
- `E(x)` be validated regional exposure permille.

Every factor is an integer in `[0, 1000]`. The independent reference palette
is

```text
P_b(x) = floor((I_b(x) * A_b(x) * M_b(x) * E(x) + 500,000,000)
               / 1,000,000,000)
```

which is round-half-up for the current denominator and matches the causal
relation in `derived-world-rules` without calling its production helper. The
oracle uses unbounded integer or exact rational arithmetic so a narrow
intermediate cannot wrap.

Let `R(x)` be the physical-region/component annotation. `R` is deliberately
absent from `P`. For any relabelling or repartitioning `R'` that preserves the
same causal evidence and subjects:

```text
P[E, M, R](x) = P[E, M, R'](x)
```

This categorical-independence law is the objective no-false-seam rule. It does
not impose a subjective maximum colour difference: a palette difference caused
by a real input difference remains admissible.

## Typed shared-edge witness

For a canonical ordered shared edge `(left, right)`, emit exactly one
evidence-only disposition:

- `continuous_cause_exact(left, right)` when both sides contain complete,
  compatible continuous-source evidence and no sharp-cause witness;
- `sharp_cause_exact(cause, left, right)` when an explicit two-sided physical
  interface witness exists; both exact values and the cause identity survive;
- `unavailable_evidence(reason)` when any required causal dimension or the
  missing canonical 2D material-interface join prevents the claim;
- `contradictory_evidence(reason)` when continuous and sharp-cause evidence
  conflict;
- `provenance_mismatch(reason)` for subject, reconstruction, field recipe,
  spatial domain, material revision or cell mismatch;
- `noncanonical_input(reason)`, `arithmetic_out_of_range(reason)` or
  `unsupported_join(reason)` for the corresponding fail-closed condition.

These names specify oracle outcomes, not a production schema. In particular,
`continuous_cause_exact` means that the two samples are owned by
continuous-source causal evidence. It does not claim a continuous interpolant
between discrete or quantized samples.

A path reversal swaps the ordered sides of `sharp_cause_exact`, preserves its
cause identity, and maps each pointwise continuous-source sample to the same
coordinate. It cannot change any unordered edge disposition. Evaluation order
cannot change the result because no running mean, recursive blend, neighbour
accumulation or previous sample enters the pointwise calculation. Receipts are
canonically sorted by domain, cell and edge identity before encoding.

## Independent disposable oracle

The next proof, if admitted by implementation readiness, must be a disposable
Python oracle which imports no Forge production crate and calls no production
helper. It must use arbitrary integers and `Fraction`, independently encode its
receipt, and compare the current causal formula only as a frozen subject under
test. GPU acceleration is inappropriate: the proof is exact, small-integer,
branch-heavy and dominated by provenance and permutation checks.

The bounded grid portfolio is:

- `1 x 1`, `1 x 9`, `9 x 1`, `2 x 2` and `3 x 3` edge/corner cases;
- nested `5 x 5` and `9 x 9` grids compared only at exactly coincident
  coordinates with matching provenance;
- `17 x 17` rational ramps, plateaus and reversals;
- the exact `256 x 256` 65,536-cell ceiling; and
- `257 x 256`, which must fail before evaluation.

Positive steps, bounded-absent outer edges, checked coordinate arithmetic and
no wrapping remain mandatory. Resolution changes that move coordinates do not
qualify for invariance and cannot be repaired by invented interpolation.

## Hostile falsifier portfolio

1. A label-only split changes region, component and display labels while all
   causal inputs remain identical. Causal output and edge dispositions must be
   byte-identical to the unsplit control.
2. Relabelling and moving a categorical boundary while retaining the exact
   causal evidence cannot alter causal bytes.
3. Row-major, column-major, reverse, component-major, fixed-seed shuffled,
   chunked and parallel enumeration must produce one canonical receipt.
4. Two equal-signature disconnected islands separated by another signature
   remain distinct. No label may join them or propagate a fade through the
   separator.
5. Unavailable exposure, unavailable moisture, numeric zero exposure and
   numeric zero moisture remain four distinct situations. Unavailable never
   becomes zero, a sentinel band, an inferred gradient or a joinable region.
6. An explicit material-interface fixture with equal exposure and distinct
   reflectance preserves both exact palettes and forbids cross-edge averaging.
   Removing the explicit disposable interface witness yields
   `unavailable_evidence`, because no canonical spatial material join exists.
7. Equal values on both sides do not erase an explicit barrier or interface.
8. A steep continuous-source gradient without a typed sharp cause remains
   continuous-source evidence; magnitude thresholds cannot invent sharpness.
9. Exact horizontal, vertical and diagonal rational ramps may acquire rounding
   plateaus, but no reversal or label-correlated jump absent from the exact
   oracle.
10. Below-half, exact-half and above-half products, including
    `999 * 1000 * 1000 * 500`, must match arbitrary-precision round-half-up.
11. Negative or above-1000 factors, forged outputs, zero steps, oversized
    coordinates, domain-size overflow and narrow-intermediate wrap attempts
    fail closed; neither saturation nor wrapping is permitted.
12. Swapping reconstruction, field recipe, spatial domain, climate revision,
    material revision or cell identity while holding numbers equal returns
    `provenance_mismatch`.
13. Stale partition receipts, missing or duplicate cells, forged membership,
    reused component IDs and noncanonical boundary ordering are rejected before
    an ecotone claim.
14. Simultaneous incompatible continuous-source and sharp-interface claims
    return `contradictory_evidence`; neither wins by priority convention.
15. Equal labels on opposite bounded domain edges remain non-neighbours. No
    component join, fade or interface claim may wrap across the domain.
16. Coarse/refined grids must agree at shared exact coordinates only; changing
    resolution while moving coordinates cannot claim invariance.
17. A recursive-blend negative control must demonstrate traversal-order drift
    and be rejected.
18. A fixed-width negative control must demonstrate both a category-correlated
    halo and averaging through a true interface and be rejected.
19. Heterogeneous exposure, moisture and material evidence must remain
    separately typed; collapsing them into one distance or weight fails.

## Pass rule and claim ceiling

Advance to a later implementation-readiness decision only if every valid
fixture reproduces the independent exact result, every nuisance-label and
enumeration transformation is invariant, every explicit sharp interface
retains its two sides and cause, and every unavailable, contradictory,
noncanonical or provenance-invalid case remains typed and fails closed.

Passing would establish exact causal replay, label independence, bounded
grid/order/reversal invariance and synthetic sharp-cause retention. It cannot
manufacture metres-per-coordinate-unit evidence or a canonical 2D
material-interface owner. It cannot establish a physically calibrated ecotone
width, scientific validation, production performance, rendering quality,
perceptual visibility, biome meaning, ecology, organism behavior, runtime
behavior, promotion or C3 closure.

## Stop condition

This stage stops at the selected mathematical witness and adversarial oracle
specification. It adds no contract schema, crate, dependency, production test,
production source or downstream consumer. Physical applicability remains
explicitly evidence-blocked.

Nothing broader is locked in. One consumer first, reassess before expanding.

## Verification receipt

- `tools/verify.ps1`: exit `0` on 2026-07-18.
- Measured wall time: `324.2` seconds.
- Captured verification output: `2,434` lines.
- Record-role verification: `846` durable files classified by `32` ordered
  rules.
- Modularity verification: `52` modules with no forbidden imports or
  dependency cycles.
- The complete PowerShell parser sweep, 19 ecotone-compatible historical C3
  route shields, checkpoint integrity, bootstrap, generated context, UI build,
  Rust workspace and isolated desktop build all passed.
