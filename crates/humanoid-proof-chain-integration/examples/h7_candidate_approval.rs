use std::path::Path;

use forge_kernel::{
    ActorKind, AuthorityBasis, CandidateState, EventType, persistence::PersistentForge,
};
use humanoid_proof_chain::reference_promotion_package;

const EVIDENCE_ID: &str = "f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c";
const CANDIDATE_ID: &str = "c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433";

#[derive(Debug, Eq, PartialEq)]
struct ApprovalReceipt {
    candidate_id: String,
    state: CandidateState,
    already_approved: bool,
    events_added: usize,
    approval_event_id: Option<String>,
    pre_approval_backup: Option<String>,
    pre_approval_backup_sha256: Option<String>,
}

fn approve(database: &Path, backup: &Path) -> Result<ApprovalReceipt, String> {
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
    if candidate.state == CandidateState::Approved {
        return Ok(ApprovalReceipt {
            candidate_id: CANDIDATE_ID.into(),
            state: CandidateState::Approved,
            already_approved: true,
            events_added: 0,
            approval_event_id: candidate.evidence_events.last().cloned(),
            pre_approval_backup: None,
            pre_approval_backup_sha256: None,
        });
    }
    if candidate.state != CandidateState::Proposed {
        return Err(format!(
            "exact H7 candidate must be Proposed, not {:?}",
            candidate.state
        ));
    }

    let backup_receipt = forge
        .backup_to(backup)
        .map_err(|error| format!("pre-approval backup failed: {error:?}"))?;
    PersistentForge::verify_backup(&backup_receipt)
        .map_err(|error| format!("pre-approval backup verification failed: {error:?}"))?;
    let events_before = forge.kernel().events().len();
    let approval_event_id = forge
        .kernel_mut()
        .approve_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            CANDIDATE_ID,
            "chat:explicit-owner-approval:h7-proof-baseline",
        )
        .map_err(|error| format!("{error:?}"))?;
    let added = &forge.kernel().events()[events_before..];
    if added.len() != 1
        || added[0].event_type != EventType::CandidateApproved
        || added[0].id != approval_event_id
        || added[0].actor != ActorKind::DirectProjectUser
        || added[0].authority != AuthorityBasis::ExplicitUserAuthorization
        || added[0].input_objects != vec![EVIDENCE_ID.to_owned()]
    {
        return Err("H7 approval attempted an unexpected ledger effect".into());
    }
    forge.commit().map_err(|error| format!("{error:?}"))?;
    drop(forge);

    let reopened = PersistentForge::open(database).map_err(|error| format!("{error:?}"))?;
    if reopened
        .kernel()
        .candidate(CANDIDATE_ID)
        .is_none_or(|candidate| candidate.state != CandidateState::Approved)
    {
        return Err("H7 approval did not survive deterministic reopen".into());
    }
    drop(reopened);
    Ok(ApprovalReceipt {
        candidate_id: CANDIDATE_ID.into(),
        state: CandidateState::Approved,
        already_approved: false,
        events_added: 1,
        approval_event_id: Some(approval_event_id),
        pre_approval_backup: Some(backup_receipt.path.display().to_string()),
        pre_approval_backup_sha256: Some(backup_receipt.sha256),
    })
}

fn main() {
    let database = std::env::args_os()
        .nth(1)
        .expect("usage: h7_candidate_approval <forge.sqlite3> <pre-approval-backup.sqlite3>");
    let backup = std::env::args_os()
        .nth(2)
        .expect("usage: h7_candidate_approval <forge.sqlite3> <pre-approval-backup.sqlite3>");
    let receipt = approve(Path::new(&database), Path::new(&backup))
        .expect("H7 candidate approval failed closed");
    println!("kernel_candidate_id={}", receipt.candidate_id);
    println!("kernel_candidate_state=approved");
    println!("already_approved={}", receipt.already_approved);
    println!("events_added={}", receipt.events_added);
    println!(
        "approval_event_id={}",
        receipt.approval_event_id.as_deref().unwrap_or("none")
    );
    println!(
        "pre_approval_backup={}",
        receipt.pre_approval_backup.as_deref().unwrap_or("none")
    );
    println!(
        "pre_approval_backup_sha256={}",
        receipt
            .pre_approval_backup_sha256
            .as_deref()
            .unwrap_or("none")
    );
    println!("authority=direct_project_user_explicit_approval");
    println!("promotion_effect=false");
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    fn proposed_database(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let suffix = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "mindwarp-h7-approval-{name}-{}-{suffix}",
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
        forge.commit().unwrap();
        drop(forge);
        (directory, database)
    }

    #[test]
    fn exact_approval_is_one_event_backup_first_durable_and_idempotent() {
        let (directory, database) = proposed_database("idempotent");
        let backup = directory.join("pre-approval.sqlite3");
        let first = approve(&database, &backup).unwrap();
        assert!(!first.already_approved);
        assert_eq!(first.events_added, 1);
        assert_eq!(first.state, CandidateState::Approved);
        assert_eq!(first.pre_approval_backup.as_deref(), backup.to_str());
        assert_eq!(first.pre_approval_backup_sha256.as_ref().unwrap().len(), 64);

        let before = PersistentForge::open(&backup).unwrap();
        assert_eq!(
            before.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Proposed
        );
        assert_eq!(before.kernel().events().len(), 2);
        drop(before);

        let second = approve(&database, &backup).unwrap();
        assert!(second.already_approved);
        assert_eq!(second.events_added, 0);
        assert!(second.pre_approval_backup.is_none());
        let after = PersistentForge::open(&database).unwrap();
        assert_eq!(
            after.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Approved
        );
        assert_eq!(after.kernel().events().len(), 3);
        assert!(
            after
                .kernel()
                .events()
                .iter()
                .all(|event| event.event_type != EventType::CandidatePromoted)
        );
        drop(after);

        fs::remove_file(backup).unwrap();
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn approval_fails_closed_if_candidate_is_already_promoted() {
        let (directory, database) = proposed_database("state-drift");
        let mut forge = PersistentForge::open(&database).unwrap();
        forge
            .kernel_mut()
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                CANDIDATE_ID,
                "test:approval",
            )
            .unwrap();
        forge
            .kernel_mut()
            .promote_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                CANDIDATE_ID,
                "test:promotion",
            )
            .unwrap();
        forge.commit().unwrap();
        drop(forge);

        let unused_backup = directory.join("must-not-exist.sqlite3");
        assert!(approve(&database, &unused_backup).is_err());
        assert!(!unused_backup.exists());
        let reopened = PersistentForge::open(&database).unwrap();
        assert_eq!(reopened.kernel().events().len(), 4);
        assert_eq!(
            reopened.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Promoted
        );
        drop(reopened);
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }
}
