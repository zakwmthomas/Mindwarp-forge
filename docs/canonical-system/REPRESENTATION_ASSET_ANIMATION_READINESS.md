# Representation, Asset, and Animation: Readiness Package

**Status:** readiness retained; bounded P7a contract/lineage, P7b-0 protocol,
and P7b-1a containment-policy references verified; P7b-1b Trial 1 failed before
resume and its no-weaken compatibility repair is compiled but unexecuted, so
denial behavior remains unproved. See
`P7B_CONTROLLED_PERCEPTION_DESIGN_GATE.md` and the researched local boundary in
`P7B1_CONTAINMENT_DESIGN_GATE.md`.
The executable boundary is specified in `P7B1B_DENIAL_CANARY_DESIGN_GATE.md`.
The retained failure and prospective compatibility decision are recorded in
`P7B1B_DENIAL_CANARY_RESULT.md` and
`P7B1B_DENIAL_CANARY_FAILURE_ANALYSIS.md`.
Its single authorized first trial failed safely before canary resume and is
recorded in `P7B1B_DENIAL_CANARY_RESULT.md`; denial behavior remains unproved.
This package does not create meshes, textures, rigs, scenes, engine files, or
game-runtime assets.

## Purpose

Validated construction recipes need an explicit, inspectable choice of
representation before they become artifacts. Articulation and temporal fidelity
must consume the same part-role constraints and shared significance semantics.
This prevents geometry, materials, LOD, and motion from diverging into separate
category-specific pipelines.

## Boundary to establish

| Record | Required role | Must not contain |
|---|---|---|
| `RepresentationDecision` | Compared options, functional requirements, cost/fidelity assumptions, selected neutral form | Engine object type or hidden category default |
| `ArtifactManifest` | Recipe/version refs, representation, materials, LODs, validators, review artifacts, repair lineage | Runtime import path or executable payload |
| `MaterialRegionSet` | Typed regions and functional/visual constraints | Renderer-specific shader state |
| `ArticulationPlan` | Joints/deformation/contact/behaviour interfaces from the part-role graph | Physics/animation component references |
| `TemporalFidelityPlan` | Cadence, interpolation, fallback, and significance mapping | Private priority model |
| `VisualReviewReceipt` | Seed, recipe, view conditions, assertions, cost label, human-feedback link if any | Promotion authority or unstructured “looks good” claim |

## Core invariants

- Representation is selected from explicit functional requirements and compared
  alternatives, not an asset label or assumed engine default.
- Artifact identity references a replayable construction recipe and a
  representation decision; LODs and repairs retain that lineage.
- Material boundaries, sockets, support, collision, articulation, and
  deformation remain validated against the same part-role graph.
- Temporal fidelity is derived from the shared ImportancePacket and has named
  interpolation/fallback behavior; it cannot create a private LOD hierarchy.
- A visually plausible output is not accepted without recipe/validator/review
  evidence, and a numerical pass is not accepted without required perception
  evidence for player-visible artifacts.

## Fixture matrix

| Fixture | Required observation |
|---|---|
| Functional category matrix | Representation decision compares deformation, recombination, sharpness, instancing, volume, cost, and authoring limits |
| Same recipe replay | ArtifactManifest/representation/LOD lineage is reconstructable |
| Material-boundary fault | Validator identifies invalid region/interface relation |
| Socket/support/collision fault | Recipe error remains visible before artifact packaging |
| Articulated neutral fixture | Contact/topology/interpolation behavior follows ArticulationPlan |
| Temporal LOD transition | Significance change produces declared cadence/fallback without private priority logic |
| Repair candidate | Failure retains original lineage and records repair attempt/result |
| Perception review | Seed, view conditions, assertions, reviewer feedback, and limitation are structured rather than prose-only |
| Cost comparison | Measurements are labelled measured/simulated/estimated and do not imply engine performance |

## Required inspection

Reference Studio must be able to inspect a recipe, representation comparison,
ArtifactManifest, LOD/material/articulation/temporal plans, validation failure,
repair lineage, review conditions, and linked ProofReceipt. It must not run an
artifact, fetch assets, edit a recipe, or promote a candidate.

## Neighbour contracts

| Neighbour | Provides | Receives |
|---|---|---|
| Semantic/construction | Validated PartRoleGraph, sockets, regions, articulation roles | Representation decision and artifact/validator receipts |
| Significance/scheduler | Shared priority/cadence semantics | Declared asset/animation work requests and fallbacks |
| Reference Studio | Read-only inspection/review projection | Evidence-linked manifests only |
| Future runtime adapter | Promoted neutral ArtifactManifest only after gates | No engine state back into canonical recipe meaning |

## Readiness gaps deliberately left open

The evidence does not select representation scoring/weights, neutral geometry
or material format, LOD policy, visual assertion method, review thresholds,
contact solver, interpolation policy, repair strategy, or hardware classes.
Those choices affect art direction and technical cost; they require bounded
research/design and structured owner review before implementation.

## Entry criteria for a future implementation package

- Semantic/construction and significance contracts are selected and
  reference-tested.
- Representation comparison, neutral ArtifactManifest, review, and repair
  schemas are versioned together.
- Fixture matrix includes structural, temporal, visual, repair, and cost cases.
- Any visual test is explicit about renderer conditions and cannot be promoted
  merely from an AI-generated image or unstructured chat reaction.
- Work remains engine-neutral, data-first, and without external execution or
  code-promotion authority.
- P7b begins with a capability-free review-protocol/receipt harness. A renderer
  or imported asset cannot enter that slice. Any containment runner and any
  actual visual review remain later, separately authorized packages.
