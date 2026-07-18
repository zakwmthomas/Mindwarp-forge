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

## Token-efficient package execution

Use one parent writer and zero specialists by default. Use at most two
concurrent read-only specialists only when they own distinct unresolved
questions and their expected information gain exceeds their context and token
cost. Never commission duplicate inventories or broad reviews. Bound every
specialist with exact files, a stop condition, and a compact return schema, and
keep its output in the parent result unless it creates a distinct reusable
evidence class.

Close a stable package with the smallest focused gate set, a programmatic
route-shield inventory, and then one complete Forge gate. Retain concise
checksums, counts, timings, and failure classifications rather than raw-log
repetition. This order changes cost and timing only: it never removes or
weakens bootstrap, hostile, platform, owner, provenance, or final integration
requirements. If the default misses a material contradiction that a bounded
independent review would probably have found, restore one targeted specialist
for that risk class.

## Simulation-first execution ladder

Before implementation, choose the cheapest tier capable of falsifying the
current assumptions: static reasoning, typed model, in-memory fixture,
disposable simulation, bounded integrated PC test, then external execution.
Do not skip a cheaper applicable tier. Moving upward records the lower-tier
result, unresolved risk, expected information gain, execution and context
cost, regression guard, and stop condition. A cheap pass improves the starting
point but never replaces a required final integrated verification.

## Visual reference fitness

Technical validity is not visual fitness. Inspect actual rendered pixels at a
useful scale before any visual asset becomes a reference, comparison target,
fixture, or candidate. Record source identity and provenance, views inspected,
visible defects, intended comparison, accuracy limitations, and disposition.
For a human comparison, reject anatomically incoherent, badly occluded,
insufficiently detailed, misleadingly posed or lit, or visibly corrupted
references. If fitness or the intended aesthetic target remains uncertain,
route one labelled comparison to the owner and pause the dependent work.

## Natural-method candidate gate

Before localizing an owner-supplied reusable mathematical, natural,
architectural, or systemic mechanism, run a **whole-system discovery audit**.
Extract separable abstractions, enumerate every registered Forge,
game-canonical, and runtime system, and record fit, non-fit, duplication,
possible shared utilities, and cross-layer feedback. Rank experiments only
after that complete registry pass. Broad discovery is not universal adoption:
the P16 target-local transfer gate still owns every application, and no
implementation, parameter, objective, conclusion, dependency, or authority
transfers from the map. Skip the whole-system pass only when the owner
explicitly limits the scope.

Translate every nature-inspired proposal into a mathematical or engineering
abstraction before treating it as a method. Record assumptions, the receiving
domain's metric and constraints, a simple baseline, implementation and
operating cost, a falsifier, at least one counterexample, and non-applicable
scope. Run deterministic disposable cases first, then the Federated
Improvement Kernel's target-local transfer gate. Natural resemblance is never
evidence of correctness, efficiency, universality, fitness, or authority. A
shared experimental protocol may transfer; objectives, resistance heuristics,
parameters, and conclusions remain local.

## Three-batch optimization audit

Count only completed meaningful batches, never heartbeat wake-ups or individual
edits. After every third completed batch, run and record an optimization audit:
compare planned versus completed exit criteria, redundant narration/tool work,
verification failures and repair loops, context rediscovery, handoff quality,
and whether the batch state let work resume directly. Keep improvements that
have evidence; revert or revise changes that did not improve the workflow.

## Hybrid step-leader divergence

After every third meaningful completed batch, or earlier on a verification
failure, two repeated workarounds, milestone transition, stagnation, or a new
mechanism applicable to at least three registered systems, run one bounded
step-leader assessment. Preserve the exact mainline checkpoint. Complete the
P18 registry map, calculate target-local VOI and local net gain, and select at
most one positive target under the capability-free controller.

The proposed probe may consume no more than ten percent of the measured prior
three-batch cost and no more than one normal batch. Repository evidence comes
before capped external research. Ordinary edits and heartbeat wakes are not triggers.
Missing registry coverage, failed budget, non-positive value, a
fired falsifier or any participating regression reconnects to mainline without
adoption. Two independent successful modules are still required before B5 may
publish a transfer candidate. No controller decision executes work or grants
authority.

## Adaptive recovery cadence

Follow `SYSTEM_EFFICIENCY_AUDIT.md`. Choose cadence from observed package
duration, idle delay, interruption, and context cost—not a fixed range. Change
only incrementally and retain the measurement, rationale, and regression signal.
