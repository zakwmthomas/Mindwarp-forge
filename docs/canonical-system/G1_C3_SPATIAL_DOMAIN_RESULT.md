# G1 C3 spatial-domain v1 result

Date: 2026-07-15

Status: **bounded capability-free prototype evidence; C3 remains active**.

The owner-approved `spatial-domain` crate implements the finite rectified
sampling seam selected in `G1_C3_SPATIAL_DOMAIN_DESIGN.md`.

`SpatialDomainInput` binds exact logical-world and reconstruction identity,
the fixed Forge Q32.32 Cartesian frame, cell-centre origin, positive per-axis
steps, finite counts, shared-edge four-neighbour adjacency and bounded-absent
edges. The compiler proves total proof-cell bounds and the furthest coordinate
before issuing a domain identity.

Cells are never trusted as caller facts. Forge reconstructs every cell's
coordinate, identity and ordered neighbour list from the exact validated
domain and index. Cell serialization now requires that exact domain, so a
forged coordinate, wrap edge, identity or authority claim cannot even produce
canonical bytes through the public API.

Nine focused spatial-domain tests prove:

- strict descriptor, domain, cell and cell-index replay;
- exact negative and positive Q32.32 coordinate mapping;
- ordered corner, edge, interior and one-cell neighbours with no wrap;
- domain and cell re-keying when descriptor content changes;
- missing identity, zero count/step and proof-ceiling rejection;
- furthest-coordinate overflow and out-of-bounds rejection;
- forged coordinate, neighbour, identity and authority rejection; and
- unknown-field, whitespace and schema-drift rejection.

Nine regional-environment tests also pass. The new bounded helper reconstructs
the cell first, requires the exact reconstruction identity, substitutes only
the reconstructed sample coordinate, and then uses the unchanged regional
compiler and validator. Existing direct point sampling remains valid.

The 36-module boundary and generated front-door gates pass. `spatial-domain`
can depend only on `field-basis`; capability imports, protected Kernel access,
desktop/UI access, filesystem/process access and networking are forbidden.

This result does not represent a spherical planet or global surface. It adds
no wrapping, projection, terrain, storage layout, physical-region membership,
biome semantics, visibility, traversability, runtime, engine, approval,
promotion or persistence authority. A future globe-aware domain requires a new
schema and migration evidence; it cannot silently reinterpret v1 cells.

The next dependency-safe C3 step is a separate physical partition-policy
design audit over this bounded domain. It must compare a no-partition baseline,
exact equality, explicit per-dimension tolerances and content-authored recipes,
and must stop before named ecology or implementation without its own readiness
and authorization gates.

