# Derived World Rules Contract v1

This is a capability-free causal seam between canonical field samples and
later hierarchy, ecology, sensory, aesthetic, construction and representation
systems. It is a synthetic reference contract, not a scientifically complete
planet model, visual-quality claim or runtime generator.

`WorldGenerationInput` binds the exact reconstruction,
the field contract version, an exact replayed `SurfaceMaterialContract` that
contains exact climate, hydrological, geological/atmospheric and stellar/orbital
evidence plus bounded dominant-surface reflectance, an exact replayed
`RegionalEnvironmentContract` containing canonical field recipe bytes,
reconstruction identity, Q32.32 coordinates and normalized exposure, and a
unique bounded set of environmental signal potentials. Every nested
reconstruction identity must match the containing
world input; caller-authored stellar, gravity, pressure, substrate,
atmospheric transmission or liquid-medium state cannot bypass those contracts.
Signal propagation is not accepted as caller-authored state because it is not
implemented. Baseline potentials are normalized integer permille values with
no physical unit claim. Floats, implicit units, cache
state, aesthetic intent, organisms, executable expressions and runtime objects
are not canonical inputs.

Signal potentials are a canonical set: serialization and compilation sort them
by channel, so caller ordering cannot change input, packet, graph, or lineage
identity. Legacy `transmission_permille` bytes and unknown fields fail strict
decoding. Packet content validates schema, ranges, signal ordering,
availability, limitations, authority effect, and content-derived identity.
Every downstream causal consumer must also receive the exact
`WorldGenerationInput` and replay `validate_world_packet`; a packet identifier
or schema number alone is never sufficient causal evidence.

The v1 compiler produces only:

- a physically caused three-band palette bound computed from validated
  stellar/orbital state, validated geological/atmospheric transmission and
  validated surface-material reflectance, then bounded by regional exposure;
- bounded environmental signal availability after only implemented modifiers;
  regional exposure modifies visible radiance while nonvisual potentials stay
  unchanged;
- explicit limitations and content-derived identifiers.

Pressure waves require atmosphere, chemical gradients require atmosphere or
exact hydrological evidence of surface-accessible liquid, and substrate
vibration requires solid substrate. Contradictory
medium claims, duplicate channels, invalid ranges, unknown fields, unsupported
field versions and noncanonical bytes fail closed. Eyes and visible light are
not assumed: a valid world may expose magnetic, chemical or other channels
while visible radiance is absent.

Regional exposure multiplies palette and visible-radiance strength only.
Nonvisual channels are unchanged. This is procedural spatial variation, not a
terrain, shadow, cloud, radiative-transfer or physical visibility model.
Baseline potential is not emitted power, propagated intensity, a detector
measurement, biological detectability, or evidence of distance/frequency
attenuation.

The output cannot contain palette preference, beauty, harmony, organism,
biome, shader, asset, approval, promotion or runtime authority. Later systems
may choose accessible player-facing representations, but they must retain this
physical cause separately from aesthetic intent.

Nature-inspired methods are not selected here. Diffusion, SDF, Voronoi,
branching and related mechanisms remain P16 candidates with their own local
baseline, metric, cost, falsifier and non-applicable scope.
