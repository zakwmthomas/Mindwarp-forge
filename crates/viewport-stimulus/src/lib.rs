//! Exact bridge from Forge's built-in viewport to the controlled-perception protocol.

use perception_protocol::*;
use reference_viewport::{
    ControlledSnapshot, NegativeControlKind, ViewportSnapshot, negative_control_snapshots,
    reference_snapshot,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedViewportStimulus {
    pub stimulus_id: String,
    pub control: Option<NegativeControlKind>,
    pub snapshot: ViewportSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ControlledStimulusBundle {
    pub schema_version: u16,
    pub status: &'static str,
    pub viewport_profile: &'static str,
    pub base_scene_fingerprint: String,
    pub stimuli: Vec<NamedViewportStimulus>,
    pub protocol_package: PerceptionProtocolPackage,
    pub observed_claim_count: u32,
    pub limitations: Vec<&'static str>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerObservationInput {
    pub expected_base_scene_fingerprint: String,
    pub pair_index: u8,
    pub outcome: ReviewOutcome,
    pub confidence: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnerObservationReceipt {
    pub schema_version: u16,
    pub status: &'static str,
    pub receipt_fingerprint: String,
    pub base_scene_fingerprint: String,
    pub compared_scene_fingerprint: String,
    pub pair_index: u8,
    pub control: ControlKind,
    pub assertion_id: Id,
    pub outcome: ReviewOutcome,
    pub confidence: u8,
    pub reviewer_class: ReviewerClass,
    pub claim_class: ClaimClass,
    pub observed_claim_count: u32,
    pub authority_effect: &'static str,
    pub limitations: Vec<&'static str>,
}

pub fn owner_observation_receipt(
    input: OwnerObservationInput,
) -> Result<OwnerObservationReceipt, String> {
    let bundle = controlled_stimulus_bundle()?;
    if input.expected_base_scene_fingerprint != bundle.base_scene_fingerprint {
        return Err("stale base scene fingerprint".into());
    }
    if input.pair_index > 2 {
        return Err("owner entry is limited to the three visible negative-control pairs".into());
    }
    if input.outcome == ReviewOutcome::NotObserved {
        return Err("not_observed is a placeholder, not a direct owner entry".into());
    }
    if !(1..=100).contains(&input.confidence) {
        return Err("direct owner confidence must be between 1 and 100".into());
    }
    let index = usize::from(input.pair_index);
    let mut package = bundle.protocol_package.clone();
    let pair = package
        .stimuli
        .pairs
        .get(index)
        .ok_or("owner pair is unavailable")?
        .clone();
    let control = pair.control.ok_or("owner pair lacks a declared control")?;
    let assertion_id = pair.assertion_refs[0];
    let observation = package
        .observations
        .observations
        .get_mut(index)
        .ok_or("owner observation slot is unavailable")?;
    observation.outcome = input.outcome;
    observation.confidence = input.confidence;
    observation.reason_code = "direct_owner_review".into();
    observation.limitations = vec!["single_creative_director_project_direction_only".into()];
    package.analysis.summaries = summarize(&package.observations.observations);
    package.analysis.observation_fingerprint = package
        .observations
        .fingerprint()
        .map_err(|error| error.to_string())?;
    package.analysis.limitations = vec![
        "one direct owner observation; remaining outcomes stay not_observed".into(),
        "project direction only; no population preference or authority effect".into(),
    ];
    let report = validate_package(&package, 1024);
    if report.status != ValidationStatus::Valid {
        return Err(format!(
            "owner observation package invalid: {:?}",
            report.violations
        ));
    }
    let compared_scene_fingerprint = bundle.stimuli[index + 1].snapshot.scene_fingerprint.clone();
    let receipt_fingerprint = observation_receipt_fingerprint(
        &bundle.base_scene_fingerprint,
        &compared_scene_fingerprint,
        input.pair_index,
        input.outcome,
        input.confidence,
    );
    Ok(OwnerObservationReceipt {
        schema_version: 1,
        status: "validated_direct_owner_observation",
        receipt_fingerprint,
        base_scene_fingerprint: bundle.base_scene_fingerprint,
        compared_scene_fingerprint,
        pair_index: input.pair_index,
        control,
        assertion_id,
        outcome: input.outcome,
        confidence: input.confidence,
        reviewer_class: ReviewerClass::CreativeDirector,
        claim_class: ClaimClass::ProjectDirection,
        observed_claim_count: 1,
        authority_effect: "none",
        limitations: vec![
            "This receipt is not approval or promotion.",
            "This receipt cannot represent population preference.",
            "The receipt is returned to the owner; it does not mutate the protected Kernel.",
        ],
    })
}

fn summarize(observations: &[Observation]) -> Vec<AssertionSummary> {
    let mut summaries = BTreeMap::<Id, AssertionSummary>::new();
    for observation in observations {
        let summary = summaries
            .entry(observation.assertion_id)
            .or_insert(AssertionSummary {
                assertion_id: observation.assertion_id,
                satisfied: 0,
                violated: 0,
                no_preference: 0,
                indeterminate: 0,
                not_observed: 0,
            });
        match observation.outcome {
            ReviewOutcome::Satisfied => summary.satisfied += 1,
            ReviewOutcome::Violated => summary.violated += 1,
            ReviewOutcome::NoPreference => summary.no_preference += 1,
            ReviewOutcome::Indeterminate => summary.indeterminate += 1,
            ReviewOutcome::NotObserved => summary.not_observed += 1,
        }
    }
    summaries.into_values().collect()
}

fn observation_receipt_fingerprint(
    base: &str,
    compared: &str,
    pair_index: u8,
    outcome: ReviewOutcome,
    confidence: u8,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.viewport-owner-observation.v1\0");
    hasher.update(base.as_bytes());
    hasher.update([0]);
    hasher.update(compared.as_bytes());
    hasher.update([pair_index, outcome as u8, confidence]);
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn controlled_stimulus_bundle() -> Result<ControlledStimulusBundle, String> {
    let base = reference_snapshot().map_err(|error| error.to_string())?;
    let controls = negative_control_snapshots().map_err(|error| error.to_string())?;
    let base_ref = fingerprint_id(&base.scene_fingerprint)?;
    let control_refs: BTreeMap<NegativeControlKind, Id> = controls
        .iter()
        .map(|control| {
            fingerprint_id(&control.snapshot.scene_fingerprint).map(|id| (control.control, id))
        })
        .collect::<Result<_, _>>()?;
    let connection_assertion = id_for("assertion-connection-complete");
    let silhouette_assertion = id_for("assertion-silhouette-distinct");
    let articulation_assertion = id_for("assertion-articulation-stable");
    let artifact_ref = id_for("artifact-reference-viewport-002");
    let derivative_refs = vec![
        base_ref,
        control_refs[&NegativeControlKind::BrokenConnection],
        control_refs[&NegativeControlKind::SilhouetteCollapse],
        control_refs[&NegativeControlKind::ArticulationDrift],
    ];
    let protocol = ReviewProtocol {
        schema_version: 1,
        artifact_ref,
        derivative_refs: derivative_refs.clone(),
        assertions: vec![
            AssertionSpec {
                assertion_id: connection_assertion,
                question_class: QuestionClass::DefectDetection,
                statement: "Every declared support connection remains visible in all required diagnostic views.".into(),
                required_modes: vec![PresentationMode::Representative, PresentationMode::TopologyDiagnostic],
            },
            AssertionSpec {
                assertion_id: silhouette_assertion,
                question_class: QuestionClass::FunctionalLegibility,
                statement: "The articulated span remains distinct in silhouette rather than collapsing into the centreline.".into(),
                required_modes: vec![PresentationMode::Representative, PresentationMode::Silhouette],
            },
            AssertionSpec {
                assertion_id: articulation_assertion,
                question_class: QuestionClass::TemporalContinuity,
                statement: "The declared articulation remains attached and spatially coherent across both pose frames.".into(),
                required_modes: vec![PresentationMode::Representative, PresentationMode::TemporalSequence],
            },
        ],
        method: PresentationMethod::BlindPair,
        randomization_profile: id_for("viewport-fixed-blind-order-v1"),
        blinding_profile: id_for("opaque-a-b-labels-v1"),
        anchor_refs: vec![base_ref],
        controls: vec![
            ControlKind::DuplicatePair,
            ControlKind::SwappedOrder,
            ControlKind::MetricContradiction,
            ControlKind::BrokenConnection,
            ControlKind::SilhouetteCollapse,
            ControlKind::ArticulationDrift,
        ],
        repeat_assertion_refs: vec![connection_assertion, silhouette_assertion],
        stop_rule: "stop_before_claim_if_any_binding_control_or_view_is_missing".into(),
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
        tool_profile: id_for("reference-viewport-0.1.0"),
        tool_binary_fingerprint: id_for("forge-owned-reference-viewport-source-v2"),
        tool_config_fingerprint: id_for(base.renderer_profile),
        os_profile: "forge_desktop_existing_host".into(),
        device_profile: "existing_tauri_webview_canvas".into(),
        driver_profile: "software_orthographic_integer_projection".into(),
        deterministic_seed: 0,
        sampling_profile: "exact_two_pose_frames".into(),
        coordinate_profile: "right_handed_integer_xyz".into(),
        unit_profile: "reference_fixture_units".into(),
        camera_profile: id_for("front-side-top-orthographic"),
        projection_profile: id_for(base.renderer_profile),
        framing_profile: id_for("fixed-centred-scale-0.55"),
        width: 960,
        height: 360,
        time_samples: vec![0, 1],
        lighting_profile: id_for("wireframe-no-lighting"),
        background_profile: id_for("forge-slate-background"),
        presentation_modes: vec![
            PresentationMode::Representative,
            PresentationMode::Silhouette,
            PresentationMode::TopologyDiagnostic,
            PresentationMode::TemporalSequence,
        ],
        color_config_fingerprint: id_for("fixed-wireframe-role-colours-v2"),
        output_transform: "existing_canvas_integer_projection".into(),
        display_conditions: "owner_local_forge_desktop_unverified_display".into(),
        reproducibility: ReproducibilityClass::SemanticCrossEnvironment,
    };
    let protocol_fingerprint = protocol.fingerprint().map_err(|error| error.to_string())?;
    let environment_fingerprint = environment
        .fingerprint()
        .map_err(|error| error.to_string())?;
    let pairs = vec![
        pair(
            "broken",
            base_ref,
            control_refs[&NegativeControlKind::BrokenConnection],
            connection_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::TopologyDiagnostic,
            ],
            Some(ControlKind::BrokenConnection),
        ),
        pair(
            "collapsed",
            base_ref,
            control_refs[&NegativeControlKind::SilhouetteCollapse],
            silhouette_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::Silhouette,
            ],
            Some(ControlKind::SilhouetteCollapse),
        ),
        pair(
            "drift",
            base_ref,
            control_refs[&NegativeControlKind::ArticulationDrift],
            articulation_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::TemporalSequence,
            ],
            Some(ControlKind::ArticulationDrift),
        ),
        pair(
            "duplicate",
            base_ref,
            base_ref,
            connection_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::TopologyDiagnostic,
            ],
            Some(ControlKind::DuplicatePair),
        ),
        pair(
            "swapped",
            control_refs[&NegativeControlKind::BrokenConnection],
            base_ref,
            connection_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::TopologyDiagnostic,
            ],
            Some(ControlKind::SwappedOrder),
        ),
        pair(
            "metric",
            base_ref,
            control_refs[&NegativeControlKind::SilhouetteCollapse],
            silhouette_assertion,
            vec![
                PresentationMode::Representative,
                PresentationMode::Silhouette,
            ],
            Some(ControlKind::MetricContradiction),
        ),
    ];
    let stimuli = StimulusManifest {
        schema_version: 1,
        protocol_fingerprint,
        environment_fingerprint,
        immutable_input_refs: std::iter::once(artifact_ref)
            .chain(derivative_refs.iter().copied())
            .collect(),
        pairs,
        execution_receipt_ref: id_for("built-in-projection-no-external-execution"),
        opaque_stimulus_refs: derivative_refs.clone(),
        omissions: vec![
            "owner_observation_not_yet_recorded".into(),
            "no_external_renderer_or_program".into(),
            "no_material_lighting_physics_or_runtime_claim".into(),
        ],
    };
    let stimulus_fingerprint = stimuli.fingerprint().map_err(|error| error.to_string())?;
    let observations: Vec<Observation> = stimuli
        .pairs
        .iter()
        .enumerate()
        .map(|(index, pair)| Observation {
            observation_id: id_for(&format!("pending-observation-{index}")),
            pair_id: pair.pair_id,
            assertion_id: pair.assertion_refs[0],
            reviewer_class: ReviewerClass::CreativeDirector,
            claim_class: ClaimClass::ProjectDirection,
            outcome: ReviewOutcome::NotObserved,
            confidence: 0,
            reason_code: "awaiting_owner_observation".into(),
            limitations: vec!["placeholder_records_no_judgment".into()],
            presentation_order: index as u32,
        })
        .collect();
    let observation_set = ObservationSet {
        schema_version: 1,
        protocol_fingerprint,
        stimulus_fingerprint,
        observations,
        contradiction_refs: vec![id_for("metric-control-pending-no-claim")],
    };
    let observation_fingerprint = observation_set
        .fingerprint()
        .map_err(|error| error.to_string())?;
    let summaries = [
        connection_assertion,
        silhouette_assertion,
        articulation_assertion,
    ]
    .into_iter()
    .map(|assertion_id| AssertionSummary {
        assertion_id,
        satisfied: 0,
        violated: 0,
        no_preference: 0,
        indeterminate: 0,
        not_observed: observation_set
            .observations
            .iter()
            .filter(|item| item.assertion_id == assertion_id)
            .count() as u32,
    })
    .collect();
    let analysis = AnalysisReceipt {
        schema_version: 1,
        protocol_fingerprint,
        stimulus_fingerprint,
        observation_fingerprint,
        summaries,
        control_failures: vec![],
        limitations: vec!["all outcomes remain not_observed until direct owner review".into()],
    };
    let protocol_package = PerceptionProtocolPackage {
        schema_version: 1,
        protocol,
        environment,
        stimuli,
        observations: observation_set,
        analysis,
    };
    let report = validate_package(&protocol_package, 1024);
    if report.status != ValidationStatus::Valid {
        return Err(format!(
            "controlled stimulus package invalid: {:?}",
            report.violations
        ));
    }
    let mut named_stimuli = vec![NamedViewportStimulus {
        stimulus_id: "reference".into(),
        control: None,
        snapshot: base.clone(),
    }];
    named_stimuli.extend(controls.into_iter().map(named_control));
    Ok(ControlledStimulusBundle {
        schema_version: 1,
        status: "stimuli_ready_observations_pending",
        viewport_profile: base.renderer_profile,
        base_scene_fingerprint: base.scene_fingerprint,
        stimuli: named_stimuli,
        protocol_package,
        observed_claim_count: 0,
        limitations: vec![
            "No owner outcome has been inferred or recorded.",
            "Controls are deliberate defects, not generated asset failures.",
            "This package grants no promotion, runtime, engine, or general quality claim.",
        ],
    })
}

fn pair(
    name: &str,
    left: Id,
    right: Id,
    assertion: Id,
    modes: Vec<PresentationMode>,
    control: Option<ControlKind>,
) -> StimulusPair {
    StimulusPair {
        pair_id: id_for(&format!("pair-{name}")),
        left_derivative_ref: left,
        right_derivative_ref: right,
        left_label: "A".into(),
        right_label: "B".into(),
        order_token: id_for(&format!("order-{name}")),
        assertion_refs: vec![assertion],
        presentation_modes: modes,
        control,
    }
}

fn named_control(control: ControlledSnapshot) -> NamedViewportStimulus {
    let stimulus_id = match control.control {
        NegativeControlKind::BrokenConnection => "broken-connection",
        NegativeControlKind::SilhouetteCollapse => "silhouette-collapse",
        NegativeControlKind::ArticulationDrift => "articulation-drift",
    };
    NamedViewportStimulus {
        stimulus_id: stimulus_id.into(),
        control: Some(control.control),
        snapshot: control.snapshot,
    }
}

fn id_for(value: &str) -> Id {
    Sha256::digest(value.as_bytes()).into()
}

fn fingerprint_id(value: &str) -> Result<Id, String> {
    if value.len() != 64 {
        return Err("scene fingerprint is not SHA-256 hex".into());
    }
    let mut result = [0_u8; 32];
    for (index, byte) in result.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .map_err(|_| "scene fingerprint is not SHA-256 hex")?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_viewport_bundle_is_valid_deterministic_and_pending() {
        let first = controlled_stimulus_bundle().unwrap();
        let second = controlled_stimulus_bundle().unwrap();
        assert_eq!(first, second);
        assert_eq!(
            validate_package(&first.protocol_package, 1024).status,
            ValidationStatus::Valid
        );
        assert_eq!(first.status, "stimuli_ready_observations_pending");
        assert_eq!(first.observed_claim_count, 0);
    }

    #[test]
    fn three_named_negative_controls_are_exactly_bound() {
        let bundle = controlled_stimulus_bundle().unwrap();
        let controls: Vec<_> = bundle
            .stimuli
            .iter()
            .filter_map(|item| item.control)
            .collect();
        assert_eq!(
            controls,
            vec![
                NegativeControlKind::BrokenConnection,
                NegativeControlKind::SilhouetteCollapse,
                NegativeControlKind::ArticulationDrift
            ]
        );
        for item in &bundle.stimuli {
            assert!(
                bundle
                    .protocol_package
                    .stimuli
                    .opaque_stimulus_refs
                    .contains(&fingerprint_id(&item.snapshot.scene_fingerprint).unwrap())
            );
        }
    }

    #[test]
    fn no_owner_judgment_is_fabricated() {
        let bundle = controlled_stimulus_bundle().unwrap();
        assert!(
            bundle
                .protocol_package
                .observations
                .observations
                .iter()
                .all(|item| item.outcome == ReviewOutcome::NotObserved
                    && item.confidence == 0
                    && item.reason_code == "awaiting_owner_observation")
        );
        assert!(
            bundle
                .protocol_package
                .analysis
                .summaries
                .iter()
                .all(|item| item.satisfied == 0
                    && item.violated == 0
                    && item.no_preference == 0
                    && item.indeterminate == 0)
        );
    }

    #[test]
    fn removing_a_declared_control_fails_closed() {
        let mut bundle = controlled_stimulus_bundle().unwrap();
        bundle
            .protocol_package
            .stimuli
            .pairs
            .retain(|pair| pair.control != Some(ControlKind::ArticulationDrift));
        bundle
            .protocol_package
            .observations
            .observations
            .retain(|item| {
                bundle
                    .protocol_package
                    .stimuli
                    .pairs
                    .iter()
                    .any(|pair| pair.pair_id == item.pair_id)
            });
        assert!(
            validate_package(&bundle.protocol_package, 1024)
                .violations
                .iter()
                .any(|item| item.code == "declared_control_not_instantiated")
        );
    }

    #[test]
    fn stale_scene_reference_fails_closed() {
        let mut bundle = controlled_stimulus_bundle().unwrap();
        bundle.protocol_package.stimuli.pairs[0].right_derivative_ref = id_for("not-a-bound-scene");
        assert!(
            validate_package(&bundle.protocol_package, 1024)
                .violations
                .iter()
                .any(|item| item.code == "invalid_stimulus_pair")
        );
    }

    #[test]
    fn bundle_has_no_external_execution_or_authority_claim() {
        let bundle = controlled_stimulus_bundle().unwrap();
        let text = format!("{:?}", bundle);
        for forbidden in [
            "owner_approved",
            "promoted",
            "external_program_executed",
            "population_preference",
        ] {
            assert!(!text.contains(forbidden));
        }
    }

    fn direct_input() -> OwnerObservationInput {
        OwnerObservationInput {
            expected_base_scene_fingerprint: controlled_stimulus_bundle()
                .unwrap()
                .base_scene_fingerprint,
            pair_index: 0,
            outcome: ReviewOutcome::Satisfied,
            confidence: 80,
        }
    }

    #[test]
    fn direct_owner_entry_is_exact_valid_deterministic_and_authority_negative() {
        let first = owner_observation_receipt(direct_input()).unwrap();
        let second = owner_observation_receipt(direct_input()).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.status, "validated_direct_owner_observation");
        assert_eq!(first.observed_claim_count, 1);
        assert_eq!(first.authority_effect, "none");
        assert_eq!(first.reviewer_class, ReviewerClass::CreativeDirector);
        assert_eq!(first.claim_class, ClaimClass::ProjectDirection);
        assert_ne!(
            first.base_scene_fingerprint,
            first.compared_scene_fingerprint
        );
    }

    #[test]
    fn stale_placeholder_and_non_visible_pair_entries_fail_closed() {
        let mut stale = direct_input();
        stale.expected_base_scene_fingerprint = "0".repeat(64);
        assert_eq!(
            owner_observation_receipt(stale).unwrap_err(),
            "stale base scene fingerprint"
        );
        let mut placeholder = direct_input();
        placeholder.outcome = ReviewOutcome::NotObserved;
        assert!(
            owner_observation_receipt(placeholder)
                .unwrap_err()
                .contains("placeholder")
        );
        let mut hidden = direct_input();
        hidden.pair_index = 3;
        assert!(
            owner_observation_receipt(hidden)
                .unwrap_err()
                .contains("three visible")
        );
    }

    #[test]
    fn direct_entry_requires_nonzero_bounded_confidence() {
        for confidence in [0, 101] {
            let mut input = direct_input();
            input.confidence = confidence;
            assert!(
                owner_observation_receipt(input)
                    .unwrap_err()
                    .contains("between 1 and 100")
            );
        }
    }
}
