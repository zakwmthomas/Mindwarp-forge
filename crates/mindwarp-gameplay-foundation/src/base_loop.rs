//! GP1 deterministic fixed base-loop proof.

use std::collections::BTreeSet;

use derived_world_rules::{CausalWorldPacket, WorldGenerationInput};
use serde::{Deserialize, Serialize};

use crate::{
    Action, C3AWorldReferenceV1, CONTRACT_VERSION, GameplayError, SessionRecordV1, SessionState,
    TypedMutation, WorldHistoryV1, apply_action, bind_validated_c3a_world, validate_id,
    validate_mutation,
};

pub const MAX_RECOVERIES: u8 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LoopPhaseV1 {
    Prepare,
    Depart,
    Encounter,
    Consequence,
    Return,
    RememberedResponse,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeActionV1 {
    Prepare,
    Depart,
    Recover,
    BeginReturn,
    RecordRememberedResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StableStopV1 {
    pub completed_phase: Option<LoopPhaseV1>,
    pub resume_action: Option<ResumeActionV1>,
    pub terminal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreparationV1 {
    pub session_id: String,
    pub intent_id: String,
    pub tool_id: String,
    pub divert_threat: bool,
}

impl PreparationV1 {
    fn validate(&self, record: &SessionRecordV1) -> Result<(), GameplayError> {
        if self.session_id != record.session_id {
            return Err(GameplayError::Invalid("preparation session mismatch"));
        }
        validate_id(&self.intent_id)?;
        validate_id(&self.tool_id)?;
        if self.divert_threat && record.threat_contribution.is_none() {
            return Err(GameplayError::Invalid("unsupported threat diversion"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LoopWorldContextV1 {
    AuthoredFixture,
    ValidatedC3A(C3AWorldReferenceV1),
}

impl LoopWorldContextV1 {
    fn validate(&self) -> Result<(), GameplayError> {
        match self {
            Self::AuthoredFixture => Ok(()),
            Self::ValidatedC3A(reference) => reference.validate(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterFailureV1 {
    pub reason_id: String,
    pub opportunity_cost: TypedMutation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseLoopLedgerV1 {
    pub schema_version: u16,
    pub world_history: WorldHistoryV1,
    pub gp1_event_floor: Option<u32>,
    pub completed_runs: Vec<CompletedRunReceiptV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedRunReceiptV1 {
    pub run_id: String,
    pub event_sequence: u32,
    pub session_id: String,
    pub outcome_id: String,
}

impl BaseLoopLedgerV1 {
    pub fn empty() -> Self {
        Self {
            schema_version: CONTRACT_VERSION,
            world_history: WorldHistoryV1::empty(),
            gp1_event_floor: None,
            completed_runs: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(GameplayError::Invalid("unsupported loop ledger schema"));
        }
        self.world_history.validate()?;
        let mut unique_ids = BTreeSet::new();
        let mut unique_sequences = BTreeSet::new();
        for receipt in &self.completed_runs {
            validate_id(&receipt.run_id)?;
            validate_id(&receipt.session_id)?;
            validate_id(&receipt.outcome_id)?;
            if !unique_ids.insert(&receipt.run_id)
                || !unique_sequences.insert(receipt.event_sequence)
            {
                return Err(GameplayError::Invalid("duplicate completed run"));
            }
        }
        match self.gp1_event_floor {
            None if !self.completed_runs.is_empty() => {
                return Err(GameplayError::Invalid(
                    "completed run lacks GP1 event floor",
                ));
            }
            Some(floor) => {
                let history_len = u32::try_from(self.world_history.events.len())
                    .map_err(|_| GameplayError::Invalid("history length overflow"))?;
                if floor == 0 || floor > history_len {
                    return Err(GameplayError::Invalid("invalid GP1 event floor"));
                }
                let expected = (floor..=history_len).collect::<BTreeSet<_>>();
                if unique_sequences != expected {
                    return Err(GameplayError::Invalid("GP1 append receipt coverage drift"));
                }
                for (index, receipt) in self.completed_runs.iter().enumerate() {
                    let expected_sequence = floor
                        .checked_add(u32::try_from(index).map_err(|_| {
                            GameplayError::Invalid("completed run receipt overflow")
                        })?)
                        .ok_or(GameplayError::Invalid("completed run receipt overflow"))?;
                    if receipt.event_sequence != expected_sequence {
                        return Err(GameplayError::Invalid("GP1 append receipt order drift"));
                    }
                    let event = &self.world_history.events[(expected_sequence - 1) as usize];
                    if receipt.session_id != event.session_id
                        || receipt.outcome_id != event.outcome_id
                    {
                        return Err(GameplayError::Invalid("GP1 append receipt event mismatch"));
                    }
                }
            }
            None => {}
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, GameplayError> {
        self.validate()?;
        serde_json::to_vec(self).map_err(|error| GameplayError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GameplayError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GameplayError::Codec(error.to_string()))?;
        value.validate()?;
        if value.to_bytes()? != bytes {
            return Err(GameplayError::Invalid("noncanonical loop ledger bytes"));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BaseLoopActionV1 {
    Prepare(PreparationV1),
    Depart,
    ChooseOutcome {
        outcome_id: String,
    },
    FailEncounter {
        reason_id: String,
        opportunity_cost: TypedMutation,
    },
    Recover,
    BeginReturn,
    RecordRememberedResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseLoopStateV1 {
    pub schema_version: u16,
    pub run_id: String,
    pub session_id: String,
    pub world_context: LoopWorldContextV1,
    pub phase: LoopPhaseV1,
    pub preparation: Option<PreparationV1>,
    pub predecessor_outcome_id: Option<String>,
    pub session_state: SessionState,
    pub ledger_before: BaseLoopLedgerV1,
    pub ledger_after: BaseLoopLedgerV1,
    pub failure: Option<EncounterFailureV1>,
    pub recoveries_used: u8,
    pub stable_stop: StableStopV1,
    pub trace: Vec<BaseLoopActionV1>,
}

impl BaseLoopStateV1 {
    fn validate_shape(&self, record: &SessionRecordV1) -> Result<(), GameplayError> {
        if self.schema_version != CONTRACT_VERSION || self.session_id != record.session_id {
            return Err(GameplayError::Invalid("invalid loop state identity"));
        }
        validate_id(&self.run_id)?;
        self.world_context.validate()?;
        self.ledger_before.validate()?;
        self.ledger_after.validate()?;
        self.session_state.validate_against(record)?;
        if self.session_state.predecessor_outcome_id != self.predecessor_outcome_id {
            return Err(GameplayError::Invalid("loop predecessor drift"));
        }
        if self.recoveries_used > MAX_RECOVERIES {
            return Err(GameplayError::Invalid("recovery count overflow"));
        }
        if let Some(failure) = &self.failure {
            validate_id(&failure.reason_id)?;
            validate_mutation(&failure.opportunity_cost)?;
            if self.phase != LoopPhaseV1::Encounter
                || self.stable_stop.resume_action != Some(ResumeActionV1::Recover)
            {
                return Err(GameplayError::Invalid("failure outside encounter"));
            }
        }
        validate_stop(self)?;
        Ok(())
    }

    pub fn validate_against(&self, record: &SessionRecordV1) -> Result<(), GameplayError> {
        self.validate_shape(record)?;
        let mut replayed = start_unchecked(
            record,
            &self.run_id,
            self.ledger_before.clone(),
            self.world_context.clone(),
        )?;
        for action in &self.trace {
            replayed = apply_unchecked(record, &replayed, action)?;
        }
        if replayed != *self {
            return Err(GameplayError::Invalid(
                "loop state does not match deterministic replay",
            ));
        }
        Ok(())
    }

    pub fn to_bytes(&self, record: &SessionRecordV1) -> Result<Vec<u8>, GameplayError> {
        self.validate_against(record)?;
        serde_json::to_vec(self).map_err(|error| GameplayError::Codec(error.to_string()))
    }

    pub fn from_bytes(
        record: &SessionRecordV1,
        expected_world_context: &LoopWorldContextV1,
        bytes: &[u8],
    ) -> Result<Self, GameplayError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GameplayError::Codec(error.to_string()))?;
        if &value.world_context != expected_world_context {
            return Err(GameplayError::Invalid(
                "world context does not match expected authority",
            ));
        }
        value.validate_against(record)?;
        if value.to_bytes(record)? != bytes {
            return Err(GameplayError::Invalid("noncanonical loop state bytes"));
        }
        Ok(value)
    }
}

pub fn start_authored_base_loop(
    record: &SessionRecordV1,
    run_id: &str,
    ledger: BaseLoopLedgerV1,
) -> Result<BaseLoopStateV1, GameplayError> {
    start_loop(record, run_id, ledger, LoopWorldContextV1::AuthoredFixture)
}

pub fn start_c3a_base_loop(
    record: &SessionRecordV1,
    run_id: &str,
    ledger: BaseLoopLedgerV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<BaseLoopStateV1, GameplayError> {
    let reference = bind_validated_c3a_world(input, packet)?;
    start_loop(
        record,
        run_id,
        ledger,
        LoopWorldContextV1::ValidatedC3A(reference),
    )
}

fn start_loop(
    record: &SessionRecordV1,
    run_id: &str,
    ledger: BaseLoopLedgerV1,
    context: LoopWorldContextV1,
) -> Result<BaseLoopStateV1, GameplayError> {
    let state = start_unchecked(record, run_id, ledger, context)?;
    state.validate_against(record)?;
    Ok(state)
}

fn start_unchecked(
    record: &SessionRecordV1,
    run_id: &str,
    ledger: BaseLoopLedgerV1,
    world_context: LoopWorldContextV1,
) -> Result<BaseLoopStateV1, GameplayError> {
    record.validate()?;
    validate_id(run_id)?;
    ledger.validate()?;
    world_context.validate()?;
    if ledger
        .completed_runs
        .iter()
        .any(|existing| existing.run_id == run_id)
    {
        return Err(GameplayError::Invalid("run already completed"));
    }
    let predecessor_outcome_id = infer_predecessor(record, &ledger.world_history)?;
    let session_state =
        SessionState::new_with_predecessor(record, predecessor_outcome_id.as_deref())?;
    Ok(BaseLoopStateV1 {
        schema_version: CONTRACT_VERSION,
        run_id: run_id.into(),
        session_id: record.session_id.clone(),
        world_context,
        phase: LoopPhaseV1::Prepare,
        preparation: None,
        predecessor_outcome_id,
        session_state,
        ledger_before: ledger.clone(),
        ledger_after: ledger,
        failure: None,
        recoveries_used: 0,
        stable_stop: StableStopV1 {
            completed_phase: None,
            resume_action: Some(ResumeActionV1::Prepare),
            terminal: false,
        },
        trace: Vec::new(),
    })
}

pub fn apply_base_loop_action(
    record: &SessionRecordV1,
    state: &BaseLoopStateV1,
    action: &BaseLoopActionV1,
) -> Result<BaseLoopStateV1, GameplayError> {
    state.validate_against(record)?;
    apply_unchecked(record, state, action)
}

fn apply_unchecked(
    record: &SessionRecordV1,
    state: &BaseLoopStateV1,
    action: &BaseLoopActionV1,
) -> Result<BaseLoopStateV1, GameplayError> {
    state.validate_shape(record)?;
    if state.phase == LoopPhaseV1::RememberedResponse {
        return Err(GameplayError::Invalid("loop is terminal"));
    }
    let mut next = state.clone();
    match action {
        BaseLoopActionV1::Prepare(preparation) if state.phase == LoopPhaseV1::Prepare => {
            preparation.validate(record)?;
            next.preparation = Some(preparation.clone());
            next.phase = LoopPhaseV1::Depart;
            next.stable_stop = stop(
                Some(LoopPhaseV1::Prepare),
                Some(ResumeActionV1::Depart),
                false,
            );
        }
        BaseLoopActionV1::Depart if state.phase == LoopPhaseV1::Depart => {
            next.phase = LoopPhaseV1::Encounter;
            next.stable_stop = no_stop();
        }
        BaseLoopActionV1::ChooseOutcome { outcome_id }
            if state.phase == LoopPhaseV1::Encounter && state.failure.is_none() =>
        {
            validate_id(outcome_id)?;
            let preparation = state
                .preparation
                .as_ref()
                .ok_or(GameplayError::Invalid("missing preparation"))?;
            let outcome = record.outcome(outcome_id)?;
            let actions = if outcome.trigger == crate::OutcomeTrigger::Retreat {
                vec![Action::Retreat {
                    outcome_id: outcome_id.clone(),
                }]
            } else {
                let mut actions = vec![Action::ObserveCause, Action::MakeFittingTool];
                if preparation.divert_threat {
                    actions.push(Action::DivertThreat);
                }
                actions.push(Action::CommitOutcome {
                    outcome_id: outcome_id.clone(),
                });
                actions
            };
            let mut reduced = state.session_state.clone();
            for session_action in actions {
                reduced = apply_action(record, &reduced, &session_action)?;
            }
            next.session_state = reduced;
            next.phase = LoopPhaseV1::Consequence;
            next.stable_stop = stop(
                Some(LoopPhaseV1::Consequence),
                Some(ResumeActionV1::BeginReturn),
                false,
            );
        }
        BaseLoopActionV1::FailEncounter {
            reason_id,
            opportunity_cost,
        } if state.phase == LoopPhaseV1::Encounter && state.failure.is_none() => {
            if state.recoveries_used >= MAX_RECOVERIES {
                return Err(GameplayError::Invalid("recovery limit exhausted"));
            }
            validate_id(reason_id)?;
            validate_mutation(opportunity_cost)?;
            next.failure = Some(EncounterFailureV1 {
                reason_id: reason_id.clone(),
                opportunity_cost: opportunity_cost.clone(),
            });
            next.stable_stop = stop(None, Some(ResumeActionV1::Recover), false);
        }
        BaseLoopActionV1::Recover
            if state.phase == LoopPhaseV1::Encounter && state.failure.is_some() =>
        {
            next.failure = None;
            next.recoveries_used += 1;
            next.stable_stop = no_stop();
        }
        BaseLoopActionV1::BeginReturn if state.phase == LoopPhaseV1::Consequence => {
            next.phase = LoopPhaseV1::Return;
            next.stable_stop = stop(
                Some(LoopPhaseV1::Return),
                Some(ResumeActionV1::RecordRememberedResponse),
                false,
            );
        }
        BaseLoopActionV1::RecordRememberedResponse if state.phase == LoopPhaseV1::Return => {
            if state
                .ledger_after
                .completed_runs
                .iter()
                .any(|existing| existing.run_id == state.run_id)
            {
                return Err(GameplayError::Invalid("run already completed"));
            }
            let mut ledger = state.ledger_after.clone();
            ledger.world_history = ledger.world_history.append(record, &state.session_state)?;
            let event_sequence = u32::try_from(ledger.world_history.events.len())
                .map_err(|_| GameplayError::Invalid("history length overflow"))?;
            if ledger.gp1_event_floor.is_none() {
                ledger.gp1_event_floor = Some(event_sequence);
            }
            ledger.completed_runs.push(CompletedRunReceiptV1 {
                run_id: state.run_id.clone(),
                event_sequence,
                session_id: record.session_id.clone(),
                outcome_id: state
                    .session_state
                    .selected_outcome_id
                    .clone()
                    .ok_or(GameplayError::Invalid("remembered response lacks outcome"))?,
            });
            ledger.validate()?;
            next.ledger_after = ledger;
            next.phase = LoopPhaseV1::RememberedResponse;
            next.stable_stop = stop(Some(LoopPhaseV1::RememberedResponse), None, true);
        }
        _ => return Err(GameplayError::Invalid("action invalid for loop phase")),
    }
    next.trace.push(action.clone());
    next.validate_shape(record)?;
    Ok(next)
}

fn infer_predecessor(
    record: &SessionRecordV1,
    history: &WorldHistoryV1,
) -> Result<Option<String>, GameplayError> {
    if record.session_id != "gp0.s5.afterlight" {
        return Ok(None);
    }
    let candidate = history
        .events
        .iter()
        .filter(|event| event.session_id == "gp0.s1.colony-conduit")
        .next_back()
        .map(|event| event.outcome_id.clone())
        .ok_or(GameplayError::Invalid("afterlight predecessor missing"))?;
    if !record
        .admitted_predecessor_outcomes
        .iter()
        .any(|admitted| admitted == &candidate)
    {
        return Err(GameplayError::Invalid(
            "afterlight latest predecessor inadmissible",
        ));
    }
    Ok(Some(candidate))
}

fn stop(
    completed_phase: Option<LoopPhaseV1>,
    resume_action: Option<ResumeActionV1>,
    terminal: bool,
) -> StableStopV1 {
    StableStopV1 {
        completed_phase,
        resume_action,
        terminal,
    }
}

fn no_stop() -> StableStopV1 {
    stop(None, None, false)
}

fn validate_stop(state: &BaseLoopStateV1) -> Result<(), GameplayError> {
    let expected = match (state.phase, state.failure.is_some()) {
        (LoopPhaseV1::Prepare, false) => stop(None, Some(ResumeActionV1::Prepare), false),
        (LoopPhaseV1::Depart, false) => stop(
            Some(LoopPhaseV1::Prepare),
            Some(ResumeActionV1::Depart),
            false,
        ),
        (LoopPhaseV1::Encounter, false) => no_stop(),
        (LoopPhaseV1::Encounter, true) => stop(None, Some(ResumeActionV1::Recover), false),
        (LoopPhaseV1::Consequence, false) => stop(
            Some(LoopPhaseV1::Consequence),
            Some(ResumeActionV1::BeginReturn),
            false,
        ),
        (LoopPhaseV1::Return, false) => stop(
            Some(LoopPhaseV1::Return),
            Some(ResumeActionV1::RecordRememberedResponse),
            false,
        ),
        (LoopPhaseV1::RememberedResponse, false) => {
            stop(Some(LoopPhaseV1::RememberedResponse), None, true)
        }
        _ => return Err(GameplayError::Invalid("invalid loop stop state")),
    };
    if state.stable_stop != expected {
        return Err(GameplayError::Invalid("stable stop invariant"));
    }
    Ok(())
}
