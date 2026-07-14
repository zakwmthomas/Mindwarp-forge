use serde::Serialize;

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepresentationContractProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub measurement_classification: String,
    pub package_fingerprint: String,
    pub decision_fingerprint: String,
    pub artifact_id: String,
    pub examined: u32,
    pub violations: usize,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

fn id(byte: u8) -> Id {
    [byte; 32]
}

fn trade(dimension: u8, value: i32) -> TradeEvidence {
    TradeEvidence {
        dimension_id: id(dimension),
        value,
        unit: "fixture_units".into(),
        classification: MeasurementClass::Simulated,
        method: "bounded_integer_fixture".into(),
        uncertainty: "synthetic_only".into(),
    }
}

pub fn reference_package() -> Result<RepresentationContractPackage, RepresentationContractError> {
    let portfolio = RepresentationPortfolio {
        options: vec![
            RepresentationOption {
                id: id(10),
                family: RepresentationFamily::RigidAssembly,
                mechanism_evidence: vec![id(11)],
                requirement_refs: vec![id(1), id(2)],
                hard_constraints_satisfied: true,
                trade_vector: vec![trade(20, 2)],
                rejection_reasons: vec![],
            },
            RepresentationOption {
                id: id(12),
                family: RepresentationFamily::NeutralSurface,
                mechanism_evidence: vec![id(13)],
                requirement_refs: vec![id(1), id(2)],
                hard_constraints_satisfied: true,
                trade_vector: vec![trade(20, 3)],
                rejection_reasons: vec![],
            },
        ],
        selected_option: Some(id(10)),
        selection_rationale: vec![
            "both hard constraints pass; lower declared synthetic work".into(),
        ],
        single_feasible_representation: None,
    };
    let decision = portfolio.fingerprint()?;
    let recipe = id(3);
    let generator = id(4);
    let artifact = artifact_identity(recipe, decision, generator);
    Ok(RepresentationContractPackage {
        schema_version: 1,
        semantic_package_ref: id(2),
        recipe_ref: recipe,
        portfolio,
        manifest: ArtifactManifest {
            schema_version: 1,
            artifact_id: artifact,
            recipe_fingerprint: recipe,
            decision_fingerprint: decision,
            generator_profile: generator,
            references: vec![LogicalReference {
                reference_id: id(30),
                content_fingerprint: id(31),
                locator: format!("cid:sha256:{}", crate::hex(&id(31))),
            }],
            derivatives: vec![
                DerivativeRecord {
                    id: id(32),
                    parent_id: artifact,
                    kind: DerivativeKind::Fidelity,
                    method_profile: id(33),
                    declared_loss: vec![trade(21, 1)],
                    validation_ref: id(34),
                },
                DerivativeRecord {
                    id: id(37),
                    parent_id: id(32),
                    kind: DerivativeKind::RepairCandidate,
                    method_profile: id(39),
                    declared_loss: vec![trade(22, 0)],
                    validation_ref: id(38),
                },
            ],
            repairs: vec![RepairAttempt {
                id: id(35),
                parent_id: id(32),
                allowed_scope: vec![id(36)],
                changed_scope: vec![id(36)],
                candidate_id: Some(id(37)),
                result: RepairResult::ValidatedCandidate,
                validation_ref: id(38),
            }],
        },
        materials: MaterialRegionPlan {
            schema_version: 1,
            regions: vec![MaterialRegionBinding {
                region_id: id(40),
                source_role: id(41),
                boundary_refs: vec![id(42)],
                appearance_constraint_refs: vec![id(43)],
            }],
        },
        articulation: ArticulationPlan {
            schema_version: 1,
            frames: vec![LocalFrame {
                id: id(50),
                source_role: id(51),
                handedness: Handedness::Right,
                linear_unit: "fixture_length".into(),
                angular_unit: "fixture_angle".into(),
                transform_order: TransformOrder::ScaleRotateTranslate,
            }],
            degrees_of_freedom: vec![DegreeOfFreedom {
                id: id(52),
                frame_id: id(50),
                source_role: id(53),
                minimum: -1,
                maximum: 1,
                unit: "fixture_angle".into(),
            }],
            symbolic_contact_refs: vec![id(54)],
        },
        temporal: TemporalFidelityPlan {
            schema_version: 1,
            importance_packet_ref: id(60),
            importance_policy_version: 1,
            request_epoch: 1,
            mappings: vec![
                TemporalTierMapping {
                    tier: FidelityTier::Dormant,
                    fidelity_level: 0,
                    cadence_units: 0,
                    interpolation: InterpolationMode::Hold,
                    fallback_tier: None,
                },
                TemporalTierMapping {
                    tier: FidelityTier::Coarse,
                    fidelity_level: 1,
                    cadence_units: 1,
                    interpolation: InterpolationMode::Linear,
                    fallback_tier: Some(FidelityTier::Dormant),
                },
                TemporalTierMapping {
                    tier: FidelityTier::Standard,
                    fidelity_level: 2,
                    cadence_units: 2,
                    interpolation: InterpolationMode::Linear,
                    fallback_tier: Some(FidelityTier::Coarse),
                },
                TemporalTierMapping {
                    tier: FidelityTier::Protected,
                    fidelity_level: 3,
                    cadence_units: 4,
                    interpolation: InterpolationMode::DeclaredSpline,
                    fallback_tier: Some(FidelityTier::Standard),
                },
            ],
        },
        review: ReviewCase {
            schema_version: 1,
            artifact_ref: artifact,
            conditions: ReviewConditions {
                renderer_profile_ref: id(70),
                camera_profile_ref: id(71),
                lighting_profile_ref: id(72),
                color_profile_ref: id(73),
                assertion_refs: vec![id(74)],
            },
            rendered_evidence_refs: vec![],
        },
    })
}

pub fn reference_proof_evidence()
-> Result<RepresentationContractProofEvidence, RepresentationContractError> {
    let package = reference_package()?;
    let report = validate_package(&package, 256);
    if report.status != ValidationStatus::Valid {
        return Err(RepresentationContractError::ValidationFailed);
    }
    Ok(RepresentationContractProofEvidence {
        schema_version: 1,
        system_ids: vec!["representation-selector".into(), "asset-factory".into(), "procedural-animation".into()],
        proof_id: "bounded-p7a-contract-lineage-reference".into(),
        fixture_id: "representation-contract-v1/synthetic-functional-artifact".into(),
        measurement_classification: "simulated".into(),
        package_fingerprint: crate::hex(&package.fingerprint()?),
        decision_fingerprint: crate::hex(&package.portfolio.fingerprint()?),
        artifact_id: crate::hex(&package.manifest.artifact_id),
        examined: report.examined,
        violations: report.violations.len(),
        capabilities: Vec::new(),
        limitations: vec![
            "Tiny synthetic records only; not Mind Warp product vocabulary, representation policy, format, asset, or art direction.".into(),
            "No geometry, materials, shader graph, rig, animation generation, physics, rendering, perception, runtime, engine, or performance claim.".into(),
            "Evidence grants no approval, promotion, execution, spending, publishing, credential, or protected-Kernel authority.".into(),
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
            RepresentationContractPackage::from_bytes(&bytes).unwrap(),
            package
        );
        let mut spaced = bytes.clone();
        spaced.push(b' ');
        assert_eq!(
            RepresentationContractPackage::from_bytes(&spaced),
            Err(RepresentationContractError::NonCanonical)
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value.as_object_mut().unwrap().insert(
            "category_default".into(),
            serde_json::json!("creature_to_mesh"),
        );
        assert!(
            RepresentationContractPackage::from_bytes(&serde_json::to_vec(&value).unwrap())
                .is_err()
        );
    }

    #[test]
    fn mechanism_evidence_not_renameable_ids_proves_diversity() {
        let mut package = reference_package().unwrap();
        package.portfolio.options[1].id = id(99);
        package.portfolio.options[1].mechanism_evidence =
            package.portfolio.options[0].mechanism_evidence.clone();
        package.manifest.decision_fingerprint = package.portfolio.fingerprint().unwrap();
        package.manifest.artifact_id = artifact_identity(
            package.recipe_ref,
            package.manifest.decision_fingerprint,
            package.manifest.generator_profile,
        );
        package.review.artifact_ref = package.manifest.artifact_id;
        package.manifest.derivatives[0].parent_id = package.manifest.artifact_id;
        assert!(has(
            &validate_package(&package, 256),
            "fake_or_missing_diversity"
        ));
    }

    #[test]
    fn infeasible_selection_and_missing_rationale_fail() {
        let mut package = reference_package().unwrap();
        package.portfolio.options[0].hard_constraints_satisfied = false;
        package.portfolio.selection_rationale.clear();
        assert!(has(
            &validate_package(&package, 256),
            "selected_option_not_feasible"
        ));
        assert!(has(
            &validate_package(&package, 256),
            "missing_selection_rationale"
        ));
    }

    #[test]
    fn incomplete_trade_evidence_fails() {
        let mut package = reference_package().unwrap();
        package.portfolio.options[0].trade_vector[0].unit.clear();
        assert!(has(
            &validate_package(&package, 256),
            "incomplete_trade_evidence"
        ));
    }

    #[test]
    fn artifact_identity_binds_recipe_decision_and_generator() {
        let mut package = reference_package().unwrap();
        package.manifest.recipe_fingerprint = id(99);
        assert!(has(
            &validate_package(&package, 256),
            "artifact_identity_mismatch"
        ));
    }

    #[test]
    fn paths_uris_expansion_and_content_mismatch_are_rejected() {
        for locator in [
            "../asset.bin",
            "C:\\asset.bin",
            "\\\\host\\asset",
            "https://example.invalid/a",
            "$HOME/asset",
            "cid:sha256:00",
        ] {
            let mut package = reference_package().unwrap();
            package.manifest.references[0].locator = locator.into();
            assert!(
                has(
                    &validate_package(&package, 256),
                    "hostile_or_unbound_reference"
                ),
                "{locator}"
            );
        }
    }

    #[test]
    fn derivative_parent_must_preexist_in_order() {
        let mut package = reference_package().unwrap();
        package.manifest.derivatives[0].parent_id = id(99);
        assert!(has(
            &validate_package(&package, 256),
            "unknown_derivative_parent"
        ));
    }

    #[test]
    fn repair_cannot_escape_declared_scope_or_lose_candidate() {
        let mut package = reference_package().unwrap();
        package.manifest.repairs[0].changed_scope.push(id(99));
        package.manifest.repairs[0].candidate_id = None;
        let report = validate_package(&package, 256);
        assert!(has(&report, "repair_scope_escape"));
        assert!(has(&report, "missing_repair_candidate"));
    }

    #[test]
    fn repair_candidate_must_remain_in_derivative_lineage() {
        let mut package = reference_package().unwrap();
        package.manifest.repairs[0].candidate_id = Some(id(99));
        assert!(has(
            &validate_package(&package, 256),
            "repair_candidate_outside_lineage"
        ));
    }

    #[test]
    fn material_regions_require_grounded_roles_and_boundaries() {
        let mut package = reference_package().unwrap();
        package.materials.regions[0].source_role = [0; 32];
        package.materials.regions[0].boundary_refs.clear();
        assert!(has(
            &validate_package(&package, 256),
            "ungrounded_material_region"
        ));
    }

    #[test]
    fn articulation_requires_frames_units_ranges_and_roles() {
        let mut package = reference_package().unwrap();
        package.articulation.frames[0].linear_unit.clear();
        package.articulation.degrees_of_freedom[0].frame_id = id(99);
        package.articulation.degrees_of_freedom[0].minimum = 2;
        package.articulation.degrees_of_freedom[0].maximum = 1;
        let report = validate_package(&package, 256);
        assert!(has(&report, "incomplete_frame"));
        assert!(has(&report, "invalid_degree_of_freedom"));
    }

    #[test]
    fn temporal_map_is_p5_bound_monotone_and_fallback_checked() {
        let mut package = reference_package().unwrap();
        package.temporal.importance_policy_version = 0;
        package.temporal.mappings[2].fidelity_level = 0;
        package.temporal.mappings[2].fallback_tier = Some(FidelityTier::Protected);
        let report = validate_package(&package, 256);
        assert!(has(&report, "invalid_importance_binding"));
        assert!(has(&report, "non_monotone_temporal_map"));
        assert!(has(&report, "invalid_temporal_fallback"));
    }

    #[test]
    fn a_decision_and_temporal_plan_cannot_be_empty() {
        let mut package = reference_package().unwrap();
        package.portfolio.selected_option = None;
        package.portfolio.options[0].trade_vector.clear();
        package.temporal.mappings.clear();
        let report = validate_package(&package, 256);
        assert!(has(&report, "missing_selected_option"));
        assert!(has(&report, "missing_trade_evidence"));
        assert!(has(&report, "empty_temporal_map"));
    }

    #[test]
    fn review_conditions_are_required_but_rendered_evidence_is_p7b() {
        let mut package = reference_package().unwrap();
        package.review.conditions.assertion_refs.clear();
        package.review.rendered_evidence_refs.push(id(99));
        let report = validate_package(&package, 256);
        assert!(has(&report, "incomplete_review_conditions"));
        assert!(has(&report, "p7b_evidence_not_authorized"));
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
        package.schema_version = 2;
        assert!(has(&validate_package(&package, 256), "unknown_schema"));
    }

    #[test]
    fn integrated_reference_is_deterministic_and_authority_negative() {
        let first = reference_package().unwrap();
        let second = reference_package().unwrap();
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            validate_package(&first, 256).status,
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
