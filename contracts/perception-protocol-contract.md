# Controlled perception protocol contract

Status: `prototype_tested` for the bounded capability-free P7b-0 reference.
This is protocol evidence only. It does not create, open, inspect, compare, or
judge images and does not select a renderer, DCC, asset format, art style,
quality threshold, runtime, or engine.

## Contract boundary

- `ReviewProtocol` binds an exact P7a artifact and derivative lineage before
  stimuli exist. Assertions, presentation modes, blinding, randomization,
  anchors, failure controls, repeat policy, allowed outcomes, and stop rule are
  explicit. Hindsight-written assertions change the fingerprint and fail.
- `EnvironmentProfile` pins tool/config fingerprints, coordinate and unit
  conventions, camera, projection, framing, resolution, time samples, lighting,
  background, color configuration, output transform, display conditions, and a
  declared reproducibility class. P7b-0 uses inert synthetic identifiers and
  launches no tool.
- `StimulusManifest` retains immutable artifact/derivative inputs, exact
  protocol/environment fingerprints, blinded pair order, assertion coverage,
  complementary presentation modes, controls, omissions, and opaque synthetic
  stimulus references. A beauty view alone is insufficient.
- `ObservationSet` binds assertion-specific outcomes and limitations. It keeps
  `no_preference`, `indeterminate`, and metric/human contradictions. Creative
  director evidence has project-direction scope only; it cannot become a
  population-preference claim.
- `AnalysisReceipt` contains recomputed per-assertion counts. Missing coverage,
  duplicate presentation order, failed controls, stale bindings, lost
  contradictions, or fabricated summaries fail closed.

Validation is deterministic and bounded. Budget exhaustion returns
`indeterminate_budget`. The reference has no filesystem, network, process, GPU,
desktop, renderer, or protected-Kernel dependency. Forge Desktop stores only
serialized evidence as a read-only ProofReceipt and changes no Kernel object,
event, candidate, approval, promotion, or authority state.

The fixture identities, statements, profiles, orders, outcomes, and counts are
synthetic discriminating data. They are not Mind Warp art direction, player
research, general recognisability evidence, a rendering result, or authority for
P7b-1 containment, P7b-2 visual review, asset generation, or animation.

