# G1 C3 physical-regime identity result

Status: **bounded prototype evidence; C3 remains active**.

The environmental-opportunity graph now retains two distinct identities:

- its existing graph fingerprint binds the exact world packet, node identities
  and provenance; and
- `physical_regime_id` hashes only the canonical sorted physical opportunity
  values.

Macro-lineage candidates bind both the exact graph reference and the physical
regime reference. This lets later work recognize exact physical equality across
different places without erasing place identity or inventing exposure/moisture
bands.

Focused proof passes seven opportunity-graph tests, six macro-lineage tests and
all 41 desktop tests. The new adversarial fixture proves different provenance
with identical physical values yields different graph fingerprints and equal
regime IDs, while changing regional moisture changes the regime ID.

This identity supports exact equality only. It is not a biome, fuzzy similarity
metric, clustering method, habitability or quality score, organism inference or
authority to reuse one place's mutable state in another.

The complete repository gate passes governance, canonical coherence, all 35
module fronts, UI build and every workspace test. Its ordinary final desktop
build is blocked only by the running executable lock; the isolated warnings-
denied desktop build passes.
