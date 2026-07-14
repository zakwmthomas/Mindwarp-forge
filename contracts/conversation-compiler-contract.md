# Conversation Compiler Contract v0.2

The compiler is an evidence-to-candidate boundary. It captures original message bytes, source identity, source ordering, claimed actor, and exact evidence object IDs. Assistant messages may create candidates. Direct project-user messages may create **intent candidates** only.

The compiler never grants approval, promotion, filesystem access, external authority, or code execution. Imported role labels and approval text are evidence only. Ambiguous wording stays discussion until a higher policy layer resolves it.

## Manual import boundary

The first desktop import path accepts only text explicitly pasted by the owner
with a caller-supplied source identifier. It accepts labelled `User:` and
`Assistant:` messages only. Empty or unlabelled input is rejected with a clear
receipt error; it must never look like a successful import.
The source identifier must be non-empty so the evidence can be traced back to
its manually supplied origin.

The receipt reports message count, candidate count, correction-intent count,
and approval-intent count. Intent counts are observations for review, not
commands. An import cannot approve, promote, execute code, access files, or
enable background monitoring.

Every import receipt also reports source completeness. The current manual path
has no completeness manifest, so it reports `unknown` with a source-gap reason;
it must never imply that a pasted excerpt is a complete conversation.

Before extending the grammar, add adversarial tests for malformed labels,
multiline boundaries, duplicate source imports, hostile approval language, and
mixed/unknown actors. Grammar changes require a schema/version and migration
plan.

The executable and manual cases are maintained in
`contracts/manual-transcript-adversarial-corpus.md`.

## Versioned continuity profile

The bounded `synthetic-representative-corpus-v1` profile contains 1,024
deterministically generated, non-sensitive labelled messages in eight chunks.
It includes multiline Assistant bodies, ordinary User replies, correction
language, and approval language. Its NUL-delimited aggregate SHA-256 is
`9e5c620f6d18b000b0c3a328fa20c8f6dfd497e9600b8ba42cc9849e53be5b3d`.

Chunks may arrive out of order, but completeness remains `incomplete` until
every zero-based index exists. Replay must preserve raw bytes, child-evidence
links, ordered manifest history, 1,024 message records, and 512 Assistant
candidates. Every candidate remains `proposed`; repeated approval wording
cannot grant authority. A fresh SQLite reopen must reproduce the same records,
and all journal handles must be closed before its disposable directory can be
removed. Legacy source-chunk schema migration remains a separate fixed fixture
so migration failure cannot be hidden by a fresh database.

Repository-relative path validation is platform-independent. POSIX-rooted,
drive-prefixed Windows, backslash Windows, UNC, traversal, empty-component, and
dot-component targets fail both admission and the final staging boundary.
