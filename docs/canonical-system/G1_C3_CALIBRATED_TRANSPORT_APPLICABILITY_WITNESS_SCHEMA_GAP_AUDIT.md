# G1 / C3 calibrated transport-applicability witness schema gap audit

Date: 2026-07-18

Status: **schema gap confirmed. Existing V1 owners remain unchanged. A future
separate capability-free applicability sibling is the only coherent ownership
location, but it cannot advance to mathematical design or implementation
readiness until spatial-scale evidence and a conservative coefficient-validity
theorem exist. No schema or implementation is authorized.**

## Delta from the earlier audit

The earlier calibrated-basis audit established that source calibration and
transport applicability must not share one owner. The calibrated spectral/time
basis and calibrated source-energy distribution now exist and are verified.
This audit therefore does not repeat the source-calibration decision. It asks
whether the new exact source subject can be joined to existing path and
dimensionless-transfer evidence as a physical transport claim.

This is a read-only code-facing audit. It changes no crate, contract schema,
dependency, production test, production source, module boundary, V1 bytes,
identity or downstream consumer.

## Current identity inventory

| Evidence owner | Identity already committed | Missing relation |
|---|---|---|
| `calibrated-source-energy-distribution` | source ID, source scope and revision, calibrated-basis ID, selected band and derived band/time ID, optical root and reconstruction, exact cell IDs and joules | no spatial scale, coefficient evidence, path applicability or received-energy authority |
| `optical-phase-space-cell-binding` | source, scope, reconstruction, revision, root, axis-bearing path, exact measure and cell ID | no physical length, coefficient profile or wavelength/time validity theorem |
| `physical-path-substrate` | Cartesian Q32.32 volume recipe, abstract origin/cell step, scope, reconstruction and revision | no SI or metre mapping and no optical subject |
| `visible-radiance-bulk-transfer` | profile source/scope/reconstruction/revision, embedded physical recipe, three Q16.48 extinction coefficients per coordinate unit and profile ID | no calibrated-basis/band-time commitment, coefficient provenance or real-world validity |
| optical transport and certificate | exact optical cell, physical recipe/volume, current cell, band/time ID and certificate identities | no proof that the profile coefficients describe that calibrated physical subject |
| whole-cell dimensionless transfer | complete bulk profile, coupling/transport evidence, selected band/time binding and conservative dimensionless outcome | no source magnitude, physical coefficient calibration or received-energy claim |

The current joins are partial. Source distribution commits the calibrated basis
and optical cell. Transport validation binds cell scope and reconstruction to
the physical recipe, while dimensionless transfer binds band/time to transport
and certificate and binds the bulk physical-volume identity to transport. Bulk
profile validation requires its top-level reconstruction ID to equal the
embedded recipe reconstruction ID, but it does not require its top-level scope
ID to equal that recipe scope. None of these joins supplies a physical length
scale or binds the bulk coefficients to the calibrated spectral/time basis.

## Exact missing relations

### Physical spatial unit

A future witness would need an exact positive metres-per-coordinate-unit value,
or an equivalent exact affine physical-length calibration, bound to the exact
coordinate frame and physical recipe. It must carry nonzero provenance,
positive revision and an explicit validity domain. Existing Q32.32 and Q160
bytes remain unchanged and abstract; the witness may attest their physical
interpretation but may not reinterpret their stored identity.

### Coefficient and spectral/time applicability

The witness would need the exact bulk profile and revision, coefficient-evidence
provenance and validity, exact calibrated-basis ID, selected band and derived
band/time ID. More importantly, it needs a conservative theorem or enclosure
valid for every wavelength and instant in the calibrated cell. An RGB label,
midpoint, sample, average or shared opaque ID cannot establish pointwise
validity.

### Exact subject and path

The source distribution subject and distribution identity, selected allocation
cell, optical transport cell, certificate, physical recipe and volume, bulk
profile, selected path/step and dimensionless-transfer result must replay as one
exact graph. Scope or reconstruction equality alone is insufficient. The
applicability result must be downstream of all these owners and cannot make the
source allocation receiver-specific.

Same cell identity is necessary but insufficient. A source allocation carries
aggregate joules and exact cell measure but proves no within-cell energy uniformity.
Dimensionless transfer separately retains accepted, zero and
unresolved coupling measures, and no current owner allocates source joules
across those buckets. Direct whole-cell joules times accepted transfer therefore
requires either a conservative transfer bound valid everywhere in the complete
source cell or a separate joint source/coupling integration proof.

Likewise, one stored Q16.48 scalar for an RGB band is not a physical coefficient enclosure.
Applicability may attest that scalar only if evidence proves it valid
throughout the complete calibrated wavelength, time and spatial domain;
otherwise new coefficient-enclosure mathematics is required.

### Applicability disposition

The minimum meaningful disposition is either `certified_everywhere` for the
declared whole optical cell and path domain or `conservatively_unresolved`.
Unavailable evidence, opaque transfer, exact zero, finite transfer with a zero
projected lower endpoint, and unresolved applicability remain semantically
distinct. Underflow remains a finite enclosure and must not become opacity.
Unresolved applicability blocks any physical received-energy claim even when a
numerical joule-times-transfer product can be formed.

### Correction and validity

Records remain immutable. A correction carries new nonzero provenance or a
higher positive revision and derives a new applicability subject and result
identity committing every exact upstream identity and validity boundary. It
does not rewrite old evidence, select an active version, attach meaning to an
old opaque ID or require a mutable supersession registry.

## Falsifying counterexamples

### A. Scale ambiguity

Interpret one unchanged recipe, profile, source distribution and transfer once
with one coordinate unit equal to one metre and once with one coordinate unit
equal to ten metres. Current bytes and dimensionless output are identical. A
fixed SI extinction law would require a corresponding coefficient conversion,
so the same declared per-coordinate coefficient and received-energy product
cannot be physically valid under both interpretations. Current evidence cannot
choose between them.

### B. Spectral/time ambiguity

Reuse one accepted RGB bulk profile with two valid calibrated bases whose
wavelength intervals or time cells differ. Each transport can carry its own
internally matching band/time ID, yet no current bulk-profile field or validator
binds its coefficients to either calibrated basis. Both compositions can pass
their local identity checks while asserting incompatible physical meanings.

### C. Subject mismatch

Construct a bulk profile whose top-level scope differs from the scope inside its
embedded physical recipe while retaining the required reconstruction equality.
Current bulk validation intentionally accepts that relation, and dimensionless
transfer can still join through the physical-volume identity. This is not a present validation defect;
it proves only that nearby opaque scope,
reconstruction or volume identities are insufficient physical-applicability
proof for the transported source subject.

## Ownership comparison

### Mutate `physical-path-substrate` - reject

The substrate is channel-neutral and owns abstract geometry. Optical wavelength,
coefficient provenance and source identity would reverse its dependency
direction and change its established non-goals.

### Mutate `visible-radiance-bulk-transfer` - reject

The owner intentionally evaluates declared coefficients per coordinate unit. SI
calibration and source-subject applicability would change V1 meaning and couple
material evidence to one source claim.

### Mutate whole-cell dimensionless transfer - reject

Dimensionless transfer is reusable source-independent attenuation evidence.
Adding joules, spatial calibration or source identity would mix source and path
truth and change frozen V1 dependencies.

### Mutate calibrated basis or source distribution - reject

Spatial/path coefficient validity is downstream of source calibration and
allocation. Importing it would make immutable source evidence transport- or
receiver-specific and create correction and identity pressure.

### Add an adapter over only opaque IDs - reject

An adapter can translate shape but cannot manufacture physical scale,
coefficient provenance or a pointwise theorem. Equality of opaque IDs is not
evidence of scientific applicability.

### Future separate applicability sibling - only coherent boundary

A future capability-free sibling may replay the exact existing owners and
attest the missing bridge. It may not discover coefficients, choose normative
calibration, own source allocation, multiply received energy, or grant detector,
visibility, runtime, promotion or C3 authority. It remains at schema-gap status:
the repository currently contains neither the required spatial calibration
evidence nor the conservative coefficient-validity theorem.

## Minimum future witness obligations

Any later evidence-acquisition or mathematical-design audit must establish:

- exact physical recipe/frame and positive physical-length mapping;
- exact calibrated basis, band and band/time cell;
- exact bulk profile/revision and coefficient provenance/validity;
- exact source distribution subject/distribution and allocation cell;
- exact transport cell, certificate, physical volume, selected path/step and
  dimensionless-transfer result;
- a declared whole-cell/path domain with pointwise-everywhere proof or a typed
  unresolved result;
- immutable correction-by-new-identity with bounded canonical codecs and
  non-circular identities; and
- no capability, received-energy, detector, visibility or runtime authority.

A record shape without the physical evidence and theorem would manufacture
authority and is therefore inadmissible.

## Decision, rollback and stop condition

The gap is real and remains upstream of received energy. Existing owners, V1
bytes, identities and zero-downstream-consumer status remain unchanged. This
audit is rollback-safe by deleting only this record, its static verifier and
route references.

Stop before schema or mathematical implementation readiness. Any continuation
must be a separately owner-gated evidence-acquisition or mathematical-design
audit for the future sibling. Stop immediately if it would invent a spatial
scale or coefficient fact, mutate an existing owner, collapse typed outcomes,
or grant received-energy, detector, visibility, runtime, promotion or C3
closure authority.

## Verification receipt

- Static applicability-gap, checkpoint, master-program, bootstrap, record-role,
  modularity, module-context and affected historical route shields pass.
- Complete Forge command: `tools/verify.ps1`
- Exit code: `0`
- Wall time: `301.0 seconds`
- Output lines: `2,386`
- Durable files classified: `832`
- Declared modules verified: `52`

Nothing broader is locked in. One consumer first, reassess before expanding.
