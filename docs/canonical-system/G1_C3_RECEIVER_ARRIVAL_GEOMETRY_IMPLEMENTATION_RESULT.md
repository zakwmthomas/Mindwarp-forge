# G1 / C3 receiver-arrival geometry implementation result

Date: 2026-07-16

Status: **implemented and verified as an additive capability-free exact-ray
geometry owner; no source magnitude, spreading, detector, visibility,
perception, runtime, promotion or C3 closure claim.**

## Implemented surface

`crates/receiver-arrival-geometry-binding` now binds one replayed
`OpticalLineageBundleInputV1` and `OpticalLaneManifestV1` to one positive-volume
Cartesian Q160 `ReceiverAabbV1`. It produces strict result and transcript
identities under the frozen receiver domains and reconstruct-and-compare JSON
codecs.

The classifier preserves five distinct facts:

- start strictly inside the receiver;
- certified strict-interior arrival within one ordered lineage step;
- contact-only evidence, including tangent and current-face tie;
- typed unsupported conditional evidence; and
- typed upstream/no-arrival terminal preservation.

The two-step face-tie shield proves that equality with the current physical
face is contact on that step and that any later strict entry belongs to the
successor step.

## Arithmetic result

The implementation uses only shared checked signed-512 arithmetic. Q1.62
direction is exactly liftable to Q160; before rational cross-products the
common `2^98` lift is algebraically cancelled against the `2^160` numerator,
leaving the identical `(bound - point) * 2^62 / direction_q1_62` ratio. This
prevents a needless over-wide representation while retaining exact ordering
inside the approved 414-bit live shield.

The permanent receipt enforces 64 steps, 384 directed divisions, 768 bound
comparisons, 64 intersections, 18 MiB input, 256 KiB output and 32 MiB live
canonical validation ceilings.

## Verification receipts

- Five focused Rust integration tests pass with warnings denied. They cover
  strict entry, start-inside, no-arrival, tangent contact, reverse direction,
  parallel-outside rejection, point/direction conditional exclusion,
  receiver-face tie with successor ownership, identities, physical bounds,
  strict codecs, trailing bytes and authority forgery.
- The pinned independent oracle passes all 18 exact-rational portfolios and 26
  hostile mutation families with receipt
  `25c31003ff4ee8d1be3b01a5a2203958238205e4adc80e6cc50623c27af69aea`.
- Executable `i686-pc-windows-msvc` tests pass and
  `aarch64-linux-android` ARM64 compilation passes.
- Physical-path, optical-lineage and cumulative-transfer owner suites pass
  unchanged after source.
- Module boundaries, module context, record roles, formatting, workspace tests,
  UI build and isolated desktop build pass in complete `tools/verify.ps1`.
- The final complete Forge verification passed in **229.3 seconds**.

## Authority and remaining limits

The result remains `none_evidence_only`. Nondegenerate conditional boxes are
typed unsupported rather than sampled. No real mobile-device performance was
measured. Geometry does not imply aperture acceptance, detector response,
received power, visibility, darkness, perception or gameplay line of sight.

Rollback remains deletion-only for the new crate, workspace membership,
contract, module records, result and permanent verifier; no upstream schema or
object migration is required.
