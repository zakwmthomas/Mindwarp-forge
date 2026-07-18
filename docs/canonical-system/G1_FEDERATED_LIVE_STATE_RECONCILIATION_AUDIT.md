# G1 Federated Live-State Reconciliation Audit

Date: 2026-07-17  
Disposition: read-only audit complete; owner accepted V4 on 2026-07-17.

## Finding

The live V4 generation is internally complete for its recorded boundary. There
is no evidence of a partial generation, corrupt database, or unexplained row
drift.

The live database was opened read-only. `PRAGMA quick_check` returned `ok`.
The sole generation receipt records classifier V4, 3,790 evidence objects,
26,707 expected records, 26,707 written records, and digest
`a840f2d069511535fc246dbe5a6205dd614109cf4a2424fbfb46a39c73853b5f`.

The receipt boundary was reconstructed independently from the immutable event
order. The first 3,790 distinct `EvidenceRegistered` objects map to exactly
26,707 retained V4 rows. Recomputing the contract's domain-separated SHA-256
over their sorted `record-id:content-fingerprint` identities produced the exact
receipt digest above.

At the audit boundary the database contained 3,829 distinct evidence objects
and 26,967 V4 rows. The 39 evidence objects captured after the receipt boundary
map to exactly the 260 additional V4 rows:

`26,707 receipt-bound rows + 260 later-capture rows = 26,967 current rows`.

This is ordinary append-only capture after a complete generation receipt, not
an incomplete backfill. V1-V3 records remain retained. Normalized reference
tables and FTS remain additive projections; no authority follows from them.

## Decision recommendation

Recommend **accept/promote the existing V4 receipt as the live baseline**.
This has the strongest evidence because the receipt count and digest reproduce
exactly and every later row is explained by later evidence capture.

- **Promote/accept:** recommended. Record owner acceptance of the already
  complete baseline; keep all later V4 rows and every V1-V3 row.
- **Retain V3 selection:** safe but unnecessary as a conservative delay. It
  would forgo the verified indexed retrieval path while retaining the V4 data.
- **Repair:** not recommended. No mismatch or missing receipt-bound row was
  found, so a repair would add risk without correcting an observed defect.
- **Rollback/delete:** not recommended and not authorized. It would discard or
  obscure valid append-only projection history without an evidence-based cause.

## Owner decision

The owner explicitly accepted the recommended V4 option on 2026-07-17. The
existing independently reproduced V4 generation receipt is therefore the
canonical live search baseline. The 260 later rows remain ordinary append-only
capture after that accepted boundary, and every V1-V3 row remains retained.

Acceptance records the selection already enforced by the complete-generation
receipt rule. It requires no database rewrite and grants no repair, rollback,
deletion, cleanup, merge, publication, credential, spending, game/runtime or
C3-closure authority.

## Authority boundary

This audit performed no database write, repair, rollback, deletion, cleanup,
project merge, or authority grant. The later owner decision accepts only the
verified V4 selection. Cache cleanup remains preview-only and unexecuted.

## Read-only receipts

- SQLite integrity: `quick_check = ok`.
- Receipt: V4; evidence `3790`; expected/written `26707/26707`.
- Independent receipt-bound row count: `26707`.
- Independent digest: exact match.
- Later capture: `39` evidence objects and `260` V4 rows.
- Current audit boundary: `3829` evidence objects and `26967` V4 rows.
- Complete Forge verification: `tools/verify.ps1` passed after the V1 source
  package and continuity-preservation changes.
