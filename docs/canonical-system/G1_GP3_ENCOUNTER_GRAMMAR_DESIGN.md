# G1 GP3 encounter grammar design

Status: decision-complete pre-source design proposed for independent review.

## Canonical digest framing

All hashes are SHA-256 over:

`u32_be(domain_byte_length) || domain_utf8 || u64_be(payload_byte_length) || canonical_json_payload`

The domains are `mindwarp.gp3.fixed-session.v1`,
`mindwarp.gp3.session-fact.v1`, `mindwarp.gp3.risk.v1`,
`mindwarp.gp3.threat.v1`, `mindwarp.gp3.fixed-situation.v1`,
`mindwarp.gp3.fixed-grammar.v1`, and
`mindwarp.gp3.consequence.{mutation|opportunity-cost|memory|grant|named-decision|threat-contribution}.v1`.
Canonical payload bytes come from the owning GP0 value's strict JSON order, not
from caller text. The five session digests and every element digest are frozen
in `G1_GP3_ENCOUNTER_GRAMMAR_FIXED_REGISTRY.md`.

## Public contract and exact-authored equality

`EncounterGrammarV1` contains `schema_version` and exactly five
`EncounterSituationV1` values. `fixed_encounter_grammar()` is the sole registry
constructor. Callers may decode a strict canonical copy or query a situation,
but cannot register, generate, score, reorder, or select candidates.

Each `EncounterSituationV1` contains:

- `situation_id`, exact GP0 `session_id`, and `session_digest`;
- exact ordered `domain_facets: Vec<DomainFacetV1>`;
- exact ordered `evidence_refs: Vec<EncounterEvidenceRefV1>` and
  `risk_refs: Vec<EncounterRiskRefV1>`;
- exact ordered `approaches: Vec<EncounterApproachV1>`; and
- an optional exact `EncounterThreatRefV1`.

Validation requires equality with the corresponding complete fixed authored
situation, not merely a self-consistent shape or matching IDs. Equality covers
all order, IDs, digests, prepared tools, steps, prerequisites, dispositions,
facet propositions, explanations and limitations. The registry similarly must
equal the exact ordered five-situation registry. This rejects coherent but
unauthorized content.

## Multi-domain facets and evidence

`DomainFacetV1` is a `deny_unknown_fields` tagged enum with environment,
creature, society, anomaly and construction variants. Each variant contains an
exact `facet_id`, bounded authored `proposition`, and one or more supporting
evidence IDs. A situation has all and only the ordered facets frozen in the
fixed-registry appendix; it is not reduced to one primary domain.

`EncounterEvidenceRefV1` binds `fact_id`, expected `FactKind`, exact
`EvidenceClass::AuthoredGameplayNonC3B`, and canonical fact digest. Validation
resolves the exact fact in the bound GP0 session and requires all four fields.
It rejects foreign facts, altered propositions, `ObservedC3AOutput`, invented
facts, and facet support not present in the situation evidence list.

`EncounterRiskRefV1` binds exact `risk_id` and canonical risk digest. Risk cue
payload remains GP0-owned. Each approach has exactly one disposition for every
situation risk and no foreign disposition.

## Authored approaches and causal explanations

`EncounterApproachV1` contains:

- a distinct `approach_id`, never equal to its `outcome_id`;
- noncombat `ApproachKindV1`;
- exact `prepared_tool_id: Option<String>` (`None` only for retreat);
- ordered `InterventionStepV1 { step_id, kind, subject_ids,
  proposition }` values, where `kind` is exactly `repair`, `construct`,
  `coordinate`, `care`, `extract`, `negotiate`, `traverse`, `coerce`,
  `dismantle`, or `withdraw`;
- ordered tagged `ApproachPrerequisiteV1` values whose kinds are exactly
  `observed_fact`, `available_inference`, `prepared_tool`, `authored_state`, or
  `exact_predecessor`, with `reference_id` and optional `expected_digest`;
- ordered `RiskDispositionV1 { risk_id, disposition, explanation }` values,
  where disposition is exactly `resolved`, `mitigated`, `accepted`,
  `transferred`, or `unchanged`;
- one `CausalExplanationV1`; and
- exact GP0 `outcome_id` plus its complete `ConsequenceRefV1` set.

The approach kinds are `intervention`, `care`, `negotiation`,
`alternate_route`, `construction`, `force_partial`, and `retreat`. All are
noncombat. Every situation has at least two non-retreat approaches for which
threat diversion is not a prerequisite, and exactly one retreat.

`CausalExplanationV1` binds exact admitted evidence IDs, the same ordered step
IDs, the complete consequence-reference set, every risk-disposition reference,
and bounded authored `explanation` plus `limitation`. Validation requires the
closed chain to use only admitted evidence and authored steps and to cover the
exact outcome elements. Fixed equality prevents motive, unseen evidence,
erased opportunity cost, substituted liability, or invented causality.

`s3.approach.force` alone uses `force_partial`. It references `s3.force`, whose
GP0 `resolves_core_tension` flag is false. Its limitation explicitly preserves
unresolved ownership and damaged cooperation. Validation rejects any second
force approach, any force-complete claim, or a force route to a resolving
outcome.

## Complete abstract consequence references

`ConsequenceRefV1` contains tagged `kind`, zero-based `ordinal`, and canonical
element digest. The kinds are exact mutation, fixture-owned opportunity cost,
memory, typed grant and named decision. Each approach's outcome set must cover
every element in the referenced GP0 outcome exactly once and nothing else.
When optional threat composition is selected, each exact GP0 threat mutation is
covered separately and exactly once using the `threat_contribution` kind. Those
refs are world-contribution-only, nonterminal, and never members of the outcome
consequence set.

`resolve_outcome(session, approach)` first binds the complete approach against
the fixed session registry, then returns the referenced GP0 `OutcomeRecordV1`;
`resolve_consequence(session, approach, ref)` repeats that authority check and
returns the corresponding borrowed typed GP0 element. GP3 never copies consequence payloads and
never accepts them from a caller. Outcome trigger, `resolves_core_tension` and
`afterlight_trigger` remain GP0-owned validation inputs rather than duplicated
GP3 authority.

## Exact S5 predecessor prerequisite

Every S5 approach includes `LatestS1OutcomeV1` with admitted outcome IDs exactly
`s1.direct`, `s1.bypass`, and `s1.ration`, and rejected ID exactly
`s1.retreat`. `validate_approach_context` consumes the ordered GP0 history,
selects the latest prior `gp0.s1.colony-conduit` event, and requires its outcome
to equal the supplied predecessor. Missing history, no S1 event, a stale older
S1 selection, reordered history, a foreign session, or latest S1 retreat is
rejected. This is validation only: GP3 does not mutate history or map the
predecessor into GP2 progression.

## Optional nonterminal threat composition

`EncounterThreatRefV1` binds exact `threat_id`, whole canonical threat digest,
complete ordered `ThreatMutationRefV1 { kind: threat_contribution, ordinal,
digest }`, and
`nonterminal: true`. It is present only for S2 `predator`, S4
`wire-scavengers`, and S5 `food-scavengers`; S1 and S3 require `None`.

`compose_optional_threat(session, situation, approach, false)` validates all
three fixed records and returns no contributing mutations;
`compose_optional_threat(session, situation, approach, true)` performs the same
validation and may return only the exact borrowed GP0 threat mutations. It
cannot supply a prepared tool, satisfy a prerequisite,
choose or change an outcome, alter consequence references, emit progression,
be terminal, or resolve core tension. Both composition choices validate for
every non-retreat and retreat route in S2, S4 and S5.

## Strict codec and nested bounds

Every public struct and tagged enum uses `deny_unknown_fields`. Registry decode
rejects inputs above 131,072 bytes and situation decode above 32,768 bytes
before deserialization. Nested strings are capped at 1,024 bytes, IDs at 96
bytes, and each vector at 32 elements immediately after bounded decode and
before semantic traversal. The fixed registry then imposes smaller exact
counts. Serialization validates first; deserialization validates, reserializes,
and requires byte equality, rejecting trailing bytes, field reordering,
duplicate/unknown fields, noncanonical encodings and any authored-text drift.

## Hostile boundary

The implementation has no procedural generator, random dependency, weights,
probabilities, candidate selection, runtime adapter, UI, persistence,
filesystem, database, network, Greenfield, economy, GP2 progression map, GP4
vertical, C3B path, or Kernel mutation. It remains one additive module in
`mindwarp-gameplay-foundation`, whose existing GP0-GP2 behavior remains
unchanged.
