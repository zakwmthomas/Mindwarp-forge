# System Efficiency Audit

The optimization target is verified useful progress per unit of context, token,
tool time, machine time, and owner attention—not raw wake-up frequency.

## Audit dimensions

1. **Batching:** avoid splitting contiguous work across turns.
2. **Context:** use layered navigation and durable batch state; avoid rediscovery.
3. **Prompting:** keep the canonical prompt concise, explicit, and regression-tested.
4. **Planning:** select work by dependency leverage, risk, reversibility, and
   expected closure value.
5. **Research:** use bounded questions and retained claims; stop at diminishing
   information return.
6. **Tool use:** batch compatible reads/verifications; avoid repeated status calls.
7. **Verification:** run proportional gates, repair failures before new work,
   and retain receipts.
8. **Communication:** report only material checkpoints; use durable records for
   detail.
9. **Cadence:** choose wake-up delay from observed work duration, idle cost,
   interruption cost, and context cost; no fixed range is inherently optimal.

## Diminishing-return rule

At each three-batch audit, rank proposed improvements by expected verified gain
divided by implementation/context cost. Apply only improvements above the
current threshold. Record rejected ideas and revisit only when conditions or
evidence change.

## Universal improvement measurement contract

This applies to workflow, research, compiler, telemetry, control-plane, and
future production modules alike. Before optimizing, record: the baseline,
target outcome, expected gain, implementation cost, recurring operating cost,
uncertainty, regression guard, and stop/refocus condition. After the bounded
trial, record the observed gain and cost with the same units where possible.

Do not collapse unlike outcomes into one global score. Compare each candidate
against its local baseline, then use a multi-metric decision: retained benefit,
quality/verification coverage, reversibility, resource cost, and regressions.
If the expected marginal verified gain falls below the measured cost, samples
remain insufficient, or a guard fails, stop the line of optimization and move
to the highest-leverage unresolved constraint.

## Cadence decision

Use the shortest delay that does not cause empty wake-ups, repeated context
loading, or interruption of an active meaningful batch. If a package routinely
finishes before the next wake-up, lower delay incrementally; if wake-ups produce
empty/micro cycles, raise it incrementally. The worker must record the observed
duration, decision, expected benefit, and regression signal before changing the
automation schedule.

## Owner-wait suspension trial

- **Baseline:** one prior F5 wait produced 120 consecutive `no_work` wakes; the
  current visual gate produced additional five-minute interruptions even after
  work was correctly deduplicated.
- **Target outcome:** zero scheduled heartbeat wakes between a recognized owner
  gate handoff and new user-authored input that resolves or releases that gate.
- **Expected gain:** remove every repeated bootstrap, selector, and narration
  cycle during the wait while preserving the exact dependency and authority
  boundary.
- **Implementation cost:** one small atomic status-control script, one fixture,
  canonical prompt/policy updates, and one automation status write per pause or
  resume.
- **Recurring cost:** negligible local TOML read/write at gate transitions;
  visual gates additionally require one bounded current-viewport capture.
- **Uncertainty:** the external scheduler must honor `status = "PAUSED"` without
  rewriting the automation record.
- **Regression guard:** never resume on unrelated chat, elapsed time, captured
  evidence, or generated summaries; never infer or submit owner input.
- **Stop/refocus:** if a heartbeat fires while the saved status remains
  `PAUSED`, stop prompt tuning and repair the scheduler integration instead.

## Planned-duration control

Before starting a meaningful batch, estimate the expected uninterrupted work
duration from scope, dependent tools, test cost, and recent comparable batches.
Record the estimate in Batch State. If the recovery cadence is materially
shorter than the estimate and would likely create redundant wake-ups, increase
it toward the estimate; if it is materially longer than a short verified batch,
decrease it toward the estimate. After the batch, record actual duration and
estimate error. Future cadence changes must use this estimate-versus-actual
history, not intuition alone.
