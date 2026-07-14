# Knowledge Record Contract v1

Forge preserves source evidence before interpreting it. A knowledge record is
a typed, reviewable projection over one or more immutable evidence objects; it
is never evidence itself and never grants authority.

## Record shape

Every record has a stable content-derived ID, schema version, record type,
lifecycle state, title, normalized summary, source evidence IDs, content
fingerprint, relationship links, authority lane, required gate, classifier
method/version/confidence, and creation/update timestamps.

Record types are `idea`, `plan`, `decision`, `task`, `research`, `correction`,
and `unclassified`. Lifecycle states are `detected`, `triaged`,
`awaiting_owner`, `approved`, `promoted`, `superseded`, `rejected`, and
`archived`.

Relationships are typed: `parent`, `dependency`, `related`, `duplicate`,
`contradiction`, and `supersedes`. A correction links to the prior record; it
does not rewrite or delete prior evidence.

## Intake and authority

Intake is idempotent by evidence ID, normalized content fingerprint, and
classifier version. Deterministic extraction handles explicit plan blocks,
headings, decisions, corrections, and task lists. Uncertain input remains
`unclassified`; Forge must not force a semantic guess.

Automatic intake may classify, summarize, group, deduplicate, and link.
It may not approve, promote, reprioritize, mutate `MASTER_PROGRAM.json`, submit
an owner observation, or cross an authority gate. Promotion requires direct
owner authorization or a recorded delegation and an expected master-program
revision. Failed validation is atomic and produces no partial canonical write.

## Projection boundary

Plans, Ideas, Decisions, Tasks, Research, and Corrections are generated views
over these records. They are not parallel physical sources of truth. Generated
views carry their source revision and can be deleted and rebuilt.

