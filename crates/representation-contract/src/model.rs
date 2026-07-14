use serde::{Deserialize, Serialize};

use crate::{CONTRACT_VERSION, RepresentationContractError, canonical_json, hash};

const PACKAGE_DOMAIN: &[u8] = b"mindwarp.representation-contract.package.v1";
const DECISION_DOMAIN: &[u8] = b"mindwarp.representation-contract.decision.v1";
const ARTIFACT_DOMAIN: &[u8] = b"mindwarp.representation-contract.artifact.v1";

pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationFamily {
    NeutralSurface,
    VolumetricField,
    RigidAssembly,
    DeformableSurface,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementClass {
    Measured,
    Simulated,
    Estimated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TradeEvidence {
    pub dimension_id: Id,
    pub value: i32,
    pub unit: String,
    pub classification: MeasurementClass,
    pub method: String,
    pub uncertainty: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationOption {
    pub id: Id,
    pub family: RepresentationFamily,
    pub mechanism_evidence: Vec<Id>,
    pub requirement_refs: Vec<Id>,
    pub hard_constraints_satisfied: bool,
    pub trade_vector: Vec<TradeEvidence>,
    pub rejection_reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationPortfolio {
    pub options: Vec<RepresentationOption>,
    pub selected_option: Option<Id>,
    pub selection_rationale: Vec<String>,
    pub single_feasible_representation: Option<String>,
}

impl RepresentationPortfolio {
    pub fn fingerprint(&self) -> Result<Id, RepresentationContractError> {
        Ok(hash(DECISION_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LogicalReference {
    pub reference_id: Id,
    pub content_fingerprint: Id,
    pub locator: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivativeKind {
    Fidelity,
    MaterialVariant,
    TemporalCompression,
    Conversion,
    RepairCandidate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivativeRecord {
    pub id: Id,
    pub parent_id: Id,
    pub kind: DerivativeKind,
    pub method_profile: Id,
    pub declared_loss: Vec<TradeEvidence>,
    pub validation_ref: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairResult {
    Rejected,
    Quarantined,
    ValidatedCandidate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepairAttempt {
    pub id: Id,
    pub parent_id: Id,
    pub allowed_scope: Vec<Id>,
    pub changed_scope: Vec<Id>,
    pub candidate_id: Option<Id>,
    pub result: RepairResult,
    pub validation_ref: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactManifest {
    pub schema_version: u16,
    pub artifact_id: Id,
    pub recipe_fingerprint: Id,
    pub decision_fingerprint: Id,
    pub generator_profile: Id,
    pub references: Vec<LogicalReference>,
    pub derivatives: Vec<DerivativeRecord>,
    pub repairs: Vec<RepairAttempt>,
}

pub fn artifact_identity(recipe: Id, decision: Id, generator: Id) -> Id {
    let mut bytes = Vec::with_capacity(96);
    bytes.extend_from_slice(&recipe);
    bytes.extend_from_slice(&decision);
    bytes.extend_from_slice(&generator);
    hash(ARTIFACT_DOMAIN, &bytes)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialRegionBinding {
    pub region_id: Id,
    pub source_role: Id,
    pub boundary_refs: Vec<Id>,
    pub appearance_constraint_refs: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialRegionPlan {
    pub schema_version: u16,
    pub regions: Vec<MaterialRegionBinding>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Handedness {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformOrder {
    ScaleRotateTranslate,
    RotateTranslate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalFrame {
    pub id: Id,
    pub source_role: Id,
    pub handedness: Handedness,
    pub linear_unit: String,
    pub angular_unit: String,
    pub transform_order: TransformOrder,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DegreeOfFreedom {
    pub id: Id,
    pub frame_id: Id,
    pub source_role: Id,
    pub minimum: i32,
    pub maximum: i32,
    pub unit: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArticulationPlan {
    pub schema_version: u16,
    pub frames: Vec<LocalFrame>,
    pub degrees_of_freedom: Vec<DegreeOfFreedom>,
    pub symbolic_contact_refs: Vec<Id>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FidelityTier {
    Dormant,
    Coarse,
    Standard,
    Protected,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InterpolationMode {
    Hold,
    Linear,
    DeclaredSpline,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalTierMapping {
    pub tier: FidelityTier,
    pub fidelity_level: u16,
    pub cadence_units: u16,
    pub interpolation: InterpolationMode,
    pub fallback_tier: Option<FidelityTier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalFidelityPlan {
    pub schema_version: u16,
    pub importance_packet_ref: Id,
    pub importance_policy_version: u16,
    pub request_epoch: u64,
    pub mappings: Vec<TemporalTierMapping>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewConditions {
    pub renderer_profile_ref: Id,
    pub camera_profile_ref: Id,
    pub lighting_profile_ref: Id,
    pub color_profile_ref: Id,
    pub assertion_refs: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewCase {
    pub schema_version: u16,
    pub artifact_ref: Id,
    pub conditions: ReviewConditions,
    pub rendered_evidence_refs: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationContractPackage {
    pub schema_version: u16,
    pub semantic_package_ref: Id,
    pub recipe_ref: Id,
    pub portfolio: RepresentationPortfolio,
    pub manifest: ArtifactManifest,
    pub materials: MaterialRegionPlan,
    pub articulation: ArticulationPlan,
    pub temporal: TemporalFidelityPlan,
    pub review: ReviewCase,
}

impl RepresentationContractPackage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, RepresentationContractError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RepresentationContractError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| RepresentationContractError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(RepresentationContractError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<Id, RepresentationContractError> {
        Ok(hash(PACKAGE_DOMAIN, &self.to_bytes()?))
    }
}

pub fn ensure_contract_version(version: u16) -> Result<(), RepresentationContractError> {
    if version != CONTRACT_VERSION {
        return Err(RepresentationContractError::Invalid(
            "unsupported contract version",
        ));
    }
    Ok(())
}
