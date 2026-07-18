# G1 GP2 progression design

Status: accepted design receipt, implemented by the bounded result recorded in
`G1_GP2_PROGRESSION_RESULT.md`.

## Canonical input and ledger

`apply_progression` consumes a session record, a replay-validated terminal
`BaseLoopStateV1` and a prior
`ProgressionLedgerV1`. The ledger binds the canonical digest of its source
`BaseLoopLedgerV1` plus processed progression receipts. Each new receipt binds
run ID, event sequence, session, outcome, terminal-state digest, rule ID,
emitted record IDs, exact world transitions, and the opened named decision.
The fixed rule registry is private canonical authority and is never supplied
by the caller.

Only explicit per-session and per-outcome rules exist. There is no recursive or
automatic conversion. GP1 failure costs remain attempt-local. Caller-supplied
opportunity-cost text, combat diversion, and arbitrary prepared tool text have
no durable progression authority.

## Five distinct records

- `KnowledgeRecordV1`: observed, inferred, corroborated, or superseded
  propositions. Knowledge is never spent.
- `AccessRecordV1`: permission, right, or fulfilled service with exact issuer,
  scope, state, and obligations.
- `RelationshipEventV1`: append-only exact proposition and commitment; no
  score, spending, generic reputation, or bond.
- `ConstructionRecordV1`: named artifact, function, state, predecessor, and
  exact transition.
- `CapabilityRecordV1`: named horizontal scope with no magnitude. A grant
  requires an exact allowlisted prepared tool and successful allowlisted
  outcome.

`NamedAssetV1` and typed obligation state support these records but are not
currency, inventory value, or a sixth fungible lane. Intangible progress is
nonconsumable. A fulfilled service records fulfillment, not permanent
entitlement. These creative assumptions remain explicitly proposed.

## Fixed S1 capability rules

- `s1.direct` plus `full-flow-kit` may grant `emergency-restoration`.
- `s1.bypass` plus `colony-safe-kit` may grant `bypass-installation`.
- `s1.ration` plus `timed-controller` may grant `synchronized-scheduling`.
- `s1.retreat` grants no capability.

The technique names, service interpretation, nonconsumable intangible state,
and horizontal capability model are reversible proposed assumptions, not
production promotion.

## Flow, exploit, and simulation rules

Every rule names its sources, sinks or liabilities, reset boundary, admitted
conversion if any, anti-farming causal preconditions, world transitions, and
new decision. No same-event or same-subject cycle is allowed. Conversion edges
cannot recurse or form a non-negative round trip.

Deterministic simulations run at least three named strategies:
`steward-builder`, `urgency-discovery`, and `cautious-mapper`. They retain typed
liabilities and must produce a pairwise-incomparable viable set. Recovery emits
no durable progress. The proof rejects a universal resource, dominant path,
hoarding exploit, reset duplication, positive conversion cycle, and repetition
that substitutes for a new authored decision.

## Hostile boundary

The proof must reject caller failure cost or free-form tool grants, money, XP,
levels, generic reputation, bond, generic access, generic crafting value,
knowledge without `ObserveCause`, duplicate assets, S5 without its exact
predecessor, collapsed S1 direct/bypass/ration meanings, and any runtime,
persistence, filesystem, network, Greenfield, C3B, GP3, or GP4 dependency.
