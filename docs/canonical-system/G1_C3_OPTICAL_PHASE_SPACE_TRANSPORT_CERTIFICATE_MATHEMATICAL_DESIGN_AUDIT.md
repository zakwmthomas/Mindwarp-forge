# G1 / C3 optical phase-space transport certificate mathematical design audit

Date: 2026-07-17

Status: **oracle-ready for a deliberately narrow free-space V1 mathematical
subject; no production schema or coupling consumer is authorized.**

## Authority and dependency boundary

The certificate is an independent derivation transcript. It consumes one
complete verified phase-space cell and derives new correlated forms; the
caller never supplies the claimed output forms. It carries no arrival,
coupling, emission, radiance, power, visibility or authority effect.

The mathematical subject binds opaque nonzero identities for scope,
reconstruction, band/time basis, physical recipe/profile and ordered topology
tokens. The oracle treats these as provenance equality constraints, not as
proof that a current owner emitted them. A later code-facing audit must decide
whether exact current-owner records can satisfy those bindings without reverse
dependencies or V1 changes.

## Selected free-space V1

V1 admits two exact-rational step families only.

### Fixed affine advance

For an exact rational advance `s` in the declared path parameter:

`P'(u) = P(u) + s V(u)` and `V'(u) = V(u)`.

Centre, four coefficients and remainder endpoints are updated by exact
rational addition and scalar multiplication. This operation is affine and
introduces no new remainder beyond the exact propagated input remainder.

### Axis-plane intersection enclosure

For plane `P_j = h`, define

`t(u) = (h - P_j(u)) / V_j(u)` and
`P'_i(u) = P_i(u) + t(u) V_i(u)`.

If the complete interval of `V_j` contains zero, the outcome is typed
`unsupported_parallel_or_reversed_plane`. Otherwise the oracle constructs a
first-order affine form at the parameter-box centre and encloses the exact
rational residual over the complete box. The remainder is not sampled: it is
bounded by rational interval evaluation of `exact(u) - affine(u)` with shared
symbol ranges retained in the affine part. The hit-axis form is exactly the
constant `h`.

The ordered topology token and crossing orientation are part of every step.
Changed tokens, reversed crossing, nonpositive forward `t`, or an interval
whose ordering cannot be proved are typed unsupported before any favourable
receiver classification.

## Explicit nonlinear stop

Exact Snell refraction introduces a square root and branch boundary. This V1
oracle does not pretend that decimal rounding or point samples prove a
whole-cell enclosure. Every interface/refraction step returns
`unsupported_nonlinear_interface`.

A later oracle may admit planar same-branch refraction only after freezing an
independent outward rational square-root enclosure, TIR classification,
derivative interval and remainder proof. That is not silently inherited from
the current point/box interface owner.

## Certificate result

An admitted transcript returns:

- the input `cell_id`, band/time basis and physical identity tuple;
- the exact ordered step/topology tokens;
- six compiler-derived correlated output forms;
- a per-step remainder-growth receipt;
- maximum rational numerator/denominator widths and operation counts;
- one identity over the complete derivation; and
- `authority_effect = none_evidence_only`.

It does not return full, zero or partial receiver coverage. The existing
whole-cell classifier remains a later consumer that must separately require
uniform topology/branch evidence and strict open-receiver comparisons.

## Required oracle portfolios

Positive portfolios must include:

1. identity and zero advance;
2. fixed advances with positive, negative and correlated coefficients;
3. exact constant-direction plane intersection;
4. variable-direction plane intersection with a nonzero conservative residual;
5. two ordered planes with stable topology tokens;
6. subdivision commutation and exact 4/16/64 child-measure conservation;
7. `u-u=0` correlation retention versus wider independent boxes; and
8. deterministic identity sensitivity to cell, band/time, physical tuple,
   step order, topology and arithmetic.

Hostile families must reject or type-stop forged output forms, wrong cell,
wrong band/time, foreign physical identities, reordered or missing steps,
topology/branch drift, denominator-zero direction, reversed or nonforward
crossing, remainder understatement, fold/derivative ambiguity, interface/TIR
steps, measure mutation, stale identity and all arrival/power/visibility or
authority fields.

## Cost and falsifier

Use Python `Fraction` only in a disposable oracle. Exhaustively enumerate box
corners for affine extrema and use exact rational interval operations for the
plane quotient and residual. Add deterministic interior searches only as
falsifiers; samples may expose an enclosure bug but never establish proof.

The subject fails if the exact rational plane portfolios escape the derived
remainder, if subdivision changes the represented union/measure, if a forged
form can reseal an identity, or if an unsupported interface receives a
favourable transport receipt.

## Stop boundary

Add no crate, contract schema, dependency, production test or source. Do not
modify the phase-space cell, physical, bulk, interface, lineage, cumulative or
receiver owners. After the deterministic oracle, perform a code-facing
readiness audit only if this narrow subject survives. Any implementation
package remains a separate serious owner decision.

