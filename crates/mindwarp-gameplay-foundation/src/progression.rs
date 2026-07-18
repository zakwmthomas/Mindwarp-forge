//! GP2 deterministic typed progression proof.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    Action, BaseLoopLedgerV1, BaseLoopStateV1, CONTRACT_VERSION, GameplayError, LoopPhaseV1,
    LoopWorldContextV1, SessionRecordV1, TypedGrant, fixed_sessions, validate_id,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeStatusV1 {
    Observed,
    Inferred,
    Corroborated,
    Superseded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeRecordV1 {
    pub record_id: String,
    pub proposition: String,
    pub status: KnowledgeStatusV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessKindV1 {
    Permission,
    Right,
    FulfilledService,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationStateV1 {
    Open,
    Fulfilled,
    Superseded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationV1 {
    pub obligation_id: String,
    pub proposition: String,
    pub state: ObligationStateV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccessRecordV1 {
    pub record_id: String,
    pub kind: AccessKindV1,
    pub issuer_id: String,
    pub scope: String,
    pub active: bool,
    pub obligations: Vec<ObligationV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipEventV1 {
    pub event_id: String,
    pub actor_id: String,
    pub proposition: String,
    pub commitment: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionRecordV1 {
    pub record_id: String,
    pub artifact_id: String,
    pub function_id: String,
    pub state_id: String,
    pub predecessor_state_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRecordV1 {
    pub record_id: String,
    pub capability_id: String,
    pub horizontal_scope: String,
    pub source_tool_id: String,
    pub source_outcome_id: String,
    pub applicable_decision_id: String,
    pub limitation: String,
    pub source_run_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NamedAssetV1 {
    pub asset_id: String,
    pub world_subject_id: String,
    pub function_id: String,
    pub state_id: String,
    pub custodian_id: String,
    pub predecessor_asset_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LiabilityRecordV1 {
    pub record_id: String,
    pub subject_id: String,
    pub field_id: String,
    pub value_id: String,
    pub state: ObligationStateV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProgressionReceiptV1 {
    pub run_id: String,
    pub event_sequence: u32,
    pub session_id: String,
    pub outcome_id: String,
    pub terminal_state_digest: [u8; 32],
    pub terminal_state_bytes: Vec<u8>,
    pub session_record_digest: [u8; 32],
    pub rule_registry_digest: [u8; 32],
    pub rule_id: String,
    pub emitted_record_ids: Vec<String>,
    pub world_transition_ids: Vec<String>,
    pub opened_decision_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProgressionLedgerV1 {
    pub schema_version: u16,
    pub source_base_loop_ledger_digest: [u8; 32],
    pub rule_registry_digest: [u8; 32],
    pub processed_receipts: Vec<ProgressionReceiptV1>,
    pub knowledge: Vec<KnowledgeRecordV1>,
    pub access: Vec<AccessRecordV1>,
    pub relationship_events: Vec<RelationshipEventV1>,
    pub constructions: Vec<ConstructionRecordV1>,
    pub capabilities: Vec<CapabilityRecordV1>,
    pub named_assets: Vec<NamedAssetV1>,
    pub liabilities: Vec<LiabilityRecordV1>,
}

impl ProgressionLedgerV1 {
    pub fn from_base_loop(ledger: &BaseLoopLedgerV1) -> Result<Self, GameplayError> {
        ledger.validate()?;
        Ok(Self {
            schema_version: CONTRACT_VERSION,
            source_base_loop_ledger_digest: digest_ledger(ledger)?,
            rule_registry_digest: digest_registry()?,
            processed_receipts: vec![],
            knowledge: vec![],
            access: vec![],
            relationship_events: vec![],
            constructions: vec![],
            capabilities: vec![],
            named_assets: vec![],
            liabilities: vec![],
        })
    }
    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION
            || self.source_base_loop_ledger_digest == [0; 32]
            || self.rule_registry_digest != digest_registry()?
        {
            return Err(GameplayError::Invalid(
                "invalid progression ledger identity",
            ));
        }
        if self.processed_receipts.len() > 256
            || self.knowledge.len() > 256
            || self.access.len() > 256
            || self.relationship_events.len() > 256
            || self.constructions.len() > 256
            || self.capabilities.len() > 256
            || self.named_assets.len() > 256
            || self.liabilities.len() > 256
        {
            return Err(GameplayError::Invalid("progression ledger bound exceeded"));
        }
        let mut runs = BTreeSet::new();
        let mut receipt_records = BTreeSet::new();
        for receipt in &self.processed_receipts {
            for id in [
                &receipt.run_id,
                &receipt.session_id,
                &receipt.outcome_id,
                &receipt.rule_id,
                &receipt.opened_decision_id,
            ] {
                validate_id(id)?;
            }
            if receipt.terminal_state_digest == [0; 32]
                || receipt.terminal_state_bytes.is_empty()
                || receipt.terminal_state_bytes.len() > 262_144
                || receipt.session_record_digest == [0; 32]
                || receipt.rule_registry_digest != self.rule_registry_digest
                || !runs.insert(&receipt.run_id)
            {
                return Err(GameplayError::Invalid(
                    "duplicate or invalid progression receipt",
                ));
            }
            for id in &receipt.emitted_record_ids {
                validate_id(id)?;
                if !receipt_records.insert(id.clone()) {
                    return Err(GameplayError::Invalid("duplicate progression record"));
                }
            }
            for id in &receipt.world_transition_ids {
                validate_id(id)?;
            }
        }
        let mut lane_records = BTreeSet::new();
        for item in &self.knowledge {
            validate_record(&item.record_id, &item.proposition, &mut lane_records)?;
        }
        for item in &self.access {
            validate_record(&item.record_id, &item.scope, &mut lane_records)?;
            validate_id(&item.issuer_id)?;
            if matches!(item.kind, AccessKindV1::FulfilledService) && item.active {
                return Err(GameplayError::Invalid(
                    "fulfilled service cannot remain active",
                ));
            }
            for obligation in &item.obligations {
                validate_id(&obligation.obligation_id)?;
                validate_text(&obligation.proposition)?;
            }
        }
        for item in &self.relationship_events {
            validate_record(&item.event_id, &item.proposition, &mut lane_records)?;
            validate_id(&item.actor_id)?;
            validate_text(&item.commitment)?;
        }
        for item in &self.constructions {
            validate_record(&item.record_id, &item.state_id, &mut lane_records)?;
            validate_id(&item.artifact_id)?;
            validate_id(&item.function_id)?;
        }
        for item in &self.capabilities {
            validate_record(&item.record_id, &item.horizontal_scope, &mut lane_records)?;
            validate_id(&item.capability_id)?;
            validate_id(&item.source_tool_id)?;
            validate_id(&item.source_outcome_id)?;
            validate_id(&item.applicable_decision_id)?;
            validate_text(&item.limitation)?;
            validate_id(&item.source_run_id)?;
        }
        for item in &self.named_assets {
            validate_record(&item.asset_id, &item.state_id, &mut lane_records)?;
            validate_id(&item.world_subject_id)?;
            validate_id(&item.function_id)?;
            validate_id(&item.custodian_id)?;
        }
        for item in &self.liabilities {
            validate_record(&item.record_id, &item.value_id, &mut lane_records)?;
            validate_id(&item.subject_id)?;
            validate_id(&item.field_id)?;
        }
        if lane_records != receipt_records {
            return Err(GameplayError::Invalid(
                "progression receipt emission mismatch",
            ));
        }
        Ok(())
    }
    pub fn validate_against(&self, source: &BaseLoopLedgerV1) -> Result<(), GameplayError> {
        self.validate()?;
        source.validate()?;
        if self.source_base_loop_ledger_digest != digest_ledger(source)? {
            return Err(GameplayError::Invalid(
                "progression source ledger digest mismatch",
            ));
        }
        if self.processed_receipts.is_empty() {
            return Ok(());
        }
        let first = &self.processed_receipts[0];
        let first_session = canonical_session(&first.session_id)?;
        let first_state = BaseLoopStateV1::from_bytes(
            &first_session,
            &LoopWorldContextV1::AuthoredFixture,
            &first.terminal_state_bytes,
        )?;
        let mut expected = Self::from_base_loop(&first_state.ledger_before)?;
        let mut last_sequence = 0;
        for receipt in &self.processed_receipts {
            if receipt.event_sequence <= last_sequence {
                return Err(GameplayError::Invalid("progression receipt order mismatch"));
            }
            last_sequence = receipt.event_sequence;
            let session = canonical_session(&receipt.session_id)?;
            let state = BaseLoopStateV1::from_bytes(
                &session,
                &LoopWorldContextV1::AuthoredFixture,
                &receipt.terminal_state_bytes,
            )?;
            if digest_state(&session, &state)? != receipt.terminal_state_digest {
                return Err(GameplayError::Invalid(
                    "progression terminal digest mismatch",
                ));
            }
            expected = apply_progression_inner(&session, &state, &expected, false)?;
        }
        if &expected != self {
            return Err(GameplayError::Invalid(
                "progression historical projection mismatch",
            ));
        }
        let final_receipt = self.processed_receipts.last().unwrap();
        let final_session = canonical_session(&final_receipt.session_id)?;
        let final_state = BaseLoopStateV1::from_bytes(
            &final_session,
            &LoopWorldContextV1::AuthoredFixture,
            &final_receipt.terminal_state_bytes,
        )?;
        if final_state.ledger_after != *source {
            return Err(GameplayError::Invalid("progression final history mismatch"));
        }
        Ok(())
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, GameplayError> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|e| GameplayError::Codec(e.to_string()))?;
        if bytes.len() > 1_048_576 {
            return Err(GameplayError::Invalid(
                "progression ledger byte bound exceeded",
            ));
        }
        Ok(bytes)
    }
    pub fn from_bytes(source: &BaseLoopLedgerV1, bytes: &[u8]) -> Result<Self, GameplayError> {
        if bytes.len() > 1_048_576 {
            return Err(GameplayError::Invalid(
                "progression ledger byte bound exceeded",
            ));
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|e| GameplayError::Codec(e.to_string()))?;
        value.validate_against(source)?;
        if value.to_bytes()? != bytes {
            return Err(GameplayError::Invalid(
                "noncanonical progression ledger bytes",
            ));
        }
        Ok(value)
    }
}

fn canonical_session(session_id: &str) -> Result<SessionRecordV1, GameplayError> {
    fixed_sessions()
        .into_iter()
        .find(|item| item.session_id == session_id)
        .ok_or(GameplayError::Invalid(
            "progression receipt session missing",
        ))
}

fn validate_text(value: &str) -> Result<(), GameplayError> {
    if value.is_empty() || value.len() > 512 {
        return Err(GameplayError::Invalid("invalid progression text"));
    }
    Ok(())
}
fn validate_record(id: &str, text: &str, ids: &mut BTreeSet<String>) -> Result<(), GameplayError> {
    validate_id(id)?;
    validate_text(text)?;
    if !ids.insert(id.to_owned()) {
        return Err(GameplayError::Invalid("duplicate progression record"));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProgressionRuleV1 {
    rule_id: &'static str,
    session_id: &'static str,
    outcome_id: &'static str,
    required_tool_id: Option<&'static str>,
    capability_id: Option<&'static str>,
    knowledge: bool,
    access: bool,
    construction_subject: Option<&'static str>,
    construction_predecessor: Option<&'static str>,
    asset_subject: Option<&'static str>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProgressionRuleRegistryV1 {
    rules: Vec<ProgressionRuleV1>,
}

fn fixed_progression_rules() -> &'static ProgressionRuleRegistryV1 {
    static REGISTRY: std::sync::OnceLock<ProgressionRuleRegistryV1> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| ProgressionRuleRegistryV1 {
        rules: vec![
            ProgressionRuleV1 {
                rule_id: "gp2.s1.direct",
                session_id: "gp0.s1.colony-conduit",
                outcome_id: "s1.direct",
                required_tool_id: Some("full-flow-kit"),
                capability_id: Some("emergency-restoration"),
                knowledge: true,
                access: true,
                construction_subject: Some("pump"),
                construction_predecessor: Some("failing"),
                asset_subject: None,
            },
            ProgressionRuleV1 {
                rule_id: "gp2.s1.bypass",
                session_id: "gp0.s1.colony-conduit",
                outcome_id: "s1.bypass",
                required_tool_id: Some("colony-safe-kit"),
                capability_id: Some("bypass-installation"),
                knowledge: true,
                access: true,
                construction_subject: Some("pump"),
                construction_predecessor: Some("failing"),
                asset_subject: None,
            },
            ProgressionRuleV1 {
                rule_id: "gp2.s1.ration",
                session_id: "gp0.s1.colony-conduit",
                outcome_id: "s1.ration",
                required_tool_id: Some("timed-controller"),
                capability_id: Some("synchronized-scheduling"),
                knowledge: true,
                access: true,
                construction_subject: Some("conduit"),
                construction_predecessor: Some("failing"),
                asset_subject: None,
            },
            ProgressionRuleV1 {
                rule_id: "gp2.s1.retreat",
                session_id: "gp0.s1.colony-conduit",
                outcome_id: "s1.retreat",
                required_tool_id: None,
                capability_id: None,
                knowledge: false,
                access: true,
                construction_subject: None,
                construction_predecessor: None,
                asset_subject: None,
            },
            r(
                "gp2.s2.relocate",
                "gp0.s2.storm-nest",
                "s2.relocate",
                true,
                false,
                Some("nest"),
                Some("exposed"),
                None,
            ),
            r(
                "gp2.s2.harvest",
                "gp0.s2.storm-nest",
                "s2.harvest",
                true,
                false,
                None,
                None,
                Some("crystal-specimen"),
            ),
            r(
                "gp2.s2.retreat",
                "gp0.s2.storm-nest",
                "s2.retreat",
                false,
                true,
                None,
                None,
                None,
            ),
            r(
                "gp2.s3.charter",
                "gp0.s3.memory-gate",
                "s3.charter",
                true,
                true,
                Some("passage-charter"),
                Some("absent"),
                None,
            ),
            r(
                "gp2.s3.force",
                "gp0.s3.memory-gate",
                "s3.force",
                true,
                true,
                None,
                None,
                None,
            ),
            r(
                "gp2.s3.alternate",
                "gp0.s3.memory-gate",
                "s3.alternate",
                true,
                true,
                Some("alternate-path"),
                Some("absent"),
                None,
            ),
            r(
                "gp2.s3.retreat",
                "gp0.s3.memory-gate",
                "s3.retreat",
                false,
                true,
                None,
                None,
                None,
            ),
            r(
                "gp2.s4.rescue",
                "gp0.s4.signal-anchor",
                "s4.temporary-rescue",
                true,
                false,
                Some("anchor"),
                Some("broken"),
                Some("signal"),
            ),
            r(
                "gp2.s4.permanent",
                "gp0.s4.signal-anchor",
                "s4.permanent",
                true,
                true,
                Some("anchor"),
                Some("broken"),
                None,
            ),
            r(
                "gp2.s4.long",
                "gp0.s4.signal-anchor",
                "s4.long-route",
                true,
                false,
                None,
                None,
                Some("north-detour"),
            ),
            r(
                "gp2.s4.retreat",
                "gp0.s4.signal-anchor",
                "s4.retreat",
                false,
                true,
                None,
                None,
                None,
            ),
            r(
                "gp2.s5.nightway",
                "gp0.s5.afterlight",
                "s5.nightway",
                true,
                true,
                Some("passage"),
                Some("unregistered"),
                None,
            ),
            r(
                "gp2.s5.dismantle",
                "gp0.s5.afterlight",
                "s5.dismantle",
                true,
                true,
                Some("passage"),
                Some("registered"),
                None,
            ),
            r(
                "gp2.s5.retreat",
                "gp0.s5.afterlight",
                "s5.retreat",
                false,
                true,
                None,
                None,
                None,
            ),
        ],
    })
}

const fn r(
    rule_id: &'static str,
    session_id: &'static str,
    outcome_id: &'static str,
    knowledge: bool,
    access: bool,
    construction_subject: Option<&'static str>,
    construction_predecessor: Option<&'static str>,
    asset_subject: Option<&'static str>,
) -> ProgressionRuleV1 {
    ProgressionRuleV1 {
        rule_id,
        session_id,
        outcome_id,
        required_tool_id: None,
        capability_id: None,
        knowledge,
        access,
        construction_subject,
        construction_predecessor,
        asset_subject,
    }
}
pub fn fixed_progression_rule_count() -> usize {
    fixed_progression_rules().rules.len()
}
pub fn fixed_progression_rule_keys() -> Vec<(String, String)> {
    fixed_progression_rules()
        .rules
        .iter()
        .map(|rule| (rule.session_id.into(), rule.outcome_id.into()))
        .collect()
}
pub fn conversion_rule_count() -> usize {
    0
}
pub fn reset_rule_count() -> usize {
    0
}

pub fn apply_progression(
    record: &SessionRecordV1,
    state: &BaseLoopStateV1,
    prior: &ProgressionLedgerV1,
) -> Result<ProgressionLedgerV1, GameplayError> {
    apply_progression_inner(record, state, prior, true)
}

fn apply_progression_inner(
    record: &SessionRecordV1,
    state: &BaseLoopStateV1,
    prior: &ProgressionLedgerV1,
    validate_output: bool,
) -> Result<ProgressionLedgerV1, GameplayError> {
    if state.world_context != LoopWorldContextV1::AuthoredFixture {
        return Err(GameplayError::Invalid(
            "GP2 V1 requires authored fixture context",
        ));
    }
    let bytes = state.to_bytes(record)?;
    let state = BaseLoopStateV1::from_bytes(record, &LoopWorldContextV1::AuthoredFixture, &bytes)?;
    let canonical = fixed_sessions()
        .into_iter()
        .find(|item| item.session_id == record.session_id)
        .ok_or(GameplayError::Invalid(
            "session has no fixed progression authority",
        ))?;
    if &canonical != record {
        return Err(GameplayError::Invalid(
            "fixed session record digest mismatch",
        ));
    }
    if validate_output {
        prior.validate_against(&state.ledger_before)?;
    }
    if state.phase != LoopPhaseV1::RememberedResponse {
        return Err(GameplayError::Invalid(
            "progression requires terminal remembered response",
        ));
    }
    if prior.source_base_loop_ledger_digest != digest_ledger(&state.ledger_before)? {
        return Err(GameplayError::Invalid(
            "progression source ledger digest mismatch",
        ));
    }
    if prior
        .processed_receipts
        .iter()
        .any(|item| item.run_id == state.run_id)
    {
        return Err(GameplayError::Invalid("progression run already processed"));
    }
    if state
        .ledger_before
        .world_history
        .events
        .iter()
        .any(|event| event.session_id == record.session_id)
    {
        return Err(GameplayError::Invalid(
            "progression requires new authored causal state",
        ));
    }
    let outcome_id = state
        .session_state
        .selected_outcome_id
        .as_deref()
        .ok_or(GameplayError::Invalid("progression outcome missing"))?;
    let rule = fixed_progression_rules()
        .rules
        .iter()
        .find(|rule| rule.session_id == record.session_id && rule.outcome_id == outcome_id)
        .ok_or(GameplayError::Invalid("no fixed progression rule"))?;
    let base_receipt = state
        .ledger_after
        .completed_runs
        .iter()
        .find(|item| item.run_id == state.run_id)
        .ok_or(GameplayError::Invalid("base loop receipt missing"))?;
    let prefix = outcome_id.replace('.', "-");
    let mut next = prior.clone();
    let mut emitted = Vec::new();
    if rule.knowledge && state.session_state.trace.contains(&Action::ObserveCause) {
        let proposition = record
            .facts
            .iter()
            .find(|fact| fact.kind == crate::FactKind::Observation)
            .ok_or(GameplayError::Invalid("knowledge rule lacks observation"))?
            .proposition
            .clone();
        let item = KnowledgeRecordV1 {
            record_id: format!("knowledge.{prefix}"),
            proposition,
            status: KnowledgeStatusV1::Observed,
        };
        emitted.push(item.record_id.clone());
        next.knowledge.push(item);
    }
    for (index, grant) in state.session_state.grants.iter().enumerate() {
        if let TypedGrant::Knowledge { proposition, .. } = grant {
            if rule.knowledge {
                let item = KnowledgeRecordV1 {
                    record_id: format!("knowledge.{prefix}.grant-{index}"),
                    proposition: proposition.clone(),
                    status: KnowledgeStatusV1::Corroborated,
                };
                emitted.push(item.record_id.clone());
                next.knowledge.push(item);
            }
            continue;
        }
        if !rule.access {
            continue;
        }
        let (kind, issuer, scope) = match grant {
            TypedGrant::Permission {
                grantor_id,
                proposition,
            } => (AccessKindV1::Permission, grantor_id, proposition),
            TypedGrant::Right {
                holder_id,
                proposition,
            } => (AccessKindV1::Right, holder_id, proposition),
            TypedGrant::Service {
                provider_id,
                proposition,
            } => (AccessKindV1::FulfilledService, provider_id, proposition),
            TypedGrant::Knowledge { .. } => unreachable!(),
        };
        let active = matches!(kind, AccessKindV1::Right)
            && matches!(outcome_id, "s3.charter" | "s3.alternate" | "s5.nightway");
        let item = AccessRecordV1 {
            record_id: format!("access.{prefix}.{index}"),
            kind,
            issuer_id: issuer.clone(),
            scope: scope.clone(),
            active,
            obligations: vec![ObligationV1 {
                obligation_id: format!("obligation.{prefix}.{index}"),
                proposition: state
                    .session_state
                    .next_decision
                    .as_ref()
                    .unwrap()
                    .proposition
                    .clone(),
                state: if active {
                    ObligationStateV1::Open
                } else {
                    ObligationStateV1::Fulfilled
                },
            }],
        };
        emitted.push(item.record_id.clone());
        next.access.push(item);
    }
    for (index, memory) in state.session_state.memories.iter().enumerate() {
        let item = RelationshipEventV1 {
            event_id: format!("relationship.{prefix}.{index}"),
            actor_id: memory.rememberer_id.clone(),
            proposition: memory.proposition.clone(),
            commitment: state
                .session_state
                .next_decision
                .as_ref()
                .unwrap()
                .proposition
                .clone(),
        };
        emitted.push(item.event_id.clone());
        next.relationship_events.push(item);
    }
    for (index, mutation) in state
        .session_state
        .exact_mutations
        .iter()
        .enumerate()
        .filter(|(_, mutation)| rule.construction_subject == Some(mutation.subject_id.as_str()))
    {
        let item = ConstructionRecordV1 {
            record_id: format!("construction.{prefix}.{index}"),
            artifact_id: mutation.subject_id.clone(),
            function_id: mutation.field_id.clone(),
            state_id: mutation.value_id.clone(),
            predecessor_state_id: rule.construction_predecessor.map(str::to_owned),
        };
        emitted.push(item.record_id.clone());
        next.constructions.push(item);
    }
    if let Some(subject) = rule.asset_subject {
        let mutation = state
            .session_state
            .exact_mutations
            .iter()
            .find(|item| item.subject_id == subject)
            .ok_or(GameplayError::Invalid(
                "asset rule lacks exact world subject",
            ))?;
        let item = NamedAssetV1 {
            asset_id: format!("asset.{prefix}"),
            world_subject_id: subject.into(),
            function_id: mutation.field_id.clone(),
            state_id: mutation.value_id.clone(),
            custodian_id: "player".into(),
            predecessor_asset_id: None,
        };
        emitted.push(item.asset_id.clone());
        next.named_assets.push(item);
    }
    for (index, cost) in state.session_state.opportunity_costs.iter().enumerate() {
        let item = LiabilityRecordV1 {
            record_id: format!("liability.{prefix}.{index}"),
            subject_id: cost.subject_id.clone(),
            field_id: cost.field_id.clone(),
            value_id: cost.value_id.clone(),
            state: ObligationStateV1::Open,
        };
        emitted.push(item.record_id.clone());
        next.liabilities.push(item);
    }
    if state.session_state.core_tension_resolved
        && let (Some(required), Some(capability)) = (rule.required_tool_id, rule.capability_id)
        && state.preparation.as_ref().map(|p| p.tool_id.as_str()) == Some(required)
    {
        let item = CapabilityRecordV1 {
            record_id: format!("capability.{prefix}"),
            capability_id: capability.into(),
            horizontal_scope: format!("technique for {outcome_id}"),
            source_tool_id: required.into(),
            source_outcome_id: outcome_id.into(),
            applicable_decision_id: state.session_state.next_decision.as_ref().unwrap().decision_id.clone(),
            limitation: "applies only to the named authored causal technique; it grants no general authority".into(),
            source_run_id: state.run_id.clone(),
        };
        emitted.push(item.record_id.clone());
        next.capabilities.push(item);
    }
    next.processed_receipts.push(ProgressionReceiptV1 {
        run_id: state.run_id.clone(),
        event_sequence: base_receipt.event_sequence,
        session_id: record.session_id.clone(),
        outcome_id: outcome_id.into(),
        terminal_state_digest: digest_state(record, &state)?,
        terminal_state_bytes: state.to_bytes(record)?,
        session_record_digest: digest_session(record)?,
        rule_registry_digest: digest_registry()?,
        rule_id: rule.rule_id.into(),
        emitted_record_ids: emitted,
        world_transition_ids: state
            .session_state
            .exact_mutations
            .iter()
            .map(|m| format!("{}.{}.{}", m.subject_id, m.field_id, m.value_id))
            .collect(),
        opened_decision_id: state
            .session_state
            .next_decision
            .as_ref()
            .unwrap()
            .decision_id
            .clone(),
    });
    next.source_base_loop_ledger_digest = digest_ledger(&state.ledger_after)?;
    if validate_output {
        next.validate_against(&state.ledger_after)?;
    }
    Ok(next)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyKindV1 {
    StewardBuilder,
    UrgencyDiscovery,
    CautiousMapper,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyResultV1 {
    pub strategy: StrategyKindV1,
    pub active_affordances: BTreeSet<String>,
    pub reachable_decisions: BTreeSet<String>,
    pub liabilities: BTreeSet<String>,
    meaningful_affordances: BTreeSet<&'static str>,
    meaningful_decisions: BTreeSet<&'static str>,
}
pub fn simulate_strategies() -> Result<Vec<StrategyResultV1>, GameplayError> {
    let specifications = [
        (
            StrategyKindV1::StewardBuilder,
            [
                "gp2.s1.bypass",
                "gp2.s2.relocate",
                "gp2.s3.charter",
                "gp2.s4.permanent",
                "gp2.s5.nightway",
            ],
        ),
        (
            StrategyKindV1::UrgencyDiscovery,
            [
                "gp2.s1.direct",
                "gp2.s2.harvest",
                "gp2.s3.force",
                "gp2.s4.rescue",
                "gp2.s5.dismantle",
            ],
        ),
        (
            StrategyKindV1::CautiousMapper,
            [
                "gp2.s1.ration",
                "gp2.s2.retreat",
                "gp2.s3.alternate",
                "gp2.s4.long",
                "gp2.s5.nightway",
            ],
        ),
    ];
    specifications
        .into_iter()
        .map(|(strategy, rule_ids)| {
            let mut result = StrategyResultV1 {
                strategy,
                active_affordances: BTreeSet::new(),
                reachable_decisions: BTreeSet::new(),
                liabilities: BTreeSet::new(),
                meaningful_affordances: BTreeSet::new(),
                meaningful_decisions: BTreeSet::new(),
            };
            let mut s1_outcome = None;
            for rule_id in rule_ids {
                let rule = fixed_progression_rules()
                    .rules
                    .iter()
                    .find(|item| item.rule_id == rule_id)
                    .ok_or(GameplayError::Invalid("strategy references unknown rule"))?;
                let session = fixed_sessions()
                    .into_iter()
                    .find(|item| item.session_id == rule.session_id)
                    .ok_or(GameplayError::Invalid("strategy session missing"))?;
                let outcome = session
                    .outcomes
                    .iter()
                    .find(|item| item.outcome_id == rule.outcome_id)
                    .ok_or(GameplayError::Invalid("strategy outcome missing"))?;
                if rule.session_id == "gp0.s1.colony-conduit" {
                    s1_outcome = Some(rule.outcome_id);
                }
                if rule.session_id == "gp0.s5.afterlight"
                    && !session
                        .admitted_predecessor_outcomes
                        .iter()
                        .any(|id| Some(id.as_str()) == s1_outcome)
                {
                    return Err(GameplayError::Invalid(
                        "strategy has nonviable S1 to S5 predecessor",
                    ));
                }
                let (affordance, decision) = meaningful_strategy_effect(rule.rule_id)?;
                result.meaningful_affordances.insert(affordance);
                result.meaningful_decisions.insert(decision);
                if rule.knowledge {
                    result
                        .active_affordances
                        .insert(format!("knowledge:{}", rule.outcome_id));
                }
                if rule.access {
                    result
                        .active_affordances
                        .insert(format!("access:{}", rule.outcome_id));
                }
                if let Some(subject) = rule.construction_subject {
                    result
                        .active_affordances
                        .insert(format!("construction:{subject}"));
                }
                if let Some(subject) = rule.asset_subject {
                    result.active_affordances.insert(format!("asset:{subject}"));
                }
                if let Some(capability) = rule.capability_id {
                    result
                        .active_affordances
                        .insert(format!("capability:{capability}"));
                }
                for memory in &outcome.memories {
                    result
                        .active_affordances
                        .insert(format!("relationship:{}", memory.rememberer_id));
                }
                result
                    .reachable_decisions
                    .insert(outcome.next_decision.decision_id.clone());
                for cost in &outcome.opportunity_costs {
                    result.liabilities.insert(format!(
                        "{}.{}.{}",
                        cost.subject_id, cost.field_id, cost.value_id
                    ));
                }
            }
            Ok(result)
        })
        .collect()
}
pub fn pairwise_incomparable(results: &[StrategyResultV1]) -> bool {
    (0..results.len()).all(|a| {
        ((a + 1)..results.len())
            .all(|b| !(dominates(&results[a], &results[b]) || dominates(&results[b], &results[a])))
    })
}
fn dominates(a: &StrategyResultV1, b: &StrategyResultV1) -> bool {
    let weak = a
        .meaningful_affordances
        .is_superset(&b.meaningful_affordances)
        && a.meaningful_decisions.is_superset(&b.meaningful_decisions)
        && a.liabilities.is_subset(&b.liabilities);
    weak && (a.meaningful_affordances != b.meaningful_affordances
        || a.meaningful_decisions != b.meaningful_decisions
        || a.liabilities != b.liabilities)
}

fn meaningful_strategy_effect(
    rule_id: &str,
) -> Result<(&'static str, &'static str), GameplayError> {
    match rule_id {
        "gp2.s1.direct" => Ok(("restore-urgent-service", "repair-habitat-or-orchard")),
        "gp2.s1.bypass" => Ok(("preserve-shared-habitat", "fabricate-conduit-priority")),
        "gp2.s1.ration" => Ok(("synchronize-shared-schedule", "repair-constrained-supply")),
        "gp2.s2.relocate" => Ok(("preserve-brood", "survey-nesting-route")),
        "gp2.s2.harvest" => Ok(("retain-named-specimen", "repair-displaced-shelter")),
        "gp2.s2.retreat" => Ok(("coordinate-emergency-care", "inspect-exposed-route")),
        "gp2.s3.charter" => Ok(("maintain-shared-charter", "adjudicate-charter-breach")),
        "gp2.s3.force" => Ok(("complete-urgent-crossing", "return-with-voluntary-evidence")),
        "gp2.s3.alternate" => Ok(("maintain-essential-route", "investigate-or-negotiate")),
        "gp2.s4.rescue" => Ok(("retain-signal-evidence", "pursue-signal-or-repair")),
        "gp2.s4.permanent" => Ok(("restore-permanent-crossing", "search-long-route")),
        "gp2.s4.long" => Ok(("retain-north-detour", "repair-or-map")),
        "gp2.s5.nightway" => Ok(("maintain-bounded-passage", "adjudicate-buffer-violation")),
        "gp2.s5.dismantle" => Ok(("protect-habitat-by-closure", "reconsider-future-charter")),
        _ => Err(GameplayError::Invalid(
            "strategy rule lacks meaningful effect",
        )),
    }
}
fn digest_ledger(ledger: &BaseLoopLedgerV1) -> Result<[u8; 32], GameplayError> {
    let bytes = ledger.to_bytes()?;
    Ok(domain_digest(b"mindwarp.gp2.base-loop-ledger.v1", &bytes))
}
fn digest_state(
    record: &SessionRecordV1,
    state: &BaseLoopStateV1,
) -> Result<[u8; 32], GameplayError> {
    let bytes = state.to_bytes(record)?;
    Ok(domain_digest(
        b"mindwarp.gp2.terminal-loop-state.v1",
        &bytes,
    ))
}
fn digest_session(record: &SessionRecordV1) -> Result<[u8; 32], GameplayError> {
    Ok(domain_digest(
        b"mindwarp.gp2.fixed-session.v1",
        &record.to_bytes()?,
    ))
}
fn digest_registry() -> Result<[u8; 32], GameplayError> {
    let bytes = serde_json::to_vec(fixed_progression_rules())
        .map_err(|e| GameplayError::Codec(e.to_string()))?;
    Ok(domain_digest(b"mindwarp.gp2.rules.v1", &bytes))
}
fn domain_digest(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update((domain.len() as u32).to_be_bytes());
    hasher.update(domain);
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    hasher.finalize().into()
}
