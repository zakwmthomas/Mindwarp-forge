use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

use forge_kernel::{
    ActorKind, AuthorityBasis, CandidateState, EventType, KernelError, persistence::PersistentForge,
};
use humanoid_proof_chain::{PromotionPackage, reference_promotion_package};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn exact_h7_package_proposes_only_and_survives_reopen_and_backup() {
    let suffix = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "mindwarp-h7-candidate-package-{}-{suffix}",
        std::process::id()
    ));
    fs::create_dir(&directory).unwrap();
    let database = directory.join("forge.sqlite3");
    let backup = directory.join("backup.sqlite3");

    let package = reference_promotion_package().unwrap();
    let package_bytes = package.to_bytes().unwrap();
    let mut forge = PersistentForge::open(&database).unwrap();
    let evidence_id = forge
        .kernel_mut()
        .register_evidence(
            ActorKind::Assistant,
            &package_bytes,
            "h7:disposable-candidate-proof",
        )
        .unwrap();
    let candidate_id = forge
        .kernel_mut()
        .propose_candidate(&evidence_id, "h7:disposable-candidate-proof")
        .unwrap();

    assert_eq!(package_bytes.len(), 1512);
    assert_eq!(
        evidence_id,
        "f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c"
    );
    assert_eq!(
        candidate_id,
        "c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433"
    );
    assert_eq!(
        forge.kernel().candidate(&candidate_id).unwrap().state,
        CandidateState::Proposed
    );
    assert_eq!(forge.kernel().object_count(), 1);
    assert_eq!(forge.kernel().candidate_count(), 1);
    assert_eq!(forge.kernel().events().len(), 2);
    assert_eq!(
        forge
            .kernel()
            .events()
            .iter()
            .map(|event| event.event_type.clone())
            .collect::<Vec<_>>(),
        vec![EventType::EvidenceRegistered, EventType::CandidateProposed]
    );
    assert!(forge.kernel().events().iter().all(|event| {
        event.authority == AuthorityBasis::None
            && !matches!(
                event.event_type,
                EventType::CandidateApproved
                    | EventType::CandidatePromoted
                    | EventType::CandidateSuperseded
                    | EventType::CodeApplied
            )
    }));
    assert_eq!(
        forge.kernel().object(&evidence_id).unwrap().bytes,
        package_bytes
    );
    assert_eq!(
        PromotionPackage::from_bytes(&forge.kernel().object(&evidence_id).unwrap().bytes).unwrap(),
        package
    );

    assert_eq!(
        forge.kernel_mut().approve_candidate(
            ActorKind::Assistant,
            AuthorityBasis::ExplicitUserAuthorization,
            &candidate_id,
            "h7:forged-approval",
        ),
        Err(KernelError::AuthorityDenied)
    );
    assert_eq!(forge.kernel().events().len(), 2);
    forge.commit().unwrap();
    let backup_receipt = forge.backup_to(&backup).unwrap();
    PersistentForge::verify_backup(&backup_receipt).unwrap();
    drop(forge);

    for path in [&database, &backup] {
        let reopened = PersistentForge::open(path).unwrap();
        assert_eq!(reopened.kernel().events().len(), 2);
        assert_eq!(
            reopened.kernel().candidate(&candidate_id).unwrap().state,
            CandidateState::Proposed
        );
        assert_eq!(
            PromotionPackage::from_bytes(&reopened.kernel().object(&evidence_id).unwrap().bytes)
                .unwrap(),
            package
        );
        drop(reopened);
    }

    fs::remove_file(backup).unwrap();
    fs::remove_file(database).unwrap();
    fs::remove_dir(directory).unwrap();
}
