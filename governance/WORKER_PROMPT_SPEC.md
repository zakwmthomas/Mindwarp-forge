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
15. Do not wait five wakes when scheduler control is available. At the first
    recognized owner-input gate, prepare one bounded chat handoff and run
    `tools/forge-heartbeat-control.ps1 -Mode pause` before another scheduled
    wake. For a visual gate, use `tools/forge-chat-visual.ps1` to capture only
    the actual Forge window and send one labelled side-by-side image of the
    exact reference and altered controls plus a plain response format in chat;
    never send the whole desktop, require the owner to switch between files,
    or infer or submit the observation. Resume with `-Mode
    resume` only after new user-authored input materially resolves or explicitly
    releases that exact gate. Unrelated owner chat does not resume automation.
16. Before each material substage, refresh and record both context scales. The
    macro refresh covers the master objective, dependency route, authority,
    neighbouring contracts, and system risks. The micro refresh covers exact
    inputs, invariants, known failures, fixtures, tests, and next action. The
    recorded stage ID must equal the active Batch State substage.
17. Visually inspect the actual pixels of every visual asset before using it as
    a reference, target, fixture, or candidate. Record provenance, useful-scale
    views, fitness, accuracy limits, and visible defects. A human reference must
    be anatomically credible, sufficiently complete and unobscured, and fit in
    pose, view, lighting, and detail for the comparison. A filename, hash, or
    successful decode is not a quality check. If fitness or creative intent is
    uncertain, stop at one bounded owner visual check; never substitute a poor
    or arbitrary human reference.
18. Declare the cheapest sufficient proof tier before implementation. Prefer
    static reasoning, typed models, in-memory fixtures, and disposable
    simulations before bounded PC or external runs. Escalate only with retained
    lower-tier results, unresolved risk, expected information gain, cost, and a
    stop condition. Still run the required final integration gate.
19. Treat nature-inspired mechanisms as scoped hypotheses. Before applying one,
    state its mathematical abstraction, assumptions, local metric, simple
    baseline, cost, falsifier, counterexample, and non-applicable scope; run a
    deterministic disposable comparison and the target-local transfer gate.
    Never infer correctness, universality, fitness, or authority from natural
    resemblance, and never transfer domain objectives, heuristics, parameters,
    or conclusions as a shared protocol.
20. When the owner supplies an interesting mathematical, natural,
    architectural, or systemic mechanism without explicitly limiting scope,
    run a whole-system discovery audit before localization. Extract separable
    abstractions; enumerate every registered Forge, game-canonical, and runtime system;
    record fit, non-fit, duplication, possible shared utilities, and
    cross-layer feedback; then rank bounded target-local trials under P16.
    Broad mapping is not universal adoption, and no implementation, parameter,
    objective, conclusion, dependency, or authority transfers without local
    evidence and its normal gate.
21. Before working in any declared module, read its root `MODULE.md`. If the
    work changes purpose, ownership, non-goals, interfaces, dependencies,
    invariants, risks, verification, or canonical references, update the
    canonical module-context registry and regenerate every affected front door.
    Missing or stale module context fails the package gate.
22. At a substage transition, review every handoff-critical Batch State section
    and update its schema-3 `handoff_section_receipts` entry individually with
    the active stage, exact content hash, `revised` or `carried_forward`
    disposition, and a specific review note. Never recompute or bulk-retag
    receipts merely to satisfy the verifier. Bootstrap must fail for an old
    schema or any missing, unknown, stale, malformed, or content-mismatched
    receipt. Receipt hashes prove post-review fixity, not semantic correctness.
23. Use hybrid step-leader divergence: audit after every third meaningful
    completed batch, or earlier for a verification failure, two repeated
    workarounds, milestone change, stagnation, or high-leverage input fitting
    at least three registered systems. Preserve the exact mainline checkpoint,
    complete the P18 map, spend at most ten percent of the prior three-batch
    cost and no more than one normal batch, and run at most one positive local
    probe. Ordinary edits and heartbeats do not trigger it. Any target regression blocks reuse;
    every outcome resumes mainline and grants no
    execution or authority.
24. Every Forge session is read-only by default. Use the exact
    `CODEX_THREAD_ID` to run `tools/forge-writer-lease.ps1 -Mode route` and
    then `-Mode claim` before any repository mutation. Reassert the
    checkpoint-bound lease before each material mutation or long verification
    run. A missing, expired, stale, conflicting, or mismatched lease forbids
    writes. Release the lease at completion or an owner gate; routing and a
    lease never grant approval, promotion, or owner authority.
25. Use one parent writer and zero specialists by default. Add at most two
    concurrent read-only specialists only for distinct unresolved questions
    whose expected information gain exceeds their context and token cost;
    never duplicate an inventory or broad review. Give each specialist exact
    files, a stop condition, and a compact return schema. During package
    closure, run the smallest focused gate set, then a programmatic
    route-shield inventory, then one complete Forge gate after the route is
    stable. Retain concise checksums, counts, timings, and failure
    classifications instead of repeating raw logs. This sequencing never
    reduces bootstrap, hostile, platform, owner, provenance, or final
    integration requirements.
26. Measure meaningful Forge work at its real boundaries. Run
    `tools/forge-batch-metrics.ps1 -Mode start` when a bounded batch begins and
    `-Mode finish` at its terminal checkpoint, using the exact
    `CODEX_THREAD_ID`; missing or reset counters remain unknown and cached input
    remains a subset of input. Invoke registered routine gates through
    `tools/invoke-measured-run.ps1` so duration and outcome become bounded local
    receipts. Metrics are advisory evidence only: they never replace a gate,
    authorize a change, or make an unverified criterion complete.
