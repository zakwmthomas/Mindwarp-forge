use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainmentProfileProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub measurement_classification: String,
    pub package_fingerprint: String,
    pub boundary_fingerprint: String,
    pub examined: u32,
    pub violations: usize,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

fn id(seed: u8) -> Id {
    let mut value = [0; 32];
    value[0] = seed;
    value[31] = seed.wrapping_mul(17);
    value
}

fn denied_capabilities() -> Vec<Capability> {
    vec![
        Capability::Network,
        Capability::Credentials,
        Capability::Clipboard,
        Capability::Devices,
        Capability::HostUi,
        Capability::Gpu,
        Capability::Plugins,
        Capability::Scripting,
        Capability::PackageManager,
        Capability::Repository,
        Capability::ExistingUserWrite,
        Capability::RegistryWrite,
        Capability::ChildEscape,
        Capability::Elevation,
    ]
}

pub fn reference_package() -> Result<ContainmentProfilePackage, ContainmentProfileError> {
    let tool = ToolIdentity {
        schema_version: 1,
        stage: ReadinessStage::PolicyOnly,
        selection_state: SelectionState::Unselected,
        tool_class: "future_offline_renderer_class".into(),
        official_source_required: true,
        publisher_signature_required: true,
        binary_fingerprint: None,
        dependency_fingerprints: vec![],
        license_evidence_ref: None,
        version: None,
        auto_update_disabled: true,
        removal_plan: "record_before_any_separately_authorized_install".into(),
    };
    let boundary = BoundaryProfile {
        schema_version: 1,
        kind: BoundaryKind::LessPrivilegedAppContainer,
        role: BoundaryRole::SecurityBoundaryCandidate,
        host_profile_ref: id(1),
        prerequisite_refs: vec![id(2), id(3)],
        granted_capabilities: vec![],
        denied_capabilities: denied_capabilities(),
        job_object_required: true,
        full_process_tree_termination: true,
    };
    let input = InputPolicy {
        schema_version: 1,
        generated_synthetic_only: true,
        fresh_per_run: true,
        read_only: true,
        content_addressed: true,
        repository_paths_allowed: false,
        user_directories_allowed: false,
        forbidden_hazards: vec![
            InputHazard::Traversal,
            InputHazard::AbsolutePath,
            InputHazard::UncPath,
            InputHazard::DevicePath,
            InputHazard::ReservedName,
            InputHazard::EnvironmentExpansion,
            InputHazard::AlternateDataStream,
            InputHazard::ReparsePoint,
            InputHazard::ExternalReference,
            InputHazard::Archive,
            InputHazard::NativeProject,
            InputHazard::ActiveContent,
        ],
    };
    let output = OutputPolicy {
        schema_version: 1,
        fresh_quarantine: true,
        outside_repository: true,
        outside_existing_user_content: true,
        per_run_identity: true,
        direct_durable_write: false,
        direct_preview: false,
        admission_steps: vec![
            AdmissionStep::RevokeRunnerAccess,
            AdmissionStep::PathAndReparseCheck,
            AdmissionStep::CountSizeDepthCheck,
            AdmissionStep::SignatureAndTypeCheck,
            AdmissionStep::ContentHash,
            AdmissionStep::ManifestCompleteness,
            AdmissionStep::BoundedFormatValidation,
        ],
        allowed_type_classes: vec!["future_declared_inert_output".into()],
    };
    let budget = ResourceBudget {
        schema_version: 1,
        max_wall_ms: 10_000,
        max_cpu_ms: 10_000,
        max_memory_bytes: 268_435_456,
        max_processes: 2,
        max_output_bytes: 16_777_216,
        max_output_files: 16,
        max_nesting: 4,
        max_dimension: 4_096,
        max_geometry_items: 100_000,
        max_texture_items: 16,
        max_frames: 16,
    };
    let recovery = RecoveryPlan {
        schema_version: 1,
        retain_failure_evidence: true,
        terminate_full_process_tree: true,
        dispose_boundary: true,
        revoke_output_access_before_admission: true,
        prove_project_immutability: true,
        retry_with_new_run_id: true,
        failure_action: QuarantineFailureAction::RetainInertEvidence,
    };
    let receipt = ContainmentReadinessReceipt {
        schema_version: 1,
        stage: ReadinessStage::PolicyOnly,
        tool_fingerprint: tool.fingerprint()?,
        boundary_fingerprint: boundary.fingerprint()?,
        input_fingerprint: input.fingerprint()?,
        output_fingerprint: output.fingerprint()?,
        budget_fingerprint: budget.fingerprint()?,
        recovery_fingerprint: recovery.fingerprint()?,
        status: "policy_ready_not_executed".into(),
        denied_capability_count: denied_capabilities().len() as u32,
        limitations: vec![
            "does_not_prove_runtime_containment".into(),
            "no_execution_authority".into(),
            "candidate_boundary_unselected_and_unexecuted".into(),
        ],
    };
    Ok(ContainmentProfilePackage {
        schema_version: 1,
        tool,
        boundary,
        input,
        output,
        budget,
        recovery,
        receipt,
    })
}

pub fn reference_proof_evidence() -> Result<ContainmentProfileProofEvidence, ContainmentProfileError>
{
    let package = reference_package()?;
    let report = validate_package(&package, 128);
    if report.status != ValidationStatus::Valid {
        return Err(ContainmentProfileError::ValidationFailed);
    }
    Ok(ContainmentProfileProofEvidence {
        schema_version: 1,
        system_ids: vec!["representation-selector".into(), "forge-reference-studio".into()],
        proof_id: "bounded-p7b1a-containment-profile-policy".into(),
        fixture_id: "containment-profile-v1/inert-lpac-candidate".into(),
        measurement_classification: "simulated".into(),
        package_fingerprint: crate::hex(&package.fingerprint()?),
        boundary_fingerprint: crate::hex(&package.boundary.fingerprint()?),
        examined: report.examined,
        violations: report.violations.len(),
        capabilities: vec![],
        limitations: vec![
            "Serialized policy records only; no sandbox, process, installer, tool, renderer, file, network, GPU, asset, or image was opened or executed.".into(),
            "A valid policy receipt does not prove operating-system containment, denial behavior, tool compatibility, or renderer safety.".into(),
            "The AppContainer/LPAC candidate remains unselected until a separate owner-authorized harmless denial canary.".into(),
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
            ContainmentProfilePackage::from_bytes(&bytes).unwrap(),
            package
        );
        let mut spaced = bytes.clone();
        spaced.push(b' ');
        assert_eq!(
            ContainmentProfilePackage::from_bytes(&spaced),
            Err(ContainmentProfileError::NonCanonical)
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("execute_renderer".into(), serde_json::json!(true));
        assert!(
            ContainmentProfilePackage::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
        );
    }

    #[test]
    fn policy_stage_cannot_select_or_bind_a_tool() {
        let mut package = reference_package().unwrap();
        package.tool.selection_state = SelectionState::OwnerApprovedTool;
        package.tool.binary_fingerprint = Some(id(90));
        assert!(has(
            &validate_package(&package, 128),
            "policy_stage_selected_or_bound_tool"
        ));
    }

    #[test]
    fn supply_chain_requirements_cannot_be_implicit() {
        let mut package = reference_package().unwrap();
        package.tool.publisher_signature_required = false;
        package.tool.removal_plan.clear();
        assert!(has(
            &validate_package(&package, 128),
            "incomplete_supply_chain_policy"
        ));
    }

    #[test]
    fn weak_boundaries_cannot_claim_security_boundary_status() {
        for kind in [
            BoundaryKind::JobObjectOnly,
            BoundaryKind::WslDistribution,
            BoundaryKind::FullTrustPackage,
            BoundaryKind::ProcessIsolatedContainer,
            BoundaryKind::RestrictedDirectoryOnly,
            BoundaryKind::OrdinaryProcess,
        ] {
            let mut package = reference_package().unwrap();
            package.boundary.kind = kind;
            assert!(has(
                &validate_package(&package, 128),
                "false_security_boundary_claim"
            ));
        }
    }

    #[test]
    fn job_object_cannot_be_the_package_boundary_even_when_labelled_supporting() {
        let mut package = reference_package().unwrap();
        package.boundary.kind = BoundaryKind::JobObjectOnly;
        package.boundary.role = BoundaryRole::SupportingControl;
        package.receipt.boundary_fingerprint = package.boundary.fingerprint().unwrap();
        assert!(has(
            &validate_package(&package, 128),
            "false_security_boundary_claim"
        ));
    }

    #[test]
    fn host_and_prerequisite_evidence_must_be_nonzero_and_unique() {
        let mut package = reference_package().unwrap();
        package.boundary.host_profile_ref = [0; 32];
        package.boundary.prerequisite_refs = vec![id(2), id(2)];
        assert!(has(
            &validate_package(&package, 128),
            "missing_host_or_prerequisite_evidence"
        ));
    }

    #[test]
    fn capability_deny_set_is_complete_unique_and_ungranted() {
        let mut package = reference_package().unwrap();
        package.boundary.denied_capabilities.pop();
        package
            .boundary
            .granted_capabilities
            .push(Capability::Network);
        assert!(has(
            &validate_package(&package, 128),
            "capability_deny_set_incomplete_or_granted"
        ));
    }

    #[test]
    fn hostile_input_matrix_includes_a3_and_active_content() {
        let mut package = reference_package().unwrap();
        package.input.forbidden_hazards.retain(|item| {
            *item != InputHazard::ReparsePoint && *item != InputHazard::ActiveContent
        });
        assert!(has(
            &validate_package(&package, 128),
            "hostile_input_matrix_incomplete"
        ));
    }

    #[test]
    fn repository_or_user_input_and_mutable_input_fail() {
        let mut package = reference_package().unwrap();
        package.input.repository_paths_allowed = true;
        package.input.user_directories_allowed = true;
        package.input.read_only = false;
        assert!(has(&validate_package(&package, 128), "unsafe_input_policy"));
    }

    #[test]
    fn quarantine_cannot_be_reused_previewed_or_write_durable_state() {
        let mut package = reference_package().unwrap();
        package.output.fresh_quarantine = false;
        package.output.direct_preview = true;
        package.output.direct_durable_write = true;
        assert!(has(
            &validate_package(&package, 128),
            "unsafe_output_quarantine"
        ));
    }

    #[test]
    fn runner_access_must_be_revoked_before_any_admission_parser() {
        let mut package = reference_package().unwrap();
        package.output.admission_steps.swap(0, 1);
        assert!(has(
            &validate_package(&package, 128),
            "unsafe_or_reordered_output_admission"
        ));
    }

    #[test]
    fn output_types_require_an_explicit_unique_allowlist() {
        let mut package = reference_package().unwrap();
        package.output.allowed_type_classes = vec!["".into(), "".into()];
        assert!(has(
            &validate_package(&package, 128),
            "invalid_output_allowlist"
        ));
    }

    #[test]
    fn every_resource_dimension_is_bounded() {
        let mut package = reference_package().unwrap();
        package.budget.max_wall_ms = 0;
        package.budget.max_memory_bytes = u64::MAX;
        package.budget.max_output_files = u32::MAX;
        assert!(has(
            &validate_package(&package, 128),
            "unbounded_or_excessive_resource_budget"
        ));
    }

    #[test]
    fn full_tree_termination_is_required_at_boundary_and_recovery_layers() {
        let mut package = reference_package().unwrap();
        package.boundary.full_process_tree_termination = false;
        package.recovery.terminate_full_process_tree = false;
        let report = validate_package(&package, 128);
        assert!(has(&report, "missing_supporting_process_controls"));
        assert!(has(&report, "incomplete_recovery_plan"));
    }

    #[test]
    fn recovery_requires_disposal_immutability_and_new_retry_identity() {
        let mut package = reference_package().unwrap();
        package.recovery.dispose_boundary = false;
        package.recovery.prove_project_immutability = false;
        package.recovery.retry_with_new_run_id = false;
        assert!(has(
            &validate_package(&package, 128),
            "incomplete_recovery_plan"
        ));
    }

    #[test]
    fn receipt_bindings_and_denial_count_are_recomputed() {
        let mut package = reference_package().unwrap();
        package.receipt.input_fingerprint = id(88);
        package.receipt.denied_capability_count = 99;
        assert!(has(
            &validate_package(&package, 128),
            "stale_or_fabricated_readiness_receipt"
        ));
    }

    #[test]
    fn policy_receipt_cannot_claim_runtime_containment() {
        let mut package = reference_package().unwrap();
        package.receipt.status = "runtime_containment_proven".into();
        package.receipt.limitations.clear();
        assert!(has(
            &validate_package(&package, 128),
            "readiness_claim_overreach"
        ));
    }

    #[test]
    fn version_drift_and_budget_exhaustion_fail_closed() {
        let mut package = reference_package().unwrap();
        package.receipt.schema_version = 2;
        assert!(has(&validate_package(&package, 128), "unknown_schema"));
        assert_eq!(
            validate_package(&reference_package().unwrap(), 1).status,
            ValidationStatus::IndeterminateBudget
        );
    }

    #[test]
    fn reference_is_deterministic_capability_free_and_authority_negative() {
        let first = reference_package().unwrap();
        let second = reference_package().unwrap();
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            validate_package(&first, 128).status,
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
