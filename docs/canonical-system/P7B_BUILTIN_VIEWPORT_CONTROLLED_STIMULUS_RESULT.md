# P7b built-in viewport controlled-stimulus result

## Outcome

Forge now constructs one deterministic reference stimulus and three deliberate
bad controls entirely inside the existing built-in viewer:

| Control | Deliberate change | Assertion exercised |
|---|---|---|
| Broken connection | Removes the right knee-to-foot support edge | Connection completeness |
| Silhouette collapse | Pulls the elbow and hand spans into the centreline | Silhouette distinction |
| Articulation drift | Rotates the pose-frame-1 forearms while preserving segment lengths | Temporal coherence |

All four scene snapshots retain strict inert data validation and have distinct,
repeatable SHA-256 fingerprints. The package binds the exact renderer profile,
front/side/top views, two pose frames, existing canvas environment, and six
protocol pairs: the three named controls plus duplicate-pair, swapped-order,
and metric-contradiction controls.

## Owner-observation boundary

The generated bundle still begins with no inferred judgment: every pending
protocol observation is `not_observed`, confidence `0`, with reason
`awaiting_owner_observation`, and `observed_claim_count=0`. Direct receipts are
separate, explicit owner evidence. They do not rewrite that safe default or
become approval, promotion, population preference, or protected-Kernel state.
The blank-by-default direct entry remains the only supported observation path.

Forge Desktop exposes the exact bundle read-only. Its stimulus selector renders
the reference or any deliberate control on the existing canvas. No external
program, engine, package, project, script, plugin, network, or filesystem
capability was added.

## Direct owner review result

The owner completed the three visible v1 comparisons through exact labelled
Forge-window captures. Forge produced deterministic, authority-negative
receipts bound to the current base scene:

| Control | Outcome | Confidence | Owner observation | Receipt SHA-256 |
|---|---|---:|---|---|
| `broken_connection` | `satisfied` | 90 | The altered version has no leg connection while the reference remains connected. | `f54b719ba5b0f20723f751d7fcadd8901f4ae932b093cc61ebe63d1d7c0312c3` |
| `silhouette_collapse` | `satisfied` | 90 | The altered version is incorrectly rotated at the legs. | `00d328acd8da49db903a05880805632e0c4dc9d75359d4d6635a965029d3a07a` |
| `articulation_drift` | `indeterminate` | 40 | Reference arms and legs look too short; alternative arms look too long; the simple sticks may poison certainty. | `3c26d4ca607e9b7f3d14bfe85fb9671ed4ba418bb9684cdeb0d9ca0e094cd7b7` |

The third result is a possible poisoned-fixture limitation in the simple-stick
comparison, not a pass. Review
completion means three observations exist; it does not prove that the fixture is suitable
or that every assertion is satisfied. The qualitative limb-length
feedback must remain visible in any repair or replacement stimulus.

## Poisoned-fixture repair result

The v1 implementation confirmed the owner's concern: its articulation control
moved each arm endpoint from roughly 150 horizontal units to 360, so the
comparison changed limb length and pose together. That fixture cannot isolate
articulation and its indeterminate receipt remains attached to the old scene
fingerprint.

The replacement is `neutral-articulation-fixture-v2`, bound to
`artifact-reference-viewport-002`. It replaces single-line limbs with named
shoulder/elbow/hand and hip/knee/foot segments. Every arm and leg segment has
squared length `14400` in the base and both pose frames. The altered pose keeps
the correct shoulders and elbows, changes only the hand direction, and retains
the same forearm length. The three controls therefore isolate connection,
silhouette, and articulation defects instead of stretching the compared arm.

The semantic gate and adversarial matrix now prove:

- short reference limbs fail before projection;
- length drift in a later pose frame fails before projection;
- the articulation control changes hand position while preserving both
  forearm lengths;
- the broken-connection control removes one explicit leg segment; and
- all controls remain deterministic, distinct, inert, and authority-negative.

This is implementation readiness only. The v2 fixture has not been judged by
the owner and does not replace or reinterpret any v1 observation.

## Direct v2 owner finding

The owner rejected the v2 reference as an animation baseline because the
character reads as a star formation rather than a standardized T-pose. No
confidence value was supplied, so Forge created no synthetic observation
receipt and inferred no numeric certainty. The mechanical v2 length-isolation
proof remains useful, but the v2 reference is not suitable for further owner
review or animation-readiness claims.

## Animation-ready baseline requirements

The v3 repair uses the smallest engine-neutral intersection of authoritative
practice:

- Autodesk HumanIK requires a standard T-stance with +Y up, the character
  facing +Z, arms aligned to X, flat hands, and feet perpendicular to the legs
  with toes facing +Z; it warns that an incorrect T-stance gives IK and
  retargeting solvers faulty proportion and joint-transform data
  ([Autodesk preparation guide](https://help.autodesk.com/cloudhelp/2026/ENU/Maya-CharacterAnimation/files/GUID-8D27BAFD-7785-4173-860E-515FEB2E9C98.htm)).
- Unity's Humanoid setup requires a proper T-pose after bone mapping and
  exposes `Enforce T-Pose` when the imported character is not correctly posed
  ([Unity Humanoid import guide](https://docs.unity3d.com/es/current/Manual/ConfiguringtheAvatar.html)).
- Blender separates the armature's default/rest position from pose offsets;
  rest transforms use neutral position/rotation and unit scale
  ([Blender armature posing manual](https://docs.blender.org/manual/en/4.5/animation/armatures/posing/introduction.html)).
- glTF 2.0 defines right-handed +Y-up, +Z-forward metre-space interchange,
  requires a common root for a skin's joint hierarchy, and binds joint order
  to inverse-bind matrices
  ([Khronos glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html)).

Forge therefore treats frame 0 as a declared rest/bind T-pose rather than an
arbitrary first animation frame. The reference must be bilaterally symmetric,
use character-left = +X, keep shoulders/elbows/hands collinear on X, keep
hips/knees/ankles vertical and parallel, point toes along +Z, retain one
directed pelvis-rooted hierarchy, and preserve limb lengths in later frames.
The integer fixture uses 100 units per metre; conversion to a future adapter
must be explicit rather than inferred.

## Verification and repair

- `reference-viewport`: 12 tests pass, including four v2 proportion and
  articulation-isolation tests.
- `perception-protocol`: 18 tests pass.
- `viewport-stimulus`: 9 tests pass.
- `forge-desktop`: 25 tests pass, including read-only bundle integration.
- The first focused run correctly rejected a repeat assertion that had only one
  observation. The declaration was repaired to repeat only assertions with
  paired evidence; no second owner observation was fabricated.
- Removal of a declared control and substitution of a stale scene reference
  both fail closed.

## Claim limit

This result proves deterministic stimulus construction and protocol binding.
It does not prove asset quality, recognisability, materials, lighting, physics,
runtime behavior, Unity compatibility, player preference, owner approval,
promotion, or protected-Kernel authority.

## Exact next action

The v2 fixture, focused proof gates, UI build, 31 desktop tests, and whole-Forge
gate pass. A fresh labelled window-only pose-frame-1 comparison was produced
with SHA-256
`a1ad6ec9f64b351b5ea3051401a5321056c4592d48f5b4cdc26b6a64d84a5165`.
Await the owner's direct observation while the heartbeat is paused. Keep the
v1 receipts immutable and do not convert the prior indeterminate result,
implementation completion, or screenshot delivery into approval.
