# Controlled Code Application Contract v0.1

The first application workflow is intentionally a **staging workspace** under
Forge's local app-data directory, not the Forge source repository and not an
arbitrary user-selected folder.

It accepts only a promoted code candidate after exact confirmation:

`APPLY <candidate-id>`

It creates a new file only. Existing targets, traversal/absolute paths, and
symlinked or junction ancestors are rejected. The parent is canonicalized back
under the selected root immediately before write. The code is written
atomically, then a durable `CodeApplied` ledger event is recorded. Failures
after temporary write or rename remove their artifacts before returning. No
admitted code is executed.

Overwriting files, diff/merge, package management, and external publishing are
outside this primitive. Candidate-specific rollback removes a newly created
file and records `CodeRolledBack`. Hostile environment, network, and process
strings are stored verbatim but never interpreted or executed by this module.
The separately confirmed Forge-workspace verification runner is not part of
this no-process staging primitive.
