# Organism growth and scalable fidelity requirements

**Status:** owner direction recorded; system not implemented or runtime-proven.

## Clarified product requirement

Mind Warp needs persistent growth and change across NPCs, humanoids, animals,
alien creatures, and other living agents. The same organism identity must be
able to present appropriately on a phone and on a high-end PC without forking
gameplay truth or silently becoming a different creature.

The owner subsequently narrowed lifecycle aging in
`SELECTIVE_LIVING_ENTITY_AGING_DESIGN.md`: ambient populations need stable age
diversity, while only bred companions, children, and later explicitly tracked
relationship entities require persistent growth. Baby-to-adult growth always
continues; a setting controls optional adult-to-elder progression; old age does
not cause death and the player character is outside this system.

The intended visual direction is semi-realistic anatomy and proportions with a
mature stylized treatment: simplified shapes and materials, deliberately
graphic lighting, and a restrained toon/cel-like shader. It must avoid both
photorealistic production cost and childlike caricature.

## What exists now

Forge currently has a capability-free, engine-neutral humanoid proof chain:
typed identity and hierarchy, deterministic structural generation, calibrated
broken-structure controls, lineage, and read-only inspection evidence. It is a
17-joint wire structure, not an in-game growth system, creature generator,
mesh, shader, animation runtime, or phone-performance result.

## Required separation of concerns

1. **Canonical organism state:** species/lineage identity, individual seed,
   biological age, life stage, inherited traits, acquired changes, health,
   injuries, environment, and explicit event history.
2. **Morphology evaluation:** deterministic, bounded evaluation of canonical
   state into proportions, part roles, articulation, mass distribution, and
   material regions. A humanoid is one possible morphology, not the universal
   base class for creatures.
3. **Presentation derivation:** geometry, rig, textures/materials, animation,
   collision, and effects are derived artifacts with lineage back to the same
   organism state.
4. **Fidelity policy:** shared significance selects declared presentation and
   simulation tiers; it cannot change canonical biology or create a private
   gameplay truth.
5. **Runtime adapter:** a future selected engine maps promoted neutral records
   to platform-specific assets and measured budgets. Engine choice and actual
   device claims remain gated.

## Multi-axis fidelity, not polygon count alone

Phone scalability requires coordinated degradation across independent axes:

- geometry LOD and silhouette preservation;
- skeleton/skin influence and deformation LOD;
- material/shader feature tiers and lighting complexity;
- texture resolution and residency;
- animation sampling, interpolation, and secondary-motion cadence;
- AI, sensing, pathfinding, growth, and off-screen simulation cadence;
- physics/collision fidelity;
- population density, impostors, culling, and streaming.

The high-end and phone representations must share a stable organism ID,
canonical growth state, semantic landmarks, animation intent, and derivative
lineage. Switching tiers must be monotonic and hysteretic enough to avoid
visible popping, oscillation, or gameplay divergence.

## Cheap proof sequence before runtime work

1. Typed lifecycle and morphology records with explicit units and bounds.
2. Pure deterministic age/trait/environment evaluator for one humanoid and one
   non-humanoid creature fixture.
3. Property and metamorphic tests: replay, order, boundedness, symmetry,
   impossible anatomy, stage transitions, injury persistence, and generator
   version migration.
4. Synthetic fidelity table proving every consumer maps from one shared
   significance input without modifying canonical state.
5. Disposable mesh-count, bone-count, texture-memory, animation-cadence, and
   population-budget simulation across proposed hardware classes.
6. Only then: bounded runtime adapter and representative physical-device
   profiling. PC success never proves phone success.

## Acceptance obligations

- Growth is replayable from a fixed baseline and ordered deltas.
- Biological age, visual scale, gameplay capability, and simulation cadence are
  distinct variables; none silently substitutes for another.
- Life-stage transitions preserve identity and do not require storing a unique
  production mesh for every age.
- Creature diversity is tested against withheld morphologies rather than one
  humanoid template stretched into animals.
- LOD transitions preserve silhouette, landmarks, contacts, recognisability,
  and gameplay-relevant collision within explicit tier-specific tolerances.
- Every performance statement is labelled estimated, simulated, or measured
  with hardware, scene, population, resolution, and frame-time conditions.

## Imported Gemini compendium: evidence assessment

The owner-provided compendium is retained verbatim at
`evidence/imported-chat/2026-07-14-gemini-fractal-matrix-engine.md`. Potentially
useful hypotheses include constraint-guided generation, compact lineage state,
sparse deltas, fields, morphology inheritance, influence maps, and shared
deterministic inputs.

It is not a production-ready blueprint. Claims including infinite scale,
instant high-resolution baking, free thermodynamic simulation, zero
serialization overhead, deterministic peer agreement, and greater-than-90%
bandwidth savings lack a workload, algorithmic complexity analysis, threat
model, fault model, numerical determinism policy, measurements, or recovery
proof. The supplied Unity branching example uses ambient `Random`, so it is not
deterministic as written and cannot support the peer-synchronization claim.
Commerce, anti-cheat, consensus, and real-money systems are separate high-risk
security domains and must never inherit authority from this document.

## Open design work

- Define the smallest lifecycle vocabulary that works for both a humanoid and
  a structurally different creature.
- Define representative phone, mainstream PC, and high-end PC budget envelopes
  after the future runtime route is selected.
- Calibrate a mature semi-realistic stylization target with actual visual
  comparisons; concept imagery is direction evidence, not topology or device
  performance proof.
- Clarify whether the owner's phrase "NNR" names a particular device or
  technology; current requirements conservatively interpret it as constrained
  phone-class hardware.
- Prove the selective two-lane aging state model and species-authored
  presentation profiles before any mesh, engine, or universal aging work.
