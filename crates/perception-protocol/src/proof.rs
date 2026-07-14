use serde::Serialize;

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PerceptionProtocolProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub measurement_classification: String,
    pub package_fingerprint: String,
    pub protocol_fingerprint: String,
    pub environment_fingerprint: String,
    pub examined: u32,
    pub violations: usize,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

fn id(byte: u8) -> Id {
    [byte; 32]
}

fn summary(assertion_id: Id, outcomes: &[ReviewOutcome]) -> AssertionSummary {
    let mut result = AssertionSummary {
        assertion_id,
        satisfied: 0,
        violated: 0,
        no_preference: 0,
        indeterminate: 0,
        not_observed: 0,
    };
    for outcome in outcomes {
        match outcome {
            ReviewOutcome::Satisfied => result.satisfied += 1,
            ReviewOutcome::Violated => result.violated += 1,
            ReviewOutcome::NoPreference => result.no_preference += 1,
            ReviewOutcome::Indeterminate => result.indeterminate += 1,
            ReviewOutcome::NotObserved => result.not_observed += 1,
        }
    }
    result
}

pub fn reference_package() -> Result<PerceptionProtocolPackage, PerceptionProtocolError> {
    let assertion = id(10);
    let protocol = ReviewProtocol {
        schema_version: 1,
        artifact_ref: id(1),
        derivative_refs: vec![id(2), id(3)],
        assertions: vec![AssertionSpec {
            assertion_id: assertion,
            question_class: QuestionClass::ComparativeFidelity,
            statement: "Which derivative better retains the declared fixture boundary?".into(),
            required_modes: vec![
                PresentationMode::Representative,
                PresentationMode::RegionDiagnostic,
            ],
        }],
        method: PresentationMethod::BlindPair,
        randomization_profile: id(11),
        blinding_profile: id(12),
        anchor_refs: vec![id(13), id(14)],
        controls: vec![
            ControlKind::DuplicatePair,
            ControlKind::SwappedOrder,
            ControlKind::MetricContradiction,
        ],
        repeat_assertion_refs: vec![assertion],
        stop_rule: "bounded_fixture_pairs_complete_or_control_failure".into(),
        allowed_outcomes: vec![
            ReviewOutcome::Satisfied,
            ReviewOutcome::Violated,
            ReviewOutcome::NoPreference,
            ReviewOutcome::Indeterminate,
            ReviewOutcome::NotObserved,
        ],
    };
    let environment = EnvironmentProfile {
        schema_version: 1,
        tool_profile: id(20),
        tool_binary_fingerprint: id(21),
        tool_config_fingerprint: id(22),
        os_profile: "synthetic_os_profile".into(),
        device_profile: "synthetic_cpu_profile".into(),
        driver_profile: "synthetic_software_profile".into(),
        deterministic_seed: 7,
        sampling_profile: "synthetic_fixed_sampling".into(),
        coordinate_profile: "right_handed_fixture".into(),
        unit_profile: "fixture_units".into(),
        camera_profile: id(23),
        projection_profile: id(24),
        framing_profile: id(25),
        width: 64,
        height: 64,
        time_samples: vec![0, 1],
        lighting_profile: id(26),
        background_profile: id(27),
        presentation_modes: vec![
            PresentationMode::Representative,
            PresentationMode::RegionDiagnostic,
        ],
        color_config_fingerprint: id(28),
        output_transform: "synthetic_display_transform".into(),
        display_conditions: "synthetic_pinned_conditions".into(),
        reproducibility: ReproducibilityClass::ExactSameEnvironment,
    };
    let protocol_fp = protocol.fingerprint()?;
    let environment_fp = environment.fingerprint()?;
    let pairs = vec![
        StimulusPair {
            pair_id: id(30),
            left_derivative_ref: id(2),
            right_derivative_ref: id(3),
            left_label: "A".into(),
            right_label: "B".into(),
            order_token: id(31),
            assertion_refs: vec![assertion],
            presentation_modes: vec![
                PresentationMode::Representative,
                PresentationMode::RegionDiagnostic,
            ],
            control: None,
        },
        StimulusPair {
            pair_id: id(32),
            left_derivative_ref: id(2),
            right_derivative_ref: id(2),
            left_label: "A".into(),
            right_label: "B".into(),
            order_token: id(33),
            assertion_refs: vec![assertion],
            presentation_modes: vec![
                PresentationMode::Representative,
                PresentationMode::RegionDiagnostic,
            ],
            control: Some(ControlKind::DuplicatePair),
        },
        StimulusPair {
            pair_id: id(34),
            left_derivative_ref: id(3),
            right_derivative_ref: id(2),
            left_label: "A".into(),
            right_label: "B".into(),
            order_token: id(35),
            assertion_refs: vec![assertion],
            presentation_modes: vec![
                PresentationMode::Representative,
                PresentationMode::RegionDiagnostic,
            ],
            control: Some(ControlKind::SwappedOrder),
        },
        StimulusPair {
            pair_id: id(36),
            left_derivative_ref: id(2),
            right_derivative_ref: id(3),
            left_label: "A".into(),
            right_label: "B".into(),
            order_token: id(37),
            assertion_refs: vec![assertion],
            presentation_modes: vec![
                PresentationMode::Representative,
                PresentationMode::RegionDiagnostic,
            ],
            control: Some(ControlKind::MetricContradiction),
        },
    ];
    let stimuli = StimulusManifest {
        schema_version: 1,
        protocol_fingerprint: protocol_fp,
        environment_fingerprint: environment_fp,
        immutable_input_refs: vec![id(1), id(2), id(3)],
        pairs,
        execution_receipt_ref: id(38),
        opaque_stimulus_refs: vec![id(39), id(40)],
        omissions: vec!["no_images_created_or_viewed".into()],
    };
    let stimulus_fp = stimuli.fingerprint()?;
    let outcomes = [
        ReviewOutcome::Satisfied,
        ReviewOutcome::NoPreference,
        ReviewOutcome::Satisfied,
        ReviewOutcome::Violated,
    ];
    let observations = ObservationSet {
        schema_version: 1,
        protocol_fingerprint: protocol_fp,
        stimulus_fingerprint: stimulus_fp,
        observations: (0..4)
            .map(|index| Observation {
                observation_id: id(50 + index),
                pair_id: id(30 + index * 2),
                assertion_id: assertion,
                reviewer_class: ReviewerClass::CreativeDirector,
                claim_class: ClaimClass::ProjectDirection,
                outcome: outcomes[index as usize],
                confidence: 50,
                reason_code: format!("synthetic_reason_{index}"),
                limitations: vec!["synthetic_protocol_only".into()],
                presentation_order: index as u32,
            })
            .collect(),
        contradiction_refs: vec![id(60)],
    };
    let observation_fp = observations.fingerprint()?;
    let analysis = AnalysisReceipt {
        schema_version: 1,
        protocol_fingerprint: protocol_fp,
        stimulus_fingerprint: stimulus_fp,
        observation_fingerprint: observation_fp,
        summaries: vec![summary(assertion, &outcomes)],
        control_failures: vec![],
        limitations: vec!["synthetic_protocol_only".into()],
    };
    Ok(PerceptionProtocolPackage {
        schema_version: 1,
        protocol,
        environment,
        stimuli,
        observations,
        analysis,
    })
}

pub fn reference_proof_evidence() -> Result<PerceptionProtocolProofEvidence, PerceptionProtocolError>
{
    let package = reference_package()?;
    let report = validate_package(&package, 512);
    if report.status != ValidationStatus::Valid {
        return Err(PerceptionProtocolError::ValidationFailed);
    }
    Ok(PerceptionProtocolProofEvidence {
        schema_version: 1,
        system_ids: vec!["representation-selector".into(), "forge-reference-studio".into()],
        proof_id: "bounded-p7b0-controlled-perception-protocol".into(),
        fixture_id: "perception-protocol-v1/synthetic-blind-pair".into(),
        measurement_classification: "simulated".into(),
        package_fingerprint: crate::hex(&package.fingerprint()?),
        protocol_fingerprint: crate::hex(&package.protocol.fingerprint()?),
        environment_fingerprint: crate::hex(&package.environment.fingerprint()?),
        examined: report.examined,
        violations: report.violations.len(),
        capabilities: vec![],
        limitations: vec![
            "Synthetic identifiers and protocol metadata only; no image was created, opened, inspected, or judged.".into(),
            "No renderer, DCC, format, asset, animation, tool execution, filesystem, network, process, GPU, runtime, engine, art direction, or quality threshold.".into(),
            "Single-owner observations can express project direction only; they do not prove population preference or general recognisability.".into(),
            "Evidence grants no approval, promotion, spending, publishing, credential, or protected-Kernel authority.".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has(report: &ValidationReport, code: &str) -> bool {
        report.violations.iter().any(|item| item.code == code)
    }

    #[test]
    fn canonical_bytes_are_strict_and_unknown_fields_fail() {
        let package = reference_package().unwrap();
        let bytes = package.to_bytes().unwrap();
        assert_eq!(
            PerceptionProtocolPackage::from_bytes(&bytes).unwrap(),
            package
        );
        let mut spaced = bytes.clone();
        spaced.push(b' ');
        assert_eq!(
            PerceptionProtocolPackage::from_bytes(&spaced),
            Err(PerceptionProtocolError::NonCanonical)
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("universal_quality_score".into(), serde_json::json!(100));
        assert!(
            PerceptionProtocolPackage::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
        );
    }

    #[test]
    fn assertions_cannot_be_written_after_stimuli() {
        let mut package = reference_package().unwrap();
        package.protocol.assertions[0]
            .statement
            .push_str(" changed after viewing");
        assert!(has(
            &validate_package(&package, 512),
            "protocol_drift_or_posthoc_assertion"
        ));
    }

    #[test]
    fn blind_pairs_cannot_force_choice_or_leak_labels() {
        let mut package = reference_package().unwrap();
        package
            .protocol
            .allowed_outcomes
            .retain(|outcome| *outcome != ReviewOutcome::NoPreference);
        package.stimuli.pairs[0].left_label = "preferred_candidate".into();
        let report = validate_package(&package, 512);
        assert!(has(&report, "forced_choice_protocol"));
        assert!(has(&report, "label_or_order_leak"));
    }

    #[test]
    fn declared_failure_controls_must_exist() {
        let mut package = reference_package().unwrap();
        package
            .stimuli
            .pairs
            .retain(|pair| pair.control != Some(ControlKind::SwappedOrder));
        package
            .observations
            .observations
            .retain(|item| item.pair_id != id(34));
        assert!(has(
            &validate_package(&package, 512),
            "declared_control_not_instantiated"
        ));
    }

    #[test]
    fn one_beauty_view_is_not_coverage() {
        let mut package = reference_package().unwrap();
        package.stimuli.pairs[0].presentation_modes = vec![PresentationMode::Representative];
        assert!(has(&validate_package(&package, 512), "beauty_view_only"));
    }

    #[test]
    fn temporal_assertion_requires_sequence() {
        let mut package = reference_package().unwrap();
        package.protocol.assertions[0].question_class = QuestionClass::TemporalContinuity;
        assert!(has(
            &validate_package(&package, 512),
            "temporal_assertion_without_sequence"
        ));
    }

    #[test]
    fn creative_director_cannot_imply_population_preference() {
        let mut package = reference_package().unwrap();
        package.observations.observations[0].claim_class = ClaimClass::PopulationPreference;
        let report = validate_package(&package, 512);
        assert!(has(&report, "single_owner_claim_overreach"));
        assert!(has(&report, "population_claim_unproved"));
    }

    #[test]
    fn duplicate_control_must_not_create_false_preference() {
        let mut package = reference_package().unwrap();
        package.observations.observations[1].outcome = ReviewOutcome::Satisfied;
        assert!(has(
            &validate_package(&package, 512),
            "duplicate_control_failed"
        ));
    }

    #[test]
    fn metric_human_contradiction_must_be_retained() {
        let mut package = reference_package().unwrap();
        package.observations.contradiction_refs.clear();
        assert!(has(
            &validate_package(&package, 512),
            "metric_human_contradiction_lost"
        ));
    }

    #[test]
    fn analysis_counts_cannot_be_fabricated() {
        let mut package = reference_package().unwrap();
        package.analysis.summaries[0].satisfied += 1;
        assert!(has(
            &validate_package(&package, 512),
            "fabricated_or_incomplete_analysis"
        ));
    }

    #[test]
    fn stale_environment_stimulus_and_observation_bindings_fail() {
        let mut package = reference_package().unwrap();
        package.environment.output_transform.push_str("_drift");
        package.observations.stimulus_fingerprint = id(99);
        package.analysis.observation_fingerprint = id(98);
        let report = validate_package(&package, 512);
        assert!(has(&report, "environment_profile_drift"));
        assert!(has(&report, "stale_stimulus_binding"));
        assert!(has(&report, "stale_observation_binding"));
    }

    #[test]
    fn environment_conditions_and_modes_are_not_defaults() {
        let mut package = reference_package().unwrap();
        package.environment.display_conditions.clear();
        package
            .environment
            .presentation_modes
            .retain(|mode| *mode != PresentationMode::RegionDiagnostic);
        let report = validate_package(&package, 512);
        assert!(has(&report, "incomplete_environment"));
        assert!(has(&report, "environment_missing_required_mode"));
    }

    #[test]
    fn stimulus_inputs_must_retain_artifact_and_derivative_lineage() {
        let mut package = reference_package().unwrap();
        package
            .stimuli
            .immutable_input_refs
            .retain(|item| *item != id(3));
        assert!(has(
            &validate_package(&package, 512),
            "stimulus_lineage_incomplete"
        ));
    }

    #[test]
    fn every_pair_needs_required_modes_and_an_observation() {
        let mut package = reference_package().unwrap();
        package.stimuli.pairs[0]
            .presentation_modes
            .retain(|mode| *mode != PresentationMode::RegionDiagnostic);
        package
            .observations
            .observations
            .retain(|item| item.pair_id != id(36));
        let report = validate_package(&package, 512);
        assert!(has(&report, "pair_missing_required_mode"));
        assert!(has(&report, "missing_observation_coverage"));
    }

    #[test]
    fn presentation_order_and_repeat_evidence_are_explicit() {
        let mut package = reference_package().unwrap();
        package.observations.observations[1].presentation_order = 0;
        package.protocol.repeat_assertion_refs = vec![id(99)];
        let report = validate_package(&package, 512);
        assert!(has(&report, "duplicate_presentation_order"));
        assert!(has(&report, "unknown_repeat_assertion"));
        assert!(has(&report, "missing_repeat_observation"));
    }

    #[test]
    fn budget_exhaustion_is_indeterminate() {
        assert_eq!(
            validate_package(&reference_package().unwrap(), 1).status,
            ValidationStatus::IndeterminateBudget
        );
    }

    #[test]
    fn version_drift_fails_closed() {
        let mut package = reference_package().unwrap();
        package.analysis.schema_version = 2;
        assert!(has(&validate_package(&package, 512), "unknown_schema"));
    }

    #[test]
    fn reference_is_deterministic_and_authority_negative() {
        let first = reference_package().unwrap();
        let second = reference_package().unwrap();
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            validate_package(&first, 512).status,
            ValidationStatus::Valid
        );
        let evidence = reference_proof_evidence().unwrap();
        assert!(evidence.capabilities.is_empty());
        let text = serde_json::to_string(&evidence).unwrap();
        for forbidden in [
            "\"approve\"",
            "\"promote\"",
            "\"execute\"",
            "\"publish\"",
            "\"spend\"",
            "\"credential\"",
        ] {
            assert!(!text.contains(forbidden));
        }
    }
}
