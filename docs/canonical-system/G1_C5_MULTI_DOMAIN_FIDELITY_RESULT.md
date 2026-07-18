# G1 C5 multi-domain consumer fidelity result

Status: **first C5 proof component passed; C5 remains executing.**

## What this closes

C5's next action asked for a gap audit against `SIGNIFICANCE_SCHEDULER_DESIGN_GATE.md`
followed by proof of "shared fidelity/priority control across ... AI, physics,
animation, audio, rendering and streaming" for at least two structurally
different consumer domains.

The gap audit found that `significance-scheduler` already implements almost
everything the P5 gate required (`ImportancePacket`, `HysteresisPolicy`,
`SignificanceState`, `ConsumerFidelityMap`, the scheduler's admission/
dispatch/cancellation/fallback machinery, and a read-only ProofReceipt
fixture) and is already `prototype_tested`. The one gap: every existing test
exercised `ConsumerFidelityMap` generically (abstract `u16` consumer IDs,
arbitrary monotone level arrays). Nothing had exercised multiple *named,
structurally different* consumer domains sharing one real tier at once.

## New evidence

`crates/significance-scheduler/tests/multi_domain_consumer_fidelity.rs` adds
4 focused tests using four illustrative domains (AI, animation, audio,
rendering) with deliberately different curves (front-loaded, back-loaded,
coarse, smooth):

1. The same shared tier produces different fidelity per domain — no single
   universal scalar describes all four domains.
2. A single shared, hysteresis-protected tier absorbs a raw flapping signal
   (5 raw changes collapse to fewer tier transitions), and every domain
   reads that same stabilized tier rather than reacting to raw noise
   independently.
3. A protected threat flag forces every domain to its Critical fidelity
   simultaneously, even at a near-zero raw signal.
4. Constructing or using one consumer's fidelity map cannot affect another's
   output; there is no shared mutable registry.

## Explicit non-claims

- The chosen fidelity curves (`[2,8,12,16]` for AI, etc.) are illustrative
  test fixtures, not approved product weights for any real system.
- No scheduler dispatch, admission, budget, or cancellation behavior was
  exercised by this result; it only proves the shared-significance /
  per-domain-fidelity seam. The scheduler's own admission/fairness/
  cancellation proof already existed before this result and is unchanged.
- Physics and streaming were not given named example curves in this pass;
  the proof used AI, animation, audio and rendering as the "at least two
  structurally different" requirement, which is satisfied, but a future
  pass could add explicit physics/streaming fixtures if the owner wants
  full six-domain coverage.

## What C5 still has open

- Runtime controllers, executors, real cache eviction/frequency policy, and
  production timing claims remain explicitly deferred by the original P5
  gate and are untouched here.
- Cross-system proof that connects the scheduler's dispatch/admission layer
  (not just the shared significance/fidelity layer) to multiple named
  consumer domains at once.
- Second-platform receipts, matching the same open item carried from P4/C4.

C5 is not closed by this result. The next decision is the same pattern used
for C3 and C4: present this evidence for an explicit owner promotion-or-
continue decision.
