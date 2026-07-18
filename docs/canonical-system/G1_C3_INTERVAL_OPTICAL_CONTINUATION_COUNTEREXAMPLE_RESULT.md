# G1 / C3 interval optical continuation counterexample result

Date: 2026-07-16

Status: **exact falsifiers passed; conservative axis boxes survive for geometry;
interval-incident interface arithmetic remains unproved; no code authorized.**

## Receipt

`tools/prove-g1-c3-interval-optical-continuation.py` ran twice with byte-identical
output using Python `Fraction`. Its canonical vector checksum is
`91cc00cbcb97e9a8b8157edbaacc21fd05826bf0213d1515d126c702b9b5d6fd`.

The script is explicitly a counterexample oracle, not a continuous-domain
solver. It proves that several shortcuts are unsound and fixes required typed
outcomes. It does not prove implementation readiness or adequate real-world
utility.

## Exact findings

### One-unit face reversal

From the centre of a unit square, directions
`(2^62 + 1, 2^62)` and `(2^62, 2^62 + 1)` differ by one Q1.62 raw unit per
component but hit different faces. Their midpoint hits the corner. Therefore a
one-target-unit direction enclosure is not automatically topologically unique,
and midpoint selection is permanently invalid.

### Correlation erasure

Two correlated point-direction states each reach `x_max` at `t = 1/2`. Mixing
the point from one with the direction from the other—an impossible combination
introduced by an independent axis box—reaches `y_max` at `t = 1/2`.

This does not make axis boxes unsound if the algorithm quantifies over the
entire box and returns ambiguity whenever face-time intervals overlap. It does
make them intentionally conservative: they may return `ambiguous_next_face`
when the correlated physical set has one face. No corner, midpoint or selected
representative may override that result.

### Critical branch ambiguity

For `eta_i = 3/2` and `eta_t = 1`, incident integer vectors with normal/tangent
components `(3,2)` and `(2,2)` lie on opposite sides of the exact TIR test. An
incident enclosure containing both therefore cannot emit one known transmitted
or TIR branch. It requires `ambiguous_interface_branch`, distinct from local
numerical nonconvergence and unavailable interaction evidence.

### Progress and re-entry

A near-parallel component of `2^-100` retains exact positive progress and must
not be rounded to zero. Conversely, a ray beginning on its prior face and
pointing outward has exact face time zero. It terminates `no_forward_progress`;
an epsilon nudge would fabricate a new state.

## Representation disposition

For v1 geometry, independent per-axis dyadic boxes remain the smallest bounded
candidate because they have fixed state size and can be sound under universal
certification:

- calculate outward time intervals for every admissible face;
- admit one face only if its upper time is strictly below every competitor's
  lower time and its lower time is strictly positive;
- otherwise return `ambiguous_next_face` or `no_forward_progress`;
- propagate point boxes with outward interval multiply/add; and
- never narrow from unit-vector, Snell or point-direction correlation unless a
  separately replayable certificate proves that narrowing.

Affine forms, zonotopes and exact symbolic states may reduce false ambiguity,
but their correlation symbols or algebraic expressions grow under repeated
division and refraction. They are not selected without a separate bounded-cost
proof. Finite representative chains remain rejected as canonical truth.

## Remaining prerequisite

Geometry alone is insufficient. The next bounded package is an
**interval-incident smooth-dielectric mathematical and oracle audit**. It must
evaluate a full incident component box against one exact face/model and decide
whether fixed 512-bit outward arithmetic can produce:

- known transmitted power/direction boxes containing every admitted input;
- exact all-TIR termination;
- `ambiguous_interface_branch` when the box spans TIR and transmission;
- typed nonconvergence at a hard precision/work ceiling; and
- a measured ambiguity/widening rate over normal, critical, grazing and
  repeated-event hostile portfolios.

The existing exact-path local interface v1 remains unchanged. A later
successful candidate must be versioned and prove all v1 vectors byte-identical
before any modification of its owning crate is considered.

## Authority and nonclaims

No Rust crate, schema, dependency or production algorithm is authorized. The
result does not prove an end-to-end optical path, bulk attenuation, perception,
rendering, gameplay visibility, organism meaning, planet, terrain, biome,
runtime behavior, promotion or C3 closure.
