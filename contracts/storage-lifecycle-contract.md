# Storage lifecycle contract v1

Status: owner-authorized additive Forge continuity component.

## Protected truth

Raw conversation evidence, content-addressed objects, append-only events,
authority receipts and canonical repository records are protected. They are
never automatically deleted, summarized in place or replaced by an archive.

## Verified cold archive

Cold storage uses streaming gzip level 6 over a verified SQLite backup. Its
receipt records source and archive paths and hashes, byte counts, replay
object/event/candidate counts, codec and `raw_source_retained=true`. The
archive is decompressed to a disposable verification copy and replay-verified
before success. Creation refuses to overwrite an existing archive and does not
authorize removal of its source. The destination is local by default and
caller-configurable.

## Compact inventory

The managed workspace inventory hashes source-of-truth files and records
`.git`, `.local`, `target`, `node_modules` and `artifacts` as excluded roots.
It does not hash every derived cache file. Its sorted file set produces one
deterministic root digest. The strict full inventory remains available only
for bounded staging or an explicit forensic request.

## Approval-gated reclamation

Cache reclamation is two-step. Preview may measure rebuildable `target` and
regenerable `.local/forge-bootstrap` paths and report bytes, file counts and
reasons. It always returns `approval_required=true` and `executed=false`.
No deletion API exists in the Kernel; execution requires separate explicit
owner approval and a fresh protected-path and size check.
