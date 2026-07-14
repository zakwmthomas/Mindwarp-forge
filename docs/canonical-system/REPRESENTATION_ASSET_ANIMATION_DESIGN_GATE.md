# Representation, Asset, and Animation Design Gate

**Status:** bounded P7a reference verified. P7b remains separately owner gated
through `P7B_CONTROLLED_PERCEPTION_DESIGN_GATE.md`. This record does not select
a file format, renderer, engine, representation algorithm, material system,
rig, solver, or art style.

## Decision summary

P7 must be split. A capability-free **P7a contract harness** can prove strict
representation decisions, artifact lineage, functional material regions,
articulation plans, temporal-fidelity mappings, repair history, and structured
review conditions using synthetic records. A later **P7b perception atlas** must
render controlled derivatives and obtain structured human review. P7a cannot
claim recognisability, visual quality, contact quality, or production cost, and
P7b cannot begin until its renderer/tool boundary and owner-review protocol are
separately approved.

This is smaller and safer than one end-to-end asset generator. It preserves the
project's causal chain without permanently lowering quality to whatever a first
renderer, mesh format, synthetic score, or hand-authored category table happens
to support.

## Recovered evidence audit

The fixed survival pack was inspected in memory. No recovered code, image, rig,
mesh, or report was executed or imported.

| Evidence | SHA-256 | Useful hypothesis | Failure or limit retained |
|---|---|---|---|
| `p09` `mindwarp_forge_sprint3.zip` | `366d199e176f0fc742a4cfd8c1105501f688f9e0cbc9a01c43a71ddc14876fe8` | Animation eligibility should be gated before expensive work. | Category labels directly select `skeletal_biped` or `none`; confidence is hand-authored and runtime/file capabilities leak into the prototype. |
| `p23` `mindwarp_holographic_field_pipeline.zip` | `ea24f3c9f2e76e61bdad0c04341c5e4ccdd1c829b06d2b82c6cee432404a3dbd` | A representation remains a derivative of stable causal inputs rather than universe truth. | Float thresholds and hard-coded families do not prove representation or perception. |
| `p29` `mindwarp_factorized_expression_experiment.zip` | `74b7f33e1ee4dc931a1178e2d43b27ba61ec747a2904cb29f477261d26c21fb2` | Structure, material expression, and derivative recipes can retain separate identities. | Culture and quality are hand-authored float vectors; truncated hashes, global weighted scores, and category names cannot become canonical policy. |
| `asset_category_contracts.json` | `1fe453c604ce855c2046707ae55fc71c782d2f5bf8b0dd70847aab4b6555eac0` | Required interfaces and forbidden systems are useful negative fixtures. | Category tables pre-decide rigs, supports, repairs, and materials, duplicating P6 functional truth. |
| `cross_category_contract_pipeline_report.json` | `0d1ef658299b087ea376325074ab26c90da58d5b8eb737ac3109b7c993c1737f` | Cheap structural rejection should precede expensive derivation. | Microsecond Python timings and regenerated repairs are not engine cost or local repair proof. |
| `cross_category_animation_dispatch_report.json` | `3431c0edf5c96088b48ae585d2710b79ae069b3e929563e957ac6cbd5e6ae54c` | Static, articulated, deforming, and locomoting outputs need distinct plans. | Dispatch remains category-first and attaches named runtime modules. |
| `cross_category_material_generalisation_report.json` | `12655929c20b8c703de8d7a2235cf1d5bd8a485500ab208b521c98bf0d62b71e` | Material evidence should expose ambiguity and ablation failures. | Synthetic authored features reached perfect classification while every ablation had zero effect; it explicitly is not perception proof. |
| `3d_material_surface_and_local_repair_report.json` | `17240d773a0c787082035adad50bffeced6b4a8ea3870f6dff1f618d8cd2aa8d` | Original, damage, repair candidate, and changed scope need separate lineage. | Its damage detector failed and the repair changed unrelated voxels with zero precision/recall. |
| `pipeline_information_retention_report.json` | `a64e19f9c6874795d3437940ba486ade94368c4661fafa0950dbd3fd62c5cfae` | Render channels can erase semantic distinctions, so controlled derivatives must test information retention. | Correlations are synthetic and explicitly do not measure aesthetic quality. |
| `real_3d_voxel_weapon_test_report.json` | `12e00c572d644dc8f0d82dbf99aeb1184cbb0c9148e40a7194b8b4a1baa072f2` | Orientation, anchor, silhouette, and representation-specific limits need explicit evidence. | Category labels feed the generator; repair regenerates a nearest family; there are no materials, rigs, or human review. |

The recovered material therefore supports fixture ideas, not the registry's old
`prototype_tested` label for the representation selector. That status is
repaired to `specified` until a new bounded P7a harness passes.

## Primary-practice reconciliation

- glTF 2.0 makes coordinates and units explicit, separates optional display
  names from indexed references, and defines animation key storage and declared
  interpolation while deliberately not defining playback policy. P7 records
  units, frames, and interpolation/fallback semantics but does not select glTF:
  <https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html>.
- OpenUSD demonstrates non-destructive composition and logical asset
  references, but also permits plugin-defined resolvers, URIs, and filesystem
  search. P7a adopts logical, content-bound references while rejecting all path
  resolution, URI fetching, plugins, and I/O:
  <https://openusd.org/release/tut_referencing_layers.html> and
  <https://openusd.org/release/api/ar_page_front.html>.
- MaterialX is an open graph-based interchange standard with versioned node
  definitions and validation. P7 records functional material regions and a
  future format profile, but does not adopt its shader vocabulary or renderer:
  <https://materialx.org/Specification.html>.
- Progressive meshes show that geometric derivatives can preserve a reversible
  refinement lineage instead of becoming unrelated assets. P7 therefore makes
  every fidelity derivative bind its parent and error evidence, without
  selecting progressive meshes: <https://hhoppe.com/proj/pm/>.
- Subjective textured-mesh research uses controlled render protocols and paired
  comparison, and finds geometry and texture errors interact. Numerical mesh
  error alone therefore cannot satisfy P7b perception proof:
  <https://doi.org/10.1145/2996296>.

These are design lessons, not format endorsements. Current versions are source
evidence; any later adoption requires a pinned profile, compatibility matrix,
validator, license/security review, migration plan, and owner decision.

## P7a contract boundary

`Validated P6 package + optional P5 packet -> RepresentationPortfolio -> RepresentationDecision -> ArtifactManifest -> MaterialRegionPlan + ArticulationPlan + TemporalFidelityPlan -> ValidationReport -> ReviewCase`

### Shared envelope

Every record has a strict schema version, stable record ID, exact dependency
fingerprints, claim class, bounded canonical bytes, and an explicit status.
Unknown fields, enum values, dependency versions, units, frame conventions,
reference schemes, and operation types fail closed. Display labels never define
identity or choose a representation.

### Representation portfolio and decision

1. Requirements come from validated P6 functional roles, interfaces, material
   regions, articulation/deformation needs, and declared quality/cost concerns.
2. A material decision retains at least two feasible mechanism-distinct options
   or an explicit `single_feasible_representation` receipt. Category defaults
   such as `creature -> skinned mesh` are forbidden.
3. Hard constraints are evaluated first. Remaining options retain named trade
   vectors; no universal weighted score or hidden winner is allowed.
4. Cost values are labelled `measured`, `simulated`, or `estimated`, with units,
   method, environment, uncertainty, and scope. P7a uses synthetic integer work
   counts only and makes no hardware, frame-time, memory, or renderer claim.
5. The decision describes a neutral representation family and required
   capabilities. It contains no engine object, executable generator, shader,
   solver, or final geometry/material file-format choice.

### Manifest and derivative lineage

1. `ArtifactManifest` binds the P6 recipe fingerprint, representation decision,
   generator/profile version, output-kind declaration, structural validation,
   derivative lineage, repair attempts, and evidence links.
2. Artifact identity is content and contract based. A filename, path, URI,
   display name, cache key, or runtime import location cannot define identity.
3. P7a manifests contain inert logical references only. Absolute paths, parent
   traversal, drive/UNC paths, URI schemes, executable payloads, resolver
   plugins, environment expansion, and network/process instructions reject.
4. LODs, material variants, animation compressions, conversions, and repairs
   are derivatives. Each binds its exact parent, method/profile version,
   declared loss/error evidence, and validation result. They never rewrite the
   P6 recipe or replace the original evidence.
5. A failed or partial derivation is quarantined as evidence. Repair creates a
   candidate with declared changed scope; it never silently regenerates a
   nearest category or promotes itself.

### Materials and articulation

1. `MaterialRegionPlan` binds functional P6 region roles, boundaries,
   interfaces, units, and appearance constraints. It contains no shader graph,
   renderer state, texture path, culturally inferred style, or physical truth.
2. `ArticulationPlan` binds P6 part/socket roles to named local frames, degrees
   of freedom, limits, deformation/contact intent, and required evidence.
   Frames, handedness, units, and transform order are explicit.
3. Symbolic support/contact/collision relations can be checked in P7a, but they
   are not physics, structural engineering, skin deformation, gait, or contact
   solver proof. Those claims remain indeterminate until their proper harnesses.
4. An articulation plan cannot select a skeleton or animation family from a
   category name and cannot attach an engine component.

### Temporal fidelity

1. `TemporalFidelityPlan` consumes a P5 `ImportancePacket` plus a declared,
   monotone consumer fidelity map. It cannot recompute significance, add
   hysteresis, or store a private global priority.
2. Each tier declares cadence, sampling, interpolation, transition, fallback,
   error evidence, and dependency requirements. Missing or incompatible
   evidence rejects or returns `indeterminate`; it never guesses.
3. Temporal changes cannot alter canonical recipe, representation, or
   articulation identity. Stale-epoch derivatives are quarantined under P5.
4. P7a tests only plan consistency and deterministic mappings. Motion quality,
   temporal popping, contact, and recognisability require P7b sequences.

## P7b perception boundary

P7b is a later controlled evidence package, not part of P7a authorization.

- A `ReviewCase` pins artifact/derivative IDs, renderer and profile versions,
  camera, projection, framing, resolution, lighting, background, tone/color
  handling, time samples, and assertions before images are viewed.
- A `VisualReviewReceipt` records blind or paired protocol where applicable,
  reviewer identity class, observations per assertion, limitations, and links
  to immutable derivatives. Free-form `looks good`, an AI image, or one chosen
  beauty angle cannot pass.
- Structural metrics, image metrics, and human judgments remain separate.
  Metrics may triage; they do not replace the required perception review.
- The reference must include failure controls: silhouette loss, material-region
  collapse, topology/contact faults, temporal popping, misleading lighting,
  camera omission, stale derivative, and a numerically good but visibly bad
  candidate.
- Owner review may accept or reject a candidate but never silently promote a
  format, engine, style, threshold, or universal policy.

## Whole-system alignment

| Boundary | Preserved rule |
|---|---|
| P2 identity | Logical object and recipe identities remain upstream; paths, files, caches, and renderer outputs are derivative. |
| P3 fields/world rules | P7 may cite validated conditions but cannot invent physical, ecological, or cultural truth. |
| P4 hierarchy/history | Reusable artifacts are immutable canonical objects; instance placement and mutations stay explicit deltas. Residency and caches cannot enter artifact identity. |
| P5 significance/scheduler | One shared packet feeds a declared consumer map. P7 has no private priority, hysteresis, executor, cache, or budget controller. |
| P6 semantics/construction | Functional roles and recipes remain canonical. P7 selects a derivative form and cannot back-write category assumptions into P6. |
| Reference Studio | Displays records, failures, lineage, conditions, and review evidence read-only. It cannot resolve/fetch assets, render arbitrary content, edit, approve, or promote. |
| Runtime adapter | Receives only later promoted neutral manifests. Runtime identifiers and measured performance never redefine canonical meaning. |
| Kernel/authority | P7 evidence is serialized data only. No protected-Kernel, filesystem, network, process, credential, spending, publishing, or promotion capability is added. |

## Adversarial proof matrix for an authorized P7a reference

| Fixture | Required result |
|---|---|
| Label/category permutation | Identical functional inputs retain identical decision meaning. |
| Category-default injection | Rejected because no grounded requirement chose the form. |
| Fake alternatives by rename | Does not satisfy mechanism-distinct representation diversity. |
| Hidden scalar winner | Rejected; hard feasibility and named trade evidence remain inspectable. |
| Units/handedness/frame omission | Localized strict failure. |
| Recipe or dependency drift | Reject/migrate decision; no silent reinterpretation. |
| Path/URI/executable payload | Rejected as inert manifest data cannot resolve or execute it. |
| Parent traversal, drive, UNC, environment expansion | Rejected before any reference is accepted. |
| LOD/repair without parent | Rejected; original lineage remains unchanged. |
| Repair outside declared scope | Rejected and quarantined. |
| Material region not grounded in P6 | Rejected without selecting a shader. |
| Articulation chosen from label | Rejected; explicit role/frame evidence is required. |
| Non-monotone or private temporal map | Rejected against the P5 contract. |
| Stale epoch or incompatible fallback | Quarantined/rejected with exact reason. |
| Budget exhaustion | `indeterminate_budget`, never automatic best or impossible. |
| Review with missing conditions | Incomplete evidence; never a quality pass. |
| Numerical pass without perception | P7a structural result only; P7b remains unsatisfied. |
| Read-only receipt integration | Recording/viewing changes no Kernel object, event, candidate, authority, approval, or promotion state. |

## Recovery, versioning, and limits

- Version migration creates an explicit receipt linking old bytes, new bytes,
  adapter/profile, losses, and result. It never rewrites historical manifests.
- A changed generator, representation profile, material profile, articulation
  profile, temporal map, renderer condition, or review protocol produces new
  evidence. Caches are disposable.
- Validation consumes a bounded integer work allowance and reports examined
  records, options, references, plans, derivatives, and violations.
- P7a may contain only tiny synthetic function roles and records sufficient to
  discriminate the matrix. They are not Mind Warp content, art direction,
  vocabulary, weights, formats, or quality thresholds.
- Still open: representation families and comparison dimensions, neutral
  geometry/material/animation formats, coordinate profile, generator, topology,
  LOD and compression algorithms, material vocabulary, rigs, solvers, repair
  methods, renderer, cameras, lighting, perception protocol, thresholds,
  hardware classes, engine adapter, and production asset pipeline.

## Exact confirmation (satisfied)

Authorize Codex to write and run only a capability-free P7a Rust contract
harness using tiny synthetic records to prove strict representation portfolios,
decision rationale, content-bound manifest lineage, hostile-reference rejection,
functional material/articulation plans, P5-derived temporal mappings, bounded
validation, repair quarantine, deterministic receipts, and read-only
ProofReceipt integration.

This does **not** authorize geometry, meshes, voxels, textures, material/shader
graphs, rigs, animation generation, physics/contact solving, rendering, visual
review or P7b, AI generation, asset import/export, filesystem/network/process
access, a file format, runtime or engine integration, product vocabulary,
quality weights/thresholds, credentials, spending, publishing, promotion, or
protected-Kernel mutation.

## Verified P7a reference result

- Strict canonical records and 17 adversarial tests prove grounded
  mechanism-distinct portfolios, hard feasibility, named trade evidence,
  selected-decision rationale, and bounded deterministic validation.
- Artifact identity binds the P6 recipe fingerprint, representation decision,
  and generator profile. Content references reject paths, URIs, traversal,
  environment expansion, UNC/drive forms, and fingerprint mismatch without
  resolving or fetching anything.
- Ordered derivative lineage, repair-candidate retention, scope confinement,
  functional material regions, explicit articulation frames, P5-bound monotone
  temporal mappings, fallback checks, and P7b evidence exclusion are executable.
- One Forge Desktop integration test stores only serialized P7a evidence as a
  read-only ProofReceipt and changes no Kernel object, event, candidate, or
  authority state. Nine-module boundaries bar Kernel, desktop, filesystem,
  process, and network capability from the reference crate.
- The representation selector is `prototype_tested`. Asset factory and
  procedural animation remain `specified`; P7a produces no geometry, material,
  rig, animation, render, perception, runtime, engine, or performance proof.
