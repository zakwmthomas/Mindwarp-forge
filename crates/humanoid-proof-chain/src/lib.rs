//! Capability-free H6 reconstruction of the exact H1-H5 humanoid proof chain.

use std::{collections::BTreeSet, fmt};

use control_calibration::reference_calibration;
use humanoid_generation::reference_receipt as h3_receipt;
use reference_intake::{minimal_h1_suite, suite_fingerprint};
use representation_contract::reference_neutral_humanoid_profile;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod promotion;
pub use promotion::*;

pub const SCHEMA_VERSION: u16 = 1;
const H5_DOMAIN: &[u8] = b"mindwarp.humanoid-proof-chain.h5-decision.v1\0";
const MANIFEST_DOMAIN: &[u8] = b"mindwarp.humanoid-proof-chain.manifest.v1\0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofChainError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
}

impl fmt::Display for ProofChainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ProofChainError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAuthority {
    EvidenceOnlyNoPromotion,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDisposition {
    ReplayVerified,
    CorruptionRejectedAndRebuilt,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H5DecisionContent {
    pub schema_version: u16,
    pub decision_id: String,
    pub visual_content_sha256: String,
    pub visual_role: String,
    pub feminine_direction: String,
    pub masculine_direction: String,
    pub neutral_construction: String,
    pub limitations: Vec<String>,
    pub authority: EvidenceAuthority,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H5DecisionReceipt {
    pub receipt_id: String,
    pub content: H5DecisionContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StageReceipt {
    pub stage_id: String,
    pub receipt_id: String,
    pub fingerprint: String,
    pub depends_on: Vec<String>,
    pub outcome: String,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofChainContent {
    pub schema_version: u16,
    pub chain_id: String,
    pub stages: Vec<StageReceipt>,
    pub authority: EvidenceAuthority,
    pub prohibited_effects: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofChainManifest {
    pub manifest_id: String,
    pub content: ProofChainContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryReceipt {
    pub candidate_sha256: String,
    pub disposition: RecoveryDisposition,
    pub recovered_manifest_id: String,
    pub recovered_bytes_sha256: String,
    pub authority: EvidenceAuthority,
}

impl H5DecisionReceipt {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ProofChainError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProofChainError> {
        let value: Self = strict_decode(bytes)?;
        validate_h5_decision(&value)?;
        Ok(value)
    }
}

impl ProofChainManifest {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ProofChainError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProofChainError> {
        let value: Self = strict_decode(bytes)?;
        validate_manifest(&value)?;
        Ok(value)
    }
}

pub fn reference_h5_decision() -> Result<H5DecisionReceipt, ProofChainError> {
    let content = H5DecisionContent {
        schema_version: SCHEMA_VERSION,
        decision_id: "h5-owner-approved-stylized-humanoid-direction-v1".into(),
        visual_content_sha256: "f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051"
            .into(),
        visual_role: "verified_stylized_feminine_target_and_featureless_multiview_presentation"
            .into(),
        feminine_direction: "cute_sweet_approachable".into(),
        masculine_direction: "strong_powerful_commanding".into(),
        neutral_construction: "bald_featureless_modular_no_genital_or_female_nipple_surface_detail"
            .into(),
        limitations: required_h5_limitations(),
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
    };
    let receipt = H5DecisionReceipt {
        receipt_id: hex(&domain_hash(H5_DOMAIN, &canonical_json(&content)?)),
        content,
    };
    validate_h5_decision(&receipt)?;
    Ok(receipt)
}

pub fn reference_manifest() -> Result<ProofChainManifest, ProofChainError> {
    let manifest = build_reference_manifest()?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn recover_reference_manifest(
    candidate_bytes: &[u8],
) -> Result<(ProofChainManifest, RecoveryReceipt), ProofChainError> {
    let candidate_sha256 = hex(&Sha256::digest(candidate_bytes));
    let (manifest, disposition) = match ProofChainManifest::from_bytes(candidate_bytes) {
        Ok(manifest) => (manifest, RecoveryDisposition::ReplayVerified),
        Err(_) => (
            reference_manifest()?,
            RecoveryDisposition::CorruptionRejectedAndRebuilt,
        ),
    };
    let recovered_bytes = manifest.to_bytes()?;
    let receipt = RecoveryReceipt {
        candidate_sha256,
        disposition,
        recovered_manifest_id: manifest.manifest_id.clone(),
        recovered_bytes_sha256: hex(&Sha256::digest(&recovered_bytes)),
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
    };
    Ok((manifest, receipt))
}

pub fn validate_h5_decision(receipt: &H5DecisionReceipt) -> Result<(), ProofChainError> {
    let expected_content = reference_h5_content();
    let expected_id = hex(&domain_hash(H5_DOMAIN, &canonical_json(&receipt.content)?));
    if receipt.content != expected_content || receipt.receipt_id != expected_id {
        return Err(ProofChainError::Invalid(
            "H5 decision drifted, widened authority, or changed identity",
        ));
    }
    Ok(())
}

pub fn validate_manifest(manifest: &ProofChainManifest) -> Result<(), ProofChainError> {
    if manifest.content.schema_version != SCHEMA_VERSION
        || manifest.content.chain_id != "h1-h5-neutral-humanoid-proof-chain-v1"
        || manifest.content.authority != EvidenceAuthority::EvidenceOnlyNoPromotion
        || manifest.content.prohibited_effects != required_prohibited_effects()
        || manifest.content.stages.len() != 5
    {
        return Err(ProofChainError::Invalid(
            "manifest metadata or authority drifted",
        ));
    }
    let expected_id = hex(&domain_hash(
        MANIFEST_DOMAIN,
        &canonical_json(&manifest.content)?,
    ));
    if manifest.manifest_id != expected_id {
        return Err(ProofChainError::Invalid("manifest identity drifted"));
    }
    let ids: Vec<_> = manifest
        .content
        .stages
        .iter()
        .map(|stage| stage.stage_id.as_str())
        .collect();
    if ids != ["H1", "H2", "H3", "H4", "H5"] {
        return Err(ProofChainError::Invalid(
            "stage order or completeness drifted",
        ));
    }
    let unique: BTreeSet<_> = manifest
        .content
        .stages
        .iter()
        .map(|stage| stage.receipt_id.as_str())
        .collect();
    if unique.len() != 5
        || manifest.content.stages[0].depends_on != Vec::<String>::new()
        || manifest.content.stages[1].depends_on != ["H1"]
        || manifest.content.stages[2].depends_on != ["H2"]
        || manifest.content.stages[3].depends_on != ["H3"]
        || manifest.content.stages[4].depends_on != ["H4"]
    {
        return Err(ProofChainError::Invalid("stage dependency linkage drifted"));
    }
    let expected = build_reference_manifest()?;
    if manifest != &expected {
        return Err(ProofChainError::Invalid("stage receipt content drifted"));
    }
    Ok(())
}

fn reference_h5_content() -> H5DecisionContent {
    H5DecisionContent {
        schema_version: SCHEMA_VERSION,
        decision_id: "h5-owner-approved-stylized-humanoid-direction-v1".into(),
        visual_content_sha256: "f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051"
            .into(),
        visual_role: "verified_stylized_feminine_target_and_featureless_multiview_presentation"
            .into(),
        feminine_direction: "cute_sweet_approachable".into(),
        masculine_direction: "strong_powerful_commanding".into(),
        neutral_construction: "bald_featureless_modular_no_genital_or_female_nipple_surface_detail"
            .into(),
        limitations: required_h5_limitations(),
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
    }
}

fn build_reference_manifest() -> Result<ProofChainManifest, ProofChainError> {
    let h1 = minimal_h1_suite().map_err(|_| ProofChainError::Invalid("H1 unavailable"))?;
    let h1_fingerprint = suite_fingerprint(&h1)
        .map_err(|_| ProofChainError::Invalid("H1 fingerprint unavailable"))?;
    let h2 = reference_neutral_humanoid_profile()
        .map_err(|_| ProofChainError::Invalid("H2 unavailable"))?;
    let h2_fingerprint = hex(&h2
        .fingerprint()
        .map_err(|_| ProofChainError::Invalid("H2 fingerprint unavailable"))?);
    let h3 = h3_receipt().map_err(|_| ProofChainError::Invalid("H3 unavailable"))?;
    let h4 = reference_calibration().map_err(|_| ProofChainError::Invalid("H4 unavailable"))?;
    let h4_fingerprint = h4
        .fingerprint()
        .map_err(|_| ProofChainError::Invalid("H4 fingerprint unavailable"))?;
    let h5 = reference_h5_decision()?;
    let content = ProofChainContent {
        schema_version: SCHEMA_VERSION,
        chain_id: "h1-h5-neutral-humanoid-proof-chain-v1".into(),
        stages: vec![
            stage(
                "H1",
                &h1.suite_id,
                &h1_fingerprint,
                &[],
                "verified_reference_intake",
                &["evidence_only_no_recovered_mesh_or_rig"],
            ),
            stage(
                "H2",
                &h2.profile_id,
                &h2_fingerprint,
                &["H1"],
                "verified_structural_profile",
                &["no_surface_skinning_deformation_or_visual_quality_claim"],
            ),
            stage(
                "H3",
                "h3-neutral-humanoid-generation-v1",
                &h3.candidate_fingerprint,
                &["H2"],
                "verified_capability_free_generation",
                &["unapproved_structural_candidate_only"],
            ),
            stage(
                "H4",
                &h4.calibration_id,
                &h4_fingerprint,
                &["H3"],
                "verified_structural_control_calibration",
                &["exact_synthetic_controls_not_quality_thresholds"],
            ),
            stage(
                "H5",
                &h5.content.decision_id,
                &h5.receipt_id,
                &["H4"],
                "verified_owner_visual_direction",
                &[
                    "preview_target_only_no_asset_import",
                    "no_topology_rig_runtime_or_device_cost_proof",
                ],
            ),
        ],
        authority: EvidenceAuthority::EvidenceOnlyNoPromotion,
        prohibited_effects: required_prohibited_effects(),
    };
    Ok(ProofChainManifest {
        manifest_id: hex(&domain_hash(MANIFEST_DOMAIN, &canonical_json(&content)?)),
        content,
    })
}

fn stage(
    stage_id: &str,
    receipt_id: &str,
    fingerprint: &str,
    depends_on: &[&str],
    outcome: &str,
    limitations: &[&str],
) -> StageReceipt {
    StageReceipt {
        stage_id: stage_id.into(),
        receipt_id: receipt_id.into(),
        fingerprint: fingerprint.into(),
        depends_on: depends_on.iter().map(|value| (*value).into()).collect(),
        outcome: outcome.into(),
        limitations: limitations.iter().map(|value| (*value).into()).collect(),
    }
}

fn required_h5_limitations() -> Vec<String> {
    [
        "visual_target_not_anatomical_truth",
        "marketplace_file_unacquired_and_uninspected",
        "no_asset_import_purchase_or_attribution_clearance",
        "no_topology_rig_deformation_animation_shader_or_device_cost_proof",
        "presentation_defaults_do_not_assign_capability_role_intelligence_morality_or_importance",
        "individual_cultural_age_body_lineage_and_player_variation_remains_open",
        "non_human_lineages_do_not_inherit_a_human_sex_binary",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn required_prohibited_effects() -> Vec<String> {
    [
        "approval",
        "promotion",
        "asset_import",
        "runtime_selection",
        "execution",
        "protected_kernel_mutation",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn strict_decode<T>(bytes: &[u8]) -> Result<T, ProofChainError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let value =
        serde_json::from_slice(bytes).map_err(|error| ProofChainError::Codec(error.to_string()))?;
    if canonical_json(&value)? != bytes {
        return Err(ProofChainError::NonCanonical);
    }
    Ok(value)
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
    fn reference_chain_rebuilds_and_replays_byte_identically() {
        let first = reference_manifest().unwrap();
        let second = reference_manifest().unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first.manifest_id,
            "a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d"
        );
        let bytes = first.to_bytes().unwrap();
        assert_eq!(ProofChainManifest::from_bytes(&bytes).unwrap(), first);
        let h5 = reference_h5_decision().unwrap();
        assert_eq!(
            h5.receipt_id,
            "5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520"
        );
        assert_eq!(
            h5.content.visual_content_sha256,
            "f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051"
        );
    }

    #[test]
    fn published_stage_fingerprints_remain_exact() {
        let manifest = reference_manifest().unwrap();
        assert_eq!(
            manifest.content.stages[0].fingerprint,
            "1a4e25e81bc39327bc95975054846496b88c4510d378c0bef5f3ea1a5281939a"
        );
        assert_eq!(
            manifest.content.stages[1].fingerprint,
            "c44adba610e2d70361d72cd9f78d1c3b7f56041a5574ef2f795570a72763d6e3"
        );
        assert_eq!(
            manifest.content.stages[2].fingerprint,
            "4d04df0dd58cdd8ecdb7c41e9dbde2dec1910b36b7d5643b2d254ef4b3c707fa"
        );
        assert_eq!(
            manifest.content.stages[3].fingerprint,
            "774a790aa963bb7ed329394d869fda4f5530697cce4d4d029a23d31e6d575f4d"
        );
    }

    #[test]
    fn hostile_manifest_mutations_fail_closed() {
        let good = reference_manifest().unwrap().to_bytes().unwrap();
        let original = good.clone();
        let base: serde_json::Value = serde_json::from_slice(&good).unwrap();
        let mut cases = Vec::new();
        for pointer in [
            "/content/schema_version",
            "/content/chain_id",
            "/content/stages/1/fingerprint",
            "/content/stages/4/depends_on/0",
            "/content/stages/4/limitations/0",
            "/manifest_id",
        ] {
            let mut value = base.clone();
            *value.pointer_mut(pointer).unwrap() = serde_json::json!("corrupt");
            cases.push(serde_json::to_vec(&value).unwrap());
        }
        let mut reordered = base.clone();
        reordered["content"]["stages"]
            .as_array_mut()
            .unwrap()
            .swap(3, 4);
        cases.push(serde_json::to_vec(&reordered).unwrap());
        let mut unknown = base;
        unknown
            .as_object_mut()
            .unwrap()
            .insert("authority_grant".into(), serde_json::json!(true));
        cases.push(serde_json::to_vec(&unknown).unwrap());
        cases.push(good[..good.len() / 2].to_vec());
        for bytes in cases {
            assert!(ProofChainManifest::from_bytes(&bytes).is_err());
            assert_eq!(good, original);
        }
    }

    #[test]
    fn corruption_recovery_rebuilds_known_good_without_authority() {
        let good = reference_manifest().unwrap();
        let good_bytes = good.to_bytes().unwrap();
        let (replayed, replay_receipt) = recover_reference_manifest(&good_bytes).unwrap();
        assert_eq!(replayed, good);
        assert_eq!(
            replay_receipt.disposition,
            RecoveryDisposition::ReplayVerified
        );

        let corrupt = &good_bytes[..good_bytes.len() / 2];
        let (recovered, receipt) = recover_reference_manifest(corrupt).unwrap();
        assert_eq!(recovered, good);
        assert_eq!(
            receipt.disposition,
            RecoveryDisposition::CorruptionRejectedAndRebuilt
        );
        assert_eq!(
            receipt.authority,
            EvidenceAuthority::EvidenceOnlyNoPromotion
        );
        assert_eq!(recovered.to_bytes().unwrap(), good_bytes);
    }

    #[test]
    fn h5_decision_is_strict_and_authority_negative() {
        let receipt = reference_h5_decision().unwrap();
        let bytes = receipt.to_bytes().unwrap();
        assert_eq!(H5DecisionReceipt::from_bytes(&bytes).unwrap(), receipt);
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["content"]["limitations"] = serde_json::json!([]);
        assert!(H5DecisionReceipt::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        assert_eq!(
            receipt.content.authority,
            EvidenceAuthority::EvidenceOnlyNoPromotion
        );
    }
}
