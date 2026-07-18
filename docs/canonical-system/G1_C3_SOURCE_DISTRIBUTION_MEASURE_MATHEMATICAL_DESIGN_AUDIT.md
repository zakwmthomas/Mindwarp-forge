# G1 / C3 source-distribution measure mathematical design audit

Date: 2026-07-17

Status: **oracle-ready as an abstract additive source-quantity measure; not
implementation-ready and not a claim of watts, radiance, received power or
detector response.**

## Subject and inherited owners

The subject is one already validated optical phase-space root and one cell in
its exact binary ancestry, one exact `band_time_id`, and one separate nonzero
opaque `quantity_basis_id`. The existing cell owner continues to own the root,
ancestry and exact positive abstract geometric measure. The existing
whole-cell dimensionless-transfer owner continues to own accepted, zero and
unresolved transfer measure. This design modifies neither owner.

The new mathematical candidate is an independent nonnegative finite measure
`nu` on the exact cell algebra. For every owned split:

`nu(parent) = nu(lower_child) + nu(upper_child)`.

Unlike the cell owner's geometric measure, source quantity may be exactly
zero. A future source owner would produce both child quantities atomically and
prove the equality; callers may not attach unrelated leaf values. The subject
must bind root ID, reconstruction ID, scope, revision, cell ID and ancestry,
`band_time_id`, and `quantity_basis_id`. Identity equality proves correlation,
not physical calibration.

## Primary metrology boundary

NIST defines radiance using optical flux per projected physical area and solid
angle, with the projected-area cosine factor. BIPM gives radiance the coherent
SI unit `W m^-2 sr^-1` and separately warns that a unit does not uniquely
identify a quantity. NIST defines the steradian as solid angle, not an
arbitrary dimensionless area ratio.

Sources:

- NIST, *Introduction to Optical Radiometry / Radiometric Terminology*:
  <https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=104704>
- BIPM, *The International System of Units (SI Brochure), 9th edition*:
  <https://www.bipm.org/documents/d/guest/si-brochure-9-en-pdf>
- NIST SP 330, section 5:
  <https://www.nist.gov/pml/special-publication-330/sp-330-section-5>

The current abstract cell measure proves none of projected physical area,
solid angle, a coordinate Jacobian, wavelength integration, duration or an SI
quantity basis. Exact subdivision conservation therefore cannot authorize the
label radiance. Scattering and emission also remain separate transport facts.

## Candidate comparison

| Candidate | Refinement behavior | Decision |
|---|---|---|
| Ambient absolute quantity copied onto every leaf | Duplicates quantity when a leaf is split, or discards it when one child is omitted | Reject |
| Density multiplied by the abstract cell measure | Changes when the abstract coordinate measure is rescaled unless a proven Jacobian transformation exists | Reject as a physical source primitive |
| SI radiance attached to the current cell | Lacks projected area, solid angle, spectral/temporal calibration and quantity-basis authority | Reject |
| Additive nonnegative finite source-quantity measure | Parent quantity is exactly the sum of atomic child quantities and is invariant under representation-only refinement | Select for the oracle only |

The selected candidate is abstract. `quantity_basis_id` distinguishes exact
quantity families but does not mean joules, watts, photons, radiance or RGB
display intensity. A later authority must define the physical quantity basis
and its spectral and temporal scope before any such name is permitted.

## Reparameterization counterexample

Let a cell have abstract measure `mu(C)=1` and a caller attach density `d=3`.
The inferred quantity is `3`. Relabel the same physical subject with abstract
measure `mu'(C)=2` while leaving the uncalibrated density unchanged; the
inferred quantity becomes `6`. No physical source changed. A density is safe
only with a proven transformation law and Jacobian.

The additive measure instead transports `nu(C)=3` with the subject and splits
it so the children sum to `3`. It does not divide by `mu`, so a coordinate-only
rescaling does not manufacture quantity.

## Exact composition boundary

For algebraic testing only, an accepted cell quantity `q >= 0` and a finite
dimensionless transfer enclosure `[l,u]` compose to `[q*l,q*u]`. Exact zero
transfer yields zero received candidate quantity while retaining `q` in a
typed zero-coupled source bucket. Unresolved transfer retains the full `q` in
an unresolved bucket. No sample may stand for a cell.

A positive exact product that projects to zero in a later fixed-point format
is numerical underflow, not physical zero. Zero source with positive transfer
and positive source with zero coupling are distinct cases.

The existing coupling measure already represents geometric acceptance over
the phase-space cell. Applying an additional inverse-square or spreading
factor without a separate theorem would double-count geometry and is rejected.
The oracle therefore multiplies only by the already owned dimensionless
transfer enclosure and makes no received-power claim.

## Conservation and ownership invariants

The disposable oracle must prove:

1. exact geometric-measure and source-quantity conservation at 4, 16 and 64
   leaves under nonuniform source splits, including zero-quantity children;
2. atomic child production rejects quantity duplication, deletion, wrong
   parent totals and independently authored leaves;
3. root, cell, ancestry, band/time and quantity-basis substitution fail closed;
4. negative and noncanonical rational quantities fail closed;
5. accepted, zero-coupled and unresolved quantities are neither dropped nor
   duplicated;
6. coordinate-rescaling, ambient-leaf duplication and premature-SI-radiance
   counterexamples remain explicit;
7. an added geometric-spreading multiplier changes an already composed result
   and is rejected as unproved double counting; and
8. exact positive underflow is typed as numerical projection loss.

## Stop boundary

Run one deterministic exact-rational oracle only. Add no crate, contract
schema, dependency, production test, production source or runtime integration.
Do not modify an existing owner. Do not claim watts, radiance, received power,
detector response, visibility, perception, darkness, rendering, gameplay line
of sight, promotion or C3 closure.

If the candidate survives, the next action is a code-facing
quantity-basis-and-schema gap audit. That audit must decide whether an exact
physical quantity basis and spectral/temporal calibration exist or must be
designed; it does not automatically authorize implementation.
