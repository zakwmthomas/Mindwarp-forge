# G1-C6 Result: Macro-Lineage Identity and Opportunity Occupancy Seam

## Question

After correcting the earlier category errors, what is the smallest causally
valid step between environmental opportunities and later body-plan, sensory,
species/ecomorph, and person-form work?

## Result

`crates/macro-lineage-binding` introduces one strict
`MacroLineageCandidate`. It binds:

- one exact `CausalWorldPacket`;
- that packet's exact replay-validated `WorldGenerationInput`;
- one exact `EnvironmentalOpportunityGraph`;
- a canonical non-empty subset of opportunities the candidate hypothesizes it
  can occupy;
- a stable lineage seed and optional parent lineage;
- one explicit opaque `body_plan_ref`;
- a deterministic `lineage_id` derived from all of those inputs.

The record is explicitly `Hypothesis`. Environmental opportunity does not
prove biological viability, evolution, or occupancy.

The opportunity graph validator now rebuilds the graph from the exact input
and packet. A caller-fabricated opportunity node can no longer become lineage
occupancy merely by copying the packet-ID string.

## Body-plan boundary

The owner previously deferred head/torso/body-region placement. This proof
preserves that decision:

- it binds a body-plan identity so later records no longer misuse world
  evidence as lineage identity;
- it contains no body-region fields and fixes
  `BodyRegionModelStatus::Deferred`;
- changing `body_plan_ref` changes lineage identity without inventing anatomy.

This is an identity and dependency seam, not a body-plan anatomy model.

## Evidence

`cargo test -p macro-lineage-binding`: five focused tests cover:

- exact world, graph, body-plan and occupancy binding;
- rejection of a graph from another world packet;
- rejection of empty or unknown opportunity occupancy;
- body-plan identity sensitivity with no head/torso fields;
- canonicalization of occupancy order and duplicates.

The Forge Desktop fixture records
`macro-lineage-identity-occupancy-seam` as a read-only ProofReceipt and asserts
that receipt recording changes no Kernel object, event, or candidate count.

## Non-claims

- No anatomy, body regions, organs, physiology, species, ecomorph,
  person-form, dimorphism, asset, animation, or visual-quality claim.
- No claim that an opportunity is a complete ecological niche.
- No claim that the candidate actually evolved, survives, or occupies the
  world.
- The lineage seed, body-plan reference, and occupancy fixture are synthetic
  proof inputs, not Mind Warp content grammar.

## Next causally valid step

Define the smallest bounded body-plan structural contract behind
`body_plan_ref`, still without selecting head/body-region placement unless a
separate evidence-backed scope justifies it. Sensory mechanisms can then bind
environmental support to a real lineage/body-plan candidate instead of
fabricating organs directly from world signals.
