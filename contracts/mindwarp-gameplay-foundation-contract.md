# Mind Warp gameplay foundation contract

Status: GP0 player-promise, GP1 fixed-loop, and GP2 typed progression
reference contract, schema version 1.

## Player promise

The exact primary fantasy is `causal_explorer_maker`: understand a strange
local system, make a fitting intervention, and leave consequences the world
remembers. Combat may create room to act but never resolves a session's core
tension.

The 35–60 minute session, 10–15 minute stable-stop cadence, fixed hub or
vessel, and repairable-failure model are explicit reversible proposed
assumptions. They are not production requirements.

## Records and reducer

- `GameplayConceptRecordV1` fixes the promise, exact typed non-goals, and
  reversible assumptions.
- Five `SessionRecordV1` authored fixtures separate observation from inference
  and communicate each risk with equivalent visual and audio or haptic cues.
- Every intervention and retreat is terminal and records typed exact mutations,
  opportunity costs, rememberer/proposition pairs, non-spendable permissions,
  rights, services or knowledge, and one named later decision.
- `SessionState` is accepted only when its full contents replay from its trace.
  `WorldHistoryV1` is ordered, append-only, strict, and retains mutations,
  threat contributions, costs, memories, grants, and decisions.
- Generated text is a projection of typed state and has no separate authority.

## Fixed base loop

`BaseLoopStateV1` runs every fixed session through exactly prepare, depart,
encounter, consequence, return, and remembered response. Preparation selects
intent, fitting-tool loadout, and threat posture but no outcome. The player
selects the terminal outcome during encounter after observation and inference.
Preparation, consequence, return, recoverable failure, and terminal remembered
response expose exact typed stable stops; departure and ordinary encounter do
not.

Plans are session-bound. Encounter failure retains the prepared plan, optional
world context, history, and unselected outcome. Exactly three recoveries are allowed; a fourth failure
attempt rejects before mutation. `BaseLoopLedgerV1` atomically appends history
with a typed completed run ID, rejects duplicate response, and permits later
replay of a session only with a distinct run ID. Receipts map each GP1 run to
its exact history event sequence, session, and outcome after an explicit GP1
event floor. S5 uses the latest preceding S1 event and rejects a latest retreat. Authored fixtures carry an
authority-negative context without C3A identity; validated C3A context uses the
existing exact binding seam.

## C3A boundary

`C3AWorldReferenceV1` contains only schema, reconstruction, input, and packet
identities. `bind_validated_c3a_world` accepts an exact replay-validated
`WorldGenerationInput` and `CausalWorldPacket`; path fields are unrepresentable.
An `ObservedC3AOutput` fact requires that typed reference. All five fixed
fixtures instead use `AuthoredGameplayNonC3B`: their storms, crystals, colonies,
signals, materials, timings, and consequences are authored gameplay premises,
not C3A or C3B scientific proof.

## Typed progression

`ProgressionLedgerV1` consumes only a strict replay-validated authored-fixture
terminal loop state and externally validated prior base-loop history. A private
canonical registry contains exactly one rule for each of the eighteen fixed
session outcomes. Receipts bind domain-separated digests of the source ledger,
terminal state, fixed session, and private registry plus exact history event,
world transitions, emissions, and next decision.

Knowledge, access, relationship events, construction, capabilities, and named
unique assets remain distinct typed records. There are no conversion or reset
rules. Caller failure costs, non-allowlisted free-form tools outside the exact
fixed allowlist, threat-only contributions, and repetition grant nothing
durable. Exact allowlisted prepared tools can grant a scoped capability only
with their matching successful outcome. Services are fulfilled rather than reusable;
only explicitly named rights remain active with obligations. The three S1
capabilities require an exact successful outcome and its exact allowlisted
tool, have horizontal named scope, and expose no magnitude or spend surface.

## Fixed-session invariants

- S1 uses Keeper Mara's signed permission and typed direct, bypass, ration, and
  deterministic retreat consequences; no trust token or disguised currency.
- S2 threat diversion never completes relocation or stabilization. Its whistle
  recognition and named storm/crystal knowledge are non-spendable facts.
- S3 retains separate east and west memories and an exact timed-passage charter;
  force grants one crossing and leaves ownership unresolved.
- S4 models wire scavengers, stranded surveyor Iven, the waiting caravan, and
  the closing signal window. Clearing the work area alone completes nothing.
- S5 admits exactly S1 direct, bypass, and ration, rejects S1 retreat, applies
  predecessor-specific colony state and memory, and appends rather than
  overwrites history.

## Explicit non-goals

No runtime, engine, database, filesystem, network, monetization, publishing,
Greenfield dependency, procedural breadth, C3B substitution, universal
currency, XP, levels, grind, positive conversion cycles, GP3, or GP4 is
authorized by this contract.
