# G1 / C3 calibrated source-energy distribution implementation readiness

Date: 2026-07-18

Status: **ready for one explicit owner decision only. No crate, contract,
schema, dependency, consumer, production test or production source has been
added.**

## Bounded candidate

The surviving candidate is one capability-free additive sibling tentatively
named `calibrated-source-energy-distribution`. It would own exact radiant
energy allocated over one calibrated spectral/time band and one replayed,
prefix-free closed frontier of optical phase-space cells.

It would not own calibration, phase-space geometry, an emission model,
canonical density, transport applicability, received energy, detector
response, visibility, rendering, runtime, promotion or C3 closure. The crate
and contract names remain part of the owner decision.

## Corrected upstream replay boundary

The design oracle's disposable `0/1` paths were sufficient to falsify the
abstract additive measure, but they are not a production identity model. The
implemented cell owner binds each ancestry step as `(axis, side)` and derives
the complete child cell and split identity from the exact parent.

V1 therefore accepts no caller-authored child path, child measure, child cell,
child identity or phase split receipt. Compilation begins with one complete
validated `CalibratedSpectralTimeBasisV1` and one complete validated
`OpticalPhaseSpaceCellV1` root. Every refinement directive names one allocation
currently in the frontier and one `PhaseSpaceParameterAxisV1`. The candidate
calls `split_optical_phase_space_cell` with that exact parent and axis, accepts
only the returned ordered children and split identity, then atomically replaces
the parent allocation with both child allocations.

This compact replay proves one root-connected compatible split tree without
embedding 63 duplicate full upstream receipts. Missing, repeated, retired,
foreign or non-frontier parents fail. Directive order is semantic and must be
topological; result frontier order is the upstream full axis/side path order.

## Frozen V1 query records

Every record uses `deny_unknown_fields`.

`CalibratedSourceEnergyDistributionQueryV1` contains exactly:

- `schema_version: u16`, exactly `1`;
- one complete `CalibratedSpectralTimeBasisV1`;
- `selected_band`, exactly one of ordered `blue | green | red` and its replayed
  unchanged derived band/time identity;
- `source_provenance_id: [u8; 32]`, nonzero and unequal to calibration
  provenance;
- one complete root `OpticalPhaseSpaceCellV1`, which must replay as depth zero;
- `root_joules: ExactRadiantEnergyV1`; and
- an ordered vector of at most 63 `SourceEnergyRefinementDirectiveV1` records.

The calibration and root jointly supply source ID, scope ID, positive source
revision, calibrated-basis identity, band/time identity, reconstruction
identity, root identity and exact root measure. Substitution of any one changes
the subject identity or fails replay.

`ExactRadiantEnergyV1` contains canonical unsigned decimal `numerator` and
positive `denominator`. Each is a `u128` value with at most 39 ASCII digits, no
sign, whitespace or leading zero. The fraction is reduced; zero is represented
only as `0/1`. The unit is joule through the validated calibration witness and
is not repeated as caller-controlled text.

`SourceEnergyRefinementDirectiveV1` contains exactly:

- `parent_allocation_id: [u8; 32]`, naming a current frontier allocation;
- `axis: PhaseSpaceParameterAxisV1`;
- ordered `lower_joules` and `upper_joules`; and
- ordered `lower_resolution` and `upper_resolution`, each exactly
  `resolved_leaf | unresolved_within_cell`.

The directive contains no derived geometry or child identity fields. Both
children remain explicit even when one has zero energy. Absence never means
zero. `resolved_leaf` is an asserted terminal source allocation for this
distribution only; it is not a proof that nature, transport or a future source
model cannot refine the cell.

## Frozen derived records

`CalibratedSourceEnergyAllocationV1` contains the subject identity, complete
upstream cell identity, exact full axis/side path, exact upstream measure,
joules, resolution and derived allocation identity.

`CalibratedSourceEnergySplitReceiptV1` contains the subject identity, parent
allocation identity, replay-derived upstream phase split identity, ordered
child allocation identities and derived energy-split identity. It proves local
atomic replacement and exact parent/children energy equality.

`CalibratedSourceEnergyDistributionV1` contains:

- `schema_version: u16`, exactly `1`;
- the validated subject identity and root allocation;
- ordered energy-split receipts;
- the canonically ordered closed frontier allocations;
- the derived distribution identity;
- the maximum energy-arithmetic live-bit receipt;
- fixed resource counts and byte receipts;
- `authority_effect`, exactly `none_evidence_only`; and
- the exact limitations string pinned by fixtures.

Derived fields are outputs. A replay entry point reconstructs the entire tree,
all exact quantities and every identity before comparing caller-supplied
results.

## Non-circular identity graph

Every identity is `SHA-256(domain || 0x00 || canonical_json_bytes)`.

1. `subject_id` commits to source/scope/provenance/revision, calibrated basis,
   selected band and band/time identity, reconstruction and root identity.
2. `allocation_id` commits to subject, replay-derived cell identity, exact
   joules and resolution.
3. `energy_split_id` commits to subject, parent allocation, replay-derived
   upstream split identity and ordered child allocations.
4. `distribution_id` commits to subject, root allocation, ordered split IDs and
   canonical frontier allocation IDs.

Children do not contain the energy-split identity that created them, and the
distribution does not contain its own identity input. No cycle exists.

## Correction and supersession

Distribution evidence is immutable. A corrected source claim must use a
higher positive source revision and creates a new subject and distribution.
The old result remains replayable.

V1 has no `supersedes`, `active`, tombstone or promotion field. Selecting an
active source revision, linking correction evidence or superseding historical
records belongs to a later history/provenance owner. Calibration or root
changes are new physical subjects, not in-place source corrections. The
distribution owner cannot invalidate another evidence record.

## Exact conservation and density boundary

Every accepted directive proves, with checked exact arithmetic:

`parent_joules = lower_joules + upper_joules`.

Upstream replay proves both child measures equal half the exact parent measure.
Atomic replacement therefore preserves the exact root measure and energy for
root-only, balanced, mixed-depth and skewed frontiers. The final frontier is
closed and prefix-free by construction because only a current leaf can split.

Energy divided by cell measure may be computed as a coordinate-local derived
view. It is never stored in V1 identity, never treated as radiance and never
used to infer uniformity inside an unresolved cell.

## Frozen resource and arithmetic ceilings

V1 deliberately freezes the proven envelope rather than the theoretical
depth-12 maximum:

- frontier allocations: at most `64`;
- refinement directives and energy-split receipts: at most `63`;
- upstream cell depth: at most `12`;
- canonical query: at most `128 KiB`, checked before decode/allocation;
- canonical result: at most `256 KiB`;
- aggregate live canonical upper bound: at most `4 MiB`;
- exact energy components: canonical `u128` decimals; and
- local energy equality arithmetic: checked `Signed512` with a `385`-bit live
  shield.

Local conservation avoids a global common-denominator fold. At worst,
`a/b = c/d + e/f` compares `a*d*f` with `b*(c*f + e*d)`: three 128-bit factors
and one addition require at most 385 magnitude bits. Exceedance is typed
`ArithmeticShieldExceeded`, never zero energy.

The disposable oracle observed 18,737 maximum query bytes, 66,834 maximum
result bytes before the resource receipt and a 2,248,259-byte conservative live
upper bound at 64 frontier allocations. Supporting 4,096 leaves is a future
schema/resource reassessment, not an implied V1 promise.

## Code-free readiness receipt

`tools/prove-g1-c3-calibrated-source-energy-distribution-readiness.py` models
the compact axis-bearing replay, record/identity graph, exact local energy
proofs and frozen resource envelope without creating a production owner. Two
runs are byte-identical.

- Oracle source SHA-256:
  `2910e3c9836968b8fdc0accba271873a1c84efa7d1e60e5e6993373f061888a7`
- Receipt checksum:
  `e39db1445baf6a069760f1742d267427e38f885177fc6c50e83fb65e222c1d1c`
- Portfolios: root-only, balanced 4/16/64, and 13-leaf skew to depth 12
- Hostile rejections: `19 / 19`
- Maximum frontier/directives: `64 / 63`
- Production artifacts: `none`

The typed hostile matrix covers schema, provenance alias/zero, revision, band,
root measure, decimal aliases/reduction/overflow, unknown fields, invalid axis,
foreign or retired parent, resolution, negative/mismatched energy, forbidden
child fields, duplicate/reordered directives, the 64th directive and oversize
input. Axis substitution changes distribution identity.

Implementation tests must additionally cover duplicate raw JSON keys, invalid
UTF-8, trailing bytes, exact cap and cap-plus-one bytes, pre-allocation vector
caps, forged complete result fields, maximum 385-bit success and 386-bit stop,
upstream codec substitution, same side sequence with different axes, arbitrary
legal partial-refinement order, and measured i686 live memory. The Python model
does not claim to replace those raw-codec or platform tests.

## Strict codecs and typed errors

Canonical JSON is UTF-8 with lexicographically sorted object keys, compact
separators, decimal JSON integers for fixed-width scalar integers and JSON byte
arrays for `[u8; 32]`. Decode denies unknown and duplicate fields, aliases,
invalid UTF-8 and trailing content, then validates/replays, re-encodes and
requires exact byte equality. Collection counts and byte multiplication are
checked before allocation and before conversion to platform `usize`.

Public errors remain non-semantic: invalid schema/subject/provenance,
provenance conflation, upstream replay defect, noncanonical energy, identity
mismatch, non-frontier parent, invalid resolution, depth/frontier/resource
ceiling, energy conservation mismatch, arithmetic shield/defect and codec
defect. No error means darkness, vacuum, zero source, zero coupling, transport
failure, visibility or authority.

## Dependency and platform gates

If approved, the candidate may depend only on exact pinned workspace versions
of:

- `calibrated-spectral-time-basis` for complete basis replay;
- `optical-phase-space-cell-binding` for exact root and split replay;
- `fixed-interval-arithmetic` for checked `Signed512` energy proofs; and
- `serde`, `serde_json` and `sha2` for strict codecs and identities.

No existing owner imports the candidate or changes source/API/bytes/identity.
The candidate must not import dimensionless transfer, bulk transfer, Forge
Kernel, Tauri/UI, filesystem, clock, locale, network, process, randomness,
float or native-endian encoding.

An approved implementation must pass warnings-denied native all-target tests,
executable `i686-pc-windows-msvc` tests, `aarch64-linux-android` compilation,
rustfmt, capability and modularity scans, byte-identical fixtures, exact
upstream cross-checks, identity/hostile/resource gates, module/context
governance and the complete Forge gate. No platform semantic fork is allowed.

## Rollback and consumer boundary

The implementation would be the first production consumer of the calibrated
basis and would have zero downstream consumers. No current bytes, IDs, data or
owner source migrate. Any replay, identity, codec, ceiling, dependency,
platform or full-gate failure deletes only the new crate, contract, tests,
fixtures, verifier, result and governance references. Existing owner suites
must pass after the deletion drill.

If an upstream API must change, deletion-only rollback no longer holds and the
package returns for a new serious owner decision. After a verified zero-
downstream-consumer implementation, perform a C3 closure reassessment before
selecting any received-energy or other consumer. Nothing broader is locked in.

## Transport and perception remain blocked

This candidate cannot provide physical spatial-coordinate or coefficient
calibration, validity provenance, pointwise wavelength/time transport proof,
received radiant energy, detector response, brightness, darkness, visibility,
rendering or runtime behavior. Equal transfer is not equal energy; equal energy
is not equal distribution; source magnitude is not dimensionless transfer.

## Complete Forge verification

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `296.8 seconds`
- Output lines: `2,349`
- Durable files classified: `819`
- Declared modules verified: `51`

## Exact owner decision

Approve or reject creation of one capability-free canonical owner tentatively
named `calibrated-source-energy-distribution`, with the exact compact replay,
records, identities, limits, hostile fixtures, dependencies, platform gates,
zero downstream consumers and deletion-only rollback above.

Approval authorizes only test-first creation of that bounded owner and
contract. It does not authorize normative source data or allocation, a
downstream consumer, transport applicability, received energy, detector or
visibility behavior, runtime integration, promotion or C3 closure. Rejection
leaves this readiness record and disposable oracle as evidence and creates no
production module.

General continuation is not enough for this source action. Stop here for the
explicit owner decision.
