# Reference Studio Read-Only Inspection: Readiness Package

**Status:** B3 verified. The Studio is an inspector, never a control plane,
code executor, file browser, network client, or runtime engine.

## Inspection boundary

For every proof/registry item, the Studio may display only verified local
projection data: system/proof identity, input/fixture/version references,
output/equivalence reference, measurements and labels, limitations, failures,
linked evidence, and read-only lineage. Raw capture content is hidden by
default.

Every view must state its projection source, verification time, and read-only
status. It must not mutate ledger counts/candidates/work packages, execute a
fixture, access files/network, infer authority from displayed text, or embed
approval/promotion/application controls.

## Required fixtures

- empty projection and one valid proof receipt;
- failed, blocked, and version-mismatched receipt visibility;
- missing evidence/source-gap display without fabricated details;
- mutation-negative check proving ledger/object/event/candidate counts remain
  unchanged after inspection;
- hostile displayed text that cannot create authority or a UI action.

## Entry criteria

- ProofReceipt storage boundary is resolved and receipts are verified local
  data; inspector contracts version their projections.
- Every inspector screen links to source/evidence and exposes limitation/failure
  state, not only successes.
- Visual/3D viewports remain optional later modules; they cannot become a
  prerequisite for basic receipt inspection.

## Implemented B3 slice

- A schema-versioned desktop command projects existing verified-local B1/B2
  records only: lifecycle gates, failures, blockers, rollbacks, and known
  research source gaps.
- Every view states source, verification time, read-only status, limitations,
  and explicit compatibility or version mismatch.
- The UI exposes one refresh control and no approval, promotion, application,
  execution, filesystem-browsing, network, capture, or runtime action.
- Focused desktop fixtures prove failure/blocker/source-gap visibility,
  version-mismatch display, hostile authority text inertness, and unchanged
  kernel object/event/candidate counts. UI TypeScript/Vite build passes.
- This slice deliberately does not choose or implement the owner-gated
  ProofReceipt storage binding. The full Forge gate passes with 15 desktop and
  56 kernel tests.
