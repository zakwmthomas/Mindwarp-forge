# Selective living-entity aging design

**Status:** owner requirement and bounded design recorded; not implemented.

## Intent

Mind Warp needs visible age diversity without universal aging, old-age death,
or the cost and narrative consequences of continuously simulating every living
entity. Aging serves two different purposes and must not collapse them into one
global clock.

## Two aging lanes

### 1. Ambient age diversity

Wild creatures and ordinary world NPCs receive a deterministic age cohort when
their identity is created or materialized. Species-, culture-, location-, and
context-specific distributions may produce babies/young, juveniles, adults,
and elderly beings so a population does not look uniformly adult or uniformly
young.

Ambient age is normally a stable identity characteristic, not a continuously
ticking lifetime simulation. This is the cheap lane: generation selects a
cohort and derives its presentation; unloaded entities require no aging work.

### 2. Relevant relationship lifecycle

Only explicitly tracked relationship entities need persistent growth:

- a companion or pet born through breeding;
- a child the player has with a spouse;
- a later explicitly promoted relationship entity, if a future design adds it.

These entities always progress biologically from baby/young through adult and,
with sufficient elapsed lifecycle progress, into elderly. A per-save gameplay
setting controls presentation only:

- **Adult-appearance lock on:** canonical biological age keeps advancing. A
  juvenile remains visibly juvenile; once biological adulthood is reached, an
  adult or elderly entity is presented in its adult form;
- **Adult-appearance lock off:** presentation follows the current canonical
  biological age, so an entity whose age advanced beyond adulthood while
  locked reveals its current older stage;
- old age never causes death;
- the player character does not age through this system;
- unrelated ambient NPCs do not begin ticking merely because the setting is on.

Toggle changes never rewrite, pause, rewind, rejuvenate, or accelerate canonical
age. Enabling the lock can temporarily present an already elderly eligible
entity as adult; disabling it reveals the true current stage. This makes the
setting reversible while retaining continuous history.

## Canonical state boundary

The smallest engine-neutral record should keep these concepts separate:

| Concept | Meaning | Must not imply |
|---|---|---|
| `AgeCohort` | Generated population category such as young, juvenile, adult, elderly | A running clock or death timer |
| `LifecycleMode` | Ambient snapshot or relationship-tracked | Importance, render LOD, or mortality |
| `MaturityProgress` | Bounded baby/young-to-adult progress | Elderly progression |
| `ElderProgress` | Bounded canonical adult-to-elder progress that continues independently of presentation | Death, disease, or capability loss |
| `AdultAppearanceLock` | Per-save presentation rule applied only at or beyond biological adulthood | Paused, rewritten, or rejuvenated canonical age |
| `PresentationProfile` | Species/lineage-authored visual and motion response curves | One universal human aging formula |
| `MortalityPolicy` | Separate future gameplay concern | Automatic death from age |

If an ambient entity becomes a persistent companion, conversion creates a
tracked lifecycle state consistent with its existing cohort. It must not reroll
or visibly change age during adoption.

## Cheap convincing presentation

Aging should be derived from a small, species-authored parameter vector rather
than storing a unique production model for every age. Possible cues include:

- head-to-body and eye-to-face proportion;
- muzzle, jaw, brow, cranial, ear, horn, or crest development;
- limb length/thickness, torso mass, shoulder/hip balance, and extremity scale;
- posture, center of mass, stride length, cadence, stiffness, and idle motion;
- fur, feather, scale, skin, hair, pattern, saturation, roughness, and wear;
- voice pitch/timbre and age-specific behavior vocabulary.

Head and eye scaling are valid juvenile cues only where the species profile
supports them. Applying them universally would create cute caricatures,
silhouette errors, or biologically incoherent creatures. Elderly presentation
also does not require frailty unless a separate gameplay design calls for it.

## Phone-to-PC derivation

The age signal must survive every presentation tier:

- high-end/near: continuous morphs, richer surface cues, full motion profile;
- balanced: fewer morph channels, simplified materials, retained silhouette,
  posture, major landmarks, and cadence;
- phone/far: stage-specific low-poly derivative or bounded blend, broad color
  and silhouette cues, reduced bones/material regions, sparse animation;
- impostor/unloaded: stable cohort token only, with no continuous update.

The same entity ID and lifecycle state drive every tier. Fidelity changes may
alter presentation cost but cannot change age, maturity, relationships, or
gameplay truth.

## Deterministic population distribution

Ambient cohorts should be sampled from declared weights using stable world,
population, species, and entity identity inputs. Distribution is a population
constraint, not a promise that every local group exactly matches global
percentages. Later tests must cover empty/small populations, all-zero or invalid
weights, rare elderly/young cohorts, group plausibility, replay, generator
version coexistence, and no cohort reroll after reload.

## Cheap proof plan

1. Define strict bounded enums and fixed-point progress values; no wall clock,
   renderer, mesh, engine, filesystem, or randomness capability.
2. Build a pure state-transition table covering ambient entities, bred babies,
   children, adoption, appearance-lock toggles, adulthood, hidden elderly
   progression, reveal after unlock, reload, and no-old-age-death invariants.
3. Run property/metamorphic tests: deterministic replay, monotonic biological
   age, juveniles unaffected by the lock, adult presentation clamping, hidden
   elderly progress, reveal after unlock, no rejuvenation, ambient non-ticking,
   adoption continuity, and no mortality event.
4. Test one humanoid and at least one structurally different creature profile
   so human proportions cannot masquerade as universal biology.
5. Simulate age-distribution quality and presentation budgets in memory before
   creating meshes or running a game engine.
6. Use phone-legible visual comparisons with one large subject/stage per image;
   never repeat the failed three-small-panel presentation.

## Explicit non-claims

No aging state machine, population sampler, morph system, creature generator,
shader, mesh LOD, animation set, runtime integration, or device performance is
implemented by this document. It records the clarified design boundary and the
cheapest safe route to a future proof.
