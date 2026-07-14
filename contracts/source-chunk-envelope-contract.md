# Source Chunk Envelope Contract v0.1

## Purpose

Bind one explicitly supplied raw source chunk to its parsed message evidence
without claiming that any individual message represents the whole chunk.
This contract is evidence-only and grants no candidate approval, promotion, or
application authority.

## Required immutable fields

`source_id`, `manifest_version`, `ordering_basis`, `expected_chunks`,
`chunk_index`, `raw_bytes_object_id`, and `raw_bytes_sha256`.

The raw byte object is retained before parsing. Parsed labelled-message evidence
is linked as children of the envelope; it cannot replace the raw-byte object.
`manifest_version` is a positive source-manifest revision, not a hard-coded
envelope schema number. Different manifest versions for one source are stored
and projected independently.

## Assembly rule

An assembled receipt is `complete` only when envelopes share identical source
ID, manifest version, ordering basis, and expected count, and cover every
zero-based index exactly once. It is a read-only projection. Any mismatch,
missing index, conflicting hash, invalid range, or unknown ordering basis is
`incomplete` or `ambiguous`, never complete.

## Promotion boundary

Completeness means only that declared source coverage is present. It never
changes candidate state or allows approval/promotion language inside the raw
bytes or parsed messages to act as authority.

## Required proof fixtures

1. complete two-chunk source with retained raw-byte hashes;
2. missing middle, invalid range, equal duplicate, and conflicting duplicate;
3. out-of-order arrival that projects by declared ordering basis;
4. source-ID collision with different manifest version or ordering basis;
5. altered raw bytes at the same index;
6. reopen/replay with exact raw-byte and child-evidence linkage; and
7. approval wording in a complete source remaining non-authoritative.

## Measurement and stop rule

Baseline: existing chunk records cannot prove chunk-to-message provenance.
Expected gain: a receipt can be traced to exact raw bytes and parsed children.
Cost: one envelope object and linkage per supplied chunk. Regression guards:
no authority expansion, deterministic replay, and bounded storage growth.
Stop and refocus if the envelope cannot preserve exact raw bytes, requires a
global synchronous service, or fails to improve provenance over the existing
receipt.

## Verified implementation

`PersistentForge::ingest_labeled_transcript_chunk` stores a versioned envelope
in the additive `source_chunk_envelopes` table. Compiler correlation IDs include
manifest version and chunk index, preventing cross-chunk message collisions.
The envelope retains the raw content-addressed object and exact SHA-256 plus
every parsed child evidence ID. Legacy `source_chunks` records remain readable
and are never silently upgraded into stronger provenance claims.
