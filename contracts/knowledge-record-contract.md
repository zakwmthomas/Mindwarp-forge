# Knowledge Record Contract v4

Forge preserves source evidence before interpreting it. A knowledge record is
a typed, reviewable projection over immutable evidence; it is never evidence itself
and never grants authority.

The primary operational consumer is the active AI assistant maintaining project
coherence across conversations. Owner-facing browsing is an optional secondary projection.
Every current record must therefore be discoverable through the
model-agnostic bootstrap and deterministic local search without requiring the
owner or the assistant to operate the Forge GUI.

## Record shape

Every record has a stable content-derived ID, schema version, one bounded source
span, one canonical text projection, one or more role facets, zero or more
Atlas-system, project, workstream, entity and session references, lifecycle state, source evidence IDs, content
fingerprint, relationship links, authority lane, required gate, classifier
method/version/confidence, and creation/update timestamps. Its text is stored
once; role, system and relationship memberships are references.

Role facets are `idea`, `plan`, `decision`, `task`, `research`, `correction`,
`philosophy`, `requirement`, `constraint`, `preference`, `risk`, `question`,
`observation`, `context`, and `unclassified`. Lifecycle states are `detected`,
`triaged`, `awaiting_owner`, `approved`, `promoted`, `superseded`, `rejected`,
and `archived`.

Role facets answer what kind of statement this is. Atlas-system references
answer which project areas it concerns. These axes must not be conflated. One
record may carry several values on either axis without copying its text.

An empty project/workstream reference set means unrouted and never defaults to
Forge. Routing receipts, not text keywords, supply project scope. Reference
arrays are canonical sorted unique sets. Project, workstream and session
bindings are append-only normalized indexes over the globally deduplicated
content row. They are not part of content identity and may grow when later
routing evidence arrives; changing a route never rewrites knowledge text.

Relationships are typed: `parent`, `dependency`, `related`, `duplicate`,
`contradiction`, and `supersedes`. A correction links to the prior record; it
does not rewrite or delete prior evidence.

## Intake and authority

Intake is idempotent by evidence ID, exact source span, normalized content
fingerprint, and classifier version. Deterministic extraction uses bounded
sentence or paragraph spans and high-precision role rules. Merely mentioning a
category name does not place a record in that category. Uncertain input remains
`context` or `unclassified`; Forge must not force a semantic guess.

Automatic intake may classify, summarize, group, deduplicate, and link. It may
not approve, promote, reprioritize, mutate `MASTER_PROGRAM.json`, submit an
owner observation, or cross an authority gate. Promotion requires direct owner
authorization or a recorded delegation and an expected master-program revision.
Failed validation is atomic and produces no partial canonical write.

## Projection boundary

Plans, Ideas, Decisions, Tasks, Research, Corrections, project-system views and
search results are generated references over these records. They are not
parallel physical sources of truth. Generated views carry their source revision
and can be deleted and rebuilt. Older classifier rows remain append-only and
recoverable. A classifier version becomes current only through a complete
generation receipt; a partial higher version never hides the last complete
projection. Indexed V4 search and rollback follow
`indexed-knowledge-v4-contract.md`.
