# G1 C6 ecological-niche semantics mathematical-design audit

Date: 2026-07-20

Status: **one atomic authored-hypothesis model selected for a code-free strict
contract candidate; no ecological truth or production source authorized.**

## Decision

Package 4 may represent an explicitly authored, provenance-bound hypothesis
about necessary evidence in one exact world packet. It may not construct a
complete niche or promote physical opportunity into habitat suitability,
resource yield, organism-specific harm, trophic flow, competition, realized
occupancy, viability, fitness or evolution.

The selected unit is one `AuthoredEcologicalHypothesisV1` with one closed role.
Multiple ecological statements require multiple independently identified
records. This prevents a bag of assertions from silently becoming a complete
niche, scalar fitness function or population model.

## Candidate comparison

| Candidate | Disposition | Reason |
|---|---|---|
| derive niche truth from C3 opportunity | reject | opportunity and coavailability are necessary physical evidence only |
| caller-labelled ecological fact graph | reject | labels, confidence and graph closure do not establish truth or authority |
| one aggregate complete-niche object | reject | composition would imply completeness and hide per-claim provenance |
| one atomic authored hypothesis plus necessary-evidence receipt | select | provenance, scope, indeterminacy and claim ceiling remain explicit |
| realized occupancy or fitness reducer | defer | requires later physiology, membership, observation and population evidence |

## Exact mathematical objects

Let `G=(V,E)` be the freshly replayed exact `niche-graph-binding` physical
opportunity graph for one world packet. V1 accepts exactly one tagged subject:

```text
SubjectRefV1 = LineageSubjectRefV1 | SpeciesCandidateSubjectRefV1
```

The tag participates in identity. A species-candidate is not a species member.
Individual, form-template, population and opaque subject references are
forbidden in V1.

An atomic hypothesis is:

```text
H = (version, id, authored_provenance, world_packet_id,
     opportunity_graph_fingerprint, physical_regime_id,
     role, sorted_unique_necessary_predicates)
```

`authored_provenance` contains exact source-artifact, source-revision and
assertion-seed identities and the fixed epistemic value `evidence_only`. It
contains no author display name, clock, confidence, approval or free-text
truth claim.

The role is exactly one of:

- `prospective_opportunity_association(subject)`;
- `habitat_requirement_candidate(subject, condition_ref)`;
- `resource_requirement_candidate(subject, resource_ref)`;
- `hazard_exposure_candidate(subject, hazard_ref)`;
- `trophic_relation_candidate(source_subject, consumer_subject, transfer_ref)`;
- `competition_relation_candidate(first_subject, second_subject,
  limiting_resource_ref)`; or
- `transition_candidate(subject, transition_ref)`.

Referents are opaque authored identities with exact source artifact and
revision. They cannot carry hidden quantities, labels-as-truth or physical
equivalence. Trophic direction is preserved. A trophic self-reference is not
automatically rejected because a cannibalism hypothesis must not be engineered
away by an invented universal rule. Detrital pathways remain unavailable until
a typed nonorganism resource/carrier owner exists. Competition endpoints may
be equal and their symmetric pair is canonicalized including equality; equality
preserves only an intraspecific-competition hypothesis and proves neither
plurality, overlapping demand, limitation, interaction, membership nor
realized competition.

## Closed necessary-evidence predicates

V1 admits only:

```text
opportunity_present(selector)
opportunity_scalar_inclusive(selector, scalar_kind, min_permille, max_permille)
coavailable(selector_a, selector_b)
```

A selector is an exact closed opportunity kind, including a typed signal
channel. Scalar kind must match its node type. Bounds are integers in
`[0,1000]`; booleans, floats, strings, negative values and reversed intervals
are noncanonical. There is no fuzzy weight, probability, interpolation,
absence predicate, resource yield, renewal, dose, harm, fitness or scalar
cross-dimension score.

For the unique resolver `resolve_G(s)`:

```text
present(s) = supported                    iff resolve_G(s) exists
             unavailable                  otherwise

scalar(s,k,[l,h]) = unavailable           iff resolve_G(s) is absent
                    supported             iff typed q_k exists and l <= q_k <= h
                    contradictory         iff typed q_k exists outside [l,h]

coavailable(a,b) = unavailable            iff either selector is absent
                   supported              iff the exact canonical edge exists
```

Selector/scalar mismatch is invalid input, not an ecological result. Because
the current opportunity graph asserts complete coavailability among its
resolved nodes, a missing edge between two resolved distinct nodes is invalid
upstream evidence, not a trophic or competition contradiction. Intervals on
the same selector/scalar must have a nonempty intersection; otherwise the
received necessary evidence is contradictory.

After schema, provenance, subject and cumulative-budget preflight, aggregation
is deterministic: contradiction dominates unavailable, and unavailable
dominates supported. Insufficient examination budget returns
`indeterminate_budget` before semantic work. Invalid, stale, cross-world,
cross-subject, substituted or forged inputs emit no receipt.

## Epistemic invariant

The receipt disposition is exactly one of:

- `necessary_evidence_supported`;
- `necessary_evidence_unavailable`;
- `necessary_evidence_contradictory`; or
- `indeterminate_budget`.

Every receipt also carries `ecological_truth_status: unresolved`. Therefore a
supported receipt means only that replayed physical evidence meets the named
authored necessary predicate in this packet. It never means suitable, viable,
occupied, harmful, edible, renewable, trophically connected, competitively
limited, adapted or evolutionarily fit. Missing evidence is not false or zero;
contradiction is not biological impossibility; budget exhaustion is not a
negative ecological result.

## Provenance and identity

Hypothesis and receipt identities use SHA-256 over a disjoint ASCII domain,
one NUL byte and exact compact semantic-array bytes, excluding the record's own
ID field. Domains are:

- `mindwarp/c6-ecological-authored-provenance/v1`;
- `mindwarp/c6-ecological-subject-ref/v1`;
- `mindwarp/c6-authored-ecological-hypothesis/v1`;
- `mindwarp/c6-ecological-necessary-evidence-receipt/v1`; and
- `mindwarp/c6-organism-niche-hypothesis-binding/v1`.

Equal values never repair different world, graph, regime, subject, source or
revision identities. The misleading historical
`MacroLineageCandidate.occupied_opportunity_ids` remains unchanged and may be
consumed only as legacy hypothesis association evidence; V1 names prospective
opportunity references and never upgrades that field to occupancy.

## Deterministic fixture portfolio

The code-free fixture model freezes:

1. supported exact opportunity association;
2. missing liquid as unavailable, not false or zero;
3. scalar values at both inclusive bounds;
4. an exact scalar outside the authored interval as contradictory received
   evidence while ecological truth remains unresolved;
5. intersecting and disjoint same-dimension intervals;
6. habitat necessary evidence supported with suitability unresolved;
7. liquid present with access, yield and renewal unresolved;
8. pressure present with dose, tolerance and harm unresolved;
9. directed two-subject trophic candidate with flow and quantity unresolved;
10. canonical competition candidates with unequal and equal endpoints, with
    plurality, scarcity, demand, limitation and realized interaction unresolved;
11. predicate permutation yielding identical canonical bytes;
12. changed world, subject or provenance changing identity;
13. humanoid and structurally distinct radial identity controls without sex
    or dimorphism semantics; and
14. one future additive organism-niche binding that preserves the existing
    environmental-support result as necessary evidence only.

## Frozen hostile mapping

- `C6-H100`: physical opportunity or coavailability presented as a complete
  ecological niche;
- `C6-H101`: exposure, moisture, region or boundary presented as habitat or
  suitability;
- `C6-H102`: liquid presented as accessible yield/replenishment, or exposure
  presented as organism-specific harm;
- `C6-H103`: labels or coavailability presented as trophic flow;
- `C6-H104`: prospective association presented as realized occupancy,
  survival, abundance, distribution, adaptation or evolution;
- `C6-H105`: labels or coavailability presented as competition without a
  limiting-factor candidate and overlapping-demand evidence.

Applicable integration hostiles remain `C6-H1100`, `C6-H1101`, `C6-H1102`,
`C6-H1103`, `C6-H1105`, `C6-H1106` and `C6-H1108`: disconnected receipts,
world/subject/provenance substitution, partial output, budget laundering,
codec/platform drift, authority inflation and host capability leakage.
`C6-H1104` scale generalization and `C6-H1107` real C5 consumption remain
later closure obligations.

Additional fixtures reject a physical boundary or disposable ecotone oracle
as production ecology; identity as membership; body-plan expression as sex or
dimorphism; cross-kind subject substitution; numeric-equal provenance swaps;
duplicates, unknown keys and trailing codec bytes; maximum-plus-one bytes,
predicates or examinations; dependency cycles; and mutation of predecessor
bytes. An equal-endpoint competition receipt must also be rejected as proof of
self-competition or the existence of multiple competitors.

## Ordering and dimorphism boundary

Package 4 exposes no sex, reproductive role, caste, dimorphism or life-stage
applicability field. Physiology/viability package 5,
reproduction-parentage-heredity-development package 6, realized occupancy and
evolution package 7, and species/ecomorph portfolios package 8 precede package
9 species-authored variation rules. Package 9 may define dimorphism
applicability, but applying it to an individual still requires package 10
explicit species membership. Ecology, body-plan expression, labels,
capability, personality, rank and appearance may not infer it.

## Claim ceiling and stop

This design proves only that the selected strict model can preserve authored
provenance, exact necessary-evidence replay and typed indeterminacy. It creates
no ecological fact, product vocabulary, production ecotone, calibrated scale,
fitness score, species membership, physiology, reproduction, heredity,
development, sex, dimorphism, culture, gameplay, personhood, representation,
runtime or promotion authority.

Stop at the code-free contract candidate and implementation-readiness review.
No crate, Cargo dependency, consumer implementation or implementation result
is authorized. Nothing broader is locked in. One consumer first, reassess
before expanding.
