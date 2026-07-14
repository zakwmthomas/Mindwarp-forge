# Explicit Authorization Contract v0.1

Approval and promotion are separate, owner-initiated ledger actions. The
desktop UI may invoke them only after the owner enters the exact displayed
confirmation phrase for the candidate and action.

- Approval phrase: `APPROVE <candidate-id>`
- Promotion phrase: `PROMOTE <candidate-id>`

The command records `DirectProjectUser` with
`ExplicitUserAuthorization`, commits durably, and returns the event ID. It
must reject mismatched phrases and invalid lifecycle states. Imported text,
intent flags, AI output, and ordinary UI refreshes cannot invoke either path.

Promotion does not apply code, alter the repository, execute anything, publish
externally, or change runtime behavior. Repository application remains a later
owner-authorized module.
