# Ecological niche semantics authored-hypothesis contract candidate

Status: **code-free implementation candidate; no production source authority.**

This candidate defines one atomic, provenance-bound ecological hypothesis and
one necessary-evidence assessment receipt. It validates structural binding to
exact physical evidence; it does not validate ecological truth.

## V1 records

All IDs are nonzero lowercase 64-hex `Id32` values in distinct semantic
domains. All arrays are bounded, canonically ordered and duplicate-free.

### `AuthoredProvenanceV1`

- `schema_version`: integer `1`;
- `provenance_id`;
- `source_artifact_id`;
- `source_revision_id`;
- `assertion_seed`; and
- `authority_status`: exactly `evidence_only`.

No author label, time, confidence, approval or free-text assertion is semantic
input.

### `SubjectRefV1`

Tagged union of exactly `lineage_subject` or `species_candidate`. It contains
`subject_id`, `canonical_record_fingerprint`, `lineage_subject_ref_id` and
`world_packet_id`. The exact current organism-subject identity record must be
received and replayed. Individual, form-template, population and opaque
subjects are forbidden. A species-candidate tag grants no membership.

### `AuthoredReferentV1`

Contains only `referent_id`, `source_artifact_id` and `source_revision_id`.
It is an authored identity, not a quantity, label-as-fact or physical identity.

### `EcologicalRoleCandidateV1`

Exactly one closed variant per hypothesis:

- `prospective_opportunity_association`;
- `habitat_requirement_candidate`;
- `resource_requirement_candidate`;
- `hazard_exposure_candidate`;
- `trophic_relation_candidate`;
- `competition_relation_candidate`; or
- `transition_candidate`.

Trophic endpoints preserve direction and may be equal, retaining a cannibalism
hypothesis only; detrital pathways require a later typed nonorganism
resource/carrier owner. Competition endpoints may be equal and their symmetric
pair is canonically ordered by `(subject_tag, subject_id)`, including equality.
An equal pair retains an intraspecific-competition hypothesis only and proves
no plurality, overlapping demand, limitation, interaction, membership or
realized competition. These are candidate roles only. They do not assert
habitat, use, harm, flow, limitation, transition or occupancy.

### `NecessaryEvidencePredicateV1`

Exactly one of:

- `opportunity_present(selector)`;
- `opportunity_scalar_inclusive(selector, scalar_kind, min_permille,
  max_permille)`; or
- `coavailable(selector_a, selector_b)`.

Selectors and scalar kinds are closed enums derived from the exact V1
opportunity graph. Permille bounds are integer-only `[0,1000]`. Predicates sort
by their tagged canonical bytes. Duplicate predicates reject; they are not
collapsed. Same-selector scalar intervals must share a nonempty intersection.

### `AuthoredEcologicalHypothesisV1`

- `schema_version`: integer `1`;
- `hypothesis_id`;
- `candidate_class`: exactly `authored_hypothesis`;
- one `AuthoredProvenanceV1`;
- exact `world_packet_id`;
- exact `opportunity_graph_fingerprint`;
- exact `physical_regime_id`;
- one `EcologicalRoleCandidateV1`; and
- one to twelve sorted unique necessary-evidence predicates.

At most two distinct subjects and one authored referent occur in one atomic
hypothesis. Multiple statements require multiple records.

### `NecessaryEvidenceReceiptV1`

- `schema_version`: integer `1`;
- `receipt_id`, `hypothesis_id` and exact received hypothesis fingerprint;
- exact world, opportunity-graph, physical-regime and subject fingerprints;
- sorted resolved opportunity node IDs;
- one disposition per predicate;
- `overall_disposition`;
- `examinations_used`;
- `ecological_truth_status`: exactly `unresolved`;
- `capabilities`: exactly an empty array; and
- `occupancy`, `membership`, `viability`, `ancestry`, `runtime_authority`,
  `approval_authority`, and `promotion_authority`: exactly `false`.

Overall disposition is exactly
`necessary_evidence_supported`, `necessary_evidence_unavailable`,
`necessary_evidence_contradictory`, or `indeterminate_budget`. Invalid inputs
and preflight failures emit no receipt.

## Evaluation contract

Validation order is fixed:

1. cumulative input byte/count/examination preflight;
2. strict canonical decode;
3. record identities and provenance replay;
4. exact world and subject replay;
5. opportunity graph fingerprint and completeness replay;
6. predicate structural validation;
7. deterministic predicate evaluation;
8. aggregate disposition; and
9. atomic canonical receipt construction.

Contradictory dominates unavailable; unavailable dominates supported. A
missing selector is unavailable. An exact typed scalar outside an authored
inclusive interval is contradictory received evidence. A selector/scalar type
mismatch is invalid input. A missing edge between two resolved distinct nodes
in the current complete graph is invalid upstream evidence. Insufficient
budget yields `indeterminate_budget` before semantic evaluation and emits
nothing else.

`necessary_evidence_supported` does not mean ecologically suitable, viable,
occupied, harmful, edible, renewable, trophically connected, competitively
limited or fit. Missing is not false or zero; contradiction is not biological
impossibility; budget exhaustion is not a negative ecological conclusion.

## Strict canonical codec

The codec is compact UTF-8 JSON with exact field order, lowercase closed enums,
lowercase 64-hex IDs and integer-only numeric fields. It forbids unknown,
duplicate, missing or reordered keys; duplicate array elements; noncanonical
array order; booleans where integers are required; floats; numeric strings;
NaN/infinity; invalid UTF-8; insignificant whitespace; and trailing bytes.
Decode followed by exact re-encode must equal the received bytes.

Identity is SHA-256 over the exact ASCII domain, NUL, and compact semantic
array bytes, omitting the record's own ID. Domains are exactly:

- `mindwarp/c6-ecological-authored-provenance/v1`;
- `mindwarp/c6-ecological-subject-ref/v1`;
- `mindwarp/c6-authored-ecological-hypothesis/v1`;
- `mindwarp/c6-ecological-necessary-evidence-receipt/v1`; and
- `mindwarp/c6-organism-niche-hypothesis-binding/v1`.

Diagnostic reason codes are closed snake-case values and never semantic free
text.

## Frozen resource envelope

- maximum hypothesis bytes: `16,384`;
- maximum receipt bytes: `32,768`;
- maximum predicates: `12`;
- maximum distinct subjects: `2`;
- maximum authored referents: `1`;
- maximum received opportunity nodes: `12`;
- maximum received complete coavailability edges: `66`;
- maximum examinations: `128`;
- maximum output records: `1`;
- work: `O(V + E + P)`;
- recursion, dynamic graph expansion and bulk hypothesis arrays: forbidden.

The counted examination requirement is
`16 + |V| + |E| + |predicates| + 4*|subjects|`, at most `114` under V1,
leaving fourteen examinations only as an explicit validation shield. Required
budget passes; required-minus-one returns `indeterminate_budget` before
semantic work. Every maximum and maximum-plus-one case is a permanent fixture.
All cumulative byte/count limits preflight before decode expansion. No failure
may produce a partial receipt.

## One-consumer design

The only prospective consumer is one additive
`OrganismNicheHypothesisBindingV1` in `organism-niche-binding`. It binds an
exact subject, hypothesis, necessary-evidence receipt and fingerprint of the
unchanged environmental-support package. Its status is fixed
`necessary_evidence_only`.

It may not reinterpret or change existing `SolutionFamily.feasible` bytes and
may not emit suitability, viability, sense, capability or occupancy. C3,
body-plan, macro-lineage and organism-subject-identity may not consume the new
owner. No second consumer is admitted in V1.

## Frozen nonclaims

No ecological truth; no habitat or biome fact; no resource access, yield or
renewal; no organism-specific harm; no trophic flow or competition fact; no
production ecotone; no realized occupancy, abundance or population
distribution; no physiology, viability, sense or capability; no ancestry,
adaptation or evolution; no species membership; no reproduction, heredity,
development, sex, dimorphism or caste; no culture, gameplay, personhood,
aesthetic, representation, C7, runtime, approval or promotion authority.

The contract candidate creates no crate, Cargo member/dependency, production
source, consumer implementation or migration. Implementation requires a
separate exact owner authorization and must retain deletion-only rollback.
