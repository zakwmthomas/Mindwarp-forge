# Optical phase-space cell binding contract

Status: owner-approved additive V1 implementation contract.

## Purpose and authority

`optical-phase-space-cell-binding` is a capability-free evidence owner. It
binds a declared four-symbol phase-space root into exact positive-measure
binary cells, deterministic ancestry, six correlated affine forms and an
authority-negative directed fixed-point projection receipt.

It does not prove the declaration is physically correct, implement optical
coupling, construct an existing owner input, or own emission, radiance,
spreading, received power, detector response, arrival, visibility, runtime,
approval or promotion. Every durable object carries
`authority_effect = none_evidence_only`.

## Frozen V1

- `TransverseAreaDirection4d` has exactly `U0..U3`.
- Output roles are exactly `PointX`, `PointY`, `PointZ`, `DirectionX`,
  `DirectionY`, `DirectionZ` in that order.
- Every affine scalar shares one positive denominator and the denominator plus
  all 42 signed numerators have collective gcd one.
- Roots admit at most 192 magnitude bits. Paths admit at most 12 binary steps.
  Every derived operation fails above the 368-bit live shield in opaque
  checked `Signed512` storage.
- A split maps `u=(v-1)/2` or `u=(v+1)/2`, conserves exact reduced measure,
  derives both complete children internally and returns `DepthLimit` without
  consuming parent measure at depth 12.
- Projection encloses each affine form by directed floor/ceiling at Q160 for
  position and Q1.62 for direction. Direction outside `[-2^62,2^62]` fails
  typed.

## Identities and codecs

Root, child, split and projection identities use SHA-256 over their frozen
domain separator, a zero byte and canonical JSON. Codecs deny unknown fields,
enforce byte ceilings before decode, revalidate identities and require exact
round-trip bytes. Alternate decimals, non-reduced rationals, reordered roles,
trailing content and forged derived fields fail typed.

## Dependency and rollback boundary

The crate depends only on `fixed-interval-arithmetic`, `serde`, `serde_json`
and `sha2`. No physical, interface, lineage, cumulative-transfer or receiver
owner imports it in V1. Rollback is deletion-only: remove the crate, contract,
tests, verifier, result and registry references; no current owner data or V1
identity is migrated.
