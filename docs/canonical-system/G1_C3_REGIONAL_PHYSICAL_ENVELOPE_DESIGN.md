# G1 C3 regional physical-envelope design

Status: **bounded implementation-ready design; not a biome or climate model**.

## Gap

The current regional contract varies only exposure. Atmosphere, solid surface
and surface-accessible liquid are exact but planet-wide facts. They cannot
distinguish a wet region from a dry region on the same world, so exposure bands
alone cannot honestly support regional biome structure.

## Minimum addition

Add one normalized regional moisture-potential field to the existing regional
contract. It uses exact canonical field-recipe bytes, the same coordinate and
a separately domain-keyed source identity. The signed normalized field sample
maps to `0..1000` with the same checked endpoint policy as exposure.

The value means procedural regional water-distribution potential only. It is
not rainfall, humidity, soil moisture, groundwater, surface coverage,
temperature-derived phase stability or habitat suitability. It becomes an
environmental opportunity only when the exact hydrological contract already
proves surface-accessible liquid exists.

Exposure and moisture remain separate typed dimensions. They are never summed
into a universal climate, biome, quality or habitability score.

## Adversarial requirements

- missing moisture identity, malformed/noncanonical recipe bytes and samples
  outside the normalized interval fail closed;
- changing only the moisture source or recipe may change moisture potential
  but must not change exposure;
- a dry world must not emit a moisture opportunity regardless of its field;
- graph validation must rebuild the new node from the exact world input;
- legacy bytes missing the required new field fail strict decoding;
- stop before biome labels, temperature, precipitation, transport, terrain,
  vegetation, organisms, visibility or traversability.
