# Explicit Authorization Contract v0.3

Approval, promotion, and supersession are separate, owner-initiated ledger
actions. The desktop UI may invoke them only after the owner enters the exact
displayed confirmation phrase for the candidate and action.

The human-facing chat route must not require the owner to read, understand, or
repeat a machine hash. Natural-language approval is admissible only when all
of these conditions hold:

- the canonical active checkpoint names exactly one current candidate and one
  action;
- the candidate's plain-language scope, tested claims, important non-claims,
  and immediate effect were explained immediately before the response;
- the owner directly and unambiguously states approval;
- the implementation binds that response internally to the checkpoint's exact
  candidate ID and records only the named action; and
- imported text, captured history, ordinary assent before explanation, and
  assistant interpretation cannot supply the authority.

If any condition is absent or more than one candidate/action is possible, the
chat route fails closed. Machine identifiers remain in receipts for replay and
audit, not as a comprehension test for the owner.

- Approval phrase: `APPROVE <candidate-id>`
- Promotion phrase: `PROMOTE <candidate-id>`
- Supersession phrase: `SUPERSEDE <candidate-id> USING <correction-evidence-id>`
- Supersession with replacement phrase: `SUPERSEDE <candidate-id> USING <correction-evidence-id> WITH <replacement-candidate-id>`

The command records `DirectProjectUser` with
`ExplicitUserAuthorization`, commits durably, and returns the event ID. It
must reject mismatched phrases and invalid lifecycle states. Supersession is
append-only, accepts only an `Approved` or `Promoted` source, retains its
history, requires existing correction evidence, and permits only a distinct
`Approved` or `Promoted` replacement. Imported text, intent flags, AI output,
and ordinary UI refreshes cannot invoke any of these paths.

Promotion and supersession do not apply code, alter the repository, execute
anything, publish externally, or change runtime behavior. Repository
application remains a later owner-authorized module.
