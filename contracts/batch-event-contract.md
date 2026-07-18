# BatchEvent Telemetry Contract v1

`BatchEventRecord` is immutable, append ordered, local SQLite telemetry. It
records a version, stable event/trace/parent identifiers, bounded route and
batch identifiers, event type, timestamp window, outcome, evidence references,
privacy/cardinality classes, and an optional registered metric sample.

Raw prompts, paths, free text, errors, credentials, and other unbounded or
sensitive values are never metric dimensions. Metric dimensions are limited to
`module`, `result_class`, and `measurement_source`, with bounded values. Event
evidence is retained by reference. Unknown schemas, event types, outcomes,
metrics, unbounded reads, invalid timestamp windows, sequence gaps, conflicting
idempotency retries, missing parents, and cross-trace parents fail closed.

The deterministic projection counts completed, verified, failed/blocked, and
reworked batches. A batch is verified only when a passed verification event and
a completed terminal event coexist without a retained failure/blocker. A zero
denominator remains `None`; fewer than five completed batches remains
`insufficient_sample`. High tool/activity counts cannot overcome failed output.

Telemetry and its projections are advisory evidence only. They cannot approve,
promote, execute, apply, change priority, advance a milestone, or mutate Forge
Kernel events/candidates. B5 experiment, transfer, and promotion semantics are
not part of this contract.

## Version 2 measured-run extension

Version 2 preserves the v1 record shape and append/replay rules while adding
registered event types for `routine_run_completed`, `metric_observed`,
`criterion_completed`, and `criterion_verified`. A routine run uses its trace
ID as the run ID and retains its parent worker batch in `batch_id`; it never
counts as a completed worker batch.

Registered raw observations include token categories, millisecond cost/flow
durations, exit-criterion counts, gate counts, repairs, interruptions and
rework. New bounded dimensions are `run_definition`, `platform`,
`verification_scope`, and `metric_version`. A missing or reset source produces
an `unknown` event with no metric value or unit. Historical v1 records are never
rewritten or backfilled.

Local receipt intake accepts only sequence-zero schema-v2 metadata-only JSON;
the SQLite ledger assigns the canonical sequence, rejects conflicts, and makes
identical retries idempotent. Dashboard reads are bounded, newest-first queries
returned in canonical ascending order, and remain mutation-negative.
