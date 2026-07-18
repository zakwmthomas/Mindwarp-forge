# G1 / C3 optical lane coupling-measure mathematical design audit

Date: 2026-07-16

Status: **no implementation candidate yet; one exact central lineage cannot
carry a nonzero discretization-invariant phase-space measure. A finite
correlated boundary-lineage counterexample/oracle is the smallest next proof.**

## Source basis

NIST defines radiance as radiant flux per unit projected area per unit solid
angle and emphasizes that it is a differential beam quantity whose direction
is part of the definition:

- https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=104550

NIST's SI guidance gives the free-space solid-angle special case as spherical
area divided by radius squared:

- https://www.nist.gov/pml/special-publication-330/sp-330-section-5

Igehy's original ray-differential work tracks derivatives of a ray to estimate
the distance to neighboring rays and hence a surface footprint. It supplies a
useful local geometric representation, not by itself a radiometric source or
power measure:

- https://doi.org/10.1145/311535.311555
- https://graphics.stanford.edu/papers/trd/

The passive-system brightness/étendue boundary is retained as a design shield:
lossless passive mappings preserve the appropriate phase-space measure and
cannot increase radiance merely by concentration:

- https://doi.org/10.1038/s41598-026-42509-9

The design conclusion below is an inference from those sources and the current
Forge contracts: the existing central lineage, dimensionless transfer and AABB
arrival proofs do not bind projected source area, solid angle or neighboring
ray evolution.

## Required quantity boundary

A future coupling owner must not begin with watts assigned to one ray. Its
minimum semantic subject is a same-band, same-time-basis phase-space cell:

`dG = n^2 * dA_perpendicular * dOmega`

with an associated radiance or radiant-intensity measure supplied by a later
source owner. `dA_perpendicular`, `dOmega`, refractive-index convention, band
integration and time basis must be explicit. A zero-area or zero-solid-angle
subject may retain geometric evidence but represents zero finite étendue.

The cumulative transfer remains a separate dimensionless multiplier. Receiver
arrival of the central ray remains a separate ordering fact. Neither may be
silently reinterpreted as accepted power.

## Candidate audit

| Candidate | Strength | Fatal gap | Disposition |
|---|---|---|---|
| declared scalar lane weight | cheap and deterministic | refinement can duplicate or erase total measure; no geometric coverage proof | reject |
| central ray plus two angular widths | represents a source cone | no source-area basis, correlation or receiver footprint; rotation can change the represented region | reject |
| first-order ray differential | compact local footprint/Jacobian approximation | current lineage has no derivative state; discontinuities, branch changes, caustics and interval boxes need typed failure | retain only as an oracle comparator |
| finite correlated boundary lineages | reuses exact owner replay and can prove common topology over a bounded cell | combinatorial cost and interior escape cannot be inferred from corners alone | select for counterexample/oracle audit |
| free-space source/receiver solid angle | exact semantic special case | does not generalize across refractive/interface lineage and needs transcendental enclosure for finite shapes | retain as oracle baseline only |

## Selected mathematical subject

The next oracle should compare one central lineage with a bounded correlated
phase-space cell whose boundary is represented by independently replayed
lineages. It must not assume corner sufficiency. The candidate receipt should
contain, at minimum:

- one source surface parameter cell and one source direction parameter cell;
- a declared same-band and time basis;
- central and boundary lineage identities;
- exact topology-coherence evidence: equal ordered cell/interface/terminal
  families for every admitted boundary lineage;
- source projected-area and solid-angle enclosures;
- receiver-plane footprint/acceptance enclosures only when all required rays
  reach the same receiver ordering surface;
- explicit refinement-parent identity so children partition rather than
  duplicate the parent's measure; and
- typed `unsupported_topology_change`, `unsupported_caustic_or_fold`,
  `unsupported_conditional_lineage`, `partial_receiver_coverage` and
  `zero_measure` outcomes.

## Counterexamples the oracle must retain

1. Two scenes with the same central lineage and transfer but different
   neighboring-ray focusing produce different receiver footprints.
2. A cone whose central ray arrives while every boundary ray misses is not
   full receiver acceptance.
3. A cone whose corners arrive while an interior ray crosses a different
   interface defeats corner-only certification.
4. Refining one phase-space cell into four children must preserve the union
   measure rather than multiply it by four.
5. A planar homogeneous free-space portfolio must reduce to the NIST
   solid-angle relationship within an outward enclosure.
6. Lossless passive concentration must not create radiance above its admitted
   source enclosure; bulk/interface loss may only reduce carried flux.
7. A branch/topology change, TIR boundary or caustic/fold must be typed
   unsupported rather than assigned a favourable footprint.

## Cost and stop boundary

The oracle should start with exact rational affine free-space portfolios, then
planar same-branch refractive portfolios, before any transcendental solid-angle
enclosure. It should compare finite boundary bundles against a high-precision
reference and search interior samples specifically to disprove corner-only
claims. No production precision, object cap or schema is frozen until that
counterexample search yields a surviving representation.

Do not add a crate, dependency, schema, test or production source. Do not
define source emission, received power, aperture acceptance, detector response,
visibility, darkness, perception, rendering, gameplay line of sight, runtime,
promotion or C3 closure.

