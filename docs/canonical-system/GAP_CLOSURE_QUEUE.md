# Macro-to-Micro Gap Closure Queue

Documentation is not closure. Each item closes only when its evidence and test
gate pass.

| Priority | Gap | Group boundary | Closure evidence and test | State |
|---:|---|---|---|---|
| 1 | Compiler source completeness/format drift | capture -> compiler -> bootstrap | versioned long corpus, source-gap receipts, replay/migration tests | eligible local closure |
| 2 | Unsafe controlled-application boundary | candidate -> staging -> filesystem | hostile path, symlink, env, network, process, crash, rollback tests | eligible local closure |
| 3 | Research provenance/contradictions | source -> claim -> control plane | source/claim/contradiction fixtures and authority-negative tests | eligible local closure |
| 4 | ProofReceipt storage semantics | kernel -> projection -> inspector | owner decision plus contract/validator/recovery tests | owner-gated |
| 5 | Canonical deterministic roots | identity -> field -> hierarchy | selected identity/numeric policy plus fixed-vector tests | design-gated |
| 6 | Shared runtime-independent optimisation | hierarchy -> significance -> scheduler | pressure/cancellation/starvation/thrash simulation | depends on 5 |
| 7 | Causal content chain | world/history -> semantics -> construction -> assets | typed causal/recipe/validator fixtures and structured review | depends on 5 |

## Current focus

Start with compiler source completeness because it protects every future task's
context and is locally testable without an engine, connector, or owner design
choice. Before implementation, inventory existing parser fixtures and identify
the smallest missing long-corpus/source-gap test.
