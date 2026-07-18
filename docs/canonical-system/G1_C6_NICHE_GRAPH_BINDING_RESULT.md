# G1-C6 Corrected Result: Environmental Opportunity Graph Precursor

**Status:** corrected after logical audit on 2026-07-15, then superseded at the
physical boundary by `G1_C3_ENVIRONMENTAL_OPPORTUNITY_RESULT.md`. The earlier
body/organ graph remains invalidated and grants no niche-graph completion
claim. C6 retains organism roles and occupancy; the physical opportunity graph
is now explicitly C3 evidence.

## Audit finding

The first implementation made two successive mistakes:

1. It directly joined a sensing organ to a communication organ. The owner
   correctly rejected that physical model, and the implementation was changed
   so both organs attached to an abstract body.
2. A later whole-system audit found the deeper category error: even the
   corrected body attachment model was not an ecological niche graph at all.
   It was a tiny body-part graph. `MASTER_PLAN_V2.md` says planetary
   macro-lineages fill locally generated niche graphs; it does not define a
   niche graph as physical connections among organs.

The merged `SemanticConstructionPackage` also put alternatives for two
independent roles into one `SolutionFamilySet`, then selected one family while
constructing both roles. That passed the generic validator because the
validator checks structural closure, not whether the caller grouped unrelated
decision problems correctly. The pass therefore did not prove the intended
semantics.

## Corrected bounded proof

`crates/niche-graph-binding` now builds an
`EnvironmentalOpportunityGraph` directly from a real
`derived-world-rules::CausalWorldPacket`:

- each node is one environmental signal channel whose effective strength
  clears a disposable support threshold;
- each edge means only that two retained channels are co-available in the same
  world packet;
- node and edge identities are deterministic and content-bound;
- the exact world input and packet are replayed, and validation rebuilds the
  expected graph rather than trusting caller-supplied node IDs or strengths;
- the validator rejects duplicate nodes, unsupported nodes, dangling,
  duplicated or misidentified edges, incomplete coavailability relations, and
  schema drift.

This graph is intentionally a precursor to a future ecological niche graph.
It contains no organ, body, body region, physical joint, organism capability,
lineage, emitter, receiver, habitat, resource, hazard, trophic role,
competition, or occupancy claim.

## Evidence

`cargo test -p niche-graph-binding`: five focused tests cover:

- two supported channels becoming two environmental opportunities with one
  coavailability relation;
- weak channels remaining in world evidence while being excluded from the
  opportunity graph;
- deterministic equivalence under actual signal-source input-order
  permutation;
- fail-closed rejection of incomplete edges;
- exact serialization round trip.

The Forge Desktop read-only receipt now records
`environmental-opportunity-graph-precursor`, replacing the invalid
`niche-graph-two-organ-causal-binding` claim.

## Retained limits and next dependency

This does not close the full niche-graph sub-scope. The current world contract
only exposes physical palette and signal availability. A complete ecological
niche graph still needs bounded habitat/resource/hazard/trophic-role inputs.
The next causally valid organism step is a macro-lineage/body-plan identity
contract that can later occupy such ecological opportunities without
fabricating anatomy from environmental support alone.
