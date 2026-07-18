# G1 / C3 origin-anchored optical phase-space transport design audit

Date: 2026-07-17

Status: **oracle candidate only. The rejected repeated-relinearization design
is replaced by one immutable-origin free-space run; no implementation schema
or source is authorized.**

## Why this is a distinct representation

In a uniform free-space run, direction is constant. Every later axis-face hit
is a point on the original line, so its exact rational function is derived
directly from the original correlated cell forms:

`Q_i(u; h,j) = P_origin,i(u) + ((h-P_origin,j(u)) /
V_origin,j(u)) V_origin,i(u)`.

The preceding widened affine enclosure is never used as the mathematical
origin of the next derivation. It is used only as a conservative projected box
for the unchanged physical cell-step owner to certify the next topology event.
This separates two proofs:

- current-owner Q160/Q1.62 boxes prove a unique ordered physical face for the
  complete cell; and
- immutable original rational forms derive the correlated face enclosure.

The final certificate identity binds both. Neither proof substitutes for the
other.

## Candidate run compiler

The capability-free input contains one complete replayed
`OpticalPhaseSpaceCellV1`, one complete physical recipe and volume, one current
cell index, one nonzero band/time binding and a maximum-step ceiling. The
caller supplies no plane, face, output form, topology token or branch result.

For each step, the candidate compiler:

1. projects the current origin-derived face forms outward to Q160/Q1.62;
2. constructs the existing conditional interval cell-step input with the
   original source/scope/reconstruction identity and current cell;
3. calls the existing physical owner to derive the unique event;
4. returns typed unresolved for ambiguity or non-forward progress;
5. derives the certified face height from the replayed physical cell, never
   from caller data;
6. computes the correlated face forms directly from the immutable origin;
7. verifies the derived projection is enclosed by the physical event face
   receipt and advances to the certified neighbour only when its evidence is
   exactly the same free-space medium; and
8. records the ordered event ID, face, current/successor cells, direct-form
   identity and arithmetic receipt.

Outer exit, unavailable neighbour, resource exhaustion and medium change are
typed terminals. A medium change returns `interface_required` before any
direction mutation. Interface, reflection, refraction, TIR, scattering and
caller-provided continuation remain unsupported.

## Optimized common-denominator algebra

For immutable-origin common denominator `D`, point numerators `p`, direction
numerators `v`, plane axis `j` and exact Q32.32 height `h`, define
`A = hD-p_j` and `b=v_j`. The first-order transverse forms use:

- centre denominator `D b` and numerator `p_i b + A v_i`;
- coefficient denominator `D b^2`; and
- coefficient numerator
  `p_i,k b^2 + (-p_j,k b - A v_j,k) v_i + A v_i,k b`.

The hit axis is exactly `h`. Complete input extents retain the original common
denominator. Exact quotient/product interval endpoints are formed with
cancelled shared `D`, and residual subtraction uses the deliberate common
denominator `D * V_endpoint * b^2`, not generic repeated rational addition.

No gcd cancellation is required for the safety bound. Collective reduction
may shrink canonical output only after all checked intermediates pass.

## Width hypothesis to falsify

If every immutable-origin denominator and signed numerator is at most `B`
bits, the dominant unreduced residual numerator is conservatively near
`4B + 70` bits for a 64-bit signed Q32.32 face height. Q160 outward projection
adds 160 shift bits, making the dominant candidate near `4B + 230`.

The exact oracle must derive, not assume, the off-by-one constants and test all
sign/order cases. Candidate caps around 64 to 70 bits are worth measuring. A
cap is viable only if:

- all arithmetic and Q160 projection intermediates remain at most 512 bits
  without favourable reduction;
- two- and three-face runs reuse the same immutable-origin bound rather than
  compound it;
- direct face projections preserve strict positive topology order in useful
  portfolios; and
- one-bit-over-cap hostiles fail before partial favourable output.

The current cell owner remains broader. A later transport owner may type-stop
cells outside its independently frozen cap without changing or weakening the
cell V1 contract.

## Required oracle portfolios

The disposable oracle must include:

- exact optimized algebra versus direct `Fraction` evaluation at all 16
  corners and deterministic interiors;
- positive one-, two- and three-face grid runs from one immutable origin;
- face-order and successor-cell replay using projected complete intervals;
- caps spanning 16 through 112 bits, with dense testing around the boundary;
- raw pre-reduction, stored and projection-shift widths;
- denominator-near-zero, mixed-sign direction, face tie, non-forward,
  medium-change, outer, unavailable and resource terminals;
- forged face, event, output, band/time and stale run-identity rejection; and
- an explicit comparison showing repeated relinearization fails where the
  origin-anchored run remains bounded.

## Falsifier and stop

Reject the candidate if no input cap of at least 32 bits supports three useful
ordered faces within 512 bits, if topology replay needs caller-declared truth,
if direct origin projection escapes a physical face receipt, or if width grows
with step count despite immutable anchoring.

Add no crate, contract schema, dependency, production test or production
source. If the oracle survives, return to the code-facing readiness audit and
freeze an exact serious owner action. If it fails, reject this certificate
route and retain strict whole-cell `unresolved` semantics.

