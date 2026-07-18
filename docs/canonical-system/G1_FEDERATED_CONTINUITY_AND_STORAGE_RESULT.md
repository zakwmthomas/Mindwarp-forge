# G1 Federated Continuity and Storage Result

Date: 2026-07-17  
Disposition: implemented and verified; cache cleanup remains deliberately unexecuted.

## Result

Forge now has an authority-negative federation layer for independent projects,
parallel workstreams, append-only session routes and cross-project evidence
links. Greenfield is registered as an independent repository with the explicit
aliases `Greenfeld` and `Greenfield`; its captured session
`019f6cc7-f99f-7781-81f9-88ef1d4b5121` routes to the separate
`greenfield-release-readiness` workstream. It is not merged into Mindwarp Forge
or `mindwarp-game`, and reuse remains behind a target-local gate.

Knowledge schema/classifier v4 retains all older rows and exact evidence while
adding normalized project, workstream, entity, session, facet, actor and system
indexes. SQLite FTS5 is the primary query path. `forge-query` opens the database
read-only; `tools/find-knowledge.ps1` exposes the same path without the GUI or a
full JSON-catalogue parse. A live project-scoped Android query returned the
Greenfield session and workstream below the one-second gate.

The generated knowledge catalogue is now a 160-byte SQLite pointer rather than
a 32.7 MB duplicate. The managed workspace binding is about 233 KB rather than
about 28 MB because `.git`, `.local`, `target`, `node_modules` and `artifacts`
are excluded from per-file source hashing. The retained bootstrap pack is about
5.1 MB and still contains the exact session projections and evidence catalogue.

Verified cold archives use bounded streaming gzip, source/archive SHA-256,
decompressed size limits and SQLite replay-count verification. The raw source
backup is retained. Concatenated/trailing gzip members, truncation, fixity drift,
replay mismatch and overwrite attempts fail closed.

The cleanup surface is preview-only. Its current plan reports approximately
58.1 GB across 126,000 rebuildable `target` files, with a deterministic metadata
fingerprint, `approval_required=true` and `executed=false`. No cache, backup,
conversation, evidence object or database row was deleted.

## Durable interfaces

- `contracts/federated-project-routing-contract.md`
- `contracts/indexed-knowledge-v4-contract.md`
- `contracts/storage-lifecycle-contract.md`
- `governance/federation/`
- `crates/forge-kernel/src/federation.rs`
- `crates/forge-kernel/src/bin/forge-query.rs`
- `crates/forge-kernel/src/bin/forge-federate.rs`
- `crates/forge-kernel/src/bin/forge-storage.rs`
- `tools/find-knowledge.ps1`
- `tools/preview-forge-cache-cleanup.ps1`
- `tools/verify-federated-continuity.ps1`

## Verification

- `cargo test -p forge-kernel --all-targets`: 100 tests passed at the first
  complete package run; subsequent focused federation/storage suites also pass.
- `cargo test -p forge-desktop`: 41 tests passed.
- `tools/verify-federated-continuity.ps1`: 12 focused tests plus live routing,
  compact-projection, indexed-query and preview-only cleanup checks passed.
- `tools/verify-module-context.ps1`: 48 module front doors current.
- `tools/verify-record-roles.ps1`: 746 durable files classified at the package
  recording point.
- `tools/ensure-context-current.ps1`: capture current after the upgraded desktop
  executable completed the live generation receipt.

The final complete `tools/verify.ps1` receipt is recorded in the active
checkpoint after this result is written.
