# G1 / C3 whole-cell dimensionless-transfer mathematical design audit

Date: 2026-07-17

Status: **oracle-ready as a conservative same-medium optical-depth
composition; not implementation-ready because the existing exponential kernel
is not a public shared seam and band/time identity needs an explicit additive
binding.**

## Subject

The subject is one validated `WholeCellReceiverCouplingV1`, its unchanged
origin-anchored transport input/certificate, one unchanged visible-radiance
bulk profile, one RGB band and one explicit nonzero time-basis identity. The
candidate replays every nested owner. It accepts a band only when an additive
band/time binding deterministically reproduces the transport certificate's
opaque `band_time_id`; caller proximity or an unverified enum is insufficient.

For every transport step through the selected step, the candidate constructs
the existing `ConditionalIntervalBulkQueryV1` from that step's stored
`physical_input` and `physical_event`. The visible-radiance bulk owner therefore
continues to derive current-cell evidence, complete-cell path-length bounds,
optical depth and transfer. A caller supplies none of those values.

The result is dimensionless direct-beam evidence only. The phase-space measure
remains a separate exact bucket and is never interpreted as watts, radiance or
detector input.

## Why the exact-lane result is not reused

`optical-lane-transfer-binding` follows one exact optical-lineage manifest.
The receiver-coupling result covers a finite correlated phase-space cell.
Equal central rays do not imply equal neighboring path lengths. Importing the
exact-lane scalar would therefore bind two different subjects and is rejected.

The complete-cell route instead replays the interval bulk owner for the exact
physical step inputs already retained in the transport certificate. Lost
correlation may widen a bound, but it cannot create a favourable transfer.

## Optical-depth inputs

For step `i`, the replayed conditional bulk result is exactly one of:

- vacuum identity, represented as optical depth `[0,0]`;
- finite optical depth `[a_i,b_i]`, with `0 <= a_i <= b_i` in exact Q64.64;
- opaque evidence;
- unavailable or ambiguous evidence; or
- a terminal attached after a valid current-cell transfer.

Unavailable or ambiguous current-cell evidence makes the complete-cell result
unresolved. A known-neighbor, unavailable-neighbor or outer-domain terminal
does not erase the current-cell transfer already owned by that result.

The 64-step transport ceiling and the interval bulk owner's per-step
Q64.64 raw optical-depth bound below `2^112` imply a finite prefix sum below
`2^118`. This is a design bound, not production readiness; codecs, operation
counts and a live arithmetic shield remain to be audited separately.

## Receiver-face rule

Suppose coupling is `CertifiedFullBeforeFace` by `ReceiverFace` at selected
step `k`. Every complete step `i < k` is traversed before receiver entry. The
selected step is only partially traversed. For finite evidence:

`A = sum(a_i for i < k)`

`B = sum(b_i for i <= k)`

Every admitted member's optical depth to first receiver entry lies in `[A,B]`.
The selected step contributes no positive lower bound because the current
receiver proof permits entry at the segment start; it contributes its
full-step upper bound because entry is strictly before the physical face.

If a mandatory prefix step is opaque, transfer is exactly zero. If only the
selected partial step is opaque, transfer remains `[0, exp(-A)]`: entry may be
at the segment start, so opacity alone cannot prove a positive opaque span.

## Start-inside rule

If `StartInside` occurs at step zero, first arrival is at the source parameter
origin and optical depth is exactly `[0,0]`, hence transfer is identity.

For `StartInside` at step `k > 0`, the existing result proves only that entry
occurred before the selected segment start. It does not identify the earlier
entry step. With finite prior steps the conservative interval is:

`[0, sum(b_i for i < k)]`.

An opaque prior step widens transfer to `[0,1]`; it cannot prove zero because
the receiver may have been entered before that opaque step. This distinction
prevents a later selected step from rewriting first-entry history.

## Zero and unresolved rules

`CertifiedZeroBeforeFace` carries unchanged zero accepted measure and no
receiver transfer factor. It is not darkness: other source cells, paths,
bands, scattering and emission remain outside the subject.

`UnresolvedReceiverCoupling` carries unchanged unresolved measure and no
transfer factor. Subdivision may classify exact children, but no parent or
child measure may be dropped, duplicated, averaged or promoted from samples.

## Single attenuation evaluation

For a finite optical-depth enclosure `[A,B]`, monotonicity gives:

`T = [exp(-B), exp(-A)]`.

The oracle evaluates each endpoint with exact-rational outward bounds and only
then projects if a fixed output scale is being tested. This avoids multiplying
already projected Q0.48 step factors. A positive exact lower bound that
projects to zero is typed numerical underflow, never opaque evidence.

The existing bulk owner has the verified range-reduced directed exponential
kernel, but that kernel is private. A future implementation must not duplicate
it or silently modify the bulk V1 owner. Code-facing readiness must compare a
small capability-free shared exponential seam, a new bulk-owned public
read-only evaluation receipt, and conservative multiplication of existing
public transfer intervals. No route is selected here.

## Exact band/time binding

The transport owner intentionally treats `band_time_id` as opaque, while the
bulk owner accepts an RGB enum. A future additive consumer must bind one
nonzero time-basis ID and one RGB band under a domain-separated canonical
identity, then require exact equality with the transport `band_time_id`.
Cross-band substitution, zero time basis, caller-declared equality and stale
binding identities fail closed. This design grants the binding no source
quantity or calibration meaning.

## Oracle obligations

The disposable exact-rational oracle must retain at least:

1. first-step and later-step receiver-face finite transfer;
2. step-zero and later-step start-inside behavior;
3. vacuum identity;
4. mandatory-prefix opaque zero versus selected/uncertain opaque widening;
5. zero and unresolved coupling with exact measure retention;
6. 4-, 16- and 64-child measure conservation;
7. central-lane substitution and partial receiver-truncation counterexamples;
8. single-evaluation containment and repeated-Q0.48 avoidable underflow;
9. unavailable, ambiguous and interface-required typed stops; and
10. wrong band/time, reordered/deleted step, forged optical depth, selected
    index, source-power, detector, visibility and authority mutations.

## Stop boundary

Run the exact-rational oracle only. Add no crate, contract schema, dependency,
production test, production source or runtime integration. Modify no existing
owner. Do not claim source magnitude, radiance, received power, detector
response, visibility, perception, darkness, rendering, gameplay line of
sight, promotion or C3 closure.

If the oracle survives, the next action is a code-facing readiness/gap audit
for band/time identity, exponential-kernel ownership, exact codecs, resource
ceilings and deletion-only rollback. That later audit still grants no source.
