use std::collections::{BTreeMap, BTreeSet};

use reference_intake::{minimal_h1_suite, suite_fingerprint};
use reference_viewport::{ReferenceScene, reference_scene, validate_reference_fixture_semantics};
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{RepresentationContractError, canonical_json, hash};

pub const NEUTRAL_HUMANOID_SCHEMA_VERSION: u16 = 1;
const PROFILE_DOMAIN: &[u8] = b"mindwarp.representation-contract.neutral-humanoid.v1";
const MAX_JOINTS: usize = 64;
const MAX_LINKS: usize = 96;
const MAX_FRAMES: usize = 8;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileAuthority {
    StructuralProofCandidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralClaim {
    StableJointIdentity,
    PelvisRootedHierarchy,
    RestEqualsFrameZero,
    BilateralRoleMapping,
    BoundedWireTopology,
    DeterministicSerialization,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinateConvention {
    pub handedness: String,
    pub positive_x: String,
    pub positive_y: String,
    pub positive_z: String,
    pub linear_unit: String,
    pub origin_joint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JointRoleBinding {
    pub joint_id: String,
    pub parent_joint_id: Option<String>,
    pub semantic_role: String,
    pub link_role: Option<String>,
    pub rest_position: [i32; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NeutralHumanoidProfile {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: ProfileAuthority,
    pub source_scene_id: String,
    pub source_artifact_id: String,
    pub source_scene_fingerprint: String,
    pub reference_suite_fingerprint: String,
    pub coordinate_convention: CoordinateConvention,
    pub rest_pose_frame: u16,
    pub bind_pose_rule: String,
    pub joints: Vec<JointRoleBinding>,
    pub link_count: u16,
    pub pose_frame_count: u16,
    pub claims: Vec<StructuralClaim>,
    pub limitations: Vec<String>,
}

impl NeutralHumanoidProfile {
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

    pub fn fingerprint(&self) -> Result<[u8; 32], RepresentationContractError> {
        Ok(hash(PROFILE_DOMAIN, &self.to_bytes()?))
    }
}

pub fn reference_neutral_humanoid_profile()
-> Result<NeutralHumanoidProfile, RepresentationContractError> {
    let scene = reference_scene();
    let suite = minimal_h1_suite()
        .map_err(|_| RepresentationContractError::Invalid("H1 reference suite is unavailable"))?;
    let suite_fingerprint = suite_fingerprint(&suite).map_err(|_| {
        RepresentationContractError::Invalid("H1 reference suite cannot be fingerprinted")
    })?;
    let scene_bytes = serde_json::to_vec(&scene)
        .map_err(|error| RepresentationContractError::Codec(error.to_string()))?;
    let source_scene_fingerprint = crate::hex(&sha2::Sha256::digest(scene_bytes));
    let parents = parent_map(&scene)?;
    let roles = semantic_roles();
    let joints = scene
        .vertices
        .iter()
        .map(|vertex| {
            let (parent_joint_id, link_role) = parents
                .get(vertex.id.as_str())
                .map(|(parent, role)| (Some(parent.clone()), Some(role.clone())))
                .unwrap_or((None, None));
            JointRoleBinding {
                joint_id: vertex.id.clone(),
                parent_joint_id,
                semantic_role: roles
                    .get(vertex.id.as_str())
                    .copied()
                    .unwrap_or("unmapped")
                    .into(),
                link_role,
                rest_position: [vertex.x, vertex.y, vertex.z],
            }
        })
        .collect();
    let profile = NeutralHumanoidProfile {
        schema_version: NEUTRAL_HUMANOID_SCHEMA_VERSION,
        profile_id: "forge-neutral-humanoid-structural-v1".into(),
        authority: ProfileAuthority::StructuralProofCandidate,
        source_scene_id: scene.scene_id.clone(),
        source_artifact_id: scene.artifact_id.clone(),
        source_scene_fingerprint,
        reference_suite_fingerprint: suite_fingerprint,
        coordinate_convention: CoordinateConvention {
            handedness: "right_handed".into(),
            positive_x: "anatomical_left".into(),
            positive_y: "up".into(),
            positive_z: "forward".into(),
            linear_unit: "fixture_unit_not_metric".into(),
            origin_joint: "pelvis".into(),
        },
        rest_pose_frame: 0,
        bind_pose_rule: "structural_rest_equals_frame_zero_no_inverse_bind_matrices".into(),
        joints,
        link_count: scene.edges.len() as u16,
        pose_frame_count: scene.frames.len() as u16,
        claims: required_claims(),
        limitations: required_limitations(),
    };
    validate_neutral_humanoid_profile(&profile, &scene)?;
    Ok(profile)
}

pub fn validate_neutral_humanoid_profile(
    profile: &NeutralHumanoidProfile,
    scene: &ReferenceScene,
) -> Result<(), RepresentationContractError> {
    validate_reference_fixture_semantics(scene).map_err(|_| {
        RepresentationContractError::Invalid("source scene fails v3 structural semantics")
    })?;
    let expected_suite = minimal_h1_suite()
        .map_err(|_| RepresentationContractError::Invalid("H1 reference suite is unavailable"))?;
    let expected_suite_fingerprint = suite_fingerprint(&expected_suite).map_err(|_| {
        RepresentationContractError::Invalid("H1 reference suite cannot be fingerprinted")
    })?;
    let scene_bytes = serde_json::to_vec(scene)
        .map_err(|error| RepresentationContractError::Codec(error.to_string()))?;
    let expected_scene_fingerprint = crate::hex(&sha2::Sha256::digest(scene_bytes));
    if profile.schema_version != NEUTRAL_HUMANOID_SCHEMA_VERSION
        || profile.profile_id != "forge-neutral-humanoid-structural-v1"
        || profile.authority != ProfileAuthority::StructuralProofCandidate
        || profile.source_scene_id != scene.scene_id
        || profile.source_artifact_id != scene.artifact_id
        || profile.source_scene_fingerprint != expected_scene_fingerprint
        || profile.reference_suite_fingerprint != expected_suite_fingerprint
        || profile.rest_pose_frame != 0
        || profile.bind_pose_rule != "structural_rest_equals_frame_zero_no_inverse_bind_matrices"
        || profile.coordinate_convention
            != (CoordinateConvention {
                handedness: "right_handed".into(),
                positive_x: "anatomical_left".into(),
                positive_y: "up".into(),
                positive_z: "forward".into(),
                linear_unit: "fixture_unit_not_metric".into(),
                origin_joint: "pelvis".into(),
            })
        || profile.link_count as usize != scene.edges.len()
        || profile.pose_frame_count as usize != scene.frames.len()
        || profile.joints.len() != scene.vertices.len()
        || profile.joints.len() > MAX_JOINTS
        || scene.edges.len() > MAX_LINKS
        || scene.frames.len() > MAX_FRAMES
        || profile.claims != required_claims()
        || profile.limitations != required_limitations()
    {
        return Err(RepresentationContractError::Invalid(
            "neutral humanoid profile metadata drifted or exceeded authority",
        ));
    }
    let expected_parents = parent_map(scene)?;
    let roles = semantic_roles();
    let mut ids = BTreeSet::new();
    let mut semantic = BTreeSet::new();
    let rest: BTreeMap<_, _> = scene.vertices.iter().map(|v| (v.id.as_str(), v)).collect();
    for joint in &profile.joints {
        let vertex =
            rest.get(joint.joint_id.as_str())
                .ok_or(RepresentationContractError::Invalid(
                    "profile contains unknown joint identity",
                ))?;
        let expected_parent = expected_parents.get(joint.joint_id.as_str());
        let expected_role =
            roles
                .get(joint.joint_id.as_str())
                .ok_or(RepresentationContractError::Invalid(
                    "required semantic role is missing",
                ))?;
        if !ids.insert(joint.joint_id.as_str())
            || !semantic.insert(joint.semantic_role.as_str())
            || joint.semantic_role != *expected_role
            || joint.rest_position != [vertex.x, vertex.y, vertex.z]
            || joint.parent_joint_id.as_ref() != expected_parent.map(|value| &value.0)
            || joint.link_role.as_ref() != expected_parent.map(|value| &value.1)
        {
            return Err(RepresentationContractError::Invalid(
                "joint identity, role, hierarchy, or rest pose drifted",
            ));
        }
    }
    if profile
        .joints
        .iter()
        .filter(|joint| joint.parent_joint_id.is_none())
        .count()
        != 1
        || profile
            .joints
            .iter()
            .find(|joint| joint.joint_id == "pelvis")
            .and_then(|joint| joint.parent_joint_id.as_ref())
            .is_some()
    {
        return Err(RepresentationContractError::Invalid(
            "hierarchy must have exactly one pelvis root",
        ));
    }
    Ok(())
}

fn parent_map(
    scene: &ReferenceScene,
) -> Result<BTreeMap<&str, (String, String)>, RepresentationContractError> {
    let mut parents = BTreeMap::new();
    for edge in &scene.edges {
        if parents
            .insert(edge.to.as_str(), (edge.from.clone(), edge.role.clone()))
            .is_some()
        {
            return Err(RepresentationContractError::Invalid(
                "joint hierarchy has multiple parents",
            ));
        }
    }
    for joint in &scene.vertices {
        let mut seen = BTreeSet::new();
        let mut cursor = joint.id.as_str();
        while let Some((parent, _)) = parents.get(cursor) {
            if !seen.insert(cursor) {
                return Err(RepresentationContractError::Invalid(
                    "joint hierarchy contains a cycle",
                ));
            }
            cursor = parent;
        }
    }
    Ok(parents)
}

fn semantic_roles() -> BTreeMap<&'static str, &'static str> {
    [
        ("pelvis", "root_pelvis"),
        ("chest", "torso_chest"),
        ("head", "head"),
        ("shoulder_left", "left_shoulder"),
        ("elbow_left", "left_elbow"),
        ("hand_left", "left_hand"),
        ("shoulder_right", "right_shoulder"),
        ("elbow_right", "right_elbow"),
        ("hand_right", "right_hand"),
        ("hip_left", "left_hip"),
        ("knee_left", "left_knee"),
        ("ankle_left", "left_ankle"),
        ("toe_left", "left_toe"),
        ("hip_right", "right_hip"),
        ("knee_right", "right_knee"),
        ("ankle_right", "right_ankle"),
        ("toe_right", "right_toe"),
    ]
    .into_iter()
    .collect()
}

fn required_claims() -> Vec<StructuralClaim> {
    vec![
        StructuralClaim::StableJointIdentity,
        StructuralClaim::PelvisRootedHierarchy,
        StructuralClaim::RestEqualsFrameZero,
        StructuralClaim::BilateralRoleMapping,
        StructuralClaim::BoundedWireTopology,
        StructuralClaim::DeterministicSerialization,
    ]
}

fn required_limitations() -> Vec<String> {
    vec![
        "Wire topology only; no surface, volume, or production geometry is defined.".into(),
        "No skin weights, inverse-bind matrices, deformation, or motion-quality claim is defined."
            .into(),
        "No engine compatibility, visual quality, perceptual approval, or production readiness is claimed."
            .into(),
        "Recovered legacy material remains an evidence-only adversarial input and supplies no generated content."
            .into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_profile_is_canonical_and_deterministic() {
        let first = reference_neutral_humanoid_profile().unwrap();
        let second = reference_neutral_humanoid_profile().unwrap();
        assert_eq!(first.to_bytes().unwrap(), second.to_bytes().unwrap());
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            crate::hex(&first.fingerprint().unwrap()),
            "c44adba610e2d70361d72cd9f78d1c3b7f56041a5574ef2f795570a72763d6e3"
        );
        assert_eq!(
            NeutralHumanoidProfile::from_bytes(&first.to_bytes().unwrap()).unwrap(),
            first
        );
    }

    #[test]
    fn missing_role_fails_closed() {
        let mut profile = reference_neutral_humanoid_profile().unwrap();
        profile.joints[0].semantic_role.clear();
        assert!(validate_neutral_humanoid_profile(&profile, &reference_scene()).is_err());
    }

    #[test]
    fn cycle_fails_closed() {
        let mut scene = reference_scene();
        scene.edges[0].from = "head".into();
        assert!(
            validate_neutral_humanoid_profile(
                &reference_neutral_humanoid_profile().unwrap(),
                &scene
            )
            .is_err()
        );
    }

    #[test]
    fn rest_pose_drift_fails_closed() {
        let mut profile = reference_neutral_humanoid_profile().unwrap();
        profile.joints[3].rest_position[0] += 1;
        assert!(validate_neutral_humanoid_profile(&profile, &reference_scene()).is_err());
    }

    #[test]
    fn unknown_field_and_noncanonical_bytes_fail_closed() {
        let profile = reference_neutral_humanoid_profile().unwrap();
        let mut value: serde_json::Value =
            serde_json::from_slice(&profile.to_bytes().unwrap()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("production_ready".into(), true.into());
        assert!(NeutralHumanoidProfile::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        let pretty = serde_json::to_vec_pretty(&profile).unwrap();
        assert_eq!(
            NeutralHumanoidProfile::from_bytes(&pretty),
            Err(RepresentationContractError::NonCanonical)
        );
    }

    #[test]
    fn overclaim_and_suite_drift_fail_closed() {
        let mut profile = reference_neutral_humanoid_profile().unwrap();
        profile.claims.pop();
        assert!(validate_neutral_humanoid_profile(&profile, &reference_scene()).is_err());
        let mut profile = reference_neutral_humanoid_profile().unwrap();
        profile.reference_suite_fingerprint.replace_range(..1, "0");
        assert!(validate_neutral_humanoid_profile(&profile, &reference_scene()).is_err());
    }
}
