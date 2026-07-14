# H7 humanoid promotion-readiness contract

Status: `prototype_tested`. This is a capability-free scope and lifecycle
simulation. It creates no Kernel candidate and grants no approval, promotion,
supersession, import, execution, or application authority.

The only admissible candidate type is
`engine-neutral-humanoid-proof-baseline-v1`. It binds H6 manifest
`a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d`,
H5 receipt
`5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520`,
six exact claims, eight exact non-claims, two separate owner actions, and
rollback target `no_promoted_humanoid_proof_baseline`.

The package ID is
`7b01d650258fe50b7cd59290a4a56e6df3a17271991dba313e29b6c0cf607619`.
The data-only simulated candidate ID is
`2d93d3ae31de08754852e27a3a04332d009b456141565be8e805a98eed8d6222`.
Neither is a real Kernel candidate ID.

Strict canonical decoding rejects ambiguous names, production-asset scope,
stale H6 or missing H5 bindings, merged H3/visual claims, absent non-claims,
missing rollback, missing owner actions, unknown fields, and authority drift.

The non-authoritative lifecycle model permits only:

- proposed to approved by an exact direct-owner action;
- approved to promoted by a second exact direct-owner action; and
- approved or promoted to superseded by an exact direct-owner action with a
  lowercase SHA-256 correction-evidence ID and no self-replacement.

Forged actors, stale candidate IDs, skipped stages, proposed supersession,
missing correction evidence, and self-replacement fail closed. Supersession
retains the old evidence-package ID and declares no Kernel, asset, application,
execution, publishing, or runtime-selection effect.

This simulation justifies designing a matching append-only protected-Kernel
transition. It does not authorize that implementation. The current Kernel and
explicit-authorization contracts remain unchanged until separate owner
authorization is given.
