# G1 Federated Live Writer Lease Integration Result

Status: source, disposable, complete integration and additive live registration
verified.

## Result

Forge now has a bounded project-wide writer claim built on the existing
revisioned `WorkstreamRecord` and `SessionRouteReceipt` owners:

- `CODEX_THREAD_ID` is the exact writer holder identity;
- every session routes to the registered Forge project and
  `forge-live-mainline` workstream before claiming;
- the latest workstream revision stores one bounded lease and one canonical
  checkpoint SHA-256 binding;
- another holder, an unrouted session, a wrong checkpoint, an expired claim,
  or a lease longer than 1,800 seconds fails closed;
- release expires the claim so another routed holder can acquire a successor;
- the PowerShell boundary blocks live route, claim, and release mutations
  unless `-AllowLiveDatabaseMutation` is supplied explicitly; and
- assertion is read-only and cannot create a route or lease.

The disposable end-to-end fixture passes routed claim, idempotent routing,
sequential rejection plus a simultaneous two-process election with exactly one
winner, checkpoint drift, release/takeover, missing-route rejection and TTL
bounds. The complete Forge Kernel all-target suite passes 103 tests.

The complete repository gate passed with exit 0 in 282.7 seconds on 2026-07-18.
Before live registration, SQLite's online backup API produced
`pre-live-writer-lease-20260718-163654.sqlite3`; its `PRAGMA quick_check` was
`ok` and its SHA-256 was
`1f10eae7cd97ba009488148a7ce1c110da73433cfd938c4f6bc05416d51d7908`.
The live database then received only the additive `forge-live-mainline`
workstream and this exact Codex session route. Revision 2 bound the sole writer
to checkpoint SHA-256
`9d5e6cd623b04de0f9bb6bbe01efa809d6483b79b2898148743afb5ae678110e`;
live assertion passed before the final receipt update.

## Authority retained

The separately gated live action added one workstream and one session route,
then exercised a bounded lease. It did not rewrite knowledge, approve a
package, select C3 work, promote code, or grant owner/runtime authority.

## Rollback

Remove the additive CLI verbs, wrapper, focused test and helper functions.
Existing federated records and the live SQLite database remain intact; no
evidence, checkpoint, route history, or project record is deleted.
