# G1 C3 environmental-opportunity result

Status: **bounded prototype evidence; C3 remains active**.

## Implemented boundary

The former signal-only environmental-opportunity precursor now belongs to C3's
physical-world output boundary. It derives a closed typed set from the exact
validated world input and packet:

- regional radiant-energy exposure;
- surface-accessible liquid presence;
- atmosphere presence;
- bounded global solid-substrate fraction; and
- supported environmental signal channels.

The model does not collapse these into one universal scalar. Each opportunity
retains only the evidence appropriate to its kind. Co-availability says only
that two facts occur in the same world packet.

## Proof

`cargo test -p niche-graph-binding` passes seven focused tests covering:

1. physical and signal opportunities plus the complete co-availability graph;
2. weak signals remaining world evidence without becoming supported nodes;
3. absence of liquid, atmosphere and substrate preventing those nodes while
   radiant energy and a medium-independent magnetic signal remain valid;
4. deterministic replay and canonical order under signal input permutation;
5. rejection of incomplete relations;
6. exact serialization round trip; and
7. rejection of a fabricated unique opportunity against a fresh causal rebuild.

`cargo test -p macro-lineage-binding` passes all five downstream occupancy
fixtures without changing the explicit hypothesis-only lineage boundary.
`cargo test -p forge-desktop` passes all 41 read-only integration fixtures.
The complete repository gate passes governance, canonical coherence, all 35
module fronts, UI build and every workspace test. Its ordinary final desktop
build is blocked only by the running `target\debug\forge-desktop.exe` file
lock; an isolated-target desktop build with `RUSTFLAGS=-D warnings` passes.

## Root-cause improvement

The validator does not maintain an independent list of acceptable caller data.
It rebuilds the complete graph from the exact input/packet pair and compares
the result. Adding a new physical opportunity kind therefore requires the
builder, identity, serializer and adversarial tests to agree; fabricated or
stale nodes fail closed in future consumers too.

## Retained limits

This is not a complete niche graph or biome recipe. Habitat suitability,
hazards, trophic roles, competition, organism occupancy, physiology, terrain,
propagation, physical visibility distance, body-relative traversability and
biome naming remain unimplemented. The signal threshold is disposable fixture
policy, and the physical inputs remain bounded procedural evidence rather than
scientific validation.

The next C3 reassessment should determine the smallest physical regional
envelope needed for biome structure. It must not infer organisms, add a general
habitability score, or start a continuously simulated world.
