# G1 / C3 post source-calibration consumer reassessment

Date: 2026-07-17

Status: **the physical spectral/time identity owner is verified with zero
consumers. No current owner can safely import it as its first consumer. The
next bounded step is a code-free calibrated source-energy distribution
mathematical design audit, not integration.**

## What is now closed

`calibrated-spectral-time-basis` supplies one strict physical meaning for the
three RGB wavelength intervals and one exact time cell, then commits that
meaning into unchanged legacy band/time identities. It closes the former
ambiguity in which an opaque `time_basis_id` could not prove duration,
wavelength, weighting, unit or provenance.

It does not say how many joules a source assigns to a band or phase-space
cell. It also does not calibrate spatial distance, material coefficients or
pointwise transport applicability.

## Actual consumer comparison

| Candidate | Decisive issue | Disposition |
|---|---|---|
| `optical-phase-space-dimensionless-transfer` | It can already replay the derived legacy IDs, but it owns dimensionless loss and expressly excludes source magnitude. Importing physical source calibration would reverse the clean evidence direction. | **Reject as first consumer.** Cross-check remains test-only. |
| `visible-radiance-bulk-transfer` | Its coefficients remain per abstract coordinate unit. A wavelength/time basis does not provide metre mapping, coefficient provenance or whole-cell pointwise validity. | **Reject.** Transport applicability is still blocked. |
| `optical-phase-space-cell-binding` | It owns channel-neutral parameter measure and affine provenance, not wavelength, time or energy. | **Reject.** Preserve channel-neutral geometry. |
| Attach joules directly to legacy band/time IDs | Historical arbitrary IDs are permanently uncalibrated and a side table would restore the alias/migration problem the stateless graph removed. | **Reject.** |
| One combined source-to-detector record | It would collapse allocation, transport, aggregation and response before any one boundary is proved. | **Reject.** |
| Separate calibrated source-energy distribution | It can bind the verified basis, one exact phase-space cell/ancestry and exact nonnegative band-integrated joules while remaining upstream of transport and detector policy. | **Select for code-free mathematical design and oracle only.** |

## Smallest next proof

The next design/oracle package must compare a leaf-energy record, a conserved
root distribution and a density-with-respect-to-cell-measure representation.
Any survivor must bind:

- the complete calibrated-basis ID and one derived band/time identity;
- one exact optical phase-space root/cell identity, ancestry and measure;
- exact nonnegative reduced rational joules for one named band and time cell;
- parent/child conservation under 4-, 16- and 64-way refinement;
- zero, finite and unresolved allocation without representative sampling;
- source provenance/revision distinct from calibration provenance/revision;
- strict rejection of foreign basis, cell, band/time, duplicate allocation,
  negative quantity, unit alias and coordinate-reparameterization drift; and
- an authority-negative result with no transport, detector or visibility
  conclusion.

The oracle must prove that equal calibrated bases can carry different source
energies, equal source energy can have different band/cell distributions, and
positive energy with zero coupling remains distinct from zero energy with
positive dimensionless transfer.

## Boundary and sequencing

Use static reasoning and a disposable exact-rational oracle only. Add no crate,
contract schema, dependency, production test or production source. Modify no
current owner. Do not choose a normative emission spectrum, source model,
spatial scale, coefficient catalogue, transport applicability, aperture,
detector response, visibility, runtime, promotion or C3 closure.

If a distribution candidate survives, it must return through a separate
code-facing ownership/readiness audit and exact owner decision. It would be the
first and only consumer selected for implementation; reassess again before any
second consumer. Nothing broader is locked in.

## Verification basis

After this reassessment and its retained-route verifier were added, the complete
Forge gate passed in 392.9 seconds with 2,276 output lines, 812 durable files
classified and 51 modules verified. This reassessment changes no production
source or dependency.
