# Regional Environment State Contract v1

This capability-free contract binds exact canonical `field-basis` recipe
bytes, reconstruction identity and one signed Q32.32 coordinate pair into a
deterministic regional state. Exposure retains the reconstruction identity as
its field stream key. Moisture potential uses a separately domain-keyed source
identity so the two dimensions do not silently share one random field.

`RegionalFieldBindingV1` is the canonical coordinate-free form of those exact
sources and recipes. It is owned by this module, decodes strictly, and can
reconstruct a point input either from an explicit coordinate or from an exact
`spatial-domain` cell. Downstream partition modules consume this binding rather
than inventing a second regional source schema.

Each sampled Q16.48 value must remain inside the signed unit interval. It is
mapped with checked integer arithmetic to a `0..1000` fraction:
`-1 -> 0`, `0 -> 500`, and `1 -> 1000`. Samples outside that range fail closed
rather than being clamped. Input, recipe, coordinate, state, claims and
authority fields replay exactly through strict canonical codecs.

Exposure and moisture potential are separate procedural plausibility evidence.
Exposure may bound palette
brightness and visible-radiance availability in `derived-world-rules`, but it
is not measured elevation, slope, aspect, shadow, cloud, terrain geometry,
weather, radiative transfer or physical visibility distance. Moisture
potential is not rainfall, humidity, soil moisture, groundwater, local water
quantity or surface coverage. Neither dimension is a biome,
traversability, habitability or runtime simulation.
