# B4 BatchEvent Telemetry Readiness

**Status:** Complete. Focused implementation and the full Forge gate pass.

## Implemented boundary

- append-only SQLite `BatchEventRecord` rows with stable replay order;
- exact idempotent retry and conflicting-ID rejection;
- trace-parent existence and same-trace validation;
- schema, event vocabulary, timestamp, route, outcome, evidence, privacy, and
  cardinality validation;
- registered metric names and bounded dimension names/values;
- bounded event reads and a deterministic advisory scorecard projection;
- explicit denominator-zero and `insufficient_sample` states;
- failed/blocked and rework retention so activity cannot manufacture success.

## Focused fixtures

Three kernel tests cover reopen/replay, ordering, idempotency, collision,
sequence gaps, missing/cross-trace parent rejection, unknown schema/metric,
private path dimension rejection, query bounds, Kernel mutation-negative
behavior, zero denominator, deterministic rebuild, and high-activity failed
output. The denominator-zero fixture found an eager-evaluation division defect;
the projection now branches before division.

The complete Forge gate passes with the UI build, Rust build, 15 desktop tests,
59 kernel tests, worker/governance proof, modularity proof, and whitespace
checks. This does not implement B5 federated experiments or grant any
owner/runtime/protected-Kernel authority.
