use std::collections::BTreeSet;

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

fn violation(code: &str, location: impl Into<String>) -> Violation {
    Violation {
        code: code.into(),
        location: location.into(),
    }
}

fn nonzero(id: &Id) -> bool {
    id.iter().any(|byte| *byte != 0)
}

fn unique_complete<T: Copy + Ord>(actual: &[T], required: &[T]) -> bool {
    let actual_set: BTreeSet<_> = actual.iter().copied().collect();
    let required_set: BTreeSet<_> = required.iter().copied().collect();
    actual_set.len() == actual.len() && actual_set == required_set
}

pub fn validate_package(package: &ContainmentProfilePackage, budget: u32) -> ValidationReport {
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
        ("tool", package.tool.schema_version),
        ("boundary", package.boundary.schema_version),
        ("input", package.input.schema_version),
        ("output", package.output.schema_version),
        ("budget", package.budget.schema_version),
        ("recovery", package.recovery.schema_version),
        ("receipt", package.receipt.schema_version),
    ] {
        examine!();
        if version != CONTRACT_VERSION {
            report.violations.push(violation("unknown_schema", name));
        }
    }

    examine!();
    let tool = &package.tool;
    if tool.stage != ReadinessStage::PolicyOnly
        || tool.selection_state != SelectionState::Unselected
        || tool.binary_fingerprint.is_some()
        || !tool.dependency_fingerprints.is_empty()
        || tool.license_evidence_ref.is_some()
        || tool.version.is_some()
    {
        report
            .violations
            .push(violation("policy_stage_selected_or_bound_tool", "tool"));
    }
    if tool.tool_class.trim().is_empty()
        || !tool.official_source_required
        || !tool.publisher_signature_required
        || !tool.auto_update_disabled
        || tool.removal_plan.trim().is_empty()
    {
        report
            .violations
            .push(violation("incomplete_supply_chain_policy", "tool"));
    }

    examine!();
    let boundary = &package.boundary;
    let candidate_kind = matches!(
        boundary.kind,
        BoundaryKind::AppContainer
            | BoundaryKind::LessPrivilegedAppContainer
            | BoundaryKind::HypervisorSandbox
            | BoundaryKind::HypervisorContainer
    );
    let role_valid = boundary.role == BoundaryRole::SecurityBoundaryCandidate && candidate_kind;
    if !role_valid {
        report.violations.push(violation(
            "false_security_boundary_claim",
            "boundary.kind_or_role",
        ));
    }
    let prerequisite_refs: BTreeSet<_> = boundary.prerequisite_refs.iter().collect();
    if !nonzero(&boundary.host_profile_ref)
        || boundary.prerequisite_refs.is_empty()
        || boundary.prerequisite_refs.iter().any(|id| !nonzero(id))
        || prerequisite_refs.len() != boundary.prerequisite_refs.len()
    {
        report.violations.push(violation(
            "missing_host_or_prerequisite_evidence",
            "boundary",
        ));
    }
    let required_denials = [
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
    ];
    if !boundary.granted_capabilities.is_empty()
        || !unique_complete(&boundary.denied_capabilities, &required_denials)
    {
        report.violations.push(violation(
            "capability_deny_set_incomplete_or_granted",
            "boundary.capabilities",
        ));
    }
    if !boundary.job_object_required || !boundary.full_process_tree_termination {
        report
            .violations
            .push(violation("missing_supporting_process_controls", "boundary"));
    }

    examine!();
    let input = &package.input;
    if !input.generated_synthetic_only
        || !input.fresh_per_run
        || !input.read_only
        || !input.content_addressed
        || input.repository_paths_allowed
        || input.user_directories_allowed
    {
        report
            .violations
            .push(violation("unsafe_input_policy", "input"));
    }
    let required_hazards = [
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
    ];
    if !unique_complete(&input.forbidden_hazards, &required_hazards) {
        report.violations.push(violation(
            "hostile_input_matrix_incomplete",
            "input.forbidden_hazards",
        ));
    }

    examine!();
    let output = &package.output;
    if !output.fresh_quarantine
        || !output.outside_repository
        || !output.outside_existing_user_content
        || !output.per_run_identity
        || output.direct_durable_write
        || output.direct_preview
    {
        report
            .violations
            .push(violation("unsafe_output_quarantine", "output"));
    }
    let required_steps = [
        AdmissionStep::RevokeRunnerAccess,
        AdmissionStep::PathAndReparseCheck,
        AdmissionStep::CountSizeDepthCheck,
        AdmissionStep::SignatureAndTypeCheck,
        AdmissionStep::ContentHash,
        AdmissionStep::ManifestCompleteness,
        AdmissionStep::BoundedFormatValidation,
    ];
    if output.admission_steps.as_slice() != required_steps {
        report.violations.push(violation(
            "unsafe_or_reordered_output_admission",
            "output.admission_steps",
        ));
    }
    let type_classes: BTreeSet<_> = output.allowed_type_classes.iter().collect();
    if output.allowed_type_classes.is_empty()
        || output
            .allowed_type_classes
            .iter()
            .any(|item| item.trim().is_empty())
        || type_classes.len() != output.allowed_type_classes.len()
    {
        report.violations.push(violation(
            "invalid_output_allowlist",
            "output.allowed_type_classes",
        ));
    }

    examine!();
    let limits = &package.budget;
    let bounded = limits.max_wall_ms > 0
        && limits.max_wall_ms <= 60_000
        && limits.max_cpu_ms > 0
        && limits.max_cpu_ms <= 60_000
        && limits.max_memory_bytes > 0
        && limits.max_memory_bytes <= 1_073_741_824
        && limits.max_processes > 0
        && limits.max_processes <= 8
        && limits.max_output_bytes > 0
        && limits.max_output_bytes <= 134_217_728
        && limits.max_output_files > 0
        && limits.max_output_files <= 1_024
        && limits.max_nesting > 0
        && limits.max_nesting <= 16
        && limits.max_dimension > 0
        && limits.max_dimension <= 16_384
        && limits.max_geometry_items > 0
        && limits.max_geometry_items <= 10_000_000
        && limits.max_texture_items > 0
        && limits.max_texture_items <= 1_024
        && limits.max_frames > 0
        && limits.max_frames <= 600;
    if !bounded {
        report.violations.push(violation(
            "unbounded_or_excessive_resource_budget",
            "budget",
        ));
    }

    examine!();
    let recovery = &package.recovery;
    if !recovery.retain_failure_evidence
        || !recovery.terminate_full_process_tree
        || !recovery.dispose_boundary
        || !recovery.revoke_output_access_before_admission
        || !recovery.prove_project_immutability
        || !recovery.retry_with_new_run_id
    {
        report
            .violations
            .push(violation("incomplete_recovery_plan", "recovery"));
    }

    examine!();
    let receipt = &package.receipt;
    let expected = (
        package.tool.fingerprint().unwrap_or([0; 32]),
        package.boundary.fingerprint().unwrap_or([0; 32]),
        package.input.fingerprint().unwrap_or([0; 32]),
        package.output.fingerprint().unwrap_or([0; 32]),
        package.budget.fingerprint().unwrap_or([0; 32]),
        package.recovery.fingerprint().unwrap_or([0; 32]),
    );
    if receipt.stage != ReadinessStage::PolicyOnly
        || receipt.tool_fingerprint != expected.0
        || receipt.boundary_fingerprint != expected.1
        || receipt.input_fingerprint != expected.2
        || receipt.output_fingerprint != expected.3
        || receipt.budget_fingerprint != expected.4
        || receipt.recovery_fingerprint != expected.5
        || receipt.denied_capability_count != required_denials.len() as u32
    {
        report.violations.push(violation(
            "stale_or_fabricated_readiness_receipt",
            "receipt.bindings",
        ));
    }
    if receipt.status != "policy_ready_not_executed"
        || !receipt
            .limitations
            .iter()
            .any(|item| item == "does_not_prove_runtime_containment")
        || !receipt
            .limitations
            .iter()
            .any(|item| item == "no_execution_authority")
    {
        report.violations.push(violation(
            "readiness_claim_overreach",
            "receipt.status_or_limitations",
        ));
    }

    report.violations.sort();
    report.violations.dedup();
    if !report.violations.is_empty() {
        report.status = ValidationStatus::Invalid;
    }
    report
}
