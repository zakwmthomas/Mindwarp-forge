# G1 C3 Visible-Radiance Interface Staged-Kernel Oracle Result

Date: 2026-07-16

Status: **required fixed precisions through 128 bits rejected by a retained
counterexample; staged arithmetic first meets both one-unit targets at 160
bits in this portfolio, without implementation-readiness authority.**

## Result

The capability-free Python oracle compares one independent 384-root-bit
arbitrary-precision reference result per case with separate integer-only
staged evaluations. It uses exact gcd-reduced total-internal-reflection
classification, then applies outward rounding after every fixed-point
conversion, multiply, divide and square-root operation.

The run completed **155,987 checks** across **1,045 cases**:

- 21 fixed, critical-neighborhood and hostile-width cases;
- 1,024 deterministic generated cases using seed `252341783`;
- 12 named exact/nearest critical-neighborhood cases;
- 406 TIR and 639 transmitted cases at every precision; and
- required precisions 72, 80, 96 and 128, plus sensitivity precisions 160,
  192, 256 and 384.

Every precision preserved exact/reference TIR classification, reference
containment, ordered intervals, energy containment, reflected/transmitted
unit-vector containment and Snell tangential containment. No binary float,
epsilon, clamp, saturation, external dependency, production schema or Rust
code was used.

## Precision receipt

| Fractional bits | Q0.48 power width | Q1.62 direction width | Maximum live integer bits | Maximum stored endpoint bits | Result |
|---:|---:|---:|---:|---:|---|
| 72 | 67,110,914 | 16,386 | 301 | 200 | rejected |
| 80 | 264,193 | 66 | 309 | 208 | rejected |
| 96 | 10 | 2 | 325 | 224 | rejected |
| 128 | 1 | 2 | 384 | 256 | rejected |
| 160 | 1 | 1 | 448 | 288 | supported in portfolio |
| 192 | 1 | 1 | 512 | 320 | supported in portfolio |
| 256 | 1 | 1 | 640 | 384 | supported in portfolio |
| 384 | 1 | 1 | 896 | 512 | supported in portfolio |

The constructed `coprime-wide-transmit` case is the worst required-precision
direction case at 72, 80 and 128 bits and the worst power case through 96 bits.
It combines a nearly grazing admitted path delta with nearest high Q16.48
indices. The 96-bit direction maximum also occurs in a retained below-critical
neighbor. These are deterministic counterexamples, not statistical estimates.

The first passing sensitivity point, 160 bits, is an empirical portfolio
result only. It does not prove a universal 160-bit public format, a sufficient
production limb count or a cost-acceptable implementation.

## Exact critical-width receipt

The exact classifier compares

`(S-a^2) * eta_i_raw^2 >= S * eta_t_raw^2`

after independently reducing the geometry and refractive-index ratios by
greatest common divisors. Every reduced comparison agreed with its unreduced
form and with the independent reference branch.

The maximum product was **232 bits both before and after cancellation**. The
hostile coprime case therefore proves that cancellation cannot be treated as a
`u128` width guarantee. It remains a valid exact optimization, but a later
implementation must supply a checked wider operation or an explicitly governed
arbitrary-precision route.

## Live-width consequence

The staged schedule avoids the previous oracle's 8,330-bit compound-Fraction
expression, but it does not fit a narrow primitive implementation:

- converting the exact critical-angle ratio creates a live ceiling involving
  the 232-bit exact product plus the selected fractional scale;
- normalized multiply/divide/root stages create a separate ceiling near twice
  the fractional precision plus the path-squared allowance; and
- the measured maxima grow from 301 bits at 72 fractional bits to 448 bits at
  the first portfolio-supporting 160-bit precision.

Every measured maximum remained within the oracle's explicit derived ceiling
`max(F + 232, 2F + 132)`. This replaces accidental denominator explosion with
a bounded operation schedule, but it does not select how those wide integers
should be represented.

## Failure points engineered out

- Exact TIR is decided before any rounded value can affect the branch.
- Required and exploratory sensitivity precisions are separately named, so a
  160-bit observation cannot silently rewrite the original target.
- The hostile case remains in the retained script and exercises a real
  post-cancellation width above 128 bits.
- Reference results are computed once through the earlier arbitrary-precision
  oracle; staged results do not share its compound interval expressions.
- Every stage records live and stored widths, preventing a small final result
  from hiding a large temporary.
- A precision is labelled supported only when both public projection widths
  are at most one unit and every reference/invariant check passes.
- Negative proof is a successful oracle outcome, not converted into a passing
  128-bit claim.

## Readiness consequence

The required 72/80/96/128-bit candidate set is rejected. The broader hybrid
architecture is not rejected because 160 bits and above meet the portfolio
targets with bounded live widths. Immediate implementation readiness is still
blocked because the next decision must compare:

1. a fixed 160-or-higher fractional schedule with a checked wide-integer
   representation;
2. adaptive outward refinement that begins narrower and escalates only near
   hostile output boundaries;
3. additional algebraic exactness rules that avoid widening known identities;
   and
4. retained arbitrary precision as the governed production strategy.

That comparison must include deterministic cost, worst-case termination,
dependency and maintenance consequences. It may not select 160 bits merely
because this portfolio passed.

## Nonclaims

This result defines no public interface schema, Rust module, dependency,
coefficient catalogue, scientific RGB fidelity, downstream refractive path,
received radiance, perception, rendering, passage, navigation, biome
presentation, sphere, planet, terrain, persistence, runtime, approval,
promotion or C3 closure.

Biome continuity remains unchanged: continuous physical causes require
deterministic ecotones, and categorical interface evidence cannot paint a
visible seam.

## Exact next action

Run a post-oracle numerical-strategy reassessment comparing fixed 160-or-higher
staging, bounded adaptive refinement, algebraic exactness and governed
arbitrary precision against the measured 232-bit exact-classification and
448-bit first-supporting live-width receipts. Stop before implementation
readiness, dependency choice or installation, Rust code, domain narrowing or
downstream path composition.
