# ProofReceipt Projection Contract v1

ProofReceipt is an immutable, data-only, engine-neutral result record stored in
an additive versioned SQLite projection. It references retained Kernel objects
for every input and output but is not itself a protected Kernel object or
event. Receipt status and text grant no approval, promotion, application,
execution, lifecycle transition, or owner authority.

Every admitted receipt has schema version 1; a canonical content-derived ID;
a registered canonical system and named proof; explicit pass, fail, blocked, or
incomplete state; exact input/output references; fixture, generator, and
contract versions; an equivalence method; classified measurements; limitations;
and informational runner/time provenance. Failed, blocked, or incomplete rows
must name a failure classification. Pass rows cannot conceal one.

Receipt and ordered evidence-link rows commit in one transaction. Foreign keys
and admission validation reject missing evidence. Equal retries are idempotent;
content drift, duplicate version names, unknown systems, malformed metrics,
unsupported schema versions, or corrupted linkage fail closed. Reads revalidate
the canonical ID and exact links rather than trusting stored JSON alone.

Reference Studio exposes the verified local rows read-only and visibly reports
schema mismatch. Inspection has no write command and must leave Kernel object,
event, candidate, and control-record counts unchanged. The existing verified
online backup includes the additive tables; reopen and backup fixtures must
prove recovery. An older F4 build is the rollback target and can ignore the
tables without rewriting protected history.
