# G1 / C3 whole-cell receiver-coupling mathematical design audit

Date: 2026-07-17

Status: **the smallest conservative receiver-before-face classifier survives
static derivation and a disposable exact-rational oracle; no schema or source
is authorized.**

## Decision

Use one separate whole-cell consumer that replays the unchanged phase-space
cell, immutable-origin transport certificate and receiver geometry. It returns
only `certified_full_before_face`, `certified_zero_before_face`, or
`unresolved_receiver_coupling`, with exact accepted, zero and unresolved
measure summing to the parent measure.

The classifier does not reuse the exact-ray receiver as a conditional owner,
does not reinterpret a physical face as a receiver, and does not estimate a
partial accepted fraction. Central rays, corners, independent coordinate boxes
and majority samples are rejected as proof.

## Exact subject

For one correlated phase-space cell with normalized parameters `u`, the
transport owner supplies exact affine start and next-face forms `p_i(u)` and
`f_i(u)` for axes `i in {x,y,z}`. Define `d_i(u) = f_i(u) - p_i(u)`. The
receiver is one positive-volume open AABB `(r_min, r_max)` in the same physical
scope. The certified segment is `p(u) + q d(u)` for `0 <= q <= 1`; receiver
arrival must occur strictly before the physical face, so the ordering proof
requires `q < 1`.

## Full certification

There are two safe full proofs.

1. **Uniform start-inside:** every correlated start form is strictly inside
   the receiver on every axis. This is arrival-at-start for the complete cell.
2. **One uniform inward receiver face:** choose one receiver face on axis `j`.
   After orienting the denominator inward, define positive
   `D(u) = +/- d_j(u)` and `N(u) = +/- (r_face - p_j(u))`. Prove over the
   complete correlated cell:
   - `D > 0`;
   - `N >= 0`;
   - `D - N > 0`, which is exact receiver-before-face ordering; and
   - for every cross axis `k`, both
     `(p_k-r_min_k)D + d_k N > 0` and
     `(r_max_k-p_k)D - d_k N > 0`.

The cross-axis expressions are exact quadratic polynomials after
cross-multiplication. Like monomials must be combined before bounding so
correlated cancellation such as `u-u=0` is retained. A termwise exact rational
bound may prove strict sign; failure to prove strict sign is unresolved, never
an invitation to sample.

A start point exactly on the selected receiver face may still certify full
when motion is uniformly inward, the cross axes are strictly interior and
`D-N>0`: every ray has a strict interior point immediately after contact and
before the physical face. By contrast, receiver contact only at `q=1` is not
arrival and remains unresolved at this design layer because face-coincident
ownership is intentionally separate.

## Zero certification

Zero is certified only when one axis strictly separates the complete swept
segment hull from the receiver: the maximum of the start/face bounds is below
the receiver minimum, or the minimum is above the receiver maximum. Boundary
equality is not used as a general zero shortcut here; tangency and
face-coincident equality remain unresolved.

This zero rule is deliberately incomplete. Proving that no ray enters by
exhausting every rational receiver face could add cost and policy without
closure value. Refinement may later turn unresolved children into proven zero.

## Unresolved and typed stops

Everything not proven by the two full rules or strict separating-axis zero is
`unresolved_receiver_coupling`. Named reasons include mixed receiver/face
ordering, direction-sign change, partial cross-axis overlap, tangency,
face-coincident equality, correlation-erasure widening, topology or interface
change, unsupported nonlinear form, fold/caustic possibility, arithmetic
shield failure and work exhaustion.

No unresolved measure may be discarded. If subdivision stops, every remaining
child contributes its exact measure to unresolved.

## Measure and refinement

Children preserve the existing exact binary ancestry and positive rational
measure. For every parent and every 4-, 16- and 64-child retained partition:

`accepted_measure + zero_measure + unresolved_measure = parent_measure`.

Classification moves measure only when a child's complete correlated domain
passes one proof above. It never copies parent measure into each child,
averages samples, infers a fraction from corners, or promotes a majority.

## Disposable oracle result

`tools/prove-g1-c3-whole-cell-receiver-coupling.py` implements exact rational
polynomial algebra, combines correlated monomials before bounding, and tests
12 portfolios plus three invalid-receiver rejections. Seven portfolios are
hostile non-full cases. Across 4, 16 and 64 children for every portfolio it ran
1,020 checks and retained exact measure conservation. Receipt checksum:

`8c9c2c6d5f5d6ab38483d1e7c769b833d5d0373378cacbf86ff59ccfba4a91aa`.

The oracle proves only the mathematical candidate. It does not freeze a V1
identity, codec, arithmetic width, byte ceiling, dependency or production API.

## Adopt / adapt / build

| Route | Decision |
|---|---|
| Reuse exact-ray receiver directly | Reject; it intentionally rejects nondegenerate evidence and owns no measure. |
| Use central ray, corners or independent boxes | Reject as proof; hidden interior order and correlation counterexamples survive. |
| Add receiver semantics to transport | Reject; transport is receiver-independent and reusable across receivers. |
| Separate conservative whole-cell consumer | Survives for a later code-facing readiness audit only. |

## Stop and authority boundary

Do not add a crate, contract schema, dependency, production test or production
source. Do not choose arithmetic widths, accept a partial fraction, mutate any
existing owner, or claim source magnitude, attenuation, power, detector
response, visibility, perception, runtime, promotion or C3 closure.

The next bounded action is a code-facing implementation-readiness audit for a
separate additive consumer. That audit must freeze identities, replay inputs,
quadratic polynomial representation, caps, hostile cases, rollback and the
exact owner action. Implementation remains a separate serious owner decision.
