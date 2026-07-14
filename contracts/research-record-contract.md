# Research Record Contract v0.1

Research records are local provenance evidence only. They cannot approve,
promote, execute, select an engine, change project direction, invoke a
connector, use credentials, or authorize spending.

`ResearchSourceRecord` retains source identity, origin/type, access time,
optional fixity, location, access/license notes, limitations, freshness, and
availability. Freshness is `fresh`, `stale`, or `unknown`; availability is
`available`, `missing`, or `inaccessible`. Missing or inaccessible sources may
be recorded as gaps but cannot support a claim.

`ResearchClaimRecord` contains one atomic claim, one exact source/span link,
confidence, limitations, and affected systems. An absent span, unknown source,
or unavailable source fails closed as unsupported.

`ResearchContradictionRecord` links two existing claims, preserves scope
difference and the unresolved question, and names discriminating evidence. Its
status is `unresolved`, `scope_mismatch`, or `resolved_by_evidence`; insertion
never chooses a winner or changes authority.

IDs are immutable. Equal retries are idempotent; reusing an ID for different
content is a conflict. Cached/stale and refreshed source observations use
distinct record IDs so both provenance states remain visible. Reopen/replay
must return the same records and hostile source text must leave Kernel events,
candidates, and authority unchanged.
