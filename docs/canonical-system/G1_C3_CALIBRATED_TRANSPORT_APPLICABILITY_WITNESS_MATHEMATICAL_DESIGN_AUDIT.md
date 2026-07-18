# G1 / C3 calibrated transport-applicability witness mathematical design audit

Date: 2026-07-18

Status: **the code-free mathematical design survives. Schema and implementation
remain blocked. A future capability-free applicability sibling may certify that
an existing dimensionless-transfer enclosure applies to one exactly replayed
physical subject; it may not compute received energy or manufacture missing
calibration evidence.**

## Decision and authority

The owner authorized primary-evidence acquisition and mathematical design after
the schema-gap audit. This record freezes the smallest sound theorem, identity
graph, typed outcomes, falsifiers and structural resource limits. It adds no
crate, contract schema, dependency, production test, production source,
downstream consumer or normative coefficient catalogue. Existing V1 owners,
bytes, identities and behavior remain unchanged.

The sibling remains capability-free. It may attest applicability only. It does
not own source allocation, coefficient discovery, received energy, detector
response, visibility, runtime, promotion or C3 closure.

## What primary standards establish

The [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure) and
[BIPM metre definition](https://www.bipm.org/en/si-base-units/metre) establish
the SI length basis. They do not establish how many metres one Forge coordinate
unit represents. That project-specific mapping requires evidence.

The JCGM VIM definitions of
[calibration](https://jcgm.bipm.org/vim/en/2.39.html) and
[metrological traceability](https://jcgm.bipm.org/vim/en/2.41.html) require a
documented relation and calibration hierarchy; an opaque identity or shared
reconstruction ID is not calibration provenance. The VIM distinguishes
[measurement uncertainty](https://jcgm.bipm.org/vim/en/2.26.html) and a
[coverage interval](https://jcgm.bipm.org/vim/en/2.36.html). A probabilistic
coverage statement cannot silently become a deterministic hard enclosure.

The IUPAC definitions of
[absorption coefficient](https://goldbook.iupac.org/terms/view/A00037) and the
[Beer-Lambert law](https://goldbook.iupac.org/terms/view/B00626) support an
inverse-length coefficient and dimensionless exponential attenuation relation.
Absorption alone is not automatically total extinction: IUPAC distinguishes the
[extinction/scattering relation](https://goldbook.iupac.org/terms/view/S05490).
The evidence must therefore declare whether it bounds absorption, scattering or
total extinction and must match the model actually evaluated.

NIST optical-radiometry guidance treats response and radiation as spectral
quantities rather than proving that an RGB label is a wavelength-wide bound
([NIST optical radiometry](https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=104704),
[NIST Technical Note 1889v1](https://nvlpubs.nist.gov/nistpubs/TechnicalNotes/NIST.TN.1889v1.pdf)).
The coefficient evidence must cover the complete calibrated wavelength, time,
spatial, path and environmental domain. A midpoint, sample or average is
insufficient unless a conservative enclosure theorem lifts it to that domain.

[NIST DLMF interval-arithmetic guidance](https://dlmf.nist.gov/3.1) supports
outward conservative interval evaluation. These sources define dimensional and
enclosure obligations; none supplies the missing Forge scale, coefficient
dataset, provenance, environmental validity or physical truth.

## Exact dimensional theorem

Let `x` be abstract coordinate length and let the spatial calibration provide an
exact positive mapping `s = k*x`, where `k` is metres per coordinate unit. For a
physical extinction coefficient `alpha_m` in inverse metres, the equivalent
per-coordinate coefficient is:

`alpha_coordinate = k * alpha_m`.

The optical depth is invariant:

`tau = integral(alpha_m ds) = integral(alpha_coordinate dx)`.

Optical depth is dimensionless. If hard evidence proves
`tau in [tau_lower, tau_upper]`, monotonicity of the exponential proves:

`T in [exp(-tau_upper), exp(-tau_lower)]`.

The applicability theorem does not replace the existing bulk calculation. For
every source phase-space point, wavelength/time point and traversed path segment
in the declared domain, it must prove that the physical optical-depth enclosure
is contained in the already recorded bulk optical-depth enclosure. The existing
dimensionless-transfer enclosure then contains the physical transfer.

No pointwise hard coefficient bound over the complete domain means typed
unresolved applicability. A coverage interval, probability, confidence claim,
midpoint or finite sample is not a hard deterministic enclosure. Probabilistic
evidence may be retained as a separate typed statement but cannot authorize a
`certified_everywhere` result.

## Minimum exact replay graph

The future applicability subject must replay this exact join:

`calibrated basis -> source distribution -> selected frontier allocation/cell`

joined to:

`transport input -> certificate -> selected step/path -> physical recipe/volume
-> bulk profile -> dimensionless-transfer result`.

It also requires two genuinely new immutable evidence nodes.

### Spatial calibration evidence

- exact coordinate-frame and physical-recipe identity;
- positive reduced exact rational metres per coordinate unit;
- declared spatial and environmental validity domain;
- nonzero provenance and positive revision; and
- hard deterministic enclosure semantics, or a typed non-authorizing
  probabilistic statement.

### Coefficient applicability evidence

- exact bulk-profile identity and revision;
- exact calibrated-basis identity, selected band and derived band/time identity;
- coefficient kind: absorption, scattering or total extinction;
- conservative SI inverse-metre enclosure for every relevant segment/substance;
- complete wavelength, time, spatial, path and environmental validity domain;
- nonzero provenance and positive revision; and
- hard deterministic enclosure semantics distinct from coverage probability.

The derived applicability-subject identity commits every upstream identity and
both evidence identities. The applicability-result identity commits the subject,
disposition and exact enclosure relation. No node commits a downstream result
used to derive itself, and there is no active-version or supersession registry.

Mandatory joins include exact allocation cell equality with the transfer cell;
calibrated basis/band-time equality across source, evidence and transfer; exact
certificate, selected-step, profile and physical-volume replay; and the existing
recipe reconstruction relations. Bulk-profile top-level scope remains an
independently opaque field because current validation deliberately does not
equate it with embedded recipe scope. Opaque-ID proximity is never scientific
evidence.

## Whole-cell source theorem

Source uniformity is not required when a transfer enclosure holds everywhere.
For any nonnegative source distribution with total energy `E`, pointwise
`T in [L,U]` over the complete source domain entails an integrated transported
energy bound `[E*L,E*U]`. The future sibling may attest the pointwise
applicability premise but must not emit that energy product.

Aggregate joules and equal measure do not apportion energy. Therefore
`certified_everywhere` requires accepted measure equal the complete source-cell
measure and exact zero and unresolved measures both equal zero. Exact whole-cell
zero coupling may be certified separately. Any mixed accepted/zero/unresolved
partition is `conservatively_unresolved` unless a later joint source/coupling
integration proof binds the source distribution to those partitions.

## Typed outcomes

- `certified_everywhere_finite`: finite enclosure, with the lower endpoint
  distinguished as positive or underflow-zero;
- `certified_everywhere_opaque`: exact zero because mandatory opacity applies;
- `certified_everywhere_zero_coupling`: exact zero because geometric coupling is
  zero, not because the medium is opaque;
- `certified_everywhere_vacuum_identity`: exact identity when established over
  the complete domain;
- `conservatively_unresolved`: mixed measure, incomplete coefficient domain,
  optical-depth noncontainment, path mismatch or pointwise theorem failure; and
- `unavailable_evidence`: required spatial/coefficient provenance is absent.

Underflow remains finite and must not become opacity. Unavailable evidence,
probabilistic coverage and deterministic failure remain distinguishable.

## Falsifying counterexamples

1. **Scale substitution:** replay identical current bytes under one metre per
   coordinate unit and ten metres per coordinate unit. Without new calibration
   evidence, both fit the bytes but produce incompatible SI interpretations.
2. **Spectral/time substitution:** replay one RGB bulk profile under two
   incompatible calibrated wavelength/time cells. Matching local IDs do not
   prove coefficient validity across either cell.
3. **Opaque subject mismatch:** vary bulk top-level scope while its embedded
   recipe and required reconstruction relations remain valid. Current acceptance
   is not a defect; it proves scope proximity is not an applicability theorem.
4. **Source concentration:** keep total joules and coupling measures fixed, then
   concentrate all energy once in accepted measure and once in zero/unresolved
   measure. Aggregate equality cannot distinguish the physical results.
5. **Endpoint escape:** make a sampled or midpoint coefficient match while a
   wavelength/time endpoint exceeds the claimed enclosure.
6. **Correction replay:** correct scale, coefficient domain or provenance but
   replay the old applicability identity. Immutable identity must reject it.

The design survives only because each counterexample becomes either a failed
exact join or a typed unresolved outcome; none is papered over by an opaque ID.

## Correction and resource envelope

Evidence is immutable. A corrected scale, coefficient enclosure, validity
domain, provenance or revision derives new evidence, applicability-subject and
result identities. Old evidence is not rewritten or reinterpreted.

For a future disposable readiness spike, freeze one source distribution, one
selected allocation, one basis/band/time cell, one dimensionless-transfer result
and its exact replay graph; at most 64 transport steps and 64 coefficient
segments; at most one scale enclosure and one coefficient enclosure per segment;
reduced unsigned `u128` rational endpoints; checked interval products with a
proved sub-512-bit live-arithmetic shield; and no unbounded collection, catalogue
lookup, adaptive integration, wavelength sampling or coefficient discovery.

Do not invent byte caps in this design audit. A disposable canonical-byte spike
must measure the complete nested replay and stop unless it fits the existing
192 MiB aggregate-live ceiling. These are structural readiness constraints, not
authorization to create a schema.

## Result and next boundary

The capability-free mathematical design is coherent. Implementation readiness
is not established because the repository does not contain project-specific
metres-per-coordinate-unit provenance or a coefficient dataset/enclosure with
the required spectral, temporal, spatial, environmental and model validity.

The next safe action is a code-free physical-evidence acquisition protocol: name
the authoritative spatial calibration source and conservative coefficient
evidence, define how each is independently validated, and return
`unavailable_evidence` if either does not exist. Do not advance to schema or an
implementation-readiness spike until real evidence satisfies that protocol.

Stop immediately if work would invent a scale/coefficient fact, mistake
probabilistic coverage for a hard enclosure, assume source uniformity, mutate an
existing owner, add received-energy authority, or grant detector, visibility,
runtime, promotion or C3 closure authority.

## Verification receipt

- Focused design, inherited gap, historical route, checkpoint, master-program,
  bootstrap, record-role, modularity and module-context shields pass.
- Complete Forge command: `tools/verify.ps1`
- Exit code: `0`
- Wall time: `320.2 seconds`
- Output lines: `2,390`
- Durable files classified: `834`
- Declared modules verified: `52`

Nothing broader is locked in. One consumer first, reassess before expanding.
