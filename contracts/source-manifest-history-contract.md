# Source Manifest History Contract v0.1

## Purpose

Preserve every distinct source-manifest coverage projection derived from
verified `SourceChunkEnvelope` records. History is append-only evidence: it
does not approve, promote, apply, or reinterpret imported content.

## Record

Each record contains a global sequence, source ID, positive manifest version,
coverage state and reason, declared chunk count, ordered present indices, and a
deterministic projection hash. Manifest versions are isolated; a later version
never rewrites the history of an earlier one.

## Append and replay law

A new envelope appends history only when its deterministic projection differs
from every retained projection. Equal retries are idempotent. If an interrupted
write retained the envelope but lost its history projection, retrying the equal
envelope reconstructs that projection exactly once. Reopen returns the same
ordered records and hashes.

## Authority boundary

`complete` means only that every declared envelope index is present for one
manifest version. It never changes candidate state or gives authority to
approval language inside raw or parsed source evidence.

## Proof fixtures

- out-of-order incomplete-to-complete transition history;
- equal retry without duplicate history;
- interrupted history-write repair by equal-envelope retry;
- manifest-version isolation;
- exact reopen/replay; and
- complete imported approval language remaining proposed only.
