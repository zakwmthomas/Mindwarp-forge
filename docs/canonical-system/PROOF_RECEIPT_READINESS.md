# ProofReceipt and Read-Only Inspector: Readiness Package

**Status:** discovery and readiness only. This document does not authorize F5
implementation, change the active milestone, or select a runtime engine.

## Why this is first

Every later engine-neutral proof needs an inspectable, comparable record of
what was run, with which inputs, under which versions, at what cost, and with
what outcome. The current Kernel already protects immutable evidence and replay
inputs; the control plane models lifecycle/authority; the dossier is a
read-only ledger projection. None of those current contracts defines a shared
proof-result envelope for canonical world, field, semantic, construction, or
asset tests.

This package fills that contract gap without running generated code, creating
a 3D runtime, or making a dashboard capable of mutating the ledger.

## Reuse check

| Existing component | Reuse decision | Boundary retained |
|---|---|---|
| Kernel contract and content-addressed evidence | Reuse as the authority and evidence boundary | A receipt references evidence; it never carries approval authority |
| Control-plane lifecycle | Reuse for package routing | Receipt state does not advance a work item by itself |
| Dossier projection | Extend later, not replace | Inspector remains read-only and derives from verified local data |
| Local capture/bootstrap | Reuse for source context only | Captured chat is evidence, not proof success |

## Proposed data-only contract

A versioned `ProofReceipt` should be a data record with these required fields:

| Field | Purpose | Constraint |
|---|---|---|
| `receipt_id`, `schema_version` | Stable identification and interpretation | Identifier is content-addressed or otherwise deterministic from canonical content; schema version is explicit |
| `system_id`, `proof_id` | Route result to one canonical system/proof | `system_id` must exist in the canonical registry |
| `status`, `failure_classification` | Distinguish pass, fail, blocked, and incomplete | A missing/invalid result never defaults to pass |
| `input_refs`, `fixture_id` | Reconstruct exact proof inputs | Inputs resolve to retained evidence/artifact IDs; fixture version is recorded |
| `generator_versions`, `contract_versions` | Detect semantic drift | All versions are named; no ambient runtime version is canonical |
| `output_refs`, `equivalence` | Reproduce or compare result | Output hash or explicit semantic-equivalence method is required |
| `measurements` | Record cost without claiming engine performance | Units, sampling method, and simulated/estimated labels are explicit |
| `warnings`, `limitations` | Prevent overclaiming | Reference proof cannot erase known limitations |
| `created_at`, `runner_identity` | Audit execution provenance | Informational provenance only; these fields grant no authority |

The initial contract deliberately excludes arbitrary executable payloads,
runtime-engine objects, credentials, raw transcript bytes, owner approval,
promotion controls, and network actions.

## Inspector contract boundaries

The first Reference Studio inspector may display a receipt's system/proof ID,
versions, inputs, outputs, measurements, limitations, and failure reason. It
must additionally show that it is read-only. It must not:

- execute a fixture, generated code, or external process;
- mutate the Kernel, a candidate, a work item, or a filesystem artifact;
- fetch a network source;
- infer approval/promotion from receipt text or status; or
- show sensitive/raw capture content by default.

## Required negative cases

1. Unknown registry system ID is rejected.
2. Missing input reference, fixture version, or output equivalence is rejected.
3. A `pass` receipt with an unresolved failure classification is rejected.
4. A malformed measurement unit or unlabelled estimate is rejected.
5. An inspector projection leaves ledger object/event/candidate counts unchanged.
6. A receipt carrying authority-like fields cannot authorize, promote, or apply
   a candidate.
7. A version mismatch is visible as a mismatch, never silently compared as if
   equivalent.

## Minimum proof fixture

Use one data-only fixture for `universe-identity` after the harness exists:

- fixed seed/address/generator-version inputs;
- deterministic reconstructed address and stream-partition outputs;
- a declared semantic-equivalence method;
- an explicitly simulated or measured cost record;
- a corrupt or incompatible-version sibling fixture that fails visibly.

This fixture is intentionally not a game scene, engine project, mesh, or
runtime import.

## Readiness gates for a future F5 work package

- A named contract owner/module boundary is recorded without modifying the
  protected Kernel's authority semantics.
- A schema and validator test plan cover all required and negative cases above.
- The read-only inspector is limited to verified local receipt data.
- Receipt storage/recovery/retention is defined and has a rollback path.
- The work package explicitly states its implementation scope, tests, and
  authority lane before any code is written.

## Current result

The package is ready for owner-authorized F5 implementation planning. Its
highest-risk unresolved design point is the storage binding: whether receipts
are first-class Kernel objects or a separately versioned projection linked to
Kernel evidence. That choice affects protected-kernel scope and must not be
made autonomously.
