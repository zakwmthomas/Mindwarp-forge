# G1 / C3 interval-incident interface implementation readiness

Date: 2026-07-16

Status: **not implementation-ready; no owner implementation action prepared.**

## Decision

The interval-incident oracle proves that conservative component boxes can
contain the local smooth-dielectric result, classify whole-box TIR versus
transmission exactly, survive the retained three-band 64-event portfolio and
fit the observed 512-bit arithmetic envelope. It does **not** yet define a
production-computable convergence certificate. Implementation would therefore
turn a hidden 384-bit oracle comparison into an undocumented production
assumption or would silently change the meaning of `known` and
`nonconvergent_enclosure`.

That is a blocking contract defect, not a request for more implementation
effort. No interval schema, Rust source, dependency, composer or verified v1
module is authorized by this audit.

## Blocking mismatch: the oracle knows something production would not

For each non-mixed case, the retained Python oracle evaluates a candidate at
96, 128 or 160 fractional bits and a separate reference at 384 bits. Its
`numerical_excess` function projects both results, proves that the candidate
contains the reference, and stops only when the candidate extends at most one
target unit beyond the reference endpoints.

The proposed production API has no 384-bit result. Consequently it cannot
reproduce the oracle's stop decision from its admitted input and the frozen
96/128/160 ladder. The forced 80-bit `nonconvergent_enclosure` is likewise
assigned by comparison with that unavailable reference, not by a certificate
the candidate can calculate itself.

The existing exact-path v1 rule cannot fill this gap. V1 starts from one exact
integer direction and can require its entire numerical output enclosure to be
at most one Q0.48 or Q1.62 unit wide. An interval input has legitimate physical
width: retained named boxes reach 98 Q1.62 units and the 64-event red lane
reaches 150,283,463 Q1.62 units. Applying the v1 total-width rule would
misclassify valid physical uncertainty as numerical failure.

## Ownership disposition

The one correct semantic owner remains
`visible-radiance-interface-event`. A separate crate implementing its own
Snell/Fresnel equations would create competing physical authorities. If a
later readiness audit succeeds, the narrow integration shape is an additive,
separately versioned interval API inside that crate, backed by its existing
private checked arithmetic and an internal interval evaluator. The exact-path
v1 public types, identity domains, bytes, fixtures and replay path must remain
unchanged.

This ownership answer does not itself authorize the necessary module change.
It also does not require a point box to equal a v1 point event. “V1
byte-identical” means every existing v1 input and event continues to encode,
hash, compile and replay byte-for-byte; claiming equivalence between broader
interval evidence and an exact ray would be false narrowing.

## Candidate schema questions that remain open

A later audit may only freeze a schema after resolving all of these together:

- whether incident component boxes are caller-declared evidence with explicit
  source/scope/reconstruction/revision provenance, or replayable output from a
  prior compiler;
- how a replayable chain avoids recursively nesting every prior event and
  creating unbounded input size;
- whether each Q1.62 component endpoint uses the existing canonical signed
  decimal codec and is restricted to `[-1, 1]`;
- rejection of reversed bounds, a box containing the zero vector, a box that
  does not intersect the unit sphere, and a face-normal interval that does not
  point strictly from the declared source cell to the target cell;
- exact replay of the physical recipe, shared face, media and three-band
  refractive-index record without reintroducing an exact path query;
- per-band output semantics when dispersion makes one lane all-TIR, another
  all-transmit and another `ambiguous_interface_branch`; and
- the distinction between sound physical width, avoidable numerical padding,
  branch ambiguity and arithmetic defects.

Nonzero provenance labels alone do not prove that a direction box came from a
prior event. If v1 accepts declared conditional evidence instead, that
limitation must be explicit in the identity and public nonclaims rather than
quietly described as end-to-end path proof.

## Resource questions that remain open

The retained observations are strong but not yet a complete resource contract:

- main-portfolio live arithmetic reached 324 bits and repeated evaluation
  reached 378 bits, but no admitted-domain derivation replaces the point
  kernel's existing 452-bit proof;
- three local bands imply at most nine band evaluations across the
  96/128/160 ladder, while 64 events are a future composer ceiling and must not
  be presented as work performed by one local event;
- the physical recipe can contain up to 65,536 runs, so canonical input bytes,
  decode allocation and peak reference memory need measurement and hard
  cardinality limits; and
- the oracle's 384-bit comparison is verification work, not authorized hidden
  production work.

The candidate may retain fixed checked 512-bit arithmetic, target-neutral
decimal endpoints, the strict 96/128/160 production ceiling, three bands and a
64-event hostile continuation fixture. It may not claim those ceilings are
closed until an algebraic live-bit bound, codec byte ceilings and a
production-computable stopping rule all pass.

## Failure points engineered out at readiness

| Tempting implementation shortcut | Failure | Permanent rule |
|---|---|---|
| copy the oracle's stop distribution | it depends on a hidden 384-bit reference | production outcomes are determined only from admitted input and declared production work |
| reuse v1's one-unit total-width test | physical input width is mistaken for rounding error | physical width and numerical padding remain separate quantities |
| always accept the 160-bit enclosure without changing the contract | removes the tested meaning of typed nonconvergence | any revised acceptance rule needs its own oracle, semantics and hostile fixtures |
| compare adjacent precisions and call stability a proof | two rounded enclosures can agree without bounding their distance from the limiting enclosure | a stopping rule needs a mathematical certificate, not empirical agreement alone |
| put the interval evaluator in a new crate | local optical equations gain two owners | the existing interface-event module remains the sole semantic owner |
| refactor v1 while adding the API and check it afterward | rollback and byte identity become unverifiable assumptions | freeze v1 vectors and identity domains before any authorized internal change |
| treat 64 chained events as one local call | resource accounting and responsibility blur | local event work and future composer work have separate receipts and ceilings |

## Required prerequisite

The next package is a disposable **interval numerical-padding certificate
design and oracle**, not Rust implementation. It must compare at least three
production-computable strategies:

1. an analytic outward-rounding error budget propagated beside every interval
   operation;
2. a deliberately widened shadow evaluation whose containment relation yields
   a proved endpoint-padding bound; and
3. a fixed 160-bit sound-result contract that abandons precision-tightness
   certification and uses `nonconvergent_enclosure` only for a precisely
   specified inability to produce a finite physical enclosure.

Any strategy must decide an event without consulting 384-bit production data.
The retained 384-bit and independent point oracles remain external truth for
testing that decision. A passing candidate must distinguish physical width
from numerical padding, derive the worst live-bit ceiling over the admitted
input domain, retain exact whole-box branch classification, and reproduce the
critical, zero-vector, grazing, sign, forced-cap, widening and three-band
64-event fixtures.

If no bounded certificate is useful, the correct result is to keep interval
composition unavailable. Raising precision, selecting a representative ray or
calling a wide sound interval “tight” are not fallback permissions.

## Compatibility, rollback and authority

The existing exact-path interface crate, contract, dependency pin and all v1
bytes remain untouched by this audit. Rollback is therefore deletion of this
readiness record and checkpoint routing only; no data migration or code
reversion exists.

There is deliberately no implementation phrase for the owner to approve.
General instructions to continue authorize the next design/oracle prerequisite
only. They do not authorize an interval schema, verified-module modification,
composer, representative ray, duplicated numerical kernel, native float,
runtime cast, coefficient catalogue, perception, rendering, collision,
navigation, organism, biome, sphere, planet, terrain, promotion or C3 closure.
