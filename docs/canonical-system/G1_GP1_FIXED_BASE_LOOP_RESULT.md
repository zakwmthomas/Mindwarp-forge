# G1 GP1 fixed base-loop result

Status: independently accepted and completely verified bounded GP1 closure.

## Result

The `mindwarp-gameplay-foundation` crate now reuses all five GP0 authored
sessions in one engine-neutral deterministic loop: prepare, depart, encounter,
consequence, return, and remembered response. Preparation owns intent, fitting
tool, and threat posture only. Outcome selection remains an encounter decision
after observation and inference.

Typed stable stops exist after preparation, consequence, return, recoverable
failure, and terminal history append. Ordinary departure and encounter are not
safe stops. Failure preserves the plan, optional world context, history, and
unselected outcome; three recoveries are allowed and a fourth failure rejects
before mutation.

`BaseLoopLedgerV1` pairs append-only `WorldHistoryV1` with typed run/event/session/outcome
receipts and an explicit first-GP1-event floor. Remembered response appends both
atomically, rejects duplicate or uncovered completion, and permits a later
session only under a new run ID. Existing history before the floor does not
need a synthetic run identity. S5 deterministically uses the latest preceding
S1 event, allowing legitimate repeats while rejecting missing or latest-retreat
predecessors.

Authored fixtures use an explicit authority-negative context with no C3A
identity. The optional validated context reuses `bind_validated_c3a_world` and
rejects foreign packets. State decoding also requires the expected external
world context, so serialized identity cannot self-authorize. No C3B authority
is introduced.

The focused suite proves the same six phases across all five sessions,
recovery overflow, strict codecs, replay-bound public state, phase and safe-stop
rejection, run idempotency, exact C3A binding, deterministic latest legitimate
S1 selection with missing/latest-retreat rejection, and distinct
direct/bypass/ration S1 histories producing distinct
S5 state and memory while preserving S1 then S5 order.

## Boundary

This remains a capability-free `prototype_tested` package. It adds no runtime,
graphics, persistence, procedural breadth, grind, network, Greenfield, C3B, or
GP2 progression authority.
