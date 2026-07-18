# G1 / C3 whole-cell receiver-coupling implementation readiness

Date: 2026-07-17

Status: **owner-approved on 2026-07-17 for the exact additive implementation
action below. The immutable-origin shared-symbol representation has a 391-bit
no-favourable-cancellation bound in checked 512-bit storage.**

## Candidate boundary that survives

The smallest production candidate remains one additive, capability-free
consumer, separate from every existing owner. Its input would bind and replay:

- one unchanged `OpticalPhaseSpaceCellV1`;
- its unchanged `OriginAnchoredTransportInputV1` and complete validated
  `OriginAnchoredTransportCertificateV1`;
- one selected certified same-medium face step; and
- one unchanged `ReceiverAabbV1` in the same scope and coordinate frame.

Its output would classify the complete cell only as
`certified_full_before_face`, `certified_zero_before_face` or
`unresolved_receiver_coupling`. It would carry exact accepted, zero and
unresolved `PositiveRationalV1` measures whose sum equals the input cell
measure, plus input identities, a checked-arithmetic receipt, fixed
limitations and `none_evidence_only`.

The consumer would not call the exact-ray receiver conditionally, change the
transport certificate, estimate a partial fraction, schedule unbounded
subdivision or claim source magnitude, attenuation, power, detector response,
visibility, perception, runtime, promotion or C3 closure.

## Required replay and classification

Before classification, a candidate must canonical-round-trip the nested cell,
transport input, certificate and receiver; replay the transport certificate;
bind the immutable origin cell ID, physical volume IDs, band-time ID, selected
step ID and receiver ID; and reject a receiver with mismatched scope,
reconstruction or coordinate frame.

The only admitted full proofs remain uniform start-inside or one uniform inward
receiver face with exact `D > 0`, `N >= 0`, `D - N > 0` and both strict
cross-axis quadratic signs. Zero remains only a strict swept separating-axis
proof. Equality, mixed signs, partial overlap, tangency, face coincidence,
unsupported topology, arithmetic exhaustion and every failed strict proof are
typed unresolved outcomes that retain the complete measure.

## Arithmetic finding and resolution

The upstream owners deliberately expose different representations:

- the phase-space cell retains six correlated affine forms over four shared
  symbols with one common denominator and a 368-bit derived shield;
- the transport certificate accepts only 64-bit immutable-origin scalars but
  exposes each direct-form centre, coefficient and remainder as independently
  reduced `ExactRationalV1` values;
- transport permits a 490-bit live shield, and the frozen three-face fixture
  observes 481 live bits; and
- the receiver stores Q160 AABB endpoints.

Receiver-before-face cross-axis proof multiplies affine forms to create exact
quadratic polynomials. Multiplying two independently reduced public transport
terms can require roughly twice their live numerator width before addition or
reduction. A 481-bit observed term therefore cannot be assumed safe in the
existing checked 512-bit storage, and the current records contain no
no-favourable-cancellation bound that proves otherwise. Q160 receiver
coordinates must also be included in that bound.

This rejected public-form route is a representation failure, not a failure of
the verified mathematics. The disposable width spike instead replays the
immutable 64-bit common-denominator origin and selected physical face. Physical
volume constraints bound receiver Q160 numerators to 224 bits and physical
Q32 face numerators to 96 bits. After correlated monomial combination, its
no-favourable-cancellation bounds are 391 bits for receiver-before-face order
and 359 bits for cross-axis interior, leaving 121 bits below checked 512-bit
storage. The pinned source is
`173d8e45c3c3f7944c7cae43698c722df3df679066c5ce5be0429dddccc57292`
and receipt is
`71c6e716283348cd690887e00b265cb33997a62dc22c0b6367f68a172d969ea6`.

## Rejected shortcuts

| Shortcut | Disposition |
|---|---|
| Multiply public reduced transport rationals in `Signed512` | Reject until a worst-case pre-reduction bound fits. |
| Project to independent Q160/Q1.62 boxes first | Reject; it erases the correlation needed for full proof. |
| Use central rays, corners or samples | Reject as unsound proof of the complete cell. |
| Copy the private transport quadratic kernel into a consumer | Reject as an unverified second transport implementation. |
| Modify the existing transport owner to add receiver semantics | Reject; it breaks receiver-independent ownership. |
| Add arbitrary precision or raise the arithmetic shield now | Reject; this is a new dependency/representation decision without bounded evidence. |
| Silently reduce the 64-bit transport admission cap | Reject until utility and an exact safe ceiling are measured. |

## Completed disposable representation spike

The completed code-free spike compared
three exact strategies over the retained hostile portfolios and constructed
near-cap inputs:

1. direct public-form rational products with no favourable gcd cancellation;
2. an origin-replay shared-denominator polynomial that reuses immutable input
   evidence without changing an owner; and
3. explicitly listed lower immutable-origin caps, only if they produce a
   useful guaranteed ceiling in checked 512-bit storage.

It tracks pre-reduction numerator, product and sum widths, includes Q160
receiver endpoints, and reruns all 12 classifier portfolios, seven hostile
non-full cases, three invalid receivers and exact 4/16/64-child conservation.

The immutable-origin strategy succeeds without an owner change. Direct public
products fail at 980 bits; the selected shared-symbol strategy is frozen at a
391-bit live shield and retains the upstream 64-bit transport admission cap.

## Frozen additive dependency boundary

The proposed crate is `optical-phase-space-receiver-coupling`. It may depend
only on:

- `optical-phase-space-cell-binding` for exact cell identity, measure and
  binary refinement evidence;
- `optical-phase-space-transport-certificate` for complete input/certificate
  replay and selected physical face evidence;
- `physical-path-substrate` for unchanged physical-cell and selected face-height
  replay required by the public transport certificate;
- `receiver-arrival-geometry-binding` for the unchanged `ReceiverAabbV1` and
  its identity compiler;
- `fixed-interval-arithmetic` for opaque checked `Signed512` operations;
- `serde` with derive, `serde_json` and `sha2`.

No existing crate may import the consumer in this action. No third-party or
arbitrary-precision dependency is added. Existing owner source and V1
behaviour remain byte-for-byte unchanged.

## Frozen V1 input and output

`WholeCellReceiverCouplingInputV1` contains schema version 1, one complete
`OriginAnchoredTransportInputV1`, its complete
`OriginAnchoredTransportCertificateV1`, a zero-based selected step index, and
one complete `ReceiverAabbV1`. The selected step must exist, remain
same-medium, and bind the same scope, reconstruction, coordinate frame,
physical volume, immutable cell and band-time evidence. The caller supplies no
plane equation, form, measure allocation, classification or identity.

`WholeCellReceiverCouplingOutcomeV1` is exactly:

- `CertifiedFullBeforeFace` with `StartInside` or one selected receiver
  axis/side proof;
- `CertifiedZeroBeforeFace` with one strict separating axis/side; or
- `UnresolvedReceiverCoupling` with a typed reason for mixed order, direction
  sign, partial overlap, tangency, face coincidence, topology/interface,
  arithmetic shield, unsupported evidence or work exhaustion.

`WholeCellReceiverCouplingV1` binds input ID, cell ID, transport certificate
ID, selected step ID, receiver ID, outcome, exact accepted/zero/unresolved
`PositiveRationalV1` measures, arithmetic receipt, result ID, fixed
limitations and `none_evidence_only`. Exactly one measure bucket equals the
complete input measure and the other two are zero. A caller refines only by
using the unchanged cell split owner and recompiling; this consumer never
schedules, guesses or drops children.

Identity domains are
`mindwarp.optical-phase-space.receiver-coupling.input.v1` and
`mindwarp.optical-phase-space.receiver-coupling.result.v1`, each SHA-256 over
the domain, one zero byte and canonical JSON. All public structs deny unknown
fields. Canonical codecs replay nested owners, reconstruct all derived fields,
enforce byte caps before allocation, re-encode and require exact byte equality.

## Frozen arithmetic and resource ceilings

The implementation must use immutable-origin common-denominator plane
numerators only. It may not multiply independently reduced public direct-form
rationals. It combines like monomials before exact termwise interval bounds.

- immutable-origin admission: unchanged upstream 64-bit cap;
- receiver Q160 numerator: at most 224 bits from physical-volume containment;
- physical face Q32 numerator: at most 96 bits;
- live arithmetic shield: **391 bits** in checked 512-bit storage;
- parameter symbols: exactly four;
- polynomial degree: at most two, with 1 constant, 4 linear and 16 ordered
  quadratic terms;
- receiver faces considered: at most six;
- checked integer operations: at most 16,384;
- sign/bound comparisons: at most 4,096;
- input bytes: 40 MiB;
- result bytes: 256 KiB; and
- aggregate live canonical bytes: 64 MiB.

Every cap is fail-closed before allocation or unchecked arithmetic. Shield,
codec, dependency replay, mismatch and work failures produce unresolved
evidence or a typed invalid-input error as appropriate; none becomes full or
zero.

## Required implementation tests

An approved implementation must add tests for:

1. parity with all 12 mathematical portfolios, seven hostile non-full cases
   and three invalid receivers;
2. exact 4/16/64-child accepted+zero+unresolved measure conservation;
3. pinned width source/receipt and mandatory rejection of the 980-bit public
   reduced-form route;
4. 64-bit origin acceptance, 391-bit shield edges and 392-bit typed stop;
5. uniform start-inside, lower/upper receiver entry and start-boundary inward;
6. receiver-after-face, strict separation, partial overlap, mixed direction,
   stationary mixed cell, tangency and face coincidence;
7. correlated cancellation versus deliberately widened independent boxes;
8. first and later selected segments reconstructed from immutable origin and
   physical faces;
9. forged cell, certificate, step, receiver, scope, reconstruction, frame,
   volume, band-time, measure, result and authority rejection;
10. canonical round trips plus unknown, trailing, oversized and noncanonical
    codec failures;
11. fixed input/result byte and identity fixtures;
12. unchanged cell, transport, receiver, physical, lineage and cumulative
    owner fixtures;
13. warnings-denied native all-target tests;
14. executable `i686-pc-windows-msvc` tests;
15. `aarch64-linux-android` compilation; and
16. focused module, record-role, context and complete Forge verification.

## Files if explicitly approved

The exact action may add only the new crate source/tests, its generated
`MODULE.md`, one contract, one implementation-result record, one permanent
verifier, and the workspace/lockfile/module/record/README/master/checkpoint
references required by those files. If any existing public API cannot be
consumed unchanged, implementation stops and returns to readiness.

## Rollback and authority

This audit adds only evidence. Rollback is deletion of this record and its
checkpoint/master-program references. Existing cell, transport, physical,
receiver, lineage, cumulative-transfer and arithmetic V1 identities and
behaviours remain unchanged.

Rollback is deletion-only: remove the new crate, contract, tests, verifier,
result and registry/workspace references. There is no data migration and no
existing V1 identity rewrite.

## Exact serious owner action

Approval authorizes exactly this:

> Add the capability-free `optical-phase-space-receiver-coupling` sibling
> crate and contract with immutable-origin replay, the unchanged 64-bit input
> cap, 391-bit checked live shield, conservative full/zero/unresolved
> receiver-before-face classification, exact measure accounting, strict
> canonical identities/codecs, bounded hostile/platform/full-gate tests and
> deletion-only rollback described here. Modify no existing owner source or V1
> behaviour and add no source magnitude, power, detector, visibility, runtime,
> promotion or C3-closure authority.

Any change to the representation, 64-bit cap, 391-bit shield, dependencies,
classification semantics, measure rules, resource ceilings, authority boundary
or existing owner source requires a new serious owner decision.

## Stop

The owner delegated mathematical and engineering decisions and authorized
continuation on 2026-07-17. Implementation may proceed only inside the exact
action above. Any product-direction, strategy, creative, irreversible or
material scope change still returns to the owner.
