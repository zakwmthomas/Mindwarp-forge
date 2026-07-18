# Indexed knowledge v4 contract

Status: owner-authorized additive Forge continuity component.

## Immutable evidence and generation boundary

Knowledge V4 remains a typed projection over immutable evidence. It stores one
bounded source span once. Facet, entity and Atlas-system memberships plus
project, workstream and session routing bindings are normalized references.
The latter are append-only many-to-many indexes over the globally deduplicated
content row. Empty project/workstream references mean `unrouted`; they never
default to Forge.

Record IDs bind the evidence ID, exact byte span, normalized content
fingerprint and classifier version, not a mutable route. Reference arrays are
sorted and unique; each present entry is nonempty, while a reference set may be
empty. Every source span stays inside its exact evidence bytes. Authority is
always `evidence_only`.

V1-V3 rows and their evidence IDs remain append-only and queryable for recovery.
A V4 backfill is written in one transaction and becomes current only after a
generation receipt records the classifier version, expected and written record
counts, evidence count and deterministic digest. Merely observing one V4 row or
the maximum classifier version never proves a complete migration.

## Indexed retrieval

SQLite FTS5 is the primary local search path. Normalized reference tables
support project, workstream, entity, lifecycle, actor, facet, system and session
filters without parsing the full generated catalogue. Query text and limits are
bounded; malformed FTS syntax returns a typed query error rather than falling
back to a broader search.

Results rank exact project/checkpoint and owner-authored correction or promoted
canonical evidence above assistant progress narration. Ranking is deterministic
for equal scores. Duplicate and superseded records may be collapsed in a view,
but their rows and raw evidence remain intact and directly retrievable.

The `forge-query` CLI is read-only and model-independent. The PowerShell search
helper may wrap it but cannot silently revert to a stale large-catalogue parse
after V4 is promoted. Explicit evidence-open remains the raw-byte fallback.

## Scale, recovery and rollback

The synthetic gate uses 250,000 records: warm-query p95 below 250 ms, cold-query
p95 below one second, and no full-catalogue startup parse. The current corpus
must preserve every evidence ID and V3 row across disposable migration.

Rollback restores V3 query/projection selection while retaining every V4 row,
generation receipt, index and source object. Search and routing never grant
approval, promotion or filesystem authority.
