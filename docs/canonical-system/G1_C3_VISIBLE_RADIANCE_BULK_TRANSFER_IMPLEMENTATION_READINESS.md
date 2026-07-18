# G1 C3 Visible-Radiance Bulk-Transfer Implementation Readiness

Date: 2026-07-15

Status: **implementation-ready as one bounded capability-free reference;
consumer code is not authorized by this audit.**

## Readiness decision

The first bulk-transfer reference is ready only because it remains narrower
than a general piecewise optical integrator. A known v1 result requires one
unique positive-length medium along the open path: all contributing cells are
vacuum or carry the same exact substance identity. Any transition between
distinct substances, including vacuum-to-substance, returns
`interface_model_required`.

That boundary engineers away the 337-bit mixed-rational accumulator found by
the oracle. V1 needs no bespoke U384 implementation and no new big-integer
dependency. It uses checked `u128` arithmetic after imposing one explicit
geometric admission ceiling: the sum of the three raw Q32.32 squared endpoint
deltas must fit `u128`. This retains an enormous bounded reference domain while
rejecting only paths near the extreme full-i64 diagonal rather than wrapping.

## Whole-plan alignment

The package advances observer-independent direct-beam evidence after the exact
path substrate. It does not change source radiance, biological perception,
rendering, line-of-sight gameplay, generic passage or organism ownership. It
adds no occupancy truth and selects no planet, sphere, terrain, topology or
runtime system.

Biome continuity remains a permanent downstream rule. Cells, substances and
physical regions cannot paint biome or material seams. Continuous causal
fields must generate deterministic ecotones; sharp boundaries remain sharp
only when a sharp physical cause exists.

## Exact module boundary

An authorized package adds only `crates/visible-radiance-bulk-transfer` with a
local dependency on `physical-path-substrate` plus the existing local
`serde`, `serde_json` and `sha2` pattern. It imports no Forge Kernel, UI/Tauri,
filesystem, process, network, persistence, runtime, ecology or representation
capability.

The module owns:

- a strict volume-bound three-band substance interaction profile;
- reconstruction of the exact physical volume and path witness;
- canonical reduction to unique open-path medium evidence;
- checked direct-beam bulk optical-depth and transmission bounds; and
- explicit unavailable, ambiguous-lane and interface-required evidence.

It does not own coefficient discovery, surface/interface optics, source or
receiver physics, perception, presentation, passage, navigation or biome
meaning.

## Frozen input and profile seam

`VisibleRadianceBulkProfileInputV1` contains only:

- `schema_version = 1`;
- nonzero 32-byte `profile_source_id`, `scope_id` and `reconstruction_id`;
- positive `profile_revision`;
- the exact `PhysicalVolumeRecipeInputV1`; and
- canonical `SubstanceBulkInteractionV1` entries.

Compilation rebuilds `PhysicalVolumeV1`. The profile reconstruction identity
must equal the volume recipe reconstruction identity. Entries are sorted by
`substance_source_id`, unique, nonzero and exactly cover every unique gas,
liquid or solid substance present in the reconstructed volume. No entry exists
for vacuum or unavailable evidence.

Each substance entry contains exactly three fixed ordered bands
`red`, `green`, `blue`. Each `BulkBandInteractionV1` is either:

- `finite { extinction_q16_48_per_coordinate_unit: u64 }`, where zero is valid
  transparent bulk; or
- `opaque`.

The coefficient is dimensionless optical depth per volume coordinate unit. It
is declared provenance-bound evidence, not a query multiplier or SI claim. A
future metre mapping or coefficient catalogue requires its own versioned
authority and validation.

`VisibleRadianceBulkQueryV1` contains only `schema_version = 1`, the exact
compiled profile identity and one `PhysicalPathQueryV1`. Compilation rebuilds
the profile, volume and exact witness. Callers never submit cells, spans,
optical depth, transmission, classification or output state.

## Canonical path classification

The consumer constructs all breakpoints from exact witness interval endpoints
and examines each nonempty open parameter span:

1. any active unavailable cell returns `unavailable_evidence`;
2. anything other than one active positive-length cell returns
   `ambiguous_boundary_lane`;
3. adjacent spans with the same cell evidence are merged;
4. point-only records do not contribute bulk length or a material transition;
5. a positive-length sequence containing more than one evidence identity
   returns `interface_model_required`; and
6. one vacuum sequence is identity, while one substance sequence uses its
   exact profile entry.

Evidence identity includes vacuum versus the exact substance ID; dominant
phase alone never makes two media equivalent. A path over multiple cells with
the same substance ID remains one medium and is invariant to subdivision.

A stationary path has zero bulk length. If any containing point record is
unavailable it returns `unavailable_evidence`; otherwise it returns exact
identity. Other point-only substances neither attenuate nor create an
interface.

## Frozen output seam

`VisibleRadianceBulkTransferV1` binds the exact profile, volume, query and
witness identities plus one closed `BulkTransferOutcomeV1`:

- `known { bands: [BandTransferV1; 3] }`;
- `unavailable_evidence`;
- `ambiguous_boundary_lane`; or
- `interface_model_required`.

Each known band is:

- `finite` with `optical_depth_lower_q64_64`,
  `optical_depth_upper_q64_64`, `transmission_lower_q0_48` and
  `transmission_upper_q0_48`;
- `opaque` with exact zero transmission; or
- `vacuum_identity` with exact zero optical depth and exact one transmission.

Q64.64 values use a strict two-limb `{ high_u64, low_u64 }` codec rather than a
JSON `u128` number. Every lower bound is at most its upper bound. Q0.48
transmission is at most `2^48`; exact identity is `2^48`.

## Checked arithmetic

For a nonstationary query:

1. subtract endpoint coordinates in `i128` and take checked `u64` magnitudes;
2. square each magnitude into `u128` and checked-add all three; reject
   `arithmetic_ceiling` if the sum overflows;
3. compute directed integer square root, producing adjacent raw Q32.32 length
   bounds unless the sum is a perfect square;
4. multiply each raw `u64` length bound by the selected Q16.48 coefficient in
   `u128` and divide by `2^16` with directed floor/ceiling to obtain Q64.64
   optical-depth bounds; and
5. convert the upper optical-depth bound into the transmission lower bound and
   the lower optical-depth bound into the transmission upper bound.

Zero optical depth returns exact identity. Opaque returns exact zero before the
exponential kernel.

The exponential kernel freezes the passing oracle structure:

- range-reduce each Q64.64 endpoint by powers of two until the directed Q0.64
  argument is at most one half;
- enclose the alternating `exp(-x)` series in Q0.64 using checked `u128`
  multiply/divide and at most 192 terms, stopping only when the next upper term
  is at most one Q0.64 unit;
- restore the range with directed interval squaring, special-casing exact
  Q0.64 identity so `2^64 * 2^64` is never evaluated in `u128`; and
- project outward to Q0.48.

Every operation is checked. Nonconvergence, invalid order, overflow or a bound
wider than the frozen maximum of one Q0.48 unit fails compilation. No epsilon,
float, saturation or best-effort output exists.

## Ceilings and cost

V1 retains the substrate ceilings of 65,536 cells and witness records and adds
`MAX_BULK_PROFILE_SUBSTANCES = 65_536`. Profile reconstruction and witness
classification are `O(N)`; known transfer arithmetic is constant per band
after classification. The implementation receipt must report maximum-profile
canonical bytes, maximum witness/profile validation time and the retained
65,536-cell fixture cost.

The arithmetic ceiling is part of the public v1 contract. A future broader
diagonal or multi-substance integrator requires a new candidate with proven
wide arithmetic; it cannot silently replace v1.

## Strict codecs and validation

Profile input/profile result/query/transfer codecs use deny-unknown-fields,
canonical field order, canonical substance order and exact re-encoding.
Validators rebuild the complete profile, volume, witness, classification and
transfer. Plausible caller-authored state, altered limitations, reordered
entries, duplicate identities, foreign volume/query/profile IDs, legacy
transmission multipliers, whitespace variants and trailing bytes fail closed.

## Mandatory hostile proof

An implementation must retain at least:

1. strict deterministic profile/query/transfer replay;
2. exact substance coverage and canonical profile ordering;
3. zero/nonzero IDs, revision, coefficient extremes and unknown variants;
4. vacuum identity and finite zero extinction identity;
5. positive finite attenuation and per-band opaque behavior;
6. same-substance multi-cell subdivision invariance;
7. forward/reverse exact equality;
8. monotonicity in coefficient and admitted length;
9. perfect-square and adjacent-nonsquare length bounds;
10. checked three-square overflow rejection before transfer arithmetic;
11. stationary interior/face/edge/vertex and unavailable stationary evidence;
12. tangent point contact versus positive-length thin material;
13. face/edge-aligned ambiguous positive-length lanes;
14. vacuum/substance and distinct-substance interface-required results;
15. missing profile, unavailable interval and foreign witness failures;
16. Q64.64 limb and Q0.48 bound codec forgeries;
17. exponential zero, tiny, half-range, ordinary, large and opaque fixtures;
18. permanent equivalence with the Python oracle fixed vectors and deterministic
    generated admitted cases;
19. maximum profile/path cost and all allocation ceilings; and
20. absence of interface, scattering, emission, perception, rendering,
    passage, biome, planet, runtime, approval or promotion claims.

## Integration, rollback and failure containment

An authorized reference must add workspace, module-boundary, module-context,
canonical-system registry, contract/result and C3 verifier entries. The C3
verifier must continue executing the independent Python oracle and run the new
crate with warnings denied. Complete Forge verification remains mandatory.

The package is additive. Removing the crate and its exact integration entries
restores the safe state of no canonical visible-radiance path transfer; the
path substrate, signals, surface material and all existing identities remain
unchanged.

Any arithmetic, reconstruction, ambiguity or oracle mismatch aborts without a
partial transfer. It cannot be repaired with phase defaults, caller
multipliers, wider claims or automatic interface assumptions.

## Exact owner action

The prepared bounded action is:

> Authorize the capability-free `visible-radiance-bulk-transfer` v1 reference
> exactly as bounded by this readiness audit: single-medium known results,
> checked u128 arithmetic with the explicit squared-length ceiling, directed
> Q64.64/Q0.48 enclosures, permanent Python-oracle equivalence, all hostile and
> cost receipts, and no interface, perception, biome, planet, runtime, C3
> closure or promotion authority.

This document prepares that action; it does not release it.
