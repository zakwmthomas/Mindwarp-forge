# Chunked Source Assembly Contract v0.1

A chunked source is identified by source ID plus manifest version. Each chunk
declares a zero-based index and expected count. Chunks are immutable evidence;
assembly is a read-only projection and never changes authority.

The projection reports `complete` only when every index from zero through
count-minus-one is present exactly once and uses the declared ordering. Duplicate
equal chunks are idempotent; conflicting duplicates, missing chunks, invalid
ranges, and reordering ambiguity produce explicit gap receipts. Incomplete
sources may preserve supplied messages as evidence, but cannot claim complete
conversation coverage.

Required tests: durable reopen/replay, complete ordered assembly, missing
middle, duplicate equal, conflicting duplicate, out-of-order arrival, invalid
range, and approval language remaining non-authoritative in each case.
