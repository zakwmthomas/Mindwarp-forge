# Worker Optimization Protocol

Forge automation is itself a maintained system. After any repeated micro-task
pattern, wasted wake-up, lost context, premature stop, verification failure, or
useful prompt improvement, record the observation in the active batch state and
decide whether it belongs in the automation prompt, AGENTS guidance, a policy,
or a testable tool guard.

Changes must state the failure mode, expected benefit, scope, and a way to
detect regression. Prefer durable batch state, explicit exit criteria, repair
loops, and verified closure over more frequent narration. Reassess the worker
prompt at material package boundaries and when the owner identifies friction.

An acknowledgement-only wake-up is wasted work, not a completed batch. It must
either continue to the next eligible package or remain silent; it may notify
only for a new durable result, a verification result, or a genuine recorded
blocker.

## Immediate owner-wait suspension

When scheduler control is available, the first recognized owner approval,
confirmation, or observation gate must create exactly one bounded chat handoff
and then pause the Forge heartbeat with
`tools/forge-heartbeat-control.ps1 -Mode pause` before another scheduled wake.
The handoff states the required input, consequence, reversible default, and why
automation stopped. A visual gate also includes one labelled side-by-side image
of the exact current reference and altered controls plus a plain answer format.
Create it with `tools/forge-chat-visual.ps1`: capture only the Forge window,
never the whole desktop, and never require the owner to open or memorize
separate screenshots. Screenshot delivery changes presentation only: never
infer owner judgement, preselect an answer, or submit a receipt.

Resume with `tools/forge-heartbeat-control.ps1 -Mode resume` only after new
user-authored input materially resolves or explicitly releases the exact gate.
An unrelated message, elapsed time, a generated summary, or captured evidence
does not resume the scheduler and never grants authority.

## Five-wake owner-wait safety fallback

When the same owner approval, confirmation, or observation gate remains
unchanged for five consecutive heartbeat wakes, stop reloading and narrating
that gate. On the fifth wake, inspect `MASTER_PROGRAM.json` for a different
dependency-ready item whose full dependency closure is already satisfied and
whose work does not cross the waiting gate, any descendant of that gate, or an
owner, design, engine, authority, spending, credential, publishing,
protected-Kernel, security, or destructive boundary. If one exists, checkpoint
the waiting package without changing its decision state and execute the
highest-leverage safe independent package. If none exists, emit one
deduplicated escalation and pause/delete the heartbeat when scheduler control
is available; otherwise record the unavailable control once and remain quiet.

Never treat elapsed wakes as approval, never infer owner input, and never
reorder a dependency chain merely to stay busy. An explicit owner instruction
may exempt the current wait without weakening this default for future gates.

## Return-on-context rule

Every worker chooses the largest safe contiguous batch that can be completed
and verified with the available context, tokens, and tools. Avoid repeated
status narration, rediscovery, and single-edit turns. Preserve a compact batch
state so later turns resume directly. Promote a communication or workflow
improvement into policy, guidance, a template, or a test only after recording
the problem it solves and how future work will detect regression.

## Three-batch optimization audit

Count only completed meaningful batches, never heartbeat wake-ups or individual
edits. After every third completed batch, run and record an optimization audit:
compare planned versus completed exit criteria, redundant narration/tool work,
verification failures and repair loops, context rediscovery, handoff quality,
and whether the batch state let work resume directly. Keep improvements that
have evidence; revert or revise changes that did not improve the workflow.

## Adaptive recovery cadence

Follow `SYSTEM_EFFICIENCY_AUDIT.md`. Choose cadence from observed package
duration, idle delay, interruption, and context cost—not a fixed range. Change
only incrementally and retain the measurement, rationale, and regression signal.
