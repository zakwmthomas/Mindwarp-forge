# G1 GP1 fixed base-loop readiness

Status: owner-authorized bounded design and adversarial readiness; implementation
must remain engine-neutral and stop before GP2.

## Dependency and authority

GP0 is structurally verified through its strict concept, five corrected
authored sessions, deterministic outcome/history proof, exact typed C3A seam,
independent review, and complete Forge gate. That closure does not promote the
`mindwarp-gameplay-foundation` system beyond `prototype_tested`.

GP1 may extend that same isolated crate with one pure in-memory loop. It may
reuse `SessionRecordV1`, the five fixed fixtures, `C3AWorldReferenceV1`,
`SessionState`, `WorldHistoryV1`, and their strict validation. It may not add a
runtime, graphics, persistence, procedural content, grind, generic economy,
network, monetization, Greenfield dependency, C3B fact, or GP2 progression
system.

## Selected loop contract

Every fixed session uses the same ordered phases:

1. `prepare` selects a named intent, fitting-tool loadout, and threat plan,
   but never a terminal outcome;
2. `depart` commits that plan before leaving the stable hub or vessel;
3. `encounter` observes and infers before the player selects an outcome, then
   executes the existing make, optional threat-diversion, and typed outcome
   reducer;
4. `consequence` exposes the exact mutations, costs, memories, grants, and next
   decision already owned by GP0;
5. `return` brings that immutable consequence package back to the stable hub;
6. `remembered_response` appends it to `WorldHistoryV1` without overwrite.

Preparation, consequence, return, recoverable failure, and terminal
remembered response expose typed stable stops with an exact resume action.
Departure and ordinary encounter are not silently declared safe stops.
Remembered response is terminal only after the history append succeeds; its
resume action is `none`. A stable stop is a validated state, never an abort,
success, or authority claim.

Encounter failure is a typed overlay on the encounter phase. It records a
named reason, exact opportunity cost, recovery count, and resume action while
retaining the prepared plan, optional world context, existing history, and
prior remembered consequences. Recovery clears only that failure and re-enters
the same encounter; because no outcome has yet been selected, it cannot
predetermine or rewrite the encounter decision. At most three recoveries are
permitted. A fourth failure attempt is rejected before mutation, leaving the
third recovered encounter state unchanged.

Authored fixtures carry an explicit `authored_fixture` world context that is
authority-negative and has no C3A identity. Only a packet validated through the
existing exact C3A seam may carry `validated_c3a` identity; neither context
claims C3B authority.

Every loop has a typed `run_id`. The append ledger stores typed
`run_id`/event-sequence/session/outcome receipts and the first GP1 event sequence in addition to
the immutable `WorldHistoryV1`. Remembered response atomically appends the
outcome and receipt, rejects a duplicate run or second response from the
terminal state, and permits a legitimate later play of the same session only
under a new run ID. Pre-GP1 history may precede the GP1 event floor without
receipts; every event at or after that floor must have exactly one receipt.

S5 deterministically uses the latest preceding S1 history event. This keeps
repeated legitimate S1 runs usable while still rejecting missing history or a
latest retreat. The selected event sequence is preserved by ordered history;
no chat or heuristic chooses the predecessor.

## Later-state proof

The S1 decision must materially change S5. Direct, bypass, and ration each
produce distinct S5 colony state and Keeper Mara memory through the GP0
predecessor-sensitive reducer. S1 retreat remains inadmissible as an Afterlight
trigger. The GP1 loop must run S1 to remembered response, then initialize S5
from that exact history and prove the later trace differs by the earlier
decision while preserving ordered S1 then S5 events.

## Adversarial matrix

The focused proof must reject:

- phase skipping, repetition, action after terminal response, or return before
  consequence;
- a plan for a different session, an outcome selected during preparation, an
  unknown encounter outcome, unsupported threat diversion, or retreat
  committed as an intervention;
- fabricated public loop state, noncanonical bytes, unknown fields, stale C3A
  identity, and a foreign world packet;
- a stable stop in an unsafe phase, a stop without an exact resume action, or a
  resume action that does not reproduce the same state;
- failure outside encounter, recovery without failure, a fourth failure after
  three recoveries, failure that predetermines an outcome, or recovery that
  edits history;
- remembered response before return, duplicate append, missing or fabricated
  append receipt, reordered history, overwritten S1 evidence, S5 without an
  admitted latest predecessor, or latest S1 retreat triggering S5;
- combat completing a session, currency or grind counters, authored facts
  acquiring C3A/C3B proof, or imports of filesystem, process, UI, runtime,
  network, Greenfield, or C3B owners.

## Readiness decision

The selected design is bounded, reversible, and falsifiable with typed and
in-memory fixtures. Test-first implementation is authorized inside
`mindwarp-gameplay-foundation`; GP2 and all runtime work remain forbidden until
the GP1 result is independently reviewed and canonically transitioned.
