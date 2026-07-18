# Step-Leader Controller Result

Status: `reference_proven`

## Outcome

The bounded divergence protocol is implemented as the capability-free
`step-leader-controller` Rust crate. It accepts explicit inputs and returns a
deterministic advisory decision. It has no filesystem, network, process,
runtime, model, policy, persistence, spending, publishing or promotion
authority.

The intake source was the owner-provided
[shared Gemini conversation](https://share.gemini.google/1E5E6wy1dxmd). Its
metaphors were not accepted as facts. The companion candidate map separates
mechanisms from claims, records contradictions and non-applications, and maps
one assessment row to every current registered system.

## Enforced invariants

- ordinary edits and heartbeat wakes do not trigger divergence;
- qualifying triggers are bounded to the P19 hybrid trigger set;
- an incomplete or duplicated system map fails closed;
- applicable probabilities must sum to one million parts per million;
- only positive target-local value of information and local net gain rank;
- probe cost cannot exceed ten percent of the prior three meaningful batches,
  one normal batch, or the external-source cap;
- each decision binds the saved checkpoint it must resume;
- any target regression quarantines the mechanism and blocks reuse;
- one success is local retention only, while two independent successes are
  required for a transfer candidate.

## Verification

Eight deterministic Rust tests cover trigger selection, ranking, complete-map
failure, budget failure, malformed probability input, regression masking,
two-success promotion and checkpoint-bound determinism. The repository verifier
also compares the candidate map against the live system registry so a future
registry addition makes the projection stale instead of silently disappearing.

## Mainline reconnection

This bounded tangential implementation does not change the active C3 product
architecture. Work reconnects to the revised fixed-160 interval-incident
implementation-readiness audit recorded in the canonical master program.
