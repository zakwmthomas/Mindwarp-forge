# Backup and Recovery Contract v0.1

Forge backups are local SQLite copies made through SQLite's online backup API.
They are not sync, publishing, cloud upload, or a substitute for an external
owner-controlled backup strategy.

Each backup operation must:

1. commit pending immutable records first;
2. create a new destination file and never overwrite an existing backup;
3. calculate a SHA-256 fixity hash and byte count;
4. reopen and replay the copied journal before returning success;
5. return a receipt with path, fixity, and replayed object/event/candidate
   counts.

A retained receipt can be reverified without modifying the live Forge. It must
first match the exact byte count and SHA-256, then reopen and replay to the
recorded object/event/candidate counts. Corrupt, truncated, wrong-hash, or
wrong-count artifacts fail closed and never replace the live journal.

A backup failure must return an error rather than claim recovery is possible.
The module has no network, deletion, approval, promotion, code-execution, or
external-publishing authority.

Verified lossless cold copies may additionally be created under
`storage-archive-contract.md`. Archive creation never replaces this online
backup receipt, deletes the original backup or restores over the live journal.
