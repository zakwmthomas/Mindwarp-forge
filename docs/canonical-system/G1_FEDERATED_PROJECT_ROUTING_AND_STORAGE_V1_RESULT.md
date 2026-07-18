# G1 Federated Project Routing and Storage V1 Result

Status: implemented and verified on 2026-07-17.

## Result

The bounded V1 package is complete in source and disposable integration:

- immutable, revisioned project and workstream records now provide stable identities, canonical aliases, successor evidence, blockers and leases;
- session routing is explicit and fail-closed, while suggestions cannot silently assign a project;
- Knowledge V4 retains globally deduplicated content and adds normalized project, workstream, session, entity, system, actor, facet and lifecycle bindings;
- SQLite FTS5 queries support the normalized filters and rank captured user or owner evidence ahead of assistant narration;
- generation receipts bind classifier version, expected and written counts, evidence count and digest, and incomplete higher generations cannot become current;
- verified compressed archives are written through a temporary file, replayed, hash/count checked and atomically renamed without deleting the raw backup;
- managed-source inventory is Git-bound, excludes rebuildable caches, rejects path escape, and records repository state;
- cache cleanup remains a preview-only, approval-required plan with canonical paths, byte/file counts, a tree fingerprint and a domain-separated plan hash;
- compact bootstrap output points to SQLite FTS5 instead of loading the full knowledge catalogue.

Greenfield and Greenfeld are retained as aliases of one independent project. Mindwarp Forge and Greenfield remain separate projects connected only by an evidence-only reuse link. Their workstreams and session route are durable under `governance/federation/`.

## Verification

- `cargo test -p forge-kernel --all-targets`: 102 passed.
- `tools/verify-federated-routing-storage-v1.ps1`: passed with disposable project, workstream, route, link, knowledge-generation, inventory, cleanup-plan, compact-bootstrap and read-only query integration.
- `tools/verify-conversation-compiler-continuity.ps1`: passed with indexed query filters and no full-catalogue parsing.
- module context: 48 declared modules current.
- `tools/verify.ps1`: passed in 301.7 seconds after the result implementation, including native, i686 and Android gates and the preserved C3 optical chain.

## Live-state containment

The generic live backfill command has been removed. Backfill and generation finalization require an explicit disposable fixture flag.

During implementation, previously queued helper commands outlived the source revision and wrote V4 rows to the live Forge SQLite database. A later read-only audit found `PRAGMA quick_check = ok`, legacy V1-V3 rows retained, V4 rows present, and one V4 generation receipt. The owner subsequently accepted that independently reproduced receipt as the canonical live search baseline. No deletion, rollback, replacement, cleanup execution or further live migration is authorized by this result.

The read-only reconciliation reproduced the 26,707-row receipt and exact digest and explained 260 later rows as 39 later evidence objects. Acceptance requires no live rewrite. Any repair, rollback, deletion or cleanup remains a separate serious owner decision.

## Authority retained

This result grants no deletion, cleanup execution, cross-project merge, publication, spending, credential use, game/runtime authority, or C3 closure. C3 retains its code-free whole-cell receiver-coupling next action and receiver-before-face ordering.
