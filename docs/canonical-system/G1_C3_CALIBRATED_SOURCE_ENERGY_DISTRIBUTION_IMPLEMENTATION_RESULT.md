# G1 C3 calibrated source-energy distribution implementation result

Status: implemented and verified as the owner-approved bounded V1 package.

## Outcome

The additive `calibrated-source-energy-distribution` owner now compiles one
validated spectral/time calibration and one replayed depth-zero optical
phase-space root into an immutable closed frontier of exact radiant-energy
allocations. It is the first consumer of the unchanged calibration and cell
evidence owners and has zero downstream consumers.

The implementation did not change any existing owner source, public API,
canonical bytes or identity. It adds no persistence, process, filesystem,
network, UI or protected Kernel capability.

## Implemented proof boundary

- Each compact directive identifies one current unresolved allocation and one
  upstream phase-space axis. The existing `split_optical_phase_space_cell`
  function derives both children, their paths, measures, identities and phase
  split receipt.
- Every atomic split proves exact `parent = lower + upper` joule conservation
  with checked `Signed512` arithmetic under the frozen 385-bit shield.
- Resolved leaves cannot be refined again; missing or retired frontier parents
  fail typed.
- Subject, allocation, split and distribution identities are acyclic and locked
  by a canonical fixture.
- Strict codecs reconstruct query bytes and a separate replay entry point
  reconstructs the complete claimed distribution.
- V1 admits 64 frontier allocations, 63 directives/receipts, depth 12, 128 KiB
  query, 256 KiB result and 4 MiB conservative live canonical bytes.

## Verification receipt

The focused implementation suite passes 10 contract/hostile/resource tests plus
the frozen public-surface test. The complete 64-leaf envelope is exercised using
63 real upstream axis splits.

- Native Windows all-targets: pass.
- Executable `i686-pc-windows-msvc`: pass.
- Compile-only `aarch64-linux-android`: pass.
- Locked one-split query: 4,124 bytes,
  SHA-256 `547b4f1003e54a37247b1a820ed5f508673026066a9c4f80638129047cc82cc5`.
- Locked one-split result: 3,163 bytes,
  SHA-256 `0cd4aa695ebfb9eb83525189cfc42f8fb2484c20b848bb255c06c08a7c4a3fda`.

The focused implementation verifier, module-context, modularity, record-role,
historical-route, bootstrap and deletion-only rollback shields all pass. The
complete Forge gate passes in 300.5 seconds with 2,385 output lines, 828 durable
files and 52 declared modules. The workspace-wide Rust test suite, doc tests,
UI build and desktop compile are included in that receipt.

## Authority, continuation and rollback

Transport and perception remain blocked. This result does not authorize
normative source discovery, transport applicability, received energy, detector
response, visibility, runtime integration, promotion, supersession selection or
C3 closure. Nothing broader is locked in.

Rollback is deletion-only: remove the new crate, contract, tests, fixture,
verifier, result and governance entries. Existing evidence remains replayable.
