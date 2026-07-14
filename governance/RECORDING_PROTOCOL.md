# Recording Protocol

This protocol makes work findable for the owner, a future Codex task, and the
Forge. It applies to every material research, design, implementation, test, or
decision-preparation action.

## Record the work where it belongs

| Work type | Durable record | Minimum contents |
|---|---|---|
| Active checkpoint and bounded activity | `context/active/WORKER_BATCH_STATE.json` | Objective, route, state/substage, risk, evidence, verification, authority lane, exact next action |
| Human briefing/handoff | Generated `CURRENT_STATE.md` and `BRIEFING.md` | Never hand-authored; deterministic projections of the active checkpoint |
| Canonical system fact | `docs/canonical-system/` | System/contract, provenance, proof obligation, gap, or promoted result |
| Research claim | Relevant canonical document plus source record | Question, source identity, date/access state, claim, limitation/contradiction, reuse or decision implication |
| Test or verification | Relevant contract/test plus receipt or linked output | Fixture/input ID, version, result, cost if relevant, failure classification, retained artifact location |
| Owner-level choice needed | Owner brief or unresolved-gap record | Options, consequence, evidence, reversible default, and why automation stopped |
| Conversation | Local capture and context capsule | Source identity, ordering, gap status; never treated as authority by itself |

## Mandatory close-out sequence

Before a worker moves to another material task, it must:

1. Put the durable result in the appropriate record above; do not leave it only
   in chat text or terminal output.
2. Link source evidence and test evidence from that record, or state exactly
   why no reliable source/test applies.
3. Update the canonical active checkpoint once, then run
   `tools\refresh-active-context.ps1`; never repeat the same state manually in
   multiple active files.
4. Verify the generated projections match the checkpoint.
5. Run the relevant verification gate and record a failed gate as a failure,
   not a silent retry or success claim.

## Anti-loss rules

- A chat update is a progress notification, not the durable system of record.
- A downloaded document is source evidence until a claim, limitation, and
  provenance link are recorded.
- A passing test without fixture/version/result evidence is incomplete.
- Do not duplicate an entire transcript into every document; use the evidence
  search route and cite the smallest relevant source span.
- Do not create a new status, plan, handoff, recap, or decision file when an
  existing canonical record can hold the fact. A new durable file requires a
  distinct contract, evidence class, or independently reusable result.
- Generated projections are disposable and rebuildable. They are never a
  second source of truth and must carry a generated marker.
- Never let recording mutate authority, approve candidates, select an engine,
  or promote code.
