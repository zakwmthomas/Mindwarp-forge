# P6 Semantic Emergence and Construction Language Design Gate

**Status:** `prototype_tested`; the owner authorized the bounded offline test
harness and the capability-free reference passed. This document does not select a product ontology,
creative vocabulary, solver, geometry system, runtime, engine, or art style.

## Question

What is the smallest engine-neutral proof that can show a causal path from
declared world/gameplay pressures to a replayable typed construction record,
without treating words, language-model output, search scores, or plausible
appearance as truth?

## Recovered evidence audit

The fixed survival pack
`MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK_2026-07-12.zip` has SHA-256
`f0f01b7469226d3d5c77780c23e97a96342b517ece536bc0351e9486117b251b`.
Five relevant nested packages were inspected in memory; none was extracted or
adopted:

| Package | SHA-256 | Retained tests | Useful evidence |
|---|---|---:|---|
| `mindwarp_capability_composition.zip` | `826fe8bb9a05addea9f89a5236ad83fc25e7cb686127e33c7574482f7c6d67fd` | 10 | Capabilities can compose independently of category; phenotype need not rewrite canonical rules. |
| `mindwarp_compositional_semantics.zip` | `40fb25b44445d35578e2139988ac382cdc29fd3d0d499ebf2a84193c9b807a71` | 6 | Bounded search, explicit requirements/forbiddance, and reconciliation hypotheses. |
| `mindwarp_forge_sprint5_capabilities.zip` | `d0311cb66fad4dace516d30ed223968de701577bce23240192a8e5b543998140` | 35 | Dependency/conflict tables, independent validation, invalidation, candidate diversity, and repair limits. |
| `mindwarp_semantic_ecology_experiment.zip` | `8e9166eb8382c4b691edcb8431179469f39433e2ffc9a8d8fac86665ce9bca18` | 9 | Modifier/head relations, environmental support, bounded archives, and display-name separation. |
| `mindwarp_unified_genotype_benchmark.zip` | `8cb864fc2e70d8c6b2c9613a4152c839d668847e87e63ea9d5213928f088e771` | 6 | A shared typed source model can precede differing representations. |

The 66 passing tests are hypothesis evidence, not closure. They mostly assert
outputs of hand-written vocabularies, bridge tables, category lists, float
weights, and thresholds. Specific defects prevent reuse:

- semantic search sorts words, while the ecology package says modifier/head
  order matters;
- a seed is stored but often does not affect generation;
- unknown capabilities can pass silently, duplicated compiler/validator tables
  can drift, and one compiler can report `ready` while retaining conflicts;
- beam search proves neither completeness nor stable equivalence and may prune
  the only route to a later valid composition;
- one scalar archive score hides trade-offs and hard-codes family budgets;
- capability prototypes directly select mesh, voxel, animation, and runtime
  systems, crossing the P7 representation boundary;
- names, confidence floats, and plausible explanations are not causal evidence;
- cached runtime artifacts and Python timing reports grant no engine, runtime,
  performance, or production authority.

## Primary-practice reconciliation

- Harnad's symbol-grounding problem is the central warning: symbols cannot gain
  meaning merely by being related to more symbols. P6 grounds every accepted
  semantic claim in a versioned pressure, observation, history reference, or
  declared gameplay requirement; language remains a label/proposal surface:
  <https://www.sciencedirect.com/science/article/abs/pii/0167278990900876>.
- W3C SKOS distinguishes identified concepts from preferred, alternate, and
  hidden lexical labels and defines label integrity conditions. P6 similarly
  makes labels non-canonical and requires an explicit versioned alias/equivalence
  assertion rather than string similarity: <https://www.w3.org/TR/skos-reference/>.
- W3C PROV separates entities, activities, agents, and qualified influence. P6
  reuses the principle, not RDF or OWL: evidence references, derivations, and
  responsible policy versions are distinct fields:
  <https://www.w3.org/TR/prov-o/>.
- SHACL validates data graphs against separate shapes and produces explicit
  reports. P6 adopts separate closed-world data/schema validation and localized
  violations, but does not select RDF, SHACL, SPARQL, or recursive semantics:
  <https://www.w3.org/TR/shacl/>.
- NASA logical decomposition traces stakeholder expectations through functions,
  interfaces, alternatives, trade studies, rationale, cost, risk, and
  verification. P6 retains multiple solution families and explicit trade
  vectors instead of jumping from a noun to a part graph:
  <https://www.nasa.gov/reference/4-3-logical-decomposition/> and
  <https://www.nasa.gov/reference/4-0-system-design-processes/>.
- PDDL demonstrates the value of typed objects, declared requirements,
  preconditions, effects, and safety constraints. P6 recipe operations use
  explicit typed pre/postconditions but do not select PDDL or a planner:
  <https://planning.wiki/ref/pddl/domain>.
- Algebraic graph transformation makes deletion/gluing conditions explicit.
  P6 therefore rejects dangling references and validates every rewrite's
  pre-state and post-state, without selecting DPO/SPO machinery:
  <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/AF02050525390437E1DF746DE4459926/S0960129501003425a.pdf/double-pushout-graph-transformation-revisited.pdf>.
- MAP-Elites shows why a portfolio across declared behavior dimensions can be
  more informative than one highest scalar score. P6 retains bounded,
  mechanism-distinct alternatives; the algorithm and diversity dimensions
  remain later decisions: <https://arxiv.org/abs/1504.04909>.
- Solver unsatisfiable cores illustrate useful conflict localization, but are
  optional and solver-dependent. P6 requires deterministic violation lists and
  permits a minimal conflict set only when independently reproducible; no Z3
  dependency is selected: <https://microsoft.github.io/z3guide/programming/Parameters/>.

## Repaired contract

The proof boundary is:

`PressureContext -> RoleSet -> SolutionFamilySet -> CapabilityGraph -> PartRoleGraph -> ConstructionRecipe -> ValidationReport`

### Shared envelope

Every record has a strict schema version, stable record ID, source fingerprints,
policy version, dependency versions, canonical bytes, and explicit
`observed | derived | declared | hypothesis` claim class. Confidence is evidence
metadata, never a truth bit or universal ranking score. Unknown fields, types,
concept IDs, operations, and versions fail closed.

### Semantics

1. `PressureContext` contains bounded references to P3/P4 world conditions,
   descriptor/history evidence, and explicit gameplay requirements. It cannot
   infer personality, culture, intent, or total population from labels.
2. `ConceptId` is canonical; labels and locale-specific synonyms are display
   metadata. A synonym fixture changes no canonical bytes. A homonym cannot
   merge concepts without an explicit versioned mapping.
3. `JustificationGraph` is an acyclic proof graph with typed
   `supports`, `derives`, `requires`, `conflicts`, and `rejects` edges. Other
   domain/structural graphs may contain declared cycles; proof ancestry may not.
4. `RoleSet` states functions and constraints, not geometry or asset categories.
   Every role traces to at least one accepted pressure or gameplay requirement.
5. `SolutionFamilySet` retains at least two mechanism-distinct feasible families
   for a material choice, or an explicit `single_feasible_family` receipt.
   Selection first checks hard constraints, then compares named trade-vector
   dimensions with recorded rationale. There is no private global scalar.
6. Candidate generators—including future AI—are untrusted proposal sources.
   Only deterministic validation can move a proposal into proof evidence.

### Capability and construction

1. `CapabilityGraph` uses a closed, versioned registry of capabilities,
   dependencies, conflicts, interfaces, and evidence. Unknown capabilities and
   unsatisfied dependencies are violations, not silently ignored extensions.
2. `PartRoleGraph` is category-neutral data: typed part roles, relations,
   interface/socket roles, direction, cardinality, frames, material-region roles,
   articulation/deformation roles, and declared constraint references. It
   contains no mesh, physics body, shader, executable code, or engine object.
3. `ConstructionRecipe` is an ordered list of typed graph operations with stable
   operation IDs, exact preconditions, postconditions, and expected state
   fingerprint. A failed operation leaves no partially accepted recipe result.
4. Validation is independent of generation and covers provenance closure,
   schema closure, causal ancestry, capability closure, graph identity,
   connectivity, interface compatibility, direction/cardinality, dangling
   references, declared support/collision constraints, and replay.
5. Support and collision in P6 validate declared symbolic relations and bounds;
   they make no geometric, structural-engineering, or physics-simulation claim.
6. P7 alone chooses representation. P6 may state functional requirements but
   cannot output `mesh`, `voxel`, `rig`, `shader`, runtime, or engine choices.

## Whole-system alignment

| Boundary | Preserved rule |
|---|---|
| P2 identity | Stable IDs and domain-separated fingerprints are consumed; P6 cannot redefine universe identity. |
| P3 fields/world rules | Fixed PressureContext fixtures may cite declared field/world evidence; P6 cannot invent physical truth. Derived-world production remains open. |
| P4 hierarchy/history | Descriptor and delta references are bounded and versioned. Semantics cannot mutate baselines, append deltas, or infer unobserved population. |
| P5 significance/scheduler | Scheduling may affect when proposals are evaluated, never their canonical result or meaning. P6 has no private priority model. |
| P7 representation/assets/animation | Receives only a validated functional graph and trade evidence. Representation, topology, materials, motion, and perception remain later gates. |
| Reference Studio | Displays causal chains, alternatives, violations, recipe replay, limitations, and receipts read-only. It cannot edit, execute, approve, or promote them. |
| Kernel/authority | Proof evidence is serialized data only. No protected-Kernel dependency, filesystem, process, network, credential, spending, publishing, or promotion capability is added. |

## Adversarial proof matrix for an authorized reference

| Fixture | Required result |
|---|---|
| Label/synonym permutation | Same canonical semantic and recipe fingerprints. |
| Homonym and locale collision | Distinct concept IDs remain distinct; ambiguity is explicit. |
| Poison/prompt-injection text | Stored only as inert proposal evidence; cannot add types, operations, or authority. |
| Unsupported causal leap | Missing ancestry violation with exact record/edge location. |
| Contradictory pressures | Deterministic violations and alternatives; no silent merge. |
| Justification cycle | Rejected even if the structural graph could validly contain cycles. |
| Fake diversity by renaming | Does not satisfy mechanism-diversity requirement. |
| Unknown capability or stale registry | Rejected before construction. |
| Missing dependency/conflicting capability | Closed-world violation; no `ready` result. |
| Socket direction/cardinality/type fault | Localized structural failure. |
| Dangling delete or stale precondition | Atomic recipe rejection; original graph remains valid. |
| Input/operation order variants | Canonical ordering or explicit ordered semantics; identical meaning replays identically. |
| Search budget exhaustion | `indeterminate_budget` receipt, never false impossibility or automatic best. |
| Dependency/version drift | Explicit reject/migrate decision; no silent reinterpretation. |
| Read-only receipt integration | Recording/viewing evidence changes no Kernel object, event, candidate, or authority state. |

Measurements are simulated integer counts only: records, nodes, edges,
alternatives, operations, validations, violations, and bounded work examined.
No runtime, creative-quality, physical-validity, solver-completeness, or
performance claim is permitted.

## Recovery and limits

- Invalid candidates and partial recipe attempts remain evidence but never
  canonical output.
- Revalidation uses original bytes plus pinned contract/policy versions; a new
  ontology or operation set creates a new result rather than rewriting old data.
- The reference may include only a tiny synthetic vocabulary and operation set
  sufficient to discriminate the failure matrix. These are fixtures, not the
  Mind Warp content grammar.
- Still open: pressure ontology, role/capability vocabulary, equivalence policy,
  diversity dimensions, comparison policy, graph type system, socket semantics,
  constraint solver, recipe operations, creative review, and all P7 choices.

## Exact confirmation (satisfied)

The owner authorized Codex to write and run only a capability-free P6 Rust reference harness using a tiny synthetic
fixture vocabulary and operation set to prove strict encoding, causal ancestry,
alternative retention, closed capability validation, typed graph validation,
atomic recipe replay, deterministic failure receipts, and read-only
ProofReceipt integration.

This approval does **not** authorize product vocabulary or weights, AI/LLM
generation, a constraint solver, geometry, representation, assets, animation,
runtime or engine integration, filesystem/network/process access, credentials,
spending, publishing, promotion, or protected-Kernel mutation.

## Verified reference result

- Strict canonical JSON rejects unknown fields, noncanonical bytes, and version
  drift while preserving label-independent semantic identity.
- Grounded acyclic justification, mechanism-evidence diversity, hard
  feasibility, named trade vectors, and closed capability validation are
  executable rather than prose-only.
- Typed socket compatibility/cardinality, connectivity, stale preconditions,
  dangling removal, exact ordered replay, and atomic failure are covered.
- Fourteen adversarial crate tests and a read-only Forge Desktop ProofReceipt
  integration test pass. The eight-module boundary bars Kernel, desktop,
  network, filesystem, and process capabilities.
- The full Forge gate passes. The fixture vocabulary, operations, measurements,
  and outputs remain simulated non-product evidence with every retained limit
  above unchanged.
