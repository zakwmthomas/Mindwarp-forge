# G1 / C3 receiver-arrival geometry implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; no receiver
schema, crate, dependency, test or source is authorized yet.**

## Exact additive package

If explicitly approved, add one capability-free workspace crate:

`crates/receiver-arrival-geometry-binding`

Its library dependencies are limited to:

- `optical-lineage-binding`;
- `physical-path-substrate`;
- `fixed-interval-arithmetic`;
- `serde` and `serde_json`; and
- `sha2`.

It must not modify any physical, bulk, interface, lineage or cumulative owner.
It must not depend on the cumulative-transfer crate because geometry and
magnitude remain independently composable proofs. It must not depend on Forge
Kernel, desktop/UI, filesystem, process, network, async, persistence or runtime
code.

## Frozen input and receiver subject

`ReceiverArrivalGeometryInputV1` binds exactly one complete
`OpticalLineageBundleInputV1`, its validated `OpticalLaneManifestV1`, and one
`ReceiverAabbV1`. The receiver contains schema version, nonzero source identity,
scope, reconstruction, revision, exact Cartesian Q160 minimum/maximum decimal
coordinates and its receiver identity.

Every minimum is strictly below its maximum. The AABB must fit inside the
closed physical volume but may span cell boundaries. Only one receiver and at
most 64 lineage steps are admitted. A point receiver is not a valid v1 receiver
and remains a contact-only oracle counterexample.

Callers submit no lane, transcript, step classification, arrival parameter,
terminal, arithmetic receipt, limitations, result identity or authority effect.

Candidate identity domains are frozen as:

- `mindwarp.receiver-arrival.aabb.v1`;
- `mindwarp.receiver-arrival.result.v1`; and
- `mindwarp.receiver-arrival.transcript.v1`.

## Frozen exact-ray admission

The compiler first replays the complete lineage bundle and manifest. For each
step in ordinal order, all point Q160, direction Q1.62 and owner-produced face-
time Q160 intervals required for classification must be degenerate. Any
nondegenerate required interval yields
`unsupported_conditional_evidence`; no midpoint, corner, representative ray or
favourable witness is used.

Unavailable-current, ambiguous-next-face and no-forward-progress outcomes yield
`upstream_terminal_without_face`. Certified next-face, outer-exit and
unavailable-neighbour outcomes may be tested before their face parameter. A
terminal face is never itself a receiver.

## Frozen strict-interior result

`ReceiverArrivalGeometryV1` contains exact receiver, lane, manifest transcript
and owner-object bindings; one of:

- `arrival_at_start`;
- `certified_strict_interior_arrival` with step ordinal and outward Q160
  parameter-infimum/parameter-supremum evidence;
- `contact_only` receipts retained while scanning later steps;
- `unsupported_conditional_evidence` with the first unsupported ordinal;
- `upstream_terminal_without_face`; or
- `no_arrival_before_lineage_terminal` with the unchanged ten-family lineage
  terminal.

Strict interior uses the open receiver slabs and `0 <= t < t_face`. Start-
inside is explicit. Tangent and point contact are not arrival. A contact or
strict-entry infimum equal to `t_face` is not current-step arrival. The
successor step, if present, owns later ordering.

## Frozen arithmetic and costs

Use `fixed-interval-arithmetic` checked signed-512 storage. Lift Q1.62 direction
to Q160 exactly, solve each nonzero slab boundary as
`(bound_q160 - point_q160) * 2^160 / direction_q160`, compare exact signed
numerator/denominator products before projection, and outward-project retained
parameter evidence to Q160 only once.

The existing physical interval owner establishes a conservative 414-bit
derived live-value ceiling for the same coordinate/direction/time construction;
v1 must use that 414-bit shield, never a looser inferred limit. Each admitted
step permits at most six directed divisions, twelve exact bound comparisons and
one three-axis intersection. Across 64 steps the receipt caps 384 divisions,
768 bound comparisons and 64 intersections.

No float, epsilon, clamp, wrapping, unchecked conversion, midpoint, corner-only
sample, representative ray or best-effort arrival is admitted.

## Codecs and resource ceilings

Strict reconstruct-and-compare JSON codecs are mandatory. Input is capped at
18 MiB, output at 256 KiB and conservative live canonical validation bytes at
32 MiB. Decode must reject the cap before allocation. Output validation replays
the complete lineage, receiver, exact-ray classification, arithmetic receipt,
identities, limitations and `none_evidence_only` authority effect.

## Required test-first shields

Before the happy path passes, port all 18 exact-rational portfolios and 26
hostile families from the pinned oracle. Rust tests must cover before/after
face, start-inside, tangent, point counterexample, face tie, parallel axes,
reverse direction, fractional entry, a multi-cell receiver, earliest ordered
step, outer exit, unavailable neighbour, ambiguous/no-progress/unavailable
current, work exhaustion and all three nondegenerate interval exclusions.

They must reject receiver, scope, reconstruction, frame, bounds, volume, lane,
transcript, ordinal, owner-object, parameter, terminal, limitation, authority,
deletion, duplication, reordering, resealing, unknown-field, trailing-byte,
receiver-count, step-65, midpoint-injection and face-tie-promotion mutations.

Pin oracle source SHA-256
`d1ea2e46e9e41e85b5523b629244b958b396903914fcf2f5dd70b7ad85f0a545`
and receipt SHA-256
`25c31003ff4ee8d1be3b01a5a2203958238205e4adc80e6cc50623c27af69aea`.

After focused native warnings-denied tests, run executable
`i686-pc-windows-msvc` tests, `aarch64-linux-android` ARM64 compilation, all
physical/lineage/cumulative owner suites and fixture locks, module boundaries,
module context, record roles, formatting and complete `tools/verify.ps1`.

## Rollback and exact owner action

Rollback is deletion-only: remove the additive crate, workspace membership,
contract/result records, module entries and permanent verifier. No existing
schema, object, identity or data requires migration.

Approval of this exact package authorizes only the capability-free exact-ray
receiver-arrival geometry reference and its frozen tests/verifiers. It does not
authorize conditional-box arrival, source emission, inverse-square spreading,
received power, aperture/orientation, detector response, detectability,
visibility, darkness, perception, rendering, gameplay line of sight, runtime,
persistence, promotion or C3 closure.

Without explicit approval, keep the Forge heartbeat paused and do not add the
crate, workspace member, dependency, schema, tests or production source.
