# A1 Compiler-to-Chunk Binding Review

## Question

Can the current compiler import be bound to `source_chunks` without creating a
false claim that one message-level evidence object represents an entire source
chunk?

## Finding

No. The compiler currently creates evidence per labelled message, while the
chunk table stores one `evidence_id` per declared chunk. Binding either the
first or last message evidence ID would misrepresent the chunk and weaken
provenance. The current chunk contract also names manifest version and ordering
basis, but the persisted `SourceChunk` does not yet retain either field.

## Rejected shortcuts

- Store a message evidence ID as the chunk evidence ID: loses the remaining
  messages and falsely implies full chunk coverage.
- Concatenate imported messages and use a derived identifier without retaining
  the exact supplied bytes: loses a fixity target and format provenance.
- Mark a source complete from compiler import count: import count is not a
  manifest or ordering proof.

## Safe next contract

Introduce a versioned `SourceChunkEnvelope` before binding:

1. retain immutable raw chunk bytes and their content hash as one evidence
   object before parsing;
2. retain `source_id`, manifest version, ordering basis, expected count, and
   index alongside that evidence;
3. parse labelled messages as child evidence linked to the envelope;
4. assemble coverage only from envelopes with identical source ID, manifest
   version, ordering basis, and expected count; and
5. keep assembly read-only: a complete receipt changes neither candidate state
   nor approval/promotion authority.

## Required adversarial tests before implementation

- same source ID but different manifest version is quarantined, not assembled;
- identical index with a different raw-byte hash is a conflict;
- reordered raw bytes with the same labels cannot impersonate an equal chunk;
- a complete envelope set containing approval language remains non-authoritative;
- reopen/replay retains envelope hash, version, ordering basis, and receipt.

## Decision

Do not bind the existing compiler directly to `source_chunks`. Create and
review the envelope contract first. This is a substantive provenance gap, not
an implementation detail; A1 remains partial.
