# Built-in viewport controlled-stimulus contract

This capability-free bridge binds Forge's exact `reference-viewport` output to
the existing P7b controlled-perception protocol. It does not execute, import,
install, approve, promote, or mutate project content.

## Required package

- One reference stimulus and exactly three deliberate negative controls:
  `broken_connection`, `silhouette_collapse`, and `articulation_drift`.
- Every stimulus is bound by its exact SHA-256 scene fingerprint.
- The renderer profile, front/side/top views, two pose frames, canvas profile,
  and environment conditions are explicit.
- The protocol retains duplicate-pair, swapped-order, and metric-contradiction
  controls alongside the three viewport-specific controls.
- Pending observations use `not_observed`, confidence zero, and
  `awaiting_owner_observation`; they are placeholders and never owner claims.
- Analysis counts are recomputed from the bound observation set. The package
  reports `observed_claim_count=0` until a direct owner action is recorded.
- The active reference binds `artifact-reference-viewport-002`; older owner
  receipts remain evidence for their original fingerprint and are never
  silently rebound to the repaired fixture.
- The v2 articulation control preserves both forearm segment lengths while
  changing the pose-frame-1 joint direction. A stretched alternative is an
  invalid test fixture, not an articulation result.

## Fail-closed rules

Missing declared controls, stale scene references, invalid protocol bindings,
fabricated counts, unknown data, or authority language invalidate the package.
The module has no filesystem, network, process, desktop, engine, or protected-
Kernel dependency.

This proves deterministic controlled stimuli, not asset quality, player
preference, production rendering, runtime compatibility, or owner approval.

## Direct owner-observation entry

The optional entry begins with no pair, outcome, or confidence selected. It is
limited to the three visible negative-control pairs and must bind the exact
current base scene fingerprint. `not_observed`, stale fingerprints, hidden
protocol controls, confidence outside 1–100, and invalid packages fail closed.

A valid direct action produces a deterministic
`OwnerObservationReceipt` for one `CreativeDirector` / `ProjectDirection`
observation. The receipt reports `authority_effect=none`; it is returned to the
owner and does not approve, promote, publish, imply population preference, or
mutate the protected Kernel.
