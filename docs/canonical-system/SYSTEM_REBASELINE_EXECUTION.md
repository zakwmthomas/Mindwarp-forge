# System Rebaseline: Approved Improvement Program

## Retain

| Area | Reason | Regression boundary |
|---|---|---|
| Truth Kernel | Content-addressed evidence, replay, recovery, and authority separation are locally proven | No evidence, metric, or projection grants authority |
| Compiler parsing | Labelled-message parsing, duplicate handling, long corpus, and approval-negative tests exist | Grammar remains bounded and explicit |
| Desktop shell | Read-only Atlas and local capture bootstrap build and test | No dashboard control path |
| Governance loop | Policy, feedback brief, P10, and federated measurement rules are durable | Records remain evidence or policy, never authority |

## Repair or complete

| Package | Gap | First measurable completion condition |
|---|---|---|
| A1/A2 source continuity | Chunk rows lack raw-byte envelope, version/order, and child linkage | Reopen/replay proves exact bytes and parse lineage |
| A3/A4 safety/recovery | Hostile boundary and recovery drill are unproven | Adversarial path/process/corruption fixtures pass |
| B4/B5 telemetry/improvement | No append-only events, local scorecards, or transfer gates | Projection replay and aggregate-masking rejection pass |
| B1-B3 evidence/orchestration | Research, control receipts, and inspector are incomplete | Traceability/lifecycle/mutation-negative fixtures pass |
| C1-C7 canonical production | Plans lack local proof harnesses | Per-module fixture and local-scorecard packs pass |

## Replace or defer

- Do not replace the tested Truth Kernel or select a runtime engine.
- Replace unsafe message-as-chunk provenance with `SourceChunkEnvelope`.
- Defer runtime adapters and all engine implementation until later gates.

## Program rule

Each package is additive, independently reversible, and measured against this
baseline. No broad refactor is valid without local verified gain, rollback, and
neighbour-regression coverage.

## F4 modularity evidence

The engine-neutral module graph is now declared in
`governance/module-boundaries.json`. Static verification rejects forbidden
imports, undeclared Cargo workspace dependencies, unknown nodes, missing
module roots, and dependency cycles while retaining diagnostics from multiple
failing modules. See `MODULARITY_READINESS.md` and
`contracts/module-boundary-contract.md` for the boundary and focused fixtures.
