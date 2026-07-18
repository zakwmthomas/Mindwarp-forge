# G1 / C3 receiver-arrival geometry mathematical design audit

Date: 2026-07-16

Status: **an exact-ray bounded-AABB strict-interior candidate is selected for
independent counterexample proof; conditional direction boxes remain typed
unsupported and no implementation is authorized.**

## Candidate decision

The future geometry owner, if separately approved, would consume one complete
validated optical-lineage bundle and manifest plus one explicit receiver
subject. It would not consume or alter the cumulative transfer result: geometry
and magnitude remain independent proofs that a later composition record may
bind by exact lane and transcript identity.

Three receiver candidates were compared:

| Candidate | Exact arrival meaning | Decision |
|---|---|---|
| point receiver | contact with a measure-zero coordinate | reject for v1 arrival; retain only as a contact counterexample |
| bounded axis-aligned box | first strict-interior parameter before the certified next-face parameter | select for oracle proof |
| receiver spanning multiple cells | the same bounded AABB tested step-by-step against exact world coordinates | support as a portfolio, not a separate shape |

The selected AABB has strict `minimum_q160 < maximum_q160` on all three axes,
one nonzero receiver source identity, exact scope and reconstruction identity,
and the same Cartesian coordinate frame as the replayed physical volume.
Coordinates are signed canonical Q160 integers. The receiver may cross cell
boundaries but must remain inside the closed physical volume.

## Exact-ray admission rule

V1 admits a lineage step for receiver classification only when all three start
point intervals, all three direction intervals, and the owner-produced
next-face time interval are degenerate. The direction must still satisfy the
upstream unit-sphere and forward-progress rules. This is an exact ray:

`x_i(t) = p_i + d_i * t / 2^62`, with `t` in owner Q160 units.

The audit does not narrow a nondegenerate box to a midpoint, corner,
representative ray or favourable witness. A nondegenerate point, direction or
face-time interval returns `unsupported_conditional_evidence`; it is neither a
hit nor a miss. This restriction is intentional: corner sampling is not a
universal quantifier and interval dependency can mix hit and miss rays.

## Strict-interior slab semantics

For each axis, solve the open inequality:

`receiver_min_i < p_i + d_i*t/2^62 < receiver_max_i`.

If `d_i = 0`, that axis admits all parameters only when `p_i` is strictly
inside the receiver slab; otherwise strict interior is impossible. For nonzero
`d_i`, exact rational division yields one open parameter interval. Intersect
the three open intervals with the step domain `0 <= t < t_face`.

The outcome taxonomy is:

- `arrival_at_start`: `t = 0` is strictly inside all three slabs;
- `certified_strict_interior_arrival`: the open intersection contains some
  exact parameter before `t_face`, with exact infimum/supremum evidence;
- `contact_only`: the closed AABB intersects the step domain but the strict
  interior intersection is empty, including a point receiver or tangent;
- `miss_before_face`: even the closed AABB does not intersect before the face;
- `unsupported_conditional_evidence`: a required interval is nondegenerate;
- `upstream_terminal_without_face`: unavailable current, ambiguous next face
  or no forward progress supplies no ordered step domain; and
- `no_arrival_before_lineage_terminal`: every admitted exact step misses or
  only contacts before the copied lineage terminal.

`arrival_at_start` is explicit rather than silently using a negative entry
parameter. A strict entry whose infimum equals `t_face` is not arrival in the
current step. Closed contact exactly at `t_face` is a face tie and is retained
as contact-only evidence; the successor step, if any, owns later ordering.

## Ordered multi-step rule

Replay the complete bundle and manifest first. Evaluate steps in manifest
ordinal order. A certified strict-interior arrival ends the scan. Contact-only
does not end it when a successor exists. An upstream terminal without a face
ends with its typed outcome. Outer exit and unavailable-neighbour steps may be
evaluated before their certified terminal face; the face itself is not a
receiver. Work exhaustion is copied only after all 64 admitted steps fail to
arrive.

The record binds receiver identity, lane ID, manifest transcript ID, step
ordinal, predecessor step ID, exact point/direction/face-time owner object IDs,
and exact rational parameter evidence. Deletion, duplication, reordering,
cross-lane substitution or independent resealing fails full lineage replay.

## Arithmetic and bounds

The oracle uses unbounded exact rational arithmetic. A future code-facing audit
must derive a fixed-width shield from the existing coordinate and direction
ceilings; this design does not guess one. It must compare every public directed
endpoint with the exact rational result, cap at 64 lineage steps and one
receiver, and bound input, output and conservative live canonical bytes before
decode or allocation.

No float, epsilon, clamp, midpoint, corner-only sample, representative ray,
wrapping operation, unchecked conversion or best-effort hit is admitted.

## Counterexample portfolio

The independent oracle must cover before-face arrival, after-face miss,
start-inside, tangent contact, point contact, face tie, parallel-inside,
parallel-outside, reverse direction, receiver spanning cells, earliest of two
steps, outer exit, unavailable neighbour, ambiguous/no-progress/unavailable
current, work exhaustion and nondegenerate point/direction/time rejection.

Hostile cases must reject receiver identity, scope, reconstruction, coordinate
frame, bounds, lineage transcript, lane, ordinal, owner-object, rational
endpoint, terminal, limitation, authority, deletion, duplication, reordering,
resealing, unknown field, trailing byte and cap mutations.

## Authority boundary and stop condition

This candidate proves only strict geometric intersection of one admitted exact
ray with one explicit receiver volume before an owner-produced face boundary.
It does not prove aperture acceptance, orientation, source emission,
inverse-square spreading, received power, detector response, exposure,
detectability, visibility, darkness, perception, rendering, gameplay line of
sight, runtime integration, persistence, promotion or C3 closure.

Run only the exact-rational disposable oracle and code-facing readiness audit.
Do not add a crate, dependency, schema, tests or production source. Any
implementation requires a separate exact package and explicit owner approval.
