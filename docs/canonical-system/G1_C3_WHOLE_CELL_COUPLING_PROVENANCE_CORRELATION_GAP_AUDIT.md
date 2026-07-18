# G1 / C3 whole-cell coupling provenance and correlation gap audit

Date: 2026-07-16

Status: **code-facing gap verified; current-owner reuse and mutation are
rejected; an independent additive provenance owner is the only surviving
architecture candidate, but no schema, crate or implementation is authorized.**

## Decision question

The whole-cell optical coupling oracle retained a conservative classifier:
strict whole-cell arrival, strict whole-cell exclusion, or typed unresolved
measure. This audit asks whether current production-candidate owners can supply
the classifier's missing subject without changing their contracts:

- one exact source phase-space parent identity;
- one exact nonnegative parent measure;
- exact nonoverlapping child partition ancestry and conservation;
- correlated source coordinates, rather than independent axis extrema; and
- a uniform lineage topology and interface branch or an explicit unresolved
  result.

This is an ownership and provenance question only. It does not select an
emission law, radiance value, receiver response, partial-coverage estimator or
runtime representation.

## Current code evidence

### Physical interval cell step

`ConditionalIntervalCellStepInputV1` stores `state_source_id`, scope,
reconstruction, revision, current cell, three independent Q160 point intervals
and three independent Q1.62 direction intervals
(`crates/physical-path-substrate/src/interval.rs:34-48`). Validation proves the
declared intervals are ordered, in range and locally contained, then hashes the
complete canonical input under the existing interval-input domain
(`interval.rs:323-385`).

It does **not** store a phase-space parent ID, parameter basis, exact measure,
partition path, child ordinal/count, or affine correlation coefficients. Its
own limitations say that it is conditional local interval evidence and makes
no optical lineage, endpoint-arrival or bulk-transfer claim
(`interval.rs:657-662`). A caller can therefore submit the same six axis bounds
for different correlated sets; the existing identity faithfully distinguishes
the bytes it received but cannot prove an unrepresented correlation.

### Local interface event

`VisibleRadianceIntervalInterfaceInputV1` stores a caller-declared incident
source ID, two cells, one face interaction and three independent direction
intervals (`crates/visible-radiance-interface-event/src/interval.rs:32-47`). Its
input ID hashes those canonical bytes. Its explicit limitation is
`declared conditional local direction-box evidence; no prior path or endpoint
arrival claim` (`interval.rs:183-214`). Uniform TIR/transmission evidence is
useful after a correlated cell exists, but this owner cannot establish the
source cell, its measure or the lost point-direction relationship.

### Optical lineage

`derive_optical_lane_id` starts from the first **already-compiled** interval
cell-step input ID plus reconstruction, bulk-profile, band and caller lane
source (`crates/optical-lineage-binding/src/lib.rs:121-146`). Later source IDs
are adjacency-derived from lane ID, ordinal and predecessor
(`lib.rs:149-163`). Replay checks exact successor point and direction boxes
against predecessor outputs (`lib.rs:311-355`).

This is strong object and adjacency lineage. It does not prove that the first
box is an exact partition of a parent source cell, attach measure, or recover
correlations erased before that first compiled input. Adding those meanings to
`lane_source_id` would be semantic overloading: the current domain hash does
not commit to them.

### Receiver and cumulative transfer

The receiver owner binds an exact Q160 AABB to the same physical scope and
classifies exact rays. It explicitly returns `UnsupportedConditionalEvidence`
when a certified time or point is nondegenerate
(`crates/receiver-arrival-geometry-binding/src/lib.rs:275-320`). It cannot be
reused as a whole-cell classifier without changing its proven exact-ray
contract.

The cumulative transfer owner consumes a validated bundle and manifest and
owns only dimensionless same-band factor accumulation. Its module boundary
explicitly excludes source emission, inverse-square spreading, receiver
geometry and endpoint arrival. It supplies a later multiplicative factor, not
the missing source-cell subject.

## Adopt / adapt / build comparison

| Option | Evidence | Decision |
|---|---|---|
| Adopt physical point/direction boxes as phase-space cells | Bounds have local provenance and conservative arithmetic, but no parent measure, partition ancestry or cross-coordinate correlation. The oracle proves box erasure can turn the true form `u-u=0` into an unresolved `[-2,2]` enclosure. | **Reject.** Reuse is allowed only later as a conservative projection from a separately proven correlated cell; the boxes cannot be its authority. |
| Adopt optical lane or manifest identity as the parent | Lane identity begins at an already-compiled interval input and commits to adjacency, not a source parameterization or measure. | **Reject.** It would fabricate pre-lineage ancestry from post-declaration evidence. |
| Adapt physical, interface, lineage, receiver or cumulative V1 schemas | Each owner has tested identities, codecs, non-goals and deletion/rollback seams. Adding phase-space semantics changes domain ownership and invalidates the narrow V1 claim. | **Reject.** Cross-owner mutation has higher regression cost and a worse rollback boundary than an additive prerequisite. |
| Build an independent capability-free phase-space provenance owner | Can bind the missing parent, exact measure, correlation form and partition ancestry while projecting read-only conservative boxes into existing inputs. | **Survives as a candidate only.** It is materially new schema and needs mathematical design, an independent oracle, code-facing readiness and explicit owner approval before source. |

## Smallest surviving prerequisite boundary

The candidate is provisionally named `optical-phase-space-cell-binding` only so
future evidence has a stable discussion handle. The name grants no directory,
crate or schema authority.

An eventual V1 design would have to own only:

1. a nonzero source-cell source ID, scope, reconstruction and revision;
2. an exact finite parameter dimension and canonical parameter domain;
3. an exact nonnegative rational measure with denominator and bit ceilings;
4. a correlation-preserving coordinate form plus explicit outward remainder;
5. immutable parent ID, refinement axis/rule, child ordinal/count and exact
   child-measure conservation;
6. a domain-separated cell ID committing to every field above;
7. a read-only projection receipt into existing physical/interface interval
   inputs, without granting those inputs source-cell authority; and
8. typed unsupported/unresolved outcomes for nonuniform topology, branch,
   folds, unbounded remainder or exhausted work.

It must not own source radiance, emission magnitude, inverse-square policy,
spectral catalogue, cumulative transfer, receiver acceptance, detector
response, visibility, perception, runtime, persistence, promotion or C3
closure.

## Dependency and identity seams

The only admissible direction is additive and downstream-safe:

`phase-space cell provenance -> conservative existing interval inputs ->
existing optical lineage -> existing cumulative transfer / exact receiver
evidence`.

No current owner may import the candidate. A future coupling consumer may
replay both the independent source-cell proof and the unchanged existing
lineage/receiver receipts. Existing identities remain byte-for-byte stable.

The phase-space cell ID must be receiver-independent and independent of
eventual transfer magnitude. Receiver-specific classification belongs in a
separate later consumer so one source partition can be replayed against more
than one receiver without changing source identity.

## Bounds that readiness must derive, not guess

This audit does not freeze numerical values. A later mathematical/oracle package
must derive:

- maximum parameter dimension and affine-symbol count;
- rational numerator/denominator and live intermediate bit ceilings;
- maximum refinement depth, child fan-out and retained cells;
- maximum canonical input, output and live bytes;
- exact operation counts for projection, subdivision and conservation checks;
- rejection behavior for zero measure, overlapping children, missing children,
  duplicate ordinals, cyclic ancestry and noncanonical rationals; and
- a resource-exhaustion outcome that preserves all unresolved measure.

The 16-portfolio / 24-hostile whole-cell oracle is necessary evidence for the
consumer semantics, but it does not derive these new-owner bounds.

## Rollback and compatibility

The only acceptable future implementation rollback is deletion of the additive owner
and its direct tests/verifier/contract entries. No migration or rollback
may rewrite physical, interface, lineage, cumulative or receiver V1 bytes,
fixtures, identities or public behavior. If independent projection cannot
consume those unchanged APIs conservatively, the candidate fails readiness.

## Adversarial obligations for the next proof

A disposable exact-rational oracle must, before readiness, reject or type:

- two correlated cells with identical independent bounds but different images;
- a forged child set whose measures sum correctly but overlaps geometrically;
- nonoverlapping children with a missing region;
- reordered, duplicated, skipped and cyclic partition ancestry;
- correlation-symbol renaming or coefficient permutation that changes canonical
  identity;
- zero/negative measure and noncanonical rational encodings;
- refinement whose accepted/zero/unresolved measures do not sum exactly to the
  parent;
- receiver-dependent source identity;
- topology, branch or fold ambiguity presented as uniform; and
- resource exhaustion that silently drops unresolved measure.

Positive portfolios must prove canonical identity, permutation rules,
parent-child conservation, conservative box projection, correlation retention,
receiver independence and deterministic replay.

## Result and stop condition

The code-facing gap is real. Existing owners correctly preserve local interval,
lineage, factor and exact-arrival evidence, but none owns source phase-space
measure plus correlated partition ancestry. Reusing or modifying them is
rejected. An independent additive provenance owner is the smallest surviving
candidate.

The next bounded action may be only a mathematical design audit and disposable
exact-rational oracle for that independent prerequisite. **Do not add a crate,
dependency, schema, test or production source.** If that proof survives, a
separate code-facing readiness package must present exact identities, codecs,
caps, compatibility, rollback and owner action. Creating the schema or crate is
a serious change and requires explicit owner approval.
