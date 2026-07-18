//! Capability-free advisory controller for bounded mainline divergence.
//!
//! The controller validates a whole-system assessment, enforces the declared
//! probe budget, and ranks target-local experiments. It cannot research,
//! execute, approve, promote, mutate game truth, or replace local validity.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
pub const PARTS_PER_MILLION: u32 = 1_000_000;
pub const SCHEDULED_BATCH_CADENCE: u16 = 3;
pub const REPEATED_WORKAROUND_TRIGGER: u16 = 2;
pub const BROAD_INPUT_SYSTEM_TRIGGER: usize = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerKindV1 {
    ScheduledCadence,
    VerificationFailure,
    RepeatedWorkaround,
    HighLeverageInput,
    MilestoneBoundary,
    Stagnation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FitDispositionV1 {
    Applicable,
    TestOnly,
    Duplicate,
    Deferred,
    NonApplicable,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ControllerActionV1 {
    ResumeMainline,
    RunBoundedProbe,
    DeferBudget,
    RejectCandidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconnectionActionV1 {
    AdoptLocally,
    PublishTransferCandidate,
    Revise,
    Defer,
    Reject,
    QuarantineRegression,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplorationTriggerV1 {
    pub completed_meaningful_batches: u16,
    pub verification_failure: bool,
    pub repeated_workaround_count: u16,
    pub high_leverage_new_input: bool,
    pub milestone_boundary: bool,
    pub stagnation_detected: bool,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MechanismCandidateV1 {
    pub candidate_id: String,
    pub mathematical_abstraction: String,
    pub assumptions: Vec<String>,
    pub claimed_benefit: String,
    pub counterexample: String,
    pub non_applicable_scope: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeOutcomeEstimateV1 {
    pub probability_ppm: u32,
    pub best_local_utility_after: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SystemFitAssessmentV1 {
    pub system_id: String,
    pub disposition: FitDispositionV1,
    pub baseline_evidence_id: String,
    pub local_metric: String,
    pub current_best_expected_utility: i64,
    pub expected_closure_gain: i64,
    pub expected_quality_gain: i64,
    pub expected_future_cost_saved: i64,
    pub implementation_cost: u64,
    pub migration_cost: u64,
    pub recurring_cost: u64,
    pub complexity_risk_cost: u64,
    pub probe_outcomes: Vec<ProbeOutcomeEstimateV1>,
    pub falsifier: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeBudgetV1 {
    pub prior_three_batch_cost: u64,
    pub normal_batch_cost: u64,
    pub proposed_probe_cost: u64,
    pub proposed_external_source_count: u16,
    pub external_source_cap: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StepLeaderInputV1 {
    pub schema_version: u16,
    pub mainline_checkpoint_id: String,
    pub registered_system_ids: Vec<String>,
    pub trigger: ExplorationTriggerV1,
    pub mechanism: MechanismCandidateV1,
    pub assessments: Vec<SystemFitAssessmentV1>,
    pub budget: ProbeBudgetV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RankedProbeV1 {
    pub system_id: String,
    pub value_of_information: i64,
    pub expected_local_net_gain: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StepLeaderDecisionV1 {
    pub schema_version: u16,
    pub mainline_checkpoint_id: String,
    pub trigger_kinds: Vec<TriggerKindV1>,
    pub action: ControllerActionV1,
    pub selected_probe: Option<RankedProbeV1>,
    pub ranked_probes: Vec<RankedProbeV1>,
    pub uncovered_system_ids: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub decision_id: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeResultV1 {
    pub schema_version: u16,
    pub candidate_id: String,
    pub system_id: String,
    pub observed_local_net_gain: i64,
    pub regression_detected: bool,
    pub evidence_ids: Vec<String>,
    pub falsifier_triggered: bool,
    pub independent_successful_modules: Vec<String>,
    pub regressed_modules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconnectionDecisionV1 {
    pub action: ReconnectionActionV1,
    pub resume_checkpoint_id: String,
    pub reason: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepLeaderError {
    Invalid(&'static str),
    Arithmetic(&'static str),
}

fn nonempty(value: &str) -> bool {
    !value.trim().is_empty()
}

fn trigger_kinds(trigger: &ExplorationTriggerV1, applicable_systems: usize) -> Vec<TriggerKindV1> {
    let mut kinds = Vec::new();
    if trigger.completed_meaningful_batches >= SCHEDULED_BATCH_CADENCE {
        kinds.push(TriggerKindV1::ScheduledCadence);
    }
    if trigger.verification_failure {
        kinds.push(TriggerKindV1::VerificationFailure);
    }
    if trigger.repeated_workaround_count >= REPEATED_WORKAROUND_TRIGGER {
        kinds.push(TriggerKindV1::RepeatedWorkaround);
    }
    if trigger.high_leverage_new_input && applicable_systems >= BROAD_INPUT_SYSTEM_TRIGGER {
        kinds.push(TriggerKindV1::HighLeverageInput);
    }
    if trigger.milestone_boundary {
        kinds.push(TriggerKindV1::MilestoneBoundary);
    }
    if trigger.stagnation_detected {
        kinds.push(TriggerKindV1::Stagnation);
    }
    kinds
}

fn validate(input: &StepLeaderInputV1) -> Result<(), StepLeaderError> {
    if input.schema_version != CONTRACT_VERSION
        || !nonempty(&input.mainline_checkpoint_id)
        || !nonempty(&input.mechanism.candidate_id)
        || !nonempty(&input.mechanism.mathematical_abstraction)
        || input.mechanism.assumptions.is_empty()
        || !nonempty(&input.mechanism.claimed_benefit)
        || !nonempty(&input.mechanism.counterexample)
        || input.mechanism.non_applicable_scope.is_empty()
        || input.registered_system_ids.is_empty()
    {
        return Err(StepLeaderError::Invalid("incomplete step-leader input"));
    }
    let registered: BTreeSet<_> = input.registered_system_ids.iter().collect();
    if registered.len() != input.registered_system_ids.len()
        || input.registered_system_ids.iter().any(|id| !nonempty(id))
    {
        return Err(StepLeaderError::Invalid("invalid registered system set"));
    }
    let assessed: BTreeSet<_> = input
        .assessments
        .iter()
        .map(|item| &item.system_id)
        .collect();
    if assessed.len() != input.assessments.len()
        || input.assessments.iter().any(|item| {
            !registered.contains(&item.system_id)
                || !nonempty(&item.baseline_evidence_id)
                || !nonempty(&item.local_metric)
                || !nonempty(&item.falsifier)
        })
    {
        return Err(StepLeaderError::Invalid("invalid system assessment set"));
    }
    for assessment in &input.assessments {
        let probability: u64 = assessment
            .probe_outcomes
            .iter()
            .map(|outcome| u64::from(outcome.probability_ppm))
            .sum();
        if matches!(
            assessment.disposition,
            FitDispositionV1::Applicable | FitDispositionV1::TestOnly
        ) && (assessment.probe_outcomes.is_empty()
            || probability != u64::from(PARTS_PER_MILLION))
        {
            return Err(StepLeaderError::Invalid(
                "probe probabilities do not sum to one",
            ));
        }
    }
    Ok(())
}

fn checked_cost(assessment: &SystemFitAssessmentV1) -> Result<i64, StepLeaderError> {
    let cost = assessment
        .implementation_cost
        .checked_add(assessment.migration_cost)
        .and_then(|value| value.checked_add(assessment.recurring_cost))
        .and_then(|value| value.checked_add(assessment.complexity_risk_cost))
        .ok_or(StepLeaderError::Arithmetic("local cost overflow"))?;
    i64::try_from(cost).map_err(|_| StepLeaderError::Arithmetic("local cost exceeds signed range"))
}

fn expected_local_net_gain(assessment: &SystemFitAssessmentV1) -> Result<i64, StepLeaderError> {
    let cost = checked_cost(assessment)?;
    assessment
        .expected_closure_gain
        .checked_add(assessment.expected_quality_gain)
        .and_then(|value| value.checked_add(assessment.expected_future_cost_saved))
        .and_then(|value| value.checked_sub(cost))
        .ok_or(StepLeaderError::Arithmetic("local net gain overflow"))
}

fn value_of_information(
    assessment: &SystemFitAssessmentV1,
    probe_cost: u64,
) -> Result<i64, StepLeaderError> {
    let weighted = assessment
        .probe_outcomes
        .iter()
        .try_fold(0_i128, |sum, outcome| {
            sum.checked_add(
                i128::from(outcome.probability_ppm) * i128::from(outcome.best_local_utility_after),
            )
            .ok_or(StepLeaderError::Arithmetic("VOI weighted sum overflow"))
        })?;
    let expected_after = weighted / i128::from(PARTS_PER_MILLION);
    let probe_cost = i128::from(probe_cost);
    let value = expected_after
        .checked_sub(i128::from(assessment.current_best_expected_utility))
        .and_then(|value| value.checked_sub(probe_cost))
        .ok_or(StepLeaderError::Arithmetic("VOI overflow"))?;
    i64::try_from(value).map_err(|_| StepLeaderError::Arithmetic("VOI exceeds signed range"))
}

fn budget_admitted(budget: ProbeBudgetV1) -> bool {
    budget.proposed_probe_cost > 0
        && budget.proposed_probe_cost <= budget.normal_batch_cost
        && budget
            .proposed_probe_cost
            .checked_mul(10)
            .is_some_and(|cost| cost <= budget.prior_three_batch_cost)
        && budget.proposed_external_source_count <= budget.external_source_cap
}

fn decision_id(
    input: &StepLeaderInputV1,
    action: ControllerActionV1,
    selected: Option<&RankedProbeV1>,
) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"mindwarp.step-leader.decision.v1");
    hash.update(input.mainline_checkpoint_id.as_bytes());
    hash.update(input.mechanism.candidate_id.as_bytes());
    hash.update([action as u8]);
    if let Some(probe) = selected {
        hash.update(probe.system_id.as_bytes());
        hash.update(probe.value_of_information.to_le_bytes());
        hash.update(probe.expected_local_net_gain.to_le_bytes());
    }
    hash.finalize().into()
}

pub fn decide(input: &StepLeaderInputV1) -> Result<StepLeaderDecisionV1, StepLeaderError> {
    validate(input)?;
    let assessed: BTreeSet<_> = input
        .assessments
        .iter()
        .map(|item| item.system_id.as_str())
        .collect();
    let mut uncovered = input
        .registered_system_ids
        .iter()
        .filter(|id| !assessed.contains(id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    uncovered.sort();
    let applicable = input
        .assessments
        .iter()
        .filter(|item| {
            matches!(
                item.disposition,
                FitDispositionV1::Applicable | FitDispositionV1::TestOnly
            )
        })
        .count();
    let kinds = trigger_kinds(&input.trigger, applicable);
    let mut ranked = Vec::new();
    if uncovered.is_empty() && !kinds.is_empty() && budget_admitted(input.budget) {
        for assessment in &input.assessments {
            if !matches!(
                assessment.disposition,
                FitDispositionV1::Applicable | FitDispositionV1::TestOnly
            ) {
                continue;
            }
            let probe = RankedProbeV1 {
                system_id: assessment.system_id.clone(),
                value_of_information: value_of_information(
                    assessment,
                    input.budget.proposed_probe_cost,
                )?,
                expected_local_net_gain: expected_local_net_gain(assessment)?,
            };
            if probe.value_of_information > 0 && probe.expected_local_net_gain > 0 {
                ranked.push(probe);
            }
        }
        ranked.sort_by(|a, b| {
            b.value_of_information
                .cmp(&a.value_of_information)
                .then_with(|| b.expected_local_net_gain.cmp(&a.expected_local_net_gain))
                .then_with(|| a.system_id.cmp(&b.system_id))
        });
    }
    let action = if !uncovered.is_empty() {
        ControllerActionV1::RejectCandidate
    } else if kinds.is_empty() {
        ControllerActionV1::ResumeMainline
    } else if !budget_admitted(input.budget) {
        ControllerActionV1::DeferBudget
    } else if ranked.is_empty() {
        ControllerActionV1::ResumeMainline
    } else {
        ControllerActionV1::RunBoundedProbe
    };
    let selected = ranked.first().cloned();
    Ok(StepLeaderDecisionV1 {
        schema_version: CONTRACT_VERSION,
        mainline_checkpoint_id: input.mainline_checkpoint_id.clone(),
        trigger_kinds: kinds,
        action,
        selected_probe: selected.clone(),
        ranked_probes: ranked,
        uncovered_system_ids: uncovered,
        limitations: vec![
            "advisory evidence only; no execution approval promotion or policy authority".into(),
            "domain objectives metrics validity and parameters remain target-local".into(),
            "external research requires a separately authorized capability-bearing adapter".into(),
        ],
        authority_effect: "none".into(),
        decision_id: decision_id(input, action, selected.as_ref()),
    })
}

pub fn reconnect(
    mainline_checkpoint_id: &str,
    result: &ProbeResultV1,
) -> Result<ReconnectionDecisionV1, StepLeaderError> {
    if result.schema_version != CONTRACT_VERSION
        || !nonempty(mainline_checkpoint_id)
        || !nonempty(&result.candidate_id)
        || !nonempty(&result.system_id)
        || result.evidence_ids.is_empty()
    {
        return Err(StepLeaderError::Invalid("invalid probe result"));
    }
    let successes: BTreeSet<_> = result.independent_successful_modules.iter().collect();
    let regressions: BTreeSet<_> = result.regressed_modules.iter().collect();
    let (action, reason) = if result.regression_detected || !regressions.is_empty() {
        (
            ReconnectionActionV1::QuarantineRegression,
            "a participating target regressed",
        )
    } else if result.falsifier_triggered || result.observed_local_net_gain <= 0 {
        (
            ReconnectionActionV1::Reject,
            "the local falsifier fired or verified net gain was non-positive",
        )
    } else if successes.len() >= 2 {
        (
            ReconnectionActionV1::PublishTransferCandidate,
            "two independent modules succeeded without a regression",
        )
    } else {
        (
            ReconnectionActionV1::AdoptLocally,
            "one target-local experiment produced positive verified net gain",
        )
    };
    Ok(ReconnectionDecisionV1 {
        action,
        resume_checkpoint_id: mainline_checkpoint_id.into(),
        reason: reason.into(),
        authority_effect: "none".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assessment(id: &str, disposition: FitDispositionV1, utility: i64) -> SystemFitAssessmentV1 {
        SystemFitAssessmentV1 {
            system_id: id.into(),
            disposition,
            baseline_evidence_id: format!("baseline-{id}"),
            local_metric: "verified-cost-units".into(),
            current_best_expected_utility: 0,
            expected_closure_gain: 60,
            expected_quality_gain: 20,
            expected_future_cost_saved: 20,
            implementation_cost: 10,
            migration_cost: 5,
            recurring_cost: 5,
            complexity_risk_cost: 5,
            probe_outcomes: if matches!(
                disposition,
                FitDispositionV1::Applicable | FitDispositionV1::TestOnly
            ) {
                vec![ProbeOutcomeEstimateV1 {
                    probability_ppm: PARTS_PER_MILLION,
                    best_local_utility_after: utility,
                }]
            } else {
                Vec::new()
            },
            falsifier: "candidate is no better than the local baseline".into(),
        }
    }

    fn input() -> StepLeaderInputV1 {
        StepLeaderInputV1 {
            schema_version: 1,
            mainline_checkpoint_id: "C3-fixed-160-readiness".into(),
            registered_system_ids: vec![
                "forge-research".into(),
                "streaming-scheduler".into(),
                "runtime-adapter".into(),
            ],
            trigger: ExplorationTriggerV1 {
                completed_meaningful_batches: 3,
                verification_failure: false,
                repeated_workaround_count: 0,
                high_leverage_new_input: true,
                milestone_boundary: false,
                stagnation_detected: false,
                evidence_ids: vec!["source-conversation".into()],
            },
            mechanism: MechanismCandidateV1 {
                candidate_id: "step-leader".into(),
                mathematical_abstraction: "bounded value-of-information branching".into(),
                assumptions: vec![
                    "local metrics remain comparable only to their own baseline".into(),
                ],
                claimed_benefit: "reduce downstream rework".into(),
                counterexample: "research churn costs more than the avoided work".into(),
                non_applicable_scope: vec!["automatic authority".into()],
            },
            assessments: vec![
                assessment("forge-research", FitDispositionV1::Applicable, 80),
                assessment("streaming-scheduler", FitDispositionV1::TestOnly, 60),
                assessment("runtime-adapter", FitDispositionV1::TestOnly, 0),
            ],
            budget: ProbeBudgetV1 {
                prior_three_batch_cost: 1_000,
                normal_batch_cost: 250,
                proposed_probe_cost: 50,
                proposed_external_source_count: 2,
                external_source_cap: 3,
            },
        }
    }

    #[test]
    fn hybrid_cadence_selects_the_highest_positive_local_voi() {
        let decision = decide(&input()).unwrap();
        assert_eq!(decision.action, ControllerActionV1::RunBoundedProbe);
        assert_eq!(decision.selected_probe.unwrap().system_id, "forge-research");
        assert!(
            decision
                .trigger_kinds
                .contains(&TriggerKindV1::ScheduledCadence)
        );
        assert!(
            decision
                .trigger_kinds
                .contains(&TriggerKindV1::HighLeverageInput)
        );
        assert_eq!(decision.authority_effect, "none");
    }

    #[test]
    fn a_new_registry_system_makes_the_old_map_fail_closed() {
        let mut fixture = input();
        fixture.registered_system_ids.push("new-system".into());
        let decision = decide(&fixture).unwrap();
        assert_eq!(decision.action, ControllerActionV1::RejectCandidate);
        assert_eq!(decision.uncovered_system_ids, ["new-system"]);
    }

    #[test]
    fn budget_above_ten_percent_defers_without_selecting_a_probe() {
        let mut fixture = input();
        fixture.budget.proposed_probe_cost = 101;
        let decision = decide(&fixture).unwrap();
        assert_eq!(decision.action, ControllerActionV1::DeferBudget);
        assert!(decision.selected_probe.is_none());
    }

    #[test]
    fn ordinary_edits_do_not_trigger_divergence() {
        let mut fixture = input();
        fixture.trigger.completed_meaningful_batches = 2;
        fixture.trigger.high_leverage_new_input = false;
        let decision = decide(&fixture).unwrap();
        assert_eq!(decision.action, ControllerActionV1::ResumeMainline);
        assert!(decision.trigger_kinds.is_empty());
    }

    #[test]
    fn malformed_metaphor_and_probabilities_are_rejected() {
        let mut fixture = input();
        fixture.mechanism.mathematical_abstraction.clear();
        assert_eq!(
            decide(&fixture),
            Err(StepLeaderError::Invalid("incomplete step-leader input"))
        );
        let mut fixture = input();
        fixture.assessments[0].probe_outcomes[0].probability_ppm = 999_999;
        assert_eq!(
            decide(&fixture),
            Err(StepLeaderError::Invalid(
                "probe probabilities do not sum to one"
            ))
        );
    }

    #[test]
    fn reconnection_never_masks_a_regressed_target() {
        let result = ProbeResultV1 {
            schema_version: 1,
            candidate_id: "candidate".into(),
            system_id: "forge-research".into(),
            observed_local_net_gain: 900,
            regression_detected: false,
            evidence_ids: vec!["result".into()],
            falsifier_triggered: false,
            independent_successful_modules: vec![
                "forge-research".into(),
                "streaming-scheduler".into(),
            ],
            regressed_modules: vec!["runtime-adapter".into()],
        };
        assert_eq!(
            reconnect("checkpoint", &result).unwrap().action,
            ReconnectionActionV1::QuarantineRegression
        );
    }

    #[test]
    fn reuse_needs_two_independent_successes() {
        let mut result = ProbeResultV1 {
            schema_version: 1,
            candidate_id: "candidate".into(),
            system_id: "forge-research".into(),
            observed_local_net_gain: 4,
            regression_detected: false,
            evidence_ids: vec!["result".into()],
            falsifier_triggered: false,
            independent_successful_modules: vec!["forge-research".into()],
            regressed_modules: Vec::new(),
        };
        assert_eq!(
            reconnect("checkpoint", &result).unwrap().action,
            ReconnectionActionV1::AdoptLocally
        );
        result
            .independent_successful_modules
            .push("streaming-scheduler".into());
        assert_eq!(
            reconnect("checkpoint", &result).unwrap().action,
            ReconnectionActionV1::PublishTransferCandidate
        );
    }

    #[test]
    fn decisions_are_deterministic_and_checkpoint_bound() {
        let first = decide(&input()).unwrap();
        assert_eq!(first, decide(&input()).unwrap());
        let mut other = input();
        other.mainline_checkpoint_id = "other".into();
        assert_ne!(first.decision_id, decide(&other).unwrap().decision_id);
    }
}
