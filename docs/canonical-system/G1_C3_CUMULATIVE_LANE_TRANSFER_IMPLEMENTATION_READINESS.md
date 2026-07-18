# G1 / C3 cumulative lane-transfer implementation readiness

Date: 2026-07-16

Status: **implementation-ready package explicitly approved on 2026-07-16;
additive source implementation is in progress.**

## Exact package

If explicitly approved, add one capability-free workspace crate:

`crates/optical-lane-transfer-binding`

It may depend only on:

- `optical-lineage-binding`;
- `visible-radiance-bulk-transfer`;
- `visible-radiance-interface-event`;
- `fixed-interval-arithmetic`;
- `serde` and `serde_json`; and
- `sha2`.

It must not modify the physical-path, bulk-transfer, interface-event or
optical-lineage manifests or source. It must not depend on Forge Kernel,
desktop/UI, filesystem, process, network, async, persistence or runtime code.

## Frozen v1 surface

`CumulativeOpticalLaneTransferInputV1` binds exactly one complete
`OpticalLineageBundleInputV1` and its validated `OpticalLaneManifestV1`.
Callers submit no factor list, cumulative endpoints, work receipt, terminal or
output identity.

`CumulativeOpticalLaneTransferV1` contains:

- schema version and exact bundle/manifest/lane/band identities;
- ordered compiler-produced `CumulativeLaneFactorReceiptV1` records;
- retained Q0.160 lower and upper decimal endpoints;
- public Q0.48 lower and upper integer endpoints;
- the unchanged final `OpticalLineageTerminalV1`;
- a bounded arithmetic work receipt;
- result and transcript identities;
- exact limitations; and
- authority effect `none_evidence_only`.

Strict reconstruct-and-compare JSON codecs are required for input and output.
Input is capped at 18 MiB, output at 256 KiB, factor count at 128 and
conservative one-lane validation live canonical bytes at 32 MiB.

Candidate identity domains are frozen as:

- `mindwarp.optical-lineage.cumulative-factor.v1`
- `mindwarp.optical-lineage.cumulative-result.v1`
- `mindwarp.optical-lineage.cumulative-transcript.v1`

## Frozen factor extraction

The compiler first validates the complete lineage bundle and manifest through
`optical-lineage-binding`.

For every manifest step in ordinal order it appends:

1. exactly one same-band bulk factor when the validated bulk owner outcome is
   `known_current_cell_transfer`:
   - vacuum `[2^48,2^48]`;
   - finite owner endpoints; or
   - opaque `[0,0]`; then
2. exactly one same-band transmitted-interface factor only when the manifest
   disposition is `continue_after_interface`.

The factor receipt binds ordinal, role, band, selected owner object identity
and exact Q0.48 endpoints. The three upstream terminal outcomes
`unavailable_current_cell`, `upstream_ambiguous_next_face` and
`upstream_no_forward_progress` contribute no factor because their owner never
evaluated a current-cell transfer. Same-medium continuation contributes no
interface factor. All-TIR, ambiguous, nonconvergent and unsupported terminal
interfaces contribute no interface factor. Unavailable and ambiguous evidence
never becomes zero. The final ten-family lineage terminal is copied without
reclassification.

## Frozen arithmetic

Use the design audit's directed Q0.160 recurrence:

`L' = floor(L*a/2^48)`, `U' = ceil(U*b/2^48)`.

Initialize both endpoints to `2^160`; outward-project once to Q0.48 at the
end. All work is checked integer arithmetic through
`fixed-interval-arithmetic`. The hard live-value shield is 209 bits in opaque
512-bit storage. The receipt caps 128 lower multiplications, 128 upper
multiplications, 128 floor divisions, 128 ceiling divisions and one final
lower/upper projection pair.

No float, epsilon, clamp, wrapping, unchecked conversion, representative
factor, repeated Q0.48 projection or best-effort output is admitted. Public
`[0,0]` requires an owner-produced exact-zero factor. `[0,1]` remains a valid
underflow enclosure and has no darkness meaning.

## Required test-first shields

Before the happy path passes, port the oracle portfolios and all 26 hostile
families. Rust tests must cover:

- vacuum, finite, bulk-plus-interface and opaque factors;
- products below Q0.48 and Q0.160;
- 64 bulk and 128 mixed-factor costs;
- all ten copied lineage terminals;
- factor identity, exact order and same-band selection;
- deletion, duplication, reordering, role and band substitution;
- foreign/stale/resealed nested evidence;
- terminal and same-medium interface injection;
- unavailable/ambiguous-to-zero and zero/positive mutation;
- invalid endpoints, factor 129 and 209-bit shield bypass;
- repeated-Q0.48 false-zero policy;
- strict unknown-field, trailing-byte, input/output cap, output-forgery,
  limitation and authority rejection; and
- validation replay after every output mutation.

The permanent independent oracle remains mandatory. Pin source SHA-256
`62cdd6d36a2c74d315a9990a17b06641fbeb1f04ed747dab8c0d1e9f203d88fa`
and receipt SHA-256
`ee5f237fe1c8b7581372646e01ab12c7ddedfa1707d1b0e5dbf199e81b2ba09d`.

## Pre-source and integration gates

Before workspace or source change, rerun and record:

- the four frozen local-owner fixture hashes;
- all physical-path, bulk-transfer and interface-event owner suites;
- the optical-lineage package tests;
- optical-lineage permanent verifier; and
- cumulative design/oracle verifiers.

After source exists, rerun all of the above plus native warnings-denied tests,
executable `i686-pc-windows-msvc` tests, `aarch64-linux-android` ARM64 check,
module boundaries/context, record roles, formatting and complete
`tools/verify.ps1`.

Any owner fixture, dependency, identity, test, platform, cap or complete-gate
drift fails the package.

## Rollback

Rollback is deletion-only: remove the new crate, workspace membership,
contract/result records, module entries and permanent implementation verifier.
No existing schema, object, identity or data requires migration.

## Exact owner action

Approval of this exact package authorizes only the additive
`optical-lane-transfer-binding` reference and the frozen tests/verifiers above.
It does not authorize source emission, inverse-square spreading, receiver
geometry, endpoint arrival, aperture/orientation, detector response,
detectability, visibility, perception, rendering, gameplay line of sight,
runtime integration, persistence, promotion or C3 closure.

The owner explicitly approved this exact package on 2026-07-16. The approval
does not widen any exclusion above; any material change still requires a new
owner checkpoint.
