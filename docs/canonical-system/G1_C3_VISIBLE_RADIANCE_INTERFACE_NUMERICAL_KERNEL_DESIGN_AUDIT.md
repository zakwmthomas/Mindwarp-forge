# G1 C3 Visible-Radiance Interface Numerical-Kernel Design Audit

Date: 2026-07-16

Status: **hybrid exact-classification plus staged directed fixed-point candidate
selected for disposable falsification; implementation readiness remains
blocked.**

## Decision

The next candidate is not a direct fixed-width translation of the exact
`Fraction` oracle. It is a two-lane kernel:

1. retain exact integer geometry, validation and total-internal-reflection
   classification; then
2. for a band proved to transmit, evaluate roots, Fresnel power and local
   directions through a staged outward-rounded fixed-point interval schedule.

Algebraic cancellation is mandatory in both lanes. It reduces ordinary cost,
but it is not treated as proof that primitive-width arithmetic is always
sufficient. The implementation must fail closed at a declared arithmetic
ceiling; it may not change a physical branch because an intermediate was
rounded, clamped, saturated or truncated.

This audit selects only the candidate and its counterexample test. It does not
freeze an internal precision, integer representation, dependency, public
codec, coefficient range or Rust interface.

## What the 8,330-bit result means

The arbitrary-precision oracle deliberately keeps exact rational endpoints
through repeated interval operations. Each operation multiplies inherited
numerators and denominators, so its 8,330-bit maximum at 384 root bits measures
the size of that proof expression. It does not prove that the physical result
or a staged outward-rounded kernel intrinsically needs 8,330 bits.

The stable projected results are evidence for testing a staged kernel: over the
retained 10,608-check portfolio, Q0.48 power widths were one unit at every
tested root precision and Q1.62 direction widths reached one unit at 72 bits
and above. That is empirical sensitivity evidence, not a universal 72-bit
ceiling.

The genuine width problem is smaller but cannot be ignored: exact critical
classification can exceed `u128` under the retained exploratory stress
envelope.

## Exact critical-boundary lane

For one admitted path and band, retain the oracle notation:

- `S = dx^2 + dy^2 + dz^2`, with `0 < S <= 2^128 - 1`;
- `a > 0` is the exact path component along the oriented axis normal;
- `A = S - a^2`;
- `eta_i_raw` and `eta_t_raw` are positive exact Q16.48 integers; and
- the exploratory range `[1/4,16]` gives raw values from `2^46` through
  `2^52`.

Because

`sin_t^2 = A * eta_i_raw^2 / (S * eta_t_raw^2)`,

the exact TIR rule is the integer comparison

`A * eta_i_raw^2 >= S * eta_t_raw^2`.

Equality remains TIR. No square root, epsilon or approximate angle participates
in this branch decision.

Before multiplication, reduce `eta_i_raw/eta_t_raw` by their greatest common
divisor and reduce `A/S` by their greatest common divisor. Those reductions
are exact and can substantially reduce ordinary cases. They do not establish a
`u128` worst-case guarantee: coprime operands can remain, and each unreduced
product has a conservative width ceiling below `2^232`, hence can require up
to **232 bits**. The next oracle must measure post-cancellation maxima and
construct hostile coprime cases instead of assuming cancellation will occur.

This exact lane engineers away the most dangerous failure: a low-side rounded
`sin_t^2` cannot turn TIR into transmission, and a high-side rounded value
cannot erase a valid transmitted event.

## Transmitting numerical lane

After exact non-TIR classification, the candidate may compute only local event
enclosures. Its staged schedule must:

- normalize exact ratios before approximation rather than carry the oracle's
  compound rational denominators;
- represent every intermediate as ordered integer lower/upper endpoints at a
  declared scale;
- round outward after each multiply, divide and square-root stage;
- use sign-aware interval rules for Fresnel numerators and direction
  components;
- reject a denominator interval containing zero rather than choose a sign;
- keep raw arithmetic bounds distinct from theorem-backed intersections such
  as `0 <= R,T <= 1`;
- project only once into Q0.48 power and Q1.62 direction outputs; and
- return an explicit arithmetic-ceiling or nonconvergent-enclosure outcome
  when the one-unit target cannot be certified.

The staged precision is deliberately unresolved. The follow-on proof must test
at least 72, 80, 96 and 128 internal fractional bits against the exact oracle,
including adversarial critical and grazing neighborhoods. A precision may be
recommended only if a derived error budget and counterexample portfolio both
support it. Passing the previous random portfolio alone is insufficient.

## Four-strategy comparison

| Strategy | Correctness boundary | Width and cost | Dependency consequence | Decision |
|---|---|---|---|---|
| Staged directed fixed point | Can preserve monotone enclosures if every operation rounds outward; exact TIR must remain separate | Bounded per stage, but internal precision and wide multiply/divide/root ceilings still require proof | Could use a bounded wide-integer abstraction; representation is not selected here | **Lead candidate for falsification** |
| Retained arbitrary precision | Closest to the reference and naturally handles exact classification | Deterministic values but data-dependent allocation and multi-thousand-bit proof-expression growth | Requires accepting and governing an arbitrary-precision implementation/dependency or equivalent subsystem | Retain as reference and fallback, not default production choice yet |
| Narrower admitted domain | Can reduce worst-case products if the new ceiling is proved and validated | Cheapest arithmetic only by refusing previously admitted evidence | Would turn an arithmetic convenience into a public/scientific policy without coefficient evidence | Rejected for this package; reconsider only from real domain requirements |
| Algebraic reformulation | Exact cancellation and separated formulas remove avoidable denominator growth | Reduces typical width but coprime worst cases still defeat a primitive-width assumption | No dependency by itself, but cannot supply the entire kernel | Mandatory component of the lead candidate, insufficient alone |

## Why arbitrary precision is not selected now

The exact oracle remains the comparison authority, and arbitrary precision may
ultimately be the honest production choice. Selecting it now would skip the
cheapest unresolved question: whether a bounded staged schedule can certify the
same public outputs while keeping exact branch decisions. Conversely, this
audit does not authorize a bespoke multi-limb subsystem merely to avoid a
dependency. If the next proof shows that bounded-wide arithmetic is complex,
slow or still insufficient, the readiness audit must compare a governed
arbitrary-precision dependency with that maintenance burden explicitly.

## Why the domain is not narrowed now

The Q16.48 `[1/4,16]` envelope is already labelled computational and
non-scientific. There is no selected material catalogue from which a narrower
honest bound can be derived. Narrowing it simply to fit `u128` would hide the
failure rather than solve it and could later force silent incompatibility when
real coefficients arrive.

A future evidence-led domain revision remains possible, but it must originate
from coefficient provenance, scientific fidelity requirements and codec range
analysis—not from the convenience of one arithmetic primitive.

## Mandatory disposable counterexample proof

The next package is a second capability-free Python oracle that simulates the
candidate operation schedule while retaining the existing arbitrary-precision
oracle as ground truth. It must:

1. implement exact gcd-reduced TIR cross comparison and report pre/post-
   cancellation operand and product widths;
2. include constructed coprime cases near the 232-bit conservative ceiling;
3. generate exact vectors immediately below, at and above critical equality,
   including the nearest representable Q16.48 index neighborhoods;
4. test normal, grazing, index-matched, reciprocal and dispersive cases at 72,
   80, 96 and 128 internal fractional bits;
5. apply outward rounding after every scheduled operation and compare every
   final enclosure with the arbitrary-precision reference;
6. require no more than one Q0.48 power unit and one Q1.62 direction unit;
7. prove monotone interval order, energy containment, unit-vector containment
   and Snell tangential containment without clamp or epsilon;
8. record maximum live width per stage, not only the largest final rational;
9. distinguish arithmetic-ceiling, nonconvergent-enclosure and unsupported
   evidence outcomes; and
10. retain all authority-negative checks and deterministic seeds.

The falsifiers are explicit: any TIR mismatch, lost reference containment,
inverted interval, width over one output unit, denominator-sign ambiguity,
unbounded live stage or dependence on silent saturation rejects the candidate.

## Failure points and permanent shields

- Critical equality is decided by exact integer comparison before numerical
  evaluation.
- Cancellation is an optimization with measured receipts, never a width proof
  by assumption.
- Internal precision is selected from derived and adversarial evidence, not
  from the first passing random run.
- Arithmetic failure is a typed outcome; it cannot become a plausible-looking
  physical result.
- The exact oracle remains independent ground truth rather than sharing the
  staged implementation's arithmetic.
- No arithmetic choice can fabricate face evidence, a crossing point, a
  downstream refracted path or endpoint arrival.
- A later dependency choice must compare provenance, determinism, licensing,
  platform support, cost and maintenance against a bounded local abstraction.

## Authority and continuity limits

This design grants no schema, Rust module, dependency installation, real
coefficient data, scientific RGB fidelity, downstream refractive traversal,
received radiance, perception, rendering, passage, navigation, persistence,
runtime, promotion or C3 closure.

It also grants no sphere, planet, terrain or biome-presentation semantics.
Biome continuity remains unchanged: continuous physical causes must produce
deterministic ecotones, while a sharp boundary is valid only when a sharp
physical cause is evidenced. A grid interface record cannot paint a visible
categorical seam.

## Exact next action and stop condition

Implement only the disposable staged-kernel counterexample oracle described
above. Use it to determine whether exact classification plus a bounded
outward-rounded schedule can meet the one-unit targets and what live integer
widths it actually needs. Stop before an implementation-readiness claim,
dependency choice or installation, Rust code, public codec, coefficient
selection, downstream path composition, generic passage or C3 closure.
