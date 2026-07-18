# G1 / C3 whole-cell dimensionless-transfer implementation result

Date: 2026-07-17

Status: **owner-authorized bounded implementation is implemented and fully
verified. This result grants no authority to add a second consumer or to infer
source magnitude, detector response, visibility, runtime use or C3 closure.**

## Implemented boundary

The existing `visible-radiance-bulk-transfer` owner now exposes one additive
read-only optical-depth evaluation receipt. It accepts ordered Q64.64
endpoints below 118 raw bits, calls the unchanged private exponential kernel
exactly twice and returns a strict domain-separated Q0.48 enclosure. Its input
and result are each capped at 4 KiB. The existing eight legacy V1 families,
bytes, identities, errors and behavior remain unchanged.

The new isolated `optical-phase-space-dimensionless-transfer` sibling replays
the complete bulk profile, transport certificate and receiver-coupling result.
It derives one RGB/nonzero-time band identity, rebuilds every conditional bulk
query from stored transport step evidence, composes mandatory prefix and
selected-partial depth, and asks the bulk owner for one final evaluation.

## Retained semantics

- Receiver-face prefix steps contribute full lower and upper depth; the
  selected partial step contributes zero through its full upper depth.
- Start-inside step zero is identity; later start-inside paths retain zero
  through prior upper depth.
- Mandatory prefix opacity proves exact zero. Selected-partial or prior
  start-inside opacity remains a typed widened transfer.
- Projected zero lower transfer is finite underflow, never opacity.
- Zero and unresolved coupling emit no transfer and retain the exact owner
  measure buckets unchanged.
- Every nested owner receipt, outcome, measure, limitation and authority field
  is reconstruct-and-compare identity-bound.

## Resource and authority receipts

The implementation freezes 64 steps, 128 endpoint additions, 118 raw
optical-depth bits, 128 MiB input, 256 KiB result and 192 MiB aggregate live
canonical ceilings. The new sibling imports only the three production owners
selected at readiness; fixture construction remains test-only.

No source magnitude, radiance, power, inverse-square spreading, detector,
visibility, perception, rendering, gameplay, runtime, approval, promotion or
C3-closure authority was added. Rollback remains deletion-only and requires no
existing V1 migration.

## Verification receipts

- Bulk source SHA-256: `d318b0919f8f59e64b782c30fb7a6898a8bb2aea092b6015b49372abdbcd4971`.
- Bulk evaluation test SHA-256: `5e91e9568f6c110ec966fe5f126a6a51f15130887830b36cb1a3764e029d045f`.
- Downstream source SHA-256: `0f9b5fb66cafd9c977dde80e37cf228a7cc041c4864e29b62dac20c3d2992d18`.
- Downstream test SHA-256: `fe1f42ef303a0779c8e92fd5c8d4e490be7d4c81a05e2d1fb4df88c6eab1b597`.
- Native focused suites pass four bulk-evaluation and five downstream hostile
  tests; the complete bulk all-target suite also retains every legacy test.
- Executable `i686-pc-windows-msvc` tests pass for both affected crates.
- `aarch64-linux-android` compilation passes for both affected crates. Actual
  mobile-device performance remains unmeasured.

- The complete `tools/verify.ps1` gate passed with exit 0 in **432.6 seconds**
  across **2,226 output lines**. It retained 781 durable record-role
  classifications, 50 current module front doors, every historical C3 shield,
  platform builds, workspace tests and UI regressions.

Focused, platform and complete success do not pre-authorize promotion or
another consumer. The next action is a code-free post-result reassessment.
