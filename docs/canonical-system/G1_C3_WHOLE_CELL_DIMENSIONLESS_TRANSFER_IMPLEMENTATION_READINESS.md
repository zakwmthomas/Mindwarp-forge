# G1 / C3 whole-cell dimensionless-transfer implementation readiness

Date: 2026-07-17

Status: **ready for one exact serious owner implementation decision: add one
read-only bulk-owned optical-depth evaluation receipt and one downstream
`optical-phase-space-dimensionless-transfer` sibling. No source is authorized
by this record.**

## Adopt / adapt / build decision

| Route | Decision |
|---|---|
| Import the exact-lane cumulative factor | Reject: it describes one exact lineage, not the complete correlated cell. |
| Multiply public Q0.48 step factors as the primary representation | Reject as primary: conservative but avoidably loses precision and can project a positive lower product to zero. Retain only as a hostile comparator/fallback. |
| Copy the bulk exponential kernel into a new consumer | Reject: duplicates a verified numerical owner and creates drift. |
| Extract a shared exponential crate now | Reject for V1: forces a broader migration before one consumer proves value. Reassess only after a second real consumer. |
| Add a bulk-owned read-only optical-depth evaluation receipt, then consume it from one new sibling | **Select.** It preserves kernel ownership, keeps existing V1 bytes/results unchanged and has deletion-only rollback. |

Nothing broader is locked in. Implement one consumer first and reassess before
expanding the numerical surface.

## Additive bulk-owned surface

`visible-radiance-bulk-transfer` may add only:

- `BulkOpticalDepthEvaluationInputV1`, containing schema version and ordered
  nonnegative Q64.64 lower/upper optical-depth endpoints;
- `BulkOpticalDepthEvaluationV1`, containing the replayed input ID, unchanged
  endpoints, one outward Q0.48 transfer enclosure, arithmetic receipt,
  limitations, `none_evidence_only` authority and result ID;
- compile, validate and strict reconstruct-and-compare codecs; and
- a public wrapper over the existing private `exp_neg_q0_64_bounds` kernel.

The wrapper must call the unchanged kernel. It may not copy, generalize or
retune the range reduction, series, rounding or output projection. Existing
profile, exact-path, conditional-interval, bulk V1 bytes, identities, errors
and results remain byte-for-byte unchanged.

The new identity domains are:

- `mindwarp.visible-radiance.bulk-optical-depth-evaluation.input.v1`; and
- `mindwarp.visible-radiance.bulk-optical-depth-evaluation.result.v1`.

Input and result are each capped at 4 KiB. Endpoints remain below `2^118` raw
Q64.64 units, inside `u128`. The wrapper performs exactly two kernel calls,
one for the upper-depth/lower-transfer endpoint and one for the
lower-depth/upper-transfer endpoint.

## Downstream sibling

The additive crate is named
`optical-phase-space-dimensionless-transfer`. Its only domain dependencies are:

- `optical-phase-space-receiver-coupling`;
- `optical-phase-space-transport-certificate`; and
- `visible-radiance-bulk-transfer`.

It also uses `serde`, `serde_json` and `sha2`. No existing owner imports the
new sibling.

The frozen public types are:

- `OpticalBandTimeBindingV1`;
- `WholeCellDimensionlessTransferInputV1`;
- `WholeCellDimensionlessTransferOutcomeV1`;
- `WholeCellDimensionlessTransferArithmeticReceiptV1`; and
- `WholeCellDimensionlessTransferV1`.

The input binds one complete unchanged bulk profile, receiver-coupling input
and receiver-coupling result plus one band/time binding. The result repeats
the cell, transport, coupling, bulk-profile, band/time and selected-step IDs,
the inherited exact accepted/zero/unresolved measures, one conservative
outcome, arithmetic receipt, limitations and `none_evidence_only` authority.

## Band/time identity

`OpticalBandTimeBindingV1` contains one RGB `VisibleRadianceBandV1`, one
nonzero 32-byte `time_basis_id` and its derived `band_time_id`.

The identity is

`SHA-256("mindwarp.optical-phase-space.band-time.v1" || 0x00 || canonical_json([band,time_basis_id]))`.

It must equal both the transport input and certificate `band_time_id`.
Zero time basis, cross-band substitution, stale hash, unknown enum, caller
equality, or a binding mutated after transport compilation fails closed. This
binding states identity only; it grants no duration, calibration or source
quantity.

## Composition outcomes

The downstream compiler replays the complete coupling and bulk owners, then
constructs up to 64 unchanged `ConditionalIntervalBulkQueryV1` values from the
transport steps' stored `physical_input` and `physical_event` for the selected
RGB band.

Outcomes are exactly:

- `CertifiedAcceptedFiniteTransfer`, carrying finite summed Q64.64 depth and
  the bulk-owned evaluation receipt;
- `CertifiedAcceptedOpaqueTransfer`, only when a mandatory positive prefix
  step is opaque;
- `CertifiedAcceptedUnresolvedTransfer`, for selected-partial opacity,
  uncertain earlier opacity or otherwise conservative `[0, upper]` transfer;
- `CertifiedZeroCoupling`, carrying zero accepted measure and no factor; or
- `UnresolvedCoupling`, carrying unchanged unresolved measure and no factor.

Receiver-face and start-inside rules are exactly those in the verified design
and oracle. A projected zero lower bound remains underflow, not opacity.

## Resource and codec ceilings

- transport steps / conditional bulk calls: 64;
- Q64.64 endpoint additions: 128;
- bulk optical-depth evaluation calls: one result / two kernel endpoints;
- maximum finite summed raw optical-depth bits: 118;
- band/time binding bytes: 4 KiB;
- complete input bytes before decode: 128 MiB;
- complete result bytes: 256 KiB;
- aggregate live canonical bytes: 192 MiB; and
- retained conditional bulk transfer receipts: at most 64, 16 KiB each.

The downstream input cap may reject an otherwise valid oversized bulk profile;
that is typed `ResourceCeiling`, not unavailable, vacuum or zero. Decode checks
the outer byte ceiling before nested allocation, then every nested owner
replays and the complete object re-encodes byte-identically.

Downstream identity domains are:

- `mindwarp.optical-phase-space.dimensionless-transfer.input.v1`; and
- `mindwarp.optical-phase-space.dimensionless-transfer.result.v1`.

Order, nested owners, band/time binding, selected step, every retained bulk
receipt, optical-depth endpoints, outcome, exact measures, arithmetic receipt,
limitations and authority are identity-bound.

## Test-first and adversarial matrix

Implementation begins with failing fixtures for:

- first/later receiver-face and step-zero/later start-inside;
- vacuum, finite, mandatory-prefix opaque and selected-partial opaque;
- zero and unresolved coupling with exact measure conservation;
- 4-, 16- and 64-child accounting;
- central-lane substitution and repeated-Q0.48 underflow;
- wrong/zero/stale band-time binding;
- deleted, reordered, duplicated or foreign step/bulk/coupling evidence;
- forged optical-depth, transfer, measure, limitation and authority fields;
- byte, step, addition, raw-bit and live-memory ceiling edges; and
- strict unknown, trailing and noncanonical codec bytes.

The permanent oracle source and checksum remain pinned. All existing bulk V1
identity fixtures and every cell, transport, receiver-coupling and cumulative
transfer fixture must remain unchanged.

## Platform and integration gates

Require warnings-denied native all-target tests, executable
`i686-pc-windows-msvc`, `aarch64-linux-android` compilation, source-format and
no-`unwrap`/`expect` checks, module boundaries/context, record roles, active
context, historical C3 shields and complete `tools/verify.ps1`.

Android remains compilation evidence only. Actual mobile-device performance
is unmeasured. The 128/192 MiB ceilings are admission shields, not target
budgets; measured profiles are required before runtime promotion.

## Rollback

Rollback is deletion-only:

1. delete the downstream crate, contract, tests, verifier and registry links;
2. delete the additive bulk evaluation types/functions/tests; and
3. remove their workspace and module references.

No existing V1 data, identity, fixture, codec or behavior is migrated or
rewritten. Any implementation that requires such a rewrite returns to this
readiness gate.

## Exact serious owner action

Approve or reject this exact bounded package:

1. one additive read-only bulk optical-depth evaluation receipt using the
   unchanged existing kernel; and
2. one additive `optical-phase-space-dimensionless-transfer` sibling consuming
   it with the frozen band/time, composition, cap, test and rollback rules.

Without explicit approval, add no crate, contract schema, dependency,
production test or production source. Approval grants no source magnitude,
radiance, power, detector, visibility, perception, runtime, promotion or C3
closure authority.

## Verification receipt

The complete Forge integration gate passed with exit 0 in **249.9 seconds**
across 2,012 output lines. The run retained every historical C3 proof and
authority shield, both whole-cell transfer verifiers, 773 durable record-role
classifications, 49 module front doors, platform builds and workspace/UI
regressions without creating the proposed source.
