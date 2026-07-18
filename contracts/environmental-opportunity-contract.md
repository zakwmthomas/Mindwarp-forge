# Environmental Opportunity Contract v1

This capability-free C3 contract projects already validated physical world
evidence into a deterministic environmental-opportunity graph. It supplies a
physical ecology substrate for later biome, niche and organism work; it is not
a biome model, habitat-suitability model or organism simulation.

The builder accepts an exact `WorldGenerationInput` and its validated
`CausalWorldPacket`. It replays that pair before deriving any node. Callers
cannot submit opportunity nodes as canonical facts.

The closed v1 opportunity set is:

- `radiant_energy`, present only when absorbed stellar energy and nonzero
  regional exposure both exist; it retains the bounded regional-exposure
  permille value;
- `surface_accessible_liquid`, present only when the exact hydrological state
  says surface-accessible liquid exists;
- `surface_moisture_potential`, present only with that exact liquid evidence
  and retaining the independent normalized regional potential;
- `atmosphere`, present only when the exact geological/atmospheric state has
  nonzero surface pressure;
- `solid_substrate`, present only when the exact state has nonzero solid
  surface fraction, retaining that global bounded fraction; and
- `signal`, present only when a packet signal clears the disposable fixture
  threshold, retaining its channel and effective permille value.

These values deliberately keep unlike meanings separate. Presence is not a
quantity, global solid-surface fraction is not local terrain, regional exposure
is not physical visibility, moisture potential is not weather or local water
quantity, and signal strength is not biological
detectability.

Nodes have content-bound identities. Edges are the complete unordered
co-availability relation among the nodes in one packet; they imply no causal,
trophic, spatial, evolutionary or anatomical connection. Validation rejects
schema drift, duplicate or unsupported opportunities, fabricated content,
dangling or incomplete edges, and any graph that differs from a fresh rebuild
of the exact source pair.

The graph also carries a physical-regime identity hashed from the canonical
sorted opportunity values without packet or node provenance. It supports exact
physical equality only. The place-specific graph fingerprint remains separate;
physical-regime equality is not a biome label, similarity distance, quality
score or evidence that two places are interchangeable.

The contract does not emit habitat suitability, hazards, resource yield,
trophic roles, competition, organism occupancy, physiology, body plans,
species, ecomorphs, propagation, visibility distance, terrain,
body-relative traversability, biome names, approval, promotion or runtime
authority.
