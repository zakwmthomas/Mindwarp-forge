use serde::{Deserialize, Serialize};

use crate::{PerceptionProtocolError, canonical_json, hash};

const PROTOCOL_DOMAIN: &[u8] = b"mindwarp.perception-protocol.review.v1";
const ENVIRONMENT_DOMAIN: &[u8] = b"mindwarp.perception-protocol.environment.v1";
const STIMULUS_DOMAIN: &[u8] = b"mindwarp.perception-protocol.stimulus.v1";
const OBSERVATION_DOMAIN: &[u8] = b"mindwarp.perception-protocol.observation.v1";
const PACKAGE_DOMAIN: &[u8] = b"mindwarp.perception-protocol.package.v1";

pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionClass {
    DirectorialPreference,
    DefectDetection,
    Recognisability,
    FunctionalLegibility,
    TemporalContinuity,
    ComparativeFidelity,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationMethod {
    BlindPair,
    SingleStimulus,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOutcome {
    Satisfied,
    Violated,
    NoPreference,
    Indeterminate,
    NotObserved,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerClass {
    CreativeDirector,
    Expert,
    Naive,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClass {
    ProjectDirection,
    ExpertDefect,
    PopulationPreference,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReproducibilityClass {
    ExactSameEnvironment,
    SemanticCrossEnvironment,
    Unverified,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationMode {
    Representative,
    Silhouette,
    RegionDiagnostic,
    TopologyDiagnostic,
    TemporalSequence,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlKind {
    DuplicatePair,
    SwappedOrder,
    ObviousGood,
    ObviousBad,
    MissingView,
    MisleadingLighting,
    StaleDerivative,
    MetricContradiction,
    BrokenConnection,
    SilhouetteCollapse,
    ArticulationDrift,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionSpec {
    pub assertion_id: Id,
    pub question_class: QuestionClass,
    pub statement: String,
    pub required_modes: Vec<PresentationMode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewProtocol {
    pub schema_version: u16,
    pub artifact_ref: Id,
    pub derivative_refs: Vec<Id>,
    pub assertions: Vec<AssertionSpec>,
    pub method: PresentationMethod,
    pub randomization_profile: Id,
    pub blinding_profile: Id,
    pub anchor_refs: Vec<Id>,
    pub controls: Vec<ControlKind>,
    pub repeat_assertion_refs: Vec<Id>,
    pub stop_rule: String,
    pub allowed_outcomes: Vec<ReviewOutcome>,
}

impl ReviewProtocol {
    pub fn fingerprint(&self) -> Result<Id, PerceptionProtocolError> {
        Ok(hash(PROTOCOL_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentProfile {
    pub schema_version: u16,
    pub tool_profile: Id,
    pub tool_binary_fingerprint: Id,
    pub tool_config_fingerprint: Id,
    pub os_profile: String,
    pub device_profile: String,
    pub driver_profile: String,
    pub deterministic_seed: u64,
    pub sampling_profile: String,
    pub coordinate_profile: String,
    pub unit_profile: String,
    pub camera_profile: Id,
    pub projection_profile: Id,
    pub framing_profile: Id,
    pub width: u32,
    pub height: u32,
    pub time_samples: Vec<u32>,
    pub lighting_profile: Id,
    pub background_profile: Id,
    pub presentation_modes: Vec<PresentationMode>,
    pub color_config_fingerprint: Id,
    pub output_transform: String,
    pub display_conditions: String,
    pub reproducibility: ReproducibilityClass,
}

impl EnvironmentProfile {
    pub fn fingerprint(&self) -> Result<Id, PerceptionProtocolError> {
        Ok(hash(ENVIRONMENT_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusPair {
    pub pair_id: Id,
    pub left_derivative_ref: Id,
    pub right_derivative_ref: Id,
    pub left_label: String,
    pub right_label: String,
    pub order_token: Id,
    pub assertion_refs: Vec<Id>,
    pub presentation_modes: Vec<PresentationMode>,
    pub control: Option<ControlKind>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusManifest {
    pub schema_version: u16,
    pub protocol_fingerprint: Id,
    pub environment_fingerprint: Id,
    pub immutable_input_refs: Vec<Id>,
    pub pairs: Vec<StimulusPair>,
    pub execution_receipt_ref: Id,
    pub opaque_stimulus_refs: Vec<Id>,
    pub omissions: Vec<String>,
}

impl StimulusManifest {
    pub fn fingerprint(&self) -> Result<Id, PerceptionProtocolError> {
        Ok(hash(STIMULUS_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub observation_id: Id,
    pub pair_id: Id,
    pub assertion_id: Id,
    pub reviewer_class: ReviewerClass,
    pub claim_class: ClaimClass,
    pub outcome: ReviewOutcome,
    pub confidence: u8,
    pub reason_code: String,
    pub limitations: Vec<String>,
    pub presentation_order: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationSet {
    pub schema_version: u16,
    pub protocol_fingerprint: Id,
    pub stimulus_fingerprint: Id,
    pub observations: Vec<Observation>,
    pub contradiction_refs: Vec<Id>,
}

impl ObservationSet {
    pub fn fingerprint(&self) -> Result<Id, PerceptionProtocolError> {
        Ok(hash(OBSERVATION_DOMAIN, &canonical_json(self)?))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionSummary {
    pub assertion_id: Id,
    pub satisfied: u32,
    pub violated: u32,
    pub no_preference: u32,
    pub indeterminate: u32,
    pub not_observed: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisReceipt {
    pub schema_version: u16,
    pub protocol_fingerprint: Id,
    pub stimulus_fingerprint: Id,
    pub observation_fingerprint: Id,
    pub summaries: Vec<AssertionSummary>,
    pub control_failures: Vec<Id>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerceptionProtocolPackage {
    pub schema_version: u16,
    pub protocol: ReviewProtocol,
    pub environment: EnvironmentProfile,
    pub stimuli: StimulusManifest,
    pub observations: ObservationSet,
    pub analysis: AnalysisReceipt,
}

impl PerceptionProtocolPackage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PerceptionProtocolError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PerceptionProtocolError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| PerceptionProtocolError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(PerceptionProtocolError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<Id, PerceptionProtocolError> {
        Ok(hash(PACKAGE_DOMAIN, &self.to_bytes()?))
    }
}
