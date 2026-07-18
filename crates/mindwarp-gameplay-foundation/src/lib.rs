//! GP0 capability-free gameplay contract.
//!
//! This crate owns strict authored gameplay records, typed outcomes, and a
//! pure in-memory reducer. It owns no runtime, persistence, generation,
//! scientific truth, combat resolution, network, or economy.

use std::collections::BTreeSet;

use derived_world_rules::{CausalWorldPacket, WorldGenerationInput, validate_world_packet};
use serde::{Deserialize, Serialize};

mod fixtures;
pub use fixtures::{fixed_concept, fixed_sessions};
mod base_loop;
pub use base_loop::*;
mod progression;
pub use progression::*;
mod encounter_grammar;
pub use encounter_grammar::*;

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameplayError {
    Invalid(&'static str),
    Codec(String),
    InvalidWorld,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fantasy {
    CausalExplorerMaker,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionStatus {
    ReversibleProposed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NonGoal {
    CombatResolvesCoreTension,
    CurrencyShapedProgression,
    AuthoredFactsClaimC3AOrC3BProof,
    RuntimeOrEngineAuthority,
    NetworkOrMonetizationDependency,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameplayConceptRecordV1 {
    pub schema_version: u16,
    pub concept_id: String,
    pub primary_fantasy: Fantasy,
    pub player_promise: String,
    pub non_goals: Vec<NonGoal>,
    pub assumptions: Vec<(String, AssumptionStatus)>,
}

impl GameplayConceptRecordV1 {
    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(GameplayError::Invalid("unsupported concept schema"));
        }
        validate_id(&self.concept_id)?;
        if self.primary_fantasy != Fantasy::CausalExplorerMaker {
            return Err(GameplayError::Invalid("primary fantasy drift"));
        }
        if self.player_promise.trim().is_empty() {
            return Err(GameplayError::Invalid("incomplete player promise"));
        }
        let actual = self.non_goals.iter().copied().collect::<BTreeSet<_>>();
        let required_non_goals = [
            NonGoal::CombatResolvesCoreTension,
            NonGoal::CurrencyShapedProgression,
            NonGoal::AuthoredFactsClaimC3AOrC3BProof,
            NonGoal::RuntimeOrEngineAuthority,
            NonGoal::NetworkOrMonetizationDependency,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if actual != required_non_goals || self.non_goals.len() != required_non_goals.len() {
            return Err(GameplayError::Invalid("non-goal boundary drift"));
        }
        let required = [
            "session_minutes_35_to_60",
            "stable_stop_minutes_10_to_15",
            "fixed_hub_or_vessel",
            "failure_is_repairable",
        ];
        let mut names = BTreeSet::new();
        for (name, status) in &self.assumptions {
            if *status != AssumptionStatus::ReversibleProposed || !names.insert(name.as_str()) {
                return Err(GameplayError::Invalid(
                    "assumption is not uniquely reversible",
                ));
            }
        }
        if required.iter().any(|name| !names.contains(name)) {
            return Err(GameplayError::Invalid("missing reversible assumption"));
        }
        Ok(())
    }

    strict_codec!("concept", validate);
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FactKind {
    Observation,
    Inference,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    AuthoredGameplayNonC3B,
    ObservedC3AOutput,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionFact {
    pub fact_id: String,
    pub kind: FactKind,
    pub proposition: String,
    pub evidence_class: EvidenceClass,
    pub world_reference: Option<C3AWorldReferenceV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RiskCommunication {
    pub risk_id: String,
    pub meaning: String,
    pub visual_cue: String,
    pub audio_or_haptic_cue: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedMutation {
    pub subject_id: String,
    pub field_id: String,
    pub value_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryProposition {
    pub rememberer_id: String,
    pub proposition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TypedGrant {
    Permission {
        grantor_id: String,
        proposition: String,
    },
    Right {
        holder_id: String,
        proposition: String,
    },
    Service {
        provider_id: String,
        proposition: String,
    },
    Knowledge {
        knower_id: String,
        proposition: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NamedDecision {
    pub decision_id: String,
    pub proposition: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeTrigger {
    CausalIntervention,
    Retreat,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutcomeRecordV1 {
    pub outcome_id: String,
    pub trigger: OutcomeTrigger,
    pub exact_mutations: Vec<TypedMutation>,
    pub opportunity_costs: Vec<TypedMutation>,
    pub memories: Vec<MemoryProposition>,
    pub grants: Vec<TypedGrant>,
    pub next_decision: NamedDecision,
    pub resolves_core_tension: bool,
    pub afterlight_trigger: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatContribution {
    pub threat_id: String,
    pub exact_mutations: Vec<TypedMutation>,
    pub limitation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRecordV1 {
    pub schema_version: u16,
    pub session_id: String,
    pub title: String,
    pub player_problem: String,
    pub core_tension: String,
    pub risks: Vec<RiskCommunication>,
    pub facts: Vec<SessionFact>,
    pub threat_contribution: Option<ThreatContribution>,
    pub outcomes: Vec<OutcomeRecordV1>,
    pub admitted_predecessor_outcomes: Vec<String>,
    pub rejected_predecessor_outcomes: Vec<String>,
    pub predecessor_interpretations: Vec<PredecessorInterpretation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PredecessorInterpretation {
    pub outcome_id: String,
    pub exact_mutations: Vec<TypedMutation>,
    pub memories: Vec<MemoryProposition>,
}

impl SessionRecordV1 {
    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(GameplayError::Invalid("unsupported session schema"));
        }
        validate_id(&self.session_id)?;
        if [
            self.title.as_str(),
            self.player_problem.as_str(),
            self.core_tension.as_str(),
        ]
        .iter()
        .any(|value| value.trim().is_empty())
        {
            return Err(GameplayError::Invalid("incomplete session boundary"));
        }
        if self.risks.is_empty() {
            return Err(GameplayError::Invalid("missing accessible risk"));
        }
        for risk in &self.risks {
            validate_id(&risk.risk_id)?;
            if [
                risk.meaning.as_str(),
                risk.visual_cue.as_str(),
                risk.audio_or_haptic_cue.as_str(),
            ]
            .iter()
            .any(|value| value.trim().is_empty())
                || risk.visual_cue == risk.audio_or_haptic_cue
            {
                return Err(GameplayError::Invalid("risk lacks two equivalent cues"));
            }
        }
        let mut fact_ids = BTreeSet::new();
        let mut has_observation = false;
        let mut has_inference = false;
        for fact in &self.facts {
            validate_id(&fact.fact_id)?;
            if !fact_ids.insert(fact.fact_id.as_str()) || fact.proposition.trim().is_empty() {
                return Err(GameplayError::Invalid("invalid or duplicate session fact"));
            }
            reject_currency(&fact.proposition)?;
            reject_authored_authority(fact)?;
            match fact.evidence_class {
                EvidenceClass::AuthoredGameplayNonC3B if fact.world_reference.is_some() => {
                    return Err(GameplayError::Invalid("authored fact has C3A binding"));
                }
                EvidenceClass::ObservedC3AOutput => fact
                    .world_reference
                    .as_ref()
                    .ok_or(GameplayError::Invalid(
                        "C3A observation lacks typed binding",
                    ))?
                    .validate()?,
                _ => {}
            }
            has_observation |= fact.kind == FactKind::Observation;
            has_inference |= fact.kind == FactKind::Inference;
        }
        if !has_observation || !has_inference {
            return Err(GameplayError::Invalid(
                "observation and inference are not separated",
            ));
        }
        if let Some(contribution) = &self.threat_contribution {
            validate_id(&contribution.threat_id)?;
            if contribution.exact_mutations.is_empty() || contribution.limitation.trim().is_empty()
            {
                return Err(GameplayError::Invalid("incomplete threat contribution"));
            }
            for mutation in &contribution.exact_mutations {
                validate_mutation(mutation)?;
            }
        }
        let mut outcome_ids = BTreeSet::new();
        let mut retreat_count = 0;
        for outcome in &self.outcomes {
            validate_outcome(outcome)?;
            if !outcome_ids.insert(outcome.outcome_id.as_str()) {
                return Err(GameplayError::Invalid("duplicate outcome"));
            }
            retreat_count += usize::from(outcome.trigger == OutcomeTrigger::Retreat);
        }
        if retreat_count != 1 {
            return Err(GameplayError::Invalid(
                "session requires one deterministic retreat",
            ));
        }
        validate_predecessors(self)?;
        Ok(())
    }

    pub fn outcome(&self, outcome_id: &str) -> Result<&OutcomeRecordV1, GameplayError> {
        self.outcomes
            .iter()
            .find(|outcome| outcome.outcome_id == outcome_id)
            .ok_or(GameplayError::Invalid("unknown outcome"))
    }

    strict_codec!("session", validate);
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct C3AWorldReferenceV1 {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub input_id: String,
    pub packet_id: String,
}

impl C3AWorldReferenceV1 {
    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION || self.reconstruction_id == [0; 32] {
            return Err(GameplayError::Invalid("invalid C3A world reference"));
        }
        for value in [&self.input_id, &self.packet_id] {
            if value.len() != 64
                || !value
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(GameplayError::Invalid("malformed C3A identity"));
            }
        }
        Ok(())
    }

    strict_codec!("C3A reference", validate);
}

pub fn bind_validated_c3a_world(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<C3AWorldReferenceV1, GameplayError> {
    validate_world_packet(input, packet).map_err(|_| GameplayError::InvalidWorld)?;
    if packet.content.schema_version != CONTRACT_VERSION {
        return Err(GameplayError::InvalidWorld);
    }
    let reference = C3AWorldReferenceV1 {
        schema_version: CONTRACT_VERSION,
        reconstruction_id: input.reconstruction_id,
        input_id: packet.content.input_id.clone(),
        packet_id: packet.packet_id.clone(),
    };
    reference.validate()?;
    Ok(reference)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    ObserveCause,
    MakeFittingTool,
    DivertThreat,
    CommitOutcome { outcome_id: String },
    Retreat { outcome_id: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionState {
    pub schema_version: u16,
    pub session_id: String,
    pub predecessor_outcome_id: Option<String>,
    pub observed_cause: bool,
    pub fitting_tool_made: bool,
    pub threat_diverted: bool,
    pub terminal: bool,
    pub core_tension_resolved: bool,
    pub stable_stop_available: bool,
    pub selected_outcome_id: Option<String>,
    pub exact_mutations: Vec<TypedMutation>,
    pub contributing_mutations: Vec<TypedMutation>,
    pub opportunity_costs: Vec<TypedMutation>,
    pub memories: Vec<MemoryProposition>,
    pub grants: Vec<TypedGrant>,
    pub next_decision: Option<NamedDecision>,
    pub trace: Vec<Action>,
}

impl SessionState {
    pub fn new(record: &SessionRecordV1) -> Result<Self, GameplayError> {
        Self::new_with_predecessor(record, None)
    }

    pub fn new_with_predecessor(
        record: &SessionRecordV1,
        predecessor_outcome_id: Option<&str>,
    ) -> Result<Self, GameplayError> {
        record.validate()?;
        match (record.session_id.as_str(), predecessor_outcome_id) {
            ("gp0.s5.afterlight", Some(outcome))
                if record
                    .admitted_predecessor_outcomes
                    .iter()
                    .any(|candidate| candidate == outcome) => {}
            ("gp0.s5.afterlight", _) => {
                return Err(GameplayError::Invalid(
                    "afterlight predecessor not admitted",
                ));
            }
            (_, None) => {}
            _ => return Err(GameplayError::Invalid("unexpected predecessor context")),
        }
        Ok(Self {
            schema_version: CONTRACT_VERSION,
            session_id: record.session_id.clone(),
            predecessor_outcome_id: predecessor_outcome_id.map(String::from),
            observed_cause: false,
            fitting_tool_made: false,
            threat_diverted: false,
            terminal: false,
            core_tension_resolved: false,
            stable_stop_available: false,
            selected_outcome_id: None,
            exact_mutations: Vec::new(),
            contributing_mutations: Vec::new(),
            opportunity_costs: Vec::new(),
            memories: Vec::new(),
            grants: Vec::new(),
            next_decision: None,
            trace: Vec::new(),
        })
    }

    fn validate_shape(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(GameplayError::Invalid("unsupported state schema"));
        }
        validate_id(&self.session_id)?;
        if self.session_id == "gp0.s5.afterlight" && self.predecessor_outcome_id.is_none() {
            return Err(GameplayError::Invalid("missing afterlight predecessor"));
        }
        if self.terminal
            != (self.selected_outcome_id.is_some()
                && self.next_decision.is_some()
                && self.stable_stop_available)
        {
            return Err(GameplayError::Invalid("terminal state invariant"));
        }
        Ok(())
    }

    pub fn validate_against(&self, record: &SessionRecordV1) -> Result<(), GameplayError> {
        self.validate_shape()?;
        let replayed = replay_actions_after_unchecked(
            record,
            self.predecessor_outcome_id.as_deref(),
            &self.trace,
        )?;
        if &replayed != self {
            return Err(GameplayError::Invalid(
                "state does not match deterministic replay",
            ));
        }
        Ok(())
    }

    pub fn to_bytes(&self, record: &SessionRecordV1) -> Result<Vec<u8>, GameplayError> {
        self.validate_against(record)?;
        serde_json::to_vec(self).map_err(|error| GameplayError::Codec(error.to_string()))
    }

    pub fn from_bytes(record: &SessionRecordV1, bytes: &[u8]) -> Result<Self, GameplayError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GameplayError::Codec(error.to_string()))?;
        value.validate_against(record)?;
        if value.to_bytes(record)? != bytes {
            return Err(GameplayError::Invalid("noncanonical state bytes"));
        }
        Ok(value)
    }
}

pub fn apply_action(
    record: &SessionRecordV1,
    state: &SessionState,
    action: &Action,
) -> Result<SessionState, GameplayError> {
    record.validate()?;
    state.validate_against(record)?;
    apply_action_unchecked(record, state, action)
}

fn apply_action_unchecked(
    record: &SessionRecordV1,
    state: &SessionState,
    action: &Action,
) -> Result<SessionState, GameplayError> {
    state.validate_shape()?;
    if state.session_id != record.session_id || state.terminal {
        return Err(GameplayError::Invalid("invalid reducer state"));
    }
    let mut next = state.clone();
    match action {
        Action::ObserveCause => next.observed_cause = true,
        Action::MakeFittingTool if next.observed_cause => next.fitting_tool_made = true,
        Action::MakeFittingTool => {
            return Err(GameplayError::Invalid("tool precedes causal observation"));
        }
        Action::DivertThreat => {
            let contribution = record
                .threat_contribution
                .as_ref()
                .ok_or(GameplayError::Invalid("session has no threat contribution"))?;
            next.threat_diverted = true;
            next.contributing_mutations = contribution.exact_mutations.clone();
        }
        Action::CommitOutcome { outcome_id } => {
            if !next.observed_cause || !next.fitting_tool_made {
                return Err(GameplayError::Invalid("outcome precedes observe/make loop"));
            }
            let outcome = record.outcome(outcome_id)?;
            if outcome.trigger != OutcomeTrigger::CausalIntervention {
                return Err(GameplayError::Invalid("wrong outcome trigger"));
            }
            apply_terminal_outcome(record, &mut next, outcome)?;
        }
        Action::Retreat { outcome_id } => {
            let outcome = record.outcome(outcome_id)?;
            if outcome.trigger != OutcomeTrigger::Retreat {
                return Err(GameplayError::Invalid("wrong retreat outcome"));
            }
            apply_terminal_outcome(record, &mut next, outcome)?;
        }
    }
    next.trace.push(action.clone());
    next.validate_shape()?;
    Ok(next)
}

pub fn replay_actions(
    record: &SessionRecordV1,
    actions: &[Action],
) -> Result<SessionState, GameplayError> {
    replay_actions_after(record, None, actions)
}

pub fn replay_actions_after(
    record: &SessionRecordV1,
    predecessor_outcome_id: Option<&str>,
    actions: &[Action],
) -> Result<SessionState, GameplayError> {
    let state = replay_actions_after_unchecked(record, predecessor_outcome_id, actions)?;
    state.validate_shape()?;
    Ok(state)
}

fn replay_actions_after_unchecked(
    record: &SessionRecordV1,
    predecessor_outcome_id: Option<&str>,
    actions: &[Action],
) -> Result<SessionState, GameplayError> {
    let mut state = SessionState::new_with_predecessor(record, predecessor_outcome_id)?;
    for action in actions {
        state = apply_action_unchecked(record, &state, action)?;
    }
    Ok(state)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryEventV1 {
    pub sequence: u32,
    pub session_id: String,
    pub outcome_id: String,
    pub predecessor_outcome_id: Option<String>,
    pub exact_mutations: Vec<TypedMutation>,
    pub contributing_mutations: Vec<TypedMutation>,
    pub opportunity_costs: Vec<TypedMutation>,
    pub memories: Vec<MemoryProposition>,
    pub grants: Vec<TypedGrant>,
    pub next_decision: NamedDecision,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldHistoryV1 {
    pub schema_version: u16,
    pub events: Vec<HistoryEventV1>,
}

impl WorldHistoryV1 {
    pub fn empty() -> Self {
        Self {
            schema_version: CONTRACT_VERSION,
            events: Vec::new(),
        }
    }

    pub fn append(
        &self,
        record: &SessionRecordV1,
        state: &SessionState,
    ) -> Result<Self, GameplayError> {
        self.validate()?;
        state.validate_against(record)?;
        if !state.terminal || state.session_id != record.session_id {
            return Err(GameplayError::Invalid("cannot append unfinished session"));
        }
        if record.session_id == "gp0.s5.afterlight"
            && !self.events.iter().any(|event| {
                event.session_id == "gp0.s1.colony-conduit"
                    && state.predecessor_outcome_id.as_ref() == Some(&event.outcome_id)
            })
        {
            return Err(GameplayError::Invalid(
                "afterlight predecessor not admitted",
            ));
        }
        let outcome_id = state.selected_outcome_id.clone().unwrap();
        let mut next = self.clone();
        next.events.push(HistoryEventV1 {
            sequence: u32::try_from(next.events.len() + 1)
                .map_err(|_| GameplayError::Invalid("history length overflow"))?,
            session_id: record.session_id.clone(),
            outcome_id,
            predecessor_outcome_id: state.predecessor_outcome_id.clone(),
            exact_mutations: state.exact_mutations.clone(),
            contributing_mutations: state.contributing_mutations.clone(),
            opportunity_costs: state.opportunity_costs.clone(),
            memories: state.memories.clone(),
            grants: state.grants.clone(),
            next_decision: state.next_decision.clone().unwrap(),
        });
        next.validate()?;
        Ok(next)
    }

    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(GameplayError::Invalid("unsupported history schema"));
        }
        for (index, event) in self.events.iter().enumerate() {
            if event.sequence != u32::try_from(index + 1).unwrap_or(u32::MAX) {
                return Err(GameplayError::Invalid("history sequence drift"));
            }
            validate_id(&event.session_id)?;
            validate_id(&event.outcome_id)?;
            for mutation in event
                .exact_mutations
                .iter()
                .chain(&event.contributing_mutations)
                .chain(&event.opportunity_costs)
            {
                validate_mutation(mutation)?;
            }
            for memory in &event.memories {
                validate_id(&memory.rememberer_id)?;
                reject_currency(&memory.proposition)?;
            }
            for grant in &event.grants {
                validate_grant(grant)?;
            }
            validate_decision(&event.next_decision)?;
            if event.session_id == "gp0.s5.afterlight" {
                let predecessor = event
                    .predecessor_outcome_id
                    .as_deref()
                    .ok_or(GameplayError::Invalid("history predecessor missing"))?;
                if !["s1.direct", "s1.bypass", "s1.ration"].contains(&predecessor)
                    || !self.events[..index].iter().any(|earlier| {
                        earlier.session_id == "gp0.s1.colony-conduit"
                            && earlier.outcome_id == predecessor
                    })
                {
                    return Err(GameplayError::Invalid("history predecessor mismatch"));
                }
            } else if event.predecessor_outcome_id.is_some() {
                return Err(GameplayError::Invalid("unexpected history predecessor"));
            }
        }
        Ok(())
    }

    strict_codec!("history", validate);
}

pub fn project_trace_text(state: &SessionState) -> String {
    let mut lines = vec![format!("session: {}", state.session_id)];
    for (index, action) in state.trace.iter().enumerate() {
        lines.push(format!("{:02}: {}", index + 1, action_name(action)));
    }
    for mutation in &state.exact_mutations {
        lines.push(format!(
            "mutation: {}.{}={}",
            mutation.subject_id, mutation.field_id, mutation.value_id
        ));
    }
    for memory in &state.memories {
        lines.push(format!(
            "memory: {} remembers {}",
            memory.rememberer_id, memory.proposition
        ));
    }
    if let Some(decision) = &state.next_decision {
        lines.push(format!(
            "next_decision: {}: {}",
            decision.decision_id, decision.proposition
        ));
    }
    lines.push(format!(
        "core_tension_resolved: {}",
        state.core_tension_resolved
    ));
    lines.join("\n")
}

fn apply_terminal_outcome(
    record: &SessionRecordV1,
    state: &mut SessionState,
    outcome: &OutcomeRecordV1,
) -> Result<(), GameplayError> {
    state.terminal = true;
    state.core_tension_resolved = outcome.resolves_core_tension;
    state.stable_stop_available = true;
    state.selected_outcome_id = Some(outcome.outcome_id.clone());
    state.exact_mutations = outcome.exact_mutations.clone();
    state.opportunity_costs = outcome.opportunity_costs.clone();
    state.memories = outcome.memories.clone();
    state.grants = outcome.grants.clone();
    state.next_decision = Some(outcome.next_decision.clone());
    if record.session_id == "gp0.s5.afterlight" {
        let predecessor = state
            .predecessor_outcome_id
            .as_deref()
            .ok_or(GameplayError::Invalid("missing afterlight predecessor"))?;
        let interpretation = record
            .predecessor_interpretations
            .iter()
            .find(|item| item.outcome_id == predecessor)
            .ok_or(GameplayError::Invalid("missing predecessor interpretation"))?;
        state
            .exact_mutations
            .extend(interpretation.exact_mutations.clone());
        state.memories.extend(interpretation.memories.clone());
    }
    Ok(())
}

fn validate_outcome(outcome: &OutcomeRecordV1) -> Result<(), GameplayError> {
    validate_id(&outcome.outcome_id)?;
    if outcome.exact_mutations.is_empty()
        || outcome.opportunity_costs.is_empty()
        || outcome.memories.is_empty()
    {
        return Err(GameplayError::Invalid("incomplete typed outcome"));
    }
    for mutation in outcome
        .exact_mutations
        .iter()
        .chain(&outcome.opportunity_costs)
    {
        validate_mutation(mutation)?;
    }
    for memory in &outcome.memories {
        validate_id(&memory.rememberer_id)?;
        reject_currency(&memory.proposition)?;
    }
    for grant in &outcome.grants {
        validate_grant(grant)?;
    }
    validate_decision(&outcome.next_decision)?;
    if outcome.trigger == OutcomeTrigger::Retreat && outcome.resolves_core_tension {
        return Err(GameplayError::Invalid("retreat resolves core tension"));
    }
    Ok(())
}

fn validate_grant(grant: &TypedGrant) -> Result<(), GameplayError> {
    let (actor, proposition) = match grant {
        TypedGrant::Permission {
            grantor_id,
            proposition,
        } => (grantor_id, proposition),
        TypedGrant::Right {
            holder_id,
            proposition,
        } => (holder_id, proposition),
        TypedGrant::Service {
            provider_id,
            proposition,
        } => (provider_id, proposition),
        TypedGrant::Knowledge {
            knower_id,
            proposition,
        } => (knower_id, proposition),
    };
    validate_id(actor)?;
    reject_currency(proposition)
}

fn validate_predecessors(record: &SessionRecordV1) -> Result<(), GameplayError> {
    for outcome in record
        .admitted_predecessor_outcomes
        .iter()
        .chain(&record.rejected_predecessor_outcomes)
    {
        validate_id(outcome)?;
    }
    if record.session_id == "gp0.s5.afterlight" {
        let admitted = record
            .admitted_predecessor_outcomes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected = ["s1.bypass", "s1.direct", "s1.ration"]
            .into_iter()
            .collect::<BTreeSet<_>>();
        if admitted != expected
            || record.rejected_predecessor_outcomes != ["s1.retreat".to_string()]
            || record.predecessor_interpretations.len() != 3
        {
            return Err(GameplayError::Invalid("afterlight predecessor boundary"));
        }
        let interpreted = record
            .predecessor_interpretations
            .iter()
            .map(|item| item.outcome_id.as_str())
            .collect::<BTreeSet<_>>();
        if interpreted != expected {
            return Err(GameplayError::Invalid("afterlight interpretation boundary"));
        }
        for interpretation in &record.predecessor_interpretations {
            if interpretation.exact_mutations.is_empty() || interpretation.memories.is_empty() {
                return Err(GameplayError::Invalid("empty afterlight interpretation"));
            }
            for mutation in &interpretation.exact_mutations {
                validate_mutation(mutation)?;
            }
            for memory in &interpretation.memories {
                validate_id(&memory.rememberer_id)?;
                reject_currency(&memory.proposition)?;
            }
        }
    } else if !record.admitted_predecessor_outcomes.is_empty()
        || !record.rejected_predecessor_outcomes.is_empty()
        || !record.predecessor_interpretations.is_empty()
    {
        return Err(GameplayError::Invalid("unexpected predecessor boundary"));
    }
    Ok(())
}

fn validate_decision(decision: &NamedDecision) -> Result<(), GameplayError> {
    validate_id(&decision.decision_id)?;
    if decision.proposition.trim().is_empty() {
        return Err(GameplayError::Invalid("missing named next decision"));
    }
    reject_currency(&decision.proposition)
}

fn validate_mutation(mutation: &TypedMutation) -> Result<(), GameplayError> {
    validate_id(&mutation.subject_id)?;
    validate_id(&mutation.field_id)?;
    validate_id(&mutation.value_id)
}

fn reject_authored_authority(fact: &SessionFact) -> Result<(), GameplayError> {
    let normalized = fact.proposition.to_ascii_lowercase();
    if fact.evidence_class == EvidenceClass::AuthoredGameplayNonC3B
        && (normalized.contains("c3a proves")
            || normalized.contains("c3b proves")
            || normalized.contains("scientifically proven"))
    {
        return Err(GameplayError::Invalid(
            "authored fact claims scientific authority",
        ));
    }
    Ok(())
}

fn reject_currency(value: &str) -> Result<(), GameplayError> {
    let words = value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .collect::<BTreeSet<_>>();
    if [
        "token",
        "tokens",
        "points",
        "currency",
        "reputation",
        "bond",
        "access",
        "trade",
        "credits",
    ]
    .iter()
    .any(|term| words.contains(*term))
    {
        return Err(GameplayError::Invalid("disguised currency language"));
    }
    Ok(())
}

fn validate_id(value: &str) -> Result<(), GameplayError> {
    if value.is_empty()
        || value.len() > 96
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
    {
        return Err(GameplayError::Invalid("malformed identifier"));
    }
    Ok(())
}

fn action_name(action: &Action) -> &'static str {
    match action {
        Action::ObserveCause => "observe_cause",
        Action::MakeFittingTool => "make_fitting_tool",
        Action::DivertThreat => "divert_threat",
        Action::CommitOutcome { .. } => "commit_outcome",
        Action::Retreat { .. } => "retreat",
    }
}

macro_rules! strict_codec {
    ($label:literal, $validator:ident) => {
        pub fn to_bytes(&self) -> Result<Vec<u8>, GameplayError> {
            self.$validator()?;
            serde_json::to_vec(self).map_err(|error| GameplayError::Codec(error.to_string()))
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<Self, GameplayError> {
            let value: Self = serde_json::from_slice(bytes)
                .map_err(|error| GameplayError::Codec(error.to_string()))?;
            value.$validator()?;
            if value.to_bytes()? != bytes {
                return Err(GameplayError::Invalid(concat!(
                    "noncanonical ",
                    $label,
                    " bytes"
                )));
            }
            Ok(value)
        }
    };
}

use strict_codec;
