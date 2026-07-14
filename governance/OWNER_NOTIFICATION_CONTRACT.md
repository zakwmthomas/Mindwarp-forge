# Owner Notification Contract

Forge records must not become invisible when the owner is away from the PC.
Any actionable problem is routed by the worker to this task's chat after it is
durably recorded.

## Notify

- owner decision, design gate, authority boundary, security/recovery failure,
  failed verification, repeated efficiency escalation, blocked active package,
  or material dependency change.

## Do not notify

- normal progress, successful routine verification, duplicate unresolved
  problem, or a problem already acknowledged without material change.

## Message receipt

Each notification contains problem ID, severity, affected package, evidence
link/reference, what was attempted, consequence of waiting, and the smallest
owner decision or acknowledgement needed. It never asks the owner to diagnose
code and never treats a reply as authority unless existing explicit-authority
rules are satisfied.

## Required proof

Fixtures prove remote-chat routing, deduplication, acknowledgement state,
material-change re-notification, offline queue/retry, rate limiting, and no
notification action can grant authority or mutate protected Kernel state.

Rate limiting defers an actionable notification into the durable outbox; it
never discards it. Critical security/recovery and authority-boundary failures
bypass the normal delivery limit. Revisions are monotonic per problem: the same
or an older revision is suppressed, while a higher revision is material change.
Retry rechecks acknowledgement and rate state before delivery. Notification
state and outbox writes use atomic replacement.

`delivered` means the task-chat routing adapter accepted and durably recorded
the envelope; it does not mean the owner read or acknowledged it. Owner
acknowledgement is a separate revision-bound record and never grants authority.
