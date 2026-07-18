# G1 C3 regional physical-envelope result

Status: **bounded prototype evidence; C3 remains active**.

The regional contract now exposes two separately proven dimensions: exposure
and moisture potential. Moisture uses exact canonical recipe bytes, coordinate,
reconstruction identity and a separately domain-keyed source identity. Both
samples map from the signed unit interval to integer permille with checked
arithmetic and reject out-of-range values.

The environmental-opportunity graph emits a typed regional moisture-potential
node only when the exact hydrological chain proves surface-accessible liquid.
It does not combine exposure and moisture into one score or label a biome.

Proof:

- `regional-environment-state` passes eight tests, including independent
  moisture-source causality with unchanged exposure;
- `niche-graph-binding` passes seven tests, including dry-world suppression and
  exact graph rebuild;
- `macro-lineage-binding` passes all five downstream tests;
- strict codecs reject missing/unknown fields and noncanonical bytes.

The complete repository gate passes governance, canonical coherence, all 35
module fronts, UI build and every workspace test. Its ordinary final desktop
build remains blocked only by the running executable lock; the isolated-target
desktop build passes with warnings denied.

This is not rainfall, humidity, soil moisture, groundwater, temperature,
terrain, vegetation, habitability, climate simulation or a biome recipe. The
next C3 decision may classify bounded exposure/moisture regimes only if a
current consumer needs those categories and the thresholds remain explicit
fixture policy rather than universal natural constants.
