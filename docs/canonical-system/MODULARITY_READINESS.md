# F4 Modularity Boundary Readiness

**Status:** Complete. Focused verification and the full Forge gate pass.

## Declared graph

- `forge-kernel`: no Forge-module dependencies; no desktop/UI or network
  imports.
- `forge-desktop`: depends only on `forge-kernel`; cannot import frontend
  source.
- `forge-desktop-ui`: no direct Forge-module dependency; cannot bypass the
  Tauri command surface or create filesystem, process, or network paths.

The verifier also compares declared dependencies with Cargo workspace path
dependencies and rejects unknown nodes, missing roots, and graph cycles.

## Focused proof

The live three-module graph passes. Adversarial fixtures prove simultaneous
forbidden imports are both reported, a two-node dependency cycle fails, and a
failure in one module does not suppress diagnostics for its neighbour.

This is a static architectural gate, not an authority grant or a claim that
all runtime behavior is proven. Existing kernel, desktop, UI, governance, and
worker gates remain mandatory.

## Full-gate receipt

`tools/verify.ps1` passed with the UI build, Rust build, 15 desktop tests, 56
kernel tests, worker proof harness, owner-notification fixtures, canonical
registry checks, and whitespace verification.
