use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    IndeterminateBudget,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Violation {
    pub code: String,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationReport {
    pub status: ValidationStatus,
    pub examined: u32,
    pub violations: Vec<Violation>,
}

fn nonzero(id: &Id) -> bool {
    id.iter().any(|byte| *byte != 0)
}

fn violation(code: &str, location: impl Into<String>) -> Violation {
    Violation {
        code: code.into(),
        location: location.into(),
    }
}

pub fn validate_package(package: &PerceptionProtocolPackage, budget: u32) -> ValidationReport {
    let mut report = ValidationReport {
        status: ValidationStatus::Valid,
        examined: 0,
        violations: vec![],
    };
    macro_rules! examine {
        () => {{
            if report.examined >= budget {
                report.status = ValidationStatus::IndeterminateBudget;
                report.violations.clear();
                return report;
            }
            report.examined += 1;
        }};
    }

    for (name, version) in [
        ("package", package.schema_version),
        ("protocol", package.protocol.schema_version),
        ("environment", package.environment.schema_version),
        ("stimuli", package.stimuli.schema_version),
        ("observations", package.observations.schema_version),
        ("analysis", package.analysis.schema_version),
    ] {
        examine!();
        if version != CONTRACT_VERSION {
            report.violations.push(violation("unknown_schema", name));
        }
    }

    let protocol_fp = package.protocol.fingerprint().unwrap_or([0; 32]);
    let environment_fp = package.environment.fingerprint().unwrap_or([0; 32]);
    let stimulus_fp = package.stimuli.fingerprint().unwrap_or([0; 32]);
    let observation_fp = package.observations.fingerprint().unwrap_or([0; 32]);
    if package.stimuli.protocol_fingerprint != protocol_fp
        || package.observations.protocol_fingerprint != protocol_fp
        || package.analysis.protocol_fingerprint != protocol_fp
    {
        report.violations.push(violation(
            "protocol_drift_or_posthoc_assertion",
            "bindings.protocol",
        ));
    }
    if package.stimuli.environment_fingerprint != environment_fp {
        report.violations.push(violation(
            "environment_profile_drift",
            "stimuli.environment_fingerprint",
        ));
    }
    if package.observations.stimulus_fingerprint != stimulus_fp
        || package.analysis.stimulus_fingerprint != stimulus_fp
    {
        report
            .violations
            .push(violation("stale_stimulus_binding", "bindings.stimulus"));
    }
    if package.analysis.observation_fingerprint != observation_fp {
        report.violations.push(violation(
            "stale_observation_binding",
            "analysis.observation_fingerprint",
        ));
    }

    let mut assertion_ids = BTreeSet::new();
    if !nonzero(&package.protocol.artifact_ref) {
        report.violations.push(violation(
            "missing_artifact_lineage",
            "protocol.artifact_ref",
        ));
    }
    if package.protocol.assertions.is_empty() {
        report.violations.push(violation(
            "missing_predeclared_assertions",
            "protocol.assertions",
        ));
    }
    for assertion in &package.protocol.assertions {
        examine!();
        if !nonzero(&assertion.assertion_id)
            || assertion.statement.trim().is_empty()
            || assertion.required_modes.is_empty()
            || !assertion_ids.insert(assertion.assertion_id)
        {
            report.violations.push(violation(
                "invalid_assertion",
                format!("assertion:{:?}", assertion.assertion_id),
            ));
        }
        if assertion.question_class == QuestionClass::TemporalContinuity
            && !assertion
                .required_modes
                .contains(&PresentationMode::TemporalSequence)
        {
            report.violations.push(violation(
                "temporal_assertion_without_sequence",
                format!("assertion:{:?}", assertion.assertion_id),
            ));
        }
    }
    if package.protocol.derivative_refs.len() < 2
        || package
            .protocol
            .derivative_refs
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != package.protocol.derivative_refs.len()
    {
        report.violations.push(violation(
            "insufficient_or_duplicate_derivatives",
            "protocol.derivative_refs",
        ));
    }
    if package.protocol.stop_rule.trim().is_empty() {
        report
            .violations
            .push(violation("missing_stop_rule", "protocol.stop_rule"));
    }
    if package.protocol.anchor_refs.is_empty() {
        report
            .violations
            .push(violation("missing_review_anchors", "protocol.anchor_refs"));
    }
    let allowed_outcomes: BTreeSet<ReviewOutcome> =
        package.protocol.allowed_outcomes.iter().copied().collect();
    if allowed_outcomes.len() != package.protocol.allowed_outcomes.len() {
        report.violations.push(violation(
            "duplicate_allowed_outcome",
            "protocol.allowed_outcomes",
        ));
    }
    if package.protocol.method == PresentationMethod::BlindPair {
        if !nonzero(&package.protocol.randomization_profile)
            || !nonzero(&package.protocol.blinding_profile)
        {
            report
                .violations
                .push(violation("missing_blinding_or_randomization", "protocol"));
        }
        for required in [ReviewOutcome::NoPreference, ReviewOutcome::Indeterminate] {
            if !package.protocol.allowed_outcomes.contains(&required) {
                report.violations.push(violation(
                    "forced_choice_protocol",
                    "protocol.allowed_outcomes",
                ));
            }
        }
    }
    for required in [
        ControlKind::DuplicatePair,
        ControlKind::SwappedOrder,
        ControlKind::MetricContradiction,
    ] {
        if !package.protocol.controls.contains(&required) {
            report.violations.push(violation(
                "missing_failure_control",
                format!("control:{required:?}"),
            ));
        }
    }
    for repeated in &package.protocol.repeat_assertion_refs {
        if !assertion_ids.contains(repeated) {
            report.violations.push(violation(
                "unknown_repeat_assertion",
                "protocol.repeat_assertion_refs",
            ));
        }
    }

    let environment = &package.environment;
    for (name, value) in [
        ("os", &environment.os_profile),
        ("device", &environment.device_profile),
        ("driver", &environment.driver_profile),
        ("sampling", &environment.sampling_profile),
        ("coordinates", &environment.coordinate_profile),
        ("units", &environment.unit_profile),
        ("output_transform", &environment.output_transform),
        ("display", &environment.display_conditions),
    ] {
        if value.trim().is_empty() {
            report
                .violations
                .push(violation("incomplete_environment", name));
        }
    }
    for (name, id) in [
        ("tool", &environment.tool_profile),
        ("binary", &environment.tool_binary_fingerprint),
        ("config", &environment.tool_config_fingerprint),
        ("camera", &environment.camera_profile),
        ("projection", &environment.projection_profile),
        ("framing", &environment.framing_profile),
        ("lighting", &environment.lighting_profile),
        ("background", &environment.background_profile),
        ("color", &environment.color_config_fingerprint),
    ] {
        if !nonzero(id) {
            report
                .violations
                .push(violation("incomplete_environment", name));
        }
    }
    if environment.width == 0
        || environment.height == 0
        || environment.time_samples.is_empty()
        || environment.presentation_modes.is_empty()
    {
        report.violations.push(violation(
            "incomplete_environment",
            "dimensions_or_samples_or_modes",
        ));
    }
    if environment
        .time_samples
        .windows(2)
        .any(|pair| pair[0] >= pair[1])
    {
        report.violations.push(violation(
            "unordered_time_samples",
            "environment.time_samples",
        ));
    }
    for assertion in &package.protocol.assertions {
        for mode in &assertion.required_modes {
            if !environment.presentation_modes.contains(mode) {
                report.violations.push(violation(
                    "environment_missing_required_mode",
                    format!("assertion:{:?}", assertion.assertion_id),
                ));
            }
        }
    }
    if environment.reproducibility == ReproducibilityClass::ExactSameEnvironment
        && (environment.device_profile.trim().is_empty()
            || environment.driver_profile.trim().is_empty())
    {
        report
            .violations
            .push(violation("false_exact_reproducibility", "environment"));
    }

    let derivative_ids: BTreeSet<Id> = package.protocol.derivative_refs.iter().copied().collect();
    let immutable_inputs: BTreeSet<Id> = package
        .stimuli
        .immutable_input_refs
        .iter()
        .copied()
        .collect();
    if !immutable_inputs.contains(&package.protocol.artifact_ref)
        || derivative_ids
            .iter()
            .any(|derivative| !immutable_inputs.contains(derivative))
    {
        report.violations.push(violation(
            "stimulus_lineage_incomplete",
            "stimuli.immutable_input_refs",
        ));
    }
    let mut pair_ids = BTreeSet::new();
    let mut seen_controls = BTreeSet::new();
    if package.stimuli.pairs.is_empty()
        || package.stimuli.opaque_stimulus_refs.is_empty()
        || !nonzero(&package.stimuli.execution_receipt_ref)
    {
        report
            .violations
            .push(violation("incomplete_stimulus_manifest", "stimuli"));
    }
    for pair in &package.stimuli.pairs {
        examine!();
        if !pair_ids.insert(pair.pair_id)
            || !derivative_ids.contains(&pair.left_derivative_ref)
            || !derivative_ids.contains(&pair.right_derivative_ref)
        {
            report.violations.push(violation(
                "invalid_stimulus_pair",
                format!("pair:{:?}", pair.pair_id),
            ));
        }
        if pair.left_label != "A" || pair.right_label != "B" || !nonzero(&pair.order_token) {
            report.violations.push(violation(
                "label_or_order_leak",
                format!("pair:{:?}", pair.pair_id),
            ));
        }
        for assertion in &pair.assertion_refs {
            if !assertion_ids.contains(assertion) {
                report.violations.push(violation(
                    "unknown_pair_assertion",
                    format!("pair:{:?}", pair.pair_id),
                ));
            }
            if let Some(spec) = package
                .protocol
                .assertions
                .iter()
                .find(|spec| spec.assertion_id == *assertion)
            {
                for required_mode in &spec.required_modes {
                    if !pair.presentation_modes.contains(required_mode) {
                        report.violations.push(violation(
                            "pair_missing_required_mode",
                            format!("pair:{:?}", pair.pair_id),
                        ));
                    }
                }
            }
        }
        for mode in &pair.presentation_modes {
            if !environment.presentation_modes.contains(mode) {
                report.violations.push(violation(
                    "uncontrolled_presentation_mode",
                    format!("pair:{:?}", pair.pair_id),
                ));
            }
        }
        if pair.presentation_modes.len() == 1
            && pair.presentation_modes[0] == PresentationMode::Representative
        {
            report.violations.push(violation(
                "beauty_view_only",
                format!("pair:{:?}", pair.pair_id),
            ));
        }
        if let Some(control) = pair.control {
            seen_controls.insert(control);
        }
    }
    for control in &package.protocol.controls {
        if !seen_controls.contains(control) {
            report.violations.push(violation(
                "declared_control_not_instantiated",
                format!("control:{control:?}"),
            ));
        }
    }

    let allowed = allowed_outcomes;
    let mut observation_ids = BTreeSet::new();
    let mut presentation_orders = BTreeSet::new();
    let mut observed_pairs = BTreeSet::new();
    let mut observed_assertions = BTreeSet::new();
    let mut expected: BTreeMap<Id, AssertionSummary> = assertion_ids
        .iter()
        .map(|id| {
            (
                *id,
                AssertionSummary {
                    assertion_id: *id,
                    satisfied: 0,
                    violated: 0,
                    no_preference: 0,
                    indeterminate: 0,
                    not_observed: 0,
                },
            )
        })
        .collect();
    let control_by_pair: BTreeMap<Id, ControlKind> = package
        .stimuli
        .pairs
        .iter()
        .filter_map(|pair| pair.control.map(|control| (pair.pair_id, control)))
        .collect();
    let mut metric_conflict = false;
    for observation in &package.observations.observations {
        examine!();
        if !observation_ids.insert(observation.observation_id)
            || !pair_ids.contains(&observation.pair_id)
            || !assertion_ids.contains(&observation.assertion_id)
            || !allowed.contains(&observation.outcome)
            || observation.confidence > 100
            || observation.reason_code.trim().is_empty()
        {
            report.violations.push(violation(
                "invalid_observation",
                format!("observation:{:?}", observation.observation_id),
            ));
            continue;
        }
        if !presentation_orders.insert(observation.presentation_order) {
            report.violations.push(violation(
                "duplicate_presentation_order",
                format!("observation:{:?}", observation.observation_id),
            ));
        }
        observed_pairs.insert(observation.pair_id);
        observed_assertions.insert(observation.assertion_id);
        if observation.reviewer_class == ReviewerClass::CreativeDirector
            && observation.claim_class != ClaimClass::ProjectDirection
        {
            report.violations.push(violation(
                "single_owner_claim_overreach",
                format!("observation:{:?}", observation.observation_id),
            ));
        }
        if observation.claim_class == ClaimClass::PopulationPreference {
            report.violations.push(violation(
                "population_claim_unproved",
                format!("observation:{:?}", observation.observation_id),
            ));
        }
        if control_by_pair.get(&observation.pair_id) == Some(&ControlKind::DuplicatePair)
            && !matches!(
                observation.outcome,
                ReviewOutcome::NoPreference
                    | ReviewOutcome::Indeterminate
                    | ReviewOutcome::NotObserved
            )
        {
            report.violations.push(violation(
                "duplicate_control_failed",
                format!("observation:{:?}", observation.observation_id),
            ));
        }
        if control_by_pair.get(&observation.pair_id) == Some(&ControlKind::MetricContradiction) {
            metric_conflict = true;
        }
        let summary = expected.get_mut(&observation.assertion_id).unwrap();
        match observation.outcome {
            ReviewOutcome::Satisfied => summary.satisfied += 1,
            ReviewOutcome::Violated => summary.violated += 1,
            ReviewOutcome::NoPreference => summary.no_preference += 1,
            ReviewOutcome::Indeterminate => summary.indeterminate += 1,
            ReviewOutcome::NotObserved => summary.not_observed += 1,
        }
    }
    if pair_ids.iter().any(|pair| !observed_pairs.contains(pair))
        || assertion_ids
            .iter()
            .any(|assertion| !observed_assertions.contains(assertion))
    {
        report
            .violations
            .push(violation("missing_observation_coverage", "observations"));
    }
    for repeated in &package.protocol.repeat_assertion_refs {
        let count = package
            .observations
            .observations
            .iter()
            .filter(|observation| observation.assertion_id == *repeated)
            .count();
        if count < 2 {
            report.violations.push(violation(
                "missing_repeat_observation",
                format!("assertion:{repeated:?}"),
            ));
        }
    }
    if metric_conflict && package.observations.contradiction_refs.is_empty() {
        report.violations.push(violation(
            "metric_human_contradiction_lost",
            "observations.contradiction_refs",
        ));
    }
    let supplied: BTreeMap<Id, &AssertionSummary> = package
        .analysis
        .summaries
        .iter()
        .map(|item| (item.assertion_id, item))
        .collect();
    if supplied.len() != expected.len()
        || expected
            .iter()
            .any(|(id, value)| supplied.get(id).is_none_or(|item| *item != value))
    {
        report.violations.push(violation(
            "fabricated_or_incomplete_analysis",
            "analysis.summaries",
        ));
    }

    report.violations.sort();
    if !report.violations.is_empty() {
        report.status = ValidationStatus::Invalid;
    }
    report
}
