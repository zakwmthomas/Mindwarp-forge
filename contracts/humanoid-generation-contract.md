# H3 humanoid generation contract

Status: `prototype_tested`. This is a capability-free, engine-neutral,
structural candidate generator. It is not an asset generator, mesh or volume
format, rigging or deformation system, renderer, engine adapter, visual
quality proof, approval, promotion, or production pipeline.

## Ownership boundary

- P6 `SemanticConstructionPackage` remains the sole owner of functional roles,
  capability closure, ordered recipe operations, preconditions, and exact
  graph replay. H3 binds its package, semantic, and recipe-result fingerprints;
  it does not define a second recipe.
- H2 `NeutralHumanoidProfile` remains the sole owner of joint identity,
  hierarchy, semantic role mapping, coordinate convention, rest pose, link
  roles, structural bounds, H1 lineage, and negative claims. H3 validates and
  projects those exact records; it does not invent anatomy or geometry.
- H3 owns only the deterministic one-way binding and the resulting unapproved
  structural candidate identity.

## Input and execution

`GenerationInput` is strict canonical JSON and binds the exact P6 package,
semantic kernel, recipe result, and H2 profile fingerprints. Its generator
profile is fixed to `pure-structural-projection-v1`, its maximum output is 64
joints and 96 links, and it lists the complete forbidden capability set:
filesystem, process, network, clock, randomness, plugin, external executable,
protected-Kernel mutation, approval, and promotion.

Generation is a pure in-memory projection from borrowed typed inputs. It does
not read paths, load plugins, launch processes, fetch content, consult a clock,
use randomness, write durable state, or call Forge authority. Invalid or
indeterminate P6 input, invalid H2 input, binding drift, capability drift, or
budget exhaustion returns no candidate.

## Candidate and proof

The candidate contains exactly H2's 17 joint IDs, semantic roles, and rest
positions plus its 16 directed parent-child links and link roles. Candidate
validation recomputes identity and compares every joint and link to H2. Exact
limitations forbid surface/volume geometry, skinning, inverse-bind matrices,
deformation, animation or physics quality, visual or perceptual approval,
engine compatibility, and production readiness.

The fixed reference input fingerprint is
`5667d387e4f7a0159fee99bab584c9481cc42b549535eaaec78de3a7b5796adf`.
The fixed candidate and replay fingerprint is
`4d04df0dd58cdd8ecdb7c41e9dbde2dec1910b36b7d5643b2d254ef4b3c707fa`.

This H3 stage uses no rendered visual asset. Under P13, any later rendered
human reference or candidate requires actual-pixel fitness receipts and an
owner visual check whenever anatomy, quality, accuracy, or creative intent is
uncertain.
