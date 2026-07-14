# Improvement Cascade Protocol

## Purpose

Move a verified improvement through dependent and related modules without
forcing one module's model, parameters, or conclusions onto another.

## Cascade contract

Each retained improvement publishes: source module, method, assumptions,
evidence, compatible input/output shape, expected benefit, counterexamples,
cost, rollback receipt, and required guards. The dependency map identifies
potential targets; it does not prove compatibility.

## Receiving adapter

For every target, create a local adaptation record that maps the source method
to the target's own contract, data, metrics, fixtures, constraints, and
rollback boundary. The adapter may retain, transform, narrow, or reject the
method. It must never silently copy domain parameters or conclusions.

## Execution

1. Discover direct dependencies, reverse dependencies, and modules sharing a
   compatible learning basis.
2. Classify each target as compatible, adaptable, incompatible, or gated.
3. For compatible/adaptable targets, define a target-local baseline, mapping,
   falsifier, cost budget, and rollback route.
4. Run each target independently and retain per-target results.
5. Feed retained methods, adaptations, rejections, and negative transfer back
   into the portfolio and next worker feedback brief.

## Safety

The cascade is asynchronous, idempotent, and failure-isolated. It distributes
evidence and methods only; it cannot approve, promote, execute, choose an
engine, or override a local regression. A target can improve differently from
the source and still be a successful adaptation.
