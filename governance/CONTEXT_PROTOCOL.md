# Context Continuity Protocol v1

Every package close and yellow/red context threshold produces a capsule containing:

- active objective and work item;
- package status and verification evidence;
- locked decisions and open risks;
- authority/delegation state;
- exact next action;
- references to source events and contracts.

Context health is `green`, `yellow`, or `red`. Yellow triggers a capsule before opening unrelated work. Red stops expansion and requires a handover capsule before continuation. Capsules are summaries, never replacement truth; the event ledger is authoritative.

Every material substage transition also refreshes two bounded views before
implementation. Macro context names the master objective, dependency route,
authority boundary, neighbouring contracts, and cross-system risks. Micro
context names the exact inputs, invariants, known failures, fixtures, tests,
and next action. Store the refresh in the sole active Worker Batch State with a
stage ID exactly equal to `substage_id`; changing stages without refreshing
that record fails verification.
