# Significance and scheduler reference contract

Status: `prototype_tested`. This contract defines test evidence, not a runtime,
engine, performance claim, cache policy, or owner approval surface.

## Inputs and invariants

- `ImportancePacket` is strict canonical CBOR with a descriptor fingerprint,
  request epoch, bounded signal vector, policy version, and protection flags.
- Significance uses asymmetric hysteresis and minimum hold time. Protection may
  raise a tier; it may not erase the original evidence.
- Each consumer owns a monotone tier-to-fidelity map. The shared packet does not
  prescribe rendering, simulation, AI, physics, animation, audio, or streaming
  behavior.
- `WorkTicket` names exactly one resource. Multi-resource work is an admitted
  acyclic graph of phase tickets with stable identifiers.
- A fallback is a validated, inactive, strictly cheaper ticket for the same
  target, epoch, and resource. It is never an untyped command or callback.
- `BudgetEnvelope` is a versioned input. P5 does not derive budgets from clocks,
  hardware, temperature, load, or frame time.

## Dispatch and failure rules

- Safety-deadline work is admitted only when declared reserve can finish it by
  its deadline. Unused reserve may be reclaimed by general work within the same
  deterministic step.
- A live blocked dependent may donate derived urgency to prerequisites. Donation
  changes neither canonical significance nor stored tickets, and is recomputed
  when dependency state changes.
- Bounded service debt prevents starvation independently of significance.
- Cancellation has requested, acknowledged, and settled states. Cancellation
  cascades through declared ownership links; late or stale-epoch outputs are
  traced and discarded.
- Dependency failure and missed deadlines reject the affected ticket and may
  activate only its validated fallback.
- Trace fingerprints use explicit stable decision codes, not debug formatting.

## Authority and integration boundary

The crate is deterministic, in-memory, integer-only, and capability-free. It
cannot access the filesystem, processes, network, Tauri, or protected Kernel.
Forge Desktop may store serialized proof evidence as a read-only `ProofReceipt`;
doing so must not change object, event, or candidate counts. Real executors,
threads, clocks, cache admission/eviction, resource controllers, engine objects,
product weights, and production performance claims remain future gated work.
