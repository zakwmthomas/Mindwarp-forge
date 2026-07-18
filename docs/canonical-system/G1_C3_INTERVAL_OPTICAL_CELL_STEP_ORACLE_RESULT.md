# G1 / C3 interval optical cell-step mathematical and oracle result

Date: 2026-07-16

Status: **bounded 3D candidate supported for implementation-readiness audit;
no schema or production source is authorized.**

## Decision

A channel-neutral independent-axis point/direction box can conservatively
certify one next Cartesian cell face with fixed-160 outward arithmetic. A face
is selected only when its complete positive time enclosure is strictly before
the lower time bound of every other possible face. Equality, overlap,
zero-time boundary escape and correlation loss remain typed outcomes; no
representative point or ray is selected.

The candidate is useful enough to advance to a code-facing implementation-
readiness audit. It does not yet authorize implementation, interval bulk
transfer or a composer.

## Candidate operation

One call receives a validated bounded physical volume, current cell, a Q160
point component box and a Q1.62 direction component box. For each possible
signed face it forms an outward time interval from the relevant point-to-face
distance divided by the admitted signed speed. A component interval spanning
zero may contribute competitor lower bounds but cannot be the selected face
because its upper time is unbounded.

The call returns exactly one of:

- `certified_next_face`, with one face, neighbor cell and containing time and
  point boxes;
- `ambiguous_next_face`, when no strict universal face order exists;
- `no_forward_progress`, when an admitted state can exit at time zero or has
  no forward component;
- `outer_domain_exit`; or
- `unavailable_neighbor`.

After certification, the hit axis is set to the exact selected plane. The two
tangential enclosures are intersected with the closed face bounds. This is a
sound narrowing justified by the replayable strict-face certificate, not a
midpoint or correlation guess.

Endpoint arrival is deliberately not a cell-step outcome. Ordering a declared
endpoint against the certified span belongs to a later bounded ordering
consumer. Bulk attenuation likewise depends on the cell-step receipt and is
not evaluated here.

## Oracle receipt

`tools/prove-g1-c3-interval-optical-cell-step.py` ran twice with byte-identical
output. Its receipt checksum is
`60984da9d8e4353852bc1831532c1026f5b12809325384b1f81e13bc520d7128`.

The disposable candidate performs fixed-160 outward integer operations while
independent Python `Fraction` calculations check every admitted endpoint
combination in the retained portfolio.

| Evidence | Result |
|---|---:|
| generated states | 256 |
| certified next face | 248 |
| typed ambiguous next face | 8 |
| generated certified corner-containment checks | 15,872 |
| named corner-containment checks | 512 |
| repeated lanes | red, green, blue and widened |
| steps per repeated lane | 64 |
| repeated corner-containment checks | 16,384 |
| maximum observed live bits | 321 |

All four repeated lanes reached cell 64 through 64 certified `x+` steps. The
widened lane's maximum retained point width was
40,564,857,892,929,568,516,028,093,169,792 Q160 units, approximately
`2.78e-17` coordinate units. The zero-straddling competitor control widened by
approximately `2.17e-19` coordinate units while still proving `x+` strictly
first.

## Hostile findings retained

- Swapping one Q1.62 raw unit between otherwise equal x/y components changes
  the certified face; a box spanning both returns `ambiguous_next_face`.
- Exact x/y face-time equality returns ambiguity.
- The independent box formed from two correlated x-face states returns
  ambiguity rather than admitting the impossible mixed y-face state.
- The smallest positive Q1.62 component remains positive and is not rounded to
  zero.
- A state already on the outward face returns `no_forward_progress`; no epsilon
  nudge creates a later state.
- Zero-straddling competitor directions, unavailable neighbors, outer exit and
  the maximum 65,536-coordinate-unit cell control remain distinct.
- Every certified named/generated/repeated endpoint combination hit the
  returned face and lay inside the returned time/point enclosure.

The generated portfolio is a utility sample, not a proof of a universal
ambiguity rate. The universal soundness rule comes from outward time bounds
and strict interval ordering; corner checks are independent regression
evidence against implementation mistakes.

## Arithmetic disposition

The observed 321-bit maximum is below checked 512-bit storage. A preliminary
admitted-domain derivation using signed Q32.32 coordinate bounds, Q160 lifting
and the smallest nonzero Q1.62 direction component gives roughly 413 magnitude
bits for the widest multiply before projection. The readiness audit must redo
that derivation directly from the Rust input bounds, include sign/storage
allowance, and freeze a source-level ceiling. The oracle result alone does not
authorize a 414-bit production claim.

## Ownership and lineage

The preferred owner is a private additive, separately versioned interval
submodule inside `physical-path-substrate`:

- it already owns canonical volume/cell geometry and validation;
- the operation remains channel-neutral and reusable;
- it need not depend on `visible-radiance-interface-event`, avoiding a
  dependency cycle; and
- existing exact Q32.32 path v1 can remain byte-for-byte unchanged.

The local step must describe its state as declared conditional point/direction
evidence with explicit nonzero provenance. It cannot prove an optical history
without depending backward on its consumer. A later composer must replay the
initial exact face point or prior step receipt, bind the corresponding interval
interface direction, and reject any chain mismatch. This separates local
replay from end-to-end lineage without nesting an unbounded event history.

## Next readiness audit

Run a code-facing implementation-readiness audit for one private additive
interval cell-step submodule in `physical-path-substrate`. It must freeze:

1. separate V1 input/event identities and domain separators;
2. declared conditional evidence and explicit end-to-end lineage nonclaims;
3. separately supplied validated recipe/volume objects rather than embedded
   65,536-cell payloads;
4. exact point-box range/cell containment and Q1.62 direction validity;
5. face-time, zero-straddle, strict ordering, tangential intersection and
   typed result semantics;
6. the admitted-domain 512-bit source shield, fixed-160 work, byte/allocation
   caps and maximum fixtures;
7. exact-path v1 byte/ID locks, dependency-cycle prevention and deletion-only
   rollback; and
8. native Windows, i686 and Android ARM64 compile lanes.

Stop at an explicit owner action only if every seam closes. Do not implement
the submodule, interval bulk, composer, coefficients, perception, rendering,
runtime, organism, sphere, planet, terrain, biome semantics, C3 closure or
promotion during readiness.
