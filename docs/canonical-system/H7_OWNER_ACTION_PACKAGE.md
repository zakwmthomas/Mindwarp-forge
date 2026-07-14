# H7 owner action package

Status: **exact candidate admitted to the live Forge ledger as `Proposed`;
approval is the current owner gate and promotion remains a separate later
gate**.

## Plain-language scope

This candidate is a verified **proof standard for future humanoid work**. It is
not a humanoid model, mesh, body template, rig, animation, shader, imported
asset, runtime choice, or phone-performance claim. It keeps the structural H3
wire proof and the owner-approved H5 visual direction explicitly separate.

## Exact identities

| Record | SHA-256 identity |
|---|---|
| H7 canonical package | `7b01d650258fe50b7cd59290a4a56e6df3a17271991dba313e29b6c0cf607619` |
| Canonical 1,512-byte Kernel evidence | `f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c` |
| Deterministic Kernel candidate | `c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433` |
| Required H6 manifest | `a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d` |
| Required H5 owner decision | `5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520` |

The candidate name is
`engine-neutral-humanoid-proof-baseline-v1`. Live admission reproduced the
pinned evidence and candidate IDs exactly.

## Six claims

1. H1 provenance is exact and authority-bounded.
2. The H2 neutral 17-joint structure is deterministic.
3. The H3 structural candidate rebuilds without external capability.
4. H4 structural controls are exact and local to the declared fixture.
5. H5 visual direction is owner-bound and separate from H3 geometry.
6. H6 chain identifiers, limitations, and receipts recover exactly.

## Eight non-claims

1. No generated or imported surface asset.
2. No anatomical truth or population-wide body rule.
3. No topology, UV, skinning, deformation, animation, physics, material, or
   shader proof.
4. No device, runtime, engine, production, or performance fitness.
5. No gameplay capability, intelligence, morality, social role, or importance
   rule.
6. No inheritance of human binary shape language by non-human lineages.
7. No application, execution, publishing, spending, acquisition, or protected
   Kernel mutation.
8. No approval, promotion, or supersession from evidence, readiness, or
   conversational assent.

## Tested live-admission boundary

The bounded operator path accepts no arbitrary package. It rebuilds the exact
canonical package internally, requires all pinned identities, and can add only
one assistant-authored evidence event followed by one `CandidateProposed`
event. A second run is mutation-free. It fails closed if an existing candidate
or its evidence has drifted, including if the candidate is no longer
`Proposed`.

Disposable SQLite tests prove initial admission, idempotent retry, reopen,
verified backup recovery, forged-assistant approval rejection, and state-drift
rejection. They produce no asset, code, filesystem application, execution,
publishing, runtime selection, approval, or promotion effect.

Live admission on 2026-07-14 reproduced the same boundary: the first run added
exactly two events and returned `Proposed`; immediate retry and post-restart
retry both returned `already_present=true` with zero events added. The verified
pre-admission backup is
`pre-h7-candidate-20260714-224529-4981469.sqlite3`, SHA-256
`46d6ae57a0dd9d3f9e9d1833ed0bf80115d0aa5824cecdef3ea268798cd95e1b`.
Forge capture then restarted current without changing the candidate state.

## Later owner actions, not current requests

The exact candidate now exists as `Proposed`. The first and only current owner
action is:

`APPROVE c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433`

Promotion remains a separate later action and cannot be combined with
approval:

`PROMOTE c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433`

The rollback target is `no_promoted_humanoid_proof_baseline`. If correction is
needed after approval or promotion, the protected append-only supersession path
must retain this package and bind new correction evidence. No phrase in this
document invokes any action.
