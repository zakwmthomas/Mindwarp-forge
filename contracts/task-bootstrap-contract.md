# Task Bootstrap Contract v1

Every Mind Warp Forge task starts in the Forge repository and follows the
root `AGENTS.md` procedure. The tracked `context/` documents carry stable
workflow and concise current state; the ignored `.local/forge-bootstrap/`
directory carries regenerated local capture projections and raw evidence.

The bootstrap verifier checks pack schema, capture health, freshness, and
transcript hashes before mutable work. A failed or stale bootstrap blocks
project mutation but never deletes evidence or attempts a weaker capture path.

`CURRENT_STATE.md` is a generated, source-linked summary of the canonical
`WORKER_BATCH_STATE.json` checkpoint. It must never be hand-edited. It is useful
navigation, never authority. The SQLite journal and immutable objects retain
evidence history; the master program and active checkpoint retain execution
and resume authority.
