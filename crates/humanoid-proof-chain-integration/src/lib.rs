//! Integration boundary between the capability-free humanoid proof chain and
//! the protected Forge Kernel.
//!
//! The validator in this crate is read-only. It lets a future G1 consumer bind
//! the exact promoted H7 proof baseline without treating a generic promoted
//! candidate as equivalent, and without widening H7 into an asset or runtime
//! claim.

use forge_kernel::{ActorKind, AuthorityBasis, CandidateState, EventType, ForgeKernel};
use humanoid_proof_chain::{PromotionPackage, reference_promotion_package};

pub const H7_EVIDENCE_ID: &str = "f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c";
pub const H7_CANDIDATE_ID: &str =
    "c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433";
pub const H7_CANDIDATE_TYPE: &str = "engine-neutral-humanoid-proof-baseline-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct H7DependencyReceipt {
    pub candidate_id: String,
    pub candidate_type: String,
    pub evidence_id: String,
    pub package_id: String,
    pub claims: Vec<String>,
    pub retained_blockers: Vec<String>,
    pub lifecycle_state: CandidateState,
    pub superseded: bool,
    pub authority_effect: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum H7DependencyError {
    MissingExactCandidate,
    WrongLifecycleState(CandidateState),
    EvidenceBindingDrift,
    PackageDrift,
    MissingExactPromotionEvent,
}

/// Verify the exact H7 dependency package and lifecycle without mutating the
/// Kernel. The package's mandatory non-claims are returned as retained
/// blockers so downstream consumers cannot silently discard them.
pub fn verify_h7_dependency(
    kernel: &ForgeKernel,
) -> Result<H7DependencyReceipt, H7DependencyError> {
    let candidate = kernel
        .candidate(H7_CANDIDATE_ID)
        .ok_or(H7DependencyError::MissingExactCandidate)?;
    if candidate.state != CandidateState::Promoted {
        return Err(H7DependencyError::WrongLifecycleState(
            candidate.state.clone(),
        ));
    }
    if candidate.evidence != H7_EVIDENCE_ID {
        return Err(H7DependencyError::EvidenceBindingDrift);
    }

    let stored = kernel
        .object(H7_EVIDENCE_ID)
        .ok_or(H7DependencyError::EvidenceBindingDrift)?;
    let package =
        PromotionPackage::from_bytes(&stored.bytes).map_err(|_| H7DependencyError::PackageDrift)?;
    let expected = reference_promotion_package().map_err(|_| H7DependencyError::PackageDrift)?;
    if package != expected || package.content.candidate_name != H7_CANDIDATE_TYPE {
        return Err(H7DependencyError::PackageDrift);
    }

    let exact_promotion = candidate.evidence_events.iter().any(|event_id| {
        kernel.events().iter().any(|event| {
            event.id == *event_id
                && event.event_type == EventType::CandidatePromoted
                && event.actor == ActorKind::DirectProjectUser
                && event.authority == AuthorityBasis::ExplicitUserAuthorization
                && event.input_objects == vec![H7_EVIDENCE_ID.to_owned()]
                && event.payload == serde_json::Value::String(H7_CANDIDATE_ID.to_owned())
        })
    });
    if !exact_promotion {
        return Err(H7DependencyError::MissingExactPromotionEvent);
    }

    Ok(H7DependencyReceipt {
        candidate_id: H7_CANDIDATE_ID.into(),
        candidate_type: H7_CANDIDATE_TYPE.into(),
        evidence_id: H7_EVIDENCE_ID.into(),
        package_id: package.package_id,
        claims: package.content.claims,
        retained_blockers: package.content.non_claims,
        lifecycle_state: CandidateState::Promoted,
        superseded: false,
        authority_effect: "read_only_dependency_verification",
    })
}
