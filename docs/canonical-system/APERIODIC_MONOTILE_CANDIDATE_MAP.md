# Aperiodic Monotile Candidate Map

**Status:** retained whole-system research audit and cross-cutting candidate.
This record maps the mechanism across the complete Forge, game-canonical and
runtime architecture; it does not change the active C3 checkpoint, select a
world topology, authorize implementation, or promote a production mechanism.

## Intake and source audit

The owner supplied a Gemini-written technology dossier on 2026-07-16 as an
idea to assess across Forge. That dossier is evidence, not authority. The
mathematical core was checked against the authors' papers and project pages:

- Smith, Myers, Kaplan and Goodman-Strauss, *An aperiodic monotile*,
  <https://arxiv.org/abs/2303.10798> and
  <https://cs.uwaterloo.ca/~csk/hat/>.
- Smith, Myers, Kaplan and Goodman-Strauss, *A chiral aperiodic monotile*,
  <https://arxiv.org/abs/2305.17743> and
  <https://cs.uwaterloo.ca/~csk/spectre/>.
- The authors' bounded Hat patch visualizer and source,
  <https://github.com/isohedral/hatviz>, is BSD-3-Clause and is a reuse
  candidate for disposable study, not an approved dependency.

### Claim disposition

| Intake claim | Disposition | Planning consequence |
|---|---|---|
| One Hat shape admits tilings of the plane but no periodic tiling | Supported | Exact Hat patches are valid aperiodic fixtures. |
| Hat tilings use reflected and unreflected tiles | Supported | Any exact Hat representation must retain handedness. |
| Hat tilings have metatiles and hierarchical substitution | Supported | A bounded lazy hierarchy experiment is mathematically grounded. |
| Spectres can force aperiodicity with translations and rotations only | Supported | Spectre is the cleaner one-chirality candidate if manufacture or reflection state matters. |
| A fixed roughly 12.7 percent of Hats are reflected | Not established by the checked primary summaries | Do not encode this ratio or budget around it without a separate proof. |
| The implementation is completely characterized by 30-degree rotations | Not established as a sufficient rule | Reuse or independently verify a full substitution construction. |
| Both shapes are simply fused pieces of a regular triangular grid | Oversimplified | Do not infer Forge coordinate compatibility from this description. |
| Hat or Spectre generation follows from a 6D hypercubic projection | Not established by the cited discovery papers | Treat cut-and-project constructions as separate research, not the implementation route assumed here. |
| Local caching is O(1), meso-scale utility is high, and macro-scale cost is necessarily poor | Engineering hypotheses, not mathematical results | Measure access, memory, extension and benefit against local baselines. |
| The geometry naturally produces better tectonic or biome boundaries | Application speculation | It may propose grouping or visual structure only; causal fields and ecotones remain authoritative. |

## P16 abstraction

The candidate is a finite patch cut from a deterministic hierarchical
substitution tiling. A tile has a stable patch-local identity, transform,
chirality where applicable, polygon boundary, adjacency edges, parent metatile
path and optional descendants. No tile identity is inferred from an arbitrary
Forge world coordinate until a separate addressing experiment proves a total,
deterministic and extension-stable mapping.

Assumptions:

1. A bounded patch can be generated deterministically from a versioned seed,
   substitution variant, depth and crop rule.
2. Crop or window extension can preserve identities in their overlap.
3. A target consumer benefits from reduced periodic structure enough to pay
   for irregular adjacency and hierarchy.
4. Exact topology, a grouping mask and a visual decoration are three distinct
   candidate uses and never silently substitute for one another.

## Reusable mechanism families

The transferable object is not one universal tiling engine. It is a set of
separable abstractions that may recur at multiple layers:

| Mechanism | General form | Possible shared improvement |
|---|---|---|
| Aperiodic arrangement | Deterministic coverage without translational repetition | Anti-repetition fixtures, layouts and sampling masks. |
| Substitution hierarchy | Parts form typed metaparts recursively | Shared lazy traversal, cache, inspection and LOD experiments. |
| Local adjacency graph | Meaning derives from explicit neighbours rather than array position | Adversarial graph tests and graph-native consumers. |
| Orientation/chirality state | A component carries a transform and, for Hat, handedness | Strong transform, serialization and variant fixtures. |
| Sparse materialization | Resolve only bounded requested branches | Common measurement language for lazy Forge and game systems. |
| Decoupled overlay | Keep a regular authoritative substrate and apply irregular grouping downstream | Lower-risk adoption without replacing identity, physics or storage. |

This is the project's fractal reuse boundary: recurring component, graph,
hierarchy, lazy-resolution and projection patterns should share vocabulary,
fixtures, measurements and proven utilities where their contracts really
match. Domain meaning, objectives and acceptance thresholds remain local.

## Whole Forge audit

| Forge component | Candidate manifestation | Fit | Boundary or falsifier |
|---|---|---:|---|
| Forge truth kernel | Aperiodic graph fixtures for content-addressed object/link replay and traversal-order independence | Medium as a test, low as storage | Never change canonical hashing or evidence ordering; reject if an ordinary hostile graph covers the same failures more cheaply. |
| Conversation capture | Non-periodic synthetic conversation-link topology for duplicate, correction and missing-neighbour tests | Low-medium test-only | Geometry has no semantic authority and must not determine what a conversation means. |
| Context compiler and knowledge catalogue | Hierarchical local-neighbour retrieval experiment over existing typed records | Medium research candidate | Compare with current typed search and ordinary graph/topic indexes; no monotile-shaped semantic classification. |
| Task bootstrap and handoff | Stress fixture for bounded hierarchical navigation and missing local context | Low | Bootstrap ordering remains explicit and deterministic; novelty cannot add startup cost without better fault detection. |
| Forge research | A canonical method-candidate case exercising claim correction, whole-system mapping and local falsifiers | High | This record is research evidence only and cannot authorize another system. |
| Step-leader controller | Completeness and ranking fixture for a bounded cross-system monotile probe | High protocol test fit | The controller may rank one local experiment only; it cannot execute research, transfer a tiling result, or grant authority. |
| Control plane and master program | Irregular dependency-graph visualization/test fixture | Medium test, low scheduler use | Master dependencies and owner gates remain authoritative; tiling cannot select or prioritize work. |
| Worker governance and Federated Improvement Kernel | Regression case for broad discovery followed by local transfer gates | High protocol fit | The protocol may transfer; one domain's tiling result, threshold or objective may not. |
| Module graph and verification | Non-periodic module-neighbour fixture for boundary, cycle, omission and propagation tests | High test fit | Use only if it reveals failures not covered by simpler directed-graph fixtures. |
| Forge history, backup and recovery | Substitution-tree snapshot/delta corruption portfolio | Medium test fit | Existing append-only history and content identities remain canonical. |
| Reference Studio and dashboard | Inspect metatile ancestry, adjacency, crop seams and cross-system experiment results | High inspection fit | Visualization is read-only and cannot imply quality or promotion. |
| Reference/asset intake | Exact transform/chirality and derivative-lineage fixture | Medium | Do not confuse geometric handedness with human anatomy, creative intent or asset fitness. |
| Containment and external-tool boundary | Hostile bounded graph/input fixture only | Low | It provides no security boundary and cannot justify executable content. |
| Forge desktop/runtime shell | Optional decorative background or graph layout | Low | UI legibility, accessibility, startup cost and maintenance beat mathematical novelty. |

## Whole game-canonical audit

| Canonical system | Candidate manifestation | Fit | Boundary or falsifier |
|---|---|---:|---|
| Universe identity | Versioned metatile-path identity or stream namespace experiment | Medium-risk | No retrofit. Must beat current hierarchical addressing on stable random/window access, migration and bytes. |
| Field basis | Tile/orientation/metatile identity as a deterministic decorrelation or domain-separation input | Medium | Compare with Philox hashing, blue noise and ordinary multiscale fields. |
| Stellar/orbital | Non-periodic test distribution for many-system catalogues | Low | It is not orbital dynamics, Kepler solving or a physical stellar law. |
| Geological/atmospheric | Candidate fracture, plate-seed or control-volume adjacency scaffold | Medium speculative | Physical stress, conservation and spherical topology must dominate; compare with Voronoi and physical meshes. |
| Hydrological state | Irregular control-volume or catchment-graph fixture | Low-medium | Flow must follow elevation, conservation and outlets; tile edges cannot invent drainage. |
| Climate state | Irregular finite-volume mesh candidate for selected regional solvers | Medium speculative | Requires conservation, stability, error and mobile-cost evidence against regular/icosphere/adaptive meshes. |
| Surface material state | Aperiodic material, palette and weathering-variant mask | High future candidate | Causal material state remains authoritative and transitions retain ecotones. |
| Spatial domain and regional environment | Downstream sampling/grouping overlay on the existing rectified proof domain | Medium | No planet claim, no coordinate replacement, and overlap identities must be extension-stable. |
| Physical-region partition | Alternative proposal mask or adversarial connected-component fixture | Medium-high test fit | Current physical signatures and causal evidence remain canonical; it cannot name biomes. |
| Physical path substrate | Irregular planar/section graph fixture for path and adjacency algorithms | Medium test-only | No replacement for 3D occupancy, collision or exact path witnesses without separate proof. |
| Visible-radiance bulk transfer | Irregular sequence/region fixture for medium batching | Low | It does not alter Beer-Lambert evidence, optical coefficients or exact paths. |
| Visible-radiance interface event | Orientation/chirality hostile transforms and repeated-interface fixtures | Low-medium test-only | It does not solve interval Snell arithmetic or justify a refracted composer. |
| Derived-world rules | Non-periodic region proposal or decorrelation adapter | Medium | Derived physical causes, signals and opportunities remain upstream truth. |
| Lazy universe hierarchy | Substitution tree as an alternative sparse descriptor hierarchy | High conceptual fit | Must prove total addressing, bounded materialization, stable overlap, recovery and better target-local cost than existing descriptors. |
| World history ledger | Metatile-subtree delta locality and snapshot experiment | Medium | Baseline reconstruction plus explicit deltas remain authoritative; retiling cannot rewrite history. |
| Significance system | Aggregate child significance to metatile containers | Medium-low | Shared significance semantics stay independent of geometry and do not inherit tile hierarchy automatically. |
| Streaming scheduler | Hierarchical irregular work/prefetch containers | Medium after hierarchy proof | Compare cache hit, churn, deadlines, memory and mobile cost with regular chunks; no private scheduler. |
| Semantic emergence | Aperiodic adjacency as one candidate pressure/territory/encounter graph | Medium speculative | Semantic roles and gameplay constraints dominate; geometry cannot manufacture meaning. |
| Construction language | Monotile/metatile assemblies as a typed modular-layout family | High future candidate | Sockets, support, access, function, repair and representation constraints must validate every assembly. |
| Organism ecology | Irregular territory, colony, niche-mosaic or surface-pattern candidate | Medium-low | No claim about evolution, body plans or fitness; causal ecology and continuous boundaries dominate. |
| Aesthetic grammar | Hierarchical motif, controlled repetition/variation and chirality language | High future candidate | No universal beauty claim; compare with authored, Wang and blue-noise families under owner review. |
| Representation selector | Explicit planar-tiling or aperiodic-mask representation alternative | Medium | Must be chosen from grounded requirements, never because aperiodicity is fashionable. |
| Asset factory | Floors, walls, skins, panels, decals, cities and modular prop patterns | High future candidate | Asset count, seams, UVs, instancing, memory, draw cost and repair lineage must pass. |
| Procedural animation | Tile-graph phase offsets, crowds, swarms or propagation choreography | Low-medium speculative | Motion causality, contacts and readability dominate; compare with hashes, waves and authored phase maps. |

## Runtime and platform audit

| Runtime concern | Candidate use | Decision boundary |
|---|---|---|
| Runtime adapter | Import a promoted finite patch, graph, hierarchy or mask through engine-neutral bytes | No engine-specific implementation before R1 selection. |
| Rendering | Instance repeated geometry with aperiodic transforms or masks | Measure draw calls, batching, shader cost, memory and visual gain. |
| Streaming | Materialize metatile branches on demand | PC/mobile first; retain deterministic fallback to regular containers. |
| Physics/navigation | Use a generated adjacency graph only for consumers that benefit | Keep conventional collision/BVH/nav representations unless measured evidence wins. |
| Multiplayer/replay | Reconstruct from a versioned finite recipe rather than transmit every tile | Requires byte-exact cross-platform replay, migration and hostile-input limits. |
| Portability | Integer/target-neutral codec and explicit transform/chirality | PC/mobile first, PlayStation next if ROI supports it; no native-float or native-limb canonical format. |

## Registry coverage receipt

The audit is bound to the current canonical registries, not to a hand-selected
subsystem list. `tools/verify-whole-system-method-audit.ps1` fails when either
registry gains a system not named below or when the future C3-R1 plan links
lose this record.

- Forge registry: `forge-step-leader-controller`, `forge-truth-kernel`,
  `forge-context-compiler`, `forge-research`, `forge-control-plane`,
  `forge-reference-studio`.
- Game-canonical registry: `visible-radiance-interface-event`,
  `universe-identity`, `field-basis`, `stellar-orbital`,
  `geological-atmospheric`, `hydrological-state`, `climate-state`,
  `surface-material-state`, `regional-environment-state`,
  `physical-region-partition`, `physical-path-substrate`,
  `visible-radiance-bulk-transfer`, `derived-world-rules`,
  `lazy-universe-hierarchy`, `world-history-ledger`, `significance-system`,
  `streaming-scheduler`, `semantic-emergence`, `construction-language`,
  `organism-ecology`, `aesthetic-grammar`, `representation-selector`,
  `asset-factory`, `procedural-animation`.
- Runtime registry: `runtime-adapter`.
- Atlas surfaces: `forge-kernel`, `conversation-capture`, `task-bootstrap`,
  `forge-dashboard`, `canonical-production-system`, `mindwarp-game`.

## Cross-layer compounding opportunities

The most credible shared improvements are:

1. one versioned bounded graph-fixture format usable by Forge module/control
   tests and game adjacency/path/partition tests;
2. one hierarchy measurement vocabulary for generated nodes, query/window
   latency, overlap churn, cache hits, serialized bytes and recovery;
3. one transform/chirality fixture family for codecs, assets and interfaces;
4. one experiment receipt schema that records the candidate, receiving system,
   baseline, cost, falsifier and negative-transfer result;
5. one Reference Studio inspector for ancestry, adjacency, seams and comparative
   metrics if at least two target-local experiments justify the shared tool.

These are reuse candidates, not approved shared libraries. Code becomes shared
only after at least two real consumers expose the same invariant and a common
module reduces total complexity without coupling their domain authority.

## Explicit non-applications

- **No planet or universe topology.** Hat and Spectre tile a plane; this record
  does not form a planet, solve spherical seams, or replace the finite
  non-wrapping spatial-domain proof.
- **No collision or physics substrate.** There is no current evidence that
  irregular monotile cells beat the existing occupancy, swept-AABB or future
  runtime acceleration structures.
- **No direct biome borders.** Biomes and material character must fade across
  ecotones unless a real sharp cause supports a discontinuity. A monotile may
  vary seeds or grouping proposals; it may not paint hard ecological seams.
- **No retrofit of identity or random access.** Existing exact coordinate,
  reconstruction and replay contracts remain unchanged.
- **No active optical use.** The interval-incident smooth-dielectric audit is
  unrelated and remains the active C3 action.
- **No cosmic-scale generation claim.** Infinite mathematical tilability is
  not a production memory, streaming or performance result.

## Experiment ladder

### M0 - provenance and generator qualification

In a disposable copy, pin the official Hat visualizer revision or reproduce a
minimal substitution generator from the published construction. Record
license, revision, input parameters, deterministic patch fingerprint and every
normalization step. Reject ambient randomness, mutable global construction
order and unbounded recursion.

### M1 - irregular graph fixture

Generate small bounded patches at several substitution depths. Verify polygon
non-overlap, covered area within the declared crop, reciprocal adjacency,
connectedness where expected, deterministic identities and stable overlap
under patch extension.

Baselines: equal-area square grid, hex grid and bounded Voronoi graph.

Metrics: generation time, bytes per tile, neighbour-query time, degree
distribution, path stretch, component reconstruction cost and overlap churn.
The monotile fixture succeeds here only as a useful adversarial case; it need
not outperform a regular grid.

### M2 - non-authoritative grouping mask

Project a qualified finite patch onto a disposable copy of the C3 rectified
domain without changing domain identities. Compare monotile grouping with the
current physical-signature components, Voronoi, regular chunks and multiscale
field thresholding.

Metrics: boundary length per area, component-size distribution, orientation
bias, spectral peaks, disconnected islands, crop-extension churn, bytes,
lookup cost and deterministic replay. Include continuous cross-boundary field
fixtures; a visible hard biome seam is a failure.

### M3 - visual anti-repetition mask

Use tile and metatile identities only to select among identical-cost material,
palette or modular-layout variants. Compare at the same asset count and memory
budget with Wang tiles, blue-noise selection, hashed variants and authored
layouts.

Metrics: repeated-feature distance, low-frequency spectral peaks, memory,
generation time, draw/instance overhead and phone/PC legibility. Numerical
metrics cannot replace a later blinded pixel-level owner review.

### M4 - lazy hierarchy trial

Run only if M1 or M2 demonstrates a real target-local advantage. Compare a
versioned substitution tree with the existing bounded hierarchy descriptors.

Metrics: point/window lookup latency, nodes materialized, cache hit rate,
overlap identity, patch seam consistency, serialized bytes, replay and
migration cost. PC and mobile are the first performance targets; PlayStation
remains a desired next target, while other ports are considered only when the
incremental cost is justified.

### M5 - Forge retrieval and module-graph trial

Use one bounded monotile adjacency graph as a synthetic topology for knowledge
retrieval and module-boundary verification. Compare fault discovery and query
quality with the existing typed catalogue and simpler random, tree, lattice and
directed-graph fixtures. This trial may improve test diversity; it may not
change classification or module ownership from geometry.

### M6 - construction and semantic layout trial

Adapt a finite patch into typed rooms, panels or encounter nodes, then require
the normal construction and semantic validators to reject inaccessible,
unsupported, disconnected or functionally incoherent layouts. Compare with
regular grids, Wang layouts, Voronoi graphs and authored small cases.

### M7 - simulation control-volume trial

Only after a domain owner identifies a specific numerical weakness, compare an
irregular monotile-derived planar mesh with the existing regular or adaptive
mesh. Measure conservation error, stability, anisotropy, resolution error,
neighbour cost, remeshing/crop seams and mobile execution cost. A visually
interesting boundary is not numerical evidence.

### M8 - cross-system reuse trial

Run only after two target-local experiments independently succeed. Extract the
smallest common graph, hierarchy, codec, metric or inspector utility; compare
duplicated local adapters with a shared module. Reject sharing if it couples
domain authority, expands dependencies, weakens rollback or costs more to
maintain than the duplication it removes.

## Falsifiers and stop rules

Stop the candidate in a target when any of these occurs:

- patch extension changes identities or adjacency in the retained overlap;
- exact generation cannot be bounded, versioned or replayed;
- a regular-grid, hashed, blue-noise, Wang-tile or Voronoi baseline provides
  equal target benefit at materially lower complexity or cost;
- irregular addressing introduces unacceptable mobile memory, lookup or
  streaming cost;
- grouping produces artificial biome seams or overrides causal evidence;
- the observed benefit is only novelty in a hand-picked image and does not
  survive a blinded or adversarial portfolio;
- one successful local use is generalized to another subsystem without a new
  target-local baseline and falsifier.

No universal performance thresholds are frozen in this map. Each future
experiment must declare fixture-local budgets before it runs so results cannot
be accepted post hoc.

## Plan placement

This record is linked as retained evidence across C3-C7, G1 closeout and R1,
but it creates no new selectable master-program item and changes no dependency
or state:

1. Forge research/governance retains the corrected intake and cross-system
   audit as a regression case; no promoted Forge subsystem is retrofitted.
2. C3 may use M1 as an adversarial fixture and consider M2 or M7 only after a
   specific local gap; none is required for C3 closure.
3. C4 may consider M4 after C3 closes and only if earlier evidence shows value.
4. C5 may consider hierarchy containers only after C4 proves them.
5. C6 may consider M6 for semantic, construction and organism-local cases.
6. C7 may consider M3 for representation, aesthetics, assets and animation.
7. G1 closeout checks that any adopted use retained its local baseline and did
   not silently become universal; R1 owns actual platform performance.
8. A future implementation package requires its own design/readiness gate and
   owner authorization. This map alone grants none.

The preferred first order remains **M1 irregular graph fixture**, followed by
M4 or M3 only if a real receiving-system gap makes the comparison worthwhile.
M8 is the compounding step, not the starting assumption.

## Miscommunication analysis and engineered correction

The first pass interpreted "different locations" through the active C3
checkpoint and its nearest module neighbours. That was a defensible drift
guard but the wrong ordering: it localized before applying the owner's
whole-system, recursively modular design principle. P16 required local proof
before application but did not explicitly require broad discovery before
localization.

The durable correction is P18 and its worker-governance regression:

`owner submits reusable mathematics/method -> extract mechanisms -> enumerate
Forge + game + runtime registry -> record fit/non-fit/duplicate -> identify
possible common utilities -> rank target-local trials -> apply P16 locally ->
share code only after multiple proven consumers`

The failure signal is simple: a future mathematical or systemic proposal is
mapped only to the active package or nearby modules without either a complete
registry pass or an explicit owner instruction limiting scope.
