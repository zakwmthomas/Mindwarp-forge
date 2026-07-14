use serde::{Deserialize, Serialize};

use crate::{ContainmentProfileError, canonical_json, hash};

const TOOL_DOMAIN: &[u8] = b"mindwarp.containment-profile.tool.v1";
const BOUNDARY_DOMAIN: &[u8] = b"mindwarp.containment-profile.boundary.v1";
const INPUT_DOMAIN: &[u8] = b"mindwarp.containment-profile.input.v1";
const OUTPUT_DOMAIN: &[u8] = b"mindwarp.containment-profile.output.v1";
const BUDGET_DOMAIN: &[u8] = b"mindwarp.containment-profile.budget.v1";
const RECOVERY_DOMAIN: &[u8] = b"mindwarp.containment-profile.recovery.v1";
const PACKAGE_DOMAIN: &[u8] = b"mindwarp.containment-profile.package.v1";

pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStage {
    PolicyOnly,
    DenialCanary,
    ToolCompatibility,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionState {
    Unselected,
    OwnerApprovedCanary,
    OwnerApprovedTool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryKind {
    AppContainer,
    LessPrivilegedAppContainer,
    HypervisorSandbox,
    HypervisorContainer,
    ProcessIsolatedContainer,
    WslDistribution,
    FullTrustPackage,
    JobObjectOnly,
    RestrictedDirectoryOnly,
    OrdinaryProcess,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryRole {
    SecurityBoundaryCandidate,
    SupportingControl,
    Rejected,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Network,
    Credentials,
    Clipboard,
    Devices,
    HostUi,
    Gpu,
    Plugins,
    Scripting,
    PackageManager,
    Repository,
    ExistingUserWrite,
    RegistryWrite,
    ChildEscape,
    Elevation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InputHazard {
    Traversal,
    AbsolutePath,
    UncPath,
    DevicePath,
    ReservedName,
    EnvironmentExpansion,
    AlternateDataStream,
    ReparsePoint,
    ExternalReference,
    Archive,
    NativeProject,
    ActiveContent,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionStep {
    RevokeRunnerAccess,
    PathAndReparseCheck,
    CountSizeDepthCheck,
    SignatureAndTypeCheck,
    ContentHash,
    ManifestCompleteness,
    BoundedFormatValidation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineFailureAction {
    RetainInertEvidence,
    SafeDiscard,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolIdentity {
    pub schema_version: u16,
    pub stage: ReadinessStage,
    pub selection_state: SelectionState,
    pub tool_class: String,
    pub official_source_required: bool,
    pub publisher_signature_required: bool,
    pub binary_fingerprint: Option<Id>,
    pub dependency_fingerprints: Vec<Id>,
    pub license_evidence_ref: Option<Id>,
    pub version: Option<String>,
    pub auto_update_disabled: bool,
    pub removal_plan: String,
}

impl ToolIdentity {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(TOOL_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryProfile {
    pub schema_version: u16,
    pub kind: BoundaryKind,
    pub role: BoundaryRole,
    pub host_profile_ref: Id,
    pub prerequisite_refs: Vec<Id>,
    pub granted_capabilities: Vec<Capability>,
    pub denied_capabilities: Vec<Capability>,
    pub job_object_required: bool,
    pub full_process_tree_termination: bool,
}

impl BoundaryProfile {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(BOUNDARY_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputPolicy {
    pub schema_version: u16,
    pub generated_synthetic_only: bool,
    pub fresh_per_run: bool,
    pub read_only: bool,
    pub content_addressed: bool,
    pub repository_paths_allowed: bool,
    pub user_directories_allowed: bool,
    pub forbidden_hazards: Vec<InputHazard>,
}

impl InputPolicy {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(INPUT_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputPolicy {
    pub schema_version: u16,
    pub fresh_quarantine: bool,
    pub outside_repository: bool,
    pub outside_existing_user_content: bool,
    pub per_run_identity: bool,
    pub direct_durable_write: bool,
    pub direct_preview: bool,
    pub admission_steps: Vec<AdmissionStep>,
    pub allowed_type_classes: Vec<String>,
}

impl OutputPolicy {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(OUTPUT_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceBudget {
    pub schema_version: u16,
    pub max_wall_ms: u64,
    pub max_cpu_ms: u64,
    pub max_memory_bytes: u64,
    pub max_processes: u32,
    pub max_output_bytes: u64,
    pub max_output_files: u32,
    pub max_nesting: u16,
    pub max_dimension: u32,
    pub max_geometry_items: u64,
    pub max_texture_items: u32,
    pub max_frames: u32,
}

impl ResourceBudget {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(BUDGET_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryPlan {
    pub schema_version: u16,
    pub retain_failure_evidence: bool,
    pub terminate_full_process_tree: bool,
    pub dispose_boundary: bool,
    pub revoke_output_access_before_admission: bool,
    pub prove_project_immutability: bool,
    pub retry_with_new_run_id: bool,
    pub failure_action: QuarantineFailureAction,
}

impl RecoveryPlan {
    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(RECOVERY_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainmentReadinessReceipt {
    pub schema_version: u16,
    pub stage: ReadinessStage,
    pub tool_fingerprint: Id,
    pub boundary_fingerprint: Id,
    pub input_fingerprint: Id,
    pub output_fingerprint: Id,
    pub budget_fingerprint: Id,
    pub recovery_fingerprint: Id,
    pub status: String,
    pub denied_capability_count: u32,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainmentProfilePackage {
    pub schema_version: u16,
    pub tool: ToolIdentity,
    pub boundary: BoundaryProfile,
    pub input: InputPolicy,
    pub output: OutputPolicy,
    pub budget: ResourceBudget,
    pub recovery: RecoveryPlan,
    pub receipt: ContainmentReadinessReceipt,
}

impl ContainmentProfilePackage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ContainmentProfileError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ContainmentProfileError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| ContainmentProfileError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(ContainmentProfileError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<Id, ContainmentProfileError> {
        Ok(hash(PACKAGE_DOMAIN, &self.to_bytes()?))
    }
}
