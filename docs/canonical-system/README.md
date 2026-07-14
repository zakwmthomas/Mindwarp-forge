# Canonical System Registry

This directory is the Phase 0 source of truth for the engine-neutral Mind Warp
production system. It turns the approved master plan into a navigable map before
runtime or game-engine implementation begins.

Read in this order:

1. `system-registry.json` — canonical systems, dependencies, current evidence,
   and required proof.
2. `DEPENDENCY_MAP.md` — bottom-up ordering and cross-cutting Forge services.
3. `PROOF_MATRIX.md` — the proof pack required before a system can advance.
4. `UNRESOLVED_GAPS.md` — explicit unknowns and gates; never silently assume
   these are solved.
5. `SOURCE_AUDIT.md` — recovered evidence families and their limits.

6. `MACRO_GAP_CLOSURE_AUDIT.md` - per-system contracts, risks,
   observability requirements, smallest proofs, and F5 package ordering.
7. `PROOF_RECEIPT_READINESS.md` - bounded discovery for the first F5
   ProofReceipt and read-only inspector package.
8. `UNIVERSE_IDENTITY_READINESS.md` - fixed-vector harness boundary, source
   evidence, neighbour contracts, and unresolved identity decisions.
9. `FIELD_BASIS_READINESS.md` - field recipe/sample boundary, fixture matrix,
   numerical-policy gaps, and engine-neutral entry criteria.
10. `HIERARCHY_HISTORY_READINESS.md` - lazy descriptor, observation, baseline,
    delta, replay, and recovery boundaries for the paired proof package.
11. `SIGNIFICANCE_SCHEDULER_READINESS.md` - shared priority, budget, ticket,
    trace, adversarial-fixture, and observability boundaries.
12. `SEMANTIC_CONSTRUCTION_READINESS.md` - causal semantic, capability,
    part-role graph, construction-recipe, and validation boundaries.
13. `REPRESENTATION_ASSET_ANIMATION_READINESS.md` - neutral representation,
    artifact, material, articulation, temporal-fidelity, and review boundaries.
14. `F5_READINESS_GATE.md` - consolidated coverage, blocked decisions, ordered
    implementation sequence, and remaining F4 work.
15. `CONVERSATION_COMPILER_READINESS.md` - long-corpus, source-gap,
    format-drift, and authority-negative readiness requirements.
16. `RESEARCH_RECORD_READINESS.md` - bounded research brief, source, claim,
    contradiction, experiment, and receipt requirements.
17. `CONTROL_PLANE_READINESS.md` - lifecycle, gate, blocker, rollback, and
    bounded owner-brief readiness requirements.
18. `REFERENCE_STUDIO_READINESS.md` - verified local, read-only inspection and
    mutation-negative readiness requirements.
19. `FIXTURE_SOURCE_INVENTORY.md` - bounded routing from recovered evidence to
    new engine-neutral proof fixtures.
20. `UNIVERSAL_IMPROVEMENT_KERNEL.md` - shared improvement protocol boundary;
    it centralizes evidence and safety mechanics, not domain learning.
21. `UNIVERSAL_IMPROVEMENT_KERNEL_ADVERSARIAL_REVIEW.md` - falsification
    research, transfer-gate rules, and adversarial fixture matrix for the
    federated improvement protocol.
22. `F4_EXIT_AUDIT.md` - mechanical F4 completion evidence and retained owner
    boundary.
23. `F5_OWNER_GATE.md` - retained milestone/storage decision that activated F5.
24. `F5_PROOF_RECEIPT_DECISION.md` - selected receipt projection, failure
    matrix, and bounded authority lane.
25. `UNIVERSE_IDENTITY_DESIGN_GATE.md` - recommended identity invariant,
    standards, fixed vectors, failures, and exact confirmation gate.
26. `FIELD_BASIS_DESIGN_GATE.md` - researched numerical policy, bulk-generator
    alternatives, proof matrix, recovery rules, and exact confirmation gate.
27. `HIERARCHY_HISTORY_DESIGN_GATE.md` - immutable descriptor, finite window,
    append-only delta, conflict, snapshot, migration, and recovery decision gate.
28. `HIERARCHY_HISTORY_SYSTEM_ALIGNMENT_AUDIT.md` - whole-registry dependency
    reconciliation, external practitioner lessons, failure forecast, and the
    repaired P4 implementation plan.
29. `SIGNIFICANCE_SCHEDULER_DESIGN_GATE.md` - recovered prototype audit,
    primary-source reconciliation, shared-signal, admission, fairness,
    cancellation, overload, and exact P5 confirmation boundary.
30. `SEMANTIC_CONSTRUCTION_DESIGN_GATE.md` - recovered prototype audit,
    primary-practice reconciliation, causal/lexical separation, alternatives,
    capability closure, atomic graph replay, and retained P6/P7 boundary.
31. `REPRESENTATION_ASSET_ANIMATION_DESIGN_GATE.md` - recovered prototype and
    primary-practice audit, staged P7a contract/P7b perception boundary,
    derivative lineage, hostile-reference rules, temporal mapping, and owner gate.
32. `P7B_CONTROLLED_PERCEPTION_DESIGN_GATE.md` - controlled-stimulus,
    human-observation, metric-separation, tool-containment, and staged P7b-0
    owner gate.
33. `P7B1_CONTAINMENT_DESIGN_GATE.md` - local Windows feasibility,
    security-boundary comparison, hostile fixtures, output quarantine,
    recovery, and the staged P7b-1a/P7b-1b/P7b-1c owner gates.
34. `P7B1B_DENIAL_CANARY_DESIGN_GATE.md` - stable LPAC launch, token/job
    verification, synthetic denial probes, disposable-profile quarantine,
    cleanup/rollback, and the exact one-trial owner gate.
35. `P7B1B_DENIAL_CANARY_RESULT.md` - exact Trial 1 receipt, pre-resume token
    query failure, independently verified cleanup, retained no-pass claim, and
    fresh-authority boundary for any future attempt.
36. `P7B1B_DENIAL_CANARY_FAILURE_ANALYSIS.md` - exact host/API reconciliation,
    documentation correction, compatibility alternatives, and selected
    no-weaken class-46 plus access-discriminator rule for any separately
    authorized future attempt.
37. `P7B1B_DENIAL_CANARY_TRIAL2_RESULT.md` - exact Trial 2 pre-resume
    mitigation-check failure, independently verified cleanup, `0xb` versus
    documented `0x7` root cause, prospective regression repair, and retained
    no-pass/no-retry boundary.
38. `P7B1B_DENIAL_CANARY_TRIAL3_RESULT.md` - exact Trial 3 post-resume
    `0xC0000142` startup failure, successful suspended-host verification,
    independently verified cleanup, bounded dependency diagnosis, prospective
    diagnostic repair, and retained no-pass/no-retry boundary.
39. `P7B1B_STARTUP_COMPATIBILITY_DESIGN.md` - capability-free isolated
    dynamic/static CRT comparison, deterministic PE/import receipt, adversarial
    claim limits, and the separately gated future-trial boundary.
40. `P7B1B_DENIAL_CANARY_TRIAL4_RESULT.md` - exact static-CRT Trial 4 repeated
    `0xC0000142` result, successful suspended-host verification, independent
    cleanup, falsified import-only repair, and no-retry diagnosis boundary.
41. `P7B1B_LOADER_DIAGNOSIS_DESIGN_GATE.md` - capability-free Trial 3/4
    evidence ceiling, ranked loader hypotheses, information-gain comparison,
    offline PE-surface recommendation, stop rule, and exact owner gate.
42. `P7B1B_LOADER_SURFACE_PROOF_RESULT.md` - exact offline PE32+ receipt,
    import-symbol/delay-import/TLS/load-config/resource observations, hostile
    fixtures, claim limits, and the fired static-optimization stop rule.
43. `P7B1B_ROUTINE_TEST_DELEGATION_AND_DYNAMIC_OBSERVATION.md` - standing
    owner delegation for bounded routine tests, exact native debug-event
    observer boundary, claim limits, regression guard, and one-run stop rule.
44. `P7B1B_DYNAMIC_OBSERVATION_TRIAL5_RESULT.md` - exact debug-event timeout,
    original cleanup failure, independently completed cleanup, prospective
    observer-order repair, no-retry boundary, and retained no-proof result.
45. `P7B1B_POST_TRIAL5_ROUTE_DECISION.md` - repaired one-run validation versus
    leaving P7b-1b blocked, cleanup-regression rationale, rejected silent
    routes, exact owner responses, and final diagnostic-family stop rule.
46. `P7B1B_REPAIRED_OBSERVER_VALIDATION_RESULT.md` - exact owner-selected
    repaired validation, seven-event trace, automatic plus independent cleanup,
    retained no-cause/no-denial claim, and terminal diagnostic-family stop.
47. `P7B1B_ALTERNATIVE_CONTAINMENT_ROUTE.md` - post-LPAC alternative matrix,
    recommended capability-free Windows Sandbox protocol, host-change and
    execution gates, adversarial failures, and exact owner decision.
48. `P7B1B_NO_OS_UPGRADE_REBASELINE.md` - owner rejection of an OS-edition
    upgrade, F5/G1/R1 dependency repair, Home-compatible future options, hard
    pending-containment blocker, and exact owner decision.
49. `P7B_BUILTIN_REFERENCE_VIEWPORT_DECISION.md` - owner-approved no-install
    Forge-owned viewport, corrected threat model, deterministic wireframe and
    pose projection result, adversarial limits, and next controlled-review step.
50. `P7B_BUILTIN_VIEWPORT_CONTROLLED_STIMULUS_RESULT.md` - exact reference plus
    three deliberate bad-control bindings, six protocol pairs, zero inferred
    owner claims, focused verification, and explicit observation-entry boundary.
51. `H1_REFERENCE_INTAKE_RESULT.md` - repository-local recovered/synthetic
    reference inventory, duplicate provenance, poison audit, minimal typed
    suite, and authority-negative H1 result.
52. `H2_NEUTRAL_HUMANOID_REPRESENTATION_RESULT.md` - strict neutral structural
    profile, hierarchy, coordinate, topology, and canonical-codec result.
53. `H3_NEUTRAL_HUMANOID_GENERATION_RESULT.md` - deterministic capability-free
    structural generation and replay result with explicit visual non-claims.
54. `H4_HUMANOID_FUNCTIONAL_CONTROLS_RESULT.md` - orthogonal broken-connection,
    silhouette-collapse, and articulation-drift calibration result.
55. `H5_VISUAL_REFERENCE_INTAKE.md` - actual-pixel source receipts, rejections,
    owner creative direction, and phone-legibility failure evidence.
56. `H6_HUMANOID_REPRODUCTION_RECOVERY_READINESS.md` - clean rebuild, replay,
    corruption recovery, stable-identifier, and retained-receipt coverage audit.
57. `H6_HUMANOID_REPRODUCTION_RECOVERY_RESULT.md` - exact H1-H5 manifest,
    hostile-byte recovery, durable ProofReceipt, and backup/reopen result.
58. `H7_HUMANOID_PROMOTION_READINESS.md` - narrow proof-baseline candidate,
    claim/non-claim matrix, and reversible candidate-lifecycle gap.
59. `ORGANISM_GROWTH_AND_SCALABLE_FIDELITY_REQUIREMENTS.md` - future organism
    growth and coordinated phone-to-high-end fidelity obligations.
60. `SELECTIVE_LIVING_ENTITY_AGING_DESIGN.md` - ambient age diversity versus
    relationship-tracked continuous biological aging, reversible adult-form
    presentation lock, no-old-age-death boundary, presentation cues, and cheap
    proof plan.
61. `FORGE_WIDE_NATURAL_FUNCTION_REASSESSMENT.md` - full Forge/game
    manifestation audit, imported-compendium disposition, redundancy and
    dependency consolidation, philosophy boundary, and reconciled N0-N3 route.
62. `GROUNDED_SEARCH_CASE_RESULT.md` - capability-free bounded-search case
    portfolio, negative-transfer result, selective-aging formula check, and
    explicit no-application gate.
63. `KNOWLEDGE_INTAKE_V2_RESULT.md` - multi-facet conversational knowledge
    classification, category/actor search projection, canonical routing,
    adversarial fixtures, and authority-negative limitations.

## Status meaning

- `specified`: architecture is accepted but has no sufficient executable proof.
- `prototype_tested`: a bounded prototype passed; it is not production proof.
- `reference_proven`: engine-neutral contract and proof harness have passed.
- `production_candidate`: ready for an engine-adapter or production trial.
- `promoted`: accepted standard with a retained rollback target.
- `gated`: intentionally unavailable until named prerequisites are verified.

The registry does not grant authority. It is a planning and navigation
projection linked to contracts, evidence, and future work packages.
