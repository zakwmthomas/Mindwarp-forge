# Knowledge Intake v3 Atomic Multi-Reference Result

## Owner problem

The owner observed that Forge's searchable categories were inaccurate, with
records appearing under philosophy that were not philosophies, and asked for
cross-area material to be referenced instead of copied into several locations.

## Reproduced cause

Classifier v2 matched broad words such as `philosophy` and `philosophies`, then
emitted one complete-message `KnowledgeRecord` per matched category. A live
search consequently classified the owner's complaint about misclassified
philosophies as a philosophy. Multiple categories were physical duplicate
projections rather than references to one bounded statement.

## v3 repair

1. Intake splits immutable message evidence into bounded sentence or paragraph
   spans while retaining exact byte offsets and the original evidence ID.
2. Each span becomes at most one current knowledge record.
3. `facet_types` contains every applicable statement role. Category searches
   resolve these references instead of searching duplicate records.
4. `system_refs` independently links the record to zero or more stable Project
   Atlas system IDs. Role and project area are no longer conflated.
5. Philosophy detection is conservative. Merely mentioning a category name no
   longer assigns that category.
6. Older v1/v2 rows remain append-only in SQLite. The current projection uses
   only classifier v3 and can be rebuilt from immutable evidence.
7. The generated catalogue, command-line finder and Forge library understand
   multi-role records; the finder also accepts an Atlas-system filter.

## Failure engineering

- Decimal points do not create false span boundaries.
- A multi-role span is asserted to produce one record, not one record per role.
- A single record can reference several Atlas systems.
- Unknown or ambiguous meaning falls back to context instead of forced meaning.
- Authority remains `evidence_only`; classification cannot approve, promote or
  rewrite a canonical contract, plan, policy or checkpoint.
- Append-only legacy retention and current-version projection are regression
  tested.
- Version backfill is one SQLite transaction. Invalid input rolls the whole
  batch back, and an already-current classifier skips replay on later starts.
- The 28.76 MiB generated catalogue is not rewritten by the two-second scanner
  when no new evidence or classifier revision exists.

## Verification status

- Forge Kernel: 88 tests pass, including category-mention, atomic-span,
  multi-system, decimal-boundary, atomic-backfill and legacy-projection fixtures.
- Forge desktop: 41 tests pass and the rebuilt executable starts with current
  capture.
- Forge desktop UI TypeScript and production build pass.
- Live v3 backfill projects 24,325 bounded records from the retained evidence.
  Philosophy results fell from 87 whole-message v2 rows to 9 v3 records overall
  and only 2 owner-authored records. The owner's exact complaint appears once as
  `observation`, not philosophy; its adjacent storage requirement appears once
  as `requirement` with a `forge-kernel` reference.
- An idle six-second scanner receipt left the 28.76 MiB catalogue modification
  time unchanged while Forge remained responsive.

## Limits

Deterministic rules remain search-routing aids, not semantic truth. Spoken
sentence boundaries can be imperfect, new project areas require an Atlas
vocabulary update, and contradiction/supersession links still require validated
triage. Exact source evidence remains the authority-negative fallback.
