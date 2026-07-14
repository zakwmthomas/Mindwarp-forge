//! Capability-free provenance and authority validation for reference inputs.

use std::collections::BTreeSet;

use reference_viewport::reference_snapshot;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SCHEMA_VERSION: u8 = 1;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    RecoveredLegacyReport,
    ForgeOwnedSynthetic,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentClass {
    DeclaredBlueprint,
    TypedSyntheticScene,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    EvidenceOnly,
    ForgeOwnedTestFixture,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermittedUse {
    StructuralObservationCandidate,
    AdversarialTestTarget,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ForbiddenUse {
    CanonicalBaseline,
    ProductionImport,
    RuntimeExecution,
    PerceptualApproval,
    NumericAcceptanceThreshold,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimBasis {
    DeclaredLegacyClaim,
    DeterministicallyVerified,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceClaim {
    pub statement: String,
    pub basis: ClaimBasis,
    pub limitation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceTarget {
    pub schema_version: u8,
    pub target_id: String,
    pub source_locator: String,
    pub content_sha256: String,
    pub byte_length: u64,
    pub provenance: ProvenanceClass,
    pub content_class: ContentClass,
    pub authority: AuthorityClass,
    pub executable_content: bool,
    pub permitted_uses: Vec<PermittedUse>,
    pub forbidden_uses: Vec<ForbiddenUse>,
    pub claims: Vec<ReferenceClaim>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceSuite {
    pub schema_version: u8,
    pub suite_id: String,
    pub targets: Vec<ReferenceTarget>,
    pub selection_rule: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntakeError {
    Invalid(&'static str),
    Codec(String),
    Viewport(String),
}

pub fn recovered_humanoid_blueprint_target() -> ReferenceTarget {
    ReferenceTarget {
        schema_version: SCHEMA_VERSION,
        target_id: "recovered-humanoid-blueprint-legacy-v2".into(),
        source_locator: "forge documents from gpt handover/MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK_2026-07-12.zip!MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK/07_LEGACY_REPORTS/one_button_humanoid_blueprint.json".into(),
        content_sha256: "74b23331be5291bf399cd4d4b364059de7ab4d305569e19e7090470f73502491".into(),
        byte_length: 5_430,
        provenance: ProvenanceClass::RecoveredLegacyReport,
        content_class: ContentClass::DeclaredBlueprint,
        authority: AuthorityClass::EvidenceOnly,
        executable_content: false,
        permitted_uses: vec![
            PermittedUse::StructuralObservationCandidate,
            PermittedUse::AdversarialTestTarget,
        ],
        forbidden_uses: required_forbidden_uses(),
        claims: vec![ReferenceClaim {
            statement: "The legacy report declares a bilateral articulated biped hierarchy with named support and manipulation interfaces.".into(),
            basis: ClaimBasis::DeclaredLegacyClaim,
            limitation: "No mesh, skin weights, inverse-bind matrices, deformation evidence, licensed visual target, or independently rerun generator is present.".into(),
        }],
        limitations: vec![
            "The category-first blueprint is an old report, not a reference asset or Forge architecture.".into(),
            "Its quality scores, timings, handoff readiness, and automatic-success fields are unverified self-reports.".into(),
        ],
    }
}

pub fn synthetic_v3_target() -> Result<ReferenceTarget, IntakeError> {
    let snapshot =
        reference_snapshot().map_err(|error| IntakeError::Viewport(error.to_string()))?;
    let bytes =
        serde_json::to_vec(&snapshot).map_err(|error| IntakeError::Codec(error.to_string()))?;
    Ok(ReferenceTarget {
        schema_version: SCHEMA_VERSION,
        target_id: "forge-neutral-t-pose-v3".into(),
        source_locator: "forge://reference-viewport/artifact-reference-viewport-003".into(),
        content_sha256: snapshot.scene_fingerprint,
        byte_length: bytes.len() as u64,
        provenance: ProvenanceClass::ForgeOwnedSynthetic,
        content_class: ContentClass::TypedSyntheticScene,
        authority: AuthorityClass::ForgeOwnedTestFixture,
        executable_content: false,
        permitted_uses: vec![
            PermittedUse::StructuralObservationCandidate,
            PermittedUse::AdversarialTestTarget,
        ],
        forbidden_uses: required_forbidden_uses(),
        claims: vec![ReferenceClaim {
            statement: "The synthetic scene deterministically preserves its declared joint hierarchy, rest pose, and segment lengths across two frames.".into(),
            basis: ClaimBasis::DeterministicallyVerified,
            limitation: "Wireframe structure does not establish anatomy quality, surface form, skinning, deformation, motion quality, or perceptual approval.".into(),
        }],
        limitations: vec![
            "Forge-owned structural fixture only; it is not an approved humanoid or production baseline.".into(),
        ],
    })
}

pub fn minimal_h1_suite() -> Result<ReferenceSuite, IntakeError> {
    let suite = ReferenceSuite {
        schema_version: SCHEMA_VERSION,
        suite_id: "h1-neutral-humanoid-reference-intake-v1".into(),
        targets: vec![recovered_humanoid_blueprint_target(), synthetic_v3_target()?],
        selection_rule: "Retain one recovered declared-structure challenge and one Forge-owned deterministic structural fixture; neither is a canonical baseline, numeric threshold, or perceptual approval.".into(),
    };
    validate_suite(&suite)?;
    Ok(suite)
}

pub fn validate_target(target: &ReferenceTarget) -> Result<(), IntakeError> {
    if target.schema_version != SCHEMA_VERSION
        || !safe_id(&target.target_id)
        || !safe_locator(&target.source_locator)
        || target.content_sha256.len() != 64
        || !target
            .content_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        || target.byte_length == 0
        || target.executable_content
        || target.permitted_uses.is_empty()
        || target.claims.is_empty()
        || target.limitations.is_empty()
        || target
            .claims
            .iter()
            .any(|claim| claim.statement.trim().is_empty() || claim.limitation.trim().is_empty())
        || target.limitations.iter().any(|item| item.trim().is_empty())
    {
        return Err(IntakeError::Invalid("invalid reference target"));
    }
    let forbidden: BTreeSet<_> = target.forbidden_uses.iter().collect();
    if required_forbidden_uses()
        .iter()
        .any(|required| !forbidden.contains(required))
    {
        return Err(IntakeError::Invalid(
            "reference target lacks a mandatory forbidden use",
        ));
    }
    match (&target.provenance, &target.content_class, &target.authority) {
        (
            ProvenanceClass::RecoveredLegacyReport,
            ContentClass::DeclaredBlueprint,
            AuthorityClass::EvidenceOnly,
        ) if target
            .claims
            .iter()
            .all(|claim| claim.basis == ClaimBasis::DeclaredLegacyClaim) => {}
        (
            ProvenanceClass::ForgeOwnedSynthetic,
            ContentClass::TypedSyntheticScene,
            AuthorityClass::ForgeOwnedTestFixture,
        ) if target
            .claims
            .iter()
            .any(|claim| claim.basis == ClaimBasis::DeterministicallyVerified) => {}
        _ => {
            return Err(IntakeError::Invalid(
                "reference provenance exceeds its authority",
            ));
        }
    }
    Ok(())
}

pub fn validate_suite(suite: &ReferenceSuite) -> Result<(), IntakeError> {
    if suite.schema_version != SCHEMA_VERSION
        || !safe_id(&suite.suite_id)
        || suite.selection_rule.trim().is_empty()
        || suite.targets.len() < 2
    {
        return Err(IntakeError::Invalid("invalid reference suite"));
    }
    let mut ids = BTreeSet::new();
    let mut hashes = BTreeSet::new();
    for target in &suite.targets {
        validate_target(target)?;
        if !ids.insert(&target.target_id) || !hashes.insert(&target.content_sha256) {
            return Err(IntakeError::Invalid("duplicate reference evidence"));
        }
    }
    if !suite
        .targets
        .iter()
        .any(|target| target.provenance == ProvenanceClass::RecoveredLegacyReport)
        || !suite
            .targets
            .iter()
            .any(|target| target.provenance == ProvenanceClass::ForgeOwnedSynthetic)
    {
        return Err(IntakeError::Invalid(
            "reference suite lacks provenance diversity",
        ));
    }
    Ok(())
}

pub fn canonical_suite_bytes(suite: &ReferenceSuite) -> Result<Vec<u8>, IntakeError> {
    validate_suite(suite)?;
    serde_json::to_vec(suite).map_err(|error| IntakeError::Codec(error.to_string()))
}

pub fn suite_fingerprint(suite: &ReferenceSuite) -> Result<String, IntakeError> {
    Ok(hex(&Sha256::digest(canonical_suite_bytes(suite)?)))
}

fn required_forbidden_uses() -> Vec<ForbiddenUse> {
    vec![
        ForbiddenUse::CanonicalBaseline,
        ForbiddenUse::ProductionImport,
        ForbiddenUse::RuntimeExecution,
        ForbiddenUse::PerceptualApproval,
        ForbiddenUse::NumericAcceptanceThreshold,
    ]
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn safe_locator(value: &str) -> bool {
    if value.is_empty()
        || value.len() > 512
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('<')
        || value.contains('>')
        || value.contains('\0')
    {
        return false;
    }
    let bytes = value.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return false;
    }
    if value.contains("://") && !value.starts_with("forge://") {
        return false;
    }
    !value
        .split(['/', '\\', '!'])
        .any(|component| component == "." || component == "..")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_suite_is_deterministic_diverse_and_authority_negative() {
        let first = minimal_h1_suite().unwrap();
        let second = minimal_h1_suite().unwrap();
        assert_eq!(first, second);
        assert_eq!(
            suite_fingerprint(&first).unwrap(),
            suite_fingerprint(&second).unwrap()
        );
        assert_eq!(first.targets.len(), 2);
        assert!(
            first
                .targets
                .iter()
                .all(|target| !target.executable_content)
        );
        assert!(first.targets.iter().all(|target| {
            target
                .forbidden_uses
                .contains(&ForbiddenUse::CanonicalBaseline)
                && target
                    .forbidden_uses
                    .contains(&ForbiddenUse::ProductionImport)
                && target
                    .forbidden_uses
                    .contains(&ForbiddenUse::PerceptualApproval)
                && target
                    .forbidden_uses
                    .contains(&ForbiddenUse::NumericAcceptanceThreshold)
        }));
    }

    #[test]
    fn recovered_self_report_cannot_masquerade_as_verified_or_authoritative() {
        let mut target = recovered_humanoid_blueprint_target();
        target.claims[0].basis = ClaimBasis::DeterministicallyVerified;
        assert_eq!(
            validate_target(&target),
            Err(IntakeError::Invalid(
                "reference provenance exceeds its authority"
            ))
        );
        let mut target = recovered_humanoid_blueprint_target();
        target.authority = AuthorityClass::ForgeOwnedTestFixture;
        assert!(validate_target(&target).is_err());
    }

    #[test]
    fn duplicate_archive_copy_does_not_create_independent_evidence() {
        let target = recovered_humanoid_blueprint_target();
        let mut duplicate = target.clone();
        duplicate.target_id = "emergency-pack-duplicate".into();
        duplicate.source_locator = "forge documents from gpt handover/MINDWARP_FORGE_ANDROID_BOOTSTRAP_EMERGENCY_PACK_2026-07-12.zip!MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK/07_LEGACY_REPORTS/one_button_humanoid_blueprint.json".into();
        let suite = ReferenceSuite {
            schema_version: SCHEMA_VERSION,
            suite_id: "duplicate-suite".into(),
            targets: vec![target, duplicate],
            selection_rule: "Duplicates are not independent.".into(),
        };
        assert_eq!(
            validate_suite(&suite),
            Err(IntakeError::Invalid("duplicate reference evidence"))
        );
    }

    #[test]
    fn executable_unsafe_and_ambiguous_targets_fail_closed() {
        let mut target = recovered_humanoid_blueprint_target();
        target.executable_content = true;
        assert!(validate_target(&target).is_err());
        for locator in [
            "/absolute.json",
            "C:/absolute.json",
            "C:\\absolute.json",
            "../escape.json",
            "https://example.invalid/reference.json",
            "<script>",
        ] {
            let mut target = recovered_humanoid_blueprint_target();
            target.source_locator = locator.into();
            assert!(validate_target(&target).is_err(), "accepted {locator}");
        }
    }

    #[test]
    fn missing_forbidden_use_and_single_provenance_suite_fail_closed() {
        let mut target = recovered_humanoid_blueprint_target();
        target
            .forbidden_uses
            .retain(|item| *item != ForbiddenUse::CanonicalBaseline);
        assert!(validate_target(&target).is_err());
        let target = recovered_humanoid_blueprint_target();
        let mut second = target.clone();
        second.target_id = "different-legacy-report".into();
        second.content_sha256 = "11".repeat(32);
        let suite = ReferenceSuite {
            schema_version: SCHEMA_VERSION,
            suite_id: "legacy-only-suite".into(),
            targets: vec![target, second],
            selection_rule: "Invalid single-provenance suite.".into(),
        };
        assert_eq!(
            validate_suite(&suite),
            Err(IntakeError::Invalid(
                "reference suite lacks provenance diversity"
            ))
        );
    }

    #[test]
    fn unknown_fields_and_version_drift_are_rejected() {
        let suite = minimal_h1_suite().unwrap();
        let mut value = serde_json::to_value(&suite).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("weight".into(), serde_json::json!(0.95));
        assert!(serde_json::from_value::<ReferenceSuite>(value).is_err());
        let mut suite = suite;
        suite.schema_version += 1;
        assert!(validate_suite(&suite).is_err());
    }
}
