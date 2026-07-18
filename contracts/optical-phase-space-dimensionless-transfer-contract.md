# Optical phase-space dimensionless-transfer contract

## Scope

This additive V1 contract replays one complete whole-cell receiver-coupling
result and its immutable-origin transport evidence, then asks the existing
visible-radiance bulk owner for each selected-band cell-step optical depth. It
composes a conservative dimensionless transfer enclosure for the same complete
cell and one deterministic band/time identity.

It owns no source magnitude, radiance, detector response, visibility,
perception, rendering, gameplay, runtime, approval, promotion, or C3 closure.

## Required replay

- Recompile the complete bulk profile and receiver-coupling result.
- Revalidate the nested transport input and certificate through their owner.
- Derive the band/time ID as SHA-256 of the V1 domain, one zero separator, and
  canonical JSON `[band,time_basis_id]`; reject a zero time basis.
- Require exact band/time equality with both nested transport records.
- Rebuild every conditional bulk query from the stored transport step input
  and event. Callers never supply optical depth, transfer, opacity, or factors.

## Composition

For receiver-face proof at selected step `k`, steps before `k` contribute both
depth endpoints and step `k` contributes zero through its upper endpoint. A
mandatory opaque prefix proves exact zero transfer; selected-partial opacity or
uncertain earlier evidence widens to `[0, upper]`.

For start-inside proof, step zero is identity. Later start-inside outcomes use
zero through the sum of prior upper depths; prior opacity widens to `[0,1]`.
Zero and unresolved coupling retain their exact measure buckets and emit no
transfer. A projected zero lower factor is underflow, never opacity.

## Ceilings and rollback

V1 admits at most 64 steps, 128 endpoint additions, 118 raw Q64.64 bits,
128 MiB input, 256 KiB result, and 192 MiB aggregate live canonical bytes.
Every codec is strict reconstruct-and-compare. Rollback deletes this sibling
and the additive bulk evaluation surface; no existing V1 data is migrated.
