# G1 GP4 Signal Anchor readiness

Status: owner-authorized, decision-complete pre-source boundary. Source remains
forbidden until this package passes the focused readiness verifier and an
independent reviewer explicitly accepts it.

## Objective and dependencies

Prove one runtime-independent vertical:

`fixed hub frame -> prepare -> Signal Anchor route -> temporary rescue with optional wire-scavenger diversion -> consequence -> return -> remembered response -> restart`

The only admitted dependencies are the exact verified C3A seam, recorded C4V
proof, GP0 fixed S4 session, GP1 reducer, GP2 fixed projection and GP3 fixed S4
encounter grammar. `G1-VERTICAL-CLOSEOUT` is a separate bounded evidence
sibling. Broad `G1-CLOSEOUT` and R1 are unchanged.

## Dual-world authority proof

The primary run starts with `start_c3a_base_loop` and is persisted exclusively
through one real `VerticalLogV1`. A shadow starts with
`start_authored_base_loop`, using the same canonical S4 session, run ID,
`ledger_before`, preparation and five GP1 actions in the same order. The shadow
is authority-lowering evidence only: it never claims C3A or C3B truth.

The semantic projection destructures `BaseLoopStateV1` exhaustively and includes
all thirteen fields other than `world_context`:

1. `schema_version`
2. `run_id`
3. `session_id`
4. `phase`
5. `preparation`
6. `predecessor_outcome_id`
7. `session_state`
8. `ledger_before`
9. `ledger_after`
10. `failure`
11. `recoveries_used`
12. `stable_stop`
13. `trace`

No `..` remainder is permitted. A future upstream field must therefore break
compilation until classified. Canonical semantic projection bytes must be
identical for C3A and shadow runs; both `ledger_before` and `ledger_after` must
also be byte-identical. Their common digest is:

`SHA256("mindwarp.gp4.base-loop-semantics.v1\\0" || u64be(payload_len) || canonical_projection_bytes)`

Every one of the thirteen fields receives an independent hostile mutation
test. `world_context` is the sole admitted difference. Enforceable authority
checks are exact: the C3A variant must equal `bind_validated_c3a_world` output;
the shadow variant must equal `AuthoredFixture`; each state must pass its own
replay validation; decoding C3A bytes with expected authored context and
authored bytes with expected C3A context must reject.

## Real C4V evidence and prefix restart

The exact four command IDs and batches are frozen in the fixed registry. The
bundle stores and validates the real canonical C4V log, revision-3 snapshot,
revision-4 snapshot, persistence receipt and command-ID vector. It does not use
human labels as command authority.

After `BeginReturn`, revision 3 is semantically restarted from the C4V log and
its snapshot is rebuilt and compared. Only then may
`RecordRememberedResponse` be appended. The revision-4 log is restarted again;
its exact terminal semantics, snapshot and persistence receipt must reproduce.
Stored final state or snapshot bytes are never trusted without replay.

## GP3 and GP2 binding

The bundle stores the exact strict GP3 S4 situation bytes plus approach ID,
GP4-derived approach-reference digest, selected threat ID, the upstream GP3
threat digest and a GP4-derived threat-reference digest. The two GP4 domains,
length framing and exact fixed values are frozen in the registry. Validation
resolves these through `fixed_encounter_grammar`, verifies
the temporary approach belongs to S4, verifies complete GP0 consequence
coverage, composes the optional nonterminal threat and requires it to match the
GP1 diversion trace.

GP2 accepts only the terminal authored shadow. Construction begins with
`ProgressionLedgerV1::from_base_loop(shadow.ledger_before)` and calls the real
`apply_progression`. The canonical ledger bytes, its single real
`ProgressionReceiptV1`, exact emitted IDs and exact world-transition IDs must
match the fixed registry. GP4 does not copy a private rule, fabricate a receipt,
grant a capability or apply GP2 to C3A state. The threat transition is world-only.

No invented `gp0_contract_digest` or equivalent field exists.

## Neutral presentation and accessibility contract

Each of the twenty-five fixed semantic slots binds through the registry's exact
resolver to a validated source in the run: hub status, player actor, absent
Iven, Signal Anchor opportunity, broken anchor, signal-window evidence,
wire-scavenger threat, collapse risk, temporary-brace tool, rescue choice,
intervention, work-area-safe threat mutation, six separate outcome mutations,
incomplete permanent repair, remembered response, next decision, and the four
distinct revision-1 prepared, revision-2 consequence, revision-3 return-prefix
and revision-4 terminal stable stops. Every slot
supplies text, a non-colour cue, a reduced-motion equivalent and a screen-reader
label. Equality with the fixed registry is required; free-form presentation
payloads are rejected.

These slots prove semantic coverage only. They do not prove pixels, audio,
animation, timing, accessibility conformance, creative quality or C3B facts.
No visual asset is used, so the visual-quality gate is not applicable.

## Adapter and performance requirements

The fixed twenty-nine-row hard/compare matrix is exact: sixteen hard and
thirteen compare rows, each with fixed question, required evidence, method and
target. Every status is `unmeasured`; there are no fabricated
frame time, memory, latency, device, platform or accessibility results. Runtime
selection stays at R1 and no engine, executable, path or URI is named.

## Codec and resource boundary

`SignalAnchorBundleV1` uses a strict `deny_unknown_fields` codec. Decoding checks
the 8 MiB top-level ceiling before parsing, checks all upstream nested byte
bounds after parse but before dependency traversal or semantic comparison, validates every dependency,
reserializes, and requires exact byte equality. Vectors and text are fixed or
bounded before equality. Duplicate, unknown, reordered, trailing, oversized,
malformed and noncanonical forms fail closed.

Bundle digest framing is:

`SHA256("mindwarp.gp4.signal-anchor.bundle.v1\\0" || u64be(canonical_body_len) || canonical_body_with_zero_digest)`

The bundle contains no filesystem, network, database, process, dynamic library,
script, URI, executable or runtime adapter capability.

## Required hostile matrix

Pre-acceptance tests must reject at least:

1. any of thirteen semantic-field mutations;
2. more than the sole `world_context` difference;
3. crossed, missing or fabricated C3A input/packet authority;
4. authored state presented to C4V or C3A state presented to GP2;
5. altered ledger-before or ledger-after bytes;
6. labelled, reordered, duplicated, altered-parent or altered 32-byte command IDs;
7. non-atomic `Depart`/`ChooseOutcome` or a fifth C4V batch;
8. missing revision-3 restart, corrupt prefix snapshot or trusted stored prefix;
9. missing final restart, corrupt final snapshot/log/receipt or terminal append;
10. foreign GP3 situation, approach, digest, threat or threat terminality;
11. threat selection without GP1 diversion, or diversion without selected threat;
12. incomplete GP0 consequence coverage;
13. fabricated, reordered or extra GP2 receipt, emission or transition IDs;
14. GP2 processing of the C3A-backed state;
15. missing or free-form semantic/accessibility slots;
16. any requirement status other than `unmeasured`;
17. top-level or nested over-limit payloads before semantic equality;
18. unknown, duplicate, reordered, trailing, malformed or noncanonical fields;
19. paths, URIs, executable names or runtime-selection claims;
20. broad-G1 closure, R1 mutation or runtime-promotion authority.

## Exit and rollback

Passing GP4 produces an engine-neutral interaction/state bundle and a reasoned
unmeasured adapter requirement list. After registered verification, the
separate `G1-VERTICAL-CLOSEOUT` receipt may record
`broad_g1=false`, `runtime_selected=false`,
`runtime_containment_pending=true` and evidence-only authority. The GP4 module
is additive; rollback deletes it and its registrations without rewriting GP0-
GP3, C3A, C4V, broad C4, G1 closeout or R1.
