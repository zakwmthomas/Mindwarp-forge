# Federated Universal Improvement Kernel

## Goal

Replace repeated module-specific learning machinery with one shared, engine-neutral **protocol**, not one global learner. Modules retain their own domain data, objectives, parameters, validity tests, and constraints, but use common evidence, hypotheses, experiments, metrics, regression tests, decision receipts, promotion, and rollback.

## Compression boundary

**Centralize:** event schema, trace/provenance, metric registry, experiment record, hypothesis/result model, diminishing-return audit, regression detection, policy routing, and rollback receipt.

**Do not centralize:** universe mathematics, field semantics, history deltas, asset rules, animation logic, rendering, module-specific validity tests, domain parameters, or a global objective score. The kernel learns *how to improve safely*, not one universal answer for every domain.

## Shared records

| Record | Purpose |
|---|---|
| `Observation` | Versioned event or measured condition with evidence references |
| `Hypothesis` | Claimed improvement, affected module, expected benefit, risk, and falsifier |
| `Experiment` | Controlled comparison, fixture/seed, metrics, cost budget, stop condition |
| `Result` | Outcome, uncertainty, regression signal, artifacts, and limitation |
| `ImprovementCandidate` | Proposed reusable rule, parameter, or strategy with scope |
| `DecisionReceipt` | Keep, revise, rollback, quarantine, or escalate; never self-authorizes owner decisions |

## Universal loop

`observe -> classify -> hypothesize -> experiment -> measure -> compare -> verify -> retain/revise/rollback -> publish reusable learning`

Each module exposes a small local adapter: input contract, valid metrics, fixtures, cost limits, prohibited actions, local state, and promotion boundary. The kernel supplies the loop and cross-module reuse only when a target-local transfer gate proves semantic compatibility.

Every loop records a local baseline, expected verified gain, implementation and
operating cost, uncertainty, regression guard, and predeclared stop/refocus
condition. The kernel never converts unlike module outcomes into one global
score; it compares each trial with its own baseline and retains the evidence
for stopping as carefully as evidence for promotion.

The Forge therefore has a shared **learning basis** (records, provenance,
measurement protocol, rollback, and transfer gates) and many local **learning
bases** (module objectives, fixtures, metrics, and decisions). Shared learning
proposes methods; local learning proves or rejects them. Cross-module views are
portfolios with per-module outcomes, never an average productivity league table.

## Cross-module reuse rules

- Reuse a learned rule only when its assumptions, input contract, metric definition, and uncertainty are compatible with the receiving module.
- Transfer *methods* only after a target-local trial; transfer *parameters or conclusions* only with a new experiment and local rollback receipt.
- A global improvement requires evidence from more than one applicable module and must retain module-specific counterexamples. Aggregate improvement never overrides a regression in an individual participating module.
- A shared projection, trend store, or registry may fail without preventing a local adapter from recording, testing, rejecting, or rolling back its own candidate.

## Rollout plan

1. Define contracts and fixtures for the records above.
2. Implement the protocol on the worker telemetry/governance path first.
3. Prove replay, local isolation, aggregate-masking rejection, incompatible-transfer rejection, and rollback.
4. Adapt conversation compiler, research, and control plane.
5. Adapt game-canonical modules only after their local proof harnesses exist.

## Tests

- deterministic replay of an improvement cycle;
- same-module regression rollback;
- incompatible cross-module transfer rejection;
- compatible method transfer with fresh local experiment;
- aggregate improvement with a single-module regression is rejected;
- shared telemetry/projection outage does not block local rollback;
- owner-gated global policy escalation;
- no metric, hypothesis, or result grants authority or executes code.

See `UNIVERSAL_IMPROVEMENT_KERNEL_ADVERSARIAL_REVIEW.md` for the adversarial case, research basis, and required transfer-gate fixture matrix.

## Implemented B5 slice

The shared record/transfer protocol is now implemented as append-only local
SQLite records. Compatibility is derived from method and metric contracts;
fresh target-local results and rollback evidence are mandatory. Portfolio
assessment rejects any target regression and cannot grant authority. Focused
fixtures cover replay, isolation, semantic mismatch, aggregate masking,
negative-transfer rollback, schema drift, shared-projection outage, and
Kernel mutation-negative behavior.
