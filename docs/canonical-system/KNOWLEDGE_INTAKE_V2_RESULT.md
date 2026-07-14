# Knowledge Intake v2 Result

## Purpose

Forge's first continuity stage must preserve the conversation and make material
content findable by its role, so a future worker does not depend on remembered
chat context. This is an evidence-routing layer, not an automatic truth or
planning authority.

## Verified boundary

1. The local Codex capture stores visible user and assistant text as immutable,
   ordered evidence with source cursors and gap detection.
2. Deterministic classification emits multiple facets when one message mixes
   concerns. The v2 categories are idea, plan, decision, task, research,
   correction, philosophy, requirement, constraint, preference, risk,
   question, observation, and context.
3. Every non-noise message receives at least one typed facet. Exact lightweight
   acknowledgements and operational bootstrap receipts do not flood the
   knowledge library.
4. Each facet retains the source actor, evidence ID, content fingerprint,
   classifier version and confidence. Its authority is always `evidence_only`.
5. The SQLite ledger retains old classifier records. The read-only current
   projection exposes only the newest classifier version present, preventing
   legacy duplicates and retired noise decisions from obscuring current results.
6. Forge emits `KNOWLEDGE_INDEX.md` and `KNOWLEDGE_CATALOG.json` into the local
   bootstrap pack. `tools/find-knowledge.ps1` searches by text, category and
   actor without loading whole transcripts.
7. The Forge project library exposes the expanded categories as filters. Search
   and classification remain read-only.

## Canonical routing

Classification answers “where should a worker look?” It never silently edits a
canonical plan or policy. Material triage places verified content as follows:

| Typed facet | Canonical destination when validated |
|---|---|
| Plan or task | `MASTER_PROGRAM.json`, the relevant design gate, or the active checkpoint |
| Philosophy | `MASTER_PLAN_V2.md`, Working Covenant, or an approved policy |
| Requirement or constraint | Owning system contract/design requirement and its tests |
| Preference | Creative-direction or owner-decision record; never population truth |
| Decision or correction | Owning decision/result record with the superseded claim retained |
| Risk | Active checkpoint, gap register, threat/failure analysis, or test obligation |
| Research or observation | Source-linked canonical system evidence with limits and contradictions |
| Question or context | Typed search route until resolved, rejected, or promoted into an owning record |

The mandatory closeout rule still requires material results to leave chat and
enter their owning canonical record. Typed intake reduces loss and discovery
cost; it does not replace validation or canonicalization.

## Cheap adversarial fixtures

- Ordinary conversational language about efficiency yields philosophy,
  requirement and risk rather than remaining invisible.
- Mature stylization plus phone operation yields preference, constraint and
  requirement.
- A correction about no-old-age death plus a question yields correction,
  constraint and question.
- A neutral meaningful statement receives context fallback.
- Acknowledgements and bootstrap receipts are suppressed.
- Repeated classification is deterministic and evidence-only.
- Persistent intake remains idempotent and cannot grant authority.

## Live backfill evidence

The exact repo-built Forge desktop was rebuilt and restarted against the local
ledger. Capture returned current with nine sessions and 5,086 events. The
append-only database retains 257 v1 classification rows and adds 3,226 v2 rows;
the current generated catalogue exposes only the 3,226 v2 records. It contains
2,500 assistant and 726 captured-user facets and no legacy-unknown actors.

Current category counts are: 799 constraint, 666 context, 595 requirement, 285
observation, 283 risk, 192 question, 171 task, 94 plan, 77 philosophy, 23
decision, 21 preference, 9 correction, 9 research and 2 idea. These are routing
counts, not quality or importance scores.

Bounded live searches recover:

- the owner's efficiency philosophy and weekly-allowance concern;
- adult-lock aging constraints and the phone-legibility distinction;
- phone/growth requirements and stylization preferences; and
- the request to categorize and store conversation content as philosophy,
  requirement, constraint, risk, observation and task.

The same manifest reports 2,658 immutable objects, 5,086 events and 1,972
candidates. Classification writes knowledge rows and disposable projections;
it does not add approval or promotion events.

## Limitations

- Phrase rules are routing aids and can over-classify or miss nuance. Canonical
  validation remains mandatory.
- v2 classifies whole-message facets rather than extracting exact sentence
  spans. The original bytes are retained for precise review.
- Related, duplicate, contradiction and supersession edges still require a
  later bounded semantic-linking package; the classifier does not invent them.
- Private retention/redaction policy and external connectors remain outside
  this package.
