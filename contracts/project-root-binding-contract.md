# Project Root Binding and Rollback Contract v0.1

Forge may not silently bind to its own source repository, a Unity project, or
an arbitrary filesystem location. Binding a real project root requires a
future explicit owner selection and confirmation of its canonical path.

Before binding, Forge must canonicalize the selected existing directory,
reject Forge app-data/backup/staging paths, create a read-only SHA-256
inventory excluding symlinks, display the inventory for confirmation, and
persist the bound-root identity plus inventory snapshot in the ledger.

The first real-root application version must support an atomic new-file write,
pre-write inventory record, `CodeApplied` ledger record, and a candidate/event
specific rollback. Existing targets are rejected. Overwrites, merges, deletes, Unity asset import, editor
automation, package management, builds, and network activity remain out of
scope until separately designed and authorized.

## Current readiness status

Implemented now: safe staging root, hash inventory, promoted-candidate-only
new-file staging apply, exact confirmation, durable application event, rollback
of a newly created file, and non-overwrite protection.

Blocked by owner choice: the actual external project root to bind. No default
has been assumed.

## Approved Forge root

The owner has explicitly approved `C:\Users\zakwm\Desktop\Mindwarp forge`
as the Forge project workspace. This approval applies to Forge only. It does
not authorize any Unity project access, creation, binding, mutation, or
integration. A Unity project will be created separately when the owner starts
that phase.
