# G1 / C3 source-quantity basis and schema gap audit

Date: 2026-07-17

Status: **gap confirmed; no current owner defines physical source quantity,
spectral/temporal calibration or a zero-safe source schema. Route to a separate
mathematical design, not implementation readiness.**

## Question

Can the oracle-proven abstract additive source-quantity measure be expressed
by reusing current Forge semantics and exact types, without relabelling
normalized evidence, mutating an existing V1 owner or inventing physical
calibration?

The answer is no. Some current representations are mechanically suggestive,
but none owns the required physical meaning and no combination closes the gap.

## Semantic-owner inventory

| Candidate | Exact evidence | Disposition |
|---|---|---|
| `stellar-orbital` irradiation | `irradiance_*_millionths_earth` is an inverse-square dimensionless ratio at an orbital receiving distance; `bounded_stellar_irradiance_rgb_permille` is explicitly saturated for a synthetic palette seam | Do not reuse as emitted source-cell quantity |
| `derived-world-rules::SignalPotential` | `baseline_strength_permille` is caller-declared normalized potential; its contract explicitly says no physical unit, propagation, distance attenuation or detectability claim | Do not reuse as physical quantity |
| Surface/gas/aerosol RGB permille | Dimensionless reflectance, transmission or palette modifiers | Transfer factors are not source magnitude |
| `OpticalBandTimeBindingV1` | RGB enum plus nonzero opaque `time_basis_id`, hashed into `band_time_id` | Identity correlation only; no wavelength interval, integration weighting, duration or time interval |
| `PositiveRationalV1` | Reduced exact positive abstract cell measure with bounded decimal magnitude | Its validator rejects zero and its owner explicitly disclaims physical correctness and emission |
| Receiver coupling measure fields | Reuse the nominal `PositiveRationalV1` shape but locally construct canonical `0/1` buckets and replay the complete result | Classification evidence only; the type name and local zero behavior demonstrate why structural shape is not semantic reuse authority |
| `ExactMeasureV1` in dimensionless transfer | Copies already validated accepted/zero/unresolved coupling measures | Projection DTO, not an independently validated source numeric owner |
| Q64.64/Q0.48 optical values | Optical depth and dimensionless transmission enclosures with local precision policy | Wrong quantity and scale; cannot hold arbitrary physical source magnitude |

No repository declaration of `quantity_basis_id`, emitted radiant energy,
emitted radiant power, photon quantity, wavelength boundaries, physical
duration or calibrated source-cell allocation exists.

## Why the stellar route is not the missing owner

The stellar contract has the strongest physical ancestry, but its scalar is a
ratio to Earth flux at an orbital distance. It is receiver-plane irradiation
evidence, not total stellar emitted power and not an allocation over the exact
four-symbol source phase-space cell algebra. Converting it to SI irradiance
would require an explicit reference constant and version. Converting that
irradiance into source-cell quantity would additionally require a physical
source surface, angular/spectral emission model, band integration, time scope
and a correlation-preserving allocation rule.

The saturated RGB field is still less suitable: saturation deliberately loses
magnitude and its contract names the downstream synthetic palette seam. It
cannot be unsaturated later or treated as source power.

## Numeric and schema reuse decision

Semantic reuse and implementation reuse are separate decisions. The existing
big signed arithmetic may later be a useful implementation substrate, but no
current public rational wrapper is a correct source quantity type:

- `PositiveRationalV1` cannot express zero, and changing its validation would
  mutate the abstract-cell V1 identity and failure boundary;
- receiver-coupling zero buckets are valid only because that owner reconstructs
  the entire classified result; they do not broaden the cell type's contract;
- `ExactMeasureV1` has no independent constructor, reduction rule, magnitude
  shield or source semantics; and
- fixed optical formats have quantity-specific local scales and insufficient
  range evidence for an unspecified source basis.

A later schema should therefore own a distinct zero-safe exact quantity type,
even if it internally reuses semantic-neutral arithmetic. Canonical zero must
be `0/1`; positive values must be reduced; negative, signed-zero, leading-zero,
zero-denominator, non-reduced and over-cap forms must fail closed. Exact bit
and byte ceilings require proof after the physical basis is selected.

## Missing physical design decisions

The following are mathematical semantics, not codec details, and block
readiness:

1. quantity kind: band/time-integrated radiant energy, radiant power, or an
   explicitly normalized non-SI quantity;
2. unit and scale, including reference-constant provenance for any normalized
   conversion;
3. exact spectral band boundaries, integration weighting and treatment of
   emission outside the three selected bands;
4. exact temporal interval or duration semantics and steady-state assumptions;
5. physical source scope and emission model that allocates quantity to the
   abstract phase-space root without pretending its measure is square metres
   or steradians;
6. conservation law for atomic parent-to-two-child quantity splits;
7. the meaning of zero, unresolved source allocation and unavailable source
   evidence; and
8. exact transfer-product and aggregation range bounds before fixed-width
   projection.

Radiance remains an invalid candidate until projected physical area, solid
angle and the relevant Jacobian are owned. A normalized non-SI quantity could
be useful for a synthetic system, but it would not close C3's physical
visible-radiance gap and must not be described as watts, joules or radiance.

## Code-facing obligations after mathematical selection

Only after a physical basis survives a separate design and oracle may a
readiness audit consider a new additive sibling owner. That later package must
freeze:

- exact source, scope, reconstruction, revision, root, cell ancestry,
  `band_time_id` and `quantity_basis_id` bindings;
- a basis record whose identity covers quantity kind, unit/scale, spectral
  definition, temporal definition, calibration provenance and version;
- atomic split receipts that produce both children and prove exact sum equality;
- strict canonical codecs with unknown-field/trailing-content rejection;
- root, per-record, split, depth, batch, bit, byte and live-memory ceilings;
- hostile substitution, deletion, duplication, reorder, stale-basis,
  noncanonical-number, zero/unresolved and projection-underflow fixtures;
- native x64, executable i686 and Android ARM64 compile gates; and
- deletion-only rollback with no mutation or migration of current optical V1
  owners.

The existing cell depth cap is 12, so a fully materialized binary frontier can
reach 4,096 cells. That is only a planning upper bound: a later design must
decide whether quantities are streamed, frontier-bounded or materialized and
measure actual costs before readiness.

## Decision

No current owner or type is physically and semantically reusable as the
source-quantity basis. Advance to one code-free **source-quantity-basis
mathematical design audit** comparing:

1. band/time-integrated radiant energy;
2. radiant power with explicit steady-duration semantics;
3. explicitly normalized non-SI source quantity; and
4. radiance density only as a falsification control while physical phase-space
   calibration remains absent.

The design must use primary radiometric definitions, state how each candidate
composes with dimensionless transfer, preserve exact additive subdivision, and
run a deterministic rational counterexample oracle before schema readiness.

Add no crate, dependency, contract schema, production test or production
source. Select no unit by implication. Do not claim watts, joules, radiance,
received power, detector response, visibility, runtime, promotion or C3
closure.

Nothing broader is locked in. One consumer first, reassess before expanding.

## Verification receipt

The permanent focused verifier and complete Forge gate pass. The complete gate
ran for 390.2 seconds across 2,208 output lines, classified 789 durable files
and retained all 50 current module front doors. No production module or module
context changed in this audit.
