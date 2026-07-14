# Reference Studio Inspector Contract v0.1

The Reference Studio exposes one verified-local, versioned, read-only
projection of existing Forge research gaps and control-plane lifecycle records.
It does not define or select the owner-gated ProofReceipt storage binding.

Every projection states its schema version, requested version, compatibility,
local source, verification time, read-only status, and limitations. It retains
passed, failed, blocked, rollback, and source-gap records without inferring
missing details or turning displayed text into authority.

The inspector has no command for approval, promotion, application, execution,
filesystem browsing, network access, capture disclosure, or runtime control.
Its UI has one refresh action only. Inspection must leave kernel objects,
events, candidates, and control records unchanged.

Fixtures cover an empty projection, verified records, failed gate and open
blocker visibility, missing-source visibility, schema mismatch, hostile
authority text, and mutation-negative kernel counts.
