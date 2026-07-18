# G1 C3 Visible-Radiance Interface Post-Oracle Strategy Reassessment

Date: 2026-07-16

Status: **bounded adaptive staged refinement with exact fast paths selected for
disposable falsification; implementation readiness remains blocked.**

## Decision

The next candidate combines four layers in this order:

1. exact validation, geometry and gcd-reduced TIR classification;
2. proven algebraic fast paths for exact identities;
3. a finite staged precision ladder whose independently outward-rounded
   enclosures are intersected across levels; and
4. a typed `nonconvergent_enclosure` result when the declared experimental
   ceiling does not certify both one-unit output targets.

The candidate does not select fixed 160-bit arithmetic. The staged oracle
shows only that 160 fractional bits is the first tested portfolio point meeting
both targets, with 448-bit live intermediates. That is useful evidence for an
adaptive ladder, not a universal separation theorem.

The candidate also does not select an arbitrary-precision dependency. Python
arbitrary precision remains the disposable proof mechanism and independent
reference. A later readiness audit must compare a governed dependency with the
cost and risk of any bounded wide-integer implementation; this package may not
silently create the latter.

## Why fixed 160-or-higher staging is not selected

A fixed 160-bit schedule has simple constant control flow and passed the
retained 1,045-case portfolio. It still lacks proof that every admitted exact
input lies far enough from every Q0.48/Q1.62 projection boundary to be
certified at that precision. Inputs are finite under the declared ceilings,
but finiteness alone does not provide a usable minimum-separation bound.

Selecting 160 now would therefore convert one observed portfolio threshold
into a universal numerical policy. Selecting 192, 256 or 384 has the same
logical defect at greater cost. Fixed precisions remain mandatory baselines in
the next oracle, not the selected general strategy.

## Bounded adaptive candidate

The experimental ladder is:

`96 -> 128 -> 160 -> 192 -> 256 -> 384 fractional bits`.

This is a proof-ladder candidate, not a public format. The 72- and 80-bit
levels are excluded from the lead ladder because the retained hostile case
missed the power target by 67,110,913 and 264,192 units respectively and the
direction target by 16,385 and 65 units. The next oracle must still retain
those rejected baselines as regression evidence.

At each level the evaluator recomputes a fresh outward enclosure from the same
exact input and the same operation schedule. For every output component, let
`F_k` be that fresh enclosure and `I_(k-1)` the retained enclosure from the
previous level. The new retained enclosure is:

`I_k = intersection(I_(k-1), F_k)`.

Both operands independently contain the exact value, so their intersection
must also contain it. An empty intersection is an arithmetic defect and rejects
the candidate. The retained sequence is monotone by construction; a later
level can never widen or contradict an earlier certified result.

After each level, project the retained power intervals to Q0.48 and direction
intervals to Q1.62. The event is `known` only when every required interval is
ordered, contains the independent reference and spans at most one target unit.
Otherwise the evaluator advances to the next declared level.

At the 384-bit experimental ceiling, failure to meet all targets returns
`nonconvergent_enclosure` with the last certified intervals and a precision/
width receipt. It never emits a best-effort physical result, silently invokes
another arithmetic engine or continues without a resource bound.

This rule guarantees termination of the proof candidate. It does not guarantee
that every admitted event returns `known`; coverage is a result the oracle must
measure rather than assume.

## Exact fast paths

Exact paths precede the adaptive ladder and must be individually proved:

- exact critical equality and above-critical evidence return TIR;
- normal incidence gives exact axis-aligned outgoing directions and rational
  reflectance `((eta_t-eta_i)/(eta_t+eta_i))^2`;
- equal refractive indices give exact zero reflection and preserve the incident
  direction relation without independently re-deriving transmitted angle;
- an exact perfect-square `S` permits rational incident/reflected direction
  components; and
- structurally zero components remain exact zero rather than accumulating a
  symmetric interval around zero.

Fast paths may reduce cost and prevent artificial two-unit widths at exact
target boundaries. They are not a complete decision procedure for whether a
general algebraic Fresnel or direction value equals a projection boundary.
Any unrecognized boundary case proceeds through the bounded ladder and may
return `nonconvergent_enclosure`.

Each fast path requires equivalence fixtures against the independent general
reference, plus negative neighbors proving that the predicate does not admit a
nearby non-identity case.

## Strategy comparison

| Strategy | Correctness | Termination and width | Cost | Decision |
|---|---|---|---|---|
| Fixed 160+ staging | Directed arithmetic remains sound | Terminates, but no universal one-unit separation proof; 160 needs 448 live bits in the portfolio | Constant high cost for every event | Retain as baseline, reject as universal choice now |
| Bounded adaptive staging | Exact branch plus outward intervals remain sound; cross-level intersection is monotone | Hard ladder ceiling guarantees termination; unresolved cases are typed rather than fabricated | Pays high precision only when lower levels cannot certify | **Selected for disposable falsification** |
| Algebraic exactness alone | Strong for named identities | Does not cover general irrational outputs or arbitrary projection-boundary equality | Very cheap where applicable | Mandatory fast-path layer, insufficient alone |
| Governed arbitrary precision | Can supply the integer/rational/root machinery | Still requires a precision policy, termination ceiling and boundary semantics | Data-dependent allocation and dependency governance | Retain as reference and implementation option; do not select yet |

## Why adaptive refinement does not remove the dependency question

The measured staged live ceiling is `max(F + 232, 2F + 132)`. The retained
oracle observed 448 live bits at 160 fractional bits and 896 at 384. Adaptive
control flow reduces how often the wider levels run; it does not make those
levels fit primitive integers.

A later implementation-readiness package must therefore compare at least:

- a reviewed wide-integer dependency;
- a reviewed arbitrary-precision dependency;
- an existing already-governed Forge arithmetic facility, if one actually
  meets the operations and ceilings; and
- the maintenance and verification cost of any local bounded representation.

This audit rejects a bespoke multi-limb subsystem as an implicit default. It
does not reject one forever; it requires the same explicit evidence and cost
comparison as an external dependency.

## Mandatory adaptive counterexample oracle

The next package is capability-free Python proof code. It must reuse the exact
reference and staged operation schedule while adding the adaptive controller.
It must:

1. retain exact gcd-reduced TIR and the 232-bit hostile case;
2. implement the 96/128/160/192/256/384 experimental ladder and preserve the
   rejected 72/80 baselines;
3. intersect independently computed enclosures across levels and fail on an
   empty or reference-excluding intersection;
4. stop at the first level where every Q0.48/Q1.62 interval spans at most one
   unit;
5. return typed `nonconvergent_enclosure` at the hard ceiling;
6. implement each exact fast path separately with positive, negative-neighbor
   and general-reference equivalence fixtures;
7. add deliberately target-aligned exact outputs, near-aligned neighbors,
   critical neighborhoods and hostile coprime grazing cases;
8. record stop-level distribution, maximum live/stored bits, recomputation
   count, exact-fast-path count and nonconvergent count;
9. compare adaptive work against fixed 160 and fixed 384 baselines on the same
   deterministic portfolio; and
10. retain energy, unit-vector, Snell, authority-negative and no-clamp shields.

The adaptive candidate is rejected by any branch mismatch, lost reference
containment, empty intersection, wider retained level, false exact-fast-path
match, result wider than one target unit, execution beyond the declared cap,
silent fallback, or fabricated known result at nonconvergence.

## Readiness and failure policy

A passing adaptive oracle would establish only that the strategy deserves an
implementation-readiness audit. That later audit must freeze:

- the production ladder and hard ceiling;
- whether `nonconvergent_enclosure` is an acceptable public semantic outcome;
- a checked wide-number representation and dependency decision;
- deterministic cost/resource ceilings;
- exact codec and validation rules; and
- rollback, second-platform and integration evidence.

If the adaptive oracle produces nonconvergent admitted cases at 384 bits, the
next route is not automatically “increase the cap.” The result must first
classify whether the cause is an exact output boundary, avoidable dependency
inflation, or genuine separation cost. Exactness rules, output contract,
arithmetic strategy and admitted domain must then be reconsidered explicitly.

## Authority and continuity limits

This reassessment grants no public schema, Rust module, dependency selection or
installation, coefficient catalogue, downstream refractive path, received
radiance, perception, rendering, passage, navigation, persistence, runtime,
approval, promotion or C3 closure.

It grants no sphere, planet, terrain or biome-presentation semantics. Biome
continuity remains unchanged: continuous physical causes require deterministic
ecotones, while sharp visible transitions require sharp physical evidence.

## Exact next action and stop condition

Implement only the disposable adaptive-refinement counterexample oracle above.
Measure certification distribution, nonconvergence, exact-fast-path validity,
live widths and cost relative to fixed 160/384 baselines. Stop before
implementation readiness, dependency selection or installation, Rust code,
domain narrowing, downstream path composition, generic passage or C3 closure.
