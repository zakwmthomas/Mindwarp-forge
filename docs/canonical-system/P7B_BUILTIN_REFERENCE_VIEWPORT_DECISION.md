# P7b Built-in Reference Viewport Decision and Result

## Owner decision

The owner approved: **use Forge's built-in viewer and continue**. No Windows
edition upgrade, WSL, VM, Sandboxie, Windows Sandbox, Unity, Godot, Blender,
renderer, plugin, or new program is required by this package.

## Corrected threat model

The prior P7b-1 route assumed the first visual stimulus required launching an
arbitrary third-party renderer. That assumption was not required by the
original Forge architecture, which already calls for a Forge Studio viewport
and makes external tools optional adapters.

Security is therefore proportional to the thing being run:

- strict Forge-owned scene data rendered by existing Forge UI code is not an
  arbitrary-code boundary problem;
- an owner-selected official runtime such as Unity later needs provenance,
  package pinning, project separation, backup/rollback, clean-import, build,
  identifier, replay, and profiling controls;
- unknown scripts, plugins, packages, native projects, downloaded executables,
  and AI-generated code remain untrusted and cannot auto-run; and
- an external adapter that genuinely needs hostile-content execution remains
  separately gated at R1 and may require stronger containment.

The AppContainer/LPAC trials remain truthful retained evidence about one
possible future arbitrary-executable boundary. They are no longer a
prerequisite for Forge-owned data visualization or F5 progress.

## Implemented vertical slice

`crates/reference-viewport` now supplies:

- strict `ReferenceScene`, vertex, edge, and pose-frame records;
- bounded identifiers, coordinates, cardinality, and contiguous frame order;
- complete vertex identity across every pose;
- deterministic integer front, side, and top orthographic projection;
- a SHA-256-bound exact scene snapshot and explicit renderer profile;
- a neutral two-frame articulation fixture; and
- seven adversarial tests covering unknown fields, executable/path/network/
  markup tokens, coordinate/cardinality overflow, duplicate/dangling edges,
  frame identity drift/gaps, exact axes, and repeatability.

Forge Desktop exposes the snapshot through one read-only Tauri command. The
existing TypeScript UI draws three synchronized canvas views and a pose-frame
slider. It adds no package or program and cannot fetch, import, execute, edit,
approve, or promote content.

## Claim boundary

This result proves deterministic built-in wireframe projection of one exact
typed fixture. It does not prove an asset factory, general geometry, materials,
lighting, shaders, physics, contact, production animation, runtime performance,
Unity compatibility, recognisability across categories, player preference, or
final visual quality.

## Adversarial safeguards

| Failure | Required result |
|---|---|
| Unknown scene field or schema version | Reject before projection. |
| Path, URL, markup, executable-shaped, or oversized identifier | Reject as non-inert. |
| Duplicate vertex/edge, dangling endpoint, self-edge | Reject. |
| Coordinate, vertex, edge, or frame budget exceeded | Reject. |
| Pose loses/replaces a vertex or skips an index | Reject. |
| UI attempts direct network or Node filesystem/process access | Existing module boundary rejects the build. |
| Snapshot claims material/runtime/general quality | Reject as an overclaim. |
| External program becomes an implicit prerequisite | Stop at a new owner/tool decision. |

## P10 boundary

- **Baseline:** the external-renderer assumption produced repeated OS-level
  containment trials without generating a visual stimulus.
- **Verified gain:** the built-in slice produces three synchronized views and
  two pose frames with seven passing adversarial tests and no new dependency.
- **Implementation cost:** one small capability-free Rust module, one bounded
  desktop command, and one canvas panel.
- **Operating cost:** deterministic integer projection of at most 256 vertices,
  512 edges, and 32 frames in this profile.
- **Uncertainty:** wireframes cannot establish material, lighting, physical, or
  production-runtime quality.
- **Regression guard:** module boundaries forbid file/process/network access;
  strict decoding and claim limitations remain tested.
- **Stop/refocus:** do not build a general renderer. Add only the smallest
  stimulus features needed to discriminate the next explicit P7 assertion.

## Exact next action

Completed by `P7B_BUILTIN_VIEWPORT_CONTROLLED_STIMULUS_RESULT.md`: the exact
snapshot is bound with broken-connection, silhouette-collapse, and
articulation-drift controls, and all owner outcomes remain `not_observed`.
