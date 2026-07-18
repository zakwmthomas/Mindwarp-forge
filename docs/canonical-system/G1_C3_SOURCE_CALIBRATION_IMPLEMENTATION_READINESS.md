# G1 / C3 source-calibration implementation readiness

Date: 2026-07-17

Status: **ready for one explicit owner decision only. No crate, contract,
schema, production source, dependency, registry or consumer has been added.**

## Bounded candidate

The only surviving owner candidate is one capability-free sibling tentatively
named `calibrated-spectral-time-basis`. It would own a strict physical
spectral/time basis descriptor, its deterministic identity and a derived
commitment into the unchanged legacy RGB/time identity space.

It would not own source-energy allocation, transport coefficients, spatial
calibration, transport applicability, received energy, detector response,
visibility, rendering, runtime, promotion or C3 closure. The crate name remains
part of the owner decision, not an existing module claim.

## Frozen input record

`CalibratedSpectralTimeBasisInputV1` has `deny_unknown_fields` and exactly:

- `schema_version: u32`, exactly `1`;
- `basis_version: u32`, positive;
- `calibration_provenance_id: [u8; 32]`, nonzero;
- `quantity_kind`, exactly `radiant_energy`;
- `unit`, exactly `joule`;
- `spectral_coordinate`, exactly `vacuum_wavelength_metre`;
- `spectral_weighting`, exactly `unit_energy_integral`;
- `spectral_intervals`, exactly three records in `blue`, `green`, `red`
  order, each with `band`, `lower` and `upper`; and
- `time_cell`, exactly `clock_origin_id: [u8; 32]`, nonzero,
  `start_tick: u64`, `end_tick: u64`, and positive `seconds_per_tick`.

Every interval is half-open, nonempty and contiguous with its neighbour. The
four wavelength boundaries and `seconds_per_tick` use
`ExactUnsignedRationalV1 { numerator: String, denominator: String }`.
Components are canonical unsigned base-10 `u128` values: ASCII digits only,
at most 39 digits, no sign or leading zero, denominator nonzero, fraction
reduced, and numerator positive only where the field requires it. Cross
multiplication uses checked 256-bit intermediates or an equivalent checked
comparison; float conversion is forbidden.

The fixture boundaries and tick scale are hostile-test data, not normative RGB
science or runtime defaults. The implementation must ship no implicit epoch,
boundary, duration or provenance.

## Frozen result record

`CalibratedSpectralTimeBasisV1` has `deny_unknown_fields` and exactly:

- `schema_version: u32`, exactly `1`;
- the validated canonical `input`;
- `calibrated_basis_id: [u8; 32]`;
- `derived_legacy_time_basis_id: [u8; 32]`;
- exactly three `derived_legacy_band_time_ids`, keyed blue/green/red;
- `authority_effect`, exactly `none_evidence_only`; and
- the exact limitations string pinned by fixtures.

Derived identities are outputs, never trusted input. If a replay API accepts
caller-supplied derived identities, it reconstructs and compares all of them
and rejects any mismatch before returning evidence.

## Non-circular identity graph

Canonical JSON uses UTF-8, lexicographically sorted object keys, compact
separators, decimal JSON integers for fixed-width integer fields and JSON byte
arrays for `[u8; 32]`. Decode must reject unknown/duplicate fields and aliases,
then re-encode and compare exact canonical bytes.

1. `calibrated_basis_id = SHA-256("mindwarp.calibrated-spectral-time-basis.basis.v1" || NUL || canonical_input)`.
2. `derived_legacy_time_basis_id = SHA-256("mindwarp.calibrated-spectral-time-basis.legacy-time-commitment.v1" || NUL || canonical_json([calibrated_basis_id bytes]))`.
3. For each ordered RGB band, the unchanged V1 compiler derives
   `band_time_id = SHA-256("mindwarp.optical-phase-space.band-time.v1" || NUL || canonical_json([band, time_basis_id bytes]))`.

No derived field enters the input hash, so the graph has no cycle. The legacy
time ID commits transitively to the entire spectral partition, exact time cell,
calibration provenance and basis version. One accepted legacy pair therefore
cannot alias two accepted physical meanings without a SHA-256 collision.
Existing arbitrary V1 time IDs remain valid identity-only evidence and are
permanently uncalibrated; no side table, upgrade or global registry exists.

## Frozen resource ceilings

- exactly three bands, four wavelength boundaries and one time cell;
- `u128` rational components with the 39-digit textual ceiling;
- canonical input at most 16 KiB;
- canonical result at most 32 KiB; and
- input plus result plus transient canonical buffers at most 64 KiB live.

The deterministic readiness fixture measures 896 input bytes and 1,786 result
bytes. The ceilings leave codec evolution space while remaining far below the
unrelated 128 MiB document boundary. No user-controlled collection cardinality
or unbounded integer allocation is allowed.

## Code-free falsification receipt

`tools/prove-g1-c3-source-calibration-readiness.py` models the frozen records,
strict rationals, byte ceilings and exact identity graph without creating a
production owner. Two runs were byte-identical.

- Oracle source SHA-256:
  `d8f1fd99fffea927c62642ecde46f4380ebb949ce279d0d940758a5ca31e5d22`
- Receipt checksum:
  `111245fd46c4b5639f5e63d1b3c6ea187c8dbed01bf786bb241157a64d576c3c`
- Hostile rejections: `30 / 30`
- Identity substitutions: `6 / 6` changed both basis and legacy-time IDs
- Legacy compatibility: exactly three unchanged V1 band/time derivations
- Production artifacts: `none`

The hostile matrix covers schema/basis versions, wrong quantity/unit/
coordinate/weighting, zero or malformed identities, sign and leading-zero
aliases, zero denominator, non-reduced rational, `u128` overflow, unknown
rational/interval/time/input fields, empty/reversed/gapped/overlapping or
reordered intervals, wrong interval count, invalid clock/ticks/scale, input
oversize and caller-supplied identity substitution.

Implementation tests must add duplicate-key raw JSON, invalid UTF-8, trailing
bytes, result oversize, aggregate-live-byte accounting and reconstruct/compare
codec fixtures because the typed Python model cannot manufacture every raw
Serde parser condition.

## Dependency and platform gates

The candidate production crate may depend only on exact pinned workspace
versions of `serde`, `serde_json` and `sha2`. It must not depend on the legacy
dimensionless-transfer crate, Forge Kernel, Tauri/UI, filesystem, clock,
locale, network, process, randomness, float or native-endian limb encoding.
The three legacy hashes are reproduced under the frozen V1 algorithm and
cross-checked against that owner in tests; reversing the production dependency
would make calibration import the optical transport graph.

An approved implementation must pass warnings-denied native tests, executable
`i686-pc-windows-msvc` tests, `aarch64-linux-android` compilation, rustfmt and
source-capability scans, byte-identical fixtures, cross-checks against all
three unchanged V1 band/time identities, module/context governance and the
complete Forge gate. PlayStation remains a portable-contract concern; Xbox,
Mac and Linux are lower-ROI unless cheap. No platform semantic fork is allowed.

## Rollback and first-consumer boundary

The first implementation has zero consumers. No current crate imports the
candidate and no existing bytes, IDs or data migrate. Any identity, codec,
ceiling, dependency, platform or full-gate failure causes deletion of the new
crate, contract, tests, fixtures, verifier and governance entries only.

After a verified zero-consumer implementation, select one consumer separately
and reassess before expanding. Nothing broader is locked in.

## Transport applicability remains blocked

This package does not design or authorize transport applicability. That owner
still lacks physical spatial-coordinate and coefficient calibration, validity
provenance and a conservative pointwise enclosure for every wavelength and
instant in a whole cell. A source basis cannot manufacture those facts.

## Complete Forge verification

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `396.5 seconds`
- Output lines: `2,211`
- Durable files classified: `802`
- Declared modules verified: `50`

## Exact owner decision

Approve or reject creation of one capability-free canonical owner tentatively
named `calibrated-spectral-time-basis`, with the exact input/result records,
domains, limits, hostile fixtures, dependencies, platform gates, zero
consumers and deletion-only rollback above.

Approval authorizes only a test-first implementation of that small owner and
contract. It does not authorize a gameplay consumer, normative calibration,
transport applicability, detector/visibility behavior, runtime integration,
promotion or C3 closure. Rejection leaves this document and code-free oracle
as design evidence and creates no production module.

General continuation is not enough for this source action. Stop here for the
explicit owner decision.
