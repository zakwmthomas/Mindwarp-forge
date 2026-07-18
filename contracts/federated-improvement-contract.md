# Federated Improvement Protocol Contract v1

Forge retains immutable local `ImprovementExperimentRecord`,
`ImprovementResultRecord`, and `ImprovementDecisionRecord` rows plus bounded
cross-module `TransferCandidateRecord` and `TransferGateRecord` rows. The
shared layer standardizes evidence and comparison mechanics; every module keeps
its own baseline, fixture, objective, validity rule, costs, regression guard,
falsifier, rollback trigger, stop condition, result, and decision.

A transferable candidate requires a registered metric, positive source-local
result, retained source decision, counterexamples, and non-applicable scope. A
target gate derives compatibility from method, input contract, metric name,
unit, denominator, and validity rule. Matching names with different semantics
fail before eligibility. Eligible reuse requires a fresh improved target-local
result. A regressed result must have a retained rollback/quarantine decision
before its rejection gate can be recorded.

Cross-module assessment is a portfolio, never an average. It requires two
independently successful modules for a reusable candidate and rejects the
candidate when any participating target regresses, regardless of aggregate
gain. Shared telemetry/projection failure cannot block local experiment,
result, or rollback persistence.

These records are advisory evidence. `retain`, `reusable_candidate`, or
`escalate` does not approve, promote, execute, apply, alter policy, or grant
owner authority. F5, runtime, engine, spending, credential, publishing, and
protected-storage decisions remain outside this contract.

## Step-leader integration

The capability-free step-leader controller may nominate a bounded local probe
only after it receives a complete fit assessment for every registered system,
a qualifying hybrid trigger, a positive target-local value-of-information and
local-net-gain result, and a budget no greater than ten percent of the prior
three meaningful batches or one normal batch, whichever is smaller. Its output
is advisory evidence; it does not execute the probe or write these records.

After a nominated probe is performed by an authorized worker, its local result
enters this protocol through the existing experiment, result, and decision
records. Any target regression blocks reuse and requires quarantine or
rollback. One successful target may be retained locally; two independent
successful modules are required before a transfer candidate can be published.
Aggregate gain never masks a local loss, and ordinary edits or heartbeat wakes
do not create a step-leader divergence trigger.
