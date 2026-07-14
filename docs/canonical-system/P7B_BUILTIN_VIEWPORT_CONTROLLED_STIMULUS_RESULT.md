# P7b built-in viewport controlled-stimulus result

## Outcome

Forge now constructs one deterministic reference stimulus and three deliberate
bad controls entirely inside the existing built-in viewer:

| Control | Deliberate change | Assertion exercised |
|---|---|---|
| Broken connection | Removes one declared support edge | Connection completeness |
| Silhouette collapse | Pulls both articulated spans into the centreline | Silhouette distinction |
| Articulation drift | Moves the articulated endpoints only in pose frame 1 | Temporal coherence |

All four scene snapshots retain strict inert data validation and have distinct,
repeatable SHA-256 fingerprints. The package binds the exact renderer profile,
front/side/top views, two pose frames, existing canvas environment, and six
protocol pairs: the three named controls plus duplicate-pair, swapped-order,
and metric-contradiction controls.

## Owner-observation boundary

No owner judgment was inferred. Every pending protocol observation is
`not_observed`, confidence `0`, with reason `awaiting_owner_observation`.
All satisfied, violated, preference, and indeterminate analysis counts remain
zero, and the bundle reports `observed_claim_count=0`.

Forge Desktop exposes the exact bundle read-only. Its stimulus selector renders
the reference or any deliberate control on the existing canvas. No external
program, engine, package, project, script, plugin, network, or filesystem
capability was added.

## Verification and repair

- `reference-viewport`: 8 tests pass.
- `perception-protocol`: 18 tests pass.
- `viewport-stimulus`: 6 tests pass.
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

Implemented: the Forge viewport now offers a blank-by-default direct
observation form for the three visible controls. The capability-free receipt
builder rejects stale fingerprints, `not_observed`, hidden pairs, and confidence
outside 1–100; nine module tests, twenty-six desktop tests, and the UI build
pass. A returned receipt has `authority_effect=none` and is not persisted as
approval or promotion.

The next action is direct owner review in Forge. Until the owner deliberately
submits an outcome, the canonical controlled-stimulus package remains at zero
owner claims and automation must not invent one.
