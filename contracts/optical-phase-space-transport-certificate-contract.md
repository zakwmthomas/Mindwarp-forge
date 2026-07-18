# Optical phase-space transport certificate contract

Version: 1  
Status: owner-authorized additive proof component

## Purpose

`optical-phase-space-transport-certificate` replays one complete exact
four-symbol phase-space cell and one complete physical volume, then records a
bounded ordered run of same-medium axis-face crossings. Every correlated face
form is derived directly from the immutable input cell. A predecessor face
form is never used as a new algebraic origin.

The certificate is evidence only. It grants no reflection, refraction,
scattering, attenuation, power, coupling, receiver-arrival, visibility,
perception, runtime, promotion or C3-closure authority.

## Owners and dependencies

The crate consumes only public unchanged APIs from:

- `optical-phase-space-cell-binding` for strict cell replay and initial
  Q160/Q1.62 projection;
- `physical-path-substrate` for strict recipe, volume, cell and conditional
  interval-step replay; and
- `fixed-interval-arithmetic` for opaque checked signed-512 arithmetic.

It adds no import to an existing owner and may not be imported by a coupling or
arrival consumer in V1.

## Frozen admission and work bounds

- schema version: 1;
- immutable-origin form denominators and signed numerators: at most 64 bits;
- maximum requested same-medium steps: 64;
- checked storage: 512 bits;
- conservative live arithmetic shield: 490 bits;
- input: 16 MiB;
- one face step: 256 KiB;
- certificate: 20 MiB;
- aggregate live canonical bytes: 40 MiB;
- physical cell-step calls: at most 64;
- direct-form public scalars: at most 1,024;
- checked domain arithmetic operations: at most 24,576; and
- directed projections: at most 768.

Canonical reduction is one checked domain operation; internal Euclidean
iterations are width-observed but are not misreported as separate transport
operations. Directed divisions remain counted separately.

## Direct immutable-origin rule

For physical Q32.32 face numerator `H`, scale `S = 2^32`, immutable common
denominator `D`, hit axis `j`, position numerators `p`, direction numerators
`v`, `A = H*D - p_j*S`, and central hit-axis direction `b = v_j`:

- tangential centre is `(p_i*S*b + A*v_i) / (D*S*b)`;
- each shared-symbol coefficient has denominator `D*S*b^2`; and
- coefficient numerator is
  `p_i,k*S*b^2 + (-p_j,k*b*S - A*v_j,k)*v_i + A*v_i,k*b`.

The hit-axis form is exactly `H/S`. The residual numerator is evaluated as one
exact quadratic polynomial over the four shared symbols and the four relevant
declared remainder intervals. Termwise interval bounds preserve correlation
and use the deliberate shared endpoint denominator `D*S*V_endpoint*b^2`.
This tightened bound was required by integrated testing: the earlier
independent output-extrema subtraction was safe but unnecessarily wider than
the unchanged physical owner's certified face enclosure.

Every direct projection must be contained by the current physical face
receipt. Failure is the typed `ProjectionOutOfRange` terminal; containment is
never weakened and the physical owner is never modified.

## Terminals

V1 terminates as exactly one of:

- `InterfaceRequired` for any complete medium or substance change;
- `OuterDomainExit`;
- `UnavailableNeighbor`;
- `AmbiguousNextFace`;
- `NoForwardProgress`;
- `ArithmeticShieldExceeded`;
- `ProjectionOutOfRange`; or
- `WorkExhausted` after exactly the requested otherwise-valid steps.

No terminal implies darkness, zero power, no arrival or invisibility.

## Codec and identity

All public structs deny unknown fields. Decimal integers and reduced rationals
are canonical. Decode checks byte ceilings before allocation, replays complete
nested owners, re-encodes, and requires byte equality. Certificate validation
recompiles and compares the complete result.

Identities use `SHA-256(domain || 0x00 || canonical_json)` with domains:

- `mindwarp.optical-phase-space.transport.input.v1`;
- `mindwarp.optical-phase-space.transport.form.v1`;
- `mindwarp.optical-phase-space.transport.step.v1`; and
- `mindwarp.optical-phase-space.transport.certificate.v1`.

Order, terminal, nested inputs/events, forms, projections, receipts,
limitations and `none_evidence_only` authority are identity-bound.

## Rollback

Rollback is deletion-only: remove this crate, contract, verifier, result and
their workspace/registry references. No existing data, codec, identity or
owner behavior is migrated.
