//! Capability-free H3 engine-neutral structural candidate generation.

use std::fmt;

use reference_viewport::reference_scene;
use representation_contract::{
    NeutralHumanoidProfile, reference_neutral_humanoid_profile, validate_neutral_humanoid_profile,
};
use semantic_construction::{
    SemanticConstructionPackage, ValidationStatus, reference_package, validate_package,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SCHEMA_VERSION: u16 = 1;
const INPUT_DOMAIN: &[u8] = b"mindwarp.humanoid-generation.input.v1";
const CANDIDATE_DOMAIN: &[u8] = b"mindwarp.humanoid-generation.candidate.v1";
const ID_DOMAIN: &[u8] = b"mindwarp.humanoid-generation.id.v1";
const MAX_JOINTS: usize = 64;
const MAX_LINKS: usize = 96;
const VALIDATION_BUDGET: u32 = 512;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerationError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for GenerationError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateAuthority {
    UnapprovedStructuralCandidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProhibitedCapability {
    Filesystem,
    Process,
    Network,
    Clock,
    Randomness,
    Plugin,
    ExternalExecutable,
    ProtectedKernelMutation,
    Approval,
    Promotion,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationInput {
    pub schema_version: u16,
    pub request_id: String,
    pub semantic_package_fingerprint: [u8; 32],
    pub semantic_fingerprint: [u8; 32],
    pub recipe_result_fingerprint: [u8; 32],
    pub representation_profile_fingerprint: [u8; 32],
    pub generator_profile: String,
    pub maximum_joints: u16,
    pub maximum_links: u16,
    pub authority: CandidateAuthority,
    pub prohibited_capabilities: Vec<ProhibitedCapability>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedJoint {
    pub joint_id: String,
    pub semantic_role: String,
    pub rest_position: [i32; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedLink {
    pub parent_joint_id: String,
    pub child_joint_id: String,
    pub link_role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralCandidate {
    pub schema_version: u16,
    pub candidate_id: [u8; 32],
    pub input_fingerprint: [u8; 32],
    pub joints: Vec<GeneratedJoint>,
    pub links: Vec<GeneratedLink>,
    pub authority: CandidateAuthority,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationReceipt {
    pub schema_version: u16,
    pub input_fingerprint: String,
    pub candidate_fingerprint: String,
    pub replay_candidate_fingerprint: String,
    pub joint_count: u16,
    pub link_count: u16,
    pub deterministic_replay: bool,
    pub inputs_unchanged: bool,
    pub capability_free: bool,
    pub limitations: Vec<String>,
}

impl GenerationInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GenerationError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GenerationError> {
        strict_decode(bytes)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], GenerationError> {
        Ok(hash(INPUT_DOMAIN, &self.to_bytes()?))
    }
}

impl StructuralCandidate {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GenerationError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GenerationError> {
        strict_decode(bytes)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], GenerationError> {
        Ok(hash(CANDIDATE_DOMAIN, &self.to_bytes()?))
    }
}

pub fn reference_input(
    semantic: &SemanticConstructionPackage,
    profile: &NeutralHumanoidProfile,
) -> Result<GenerationInput, GenerationError> {
    validate_sources(semantic, profile)?;
    let input = GenerationInput {
        schema_version: SCHEMA_VERSION,
        request_id: "h3-neutral-humanoid-generation-v1".into(),
        semantic_package_fingerprint: semantic
            .fingerprint()
            .map_err(|_| GenerationError::Invalid("P6 package cannot be fingerprinted"))?,
        semantic_fingerprint: semantic
            .semantic_fingerprint()
            .map_err(|_| GenerationError::Invalid("P6 semantics cannot be fingerprinted"))?,
        recipe_result_fingerprint: semantic.recipe.expected_result,
        representation_profile_fingerprint: profile
            .fingerprint()
            .map_err(|_| GenerationError::Invalid("H2 profile cannot be fingerprinted"))?,
        generator_profile: "pure-structural-projection-v1".into(),
        maximum_joints: MAX_JOINTS as u16,
        maximum_links: MAX_LINKS as u16,
        authority: CandidateAuthority::UnapprovedStructuralCandidate,
        prohibited_capabilities: required_prohibitions(),
    };
    validate_input(&input, semantic, profile)?;
    Ok(input)
}

pub fn generate(
    input: &GenerationInput,
    semantic: &SemanticConstructionPackage,
    profile: &NeutralHumanoidProfile,
) -> Result<StructuralCandidate, GenerationError> {
    validate_input(input, semantic, profile)?;
    let joints: Vec<_> = profile
        .joints
        .iter()
        .map(|joint| GeneratedJoint {
            joint_id: joint.joint_id.clone(),
            semantic_role: joint.semantic_role.clone(),
            rest_position: joint.rest_position,
        })
        .collect();
    let links: Vec<_> = profile
        .joints
        .iter()
        .filter_map(|joint| {
            Some(GeneratedLink {
                parent_joint_id: joint.parent_joint_id.clone()?,
                child_joint_id: joint.joint_id.clone(),
                link_role: joint.link_role.clone()?,
            })
        })
        .collect();
    if joints.len() > usize::from(input.maximum_joints)
        || links.len() > usize::from(input.maximum_links)
    {
        return Err(GenerationError::Invalid("declared output budget exhausted"));
    }
    let input_fingerprint = input.fingerprint()?;
    let mut identity_bytes = Vec::new();
    identity_bytes.extend_from_slice(&input_fingerprint);
    identity_bytes.extend_from_slice(&canonical_json(&(joints.as_slice(), links.as_slice()))?);
    let candidate = StructuralCandidate {
        schema_version: SCHEMA_VERSION,
        candidate_id: hash(ID_DOMAIN, &identity_bytes),
        input_fingerprint,
        joints,
        links,
        authority: CandidateAuthority::UnapprovedStructuralCandidate,
        limitations: required_limitations(),
    };
    validate_candidate(&candidate, input, profile)?;
    Ok(candidate)
}

pub fn reference_receipt() -> Result<GenerationReceipt, GenerationError> {
    let semantic = reference_package()
        .map_err(|_| GenerationError::Invalid("reference P6 package unavailable"))?;
    let profile = reference_neutral_humanoid_profile()
        .map_err(|_| GenerationError::Invalid("reference H2 profile unavailable"))?;
    let semantic_before = semantic.clone();
    let profile_before = profile.clone();
    let input = reference_input(&semantic, &profile)?;
    let first = generate(&input, &semantic, &profile)?;
    let second = generate(&input, &semantic, &profile)?;
    let first_fingerprint = first.fingerprint()?;
    let second_fingerprint = second.fingerprint()?;
    Ok(GenerationReceipt {
        schema_version: SCHEMA_VERSION,
        input_fingerprint: hex(&input.fingerprint()?),
        candidate_fingerprint: hex(&first_fingerprint),
        replay_candidate_fingerprint: hex(&second_fingerprint),
        joint_count: first.joints.len() as u16,
        link_count: first.links.len() as u16,
        deterministic_replay: first_fingerprint == second_fingerprint,
        inputs_unchanged: semantic == semantic_before && profile == profile_before,
        capability_free: true,
        limitations: required_limitations(),
    })
}

fn validate_sources(
    semantic: &SemanticConstructionPackage,
    profile: &NeutralHumanoidProfile,
) -> Result<(), GenerationError> {
    if validate_package(semantic, VALIDATION_BUDGET).status != ValidationStatus::Valid {
        return Err(GenerationError::Invalid(
            "P6 package is invalid or indeterminate",
        ));
    }
    validate_neutral_humanoid_profile(profile, &reference_scene())
        .map_err(|_| GenerationError::Invalid("H2 profile is invalid"))?;
    Ok(())
}

fn validate_input(
    input: &GenerationInput,
    semantic: &SemanticConstructionPackage,
    profile: &NeutralHumanoidProfile,
) -> Result<(), GenerationError> {
    validate_sources(semantic, profile)?;
    if input.schema_version != SCHEMA_VERSION
        || input.request_id != "h3-neutral-humanoid-generation-v1"
        || input.generator_profile != "pure-structural-projection-v1"
        || input.authority != CandidateAuthority::UnapprovedStructuralCandidate
        || input.maximum_joints as usize > MAX_JOINTS
        || input.maximum_links as usize > MAX_LINKS
        || input.prohibited_capabilities != required_prohibitions()
        || input.semantic_package_fingerprint
            != semantic
                .fingerprint()
                .map_err(|_| GenerationError::Invalid("P6 package cannot be fingerprinted"))?
        || input.semantic_fingerprint
            != semantic
                .semantic_fingerprint()
                .map_err(|_| GenerationError::Invalid("P6 semantics cannot be fingerprinted"))?
        || input.recipe_result_fingerprint != semantic.recipe.expected_result
        || input.representation_profile_fingerprint
            != profile
                .fingerprint()
                .map_err(|_| GenerationError::Invalid("H2 profile cannot be fingerprinted"))?
    {
        return Err(GenerationError::Invalid(
            "generation input drifted, exceeded bounds, or escalated authority",
        ));
    }
    Ok(())
}

pub fn validate_candidate(
    candidate: &StructuralCandidate,
    input: &GenerationInput,
    profile: &NeutralHumanoidProfile,
) -> Result<(), GenerationError> {
    let expected_joints: Vec<_> = profile
        .joints
        .iter()
        .map(|joint| GeneratedJoint {
            joint_id: joint.joint_id.clone(),
            semantic_role: joint.semantic_role.clone(),
            rest_position: joint.rest_position,
        })
        .collect();
    let expected_links: Vec<_> = profile
        .joints
        .iter()
        .filter_map(|joint| {
            Some(GeneratedLink {
                parent_joint_id: joint.parent_joint_id.clone()?,
                child_joint_id: joint.joint_id.clone(),
                link_role: joint.link_role.clone()?,
            })
        })
        .collect();
    let mut identity_bytes = Vec::new();
    identity_bytes.extend_from_slice(&input.fingerprint()?);
    identity_bytes.extend_from_slice(&canonical_json(&(
        expected_joints.as_slice(),
        expected_links.as_slice(),
    ))?);
    if candidate.schema_version != SCHEMA_VERSION
        || candidate.candidate_id != hash(ID_DOMAIN, &identity_bytes)
        || candidate.input_fingerprint != input.fingerprint()?
        || candidate.authority != CandidateAuthority::UnapprovedStructuralCandidate
        || candidate.limitations != required_limitations()
        || candidate.joints.len() != profile.joints.len()
        || candidate.links.len() != usize::from(profile.link_count)
        || candidate.joints.len() > MAX_JOINTS
        || candidate.links.len() > MAX_LINKS
        || candidate.joints != expected_joints
        || candidate.links != expected_links
    {
        return Err(GenerationError::Invalid("generated candidate is invalid"));
    }
    Ok(())
}

fn required_prohibitions() -> Vec<ProhibitedCapability> {
    vec![
        ProhibitedCapability::Filesystem,
        ProhibitedCapability::Process,
        ProhibitedCapability::Network,
        ProhibitedCapability::Clock,
        ProhibitedCapability::Randomness,
        ProhibitedCapability::Plugin,
        ProhibitedCapability::ExternalExecutable,
        ProhibitedCapability::ProtectedKernelMutation,
        ProhibitedCapability::Approval,
        ProhibitedCapability::Promotion,
    ]
}

fn required_limitations() -> Vec<String> {
    vec![
        "Structural joint-and-link candidate only; no surface or volume geometry is generated."
            .into(),
        "No skinning, inverse-bind matrices, deformation, animation quality, or physics is claimed."
            .into(),
        "No visual quality, perceptual approval, engine compatibility, or production readiness is claimed."
            .into(),
        "Generation is pure in-memory projection and grants no approval, promotion, or protected-Kernel authority."
            .into(),
    ]
}

fn strict_decode<T>(bytes: &[u8]) -> Result<T, GenerationError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let value: T =
        serde_json::from_slice(bytes).map_err(|error| GenerationError::Codec(error.to_string()))?;
    if canonical_json(&value)? != bytes {
        return Err(GenerationError::NonCanonical);
    }
    Ok(value)
}

fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, GenerationError> {
    serde_json::to_vec(value).map_err(|error| GenerationError::Codec(error.to_string()))
}

fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
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

    fn sources() -> (SemanticConstructionPackage, NeutralHumanoidProfile) {
        (
            reference_package().unwrap(),
            reference_neutral_humanoid_profile().unwrap(),
        )
    }

    #[test]
    fn typed_model_replays_deterministically_without_mutation() {
        let receipt = reference_receipt().unwrap();
        assert!(receipt.deterministic_replay);
        assert!(receipt.inputs_unchanged);
        assert!(receipt.capability_free);
        assert_eq!(receipt.joint_count, 17);
        assert_eq!(receipt.link_count, 16);
        assert_eq!(
            receipt.candidate_fingerprint,
            receipt.replay_candidate_fingerprint
        );
        assert_eq!(
            receipt.input_fingerprint,
            "5667d387e4f7a0159fee99bab584c9481cc42b549535eaaec78de3a7b5796adf"
        );
        assert_eq!(
            receipt.candidate_fingerprint,
            "4d04df0dd58cdd8ecdb7c41e9dbde2dec1910b36b7d5643b2d254ef4b3c707fa"
        );
    }

    #[test]
    fn p6_and_h2_binding_drift_fail_closed() {
        let (semantic, profile) = sources();
        let mut input = reference_input(&semantic, &profile).unwrap();
        input.semantic_package_fingerprint[0] ^= 1;
        assert!(generate(&input, &semantic, &profile).is_err());
        let mut input = reference_input(&semantic, &profile).unwrap();
        input.representation_profile_fingerprint[0] ^= 1;
        assert!(generate(&input, &semantic, &profile).is_err());
    }

    #[test]
    fn exhaustion_fails_before_partial_output() {
        let (semantic, profile) = sources();
        let mut input = reference_input(&semantic, &profile).unwrap();
        input.maximum_joints = 16;
        assert_eq!(
            generate(&input, &semantic, &profile),
            Err(GenerationError::Invalid("declared output budget exhausted"))
        );
    }

    #[test]
    fn authority_or_capability_changes_fail_closed() {
        let (semantic, profile) = sources();
        let mut input = reference_input(&semantic, &profile).unwrap();
        input.prohibited_capabilities.pop();
        assert!(generate(&input, &semantic, &profile).is_err());
    }

    #[test]
    fn strict_codecs_reject_unknown_and_noncanonical_bytes() {
        let (semantic, profile) = sources();
        let input = reference_input(&semantic, &profile).unwrap();
        let mut value: serde_json::Value =
            serde_json::from_slice(&input.to_bytes().unwrap()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("plugin_path".into(), "tool.exe".into());
        assert!(GenerationInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        let candidate = generate(&input, &semantic, &profile).unwrap();
        assert_eq!(
            StructuralCandidate::from_bytes(&serde_json::to_vec_pretty(&candidate).unwrap()),
            Err(GenerationError::NonCanonical)
        );
    }

    #[test]
    fn generated_candidate_retains_exact_h2_structure_and_negative_claims() {
        let (semantic, profile) = sources();
        let input = reference_input(&semantic, &profile).unwrap();
        let candidate = generate(&input, &semantic, &profile).unwrap();
        assert_eq!(candidate.joints.len(), profile.joints.len());
        assert_eq!(candidate.links.len(), usize::from(profile.link_count));
        assert!(
            candidate
                .limitations
                .iter()
                .any(|item| item.contains("No visual quality"))
        );
        let mut drifted = candidate.clone();
        drifted.joints[0].rest_position[0] += 1;
        assert!(validate_candidate(&drifted, &input, &profile).is_err());
    }
}
