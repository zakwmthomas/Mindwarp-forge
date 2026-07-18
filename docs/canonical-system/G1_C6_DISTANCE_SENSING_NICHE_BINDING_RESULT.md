# G1-C6 Corrected Result: Environmental Support for Sensory Candidates

**Status:** bounded prerequisite proof; corrected after logical audit on
2026-07-15. It does not close sensory-channel feasibility.

## Audit finding

The first result said that environmental signal availability proved a sensory
mechanism was feasible. That was too strong. A pressure wave does not by
itself prove echolocation: an organism also needs a viable emitter, receiver,
signal-processing physiology, body-plan integration, behaviour, and lineage
history. The same distinction applies to vision and chemoreception.

The original package also created a generic organ node before any body plan or
lineage existed, and its observed claims were attached to mechanism concepts
even when the packet only observed signal measurements.

## Corrected bounded proof

`crates/organism-niche-binding` now proves only a disposable
environmental-support gate:

- observed claims describe packet signal measurements, including weak or zero
  measurements; they do not claim an organism possesses a mechanism;
- `SolutionFamily.feasible` is explicitly local to this fixture and means
  "clears the environmental-support threshold", not biological feasibility;
- no organ or body-part node is constructed;
- the 300-permille threshold and three candidate labels remain disposable test
  data, not product vocabulary or biological constants;
- `semantic-construction`'s validator remains unchanged.
- the exact `WorldGenerationInput` must replay to the supplied packet before
  any environmental-support candidate is constructed.

## Evidence

`cargo test -p organism-niche-binding` retains four focused cases:

1. strong visible radiance supports the photopic-vision candidate at this
   necessary-condition gate;
2. pressure-wave and chemical-gradient availability support two distinct
   candidates;
3. no supported signal yields `no_feasible_family`;
4. changing only the real world packet changes the deterministic package
   fingerprint.

The Forge Desktop receipt remains read-only, but its interpretation is limited
to this environmental-support prerequisite.

## Retained limits

This result proves neither a sensory organ nor a complete sensory mechanism.
Macro-lineage identity, body plans, emitter/receiver requirements, physiology,
species/ecomorph derivation, person-form evaluation, dimorphism, assets, and
visual quality remain open.
