# H6 humanoid reproduction and recovery readiness

Status: **coverage audit complete; bounded implementation pending**.

H6 proves continuity of the exact H1-H5 proof chain. It does not create a
humanoid asset, acquire the PolyOne file, choose a runtime, reinterpret the
owner's visual decision, or authorize promotion. The audit refreshed the
program and the relevant contracts, implementations, tests, receipts, and
known Windows failure modes before selecting any implementation.

## Five-claim coverage audit

| H6 claim | Existing decisive evidence | Exact remaining gap | Cheapest next proof |
|---|---|---|---|
| Clean rebuild | H1 re-hashes both recovered archives and their exact entries. H2 and H3 rebuild executable receipts from typed sources. The complete Forge gate rebuilds the Rust workspace and desktop. | There is no single strict manifest that rebuilds H1 through H5 in dependency order and proves all stage identities together. | Capability-free typed manifest built directly from the existing H1-H4 reference constructors plus one typed H5 decision receipt. |
| Deterministic replay | H1 suite, H2 profile, H3 candidate, H4 calibration, viewport projection, and Kernel ProofReceipt IDs are independently content-derived and deterministic. | H5 is currently canonical prose/checkpoint evidence rather than a strict replayable record, and no test recomputes the aggregate chain identity twice. | Canonical H5 decision record plus two independent in-memory chain builds with byte-identical manifest and chain fingerprint. |
| Corruption recovery | Strict H1-H4 codecs reject unknown fields, drift, missing roles, stale links, overclaims, and noncanonical bytes. Kernel backup and ProofReceipt tests reject fixity, count, and evidence-link corruption. | Nothing applies corrupt, truncated, reordered, wrong-link, unknown-version, or semantically widened H5 data to the whole chain while proving the last known-good manifest remains unchanged. | Table-driven mutation cases against disposable canonical bytes; recover by rebuilding from retained authoritative inputs, never by repairing hostile bytes in place. |
| Stable identifiers | Exact H1 suite, H2 profile, H3 input/candidate, H4 calibration, reference-scene, and visual-content hashes are retained. | H5 lacks a typed decision ID, and there is no aggregate chain ID binding ordered stage IDs and their dependency links. | Domain-separated content IDs for the H5 decision and ordered H1-H5 manifest; reject duplicate, missing, reordered, or mismatched stage links. |
| Retained receipts | Kernel ProofReceipt storage is additive, link-validated, authority-negative, backed up, reopened, and displayed read-only. | Existing ProofReceipt tests use generic fixture rows; they do not retain and recover the exact H1-H5 receipt set or the owner-approved H5 fingerprint and limitations. | Persist the manifest and exact stage receipts as evidence objects in a disposable Kernel journal, record an authority-negative proof receipt, back up, reopen, and compare exact IDs/bytes/counts. |

## Audit finding repaired before implementation

`H5_VISUAL_REFERENCE_INTAKE.md` contained two stale pre-owner-resolution
paragraphs stating that no candidate was `verified_fit` and that PolyOne's
proportions were too stylized to copy. The later owner-resolution section and
the active checkpoint correctly admitted the fingerprinted preview set as the
stylized feminine visual target. H6 repaired the stale prose before designing
replay so the canonical source no longer presents incompatible states.

## Selected bounded design

Add one small engine-neutral proof-chain module rather than modifying H1-H5 or
inventing a second general recovery system. It will:

1. call the existing reference constructors for H1-H4 instead of copying their
   structural data;
2. define the smallest strict H5 owner-decision record needed to retain the
   verified preview fingerprint, intended visual role, limitations, modular
   construction rules, and absence of import/promotion authority;
3. construct an ordered H1-H5 manifest whose content-derived ID binds every
   stage ID, fingerprint, dependency, contract version, outcome, and
   limitation;
4. parse strict canonical bytes and fail closed on mutation, truncation,
   reordering, missing or duplicate stages, stale links, unknown versions, and
   semantic widening;
5. rebuild a known-good manifest only from authoritative typed inputs; and
6. use the existing Kernel object, ProofReceipt, online-backup, and reopen
   mechanisms for the durable recovery fixture rather than creating another
   database or receipt authority.

The first proof tier is in-memory canonical reconstruction and mutation. The
disposable SQLite backup/reopen fixture is justified only after that cheaper
tier passes. A full PC or graphics test has no information value for H6.

## Required assertions

- H1-H4 fingerprints remain byte-for-byte equal to their published receipts.
- H5 binds preview SHA-256
  `f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051`.
- H5 retains cute/sweet stylized feminine direction, strong/commanding
  masculine direction, neutral modular construction, and all production/import
  limitations without turning visual wording into capability rules.
- Rebuilding twice yields identical canonical bytes and aggregate chain ID.
- Every hostile case returns no replacement manifest and leaves the known-good
  bytes unchanged.
- Backup/reopen returns exact manifest, receipt, link, byte-count, and fixity
  identities.
- No observation, replay, recovery, or receipt can approve, promote, import,
  execute, select a runtime, or mutate protected authority.

## Stop condition

Stop H6 when one clean build/replay, the bounded hostile matrix, one known-good
recovery, one exact backup/reopen receipt proof, the focused verifiers, and the
complete Forge gate pass. Asset acquisition, rigging, deformation, shaders,
aging visuals, LOD generation, and device profiling remain later work.
