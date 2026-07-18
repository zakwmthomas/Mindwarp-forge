use crate::{
    CONTRACT_VERSION, EvidenceClass, FactKind, GameplayError, MemoryProposition, NamedDecision,
    OutcomeRecordV1, SessionRecordV1, TypedGrant, TypedMutation, WorldHistoryV1, fixed_sessions,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const MAX_GRAMMAR_BYTES: usize = 131_072;
pub const MAX_SITUATION_BYTES: usize = 32_768;
const MAX_NESTED_ITEMS: usize = 32;
const MAX_TEXT_BYTES: usize = 1_024;

pub const FIXED_SITUATION_DIGESTS: [&str; 5] = [
    "f2f804581e02364ce7632ca8307be1340935840a705f66a12fd47e497e19cc86",
    "c56fdd98459d0500ca0d8ad3c752ff98512f9770d4b79bab50c603286dda17c5",
    "08ead7accbe9b188888fd7a465dbe7e761c487162f1c7c6f2936771772167151",
    "c258b1b83e86cc52f30502c8e8d29d7bbda161ce7abc1031d5612a65c84d5328",
    "8c80d9b3a70c7ce77b82f78fe77e532b63b2ed6357f5f375c615b33111099766",
];
pub const FIXED_GRAMMAR_DIGEST: &str =
    "e8865e011d8b7ada0787303d49e4c769ff19164dc7a51f52d396e80b2c408b44";

const SESSION_DOMAIN: &[u8] = b"mindwarp.gp3.fixed-session.v1";
const FACT_DOMAIN: &[u8] = b"mindwarp.gp3.session-fact.v1";
const RISK_DOMAIN: &[u8] = b"mindwarp.gp3.risk.v1";
const THREAT_DOMAIN: &[u8] = b"mindwarp.gp3.threat.v1";
const SITUATION_DOMAIN: &[u8] = b"mindwarp.gp3.fixed-situation.v1";
const GRAMMAR_DOMAIN: &[u8] = b"mindwarp.gp3.fixed-grammar.v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainKindV1 {
    Environment,
    Creature,
    Society,
    Anomaly,
    Construction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DomainFacetV1 {
    Environment {
        facet_id: String,
        supporting_evidence_ids: Vec<String>,
        proposition: String,
    },
    Creature {
        facet_id: String,
        supporting_evidence_ids: Vec<String>,
        proposition: String,
    },
    Society {
        facet_id: String,
        supporting_evidence_ids: Vec<String>,
        proposition: String,
    },
    Anomaly {
        facet_id: String,
        supporting_evidence_ids: Vec<String>,
        proposition: String,
    },
    Construction {
        facet_id: String,
        supporting_evidence_ids: Vec<String>,
        proposition: String,
    },
}
impl DomainFacetV1 {
    pub fn kind(&self) -> DomainKindV1 {
        match self {
            Self::Environment { .. } => DomainKindV1::Environment,
            Self::Creature { .. } => DomainKindV1::Creature,
            Self::Society { .. } => DomainKindV1::Society,
            Self::Anomaly { .. } => DomainKindV1::Anomaly,
            Self::Construction { .. } => DomainKindV1::Construction,
        }
    }
    fn parts(&self) -> (&str, &[String], &str) {
        match self {
            Self::Environment {
                facet_id,
                supporting_evidence_ids,
                proposition,
            }
            | Self::Creature {
                facet_id,
                supporting_evidence_ids,
                proposition,
            }
            | Self::Society {
                facet_id,
                supporting_evidence_ids,
                proposition,
            }
            | Self::Anomaly {
                facet_id,
                supporting_evidence_ids,
                proposition,
            }
            | Self::Construction {
                facet_id,
                supporting_evidence_ids,
                proposition,
            } => (facet_id, supporting_evidence_ids, proposition),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterEvidenceRefV1 {
    pub fact_id: String,
    pub kind: FactKind,
    pub evidence_class: EvidenceClass,
    pub canonical_digest: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterRiskRefV1 {
    pub risk_id: String,
    pub canonical_digest: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApproachKindV1 {
    Intervention,
    Care,
    Negotiation,
    AlternateRoute,
    Construction,
    ForcePartial,
    Retreat,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StepKindV1 {
    Repair,
    Construct,
    Coordinate,
    Care,
    Extract,
    Negotiate,
    Traverse,
    Coerce,
    Dismantle,
    Withdraw,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InterventionStepV1 {
    pub step_id: String,
    pub kind: StepKindV1,
    pub subject_ids: Vec<String>,
    pub proposition: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrerequisiteKindV1 {
    ObservedFact,
    AvailableInference,
    PreparedTool,
    AuthoredState,
    ExactPredecessor,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ApproachPrerequisiteV1 {
    ObservedFact {
        reference_id: String,
        expected_digest: Option<String>,
    },
    AvailableInference {
        reference_id: String,
        expected_digest: Option<String>,
    },
    PreparedTool {
        reference_id: String,
        expected_digest: Option<String>,
    },
    AuthoredState {
        reference_id: String,
        expected_digest: Option<String>,
    },
    ExactPredecessor {
        reference_id: String,
        expected_digest: Option<String>,
        admitted_outcome_ids: Vec<String>,
        rejected_outcome_ids: Vec<String>,
    },
}
impl ApproachPrerequisiteV1 {
    pub fn kind(&self) -> PrerequisiteKindV1 {
        match self {
            Self::ObservedFact { .. } => PrerequisiteKindV1::ObservedFact,
            Self::AvailableInference { .. } => PrerequisiteKindV1::AvailableInference,
            Self::PreparedTool { .. } => PrerequisiteKindV1::PreparedTool,
            Self::AuthoredState { .. } => PrerequisiteKindV1::AuthoredState,
            Self::ExactPredecessor { .. } => PrerequisiteKindV1::ExactPredecessor,
        }
    }
    pub fn reference_id(&self) -> &str {
        match self {
            Self::ObservedFact { reference_id, .. }
            | Self::AvailableInference { reference_id, .. }
            | Self::PreparedTool { reference_id, .. }
            | Self::AuthoredState { reference_id, .. }
            | Self::ExactPredecessor { reference_id, .. } => reference_id,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskDispositionKindV1 {
    Resolved,
    Mitigated,
    Accepted,
    Transferred,
    Unchanged,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RiskDispositionV1 {
    pub disposition_id: String,
    pub risk_id: String,
    pub disposition: RiskDispositionKindV1,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceKindV1 {
    Mutation,
    OpportunityCost,
    Memory,
    Grant,
    NamedDecision,
}
impl ConsequenceKindV1 {
    fn label(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::OpportunityCost => "opportunity_cost",
            Self::Memory => "memory",
            Self::Grant => "grant",
            Self::NamedDecision => "named_decision",
        }
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsequenceRefV1 {
    pub kind: ConsequenceKindV1,
    pub ordinal: u16,
    pub canonical_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CausalExplanationV1 {
    pub admitted_evidence_ids: Vec<String>,
    pub intervention_step_ids: Vec<String>,
    pub consequence_ref_ids: Vec<String>,
    pub risk_disposition_ids: Vec<String>,
    pub explanation: String,
    pub limitation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterApproachV1 {
    pub approach_id: String,
    pub kind: ApproachKindV1,
    pub prepared_tool_id: Option<String>,
    pub intervention_steps: Vec<InterventionStepV1>,
    pub prerequisites: Vec<ApproachPrerequisiteV1>,
    pub risk_dispositions: Vec<RiskDispositionV1>,
    pub causal_explanation: CausalExplanationV1,
    pub outcome_id: String,
    pub consequence_refs: Vec<ConsequenceRefV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatMutationRefV1 {
    pub kind: ThreatContributionKindV1,
    pub ordinal: u16,
    pub canonical_digest: String,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatContributionKindV1 {
    ThreatContribution,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterThreatRefV1 {
    pub threat_id: String,
    pub canonical_digest: String,
    pub contribution_refs: Vec<ThreatMutationRefV1>,
    pub nonterminal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterSituationV1 {
    pub schema_version: u16,
    pub situation_id: String,
    pub session_id: String,
    pub session_digest: String,
    pub situation_digest: String,
    pub domain_facets: Vec<DomainFacetV1>,
    pub evidence_refs: Vec<EncounterEvidenceRefV1>,
    pub risk_refs: Vec<EncounterRiskRefV1>,
    pub approaches: Vec<EncounterApproachV1>,
    pub threat_ref: Option<EncounterThreatRefV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterGrammarV1 {
    pub schema_version: u16,
    pub grammar_digest: String,
    pub situations: Vec<EncounterSituationV1>,
}

impl EncounterSituationV1 {
    pub fn domain_kinds(&self) -> Vec<DomainKindV1> {
        self.domain_facets.iter().map(DomainFacetV1::kind).collect()
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, GameplayError> {
        self.validate_against_fixed()?;
        serde_json::to_vec(self).map_err(|e| GameplayError::Codec(e.to_string()))
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GameplayError> {
        if bytes.len() > MAX_SITUATION_BYTES {
            return Err(GameplayError::Invalid("oversized encounter situation"));
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|e| GameplayError::Codec(e.to_string()))?;
        value.validate_against_fixed()?;
        if serde_json::to_vec(&value).map_err(|e| GameplayError::Codec(e.to_string()))? != bytes {
            return Err(GameplayError::Invalid(
                "noncanonical encounter situation bytes",
            ));
        }
        Ok(value)
    }
    fn validate_against_fixed(&self) -> Result<(), GameplayError> {
        validate_situation_nested(self)?;
        let fixed = build_fixed_pinned()?;
        if !fixed.situations.iter().any(|s| s == self) {
            return Err(GameplayError::Invalid(
                "encounter situation differs from fixed registry",
            ));
        }
        Ok(())
    }
}

impl EncounterGrammarV1 {
    pub fn validate_against(&self, sessions: &[SessionRecordV1]) -> Result<(), GameplayError> {
        validate_nested(self)?;
        if sessions.len() != 5 || self != &build_fixed_pinned()? {
            return Err(GameplayError::Invalid(
                "encounter grammar differs from fixed registry",
            ));
        }
        for (situation, session) in self.situations.iter().zip(sessions) {
            validate_situation_authority(situation, session)?;
        }
        if digest_grammar(self)? != self.grammar_digest {
            return Err(GameplayError::Invalid("fixed grammar digest mismatch"));
        }
        Ok(())
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, GameplayError> {
        self.validate_against(&fixed_sessions())?;
        serde_json::to_vec(self).map_err(|e| GameplayError::Codec(e.to_string()))
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GameplayError> {
        if bytes.len() > MAX_GRAMMAR_BYTES {
            return Err(GameplayError::Invalid("oversized encounter grammar"));
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|e| GameplayError::Codec(e.to_string()))?;
        value.validate_against(&fixed_sessions())?;
        if serde_json::to_vec(&value).map_err(|e| GameplayError::Codec(e.to_string()))? != bytes {
            return Err(GameplayError::Invalid(
                "noncanonical encounter grammar bytes",
            ));
        }
        Ok(value)
    }
}

pub fn fixed_encounter_grammar() -> Result<EncounterGrammarV1, GameplayError> {
    let value = build_fixed_pinned()?;
    value.validate_against(&fixed_sessions())?;
    Ok(value)
}

pub fn resolve_outcome<'a>(
    session: &'a SessionRecordV1,
    approach: &EncounterApproachV1,
) -> Result<&'a OutcomeRecordV1, GameplayError> {
    validate_fixed_approach(session, approach)?;
    session.outcome(&approach.outcome_id)
}
pub fn validate_consequence_coverage(
    session: &SessionRecordV1,
    approach: &EncounterApproachV1,
) -> Result<(), GameplayError> {
    let outcome = resolve_outcome(session, approach)?;
    if consequence_refs(outcome)? != approach.consequence_refs {
        return Err(GameplayError::Invalid(
            "incomplete GP0 consequence references",
        ));
    }
    Ok(())
}
pub fn compose_optional_threat<'a>(
    session: &'a SessionRecordV1,
    situation: &EncounterSituationV1,
    approach: &EncounterApproachV1,
    selected: bool,
) -> Result<Vec<&'a TypedMutation>, GameplayError> {
    validate_fixed_situation(session, situation)?;
    validate_fixed_approach(session, approach)?;
    if !situation.approaches.iter().any(|a| a == approach) {
        return Err(GameplayError::Invalid(
            "approach does not belong to threat situation",
        ));
    }
    let pair = match (&situation.threat_ref, &session.threat_contribution) {
        (None, None) => {
            return if selected {
                Err(GameplayError::Invalid(
                    "situation has no threat contribution",
                ))
            } else {
                Ok(Vec::new())
            };
        }
        (Some(r), Some(t)) => (r, t),
        _ => return Err(GameplayError::Invalid("threat presence mismatch")),
    };
    let (reference, threat) = pair;
    if reference.threat_id != threat.threat_id
        || reference.canonical_digest != digest_value(THREAT_DOMAIN, threat)?
        || !reference.nonterminal
    {
        return Err(GameplayError::Invalid("threat contribution mismatch"));
    }
    let expected = threat
        .exact_mutations
        .iter()
        .enumerate()
        .map(|(i, m)| {
            Ok(ThreatMutationRefV1 {
                kind: ThreatContributionKindV1::ThreatContribution,
                ordinal: u16::try_from(i)
                    .map_err(|_| GameplayError::Invalid("threat ordinal overflow"))?,
                canonical_digest: digest_value(
                    b"mindwarp.gp3.consequence.threat-contribution.v1",
                    m,
                )?,
            })
        })
        .collect::<Result<Vec<_>, GameplayError>>()?;
    if expected != reference.contribution_refs {
        return Err(GameplayError::Invalid("threat element mismatch"));
    }
    if selected {
        Ok(threat.exact_mutations.iter().collect())
    } else {
        Ok(Vec::new())
    }
}

pub fn validate_approach_context(
    situation: &EncounterSituationV1,
    approach: &EncounterApproachV1,
    history: &WorldHistoryV1,
    predecessor: Option<&str>,
) -> Result<(), GameplayError> {
    history.validate()?;
    let sessions = fixed_sessions();
    let session = sessions
        .iter()
        .find(|s| s.session_id == situation.session_id)
        .ok_or(GameplayError::Invalid("foreign encounter session"))?;
    validate_fixed_situation(session, situation)?;
    validate_fixed_approach(session, approach)?;
    if !situation.approaches.iter().any(|a| a == approach) {
        return Err(GameplayError::Invalid(
            "approach does not belong to situation",
        ));
    }
    if situation.session_id != "gp0.s5.afterlight" {
        return if predecessor.is_none() {
            Ok(())
        } else {
            Err(GameplayError::Invalid("unexpected encounter predecessor"))
        };
    }
    let has_rule = approach.prerequisites.iter().any(|p| match p {
        ApproachPrerequisiteV1::ExactPredecessor {
            reference_id,
            expected_digest,
            admitted_outcome_ids,
            rejected_outcome_ids,
        } => {
            reference_id == "latest-gp0.s1.colony-conduit"
                && expected_digest.as_deref()
                    == Some("e7726be13efcf68e875e538103252aa46b3fd6c9e4ef86af95fc4622c160c274")
                && admitted_outcome_ids.iter().map(String::as_str).eq([
                    "s1.direct",
                    "s1.bypass",
                    "s1.ration",
                ])
                && rejected_outcome_ids
                    .iter()
                    .map(String::as_str)
                    .eq(["s1.retreat"])
        }
        _ => false,
    });
    if !has_rule {
        return Err(GameplayError::Invalid(
            "missing exact predecessor prerequisite",
        ));
    }
    let latest = history
        .events
        .iter()
        .rev()
        .find(|e| e.session_id == "gp0.s1.colony-conduit")
        .ok_or(GameplayError::Invalid("missing latest S1 predecessor"))?;
    let supplied = predecessor.ok_or(GameplayError::Invalid("missing encounter predecessor"))?;
    if latest.outcome_id != supplied || !["s1.direct", "s1.bypass", "s1.ration"].contains(&supplied)
    {
        return Err(GameplayError::Invalid(
            "stale or rejected encounter predecessor",
        ));
    }
    let s1 = &sessions[0];
    let outcome = s1.outcome(supplied)?;
    if latest.predecessor_outcome_id.is_some()
        || latest.exact_mutations != outcome.exact_mutations
        || latest.contributing_mutations != Vec::<TypedMutation>::new()
        || latest.opportunity_costs != outcome.opportunity_costs
        || latest.memories != outcome.memories
        || latest.grants != outcome.grants
        || latest.next_decision != outcome.next_decision
    {
        return Err(GameplayError::Invalid(
            "fabricated latest S1 predecessor event",
        ));
    }
    Ok(())
}

pub enum ResolvedConsequenceRefV1<'a> {
    Mutation(&'a TypedMutation),
    OpportunityCost(&'a TypedMutation),
    Memory(&'a MemoryProposition),
    Grant(&'a TypedGrant),
    NamedDecision(&'a NamedDecision),
}
pub fn resolve_consequence<'a>(
    session: &'a SessionRecordV1,
    approach: &EncounterApproachV1,
    reference: &ConsequenceRefV1,
) -> Result<ResolvedConsequenceRefV1<'a>, GameplayError> {
    validate_consequence_coverage(session, approach)?;
    if !approach.consequence_refs.contains(reference) {
        return Err(GameplayError::Invalid("foreign consequence reference"));
    }
    let outcome = session.outcome(&approach.outcome_id)?;
    let i = usize::from(reference.ordinal);
    match reference.kind {
        ConsequenceKindV1::Mutation => outcome
            .exact_mutations
            .get(i)
            .map(ResolvedConsequenceRefV1::Mutation),
        ConsequenceKindV1::OpportunityCost => outcome
            .opportunity_costs
            .get(i)
            .map(ResolvedConsequenceRefV1::OpportunityCost),
        ConsequenceKindV1::Memory => outcome
            .memories
            .get(i)
            .map(ResolvedConsequenceRefV1::Memory),
        ConsequenceKindV1::Grant => outcome.grants.get(i).map(ResolvedConsequenceRefV1::Grant),
        ConsequenceKindV1::NamedDecision if i == 0 => Some(
            ResolvedConsequenceRefV1::NamedDecision(&outcome.next_decision),
        ),
        ConsequenceKindV1::NamedDecision => None,
    }
    .ok_or(GameplayError::Invalid(
        "consequence reference ordinal mismatch",
    ))
}

fn validate_fixed_situation(
    session: &SessionRecordV1,
    situation: &EncounterSituationV1,
) -> Result<(), GameplayError> {
    let fixed = build_fixed_pinned()?;
    let expected = fixed
        .situations
        .iter()
        .find(|s| s.session_id == session.session_id)
        .ok_or(GameplayError::Invalid("foreign encounter session"))?;
    if situation != expected || situation.session_digest != digest_value(SESSION_DOMAIN, session)? {
        return Err(GameplayError::Invalid("caller-crafted encounter situation"));
    }
    Ok(())
}
fn validate_fixed_approach(
    session: &SessionRecordV1,
    approach: &EncounterApproachV1,
) -> Result<(), GameplayError> {
    let sessions = fixed_sessions();
    let canonical_session = sessions
        .iter()
        .find(|candidate| candidate.session_id == session.session_id)
        .ok_or(GameplayError::Invalid("foreign encounter session"))?;
    if session != canonical_session
        || digest_value(SESSION_DOMAIN, session)?
            != digest_value(SESSION_DOMAIN, canonical_session)?
    {
        return Err(GameplayError::Invalid("caller-crafted encounter session"));
    }
    let fixed = build_fixed_pinned()?;
    let expected = fixed
        .situations
        .iter()
        .find(|s| s.session_id == session.session_id)
        .ok_or(GameplayError::Invalid("foreign encounter session"))?;
    if !expected.approaches.iter().any(|a| a == approach) {
        return Err(GameplayError::Invalid("caller-crafted encounter approach"));
    }
    Ok(())
}

fn validate_situation_authority(
    situation: &EncounterSituationV1,
    session: &SessionRecordV1,
) -> Result<(), GameplayError> {
    if situation.schema_version != CONTRACT_VERSION
        || situation.session_id != session.session_id
        || situation.session_digest != digest_value(SESSION_DOMAIN, session)?
        || situation.situation_digest != digest_situation(situation)?
    {
        return Err(GameplayError::Invalid("situation authority mismatch"));
    }
    for reference in &situation.evidence_refs {
        let fact = session
            .facts
            .iter()
            .find(|f| f.fact_id == reference.fact_id)
            .ok_or(GameplayError::Invalid("foreign encounter evidence"))?;
        if reference.kind != fact.kind
            || reference.evidence_class != EvidenceClass::AuthoredGameplayNonC3B
            || reference.evidence_class != fact.evidence_class
            || reference.canonical_digest != digest_value(FACT_DOMAIN, fact)?
        {
            return Err(GameplayError::Invalid("encounter evidence mismatch"));
        }
    }
    for reference in &situation.risk_refs {
        let risk = session
            .risks
            .iter()
            .find(|r| r.risk_id == reference.risk_id)
            .ok_or(GameplayError::Invalid("foreign encounter risk"))?;
        if reference.canonical_digest != digest_value(RISK_DOMAIN, risk)? {
            return Err(GameplayError::Invalid("encounter risk mismatch"));
        }
    }
    for approach in &situation.approaches {
        validate_consequence_coverage(session, approach)?;
    }
    match (&situation.threat_ref, &session.threat_contribution) {
        (None, None) => {}
        (Some(_), Some(_)) => {
            for approach in &situation.approaches {
                compose_optional_threat(session, situation, approach, false)?;
                compose_optional_threat(session, situation, approach, true)?;
            }
        }
        _ => return Err(GameplayError::Invalid("encounter threat presence mismatch")),
    }
    Ok(())
}

fn validate_nested(grammar: &EncounterGrammarV1) -> Result<(), GameplayError> {
    if grammar.schema_version != CONTRACT_VERSION || grammar.situations.len() != 5 {
        return Err(GameplayError::Invalid("invalid encounter registry shape"));
    }
    bounded_digest(&grammar.grammar_digest)?;
    for situation in &grammar.situations {
        validate_situation_nested(situation)?;
    }
    Ok(())
}

fn validate_situation_nested(situation: &EncounterSituationV1) -> Result<(), GameplayError> {
    if situation.schema_version != CONTRACT_VERSION {
        return Err(GameplayError::Invalid("invalid encounter situation schema"));
    }
    for value in [&situation.situation_id, &situation.session_id] {
        bounded_id(value)?;
    }
    bounded_digest(&situation.session_digest)?;
    bounded_digest(&situation.situation_digest)?;
    bounded_vec(&situation.domain_facets)?;
    bounded_vec(&situation.evidence_refs)?;
    bounded_vec(&situation.risk_refs)?;
    bounded_vec(&situation.approaches)?;
    let evidence_ids = situation
        .evidence_refs
        .iter()
        .map(|e| e.fact_id.as_str())
        .collect::<BTreeSet<_>>();
    if evidence_ids.len() != situation.evidence_refs.len() {
        return Err(GameplayError::Invalid("duplicate encounter evidence"));
    }
    for evidence in &situation.evidence_refs {
        bounded_id(&evidence.fact_id)?;
        bounded_digest(&evidence.canonical_digest)?;
    }
    let risk_ids = situation
        .risk_refs
        .iter()
        .map(|r| r.risk_id.as_str())
        .collect::<BTreeSet<_>>();
    if risk_ids.len() != situation.risk_refs.len() {
        return Err(GameplayError::Invalid("duplicate encounter risk"));
    }
    for risk in &situation.risk_refs {
        bounded_id(&risk.risk_id)?;
        bounded_digest(&risk.canonical_digest)?;
    }
    for facet in &situation.domain_facets {
        let (id, support, text) = facet.parts();
        bounded_id(id)?;
        bounded_vec(support)?;
        bounded_text(text)?;
        for item in support {
            bounded_id(item)?;
            if !evidence_ids.contains(item.as_str()) {
                return Err(GameplayError::Invalid("facet lacks admitted evidence"));
            }
        }
    }
    let approach_ids = situation
        .approaches
        .iter()
        .map(|a| a.approach_id.as_str())
        .collect::<BTreeSet<_>>();
    if approach_ids.len() != situation.approaches.len() {
        return Err(GameplayError::Invalid("duplicate encounter approach"));
    }
    for approach in &situation.approaches {
        bounded_id(&approach.approach_id)?;
        bounded_id(&approach.outcome_id)?;
        if let Some(tool) = &approach.prepared_tool_id {
            bounded_id(tool)?;
        }
        bounded_vec(&approach.intervention_steps)?;
        bounded_vec(&approach.prerequisites)?;
        bounded_vec(&approach.risk_dispositions)?;
        bounded_vec(&approach.consequence_refs)?;
        for step in &approach.intervention_steps {
            bounded_id(&step.step_id)?;
            bounded_vec(&step.subject_ids)?;
            for subject in &step.subject_ids {
                bounded_id(subject)?;
            }
            bounded_text(&step.proposition)?;
        }
        for prerequisite in &approach.prerequisites {
            bounded_id(prerequisite.reference_id())?;
            match prerequisite {
                ApproachPrerequisiteV1::ObservedFact {
                    expected_digest, ..
                }
                | ApproachPrerequisiteV1::AvailableInference {
                    expected_digest, ..
                }
                | ApproachPrerequisiteV1::PreparedTool {
                    expected_digest, ..
                }
                | ApproachPrerequisiteV1::AuthoredState {
                    expected_digest, ..
                } => {
                    if let Some(d) = expected_digest {
                        bounded_digest(d)?;
                    }
                }
                ApproachPrerequisiteV1::ExactPredecessor {
                    expected_digest,
                    admitted_outcome_ids,
                    rejected_outcome_ids,
                    ..
                } => {
                    if let Some(d) = expected_digest {
                        bounded_digest(d)?;
                    }
                    bounded_vec(admitted_outcome_ids)?;
                    bounded_vec(rejected_outcome_ids)?;
                    for id in admitted_outcome_ids.iter().chain(rejected_outcome_ids) {
                        bounded_id(id)?;
                    }
                }
            }
        }
        for risk in &approach.risk_dispositions {
            bounded_id(&risk.disposition_id)?;
            bounded_id(&risk.risk_id)?;
            bounded_text(&risk.explanation)?;
        }
        let explanation = &approach.causal_explanation;
        bounded_vec(&explanation.admitted_evidence_ids)?;
        bounded_vec(&explanation.intervention_step_ids)?;
        bounded_vec(&explanation.consequence_ref_ids)?;
        bounded_vec(&explanation.risk_disposition_ids)?;
        for id in explanation
            .admitted_evidence_ids
            .iter()
            .chain(&explanation.intervention_step_ids)
            .chain(&explanation.consequence_ref_ids)
            .chain(&explanation.risk_disposition_ids)
        {
            bounded_id(id)?;
        }
        bounded_text(&explanation.explanation)?;
        bounded_text(&explanation.limitation)?;
        for reference in &approach.consequence_refs {
            bounded_digest(&reference.canonical_digest)?;
        }
    }
    if let Some(threat) = &situation.threat_ref {
        bounded_id(&threat.threat_id)?;
        bounded_digest(&threat.canonical_digest)?;
        bounded_vec(&threat.contribution_refs)?;
        for reference in &threat.contribution_refs {
            bounded_digest(&reference.canonical_digest)?;
        }
        if !threat.nonterminal {
            return Err(GameplayError::Invalid("terminal encounter threat"));
        }
    }
    Ok(())
}
fn bounded_id(value: &str) -> Result<(), GameplayError> {
    if value.is_empty()
        || value.len() > 96
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b))
    {
        return Err(GameplayError::Invalid("malformed encounter identifier"));
    }
    Ok(())
}
fn bounded_digest(value: &str) -> Result<(), GameplayError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(GameplayError::Invalid("malformed encounter digest"));
    }
    Ok(())
}
fn bounded_text(value: &str) -> Result<(), GameplayError> {
    if value.is_empty() || value.len() > MAX_TEXT_BYTES {
        return Err(GameplayError::Invalid("invalid encounter text"));
    }
    Ok(())
}
fn bounded_vec<T>(value: &[T]) -> Result<(), GameplayError> {
    if value.is_empty() || value.len() > MAX_NESTED_ITEMS {
        return Err(GameplayError::Invalid("invalid encounter vector bound"));
    }
    Ok(())
}

fn digest_value(domain: &[u8], value: &impl Serialize) -> Result<String, GameplayError> {
    let bytes = serde_json::to_vec(value).map_err(|e| GameplayError::Codec(e.to_string()))?;
    Ok(domain_digest(domain, &bytes))
}
fn domain_digest(domain: &[u8], bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update((domain.len() as u32).to_be_bytes());
    h.update(domain);
    h.update((bytes.len() as u64).to_be_bytes());
    h.update(bytes);
    format!("{:x}", h.finalize())
}
fn digest_situation(value: &EncounterSituationV1) -> Result<String, GameplayError> {
    let mut body = value.clone();
    body.situation_digest.clear();
    digest_value(SITUATION_DOMAIN, &body)
}
fn digest_grammar(value: &EncounterGrammarV1) -> Result<String, GameplayError> {
    let mut body = value.clone();
    body.grammar_digest.clear();
    digest_value(GRAMMAR_DOMAIN, &body)
}

fn consequence_refs(outcome: &OutcomeRecordV1) -> Result<Vec<ConsequenceRefV1>, GameplayError> {
    let mut refs = Vec::new();
    for (kind, domain, len) in [
        (
            ConsequenceKindV1::Mutation,
            b"mindwarp.gp3.consequence.mutation.v1" as &[u8],
            outcome.exact_mutations.len(),
        ),
        (
            ConsequenceKindV1::OpportunityCost,
            b"mindwarp.gp3.consequence.opportunity-cost.v1",
            outcome.opportunity_costs.len(),
        ),
        (
            ConsequenceKindV1::Memory,
            b"mindwarp.gp3.consequence.memory.v1",
            outcome.memories.len(),
        ),
        (
            ConsequenceKindV1::Grant,
            b"mindwarp.gp3.consequence.grant.v1",
            outcome.grants.len(),
        ),
    ] {
        for i in 0..len {
            let digest = match kind {
                ConsequenceKindV1::Mutation => digest_value(domain, &outcome.exact_mutations[i])?,
                ConsequenceKindV1::OpportunityCost => {
                    digest_value(domain, &outcome.opportunity_costs[i])?
                }
                ConsequenceKindV1::Memory => digest_value(domain, &outcome.memories[i])?,
                ConsequenceKindV1::Grant => digest_value(domain, &outcome.grants[i])?,
                ConsequenceKindV1::NamedDecision => unreachable!(),
            };
            refs.push(ConsequenceRefV1 {
                kind,
                ordinal: u16::try_from(i)
                    .map_err(|_| GameplayError::Invalid("consequence ordinal overflow"))?,
                canonical_digest: digest,
            });
        }
    }
    refs.push(ConsequenceRefV1 {
        kind: ConsequenceKindV1::NamedDecision,
        ordinal: 0,
        canonical_digest: digest_value(
            b"mindwarp.gp3.consequence.named-decision.v1",
            &outcome.next_decision,
        )?,
    });
    Ok(refs)
}

fn build_fixed_pinned() -> Result<EncounterGrammarV1, GameplayError> {
    let mut value = build_fixed_unpinned()?;
    for (s, d) in value.situations.iter_mut().zip(FIXED_SITUATION_DIGESTS) {
        s.situation_digest = d.into();
    }
    value.grammar_digest = FIXED_GRAMMAR_DIGEST.into();
    Ok(value)
}

fn build_fixed_unpinned() -> Result<EncounterGrammarV1, GameplayError> {
    let sessions = fixed_sessions();
    Ok(EncounterGrammarV1 {
        schema_version: CONTRACT_VERSION,
        grammar_digest: String::new(),
        situations: vec![
            build_situation(
                &sessions[0],
                "gp3.s1.colony-conduit",
                vec![
                    facet(
                        DomainKindV1::Environment,
                        "s1.facet.conduit-pressure",
                        &["s1.flow-loss"],
                        "Flow loss is localized at the failing colony conduit.",
                    ),
                    facet(
                        DomainKindV1::Creature,
                        "s1.facet.resident-colony",
                        &["s1.colony-distress"],
                        "Pump vibration threatens a resident colony without choosing the intervention.",
                    ),
                    facet(
                        DomainKindV1::Society,
                        "s1.facet.urgent-water",
                        &["s1.flow-loss"],
                        "Clinic and fire crews require an explainable water decision.",
                    ),
                    facet(
                        DomainKindV1::Construction,
                        "s1.facet.bypass-system",
                        &["s1.flow-loss", "s1.colony-distress"],
                        "Full repair, a spare bypass, and timed rationing impose different material obligations.",
                    ),
                ],
                vec![
                    spec(
                        "s1.approach.direct",
                        ApproachKindV1::Intervention,
                        Some("full-flow-kit"),
                        [
                            (
                                StepKindV1::Repair,
                                &["conduit", "pump"][..],
                                "Restore full pump flow",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["clinic-water", "fire-water"][..],
                                "Route urgent service",
                            ),
                        ],
                        "urgent-water-demand",
                        RiskDispositionKindV1::Resolved,
                        "Localized loss supports direct repair and full service while GP0 records displacement",
                        "It does not preserve the colony habitat",
                        "s1.direct",
                    ),
                    spec(
                        "s1.approach.bypass",
                        ApproachKindV1::Construction,
                        Some("colony-safe-kit"),
                        [
                            (
                                StepKindV1::Construct,
                                &["greenhouse-spare", "conduit"][..],
                                "Install the spare bypass",
                            ),
                            (
                                StepKindV1::Care,
                                &["colony"][..],
                                "Hold vibration below the authored distress condition",
                            ),
                        ],
                        "urgent-water-demand",
                        RiskDispositionKindV1::Mitigated,
                        "Flow and colony evidence support restricted stable bypass",
                        "It consumes the greenhouse spare and delays orchard recovery",
                        "s1.bypass",
                    ),
                    spec(
                        "s1.approach.ration",
                        ApproachKindV1::Intervention,
                        Some("timed-controller"),
                        [
                            (
                                StepKindV1::Coordinate,
                                &["water", "colony"][..],
                                "Synchronize delivery windows",
                            ),
                            (
                                StepKindV1::Repair,
                                &["conduit"][..],
                                "Contain rather than erase failure",
                            ),
                        ],
                        "urgent-water-demand",
                        RiskDispositionKindV1::Mitigated,
                        "Timed windows preserve the colony while containing the failing conduit",
                        "Supply remains constrained and repair is unfinished",
                        "s1.ration",
                    ),
                    retreat(
                        "s1.approach.retreat",
                        [(
                            StepKindV1::Withdraw,
                            &["player", "conduit"][..],
                            "Leave emergency ration active",
                        )],
                        RiskDispositionKindV1::Unchanged,
                        "Withdrawal preserves a stable stop through GP0 emergency rationing",
                        "The conduit remains unrepaired and orchard stress advances",
                        "s1.retreat",
                    ),
                ],
            )?,
            build_situation(
                &sessions[1],
                "gp3.s2.storm-nest",
                vec![
                    facet(
                        DomainKindV1::Environment,
                        "s2.facet.storm-ridge",
                        &["s2.exposure", "s2.crystal-hazard"],
                        "The exposed route and authored storm window make delay consequential.",
                    ),
                    facet(
                        DomainKindV1::Creature,
                        "s2.facet.brood-predator",
                        &["s2.exposure"],
                        "Brood exposure and predator approach are readable without making diversion terminal.",
                    ),
                    facet(
                        DomainKindV1::Society,
                        "s2.facet.caretaker-obligation",
                        &["s2.exposure"],
                        "The caretaker relationship distinguishes relocation from extraction.",
                    ),
                    facet(
                        DomainKindV1::Construction,
                        "s2.facet.nest-shelter",
                        &["s2.exposure", "s2.crystal-hazard"],
                        "Shelter placement must answer exposure and the named ridge hazard.",
                    ),
                ],
                vec![
                    spec(
                        "s2.approach.relocate",
                        ApproachKindV1::Care,
                        Some("sheltered-nest-kit"),
                        [
                            (
                                StepKindV1::Care,
                                &["brood", "nest"][..],
                                "Move the brood from exposure",
                            ),
                            (
                                StepKindV1::Construct,
                                &["nest", "shelter"][..],
                                "Stabilize the sheltered nest",
                            ),
                        ],
                        "storm-before-two-major-actions",
                        RiskDispositionKindV1::Resolved,
                        "Exposure and ridge evidence support relocation without requiring predator diversion",
                        "The old nest is abandoned",
                        "s2.relocate",
                    ),
                    spec(
                        "s2.approach.harvest",
                        ApproachKindV1::Intervention,
                        Some("insulated-specimen-kit"),
                        [
                            (
                                StepKindV1::Extract,
                                &["crystal-specimen", "ridge"][..],
                                "Take the named authored specimen",
                            ),
                            (
                                StepKindV1::Care,
                                &["brood"][..],
                                "Avoid direct harm while accepting displacement",
                            ),
                        ],
                        "storm-before-two-major-actions",
                        RiskDispositionKindV1::Transferred,
                        "Ridge evidence supports extraction with explicit brood displacement",
                        "Caretaker cooperation is withdrawn and no scientific authority is created",
                        "s2.harvest",
                    ),
                    retreat(
                        "s2.approach.retreat",
                        [
                            (
                                StepKindV1::Coordinate,
                                &["nest-caretaker", "brood"][..],
                                "Dispatch emergency stabilization",
                            ),
                            (
                                StepKindV1::Withdraw,
                                &["player", "nest"][..],
                                "Leave before the storm",
                            ),
                        ],
                        RiskDispositionKindV1::Transferred,
                        "Exposure supports a caretaker dispatch before withdrawal",
                        "Direct player assistance is foregone",
                        "s2.retreat",
                    ),
                ],
            )?,
            build_situation(
                &sessions[2],
                "gp3.s3.memory-gate",
                vec![
                    facet(
                        DomainKindV1::Environment,
                        "s3.facet.west-channel",
                        &["s3.ledger"],
                        "Full opening risks the west channel while essential passage remains necessary.",
                    ),
                    facet(
                        DomainKindV1::Society,
                        "s3.facet.contradictory-claims",
                        &["s3.ledger", "s3.testimony"],
                        "East and west claims remain separately legible and cannot be merged by assertion.",
                    ),
                    facet(
                        DomainKindV1::Construction,
                        "s3.facet.memory-gate",
                        &["s3.ledger", "s3.testimony"],
                        "The gate supports timed, forced-partial, alternate-route, or unchanged states.",
                    ),
                ],
                vec![
                    spec(
                        "s3.approach.charter",
                        ApproachKindV1::Negotiation,
                        Some("joint-ledger-kit"),
                        [
                            (
                                StepKindV1::Negotiate,
                                &["east-keeper", "west-keeper"][..],
                                "Preserve contradictory claims",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["gate", "passage-charter"][..],
                                "Assign timed windows and joint monitoring",
                            ),
                        ],
                        "essential-passage-required",
                        RiskDispositionKindV1::Mitigated,
                        "Ledger and testimony support timed passage without false agreement",
                        "Unrestricted passage is foregone",
                        "s3.charter",
                    ),
                    spec(
                        "s3.approach.force",
                        ApproachKindV1::ForcePartial,
                        Some("urgent-crossing-kit"),
                        [
                            (
                                StepKindV1::Coerce,
                                &["gate", "essential-traveller"][..],
                                "Complete one urgent crossing",
                            ),
                            (
                                StepKindV1::Withdraw,
                                &["gate", "player"][..],
                                "Reseal without ownership judgment",
                            ),
                        ],
                        "essential-passage-required",
                        RiskDispositionKindV1::Mitigated,
                        "Evidence permits only a forced partial crossing, not resolution",
                        "Ownership remains unresolved and cooperation is damaged",
                        "s3.force",
                    ),
                    spec(
                        "s3.approach.alternate",
                        ApproachKindV1::AlternateRoute,
                        Some("essential-path-kit"),
                        [
                            (
                                StepKindV1::Traverse,
                                &["alternate-path", "essential-travellers"][..],
                                "Mark essential-only travel",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["gate", "gate-watch"][..],
                                "Keep the disputed gate sealed",
                            ),
                        ],
                        "essential-passage-required",
                        RiskDispositionKindV1::Resolved,
                        "Contradictory claims and channel risk support an alternate route",
                        "Nonessential travel remains deferred",
                        "s3.alternate",
                    ),
                    retreat(
                        "s3.approach.retreat",
                        [(
                            StepKindV1::Withdraw,
                            &["player", "gate"][..],
                            "Leave both claims and gate unchanged",
                        )],
                        RiskDispositionKindV1::Unchanged,
                        "Withdrawal avoids inventing agreement or opening the gate",
                        "Essential travel waits and channel inspection is delayed",
                        "s3.retreat",
                    ),
                ],
            )?,
            build_situation(
                &sessions[3],
                "gp3.s4.signal-anchor",
                vec![
                    facet(
                        DomainKindV1::Environment,
                        "s4.facet.anchor-load",
                        &["s4.timing"],
                        "Anchor load and crossing conditions expose collapse risk.",
                    ),
                    facet(
                        DomainKindV1::Creature,
                        "s4.facet.wire-scavengers",
                        &["s4.wire-scavengers"],
                        "Wire scavengers obstruct work but their diversion cannot repair or rescue.",
                    ),
                    facet(
                        DomainKindV1::Society,
                        "s4.facet.iven-caravan",
                        &["s4.timing", "s4.wire-scavengers"],
                        "Iven's rescue and caravan service create distinct obligations.",
                    ),
                    facet(
                        DomainKindV1::Anomaly,
                        "s4.facet.signal-window",
                        &["s4.timing", "s4.event"],
                        "The authored three-action signal window competes with four-action permanent repair.",
                    ),
                    facet(
                        DomainKindV1::Construction,
                        "s4.facet.signal-anchor",
                        &["s4.timing", "s4.event"],
                        "Temporary brace, permanent repair, and detour preserve different consequences.",
                    ),
                ],
                vec![
                    spec(
                        "s4.approach.temporary",
                        ApproachKindV1::Construction,
                        Some("temporary-brace-kit"),
                        [
                            (
                                StepKindV1::Construct,
                                &["anchor", "brace"][..],
                                "Fit a two-action temporary brace",
                            ),
                            (
                                StepKindV1::Traverse,
                                &["iven", "crossing"][..],
                                "Return Iven and record the signal",
                            ),
                        ],
                        "three-action-signal-window",
                        RiskDispositionKindV1::Accepted,
                        "Timing evidence supports rescue and signal capture before brace expiry",
                        "Permanent repair is not completed and the caravan is delayed",
                        "s4.temporary-rescue",
                    ),
                    spec(
                        "s4.approach.permanent",
                        ApproachKindV1::Construction,
                        Some("permanent-anchor-kit"),
                        [
                            (
                                StepKindV1::Repair,
                                &["anchor", "crossing"][..],
                                "Complete permanent repair",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["caravan", "crossing"][..],
                                "Resume caravan service",
                            ),
                        ],
                        "four-action-permanent-repair",
                        RiskDispositionKindV1::Resolved,
                        "Anchor evidence supports permanent repair as a distinct priority",
                        "The signal expires and its coordinate is missed",
                        "s4.permanent",
                    ),
                    spec(
                        "s4.approach.long",
                        ApproachKindV1::AlternateRoute,
                        Some("north-route-kit"),
                        [
                            (
                                StepKindV1::Traverse,
                                &["north-detour", "iven"][..],
                                "Rescue Iven by the named detour",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["caravan", "north-detour"][..],
                                "Record the longer route",
                            ),
                        ],
                        "north-detour-available",
                        RiskDispositionKindV1::Resolved,
                        "Evidence supports rescue without loading the failed anchor",
                        "The signal expires and caravan delay extends",
                        "s4.long-route",
                    ),
                    retreat(
                        "s4.approach.retreat",
                        [
                            (
                                StepKindV1::Coordinate,
                                &["caravan-leader", "rescue-watch"][..],
                                "Schedule a later watch",
                            ),
                            (
                                StepKindV1::Withdraw,
                                &["player", "anchor"][..],
                                "Leave the failed anchor",
                            ),
                        ],
                        RiskDispositionKindV1::Unchanged,
                        "Withdrawal retains a visible signal expiry and later rescue decision",
                        "Iven's rescue and anchor repair are delayed",
                        "s4.retreat",
                    ),
                ],
            )?,
            build_situation(
                &sessions[4],
                "gp3.s5.afterlight",
                vec![
                    facet(
                        DomainKindV1::Environment,
                        "s5.facet.habitat-buffer",
                        &["s5.history", "s5.relocation"],
                        "Passage pressure is bounded by the colony habitat buffer.",
                    ),
                    facet(
                        DomainKindV1::Creature,
                        "s5.facet.colony-scavengers",
                        &["s5.relocation"],
                        "Colony state and food scavengers remain distinct living pressures.",
                    ),
                    facet(
                        DomainKindV1::Society,
                        "s5.facet.traveller-obligation",
                        &["s5.history"],
                        "Travellers and Mara must remember cleanup and habitat obligations.",
                    ),
                    facet(
                        DomainKindV1::Construction,
                        "s5.facet.nightway",
                        &["s5.history", "s5.relocation"],
                        "The passage must be chartered with obligations or dismantled.",
                    ),
                ],
                vec![
                    spec(
                        "s5.approach.nightway",
                        ApproachKindV1::Construction,
                        Some("nightway-charter-kit"),
                        [
                            (
                                StepKindV1::Construct,
                                &["nightway-boundary", "passage"][..],
                                "Mark the registered route",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["travellers", "cleanup"][..],
                                "Assign cleanup and habitat obligations",
                            ),
                        ],
                        "passage-obligations-active",
                        RiskDispositionKindV1::Mitigated,
                        "Exact latest S1 history and relocation evidence support a bounded charter",
                        "Unbounded travel is foregone and obligations remain enforceable",
                        "s5.nightway",
                    ),
                    spec(
                        "s5.approach.dismantle",
                        ApproachKindV1::Construction,
                        Some("passage-dismantling-kit"),
                        [
                            (
                                StepKindV1::Dismantle,
                                &["passage", "nightway-boundary"][..],
                                "Remove the passage",
                            ),
                            (
                                StepKindV1::Coordinate,
                                &["keeper-mara", "travellers"][..],
                                "Record closure and alternate service",
                            ),
                        ],
                        "passage-obligations-active",
                        RiskDispositionKindV1::Resolved,
                        "Exact latest S1 history supports dismantling when obligations fail",
                        "The traveller route closes",
                        "s5.dismantle",
                    ),
                    retreat(
                        "s5.approach.retreat",
                        [(
                            StepKindV1::Withdraw,
                            &["player", "passage"][..],
                            "Leave passage unresolved under temporary watch",
                        )],
                        RiskDispositionKindV1::Unchanged,
                        "Exact latest S1 history permits a stable withdrawal without erasing Afterlight",
                        "Scavengers remain active and buffer protection is delayed",
                        "s5.retreat",
                    ),
                ],
            )?,
        ],
    })
}

struct ApproachSpec {
    id: &'static str,
    kind: ApproachKindV1,
    tool: Option<&'static str>,
    steps: Vec<(StepKindV1, Vec<&'static str>, &'static str)>,
    state: &'static str,
    disposition: RiskDispositionKindV1,
    explanation: &'static str,
    limitation: &'static str,
    outcome: &'static str,
}
fn spec<const N: usize>(
    id: &'static str,
    kind: ApproachKindV1,
    tool: Option<&'static str>,
    steps: [(StepKindV1, &[&'static str], &'static str); N],
    state: &'static str,
    disposition: RiskDispositionKindV1,
    explanation: &'static str,
    limitation: &'static str,
    outcome: &'static str,
) -> ApproachSpec {
    ApproachSpec {
        id,
        kind,
        tool,
        steps: steps
            .into_iter()
            .map(|(k, s, p)| (k, s.to_vec(), p))
            .collect(),
        state,
        disposition,
        explanation,
        limitation,
        outcome,
    }
}
fn retreat<const N: usize>(
    id: &'static str,
    steps: [(StepKindV1, &[&'static str], &'static str); N],
    disposition: RiskDispositionKindV1,
    explanation: &'static str,
    limitation: &'static str,
    outcome: &'static str,
) -> ApproachSpec {
    spec(
        id,
        ApproachKindV1::Retreat,
        None,
        steps,
        "stable-withdrawal-available",
        disposition,
        explanation,
        limitation,
        outcome,
    )
}

fn facet(kind: DomainKindV1, id: &str, evidence: &[&str], proposition: &str) -> DomainFacetV1 {
    let values = (
        id.into(),
        evidence.iter().map(|s| (*s).into()).collect(),
        proposition.into(),
    );
    match kind {
        DomainKindV1::Environment => DomainFacetV1::Environment {
            facet_id: values.0,
            supporting_evidence_ids: values.1,
            proposition: values.2,
        },
        DomainKindV1::Creature => DomainFacetV1::Creature {
            facet_id: values.0,
            supporting_evidence_ids: values.1,
            proposition: values.2,
        },
        DomainKindV1::Society => DomainFacetV1::Society {
            facet_id: values.0,
            supporting_evidence_ids: values.1,
            proposition: values.2,
        },
        DomainKindV1::Anomaly => DomainFacetV1::Anomaly {
            facet_id: values.0,
            supporting_evidence_ids: values.1,
            proposition: values.2,
        },
        DomainKindV1::Construction => DomainFacetV1::Construction {
            facet_id: values.0,
            supporting_evidence_ids: values.1,
            proposition: values.2,
        },
    }
}

fn build_situation(
    session: &SessionRecordV1,
    id: &str,
    facets: Vec<DomainFacetV1>,
    specs: Vec<ApproachSpec>,
) -> Result<EncounterSituationV1, GameplayError> {
    let evidence_refs = session
        .facts
        .iter()
        .map(|f| {
            Ok(EncounterEvidenceRefV1 {
                fact_id: f.fact_id.clone(),
                kind: f.kind,
                evidence_class: f.evidence_class,
                canonical_digest: digest_value(FACT_DOMAIN, f)?,
            })
        })
        .collect::<Result<Vec<_>, GameplayError>>()?;
    let risk_refs = session
        .risks
        .iter()
        .map(|r| {
            Ok(EncounterRiskRefV1 {
                risk_id: r.risk_id.clone(),
                canonical_digest: digest_value(RISK_DOMAIN, r)?,
            })
        })
        .collect::<Result<Vec<_>, GameplayError>>()?;
    let approaches = specs
        .into_iter()
        .map(|s| build_approach(session, &evidence_refs, &risk_refs, s))
        .collect::<Result<Vec<_>, GameplayError>>()?;
    let threat_ref = session
        .threat_contribution
        .as_ref()
        .map(|t| {
            Ok(EncounterThreatRefV1 {
                threat_id: t.threat_id.clone(),
                canonical_digest: digest_value(THREAT_DOMAIN, t)?,
                contribution_refs: t
                    .exact_mutations
                    .iter()
                    .enumerate()
                    .map(|(i, m)| {
                        Ok(ThreatMutationRefV1 {
                            kind: ThreatContributionKindV1::ThreatContribution,
                            ordinal: u16::try_from(i)
                                .map_err(|_| GameplayError::Invalid("threat ordinal overflow"))?,
                            canonical_digest: digest_value(
                                b"mindwarp.gp3.consequence.threat-contribution.v1",
                                m,
                            )?,
                        })
                    })
                    .collect::<Result<Vec<_>, GameplayError>>()?,
                nonterminal: true,
            })
        })
        .transpose()?;
    Ok(EncounterSituationV1 {
        schema_version: CONTRACT_VERSION,
        situation_id: id.into(),
        session_id: session.session_id.clone(),
        session_digest: digest_value(SESSION_DOMAIN, session)?,
        situation_digest: String::new(),
        domain_facets: facets,
        evidence_refs,
        risk_refs,
        approaches,
        threat_ref,
    })
}

fn build_approach(
    session: &SessionRecordV1,
    evidence: &[EncounterEvidenceRefV1],
    risks: &[EncounterRiskRefV1],
    spec: ApproachSpec,
) -> Result<EncounterApproachV1, GameplayError> {
    let outcome = session.outcome(spec.outcome)?;
    let steps = spec
        .steps
        .iter()
        .enumerate()
        .map(|(i, (kind, subjects, text))| InterventionStepV1 {
            step_id: format!("{}.step.{}", spec.id, i + 1),
            kind: *kind,
            subject_ids: subjects.iter().map(|s| (*s).into()).collect(),
            proposition: (*text).into(),
        })
        .collect::<Vec<_>>();
    let mut prerequisites = Vec::new();
    if spec.kind != ApproachKindV1::Retreat {
        for item in evidence {
            prerequisites.push(match item.kind {
                FactKind::Observation => ApproachPrerequisiteV1::ObservedFact {
                    reference_id: item.fact_id.clone(),
                    expected_digest: Some(item.canonical_digest.clone()),
                },
                FactKind::Inference => ApproachPrerequisiteV1::AvailableInference {
                    reference_id: item.fact_id.clone(),
                    expected_digest: Some(item.canonical_digest.clone()),
                },
            });
        }
        prerequisites.push(ApproachPrerequisiteV1::PreparedTool {
            reference_id: spec.tool.unwrap().into(),
            expected_digest: None,
        });
    }
    prerequisites.push(ApproachPrerequisiteV1::AuthoredState {
        reference_id: spec.state.into(),
        expected_digest: None,
    });
    if session.session_id == "gp0.s5.afterlight" {
        prerequisites.push(ApproachPrerequisiteV1::ExactPredecessor {
            reference_id: "latest-gp0.s1.colony-conduit".into(),
            expected_digest: Some(
                "e7726be13efcf68e875e538103252aa46b3fd6c9e4ef86af95fc4622c160c274".into(),
            ),
            admitted_outcome_ids: vec!["s1.direct".into(), "s1.bypass".into(), "s1.ration".into()],
            rejected_outcome_ids: vec!["s1.retreat".into()],
        });
    }
    let risk_dispositions = risks
        .iter()
        .map(|r| RiskDispositionV1 {
            disposition_id: format!("{}.risk.{}", spec.id, r.risk_id),
            risk_id: r.risk_id.clone(),
            disposition: spec.disposition,
            explanation: spec.explanation.into(),
        })
        .collect::<Vec<_>>();
    let refs = consequence_refs(outcome)?;
    let ref_ids = refs
        .iter()
        .map(|r| format!("{}.{}.{}", outcome.outcome_id, r.kind.label(), r.ordinal))
        .collect();
    let causal_explanation = CausalExplanationV1 {
        admitted_evidence_ids: evidence.iter().map(|e| e.fact_id.clone()).collect(),
        intervention_step_ids: steps.iter().map(|s| s.step_id.clone()).collect(),
        consequence_ref_ids: ref_ids,
        risk_disposition_ids: risk_dispositions
            .iter()
            .map(|r| r.disposition_id.clone())
            .collect(),
        explanation: spec.explanation.into(),
        limitation: spec.limitation.into(),
    };
    Ok(EncounterApproachV1 {
        approach_id: spec.id.into(),
        kind: spec.kind,
        prepared_tool_id: spec.tool.map(Into::into),
        intervention_steps: steps,
        prerequisites,
        risk_dispositions,
        causal_explanation,
        outcome_id: spec.outcome.into(),
        consequence_refs: refs,
    })
}
