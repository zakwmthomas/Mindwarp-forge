# G1 / C3 cumulative lane-transfer oracle result

Date: 2026-07-16

Status: **Q0.160 directed accumulator survives the independent exact-rational
oracle and hostile portfolio; implementation-readiness audit is justified, but
no schema or source is authorized.**

## Stable receipt

`tools/prove-g1-c3-cumulative-lane-transfer.py` ran twice with byte-identical
output. Its source SHA-256 is
`62cdd6d36a2c74d315a9990a17b06641fbeb1f04ed747dab8c0d1e9f203d88fa`
and canonical receipt SHA-256 is
`ee5f237fe1c8b7581372646e01ab12c7ddedfa1707d1b0e5dbf199e81b2ba09d`.

The exact-rational oracle confirms containment after every factor and final
Q0.48 projection for vacuum identity, one finite bulk factor, bulk plus
transmitted interface, opaque zero, products below Q0.48, products below
Q0.160, 64 bulk factors and 128 mixed factors.

## Arithmetic result

The retained accumulator uses Q0.160 endpoints. Each Q0.48 factor update takes
floor on the lower product and ceiling on the upper product. The 128-factor
portfolio reaches, but does not exceed, the frozen 209-bit live-value shield.
The factor ceiling is 128.

A positive exact product below Q0.160 projects to public Q0.48 `[0,1]`, not
false exact zero. Exact `[0,0]` occurs only after an exact-zero owner factor.
The oracle therefore preserves the distinction between directed underflow,
opaque zero, darkness and detectability.

## Hostile result

All 26 cases are rejected, including factor deletion, duplication, reordering,
cross-band and factor-role substitution, foreign step and local-object IDs,
stale manifest and bundle receipt, independently resealed endpoint change,
terminal and same-medium interface injection, unavailable/ambiguous-to-zero,
zero/positive mutation, invalid bounds, factor-cap and live-bit bypass,
repeated-Q0.48 false-zero policy, and transcript, limitation and authority
mutation.

## Boundary

This proves only that a cumulative dimensionless followed-lane transfer owner
can be designed with bounded directed arithmetic. It does not prove source
emission, geometric spreading, receiver geometry, endpoint arrival, aperture,
orientation, detector response, detectability, visibility, perception,
rendering, gameplay line of sight, runtime behavior, promotion or C3 closure.

The next permitted step is an implementation-readiness audit that freezes the
exact additive dependency boundary, schema, domains, factor extraction rules,
caps, Rust/independent-oracle tests, platform gates and deletion-only rollback.
It must stop at an exact owner action. No crate, dependency, schema or source is
authorized by this oracle result.
