# G1 C4 broad hierarchy/history closure readiness

Status: **owner-authorized readiness; source hardening is blocked until this
boundary and hostile matrix pass.**

## Exact route

C4 depends exactly on C2 and promoted C3A. It consumes stable universe identity
and an exact replay-validated `WorldGenerationInput` / `CausalWorldPacket` pair.
C3B physical applicability, visibility and presentation evidence is independent
and not a C4 dependency. C5-C7, broad G1 closeout and R1 remain gated.

The verified GP4 Signal Anchor vertical and `G1-VERTICAL-CLOSEOUT` receipt are
immutable predecessor evidence; neither substitutes for broad C4.

## Existing retained proof

- `hierarchy-history`: 11 tests for strict descriptors, bounded windows,
  residency-neutral identity, ordered/idempotent replay, corruption rejection,
  snapshot distrust, one-hop migration and baseline coexistence.
- `addressable-world-binding`: 5 tests for exact C3A replay, deterministic
  binding, crossed-pair rejection and identity/provenance separation.
- `entity-lifecycle`: 11 deterministic transition and validation tests.
- `entity-lifecycle-history-binding`: 6 recovery, continuation and hostile
  stored-value tests.
- C4V remains a separately verified GP4-only regression seam.

## Closure obligations

1. Stable dynamic instance identity and strict distinct `NeverObserved`,
   `Absent`, `Present` and `Tombstoned` evidence without implementing deletion
   or topology changes.
2. Stable per-entity ambient cohort binding from exact entity logical identity
   plus an assignment-contract/version fingerprint, with strict bytes,
   cross-identity rejection and no reroll across reload or baseline replay.
   Species weights and population-distribution quality remain C6-owned.
3. Exact full baseline dependency availability and fingerprint matching. The
   availability set must equal the manifest's registered output-affecting set;
   unknown extras, incidental dependencies and C3B kinds fail closed.
4. Known-good-prefix recovery from corrupt/truncated tails without scanning
   past the first failure or mutating retained bytes.
5. Explicit no-mutation rejection of reparent, split and merge semantics.
6. Bounded identity-only migration chains with every intermediate validated,
   missing/failed hop rejection and readable source rollback.
7. Deterministic fixed window, event-log, replay and snapshot byte/work rows;
   timings remain local observations, never canonical acceptance thresholds.
8. Byte-identical semantic receipts in separate fresh processes and with both
   independent execution provenance and platform diversity: a different OS
   family or actual target architecture/device class at the same source
   manifest. Remote Windows x64 does not add platform diversity; same-host
   i686 does not add independent execution.
9. Dedicated hostile, dependent-regression, module-context, record-role and
   registered full-gate evidence before C4 becomes complete.

## Frozen bounds

- child limits: `0`, `1`, `16`, `256`; maximum admitted request `256`;
- history event rows: `0`, `1`, `16`, `64`, `256`;
- recovery: at most `1024` records and `16 MiB` encoded input;
- migration: identity-only, at most `2` hops in the C4 receipt;
- receipt: strict typed JSON/bytes, unknown fields and coercion rejected;
- no random, wall-clock, filesystem, network, process, database, runtime,
  renderer or Kernel capability in canonical source.

## Hostile families

- identity/origin/recipe drift, zero/colliding dynamic IDs and presence-tag,
  length, trailing-byte or fingerprint substitution;
- unsorted, duplicate, zero, missing or fingerprint-mismatched dependencies;
- wrong baseline/target, gap, stale head, fork, changed equal-command retry,
  unknown operation, cross-target operation, reparent/split/merge;
- corrupt/truncated/trailing envelope and attempted recovery past the last
  verified event;
- snapshot baseline/head/sequence/reducer/builder/state/hash substitution;
- missing, reordered, non-contiguous, repeated, failed or over-bound migration
  hops and altered source/adapter/output retry;
- semantic receipt unknown/missing/reordered/coercible fields, dependency or
  source drift, authority-flag flips and hash drift;
- single-process output presented as fresh-process evidence, compile-only
  target presented as execution, or same-host architecture presented as an
  independent platform.

### Frozen hostile registry

The ordered registry has 74 IDs. Its digest is
`4d4b7cb792f5b410092d247354bac62a5b8f3dc880fcb2a6ad61ffafadff127c`,
computed as SHA-256 over UTF-8
`mindwarp/c4-hostile-registry/v1\0` followed by the IDs joined with LF:

`identity.dynamic-zero-parent`, `identity.dynamic-zero-instance`,
`identity.dynamic-domain-drift`, `identity.dynamic-vector-drift`,
`presence.unknown-tag`, `presence.state-substitution`,
`presence.zero-fingerprint`, `presence.trailing-bytes`, `cohort.zero-entity`,
`cohort.zero-contract`, `cohort.entity-drift`, `cohort.contract-drift`,
`cohort.value-drift`, `cohort.reroll`, `cohort.trailing-bytes`,
`dependency.manifest-invalid`, `dependency.missing`,
`dependency.fingerprint-mismatch`, `dependency.extra`,
`dependency.c3b-extra`, `dependency.unsorted`, `dependency.duplicate`,
`dependency.zero-kind`, `history.wrong-baseline`, `history.wrong-target`,
`history.gap`, `history.stale-head`, `history.fork`,
`history.command-conflict`, `history.unknown-schema`, `history.cross-target`,
`history.reparent`, `history.split`, `history.merge`,
`history.corrupt-envelope`, `history.truncated-envelope`,
`history.trailing-envelope`, `history.recovery-past-prefix`,
`history.recovery-bound-overflow`, `snapshot.wrong-baseline`,
`snapshot.wrong-head`, `snapshot.wrong-sequence`, `snapshot.wrong-reducer`,
`snapshot.wrong-builder`, `snapshot.wrong-state`, `snapshot.wrong-hash`,
`migration.missing-adapter`, `migration.zero-adapter`,
`migration.duplicate-adapter`, `migration.wrong-logical-id`,
`migration.same-baseline`, `migration.reordered-hop`,
`migration.noncontiguous-hop`, `migration.failed-hop`,
`migration.overbound`, `migration.altered-source`,
`migration.changed-retry`, `migration.receipt-tamper`,
`receipt.unknown-field`, `receipt.missing-field`,
`receipt.dependency-reorder`, `receipt.type-coercion`, `receipt.proof-drift`,
`receipt.source-drift`, `receipt.authority-flip`, `receipt.hash-drift`,
`portability.single-process`, `portability.stdout-mismatch`,
`portability.source-mismatch`, `portability.compile-as-execution`,
`portability.same-host-as-independent`, `portability.same-platform-remote`,
`portability.target-drift`, `portability.runner-drift`.

## Explicit exclusions

Per-entity ambient cohort binding is C4-owned. Population weights,
distribution quality, species/ecomorph and `PresentationProfile` semantics
belong to C6. Rendered creature/phone comparison belongs to C7. A
provenance-independent physical-equivalence fingerprint is optional until a
named consumer needs it. Storage engines, real cache/residency policy,
filesystem layouts, destructive compaction, multiplayer merge, cross-target
transactions and runtime object IDs belong to R1 or later. No C3B, Companion,
Greenfield, visual asset or protected-Kernel mutation is authorized.

## Stop rule

Local x64 fresh processes, same-host i686 execution classified exactly as
`same_host_second_architecture`, and Android compilation classified exactly as
`compile_only` may produce honestly classified evidence, but cannot satisfy
both independent execution and platform diversity. If no qualifying second
OS/device/independent runner is
available, stop at `candidate_verified_local`; do not promote C4 or activate C5.
