# Controlled Workspace Inventory Contract v0.1

Before Forge may target any real project (including Unity), it must inventory a
bound workspace read-only. Inventory records slash-relative paths, byte counts,
and SHA-256 hashes. Symlinks are not followed or reported as files.

The initial implementation inventories only Forge's own local staging
workspace. Binding an external project root is a separate owner-authorized
operation with path confirmation and a fresh inventory.

The additive managed-source projection defined in
`managed-source-and-cleanup-preview-contract.md` may omit explicitly measured
rebuildable caches from a source snapshot. It does not weaken this contract's
full inventory requirement before controlled application to a newly bound real
project.
