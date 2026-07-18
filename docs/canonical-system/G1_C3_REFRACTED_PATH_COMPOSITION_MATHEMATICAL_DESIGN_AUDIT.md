# G1 / C3 refracted-path composition mathematical design audit

Date: 2026-07-16

Status: **minimum candidate selected; implementation readiness subsequently
found a missing interval-continuation substrate; no implementation authorized.**

## Readiness correction

The later code-facing audit found that this document overestimated direct
reuse of the two verified optical modules. `visible-radiance-bulk-transfer`
accepts exact Q32.32 segment endpoints, while a refracted face point is an
enclosure and generally cannot be encoded as Q32.32. Likewise,
`visible-radiance-interface-event` accepts an exact integer-delta path, while
the incident direction after the first event is an enclosure. Therefore a
composer cannot honestly invoke either existing compiler for its second
segment without snapping, fabricating an exact ray, duplicating arithmetic or
changing an already verified contract.

The transmitted-only three-lane choice remains the preferred bounded topology,
but it is not implementation-ready. The statements below about evaluating
subsequent bulk spans and local events are target semantics for a future route,
not currently available APIs. The prerequisite is now an interval-continuation
counterexample and representation audit recorded separately.

## Question and boundary

Forge has exact bulk transfer along one straight physical path and exact local
reflection/transmission enclosures at one uniquely evidenced face. This audit
asks for the smallest deterministic composition that can continue direct
transmitted visible-radiance opportunity across multiple interfaces.

It does not own indirect illumination, recursive reflections, scattering,
emission, polarization, perception, detectability, rendering, real material
coefficients, passage, runtime behavior, planets, terrain or biomes.

## Candidate comparison

| Candidate | Advantage | Counterexample and decision |
|---|---|---|
| snap every event point and direction back to Q32.32 | reuses the point-path API directly | a near-edge ray can select a different next face after one least-bit snap; reject because topology becomes rounding-dependent |
| exact symbolic radical path | preserves ideal Snell geometry | successive unrelated square roots do not remain a bounded practical canonical field; reject as unbounded algebraic authority |
| recursively follow reflected and transmitted RGB branches | retains more optical energy | up to six children per event creates exponential work and crosses into indirect radiative transfer; reject for this direct-path package |
| one shared three-band continuation direction | compact | dispersion gives different transmitted directions per band; reject because it fabricates a common path |
| **three transmitted-only dyadic enclosure lanes** | bounded, matches existing outward direction intervals and preserves dispersion | **select**, with typed ambiguity whenever an enclosure cannot certify one next face |

## Selected state and reconstruction

One query declares an exact validated starting path, endpoint target region,
interface profile set and `max_interface_events` in `1..=64`. A known local
event may create at most three transmitted spectral lanes: red, green and blue.
Reflection power is retained in the event receipt but reflection is terminal
for this direct-transmission composer. Total internal reflection terminates
that lane as `total_internal_reflection`; it is not zero transmission inferred
from rounding.

Each live lane carries:

- exact volume, reconstruction, band and originating-event identities;
- a three-axis event-point enclosure and direction enclosure as canonical
  signed decimal dyadic endpoints with one explicit scale;
- accumulated directed power bounds;
- current cell, prior face, event count and ordered segment receipts.

Original Q32.32 coordinates lift exactly into the dyadic scale. V1 uses the
existing checked 512-bit arithmetic and a fixed 160 fractional-bit composition
scale. It never rounds an enclosure to a point. Face crossing evaluates all
six cell planes with outward division, rejects negative parameters, and admits
the next step only when one positive face-time enclosure is strictly before
all competitors. Overlap or equality is `ambiguous_next_face`, including
edge/corner and enclosure-width attacks. The prior face cannot be immediately
re-entered unless the certified direction points back across it; no epsilon or
origin nudge is allowed.

At a certified face, the composer reconstructs the exact neighboring cell and
requires the existing explicit face interaction evidence. Bulk transfer is
evaluated over the certified open segment per spectral lane; the local
interface event supplies the next transmitted direction and power enclosure.
All public results replay from owning inputs rather than trusting submitted
segments, faces, powers or outcomes.

## Typed termination

Known endpoint arrival, outer-domain exit, total internal reflection, opaque
bulk termination, unavailable evidence, missing interface evidence,
unsupported model, local nonconvergence, ambiguous next face, ambiguous
endpoint relation, no forward progress, repeated state, event ceiling, work
ceiling and arithmetic defect remain distinct. Only arithmetic defects are
errors; physical/evidence terminations are canonical outcomes.

A repeated-state key binds band, cell, prior face and all point/direction
enclosure endpoints. Exact repetition terminates `repeated_state`; enclosure
widening never silently retries. Each accepted step must either reach an
endpoint/terminal outcome or have a strictly positive lower face parameter.

## Resource and portability policy

- at most 3 spectral lanes;
- at most 64 interface events and 65 segment receipts per lane;
- at most 1,248 certified face-order comparisons;
- at most 3 times 64 invocations of each owning local interface/bulk kernel;
- at most 8 MiB canonical result bytes and 64 MiB peak reference memory;
- fixed 512-bit checked storage and target-neutral decimal codecs only.

One portable core remains the PC/mobile plan. Windows x64/i686 execution and
Android ARM64 compilation would be required before any reference result;
actual mobile-device cost remains a promotion gate. Platform-specific floating
point or engine ray casting cannot become canonical truth.

## Independent oracle and hostile portfolio

The implementation-readiness route must first build an independent Python
oracle using `Fraction` for rational plane geometry plus independently coded
arbitrary-precision outward interval operations. It may not reuse the Rust
face-selection or traversal routine. Required cases include normal incidence,
dispersion into three different next cells, near-parallel direction, exact
edge/corner ambiguity, prior-face re-entry, alternating-interface loop,
TIR, opaque and unavailable lanes, endpoint-on-face ambiguity, 64-event
termination, enclosure widening, reversal, unknown fields and forged history.

## Rollback and next gate

Any future composer is additive. Removing it must leave
`physical-path-substrate`, `visible-radiance-bulk-transfer`,
`visible-radiance-interface-event` and `swept-aabb-passage` byte-for-byte
semantically unchanged and require no data migration.

Next, run an implementation-readiness audit that freezes exact schemas,
postconditions, oracle vectors, cost fixtures and rollback commands. Stop at an
explicit owner implementation gate. This audit itself authorizes no crate.
