# Forge Context Store

This directory is the human-readable continuity layer for Mind Warp Forge.
It complements, but does not replace, the immutable object/event ledger.

- `capsules/` contains time-bounded recovery summaries created at a context
  threshold or package close.
- Recovered handover archives remain untouched in
  `forge documents from gpt handover/`; their hashes are recorded in
  `evidence/handover-manifest.json`.

Every capsule must state its source coverage, verified facts, open risks,
authority state, and exact next action. It must never present a summary as
stronger evidence than the source material it cites.
