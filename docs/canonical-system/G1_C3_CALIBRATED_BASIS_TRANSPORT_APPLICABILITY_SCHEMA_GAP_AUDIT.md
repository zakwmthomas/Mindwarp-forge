# G1 / C3 calibrated-basis and transport-applicability schema gap audit

Date: 2026-07-17

Status: **no current owner or schema can supply either physical calibration
or transport applicability. Source calibration may advance independently to
one bounded implementation-readiness audit; transport applicability remains
blocked. No crate or schema is authorized.**

## Question

The calibrated spectral/time oracle proved that a separate versioned witness
can bind additive radiant energy to unchanged legacy RGB/time identities. This
audit asks where that witness could live, how legacy-pair uniqueness can be
enforced, and whether physical transport applicability can share the same
owner.

This is a read-only code-facing audit. It changes no module ownership, V1
bytes, schema, source or runtime behavior.

## Current-owner inventory

| Owner | Existing fact | Missing fact / decision |
|---|---|---|
| `optical-phase-space-dimensionless-transfer` | `OpticalBandTimeBindingV1` stores `band`, nonzero opaque `time_basis_id` and derived `band_time_id`; canonical JSON and SHA-256 replay are strict; binding cap is 4 KiB | no wavelength interval, weighting, clock origin, duration, joule quantity, calibration provenance, source magnitude, spatial calibration or pointwise spectral/time applicability |
| `visible-radiance-bulk-transfer` | owns three Q16.48 extinction coefficients per abstract volume coordinate unit and exact dimensionless optical-depth/transfer evaluation | explicitly no coefficient catalogue, SI claim, metre mapping, real-world coefficient validity, source quantity or time basis |
| `physical-path-substrate` | owns exact channel-neutral coordinate geometry, path and conditional cell-step evidence | no optical band, SI spatial scale, coefficient calibration, source energy or transport applicability |
| calibrated source basis | no crate, contract, module boundary or canonical codec exists | needs exact physical descriptor, identity, strict codec, legacy commitment and bounded arithmetic representation |
| transport applicability | no owner or record exists | needs calibrated spectral/time basis, exact transport profile/revision, spatial calibration and proof that one enclosure is valid everywhere in the whole cell |

Repository search found no production `wavelength`, `seconds_per_tick`,
`calibration_provenance`, `spatial_calibration`, `transport_applicability` or
joule-valued source schema. Documentation and disposable oracles are the only
current holders of those concepts.

## Ownership candidates

### Mutate `OpticalBandTimeBindingV1` - reject

Adding physical fields changes frozen V1 bytes and identity. That crate owns
dimensionless transfer and expressly excludes source magnitude and energy
transport. Historical opaque IDs would be retroactively reinterpreted.

### Mutate `visible-radiance-bulk-transfer` - reject

This would turn abstract per-coordinate coefficients into physical spectral
calibration without an SI spatial basis. It also conflates material transport
evidence with source energy ownership and prevents deletion-only rollback.

### Put calibration in `physical-path-substrate` - reject

The substrate is channel-neutral. Importing optical bands, wavelength and
source quantity would reverse its dependency direction and make geometry own
unrelated radiometry.

### One combined calibration/applicability sibling - reject

Source-subject calibration is independently coherent. Transport applicability
is not: it still lacks spatial calibration, coefficient provenance and a
whole-cell pointwise proof. Combining them would block the ready half on the
unready half and give one record authority over source and transport.

### Separate source-calibration sibling - select for readiness audit only

A capability-free sibling tentatively named
`calibrated-spectral-time-basis` is the only clean candidate. It would own the
physical basis descriptor, deterministic identity, exact legacy commitment
and strict bounded codecs. It would not own source allocation, transport,
coefficients, spatial calibration, detector response or visibility.

This is a schema candidate, not an implementation authorization or final
crate-name decision.

## Uniqueness without a registry

The mathematical oracle used an abstract uniqueness registry to reject two
physical meanings for one legacy pair. A persisted registry is the wrong code
boundary here: it adds conflict resolution, storage, migration and cleanup,
and rollback would no longer be deletion-only.

The readiness candidate must instead use a stateless derived commitment:

1. canonicalize the complete physical descriptor, excluding all derived
   identities;
2. derive `calibrated_basis_id` under a new domain separator;
3. derive the legacy `time_basis_id` from the complete calibrated basis and
   exact time cell under a second domain separator;
4. call the unchanged V1 band/time compiler for each RGB band; and
5. reject caller-supplied derived IDs or any mismatch on replay.

Because the derived time identity commits to the full spectral partition,
time coordinate, provenance and version, one unchanged `(band,time_basis_id)`
pair cannot name two accepted physical meanings. Existing arbitrary V1 time
IDs remain valid identity-only evidence but are permanently uncalibrated; they
cannot be upgraded by attaching a side table.

The implementation-readiness audit must prove this exact non-circular identity
graph before an owner gate. It must not introduce a global registry.

## Minimum source-calibration schema obligations

The later readiness audit must freeze, falsify and cap at least:

- schema version, quantity kind `radiant_energy` and unit `joule`;
- vacuum-wavelength coordinate and unit-energy integration weighting;
- exactly three ordered contiguous nonempty half-open RGB intervals;
- reduced nonnegative rational metre endpoints with explicit bit/digit caps;
- nonzero clock-origin identity, integer half-open ticks and reduced positive
  rational seconds per tick;
- nonzero calibration provenance identity and positive basis version;
- derived calibrated-basis, time-basis and three legacy band/time identities;
- strict `deny_unknown_fields`, reconstruct-and-compare canonical JSON;
- typed schema, rational, ordering, identity, codec and byte-ceiling failures;
  and
- explicit limitations and `none_evidence_only` authority.

No current 4 KiB, 64 KiB or 128 MiB ceiling automatically applies. The new
owner needs its own small input/result ceilings, rational magnitude/digit
limits and aggregate-live-byte bound. Exactly three bands and one time cell
per record are natural structural caps, but numeric byte ceilings remain for
the readiness audit to measure and freeze.

## Platform boundary

The candidate can remain capability-free and portable by using canonical
decimal integers/rationals, checked parsing, SHA-256 and canonical JSON only.
No float, native-endian limb encoding, filesystem, clock, locale or network
dependency is admissible.

Readiness must require native tests, `i686-pc-windows-msvc` compilation/tests,
`aarch64-linux-android` compilation, source-format checks and byte-identical
fixture replay. PlayStation remains a portable-contract concern; Xbox/Mac and
Linux remain lower-ROI unless cheap, consistent with the program platform
policy.

## Transport-applicability gap

An applicability receipt cannot advance with the source sibling. It still
requires:

- exact calibrated-basis identity;
- exact bulk/whole-cell transport profile and revision identities;
- physical spatial-coordinate and coefficient calibration;
- a conservative pointwise enclosure for every wavelength and instant in the
  calibrated cell, not a midpoint, sample or average;
- validity interval and provenance for the coefficient evidence; and
- strict treatment of unavailable, opaque, zero, underflow and unresolved
  cases.

No current owner supplies the spatial calibration or the pointwise theorem.
Creating only a record shape would manufacture authority with no evidence.
Transport applicability therefore remains at mathematical/schema gap status
and cannot enter implementation readiness.

## Rollback and dependency boundary

The source-calibration candidate may depend only on serialization, hashing and
possibly a read-only call to the existing band/time compiler. Existing crates
must not import it during the first bounded implementation. This keeps the
first consumer at zero and makes rollback deletion-only: remove the new
sibling, contract, tests, verifier and registry references; no V1 data or
identity migrates.

After a verified implementation, one consumer must be chosen separately and
reassessed before expansion. Transport applicability is not that first
consumer unless its independent gaps close.

## Decision and next action

Advance only the source-calibration sibling to one **code-facing
implementation-readiness audit**. That audit must freeze the stateless identity
graph, exact record shapes, rational and byte ceilings, dependency boundary,
hostile fixtures, platform gates and deletion-only rollback, then stop at an
explicit owner decision.

Do not implement now. Add no crate, dependency, contract schema, production
test, production source, registry or consumer. Do not choose normative RGB
boundaries, tick duration, epoch, source spectrum, spatial scale or coefficient
catalogue. Do not claim received energy, detector response, visibility,
runtime, promotion or C3 closure.

Nothing broader is locked in. One consumer first, reassess before expanding.

## Complete Forge verification

- Command: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/verify.ps1`
- Exit code: `0`
- Wall time: `392.9 seconds`
- Output lines: `2,211`
- Durable files classified: `799`
- Declared modules verified: `50`
