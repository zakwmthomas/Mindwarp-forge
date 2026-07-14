# Conversation Compiler Contract v0.1

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
