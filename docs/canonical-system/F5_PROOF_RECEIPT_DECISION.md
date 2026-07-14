# F5 ProofReceipt Storage Decision

**Decision:** owner-approved versioned projection. **Date:** 2026-07-13.

The owner explicitly delegated the F4-to-F5 transition and asked Forge to
research the choice, select the logical lane, examine likely failures, and
repair them. The selected lane stores immutable ProofReceipt rows in a
separately versioned SQLite projection linked to existing Kernel evidence.
It does not add a Kernel object, event, authority basis, or candidate state.

## Why this lane

- The Kernel remains the sole protected evidence and authority boundary.
- A read-only projection matches the existing Reference Studio architecture.
- The existing online SQLite backup copies the full database, so receipt rows
  and their linkage rows travel with the same verified local backup.
- Additive tables provide a bounded rollback: an older build ignores the new
  tables while the protected event journal remains unchanged.
- Schema version, canonical receipt ID, immutable retry behavior, and explicit
  mismatch reporting make evolution visible.

Primary guidance supports the shape without making it authoritative: SQLite
foreign keys can enforce retained row linkage when enabled per connection;
transactions make the receipt and link insert atomic; the online backup API
copies a consistent database; and read-only materialized projections are a
standard way to query an authoritative append-only record separately.

Sources:

- <https://www.sqlite.org/foreignkeys.html>
- <https://www.sqlite.org/lang_transaction.html>
- <https://www.sqlite.org/backup.html>
- <https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing>

## Adversarial failure review

| Failure | Required shield |
|---|---|
| Dangling input/output reference | Link rows have foreign keys to retained Kernel objects; admission also resolves every reference before write |
| Partial receipt/link write | One SQLite transaction commits the receipt and all ordered links |
| Conflicting retry | Canonical content hash is the receipt ID; equal retry is idempotent and altered content fails closed |
| Projection drift or corruption | Reads revalidate schema, canonical ID, evidence existence, and exact ordered linkage |
| Silent schema comparison | Expected and local versions return `compatible` or `version_mismatch`; unsupported writes are rejected |
| Backup omission | Recovery fixture reopens both live and online-backup databases and reads identical receipt counts |
| Receipt text becomes authority | Contract excludes authority fields; hostile authority-like text and inspection leave Kernel counts unchanged |
| Registry typo or scope escape | Schema v1 accepts only canonical game-system IDs mirrored from `system-registry.json` |
| Misleading performance claim | Every measurement names unit, method, and measured/simulated/estimated classification |
| Protected-Kernel expansion | No change to `ForgeKernel`, `EventType`, `AuthorityBasis`, or candidate lifecycle |

## Bounded package

Owner/module boundary: `forge-kernel::contracts` defines the neutral record;
`forge-kernel::persistence` owns projection validation and storage; Reference
Studio only reads it. Rollback target: the verified F4 database and inspector,
which remain compatible because the migration is additive. Authority lane:
delegated implementation only; no approval, promotion, execution, engine,
runtime, network, spending, credential, or publishing authority.

Exit proof is the contract, validator fixtures, backup/reopen/corruption tests,
Reference Studio mutation-negative fixture, UI build, full Rust tests, desktop
build, and the repository verification gate.
