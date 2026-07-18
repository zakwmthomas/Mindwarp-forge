# G1 C3 Generic Physical Probe and Swept Passage Mathematical Design Audit

Date: 2026-07-16

Status: **minimum mathematical candidate selected; counterexample oracle named;
no schema or implementation authorized.**

## Scope and research basis

This audit asks only what observer-independent evidence C3 needs to determine
whether one declared local envelope, translated without rotation, encounters a
bounded occupancy volume. It does not define an organism, vehicle, world shape,
route, navigation system, terrain or runtime collision response.

Configuration-space motion planning represents collision-free translations by
expanding obstacles with the negated moving body. This is the relevant
mathematical reduction, not a runtime-engine convention. The original GJK work
establishes convex-distance queries through support mappings, while later
continuous-collision work shows that first-contact computation is a separate
problem from discrete overlap. Sources: Gilbert, Johnson and Keerthi,
`10.1109/56.2083`; Agarwal et al., *On Translational Motion Planning of a Convex
Polyhedron in 3-Space*, `10.1137/S0097539794266602`; and Redon et al., *Fast
Continuous Collision Detection between Rigid Bodies*,
`10.1111/1467-8659.t01-1-00587`.

## Envelope comparison

| Candidate | Exact fixed-translation fit | Benefit | Failure pressure | Disposition |
|---|---|---|---|---|
| sphere | segment against rounded-box configuration obstacles requires squared-distance/root reasoning | isotropic and rotation-free | poor elongated-body fit; irrational boundary events complicate the existing rational witness contract; convenience could be mistaken for world geometry | reject for v1; retain as later typed envelope |
| capsule | similarly needs cylindrical/cap distance events | useful for some upright bodies | silently imports a preferred axis and body-like semantics; more event classes than C3 currently needs | reject for v1; retain for a later consumer-backed extension |
| axis-aligned box | occupied cell inflated by probe half-extents remains an axis-aligned box; existing exact rational slabs apply | smallest exact extension of the verified substrate; anisotropic sizes without roots | world-axis bias and no rotation; unsuitable as a universal body model | **select as the minimum v1 mathematical candidate** |
| bounded convex polytope | configuration obstacles and support mappings are general | future extensibility | facet/edge axes, termination, arithmetic width and canonical contact manifold are substantially larger than the current gap | defer until a real consumer falsifies the box boundary |

The selected box is a local query envelope only. It does not imply cubic
creatures, box collision in production, rectilinear terrain, or any world or
planet shape.

## Selected mathematical candidate

The probe is a closed axis-aligned box with strictly positive Q32.32 half
extents and a Q32.32 reference point. Motion is one closed reference-point
segment. V1 accepts translation only and rejects rotation explicitly; it never
samples or approximates rotation.

For each reconstructed cell box `C` and probe box `P` centred at the origin,
the forbidden reference-point region is the exact configuration obstacle
`C (+) (-P)`. Because both are axis-aligned boxes, this is `C` expanded by the
probe half extents on all axes. The existing checked rational slab method can
then intersect the reference-point segment with that expanded box. No floating
epsilon, square root, iterative convergence or target-native layout is needed.

This construction is a correctness oracle, not an optimized broad phase. The
same 65,536-cell ceiling is the initial cost ceiling; implementation readiness
must measure the larger witness set before adopting it.

## Typed semantics

Geometry and mechanics remain separate:

- `clear_sweep`: no declared mechanically blocking configuration obstacle is
  touched over the closed motion interval.
- `initial_overlap`: the probe begins inside one or more blocking configuration
  obstacles; v1 reports exact witnesses and does not invent a depenetration
  direction.
- `first_contact`: the earliest exact rational parameter and the unordered set
  of cell/axis contact witnesses at that parameter.
- `stationary_clear` and `stationary_overlap`: zero-length motion is classified
  directly, never smuggled through moving-path semantics.
- `outer_domain_contact` and `unavailable_evidence`: remain distinct from known
  material contact.
- `unsupported_motion`: rotation or malformed envelope input is rejected rather
  than approximated.

Positive-length overlap after first contact is penetration opportunity, not a
physics response. Point-only tangent contact is retained as contact, not
penetration. Simultaneous face, edge and corner axes are an unordered set.

## Mechanical authority boundary

Occupancy phase cannot decide blocking, support, friction or fluid response. A
future query must consume an explicit mechanical interaction profile whose
entries are bound to exact substance or face evidence. Unknown profile entries
produce `interaction_model_required`; they do not default from `solid`,
`liquid`, `gas`, vacuum or unavailable state.

The result may report exact geometric contact, blocking-evidence provenance and
separately declared support opportunity. It may not emit a universal
traversability score, route cost, gait, destination fitness or organism claim.

## Required counterexample oracle

Name: **expanded-cell exact-rational swept-box oracle**.

Before schema readiness it must exhaustively compare the selected slab result
against an independently enumerated exact configuration-space oracle over a
small integer lattice. Required hostile families are:

1. one-cell-thick barriers crossed between endpoints;
2. exact tangent face, edge and corner events;
3. initial overlap with one and several cells;
4. zero-length clear, tangent and overlap cases;
5. simultaneous cells and axes with no semantic ordering;
6. exact reversal mapping `t -> 1-t`;
7. closed outer-domain contact and unavailable boundary evidence;
8. forged cell, axis, time and interaction witnesses;
9. geometrically equivalent coarse/fine occupied unions, distinguishing stable
   first-contact geometry from intentionally different cell provenance; and
10. a narrow opening passed by a point but rejected by the extended box.

The oracle must fail if centre-line evidence is accepted as swept clearance, if
point contact becomes penetration, if phase invents mechanics, or if equivalent
geometry changes the first-contact parameter.

## Failure and portability result

The minimum candidate uses checked integers, reduced rationals and canonical
field ordering, so one semantic core remains viable for PC and mobile. Platform
adapters may accelerate the query only after reproducing the same vectors.
There is no current reason for a platform fork.

The existing point-path substrate remains unchanged. Any future swept-box
module must be additive and removable without database or identity migration.

## Stop condition

This audit has selected the fixed-orientation translated AABB and named its
counterexample oracle. Stop here. Do not add a schema, crate, implementation,
mechanical coefficient catalogue, rotation, organism binding, navigation,
terrain, sphere, planet, biome presentation, runtime behavior or C3 closure
without a separately refreshed implementation-readiness package and authority.
