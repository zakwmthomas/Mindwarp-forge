# Managed source inventory and cleanup preview contract v1

Status: owner-authorized additive Forge continuity component.

## Managed source inventory

For a Git repository, Forge inventories tracked plus untracked non-ignored
regular files. It records commit identity, dirty paths and hashes separately.
The sorted inventory has a deterministic root digest and an explicit exclusion
receipt.

`.git`, `.local`, `target`, `node_modules`, generated artifacts and declared
caches are excluded from the source snapshot but are named and measured in the
receipt. Symlinks, junctions, reparse points, traversal, absolute child paths
and resolved escapes fail closed. A non-Git root requires an explicit
include/exclude manifest and a separate owner-confirmed binding.

## Storage report and cleanup plan

`StorageReport` records category sizes, growth, reclaimable bytes, retained
proof receipts, active locks and exact cache roots. `CacheCleanupPlan` is a
preview-only immutable proposal containing canonical candidate paths, observed
sizes and fingerprints, exclusion reasons, active-use evidence, source revision
and a domain-separated plan hash.

Plans may be generated on request or when rebuildable caches exceed the
configurable 20 GiB warning threshold. Plan generation is read-only. It rejects
or excludes evidence, app-data, backup, source, `.git`, active-cache,
symlink/reparse, changed, locked and out-of-approved-cache-root paths.

## No execution authority

This package has no cleanup executor and performs no deletion, move or
replacement. Future execution requires explicit owner approval bound to the
exact plan hash, fresh path re-resolution and a separately authorized package.
Approval of this contract or implementation does not approve any plan.

Generated projections may be regenerated or evicted only through existing
bounded projection rules. Original backups and build caches remain protected
until their own plan is explicitly approved.

