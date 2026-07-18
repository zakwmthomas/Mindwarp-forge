# G1 C3 Physical Path Substrate Result

Date: 2026-07-15

Status: **bounded capability-free v1 reference implemented and verified;
prototype only; C3 remains executing.**

## Implemented result

The owner-authorized `physical-path-substrate` crate implements the exact
readiness boundary without expanding it. It owns a separate cubic Q32.32 3D
frame, canonical sparse column runs, unavailable/vacuum/gas/liquid/solid cell
evidence, provenance and occupancy identities, strict volume/query/witness
codecs and exhaustive exact closed-cell path traversal.

The implementation deliberately uses the correctness-first `O(N)` slab oracle
for at most 65,536 cells. No optimized DDA is present. Every intersected cell
appears once with reduced exact entry/exit parameters and either `interval` or
`point`. Simultaneous crossings have no semantic axis order. Closed outer
endpoints create no phantom cells. Stationary face/edge/vertex queries retain
the full `[0/1,1/1]` parameter preimage but are point evidence, structurally
preserving the disposable proof's failure correction.

## Focused proof

`cargo test -p physical-path-substrate` passes 13 tests covering:

- strict recipe, volume, query and witness replay;
- unavailable, vacuum and substance-bound phase separation;
- nondefault, sorted, non-overlapping, maximal sparse runs;
- cell-count and coordinate overflow plus outside-endpoint rejection;
- face, edge, vertex, internal endpoint and closed outer-boundary cases;
- eight-point stationary internal-vertex evidence;
- exact reversal mapping;
- thin-barrier positive crossing versus point-only contact;
- duplicate and phantom cell rejection;
- provenance-bound volume versus semantic occupancy identity;
- forged results, noncanonical bytes and unreduced-rational rejection;
- evidence-only authority and consumer nonclaims; and
- maximum-ceiling cost.

The debug maximum-ceiling fixture reconstructed a `256 x 256 x 1` volume:

| Measurement | Receipt |
|---|---:|
| reconstructed cells | 65,536 |
| exact witness records | 766 |
| recipe canonical bytes | 918 |
| volume canonical bytes | 785 |
| witness canonical bytes | 220,020 |
| measured test body | 638 ms |

These are same-machine debug-test observations, not production performance
claims. The test fails if the fixture exceeds 30 seconds. A future change must
retain or deliberately revise the proof ceiling with new cost evidence.

## Integration and ownership

The crate has no local dependencies and governance forbids Forge Kernel,
desktop/UI, network, filesystem and process imports. Its generated `MODULE.md`
is backed by the module-context registry. The C3 verifier requires the contract,
result, source tokens, failure fixtures and focused test.

The existing 2D `SpatialDomain` and physical-region partition remain unchanged
and are not dependencies. Channel consumers still own attenuation and transfer;
probe consumers own collision, clearance and motion validity; C6 owns organism
meaning; later biome and presentation consumers must derive deterministic
ecotones from continuous causal fields. A categorical region or occupancy cell
cannot directly paint a visible seam, while a sharp physical interface must not
be blurred away.

## Limits and next route

This result proves a bounded engine-neutral prototype, not scientific material
physics, production geometry, a sphere or planet, a terrain engine, runtime
voxels, storage/streaming, a navigation mesh, propagation, visibility,
traversability, biome generation, organism binding, approval or promotion.

C3 remains executing. The next safe action is a post-substrate C3 consumer
reassessment: determine the smallest typed propagation/visibility and generic
probe/traversability contracts that consume this evidence without duplicating
occupancy truth or importing organism/runtime policy. No consumer implementation
or C3 closure is authorized by this result.

