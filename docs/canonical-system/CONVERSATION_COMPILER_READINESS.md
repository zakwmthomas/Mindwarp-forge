# Conversation Compiler Long-Corpus and Format-Drift: Readiness Package

**Status:** R0.4 bounded continuity profile verified. This document does not expand the
manual-import grammar, enable clipboard watching, add file import, or grant
captured text any authority.

## Existing proven boundary

The current explicit-paste importer is intentionally narrow: labelled `User:`
and `Assistant:` text, a supplied source identifier, evidence/candidate
creation, and intent-only treatment of corrections and approval language. Its
adversarial corpus already protects empty/unlabelled input, multiline order,
approval forgery, duplicate import, reserved actor labels, and large pastes.

The remaining gap is not merely more samples. It is a versioned long-corpus
contract that can show whether capture remains lossless, ordered, idempotent,
gap-aware, and authority-safe as input grows or source formats drift.

## Boundary to establish

| Record | Required role | Must not imply |
|---|---|---|
| `SourceManifest` | Source ID, declared format/version, acquisition method, byte hash, ordering basis, completeness state | That a source is complete, trusted, or approved |
| `SourceGapReceipt` | Exact missing/ambiguous span, detection method, effect on downstream claims | Permission to invent/reconstruct missing conversation text |
| `ParseReceipt` | Grammar version, counts, rejected spans, correction/approval intent counts, idempotency result | Approval, promotion, code execution, or filesystem authority |
| `CorpusFixture` | Retained source bytes or synthetic fixture plus expected evidence/receipt assertions | Raw chat content available to every inspector by default |
| `FormatClassification` | Explicit supported/unsupported/ambiguous result | Silent fallback to a nearby grammar |

## Long-corpus fixture matrix

| Fixture | Required observation |
|---|---|
| Large valid labelled corpus | Stable ordering, bounded memory/cost receipt, exact message/evidence counts |
| Long multiline corpus | No boundary drift or content loss across repeated parsing |
| Duplicate source with partial retry | Idempotent final evidence/event state with clear retry receipt |
| Truncated tail/head/middle | SourceGapReceipt names the affected span; no fabricated continuity |
| Reordered chunks | Ordering ambiguity is rejected/flagged, never silently normalized |
| Mixed labels/unknown actors | Unsupported or reserved labels fail before evidence creation where required |
| Format near-miss | Explicit unsupported/ambiguous classification, not best-effort import |
| Correction/approval language at scale | Intent reporting remains non-authoritative regardless of repetition/context |
| Changed grammar version | Old fixture outcome is replayed or migration/rejection is explicit |
| Malformed/invalid encoding | Exact byte-preservation or explicit rejection; no silent transform |

## Non-negotiable invariants

- Captured bytes, source identity, actor claim, ordering basis, and compiler
  version remain traceable through every receipt.
- Missing source coverage is an explicit result. It cannot be hidden by a
  concise briefing or filled in by a model summary.
- Supported grammar is allowlisted. Drift or ambiguity fails closed into a
  source-gap/unsupported receipt.
- Candidate/intent extraction remains separate from approval, promotion,
  application, external authority, and code execution.
- Long-corpus handling has labelled cost/memory measurements but does not claim
  universal capacity without a reproducible fixture and environment record.

## Readiness gaps deliberately left open

The package does not choose a chat-export format family, automatic source
discovery, external connector, retention/redaction policy, maximum corpus size,
compression policy, or interpretation migration policy. Those choices affect
privacy, source provenance, and the product’s continuity guarantees, so they
require a dedicated design/readiness package before code changes.

## Entry criteria for a future implementation package

- A synthetic, non-sensitive long-corpus fixture is versioned with expected
  hashes, counts, gap receipts, and authority-negative assertions.
- Grammar-version and migration/rejection behavior is specified.
- Corpus runner reports time/memory and does not require network/file/clipboard
  capability beyond the explicitly authorized source boundary.
- Existing adversarial cases remain passing; no grammar expansion is accepted
  merely because it parses more text.

## R0.4 verification result

The bounded v1 profile now closes the risk-critical continuity gap without
selecting an external export format or widening compiler authority:

- `synthetic-representative-corpus-v1` deterministically generates 1,024
  non-sensitive messages across eight chunks; aggregate fixture SHA-256 is
  `9e5c620f6d18b000b0c3a328fa20c8f6dfd497e9600b8ba42cc9849e53be5b3d`.
- Out-of-order ingestion remains incomplete until all indices exist. Exact raw
  bytes, 1,024 child-evidence links, ordered manifest history, and 512 proposed
  Assistant candidates survive a fresh SQLite reopen.
- The reopened journal is explicitly dropped before the disposable database
  directory is removed, which passes on Windows and guards the prior WinError
  32 failure mode.
- The retained legacy-database fixture adds the envelope schema and replays
  exact links without losing the old `source_chunks` row.
- Platform-independent shared portable validation rejects `/absolute`, `C:/absolute`,
  `C:\\absolute`, UNC, and traversal forms at admission and staging.
- Approval and correction language are counted only as evidence/intent; no
  candidate becomes approved or promoted.

This proves the declared synthetic v1 boundary, not universal corpus capacity,
arbitrary export compatibility, privacy policy, or automatic source discovery.
