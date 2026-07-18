# Worker Telemetry and Efficiency Module

**Priority:** high. The module measures whether Forge workflow is improving. It
is advisory only: no metric or graph can grant authority, change priority,
advance a milestone, or promote code.

## Five-layer architecture

1. **Immutable event ledger** — versioned events, one writer, replayable.
2. **Trace hierarchy** — `project -> work package -> batch -> step -> tool or
   verification`, connected by correlation IDs.
3. **Metric registry** — versioned definitions, units, denominators,
   exclusions, uncertainty, sample-size threshold, and Goodhart-risk note.
4. **Derived projections** — deterministic local SQLite views/snapshots; raw
   events remain canonical and projections may be rebuilt.
5. **Read-only trend view** — bounded queries and graphs for calibration,
   verified completion, rework, idle delay, and diminishing returns.

## Event contract

Every event records schema version, event ID, trace/parent ID, event type,
timestamp window, route (system/group/contract), work-package/batch ID,
outcome, evidence references, and privacy/cardinality class. Large or sensitive
content is represented by retained evidence IDs/hashes, never copied into graph
labels.

Required event types: `batch_started`, `step_completed`, `tool_completed`,
`verification_completed`, `batch_blocked`, `batch_completed`,
`governance_change_proposed`, `governance_change_verified`, and
`projection_rebuilt`.

## Metric guardrails

- Metrics measure verified outcomes and rework alongside time/activity; no
  count-only metric is a productivity score.
- Every metric has an explicit denominator and `insufficient_sample` state.
- IDs, paths, prompts, free text, and unbounded error strings are prohibited as
  metric dimensions; they remain event references.
- Estimated, simulated, and measured values are never merged silently.
- Trends are recommendations requiring normal governance, not automatic action.

## Local storage and projection rules

- SQLite event tables use append-only inserts and indexed bounded reads.
- Reader projections are short-lived snapshots; no dashboard query may hold a
  long transaction that prevents WAL checkpointing.
- Schema migration adds a new event version and replay/migration fixture;
  historical events are never rewritten.
- SQLite window functions may calculate rolling estimate error and trend views;
  no external telemetry service is required for the first version.

## Required fixtures

1. Event idempotency, ordering, trace parent validity, and replay.
2. Malformed/unknown version and migration/rejection behavior.
3. Cardinality/privacy rejection for prohibited metric dimensions.
4. Metric denominator-zero and insufficient-sample behavior.
5. Projection rebuild equals event-derived expected metrics.
6. Read-only dashboard mutation-negative, bounded-query, and stale-snapshot
   behavior.
7. Goodhart guard: high activity with failed/reworked output cannot improve the
   primary verified-outcome recommendation.

## Phased implementation

1. Event/metric registry contracts and fixtures.
2. SQLite append-only BatchEvent persistence with replay tests.
3. Deterministic metric projection and calibration queries.
4. Read-only Forge dashboard trend view.
5. Advisory cadence/planning recommendations with governance receipts.

## Implemented B4 slice

Phases 1-3 now have a bounded local implementation: the v1 contract, SQLite
append-only persistence/replay, registered metric validation, and deterministic
advisory projection. Focused fixtures cover the required replay,
privacy/cardinality, denominator/sample, projection, bounded-read,
mutation-negative, and Goodhart cases. Dashboard trends and governance
recommendations remain later read-only consumers, not B4 closure requirements.

## Owner-approved dashboard and recommendation continuation

The owner released the remaining B4 phases as one bounded Forge-workflow-first
package. It measures meaningful worker batches and registered routine runs,
projects them into one read-only desktop Metrics tab, and produces advisory
local improvement trials. It does not instrument Mind Warp runtime performance,
invent unavailable history, calculate currency cost, or automate an
improvement decision.

The continuation must preserve every v1 event byte; distinguish measured,
estimated, unknown and not-applicable values; collect only metadata from local
Codex token-count records; register routine runs before invocation; and compare
only compatible metric, run-definition, module, platform and gate scopes.
Prompts, paths, raw logs and errors remain prohibited metric dimensions.

The first dashboard opens with `insufficient_sample`. Five comparable samples
establish a local baseline. A cost or flow regression may become a proposed
experiment only when it is at least 20 percent, verification does not improve,
and the proposal records expected verified local gain, total cost, uncertainty,
regression guard, falsifier, rollback trigger and stop/refocus condition.
Every projection and recommendation remains advisory and incapable of mutation,
approval, priority change, promotion or owner-gate traversal.
