# Explicit Authorization Contract v0.2

Approval, promotion, and supersession are separate, owner-initiated ledger
actions. The desktop UI may invoke them only after the owner enters the exact
displayed confirmation phrase for the candidate and action.

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
