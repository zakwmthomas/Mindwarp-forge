# G1 / C3 post-receiver-arrival consumer reassessment

Date: 2026-07-16

Status: **receiver arrival and dimensionless lane transfer are verified as
separate proofs; neither provides the phase-space measure needed to turn one
lane into source-to-receiver power. A code-free optical lane coupling-measure
mathematical/counterexample audit is the smallest next seam.**

## Facts now available

The Forge can now replay, independently:

- the ordered physical/interface lineage of one same-band optical lane;
- its cumulative dimensionless transfer enclosure; and
- whether the exact followed ray reaches the strict interior of one bounded
  receiver volume before the owning physical face.

These facts establish opportunity, loss and arrival ordering. They do not
establish how much source emission the lane represents or how much of that
measure a receiver accepts.

## Candidate comparison

| Candidate | Missing independent facts | Counterexample | Disposition |
|---|---|---|---|
| source emission magnitude only | physical quantity, band integration, time basis, angular measure and calibration | assigning watts to one mathematical ray changes received power when the ray discretization is refined | defer until lane measure is explicit |
| inverse-square spreading on the followed lane | source distance, homogeneous free-space assumption and receiver solid angle | refracted, reflected and focused paths do not preserve a naive source-to-endpoint inverse-square factor | reject as a universal lane rule |
| optical lane coupling measure | source angular/area measure, neighboring-ray differential or bounded solid-angle footprint, and receiver acceptance measure | two identical central rays can have different accepted power because their surrounding bundles focus differently | select for mathematical/counterexample audit only |
| receiver aperture and detector response | aperture geometry/orientation, spectral response, integration time, noise and threshold | the present positive-volume AABB can be reached while a directional aperture rejects the arriving direction | defer; geometry arrival is not acceptance |
| combined source-to-detector record | all of the above plus units and calibration | a plausible scalar can be fabricated while every independent missing fact remains unconstrained | reject as scope-collapsed |

## Why the central ray is insufficient

The cumulative transfer owner multiplies local dimensionless factors along one
validated lineage. The receiver owner proves strict geometric intersection for
that same exact ray. Neither owner binds a nonzero area or solid-angle element.
Consequently a scalar source power assigned directly to the lane has no
discretization-invariant meaning: splitting one represented angular region into
two identical central-ray samples would double power unless a measure and
quadrature rule were already bound.

The same missing fact prevents a universal inverse-square patch. Free-space
point-source spreading is one special coupling geometry, not a local factor
that survives arbitrary dielectric refraction, total internal reflection,
focusing or a finite oriented receiver.

## Selected bounded action

Run only a mathematical design and independent counterexample/oracle audit for
an optical lane coupling measure. Compare at minimum:

- a source solid-angle cell carried with one lineage;
- a bounded differential-ray or footprint/Jacobian representation; and
- direct receiver solid-angle coupling for the homogeneous free-space special
  case.

The audit must freeze quantity dimensions, source area/angular basis, band and
time basis, receiver acceptance relation, conservation/étendue limits,
refraction/focusing counterexamples, discretization refinement invariance,
cost ceilings and typed unsupported evidence.

Do not add a crate, dependency, schema, test or production source. Do not infer
source emission, received power, aperture acceptance, detector response,
visibility, darkness, perception, rendering, gameplay line of sight, runtime,
promotion or C3 closure.

