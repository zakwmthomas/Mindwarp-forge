# G1 C3 Visibility and Traversability Consumer Audit

Date: 2026-07-15

## Decision

Physical visibility and traversability remain real C3 obligations, but neither
is implementation-ready from the current consumer graph. This is a dependency
finding, not permission to close or silently move either obligation.

The current code supports exact environmental signal availability, regional
exposure, global substrate evidence and an exact physical-regime identity. It
does not support a universal visibility distance or traversability score.
Adding either now would manufacture semantics that no current consumer can
state or test.

## Consumer evidence

| Concern | Current consumer evidence | Missing minimum physical input | Decision |
|---|---|---|---|
| visibility | `organism-niche-binding` can bind a sensory candidate to an available signal channel, but explicitly stops before propagation, receiver physiology and detectability; Reference Studio and viewport modules use diagnostic/UI visibility rather than world-space optics | emitter/receiver positions or path, intervening medium/occluder geometry, and a declared propagation/attenuation contract | consumer-blocked; retain signal availability and nonclaims |
| traversability | `macro-lineage-binding` carries an opaque `body_plan_ref`; `person-form-eligibility` records only structural evidence for a locomotion capacity and explicitly does not prove grounding | local surface/volume continuity, slope or depth/clearance evidence, support/contact constraints, and a body or locomotion envelope against which passage can be evaluated | consumer-blocked; retain substrate evidence and opaque body-plan identity |

## Boundary

The environment may eventually own observer-independent facts such as path
geometry, medium occupancy, surface continuity, slope, depth and clearance.
Visibility and traversability results are then evaluations over those facts and
an observer, receiver, body plan or movement mode. They are not free properties
of an entire world or physical regime.

Therefore this audit rejects:

- a single world visibility distance;
- a single world or region traversability score;
- arbitrary exposure or moisture bands standing in for either concern;
- inference of eyes, organs, body dimensions or locomotion from opportunity
  nodes;
- using the exact physical-regime identity as a similarity, biome, access or
  interchangeability claim.

## Dependency consequence

C3 cannot honestly finish these two evaluations before a later consumer states
the minimum path and body-relative contracts, while C4-C6 are currently marked
as dependency-gated by C3. The program must treat this as an explicit staged
interface dependency rather than resolve it by fabricating a scalar.

The next independent C3 risk that can advance without that scope inflation is
deterministic portability evidence for the existing field basis. A same-host,
second-language receipt can test an independent implementation now. It must
remain labelled as same-platform evidence and cannot satisfy the separate
second-platform requirement.
