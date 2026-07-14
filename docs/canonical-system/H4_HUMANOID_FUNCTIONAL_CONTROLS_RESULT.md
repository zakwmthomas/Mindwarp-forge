# H4 humanoid functional controls result

Status: **verified typed calibration**. The complete Forge gate passes.

Static analysis first identified three independent mutation dimensions. Five
in-memory fixtures then proved exact deterministic calibration, all three
non-detection sets, fixed H3/reference/control bindings, strict canonical
encoding, and fail-closed rejection of missing controls, cross-sensitive or
post-hoc claims, stale fingerprints, unknown fields, and noncanonical data.

After those cheaper tiers passed, the H4 verifier, 18-module boundary gate, UI
build, complete Rust workspace tests, desktop build, whitespace check, and
complete Forge verification gate all passed.

The calibration fingerprint is
`774a790aa963bb7ed329394d869fda4f5530697cce4d4d029a23d31e6d575f4d`.
The three metric deltas are exactly 1 edge, 480 fixture units of rest front-span
loss, and 480 fixture units of two-hand vertical displacement. These numbers
distinguish only the exact synthetic controls; they are not visual-quality,
anatomy, preference, or production thresholds.

No visual asset is used in H4. The wire fixture remains structurally useful but
is not accepted as a good-quality human. Actual-pixel inspection and potential
owner review remain mandatory before H5 uses any rendered human reference or
candidate.
