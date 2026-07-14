use std::path::Path;

use forge_kernel::{ActorKind, CandidateState, EventType, persistence::PersistentForge};
use humanoid_proof_chain::reference_promotion_package;

const EVIDENCE_ID: &str = "f564c5fd3c6f6c7c8619717b6dbbfc1790487b90b1b3328eef75a1592fccce4c";
const CANDIDATE_ID: &str = "c8df5d20b7bd87e09288689e6ef44ab56cabc3c8ce9a3ff95271262b3e9f4433";

#[derive(Debug, Eq, PartialEq)]
struct AdmissionReceipt {
    package_id: String,
    evidence_id: String,
    candidate_id: String,
    state: CandidateState,
    already_present: bool,
    events_added: usize,
    pre_admission_backup: Option<String>,
    pre_admission_backup_sha256: Option<String>,
}

fn admit(database: &Path, backup: &Path) -> Result<AdmissionReceipt, String> {
    let package = reference_promotion_package().map_err(|error| format!("{error:?}"))?;
    let package_bytes = package.to_bytes().map_err(|error| format!("{error:?}"))?;
    if package_bytes.len() != 1512 {
        return Err("canonical H7 package length drifted".into());
    }

    let mut forge = PersistentForge::open(database).map_err(|error| format!("{error:?}"))?;
    if let Some(candidate) = forge.kernel().candidate(CANDIDATE_ID) {
        if candidate.evidence != EVIDENCE_ID
            || candidate.state != CandidateState::Proposed
            || forge
                .kernel()
                .object(EVIDENCE_ID)
                .is_none_or(|object| object.bytes != package_bytes)
        {
            return Err(
                "existing H7 candidate identity, evidence, or Proposed state drifted".into(),
            );
        }
        return Ok(AdmissionReceipt {
            package_id: package.package_id,
            evidence_id: EVIDENCE_ID.into(),
            candidate_id: CANDIDATE_ID.into(),
            state: CandidateState::Proposed,
            already_present: true,
            events_added: 0,
            pre_admission_backup: None,
            pre_admission_backup_sha256: None,
        });
    }

    let backup_receipt = forge
        .backup_to(backup)
        .map_err(|error| format!("pre-admission backup failed: {error:?}"))?;
    PersistentForge::verify_backup(&backup_receipt)
        .map_err(|error| format!("pre-admission backup verification failed: {error:?}"))?;
    let events_before = forge.kernel().events().len();
    let evidence_id = forge
        .kernel_mut()
        .register_evidence(
            ActorKind::Assistant,
            &package_bytes,
            "h7:canonical-candidate-admission",
        )
        .map_err(|error| format!("{error:?}"))?;
    if evidence_id != EVIDENCE_ID {
        return Err("canonical H7 evidence identity drifted".into());
    }
    let candidate_id = forge
        .kernel_mut()
        .propose_candidate(&evidence_id, "h7:canonical-candidate-admission")
        .map_err(|error| format!("{error:?}"))?;
    if candidate_id != CANDIDATE_ID {
        return Err("canonical H7 candidate identity drifted".into());
    }
    let added = &forge.kernel().events()[events_before..];
    if added.len() != 2
        || added[0].event_type != EventType::EvidenceRegistered
        || added[1].event_type != EventType::CandidateProposed
        || added.iter().any(|event| {
            matches!(
                event.event_type,
                EventType::CandidateApproved
                    | EventType::CandidatePromoted
                    | EventType::CandidateSuperseded
                    | EventType::CodeApplied
            )
        })
    {
        return Err("H7 admission attempted an unexpected ledger effect".into());
    }
    forge.commit().map_err(|error| format!("{error:?}"))?;
    Ok(AdmissionReceipt {
        package_id: package.package_id,
        evidence_id,
        candidate_id,
        state: CandidateState::Proposed,
        already_present: false,
        events_added: 2,
        pre_admission_backup: Some(backup_receipt.path.display().to_string()),
        pre_admission_backup_sha256: Some(backup_receipt.sha256),
    })
}

fn main() {
    let database = std::env::args_os()
        .nth(1)
        .expect("usage: h7_candidate_admission <forge.sqlite3> <pre-admission-backup.sqlite3>");
    let backup = std::env::args_os()
        .nth(2)
        .expect("usage: h7_candidate_admission <forge.sqlite3> <pre-admission-backup.sqlite3>");
    let receipt = admit(Path::new(&database), Path::new(&backup))
        .expect("H7 candidate admission failed closed");
    println!("package_id={}", receipt.package_id);
    println!("kernel_evidence_id={}", receipt.evidence_id);
    println!("kernel_candidate_id={}", receipt.candidate_id);
    println!("kernel_candidate_state=proposed");
    println!("already_present={}", receipt.already_present);
    println!("events_added={}", receipt.events_added);
    println!(
        "pre_admission_backup={}",
        receipt.pre_admission_backup.as_deref().unwrap_or("none")
    );
    println!(
        "pre_admission_backup_sha256={}",
        receipt
            .pre_admission_backup_sha256
            .as_deref()
            .unwrap_or("none")
    );
    println!("authority=evidence_only_no_approval_or_promotion");
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    use forge_kernel::{ActorKind, AuthorityBasis};

    use super::*;

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    fn database_path(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let suffix = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "mindwarp-h7-admission-{name}-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        let database = directory.join("forge.sqlite3");
        (directory, database)
    }

    #[test]
    fn exact_admission_is_proposed_only_durable_and_idempotent() {
        let (directory, database) = database_path("idempotent");
        let backup = directory.join("pre-admission.sqlite3");
        let first = admit(&database, &backup).unwrap();
        assert!(!first.already_present);
        assert_eq!(first.events_added, 2);
        assert_eq!(
            first.pre_admission_backup.as_deref(),
            Some(backup.to_str().unwrap())
        );
        assert_eq!(
            first.pre_admission_backup_sha256.as_ref().unwrap().len(),
            64
        );
        let backup_state = PersistentForge::open(&backup).unwrap();
        assert_eq!(backup_state.kernel().candidate_count(), 0);
        assert_eq!(backup_state.kernel().events().len(), 0);
        drop(backup_state);

        let second = admit(&database, &backup).unwrap();
        assert!(second.already_present);
        assert_eq!(second.events_added, 0);
        assert!(second.pre_admission_backup.is_none());
        assert_eq!(second.candidate_id, first.candidate_id);

        let reopened = PersistentForge::open(&database).unwrap();
        assert_eq!(reopened.kernel().events().len(), 2);
        assert_eq!(reopened.kernel().candidate_count(), 1);
        assert_eq!(
            reopened.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Proposed
        );
        drop(reopened);
        fs::remove_file(backup).unwrap();
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn admission_fails_closed_if_existing_candidate_is_not_proposed() {
        let (directory, database) = database_path("state-drift");
        let backup = directory.join("pre-admission.sqlite3");
        admit(&database, &backup).unwrap();
        let mut forge = PersistentForge::open(&database).unwrap();
        forge
            .kernel_mut()
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                CANDIDATE_ID,
                "test:explicit-approval",
            )
            .unwrap();
        forge.commit().unwrap();
        drop(forge);

        let unused_backup = directory.join("must-not-exist.sqlite3");
        assert!(admit(&database, &unused_backup).is_err());
        assert!(!unused_backup.exists());
        let reopened = PersistentForge::open(&database).unwrap();
        assert_eq!(reopened.kernel().events().len(), 3);
        assert_eq!(
            reopened.kernel().candidate(CANDIDATE_ID).unwrap().state,
            CandidateState::Approved
        );
        drop(reopened);
        fs::remove_file(backup).unwrap();
        fs::remove_file(database).unwrap();
        fs::remove_dir(directory).unwrap();
    }
}
