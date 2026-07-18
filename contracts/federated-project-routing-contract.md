# Federated project and workstream routing contract v1

Status: owner-authorized additive Forge continuity component.

## Purpose and authority

Forge stores one federated vault of immutable evidence while keeping projects,
workstreams and authority lanes explicit. Routing is a recorded evidence
relationship. It never merges projects, approves work, transfers code, grants
credentials, publishes, spends, deletes evidence or grants game/runtime
authority.

V1 adds four strict record families: `ProjectRecord`, `WorkstreamRecord`,
`SessionRouteReceipt` and `CrossProjectLink`. All deny unknown fields and use
canonical deterministic JSON for durable comparison. Failed multi-record
writes are atomic.

## Project identity and aliases

A project has one stable domain-separated ID derived from its normalized
canonical alias and immutable initial repository URI. A repository relocation
must be represented by a later binding record; it must not silently change the
project ID. Display names may be broader, but V1 routing aliases are nonempty
ASCII letters and digits after lowercase normalization.

Project revisions begin at 1 and increase by exactly one. The canonical name,
alias set, initial repository URI, bootstrap entry point and authority boundary
are immutable in V1. Status may move `active -> paused -> active` or to terminal
`archived`. Related-project and evidence IDs are sorted, unique, nonempty and
append-only. An alias may belong to only one project across all revisions.

Active workstreams are derived from current workstream revisions rather than
duplicated into the project record.

## Workstream isolation and leases

A workstream belongs to exactly one registered project. Its identity and
project never change. Revisions begin at 1 and increase by exactly one. Stage,
status, authority lane, dependencies, blockers, checkpoint URI, next action,
lease and evidence may change only through a valid successor.

Dependencies and blockers are sorted, unique and nonempty. Self-dependencies,
cross-project dependencies without an evidence-only cross-project link, and
dependency cycles fail before commit. A live lease may be renewed only by its
current holder. Another holder may acquire it only after expiry. A stale
revision or lease conflict writes nothing.

## Session routing precedence

Routing methods are closed and ordered:

1. `explicit_owner_task_binding`;
2. `registered_repository_root`;
3. `explicit_project_alias`;
4. `deterministic_suggestion`; and
5. `unrouted_inbox`.

Only the first three may produce `routed`, and the selected project must be a
registered candidate. A selected workstream must exist in that project.
`deterministic_suggestion` is always `ambiguous` with no assignment;
`unrouted_inbox` is always `unrouted`. Candidate IDs are sorted and unique.
Text keywords and assistant guesses never select or create a project. Route
revisions are append-only and increase by exactly one.

Greenfield is the first acceptance fixture. Explicit aliases `Greenfield` and
`Greenfeld` resolve to one independent project; `Greenfields` does not. Its
relationship to Forge is evidence only. `mindwarp-game` remains an Atlas/system
horizon, not an independent project registration in V1.

## Cross-project links

Links name two different registered projects, one closed relation
(`reuse_candidate`, `dependency`, `contradiction`, `transfer_proposal` or
`related`), evidence, state and required target-local gate. Link IDs are
domain-separated from the ordered project pair and relation. A link never
transfers objectives, heuristics, conclusions, code or authority.

## Bounds and rollback

V1 admits at most 1,024 aliases, relationships, evidence references,
dependencies, blockers or route candidates per record, each string at most
4,096 UTF-8 bytes and each canonical record at most 4 MiB. Persistence rejects
larger records before allocation-intensive work.

Rollback disables V1 readers and projections and leaves the additive tables
and all source evidence intact. No legacy evidence, checkpoint or project
bytes are rewritten or deleted.

