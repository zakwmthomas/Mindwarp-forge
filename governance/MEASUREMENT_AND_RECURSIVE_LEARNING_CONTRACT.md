# Measurement and Recursive Learning Contract

## Principle

Forge measures work to improve decisions, never to manufacture a single
productivity score. Numbers are valid only with a defined unit, denominator,
baseline, collection method, uncertainty, and interpretation guard.

## Measurement ledger

Every meaningful batch and improvement trial records, where measurable:

| Dimension | Required record | Interpretation guard |
|---|---|---|
| Delivery | planned versus completed exit criteria | Count only verified criteria as complete |
| Quality | verification result, failures, repair loops, regressions | Passing count alone is not quality |
| Cost | context/tokens, tool time, machine time, owner attention | Mark unavailable values as unknown, never zero |
| Flow | elapsed active time, idle/recovery delay, interruption count | Separate external wait from active work |
| Learning | baseline, hypothesis, expected gain, observed gain, uncertainty | Compare only locally compatible trials |
| Reversal | rollback, reverted change, rework cause, recovered loss | A reversal is evidence, not hidden failure |
| Safety | authority-negative checks and boundary violations | No efficiency gain offsets a safety regression |

## Effectiveness is a vector, not a percentage

There is no honest universal "2% effective" number. A batch receives a local
scorecard: verified delivery, quality/regressions, total cost, flow loss,
learning confidence, and safety status. A percentage may be shown only for a
single defined metric with its denominator (for example, verified closure rate
or estimate calibration error). The decision record must state which dimension
changed and what trade-off was accepted.

## Federated measurement architecture

Forge uses two connected learning layers:

1. **Shared measurement basis:** common event vocabulary, provenance,
   experiment/rollback protocol, cost categories, metric-definition rules,
   uncertainty labels, and safety/Goodhart guards. This makes evidence from
   different modules comparable at the level of *method*, not outcome.
2. **Local measurement basis:** every bounded module defines its own baseline,
   objective, valid denominators, fixture set, cost profile, quality threshold,
   and stop rule. This is where effectiveness is actually judged.

The shared layer may identify a transferable pattern (for example, a better
batch-sizing method or a regression guard), but it must submit that pattern to
a target-local experiment. It may not average local scores, rank unrelated
modules on one productivity scale, or override a local regression because a
global aggregate improved.

Cross-module reports therefore show a **portfolio**, not a league table:
per-module scorecards, confidence, unresolved risks, and explicitly scoped
reusable candidates. An aggregate is permitted only for identical metric
definitions, compatible modules, and an accompanying per-module breakdown.

## Improvement experiment record

Before changing a workflow, prompt, tool, or module strategy, record:

1. local baseline and sample window;
2. hypothesis and expected gain by dimension;
3. implementation and operating cost budget;
4. fixture or comparison method, uncertainty, and confounders;
5. quality, safety, and anti-Goodhart guards;
6. promotion threshold and rollback trigger; and
7. stop/refocus condition.

Afterward, record actual values, comparison result, failures/rework, and one
of `retain`, `revise`, `rollback`, `quarantine`, or `refocus`. A result with
insufficient samples is explicitly inconclusive, not a win.

## Recursive-learning rules

- A proven improvement becomes reusable only with its scope and counterexamples.
- A shared method is a candidate; each local basis must independently measure
  it before retaining it. Negative transfer in one target is a failed local
  trial even if other targets improved.
- A regression, rollback, or worse total-cost result is retained in the learning
  ledger and reduces confidence in the same strategy elsewhere.
- Metric definitions, raw event evidence, projections, and recommendations are
  separate layers; projections can be rebuilt and recommendations never grant
  authority.
- At each audit, rank alternatives by expected *verified local gain* against
  total cost and risk. Stop the current line when marginal gain is below cost,
  quality/safety degrades, or evidence remains inconclusive.

## Instrumentation state

B4 persists bounded append-only `BatchEvent` telemetry and computes a
deterministic local scorecard projection. B5 now retains local experiments,
results, decisions, transfer candidates, and target-local gates with rollback
and aggregate-masking guards. Unknown token/time values remain unknown rather
than zero, and every projection/assessment remains advisory evidence.
