# G1 / C3 one-band interval bulk-transfer implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; no interval bulk
source, manifest, contract, module registry or optical arithmetic change has
been performed.**

## Bounded action

Add one conditional, one-band interval bulk-transfer surface inside the
existing `visible-radiance-bulk-transfer` owner. It consumes one validated
`ConditionalIntervalCellStepInputV1` and its matching validated event, rebuilds
the current physical cell from the existing bulk profile's volume recipe,
computes the oracle-proved dual length certificate with
`fixed-interval-arithmetic`, and reuses the existing bulk owner's Q64.64
optical-depth and Q0.48 exponential kernel.

The package is additive. It does not change existing profile, exact-path query
or three-band `VisibleRadianceBulkTransferV1` types, domains, bytes, identities,
outcomes, limitations or arithmetic. It does not call the interface owner,
bind an ordered sequence or select an endpoint.

## Frozen public surface

Add these bulk-owned types with `deny_unknown_fields` and canonical JSON:

- `VisibleRadianceBandV1`: exactly `red`, `green` or `blue`;
- `ConditionalIntervalBulkQueryV1`: schema version, exact bulk-profile ID,
  band, nested physical cell-step input and nested matching cell-step event;
- `IntervalBulkLengthCertificateV1`: speed-time Q160 enclosure,
  displacement Q160 enclosure and their nonempty Q160 intersection;
- `IntervalBulkTerminalV1`: `known_neighbor`, `unavailable_neighbor` or
  `outer_domain_exit`, including the certified neighbour index when one
  exists;
- `IntervalBulkArithmeticReceiptV1`: 160 fractional bits, 512 storage bits,
  414 derived maximum magnitude bits, observed maximum magnitude bits and
  fixed work ceilings;
- `ConditionalIntervalBulkOutcomeV1`: `known_current_cell_transfer`,
  `unavailable_current_cell`, `upstream_ambiguous_next_face` or
  `upstream_no_forward_progress`; and
- `ConditionalIntervalBulkTransferV1`: exact profile, physical volume,
  cell-step input/event, band and current-cell bindings, local outcome,
  arithmetic receipt, transfer identity, limitations and
  `none_evidence_only` authority effect.

`known_current_cell_transfer` contains the dual length certificate, exactly
one existing `BandTransferV1` and the terminal disposition. The existing band
type preserves the distinction between finite zero, finite positive, opaque
and vacuum identity. An unavailable current cell never becomes vacuum. An
outer or unavailable neighbour is attached only after retaining the certified
current-cell transfer.

The query and transfer byte caps are respectively 64 KiB and 16 KiB. The query
cap safely contains the existing 16 KiB cell-step input and 32 KiB event caps
plus the bounded wrapper; the profile remains a separate validated argument
and is never nested into the query.

## Validation and identities

The compiler and validator must:

1. rebuild and validate the exact bulk profile, physical recipe and volume;
2. require the query profile ID to equal the supplied profile;
3. validate the nested cell-step input and event through their owning physical
   functions;
4. require exact recipe, volume, current-cell and input/event identity
   agreement;
5. reconstruct current-cell evidence instead of accepting it from the caller;
6. select exactly one profile band by the closed band enum;
7. reconstruct and compare the complete transfer during decode; and
8. reject unknown fields, noncanonical bytes, foreign identities, forged
   output, limitation drift and authority mutation.

Freeze these domains:

- query: `mindwarp.visible-radiance.interval-bulk-query.v1` over its canonical
  bytes; and
- transfer: `mindwarp.visible-radiance.interval-bulk-transfer.v1` over the
  profile ID, volume ID, query ID, cell-step event ID, band, current cell,
  outcome and arithmetic receipt.

No native limb, memory-layout or platform representation may enter either
identity.

## Frozen arithmetic route

All new wide arithmetic uses the existing semantic-neutral shared crate:

1. parse every canonical decimal endpoint;
2. lift the six direction endpoints from Q62 to Q160 by an exact 98-bit shift;
3. enclose speed with three interval squares, two additions and one directed
   square root;
4. multiply speed by the certified positive Q160 face-time interval;
5. subtract start-point boxes from certified hit-point boxes, then enclose the
   displacement norm with three squares, two additions and one directed root;
6. intersect the speed-time and displacement certificates;
7. lift the selected Q16.48 finite coefficient to Q160, multiply once and
   outward-project optical depth to Q64.64; and
8. call the existing bulk exponential kernel unchanged to produce Q0.48
   transmission.

The fixed maximum wide work is seven endpoint/coefficient shifts, eight
interval multiplications, four interval additions, three interval
subtractions, two directed square roots, one intersection and one projection.
Opaque and vacuum paths may perform less transfer work but still retain the
length certificate. The existing exponential ceiling remains 192 terms.

The wide intermediate shield is 414 **magnitude** bits inside opaque 512-bit
storage. Final intersected length remains below 192 raw Q160 bits and finite
coefficient multiplication below 256 raw bits before Q64.64 projection. The
implementation records observed magnitude bits and fails before returning an
event if any derived ceiling, storage bound, scale, interval order or
intersection invariant fails.

Every `FixedArithmeticError` maps to one bulk-owned nonserialized arithmetic
failure category. Shared error display text never enters bytes, identities or
limitations. Precision policy, band meaning, coefficient meaning, codecs and
outcomes remain in the bulk owner.

## Exact bulk V1 compatibility capture

Before adding the direct shared-core dependency or conditional source, capture
canonical input/profile/query/transfer byte lengths and SHA-256 values plus all
available profile, volume, path-query, path-witness and transfer identities for
eight permanent existing V1 families:

1. vacuum identity;
2. finite-zero identity;
3. finite-positive attenuation;
4. opaque termination;
5. unavailable evidence;
6. ambiguous boundary lane;
7. interface-model-required transition; and
8. stationary point behavior.

All eight families must remain byte- and identity-identical after the additive
change. Existing exact-path, physical interval cell-step and optical point-V1
identity fixtures remain downstream shields.

## Hostile and oracle matrix

The additive tests must cover finite-zero, finite-positive, opaque, vacuum and
unavailable current cells; all three bands; all six face directions;
minimum-positive Q1.62 motion; zero-straddling, near-parallel,
correlation-erased, negative-coordinate and maximum-range boxes; exact and
nonsquare norms; empty dual-certificate intersection; outer exit,
unavailable neighbour, known neighbour, ambiguous face and no-forward-progress
outcomes; coefficient extremes; byte-cap poison; unknown fields; foreign
profile/volume/input/event identities; forged arithmetic receipts; and four
64-step dispersed-band lanes.

The permanent independent Python oracle must retain its source and receipt
hashes, 11 named cases, 512 named and 15,808 generated corner witnesses, four
64-step lanes, 16,384 repeated-lane witnesses, 321 observed live bits and the
414-bit derived shield. Rust fixed vectors and generated cases must match its
length, optical-depth, transmission, terminal and ambiguity results.

## Dependency, allocation and module boundary

Add exactly one local dependency from `visible-radiance-bulk-transfer` to
`fixed-interval-arithmetic`. This adds no external package, version or feature;
the shared crate retains exactly `crypto-bigint = 0.7.5` with default features
disabled. Do not add a dependency on `visible-radiance-interface-event` or
expose native `crypto-bigint` values in bulk source.

Update the canonical module registry so the shared arithmetic module lists
bulk transfer as a downstream neighbour and bulk transfer lists the shared
core as an upstream neighbour. Preserve physical-path substrate as the owner
of cell-step reconstruction.

Arithmetic operations allocate no dynamic collections. Canonical decimal
conversion allocates only bounded strings. Query/event decoding is capped at
64/16 KiB, and existing profile substance and volume-cell ceilings remain
65,536. No filesystem, network, process, persistence, UI or external
capability is added.

## Verification gates

The exact package must pass:

- all eight pre-source bulk V1 compatibility families unchanged;
- `cargo test -p fixed-interval-arithmetic` and warnings-denied native bulk
  tests;
- executable `i686-pc-windows-msvc` tests for shared arithmetic, physical
  substrate and bulk transfer;
- `aarch64-linux-android` compilation for all three crates;
- physical interval cell-step, exact-path, swept-AABB and optical interface
  suites unchanged;
- the permanent one-band interval-bulk and cell-step Python oracles;
- a new verifier rejecting private signed-512 code, optical dependencies,
  domain drift, missing compatibility families, cap drift and capability
  imports;
- Cargo metadata proof of no new external package, version or feature;
- module-context and record-role verification; and
- the complete Forge gate plus `git diff --check`.

Actual Android-device execution and production performance remain later
promotion evidence. Passing host compilation is not a mobile-performance
claim.

## Rollback

Rollback is deletion-only because the package is additive and no stored data
or existing public identity may migrate.

On any byte, identity, arithmetic, oracle, feature, platform, dependency,
module or full-gate failure, delete the additive conditional types/functions,
tests, fixtures, verifier and contract additions; remove the direct shared-core
dependency and module-neighbour entries; and restore the pre-action bulk
source. Existing bulk V1 has no data migration and must remain the exact
rollback target.

## Exclusions

The package does not authorize optical arithmetic migration or shared API
expansion. It also does not authorize ordered lineage, multi-cell or
multi-interface composition, endpoint arrival, coefficient discovery or SI
mapping, perception, rendering, gameplay visibility, collision, navigation,
organism, biome, sphere, planet, terrain, runtime, promotion or C3 closure.

## Exact owner action

Approve one test-first additive implementation package that:

1. captures the eight exact bulk V1 byte/identity families before source;
2. adds only a direct local `fixed-interval-arithmetic` dependency to the bulk
   owner with no external dependency or feature change;
3. implements the frozen one-band query, transfer, identities, typed outcomes,
   dual length certificate and 414-bit arithmetic receipt;
4. reuses the existing bulk optical-depth and exponential policy without
   changing bulk V1;
5. adds no optical dependency, lineage, composer or downstream semantics;
6. passes native x64, executable i686 Windows, Android ARM64, independent
   oracle, compatibility, downstream, module and complete-Forge gates; and
7. rolls back the entire additive package on any drift.

General continuation does not authorize this source action. Explicit owner
approval of this exact package is required.
