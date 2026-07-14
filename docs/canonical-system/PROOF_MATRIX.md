# Canonical Proof Matrix

Every canonical system advances only with a complete proof pack.

| Proof area | Required evidence |
|---|---|
| Architecture | Purpose, non-goals, dependencies, ownership, interfaces, and invariants. |
| Research | Reuse check, primary sources, alternatives, contradictions, and decision rationale. |
| Determinism | Seed/versioned inputs and reproducible output hashes or stable semantic equivalence. |
| Adversarial behavior | Invalid input, poison cases, corruption, drift, boundary, and hostile-path tests. |
| Integration | Contract tests with every direct dependency and affected shared system. |
| Cost | Runtime, memory, storage, generation time, cache residency, and maintenance complexity. |
| Perception | Rendered inspection, visual assertions, and structured owner review where player-visible. |
| Recovery | Failure state, known-good fallback, rollback, and retained evidence/artifacts. |
| Promotion | Explicit criteria, regression suite, previous standard, and exact next action. |

## Required test classes by layer

- **Universe/field:** deterministic reconstruction, cross-seed variation,
  address collision, frequency/aliasing, cache, and range tests.
- **Hierarchy/history:** lazy residency, promotion/eviction, delta replay,
  migration, and corrupt-save tests.
- **Significance/scheduler:** combat protection, hysteresis, starvation,
  dependency inversion, admission, deadline, cancellation settlement, stale
  output, fallback, deterministic pressure, cache-policy separation, and
  authority-negative tests.
- **Semantics/construction:** poison words, alternatives, causal explanation,
  connectivity, support, socket, collision, and recipe-replay tests.
- **Asset/animation:** silhouette, recognisability, material boundaries,
  topology, contact, articulation, LOD, and cost tests.
- **Forge/control:** source gap, authority forgery, replay, dependency stage,
  candidate quarantine, rollback, and recovery tests.

Estimated or simulated thresholds must be labelled as such. Engine-specific
performance claims remain provisional until the final runtime-adapter phase.
