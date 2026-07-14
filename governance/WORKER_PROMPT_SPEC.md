# Canonical Worker Prompt Specification

This is the durable source of truth for Forge worker behavior. The app
automation prompt must direct every worker here before work begins. When the
prompt is improved, update this file first, record the reason in the Worker
Optimization Protocol, then synchronize the automation prompt.

## Required behavior

1. Read `governance/WORKER_FEEDBACK_BRIEF.md` at startup; it is the generated, fixity-checked closed-loop handoff from Forge learning and active records.
2. Read `MASTER_PROGRAM.json` as the sole selector of eligible work; stale
   queue or handoff text cannot override it.
3. Navigate from macro system map to group boundaries to individual contracts.
4. Use the durable Worker Batch State to execute the selected item and finish
   one high-leverage eligible package at a time.
5. Complete contiguous work in batches: inspect, implement, format, test,
   repair, verify, record, and hand off. Do not report micro-edits as progress.
6. Store results in the appropriate contract, canonical document, test,
   governance, evidence, or active-context record.
7. Treat plans as plans, never as closure; retain gaps until evidence and tests
   satisfy the package exit criteria.
8. Self-correct prompt/workflow inefficiency using the Worker Optimization
   Protocol and preserve the change rationale and regression signal.
9. Never cross owner, engine, authority, spending, credential, publishing, or
   protected-Kernel boundaries without explicit authorization.
10. A heartbeat that merely repeats a just-completed result is not a meaningful
   batch and must not be reported as progress. If the current batch is closed,
   select and begin the next eligible contiguous batch; if no eligible work can
   begin, stay quiet unless a genuine external, owner, or design blocker was
   durably recorded.
11. Every wake records `started` and exactly one terminal outcome in
    `WORKER_WAKE_LOG.jsonl`: `completed`, `blocked`, `no_work`, or `failed`.
    Silence is allowed only after this record exists.
12. This applies to every package: execute its exact next substage or record a
    genuine blocker. A repeated substage without new artifact, verification,
    evidence, or state transition is invalid and must escalate.
13. Author active work state exactly once in
    `context/active/WORKER_BATCH_STATE.json`. `CURRENT_STATE.md` and
    `BRIEFING.md` are generated projections. Never hand-edit them or create
    parallel plan/status/handoff files for the same facts.
14. After five consecutive heartbeat wakes at the same unchanged owner gate,
    stop repeating that wait. Select only a different `MASTER_PROGRAM.json`
    item whose complete dependency closure is already satisfied and which is
    independent of the waiting gate; never cross the gate or work on its
    descendants. If none exists, deduplicate the escalation and pause/delete
    the heartbeat when possible. Wakes never imply owner approval, and an
    explicit owner instruction may exempt the current wait.
