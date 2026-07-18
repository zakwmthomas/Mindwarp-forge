# G1 C3 Signal Potential Result

## Result

The universal caller-authored `transmission_permille` multiplier has been
removed from derived-world inputs. `SignalPotential` now contains only channel
and bounded `baseline_strength_permille`. Visible radiance receives the exact
regional exposure modifier; nonvisual channels retain their baseline unless a
validated required medium is absent, which fails compilation.

Legacy transmission fields fail the strict input codec. The output limitation
states that potentials do not prove propagation, distance attenuation,
measurement or biological detectability.

## Root-cause repair

The previous contract said caller-authored transmission could not bypass
validated media, while the implementation accepted and multiplied exactly
such a value across optical, acoustic, chemical, substrate, electric and
magnetic channels. Removing the universal field resolves the contradiction at
the input boundary instead of tuning individual fixtures.

## Bounded proof

- Eleven derived-world tests cover strict replay, legacy-field rejection,
  baseline-potential semantics, regional visible causality, unrelated-channel
  control, medium contradictions, canonical ordering and hostile state.
- Twenty-one addressable, sensory-support, opportunity-graph and macro-lineage
  tests pass with the new exact input.
- All 41 desktop tests pass.
- The disposable portfolio remains green at 125/125 range cases, 32/32
  reconstruction identities, 31 exposures and 30 palettes across 32
  coordinates.
- The complete repository gate passes governance, canonical coherence, all 35
  module fronts, UI build and workspace tests. Its ordinary final desktop
  build reaches only the known live-executable lock; an isolated desktop build
  with warnings denied passes.

## Evidence boundary

NIST distinguishes source intensity/radiance from irradiance or radiance at a
receiving surface or detector. NOAA treats acoustic transmission loss as a
separate propagation result caused by spreading or attenuation and informed
by environmental evidence. Those distinctions support removing one universal
caller multiplier; they do not validate the procedural potential values.

- https://www.nist.gov/publications/radiometry-and-photometry-review-vision-optics
- https://www.fisheries.noaa.gov/insight/technical-assistance

## Retained limitations

No emitted power, propagation distance, geometry, frequency response,
attenuation, scattering, physical visibility range, sensory physiology,
biological detectability or runtime behavior is implemented.
