use std::path::Path;

use forge_kernel::{
    ActorKind, AuthorityBasis, CandidateState, EventType, persistence::PersistentForge,
};
use humanoid_proof_chain::reference_promotion_package;

const EVIDENCE_ID: &str = "f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c";
const CANDIDATE_ID: &str = "c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433";

#[derive(Debug, Eq, PartialEq)]
struct PromotionReceipt {
    candidate_id: String,
    state: CandidateState,
    already_promoted: bool,
    events_added: usize,
    promotion_event_id: Option<String>,
    pre_promotion_backup: Option<String>,
    pre_promotion_backup_sha256: Option<String>,
}

fn promote(database: &Path, backup: &Path) -> Result<PromotionReceipt, String> {
    let package = reference_promotion_package().map_err(|error| format!("{error:?}"))?;
    let package_bytes = package.to_bytes().map_err(|error| format!("{error:?}"))?;
    if package_bytes.len() != 1512 {
        return Err("canonical H7 package length drifted".into());
    }

    let mut forge = PersistentForge::open(database).map_err(|error| format!("{error:?}"))?;
    let candidate = forge
        .kernel()
        .candidate(CANDIDATE_ID)
        .ok_or_else(|| "exact H7 candidate is absent".to_owned())?;
    if candidate.evidence != EVIDENCE_ID
        || forge
            .kernel()
            .object(EVIDENCE_ID)
            .is_none_or(|object| object.bytes != package_bytes)
    {
        return Err("H7 candidate identity or canonical evidence drifted".into());
    }
    if candidate.state == CandidateState::Promoted {
        return Ok(PromotionReceipt {
            candidate_id: CANDIDATE_ID.into(),
            state: CandidateState::Promoted,
            already_promoted: true,
            events_added: 0,
            promotion_event_id: candidate.evidence_events.last().cloned(),
            pre_promotion_backup: None,
            pre_promotion_backup_sha256: None,
        });
    }
    if candidate.state != CandidateState::Approved {
        return Err(format!(
            "exact H7 candidate must be Approved, not {:?}",
            candidate.state
        ));
    }

    let backup_receipt = forge
        .backup_to(backup)
        .map_err(|error| format!("pre-promotion backup failed: {error:?}"))?;
    PersistentForge::verify_backup(&backup_receipt)
        .map_err(|error| format!("pre-promotion backup verification failed: {error:?}"))?;
    let events_before = forge.kernel().events().len();
    let promotion_event_id = forge
        .kernel_mut()
        .promote_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            CANDIDATE_ID,
            "chat:explicit-owner-promotion:h7-proof-baseline",
        )
        .map_err(|error| format!("{error:?}"))?;
    let added = &forge.kernel().events()[events_before..];
    if added.len() != 1
        || added[0].event_type != EventType::CandidatePromoted
        || added[0].id != promotion_event_id
        || added[0].actor != ActorKind::DirectProjectUser
        || added[0].authority != AuthorityBasis::ExplicitUserAuthorization
        || added[0].input_objects != vec![EVIDENCE_ID.to_owned()]
        || added[0].payload != serde_json::Value::String(CANDIDATE_ID.into())
    {
        return Err("H7 promotion attempted an unexpected ledger effect".into());
    }
    forge.commit().map_err(|error| format!("{error:?}"))?;
    drop(forge);

    let reopened = PersistentForge::open(database).map_err(|error| format!("{error:?}"))?;
    if reopened
        .kernel()
        .candidate(CANDIDATE_ID)
        .is_none_or(|candidate| candidate.state != CandidateState::Promoted)
        || reopened
            .kernel()
            .events()
            .iter()
            .any(|event| event.event_type == EventType::CodeApplied)
    {
        return Err("H7 promotion did not survive reopen without application effects".into());
    }
    drop(reopened);

    Ok(PromotionReceipt {
        candidate_id: CANDIDATE_ID.into(),
        state: CandidateState::Promoted,
        already_promoted: false,
        events_added: 1,
        promotion_event_id: Some(promotion_event_id),
        pre_promotion_backup: Some(backup_receipt.path.display().to_string()),
        pre_promotion_backup_sha256: Some(backup_receipt.sha256),
    })
}

fn main() {
    let database = std::env::args_os()
        .nth(1)
        .expect("usage: h7_candidate_promotion <forge.sqlite3> <pre-promotion-backup.sqlite3>");
    let backup = std::env::args_os()
        .nth(2)
        .expect("usage: h7_candidate_promotion <forge.sqlite3> <pre-promotion-backup.sqlite3>");
    let receipt = promote(Path::new(&database), Path::new(&backup))
        .expect("H7 candidate promotion failed closed");
    println!("kernel_candidate_id={}", receipt.candidate_id);
    println!("kernel_candidate_state=promoted");
    println!("already_promoted={}", receipt.already_promoted);
    println!("events_added={}", receipt.events_added);
    println!(
        "promotion_event_id={}",
        receipt.promotion_event_id.as_deref().unwrap_or("none")
    );
    println!(
        "pre_promotion_backup={}",
        receipt.pre_promotion_backup.as_deref().unwrap_or("none")
    );
    println!(
        "pre_promotion_backup_sha256={}",
        receipt
            .pre_promotion_backup_sha256
            .as_deref()
            .unwrap_or("none")
    );
    println!("authority=direct_project_user_explicit_promotion");
    println!("application_effect=false");
    println!("runtime_effect=false");
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    fn candidate_database(name: &str, approve: bool) -> (std::path::PathBuf, std::path::PathBuf) {
        let suffix = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "mindwarp-h7-promotion-{name}-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        let database = directory.join("forge.sqlite3");
        let package = reference_promotion_package().unwrap();
        let package_bytes = package.to_bytes().unwrap();
        let mut forge = PersistentForge::open(&database).unwrap();
        let evidence = forge
            .kernel_mut()
            .register_evidence(ActorKind::Assistant, package_bytes, "test:h7-proposal")
            .unwrap();
        assert_eq!(evidence, EVIDENCE_ID);
        let candidate = forge
            .kernel_mut()
            .propose_candidate(&evidence, "test:h7-proposal")
            .unwrap();
        assert_eq!(candidate, CANDIDATE_ID);
        if approve {
            forge
                .kernel_mut()
                .approve_candidate(
                    ActorKind::DirectProjectUser,
                    AuthorityBasis::ExplicitUserAuthorization,
                    CANDIDATE_ID,
                    "test:h7-approval",
                )
                .unwrap();
        }
        forge.commit().unwrap();
        drop(forge);
        (directory, database)
    }

    #[test]
    fn exact_promotion_is_one_event_backup_first_durable_and_idempotent() {
        let (directory, database) = candidate_database("idempotent", true);
        let backup = directory.join("pre-promotion.sqlite3");
        let first = promote(&database, &backup).unwrap();
        assert!(!first.already_promoted);
        assert_eq!(first.events_added, 1);
        assert_eq!(first.state, CandidateState::Promoted);
        assert_eq!(first.pre_promotion_backup.as_deref(), backup.to_str());
        assert_eq!(
            first.pre_promotion_backup_sha256.as_ref().unwrap().len(),
            64
        );

        let before = PersistentForge::open(&backup).unwrap();
        assert_eq!(
            before.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Approved
        );
        assert_eq!(before.kernel().events().len(), 3);
        drop(before);

        let second = promote(&database, &backup).unwrap();
        assert!(second.already_promoted);
        assert_eq!(second.events_added, 0);
        assert!(second.pre_promotion_backup.is_none());

        let workspace = directory.join("must-not-be-created");
        let mut after = PersistentForge::open(&database).unwrap();
        assert!(after.apply_promoted_code(CANDIDATE_ID, &workspace).is_err());
        assert!(!workspace.exists());
        assert_eq!(after.kernel().events().len(), 4);
        assert_eq!(
            after.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Promoted
        );
        drop(after);

        fs::remove_file(backup).unwrap();
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn promotion_fails_closed_before_approval_without_creating_a_backup() {
        let (directory, database) = candidate_database("proposed", false);
        let backup = directory.join("must-not-exist.sqlite3");
        assert!(promote(&database, &backup).is_err());
        assert!(!backup.exists());
        let reopened = PersistentForge::open(&database).unwrap();
        assert_eq!(reopened.kernel().events().len(), 2);
        assert_eq!(
            reopened.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Proposed
        );
        drop(reopened);
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }
}
