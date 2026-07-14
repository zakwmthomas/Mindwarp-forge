# P7b Controlled Perception Design Gate

**Status:** bounded P7b-0 protocol reference verified. P7b-1 containment and
P7b-2 actual review remain separately gated. This package rendered nothing,
imported nothing, selected no tool or format, and established no art direction
or quality threshold.

## Decision summary

P7b must not be a renderer demo or a single quality score. It should be three
separate evidence lanes:

1. P7a structural evidence remains authoritative for contract, lineage, and
   functional validity.
2. A controlled stimulus lane may later create immutable views or sequences
   under a pinned tool and environment profile.
3. A human observation lane records assertion-specific judgments under a
   predeclared protocol. It does not rewrite structural evidence or grant
   promotion.

The recommended next implementation is **P7b-0 only**: a capability-free data
contract and adversarial validator for review protocols, environment profiles,
stimulus manifests, observations, and analysis receipts. It creates no image,
runs no tool, and adds no filesystem, network, process, GPU, engine, runtime, or
promotion capability. A later P7b-1 containment/tool trial and P7b-2 controlled
owner review remain separately gated.

This staging is deliberately conservative. It preserves the original artifact,
all derivatives, all view conditions, and every disagreement. It prevents a
first renderer, display, metric, format, or chosen beauty angle from becoming a
permanent quality ceiling.

## Current primary-source research

Sources were checked on 2026-07-13. They are evidence, not tool or policy
endorsements.

| Source | Relevant practice | Forge implication and retained limit |
|---|---|---|
| [ITU-R BT.500-15](https://www.itu.int/rec/R-REC-BT.500) | Subjective assessment declares viewing conditions, source material, observers, session design, anchors, analysis, and result presentation. | P7b must pin the conditions and protocol before viewing. This is a graphics adaptation, not a claim of BT.500 laboratory compliance. |
| [ITU-T P.910 (10/2023)](https://www.itu.int/rec/T-REC-P.910-202310-I/en) | Provides absolute, degradation, comparison, and pair-comparison methods; reporting must expose experimental choices. Pair comparison is sensitive to small differences but grows combinatorially. | Use blinded randomized pairs with a `no_preference` option for close alternatives, bounded sampling, and explicit uncertainty. Do not manufacture a population-quality claim from one owner. |
| [ACES output-transform documentation](https://docs.acescentral.com/system-components/output-transforms/) | A display-ready result depends on a declared output transform, display characteristics, and viewing conditions. | Record the color configuration and display transform as evidence. Never treat unpinned screenshots or differently transformed outputs as equivalent. ACES itself is not selected. |
| [Subjective and Objective Visual Quality Assessment of Textured 3D Meshes](https://doi.org/10.1145/2996296) | Paired comparisons found that geometry, texture, and rendering conditions interact. | Geometry-only, image-only, or one-lighting metrics cannot close perception. Test combined faults and multiple controlled presentation modes. |
| [Attacking Perceptual Similarity Metrics](https://arxiv.org/abs/2305.08840) | Common perceptual metrics can reverse judgments under imperceptible adversarial perturbations. | Automated metrics may triage or flag regression only. They never approve, rank globally, or override an assertion-specific human observation. |
| [Blender scripting and security](https://docs.blender.org/manual/en/latest/advanced/scripting/security.html) | Scene files can contain Python text blocks, drivers, and other paths to script execution; disabling automatic execution does not remove every manual execution path. | Any future DCC/render tool is an executable capability boundary. Imported native project files, scripts, drivers, plugins, and auto-execution remain forbidden in the first trial. Blender is not selected. |
| [glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html) | Assets can reference external resources, data URIs, extensions, and optional implementation-specific URI forms. | A future format allowlist and parser budget are necessary; a file extension or schema pass is not containment. glTF is not selected. |
| [Windows Sandbox configuration](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file) | Networking, clipboard, mapped folders, vGPU, devices, and protected-client mode are distinct controls; defaults expose network, clipboard, and vGPU. | If evaluated later, defaults are unacceptable. Start with network/clipboard/devices/vGPU disabled, protected client enabled, read-only inputs, and a fresh dedicated output quarantine. Windows Sandbox is only a candidate, not an approved runner. |
| [OWASP File Upload Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html) | Content type and extension are spoofable; parsers face malicious payloads, resource bombs, overwrites, and active content. | Treat every imported asset and every tool output as untrusted bytes. Apply allowlists, size/depth/count budgets, quarantine, signature/content checks, and no direct project writes. |

## P7b-0 capability-free protocol contract

`P7a ReviewCase -> ReviewProtocol + EnvironmentProfile -> StimulusManifest -> ObservationSet -> AnalysisReceipt`

P7b-0 validates these records using tiny inert synthetic identifiers. It does
not resolve an artifact reference, create a scene, launch a process, open an
image, or decide visual quality.

### `ReviewProtocol`

- Binds exact P7a artifact and derivative fingerprints plus a protocol version.
- Declares the question, assertion IDs, eligible presentation method, reference
  availability, randomization/blinding method, anchors and failure controls,
  replay policy, stop rule, and allowed outcomes before stimuli exist.
- Uses assertion outcomes such as `satisfied`, `violated`, `indeterminate`, and
  `not_observed`; no universal weighted quality score is permitted.
- Separates directorial preference, defect detection, recognisability,
  functional legibility, temporal continuity, and comparative fidelity. One
  cannot silently stand in for another.

### `EnvironmentProfile`

- Pins a renderer/tool profile ID and binary/config fingerprints without
  selecting a product in the contract.
- Declares OS, CPU/GPU class, driver, deterministic seed, sampling controls,
  coordinate and unit profile, camera and projection, framing, resolution,
  time samples, lights, background, material/display mode, color configuration,
  output transform, and declared display/viewing conditions.
- Classifies reproducibility as `exact_same_environment`,
  `semantic_cross_environment`, or `unverified`. Cross-GPU exact pixel identity
  is never assumed.
- Unknown or missing conditions make the stimulus incomplete; defaults may not
  be reconstructed after viewing.

### `StimulusManifest`

- Binds immutable input fingerprints, environment profile, render request,
  output fingerprints, execution receipt, and all omissions/failures.
- Retains multiple complementary views or sequences where the assertion needs
  them. A chosen beauty angle cannot satisfy coverage by itself.
- Keeps diagnostic presentations distinct from representative presentations.
  Silhouette, flat/region views, normals, wire/topology, shaded appearance, and
  temporal sequences answer different questions and cannot be averaged into
  one result.
- Never replaces the source artifact or hides a failed/partial output.

### `ObservationSet` and `AnalysisReceipt`

- Each observation binds the exact stimulus, assertion, presentation order,
  reviewer class, outcome, confidence/uncertainty, reason code, limitation, and
  optional bounded note.
- The owner is identified as a creative-director reviewer. A single-owner
  result is valid evidence of project direction, not population preference or
  general recognisability.
- Repeated controls expose intra-review stability. Contradiction, control
  failure, missing coverage, or unstable replay yields `indeterminate`; it is
  never silently averaged away.
- Analysis retains per-assertion counts, order, disagreement, controls, and
  confidence intervals where applicable. It may compare candidates but cannot
  promote one, set a universal threshold, or rewrite P7a validity.

## Future P7b-1 containment boundary

No runner is authorized by this document. Any later trial must first select and
verify one disposable containment profile with these minimum properties:

- pinned tool binary, dependency and license inventory; no auto-update;
- no credentials, network, clipboard, audio, video, printers, host UI control,
  arbitrary plugins, startup scripts, drivers, macros, or native project files;
- read-only content-addressed input containing only generated synthetic fixtures;
- a fresh dedicated writable output quarantine, never the repository or an
  existing user directory;
- strict byte, file-count, nesting, geometry, texture, frame, time, memory, and
  process limits; timeout and crash receipts are evidence;
- software/CPU rendering first. GPU or shared-device access is a separate
  attack-surface and reproducibility gate;
- outputs are scanned, content-hashed, manifest-checked, and copied into durable
  evidence only after the runner stops; no output is automatically trusted or
  promoted;
- hostile fixtures cover external references, traversal, symlinks/reparse
  points, decompression/resource bombs, unsupported extensions, scripts,
  plugins, malformed buffers, NaN/infinity, oversized dimensions, and partial
  writes;
- recovery proves runner disposal, project immutability, a clean retry, and a
  retained failed receipt.

The prior controlled-application Windows path escape is a hard negative fixture.
Passing P7a reference checks does not establish this future execution boundary.

## Future P7b-2 controlled review protocol

The smallest useful review is a bounded paired comparison of tiny synthetic
derivatives, not Mind Warp content or an end-to-end generator:

1. Pre-register one question and a small assertion set.
2. Produce identical-condition reference/candidate pairs plus obvious-good,
   obvious-bad, duplicate, swapped-order, missing-view, misleading-lighting,
   stale-derivative, and numerically-good/visibly-bad controls.
3. Randomize order and hide candidate identity; allow `no_preference` and
   `indeterminate` rather than forcing false certainty.
4. Review every required presentation mode and temporal sample. Record display
   conditions and any deviation.
5. Repeat a bounded subset later to measure reviewer stability.
6. Report per assertion. Metrics, structural tests, and owner judgments remain
   separate columns.
7. Retain all stimuli, failures, observations, limitations, and cost labels.
   Nothing promotes automatically.

## Adversarial failure matrix

| Failure | Required result |
|---|---|
| Assertion written after images are seen | Reject as hindsight-biased evidence. |
| Unpinned camera, lighting, color transform, time, or tool profile | Incomplete stimulus; no comparison. |
| One beauty view hides a fault | Coverage failure, even if the owner likes the image. |
| Candidate labels or order leak | Blinding failure is retained; rerun or mark indeterminate. |
| Forced choice when differences are invisible | Reject protocol; `no_preference` must remain available where appropriate. |
| Same derivative presented as both alternatives | Control must detect equivalence; otherwise review validity is indeterminate. |
| Geometry improves while texture/material legibility worsens | Separate assertions retain the trade; no scalar average. |
| Static frames pass while motion pops or contact fails | Temporal assertion fails independently. |
| Metric improves but human judgment worsens | Retain contradiction; metric cannot override observation. |
| Owner preference conflicts across repeat | Record instability; do not fabricate consensus. |
| Single-owner result described as general player preference | Reject claim-class mismatch. |
| Stale artifact, protocol, tool, config, or display profile | Quarantine as non-comparable evidence. |
| Tool follows an external reference or executes embedded content | Security failure; terminate and retain receipt. |
| Crash, timeout, resource exhaustion, or partial output | Failed evidence, clean disposal, no silent retry-to-pass. |
| Output attempts to write into the repository | Hard containment failure. |
| Cross-device pixels differ | Compare only under declared semantic equivalence; never relabel exact replay. |

## Whole-system reconciliation

| Boundary | Alignment result |
|---|---|
| P2 universe identity | Stimuli and observations are derivatives; no image, filename, camera, or tool identity becomes universe truth. |
| P3 fields/world rules | P7b may inspect a rendered consequence but cannot infer new physical, ecological, or cultural law from appearance. |
| P4 hierarchy/history | Canonical artifacts and review records are immutable. Display caches, residency, and temporary scenes remain disposable. |
| P5 significance/scheduler | Controlled review uses pinned samples, not live significance. It cannot create private LOD priority or runtime policy. |
| P6 semantics/construction | Perception can reveal a failed derivative or ambiguous presentation, but cannot back-write category assumptions into functional truth. |
| P7a representation | P7b binds exact decisions and lineage. Visual approval never cures an invalid manifest, repair, material region, articulation, or temporal map. |
| Research/control plane | Sources, contradictions, owner decisions, runner failures, and rollback receipts remain versioned evidence. Imported bytes grant no authority. |
| Reference Studio | May display already-verified stimuli and receipts read-only. It cannot browse arbitrary files, render, execute, edit, approve, or promote. |
| Asset factory/animation | P7b proves only the review method on synthetic derivatives. It does not authorize generators, rigs, solvers, assets, or animation. |
| Runtime adapter | No format, engine, hardware target, frame budget, or runtime quality policy is selected. |
| Kernel/authority | P7b-0 is serialized data only. P7b-1 would be a separately isolated executor and may never mutate protected Kernel state or infer owner authority. |

## Alternatives rejected or deferred

- **End-to-end asset generator now:** rejected because failures would confound
  representation, generation, rendering, perception, and containment.
- **Choose a renderer/format first:** deferred because the evidence schema and
  failure controls should determine tool requirements, not the reverse.
- **One automated perceptual score:** rejected because metrics can be
  adversarially wrong and hide assertion-specific trade-offs.
- **Unstructured owner screenshot review:** rejected because conditions,
  omissions, lineage, and repeatability disappear.
- **Formal multi-observer study now:** deferred as disproportionate before the
  protocol and containment lane work. Single-owner review is useful only when
  its claim class remains narrow.
- **Exact pixel hashes everywhere:** rejected across tool/hardware changes;
  retain exact replay only within a pinned environment and use declared
  semantic equivalence elsewhere.

## Exact next confirmation (satisfied)

Authorize Codex to implement and test only **P7b-0**, a capability-free Rust
protocol/receipt harness using tiny synthetic identifiers. It will validate
predeclared assertions, pinned environment conditions, immutable stimulus
lineage, randomized/blinded pair metadata, `no_preference` and `indeterminate`
outcomes, reviewer claim classes, failure controls, contradiction retention,
bounded analysis, deterministic receipts, and read-only ProofReceipt
integration.

This does **not** authorize rendering, image creation or inspection, asset or
animation generation, imported files, a renderer/DCC, tool installation or
execution, filesystem/network/process/GPU capability, a format, art style,
quality weights or thresholds, general-player claims, runtime or engine work,
credentials, spending, publishing, promotion, or protected-Kernel mutation.

In plain language: **may Codex build and test the rules for a fair, repeatable
future visual comparison, without making or viewing any images yet?**

The owner authorized continuation in the same researched, adversarially tested,
independent-then-integrated fashion. That authorization satisfied only the
bounded P7b-0 question above.

## Verified P7b-0 result

- The capability-free `perception-protocol` crate validates predeclared
  assertions, P7a artifact/derivative lineage, pinned environment conditions,
  blinded pair metadata, complementary presentation coverage, failure controls,
  repeat evidence, claim classes, contradiction retention, and recomputed
  per-assertion analysis.
- Eighteen independent/adversarial tests reject post-hoc assertions, forced
  choice, leaked labels/order, missing controls, beauty-view-only evidence,
  missing temporal or diagnostic coverage, stale bindings, single-owner
  population claims, duplicate-control false preferences, lost metric/human
  contradictions, missing observations, duplicate presentation order,
  fabricated summaries, unknown versions, and budget exhaustion.
- Critical review after the first green run added mandatory stimulus lineage,
  per-pair required-mode coverage, complete observation coverage, unique
  presentation order, and repeat-evidence checks.
- One Forge Desktop test stores only serialized P7b-0 evidence as a read-only
  ProofReceipt without changing Kernel object, event, candidate, or authority
  state. Ten-module boundaries exclude Kernel, desktop, filesystem, process,
  network, and renderer capability from the crate.
- P7b-0 creates and views no image and proves no visual quality. P7b-1 runner
  containment, any renderer/tool trial, imported input, P7b-2 human review,
  assets, animation, formats, runtime, and engine remain unproved and gated.

The follow-on local feasibility and containment decision is recorded in
`P7B1_CONTAINMENT_DESIGN_GATE.md`. It stages a capability-free P7b-1a policy
reference before any separately approved denial canary or renderer trial.
