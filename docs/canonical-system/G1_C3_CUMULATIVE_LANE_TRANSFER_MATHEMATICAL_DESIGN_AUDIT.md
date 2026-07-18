# G1 / C3 cumulative lane-transfer mathematical design audit

Date: 2026-07-16

Status: **bounded directed Q0.160 accumulator selected for independent oracle;
no schema or implementation is authorized.**

## Quantity and input boundary

The candidate owns one dimensionless direct-beam transfer enclosure along the
exact followed portion of one validated `OpticalLaneManifestV1`. Its input must
contain the complete `OpticalLineageBundleInputV1` and manifest; the lineage
owner replays both before any factor is selected.

For each manifest step, factor order is fixed:

1. the step's same-band bulk factor; then
2. only when disposition is `continue_after_interface`, the selected band's
   transmitted interface-power factor.

Vacuum contributes exact one, opaque contributes exact zero, and a finite bulk
factor contributes its owner-produced Q0.48 lower/upper endpoints. A continued
interface contributes its owner-produced Q0.48 transmitted endpoints. Same-
medium continuation has no interface factor. Terminal interface outcomes do
not contribute reflected, guessed transmitted or zero factors because the
lane did not follow any such branch. Unavailable and ambiguous local outcomes
cannot appear as numeric factors.

The final lineage terminal is copied exactly and remains semantically separate
from the numeric product.

## Directed accumulator

Let `S = 2^160`. Initialize `L = U = S`. Every admitted Q0.48 factor has
integer endpoints `0 <= a <= b <= 2^48`. Update in exact unsigned integer
arithmetic:

```
L' = floor(L * a / 2^48)
U' = ceil (U * b / 2^48)
```

The final public Q0.48 enclosure is:

```
lower48 = floor(L / 2^112)
upper48 = ceil (U / 2^112)
```

All values are nonnegative and each factor is at most one, so multiplication
is monotone. `L` is never greater than the exact product scaled by `S`; `U` is
never less. Induction therefore proves containment after every factor and at
final projection.

The largest live multiplication is `2^160 * 2^48 = 2^208`, whose inclusive
bit length is 209. A 256-bit unsigned work value is sufficient; an eventual
implementation may instead reuse the opaque checked signed-512 shared
arithmetic, but it must freeze a 209-bit live-value shield and expose no native
limbs. At most 64 bulk plus 64 transmitted-interface factors are admitted, so
the hard factor and multiplication ceiling is 128. The work receipt records
factor count, multiplications, floor projections, ceiling projections,
fractional bits, maximum observed bit length and the 209-bit shield.

## Underflow and zero semantics

Repeated projection may make the retained lower endpoint zero while the upper
endpoint remains positive. This is a valid enclosure, not opacity, darkness or
non-detectability. Directed ceiling means the final upper Q0.48 endpoint is
positive whenever the retained upper Q0.160 endpoint is positive. Consequently
the public result may be exactly `[0,0]` only after an owner-produced exact-zero
factor. No clamp, epsilon, float or best-effort state is admitted.

The oracle must compare every retained update and final projection with the
exact rational product, including products below Q0.48 and Q0.160 resolution.

## Candidate evidence surface

A future readiness audit may consider one downstream additive crate with:

- a strict input binding the complete bundle and manifest;
- ordered factor receipts binding step ordinal, factor role, band, local owner
  object identity and exact Q0.48 endpoints;
- cumulative Q0.160 retained endpoints and outward Q0.48 public endpoints;
- the unchanged ten-family final lineage terminal;
- a work receipt and separate result/transcript identity domains; and
- exact limitations plus authority effect `none_evidence_only`.

The candidate must not accept a caller-authored factor list. Factor receipts
are compiler output reconstructed from nested owner evidence.

## Identity and resource candidates

The oracle freezes candidate domains for later readiness comparison only:

- `mindwarp.optical-lineage.cumulative-factor.v1`
- `mindwarp.optical-lineage.cumulative-result.v1`
- `mindwarp.optical-lineage.cumulative-transcript.v1`

The existing 64-step, 16 MiB bundle, 1 MiB manifest and 384-local-object caps
remain upstream. A candidate result should be capped at 256 KiB and validation
should stay below 32 MiB conservative live canonical bytes for one lane. These
are oracle targets, not implementation authority.

## Required hostile cases

The independent oracle must reject factor deletion, duplication, reordering,
cross-band substitution, bulk/interface role substitution, foreign step,
foreign local object, stale manifest, stale bundle receipt, independently
resealed endpoint change, terminal-interface factor injection, same-medium
interface injection, unavailable-to-zero, ambiguous-to-zero, zero-to-positive,
positive-to-zero, lower/upper inversion, endpoint above one, factor 129,
live-bit shield bypass, repeated-Q0.48 false-zero policy, transcript mutation,
limitation mutation and authority mutation.

It must retain fixed portfolios for vacuum identity, one finite bulk factor,
bulk plus interface, exact opaque zero, sub-Q0.48 positive upper, sub-Q0.160
positive exact product, 64 bulk factors and 128 mixed factors.

## Stop condition

Produce only this design, an independent deterministic oracle, stable receipt
and permanent verifier. Do not add a crate, dependency, schema, Rust type or
production source. Do not define source emission, inverse-square spreading,
receiver geometry, endpoint arrival, aperture, orientation, detector response,
detectability, visibility, perception, rendering, gameplay line of sight,
runtime, promotion or C3 closure. Any implementation requires a separate
readiness audit and explicit owner approval.
