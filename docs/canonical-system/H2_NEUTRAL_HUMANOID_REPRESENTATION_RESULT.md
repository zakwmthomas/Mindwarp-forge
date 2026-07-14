# H2 neutral humanoid representation result

Status: **verified structural proof candidate**. This closes H2 only. It does
not approve a humanoid, generate an asset, or authorize H3 output.

## Reconciliation result

The pre-H2 layers were complementary but not aligned:

- `representation-contract` already supplied bounded canonical proof and
  lineage machinery, but its articulation records were intentionally general
  and fingerprint-based rather than a concrete humanoid schema.
- `reference-viewport` v3 supplied 17 named points, 16 directed links, a
  pelvis-rooted hierarchy, rest geometry, and two frames, but its coordinate
  units and semantic role mapping were implicit.
- H1 supplied authority-safe recovered and synthetic reference intake, but was
  not bound into a representation artifact.

H2 adds one small `neutral_humanoid` module inside `representation-contract`.
It derives a typed profile from the Forge-owned v3 scene and binds the H1 suite
by fingerprint. No recovered JSON is parsed, copied, generated from, or treated
as canonical authority.

## Exact retained evidence

- profile: `forge-neutral-humanoid-structural-v1`
- profile SHA-256 domain fingerprint:
  `c44adba610e2d70361d72cd9f78d1c3b7f56041a5574ef2f795570a72763d6e3`
- source scene fingerprint:
  `f94ebe29d2d8a5b9abfcd906412db4ad0da0a2e8e0947de7a422f51274ddac82`
- H1 suite fingerprint:
  `1a4e25e81bc39327bc95975054846496b88c4510d378c0bef5f3ea1a5281939a`
- topology: 17 joints, 16 links, two frames; hard H2 ceilings are 64 joints,
  96 links, and eight frames.
- coordinates: right-handed fixture convention; +x anatomical-left, +y up,
  +z forward; pelvis origin; lengths explicitly non-metric.
- rest/bind boundary: frame zero equals structural rest positions. No inverse
  bind matrices, skin weights, surface, deformation, or engine mapping exists.

## Adversarial proof

The focused crate now has 23 passing tests: the inherited 17 P7a tests plus six
H2 tests covering deterministic canonical bytes and fixed fingerprint, missing
role, hierarchy cycle, rest drift, unknown field and noncanonical encoding,
H1-suite drift, and structural overclaim. Validation also rejects identity or
role duplication, unknown joints, multiple parents, source fingerprint drift,
coordinate drift, topology exhaustion, and authority changes.

## Remaining boundary

The profile is deliberately a wire-structure contract. Surface topology,
meshing, skinning, inverse-bind transforms, deformation, animation quality,
visual convergence, engine compatibility, external reference acquisition, and
production promotion remain unproved. H3 may construct only deterministic,
engine-neutral candidate data from declared Forge inputs and must retain these
negative boundaries.
