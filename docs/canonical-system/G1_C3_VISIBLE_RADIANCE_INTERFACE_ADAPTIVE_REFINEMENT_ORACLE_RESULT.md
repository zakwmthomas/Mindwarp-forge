# G1 C3 Visible-Radiance Interface Adaptive-Refinement Oracle Result

Date: 2026-07-16

Status: **bounded adaptive strategy supported in the retained portfolio;
implementation readiness and wide-number representation remain blocked.**

## Result

The capability-free Python oracle reuses the independent 384-root-bit
arbitrary-precision reference and the retained integer-only staged evaluator.
It adds:

- full-event exact fast paths with negative-neighbor predicates;
- independent 96/128/160/192/256/384-bit evaluations;
- monotone intersection of certified enclosures across levels;
- first-certifying stop behavior;
- typed hard-cap `nonconvergent_enclosure`; and
- work receipts against fixed 160- and 384-bit baselines.

The run covered **1,049 cases**, including 1,024 deterministic generated cases,
the retained fixed/critical/coprime hostile portfolio and four new fast-path
negative neighbors. The adaptive controller completed 7,536 direct checks and
the reused staged layer completed 57,298 checks. Exact/reference branch,
containment, ordered interval, energy, unit-vector, Snell, structural-zero,
fast-path equivalence, cap and authority shields passed.

## Stop distribution

| Outcome | Cases |
|---|---:|
| Exact full-event fast path | 11 |
| Certified at 96 bits | 1,031 |
| Certified at 128 bits | 6 |
| Certified at 160 bits | 1 |
| Certified at 192/256/384 bits | 0 |
| Main-portfolio `nonconvergent_enclosure` | 0 |

The exact fast paths were:

- 2 normal-incidence events;
- 1 index-matched perfect-square event; and
- 8 TIR perfect-square events.

Each predicate has a retained negative neighbor. The fast-path result and the
general reference independently contain the same exact analytic power and
direction values; the fast path is not required to contain the reference's
wider proof interval.

## Monotonicity and cap receipt

The 1,038 general cases required 1,046 staged evaluations, only eight more than
one evaluation per case. Cross-level refinement performed 64 component
intersections. Every intersection was nonempty, contained the independent
reference and was a subset of the prior retained interval.

The retained hostile `coprime-wide-transmit` case was also run through a forced
`96 -> 128` ladder. It returned `nonconvergent_enclosure` after exactly two
evaluations. This proves the cap behavior is a typed terminating outcome rather
than a fabricated known result, silent fallback or unbounded precision loop.

No main-portfolio event reached the experimental 384-bit ceiling. That is a
portfolio result, not a proof that all admitted inputs certify by 160 bits or
that `nonconvergent_enclosure` can be removed from the future contract.

## Cost receipt

Precision work is recorded as the sum of fractional-bit levels evaluated. It
is an engine-neutral deterministic comparison unit, not elapsed-time or
production performance.

| Strategy | General cases supported | Staged evaluations | Fractional-bit work units |
|---|---:|---:|---:|
| Adaptive plus exact fast paths | 1,038 | 1,046 | 100,704 |
| Exact fast paths plus fixed 160 | 1,038 | 1,038 | 166,080 |
| Exact fast paths plus fixed 384 | 1,038 | 1,038 | 398,592 |

Adaptive work is about 60.6% of the fixed-160 baseline and 25.3% of the
fixed-384 baseline on this portfolio. This comparison does not model allocation,
limb multiplication, cache, branch or platform cost; those remain readiness
measurements.

The maximum adaptive live integer was **448 bits**, and the maximum stored
endpoint was **288 bits**. The one hostile case reaching 160 bits therefore
preserves the earlier wide-number blocker even though adaptive control avoids
paying that width for most events.

## Failure points engineered out

- Fast paths require exact predicates and explicit negative neighbors.
- A fast path is checked against analytic truth and the independent reference;
  it cannot inherit the general evaluator's widening as fake exactness.
- Structural zero components remain exact zero at every evaluated level.
- Each level is recomputed independently before intersection, preventing a
  previous rounding error from being reused as the next level's premise.
- Empty intersection, lost containment or retained widening is a test failure.
- Certification examines every power and direction component; one narrow value
  cannot hide a wider sibling.
- The hard cap is exercised directly and returns a typed failure result.
- Cost is recorded separately from correctness, so a passing numerical result
  cannot hide pathological recomputation.

## Readiness consequence

The adaptive strategy is supported strongly enough for an implementation-
readiness audit. That audit must still resolve:

1. the public status and payload of `nonconvergent_enclosure`;
2. the production precision ladder and hard resource ceiling;
3. the checked wide-integer/arbitrary-precision representation and dependency
   decision for up to the admitted live ceiling;
4. whether exact fast paths are encoded as validated optimizations or semantic
   result classes;
5. deterministic production cost, second-platform behavior and rollback; and
6. schema, codec, provenance and hostile-input rules.

This oracle does not authorize implementation. In particular, the measured
448-bit live maximum rejects any assumption that adaptive control makes a
primitive-only implementation sufficient.

## Nonclaims

This result defines no public schema, Rust module, dependency, coefficient
catalogue, scientific RGB fidelity, downstream refractive path, received
radiance, perception, rendering, passage, navigation, biome presentation,
sphere, planet, terrain, persistence, runtime, approval, promotion or C3
closure.

Biome continuity remains unchanged: continuous physical causes require
deterministic ecotones, and categorical interface evidence cannot paint a
visible seam.

## Exact next action

Run a visible-radiance interface adaptive-kernel implementation-readiness
audit. Freeze no dependency or code. Compare representation candidates,
production ladder/cap semantics, typed nonconvergence, exact-fast-path status,
cost ceilings, codecs, hostile fixtures, rollback and integration requirements.
Stop at an explicit owner implementation gate.
