use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{EvidenceAuthority, ProofChainError, reference_h5_decision, reference_manifest};

const PACKAGE_DOMAIN: &[u8] = b"mindwarp.humanoid-proof-chain.promotion-package.v1\0";
const TRANSITION_DOMAIN: &[u8] = b"mindwarp.humanoid-proof-chain.lifecycle-transition.v1\0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateKind {
    EngineNeutralHumanoidProofBaseline,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStatus {
    EvidencePackageOnlyNotKernelCandidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredOwnerAction {
    ApproveExactCandidate,
    PromoteExactCandidate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionPackageContent {
    pub schema_version: u16,
    pub candidate_name: String,
    pub candidate_kind: CandidateKind,
    pub h6_manifest_id: String,
    pub h5_decision_receipt_id: String,
    pub claims: Vec<String>,
    pub non_claims: Vec<String>,
    pub rollback_target: String,
    pub required_owner_actions: Vec<RequiredOwnerAction>,
    pub authority: EvidenceAuthority,
    pub readiness_status: ReadinessStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionPackage {
    pub package_id: String,
    pub content: PromotionPackageContent,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulatedCandidateState {
    Proposed,
    Approved,
    Promoted,
    Superseded,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulatedActor {
    DirectProjectUser,
    Assistant,
    ImportedContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulatedLifecycleAction {
    Approve,
    Promote,
    Supersede {
        correction_evidence_id: String,
        replacement_candidate_id: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimulatedCandidate {
    pub candidate_id: String,
    pub evidence_package_id: String,
    pub state: SimulatedCandidateState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimulatedTransitionReceipt {
    pub transition_id: String,
    pub candidate_id: String,
    pub before: SimulatedCandidateState,
    pub after: SimulatedCandidateState,
    pub retained_evidence_package_id: String,
    pub correction_evidence_id: Option<String>,
    pub replacement_candidate_id: Option<String>,
    pub authority_effect: String,
    pub prohibited_effects: Vec<String>,
}

impl PromotionPackage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ProofChainError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProofChainError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| ProofChainError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(ProofChainError::NonCanonical);
        }
        validate_promotion_package(&value)?;
        Ok(value)
    }
}

pub fn reference_promotion_package() -> Result<PromotionPackage, ProofChainError> {
    let manifest = reference_manifest()?;
    let h5 = reference_h5_decision()?;
    let content = PromotionPackageContent {
        schema_version: 1,
        candidate_name: "engine-neutral-humanoid-proof-baseline-v1".into(),
        candidate_kind: CandidateKind::EngineNeutralHumanoidProofBaseline,
        h6_manifest_id: manifest.manifest_id,
        h5_decision_receipt_id: h5.receipt_id,
        claims: required_claims(),
        non_claims: required_non_claims(),
        rollback_target: "no_promoted_humanoid_proof_baseline".into(),
        required_owner_actions: vec![
            RequiredOwnerAction::ApproveExactCandidate,
            RequiredOwnerAction::PromoteExactCandidate,
        ],
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
        readiness_status: ReadinessStatus::EvidencePackageOnlyNotKernelCandidate,
    };
    let package = PromotionPackage {
        package_id: hex(&domain_hash(PACKAGE_DOMAIN, &canonical_json(&content)?)),
        content,
    };
    validate_promotion_package(&package)?;
    Ok(package)
}

pub fn validate_promotion_package(package: &PromotionPackage) -> Result<(), ProofChainError> {
    let manifest = reference_manifest()?;
    let h5 = reference_h5_decision()?;
    let expected_content = PromotionPackageContent {
        schema_version: 1,
        candidate_name: "engine-neutral-humanoid-proof-baseline-v1".into(),
        candidate_kind: CandidateKind::EngineNeutralHumanoidProofBaseline,
        h6_manifest_id: manifest.manifest_id,
        h5_decision_receipt_id: h5.receipt_id,
        claims: required_claims(),
        non_claims: required_non_claims(),
        rollback_target: "no_promoted_humanoid_proof_baseline".into(),
        required_owner_actions: vec![
            RequiredOwnerAction::ApproveExactCandidate,
            RequiredOwnerAction::PromoteExactCandidate,
        ],
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
        readiness_status: ReadinessStatus::EvidencePackageOnlyNotKernelCandidate,
    };
    let expected_id = hex(&domain_hash(
        PACKAGE_DOMAIN,
        &canonical_json(&package.content)?,
    ));
    if package.content != expected_content || package.package_id != expected_id {
        return Err(ProofChainError::Invalid(
            "promotion package scope, evidence, rollback, or authority drifted",
        ));
    }
    Ok(())
}

pub fn simulated_candidate(
    package: &PromotionPackage,
) -> Result<SimulatedCandidate, ProofChainError> {
    validate_promotion_package(package)?;
    Ok(SimulatedCandidate {
        candidate_id: hex(&domain_hash(
            b"mindwarp.h7.simulated-candidate.v1\0",
            package.package_id.as_bytes(),
        )),
        evidence_package_id: package.package_id.clone(),
        state: SimulatedCandidateState::Proposed,
    })
}

pub fn simulate_transition(
    candidate: &SimulatedCandidate,
    package: &PromotionPackage,
    actor: SimulatedActor,
    explicit_candidate_id: &str,
    action: SimulatedLifecycleAction,
) -> Result<(SimulatedCandidate, SimulatedTransitionReceipt), ProofChainError> {
    validate_promotion_package(package)?;
    let expected = simulated_candidate(package)?;
    if candidate.candidate_id != expected.candidate_id
        || candidate.evidence_package_id != package.package_id
        || explicit_candidate_id != candidate.candidate_id
        || actor != SimulatedActor::DirectProjectUser
    {
        return Err(ProofChainError::Invalid(
            "candidate binding or direct-owner authority is invalid",
        ));
    }
    let (after, correction, replacement) = match (&candidate.state, action) {
        (SimulatedCandidateState::Proposed, SimulatedLifecycleAction::Approve) => {
            (SimulatedCandidateState::Approved, None, None)
        }
        (SimulatedCandidateState::Approved, SimulatedLifecycleAction::Promote) => {
            (SimulatedCandidateState::Promoted, None, None)
        }
        (
            SimulatedCandidateState::Approved | SimulatedCandidateState::Promoted,
            SimulatedLifecycleAction::Supersede {
                correction_evidence_id,
                replacement_candidate_id,
            },
        ) if valid_evidence_id(&correction_evidence_id)
            && replacement_candidate_id.as_deref() != Some(candidate.candidate_id.as_str()) =>
        {
            (
                SimulatedCandidateState::Superseded,
                Some(correction_evidence_id),
                replacement_candidate_id,
            )
        }
        _ => {
            return Err(ProofChainError::Invalid(
                "lifecycle transition is stale, skipped, or lacks correction evidence",
            ));
        }
    };
    let mut updated = candidate.clone();
    let before = updated.state;
    updated.state = after;
    let receipt_content = (
        &candidate.candidate_id,
        before,
        after,
        &candidate.evidence_package_id,
        &correction,
        &replacement,
    );
    let receipt = SimulatedTransitionReceipt {
        transition_id: hex(&domain_hash(
            TRANSITION_DOMAIN,
            &canonical_json(&receipt_content)?,
        )),
        candidate_id: candidate.candidate_id.clone(),
        before,
        after,
        retained_evidence_package_id: candidate.evidence_package_id.clone(),
        correction_evidence_id: correction,
        replacement_candidate_id: replacement,
        authority_effect: "simulated_only_no_kernel_state_change".into(),
        prohibited_effects: prohibited_effects(),
    };
    Ok((updated, receipt))
}

fn required_claims() -> Vec<String> {
    [
        "h1_provenance_is_exact_and_authority_bounded",
        "h2_neutral_17_joint_structure_is_deterministic",
        "h3_structural_candidate_rebuilds_capability_free",
        "h4_structural_controls_are_exact_and_fixture_local",
        "h5_visual_direction_is_owner_bound_and_separate_from_h3_geometry",
        "h6_chain_identifiers_limitations_and_receipts_recover_exactly",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn required_non_claims() -> Vec<String> {
    [
        "no_generated_or_imported_surface_asset",
        "no_anatomical_truth_or_population_rule",
        "no_topology_uv_skinning_deformation_animation_physics_material_or_shader_proof",
        "no_device_runtime_engine_or_production_fitness",
        "no_gameplay_capability_intelligence_morality_role_or_importance_rule",
        "no_non_human_inheritance_of_human_binary_shape_language",
        "no_application_execution_publishing_spending_acquisition_or_protected_kernel_mutation",
        "no_approval_promotion_or_supersession_from_evidence_or_conversational_assent",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn prohibited_effects() -> Vec<String> {
    [
        "kernel_state_change",
        "asset_import",
        "application",
        "execution",
        "publishing",
        "runtime_selection",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn valid_evidence_id(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, ProofChainError> {
    serde_json::to_vec(value).map_err(|error| ProofChainError::Codec(error.to_string()))
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_package_is_deterministic_strict_and_authority_negative() {
        let first = reference_promotion_package().unwrap();
        let second = reference_promotion_package().unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first.package_id,
            "7b01d650258fe50b7cd59290a4a56e6df3a17271991dba313e29b6c0cf607619"
        );
        assert_eq!(
            PromotionPackage::from_bytes(&first.to_bytes().unwrap()).unwrap(),
            first
        );
        assert_eq!(
            first.content.authority,
            EvidenceAuthority::EvidenceOnlyNoPromotion
        );
        assert_eq!(first.content.required_owner_actions.len(), 2);
        assert_eq!(
            simulated_candidate(&first).unwrap().candidate_id,
            "2d93d3ae31de08754852e27a3a04332d009b456141565be8e805a98eed8d6222"
        );
    }

    #[test]
    fn overbroad_stale_or_incomplete_packages_fail_closed() {
        let good = reference_promotion_package().unwrap().to_bytes().unwrap();
        let base: serde_json::Value = serde_json::from_slice(&good).unwrap();
        let mutations = [
            ("/content/candidate_name", serde_json::json!("humanoid")),
            (
                "/content/candidate_kind",
                serde_json::json!("production_asset"),
            ),
            ("/content/h6_manifest_id", serde_json::json!("0".repeat(64))),
            ("/content/h5_decision_receipt_id", serde_json::json!("")),
            (
                "/content/claims/4",
                serde_json::json!("h3_geometry_is_visually_approved"),
            ),
            ("/content/non_claims", serde_json::json!([])),
            ("/content/rollback_target", serde_json::json!("")),
            ("/content/required_owner_actions", serde_json::json!([])),
        ];
        for (pointer, replacement) in mutations {
            let mut value = base.clone();
            *value.pointer_mut(pointer).unwrap() = replacement;
            assert!(PromotionPackage::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        }
        let mut unknown = base;
        unknown
            .as_object_mut()
            .unwrap()
            .insert("auto_promote".into(), serde_json::json!(true));
        assert!(PromotionPackage::from_bytes(&serde_json::to_vec(&unknown).unwrap()).is_err());
    }

    #[test]
    fn forward_lifecycle_requires_exact_separate_direct_owner_actions() {
        let package = reference_promotion_package().unwrap();
        let proposed = simulated_candidate(&package).unwrap();
        let (approved, approval) = simulate_transition(
            &proposed,
            &package,
            SimulatedActor::DirectProjectUser,
            &proposed.candidate_id,
            SimulatedLifecycleAction::Approve,
        )
        .unwrap();
        assert_eq!(approval.after, SimulatedCandidateState::Approved);
        let (promoted, promotion) = simulate_transition(
            &approved,
            &package,
            SimulatedActor::DirectProjectUser,
            &approved.candidate_id,
            SimulatedLifecycleAction::Promote,
        )
        .unwrap();
        assert_eq!(promoted.state, SimulatedCandidateState::Promoted);
        assert_ne!(approval.transition_id, promotion.transition_id);
    }

    #[test]
    fn forged_skipped_stale_and_self_replacement_transitions_fail() {
        let package = reference_promotion_package().unwrap();
        let proposed = simulated_candidate(&package).unwrap();
        for actor in [SimulatedActor::Assistant, SimulatedActor::ImportedContent] {
            assert!(
                simulate_transition(
                    &proposed,
                    &package,
                    actor,
                    &proposed.candidate_id,
                    SimulatedLifecycleAction::Approve
                )
                .is_err()
            );
        }
        assert!(
            simulate_transition(
                &proposed,
                &package,
                SimulatedActor::DirectProjectUser,
                "stale",
                SimulatedLifecycleAction::Approve
            )
            .is_err()
        );
        assert!(
            simulate_transition(
                &proposed,
                &package,
                SimulatedActor::DirectProjectUser,
                &proposed.candidate_id,
                SimulatedLifecycleAction::Promote
            )
            .is_err()
        );
        assert!(
            simulate_transition(
                &proposed,
                &package,
                SimulatedActor::DirectProjectUser,
                &proposed.candidate_id,
                SimulatedLifecycleAction::Supersede {
                    correction_evidence_id: "a".repeat(64),
                    replacement_candidate_id: None,
                },
            )
            .is_err()
        );
        let (approved, _) = simulate_transition(
            &proposed,
            &package,
            SimulatedActor::DirectProjectUser,
            &proposed.candidate_id,
            SimulatedLifecycleAction::Approve,
        )
        .unwrap();
        assert!(
            simulate_transition(
                &approved,
                &package,
                SimulatedActor::DirectProjectUser,
                &approved.candidate_id,
                SimulatedLifecycleAction::Supersede {
                    correction_evidence_id: "bad".into(),
                    replacement_candidate_id: None,
                },
            )
            .is_err()
        );
        assert!(
            simulate_transition(
                &approved,
                &package,
                SimulatedActor::DirectProjectUser,
                &approved.candidate_id,
                SimulatedLifecycleAction::Supersede {
                    correction_evidence_id: "a".repeat(64),
                    replacement_candidate_id: Some(approved.candidate_id.clone()),
                },
            )
            .is_err()
        );
    }

    #[test]
    fn promoted_supersession_replays_and_retains_evidence_without_effects() {
        let package = reference_promotion_package().unwrap();
        let proposed = simulated_candidate(&package).unwrap();
        let (approved, _) = simulate_transition(
            &proposed,
            &package,
            SimulatedActor::DirectProjectUser,
            &proposed.candidate_id,
            SimulatedLifecycleAction::Approve,
        )
        .unwrap();
        let (promoted, _) = simulate_transition(
            &approved,
            &package,
            SimulatedActor::DirectProjectUser,
            &approved.candidate_id,
            SimulatedLifecycleAction::Promote,
        )
        .unwrap();
        let action = SimulatedLifecycleAction::Supersede {
            correction_evidence_id: "b".repeat(64),
            replacement_candidate_id: Some("c".repeat(64)),
        };
        let first = simulate_transition(
            &promoted,
            &package,
            SimulatedActor::DirectProjectUser,
            &promoted.candidate_id,
            action.clone(),
        )
        .unwrap();
        let second = simulate_transition(
            &promoted,
            &package,
            SimulatedActor::DirectProjectUser,
            &promoted.candidate_id,
            action,
        )
        .unwrap();
        assert_eq!(first, second);
        assert_eq!(first.0.state, SimulatedCandidateState::Superseded);
        assert_eq!(first.1.retained_evidence_package_id, package.package_id);
        assert_eq!(
            first.1.authority_effect,
            "simulated_only_no_kernel_state_change"
        );
        assert!(first.1.prohibited_effects.contains(&"execution".into()));
    }
}
