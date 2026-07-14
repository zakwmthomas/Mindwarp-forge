# Source Manifest Contract v0.1

A source manifest is optional metadata for an explicitly supplied transcript.
It records `source_id`, format version, expected chunk count, current chunk
index, and ordering basis. It grants no authority and does not widen accepted
message labels.

The compiler must return a SourceGapReceipt when a chunk index is out of range,
chunks are missing, or order is ambiguous. It may ingest supplied labelled
messages as evidence, but the receipt must remain `incomplete` or `ambiguous`;
no briefing or candidate may imply full conversation coverage.

Required tests: complete single chunk; missing middle chunk; out-of-order
chunks; duplicate chunk; invalid count/index; and imported approval language
remaining non-authoritative in every case.
