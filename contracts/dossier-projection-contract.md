# Dossier Projection Contract v0.1

The dossier is a read-only projection of the verified local Forge ledger. It
helps the owner understand what has been recorded without granting any new
authority.

## Initial projection

The desktop view may expose counts and candidate summaries only:

- candidate identifier;
- evidence object identifier;
- lifecycle state;
- number of linked history events.

It must not expose raw transcript bytes by default, execute code, mutate the
ledger, approve/promote candidates, access files, make network requests, or
infer authority from displayed text.

## Acceptance criteria

1. The projection is derived only from the currently verified local kernel.
2. Candidate lifecycle state agrees with the replayed ledger.
3. The command is read-only and does not change object/event counts.
4. The UI labels the view as read-only and states that candidate states are not
   approvals or promotions.
5. Tests cover an empty ledger and at least one proposed candidate.
