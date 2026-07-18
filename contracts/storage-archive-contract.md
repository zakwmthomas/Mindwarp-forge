# Lossless storage archive contract v1

Status: owner-authorized additive Forge continuity component.

## Archive format

The live SQLite journal remains uncompressed. Cold archives use streaming
gzip/Deflate level 6 through the repository's existing pure-Rust `flate2`
stack. `StorageArchiveReceipt` binds schema, archive and codec versions,
archive type, source revision, compressed and uncompressed byte counts and
SHA-256 hashes, object/event/candidate/knowledge counts, canonical paths,
completion state and a fresh decompression/replay result.

## Creation and verification

SQLite archives begin only from an existing online-backup receipt that already
passed reopen/replay. Creation writes a new temporary file, streams the source,
flushes and closes it, verifies both byte counts and hashes, then moves it
atomically to a previously nonexistent destination.

Verification streams into a fresh temporary path, enforces declared compressed
and uncompressed ceilings before and during expansion, rejects trailing or
concatenated members, matches both hashes, reopens the decompressed journal and
reproduces every recorded count. Corrupt, truncated, swapped, oversized,
path-poisoned or replay-mismatched archives fail before replacement or recovery.

An owner-selected archive root is a configuration binding, not an authority
grant. An unavailable external location returns `offline`; it never weakens
local evidence verification or redirects to an unapproved path.

## Retention and rollback

Archive creation never deletes or replaces its source. Original backups and
classifier rows remain until a separately hashed cleanup preview receives
explicit owner approval under a future execution package. V1 contains no
archive-deletion or restore-over-live operation. Rollback ignores the archive
projection and retains both originals and verified archive bytes.

