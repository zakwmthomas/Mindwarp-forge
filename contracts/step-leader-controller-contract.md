# Step-Leader Controller Contract

Version: 1  
Status: additive capability-free advisory reference

## Purpose

The controller preserves a named mainline checkpoint while periodically
testing whether a bounded divergence could produce more verified progress than
its research, implementation, migration, recurring and complexity cost. It
implements the workflow analogy of a lightning step leader without treating
natural resemblance as evidence.

It validates supplied candidate abstractions and complete per-system
assessments, calculates value of information and expected local net gain,
selects at most one bounded target-local probe, and records the checkpoint to
resume afterward. It never performs research or implementation.

## Hybrid trigger and budget

A probe becomes eligible after three completed meaningful batches, or earlier
for a verification failure, two repeated workarounds, a milestone boundary,
stagnation, or a high-leverage new input applicable/test-only to at least three
systems. Ordinary edits and heartbeat wakes are not triggers.

Probe cost must be positive, no greater than one normal batch, and no greater
than ten percent of the prior three batches. External sources are separately
capped. A budget failure returns `defer_budget`.

## Complete mapping and local mathematics

Every registered canonical system needs exactly one assessment. Missing or
duplicate rows, unknown systems, incomplete abstractions, absent baselines,
counterexamples, non-applicable scope or falsifiers, and malformed probability
portfolios fail closed.

`VOI = sum(P(outcome) * best_local_utility_after) - current_best_utility - probe_cost`

`LocalNetGain = closure_gain + quality_gain + future_cost_saved - implementation_cost - migration_cost - recurring_cost - complexity_risk_cost`

Only targets with strictly positive VOI and LocalNetGain are ranked. Units and
utilities remain target-local and unlike systems are never averaged.

## Reconnection, authority and rollback

Any participating regression quarantines the candidate. A fired falsifier or
non-positive observed gain rejects it. One positive result may be adopted only
through its local gate; publishing a transfer candidate requires two
independently successful modules and no regression.

Every decision resumes the exact saved checkpoint. The crate cannot research,
execute, approve, promote, change policy or game truth, select a runtime, spend,
use credentials, browse, persist P2P state or grant owner authority.

Removing the crate and its registrations restores manual P15/P16/P18 and
three-batch auditing without data migration or reinterpretation.
