# F4 Exit-Criteria Audit

**Result:** Every atomic F4 dependency has retained implementation evidence and
passes its stated proof gate. Atlas status remains `active` because approved
decision D3 reserves milestone changes to the owner. F5 remains `gated`.

| Item | Retained proof | Result |
|---|---|---|
| F4-MODULARITY | Declared graph, Cargo comparison, forbidden imports, cycles, isolated failures | Complete |
| W1 | Fresh feedback, selector, batch, progress, escalation, authority fixtures | Complete |
| W2 | Classification, routing, deduplication, acknowledgement, retry, rate-limit fixtures | Complete |
| A1 | Raw-byte source envelopes, ordering, child links, replay, conflict/authority fixtures | Complete |
| A2 | Append-only manifest/gap history, repair, idempotency, replay | Complete |
| A3 | Hostile path/symlink/environment/network/process/crash/rollback fixtures | Complete |
| A4 | Backup fixity, corruption, count, reopen/replay recovery fixtures | Complete |
| B1 | Research provenance, claims, contradiction, cache, authority fixtures | Complete |
| B2 | Lifecycle, gate, blocker, rollback, stale/forged authority fixtures | Complete |
| B3 | Read-only inspector, failure/gap/version visibility, mutation-negative fixtures | Complete |
| B4 | Append-only BatchEvent replay, privacy/cardinality, projection, Goodhart fixtures | Complete |
| B5 | Local experiments, transfer isolation, aggregate masking, outage, rollback fixtures | Complete |

## Mechanical gate

`tools/verify-f4-closeout.ps1` checks all twelve dependency statuses and proof
sources, mandatory closeout artifacts, Atlas D3/F4/F5 state, and the F5 owner
gate. It is part of `tools/verify.ps1`.

The full Forge gate passes with canonical/Atlas/governance/worker/modularity
checks, UI and Rust builds, 15 desktop tests, 62 kernel tests, and whitespace
validation.

## Owner boundary

Autonomous F4 work is complete. Beginning F5 requires the owner to approve the
milestone transition and choose the first ProofReceipt storage design lane:

- **Kernel object:** strongest replay/provenance integration; expands the
  protected boundary and requires kernel-contract review.
- **Versioned projection:** smaller protected core; requires precise
  linkage/recovery rules to prevent dangling evidence.

Neither option is selected by this audit. No F5 package, engine, runtime,
storage mutation, approval, or promotion is activated.
