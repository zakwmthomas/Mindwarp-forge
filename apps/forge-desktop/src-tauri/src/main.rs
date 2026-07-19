use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod codex_capture;
mod owner_presentation;
mod run_metrics_capture;

use forge_kernel::{
    ActorKind, AuthorityBasis, CandidateState, EventType, ForgeKernel,
    code_admission::{CodeAdmissionReceipt, CodePreview},
    contracts::{BridgeReceipt, ImportReport},
    persistence::{
        AppliedCodeReceipt, BackupReceipt, PersistentForge, ReferenceStudioRecords, WorkspaceFile,
        inventory_managed_workspace, inventory_workspace,
    },
};
use sha2::{Digest, Sha256};
use tauri::{Manager, State};
use tauri_plugin_autostart::ManagerExt;

struct AppState {
    forge: Arc<Mutex<PersistentForge>>,
    backup_directory: std::path::PathBuf,
    staging_directory: std::path::PathBuf,
    project_root: std::path::PathBuf,
    capture_status: Arc<Mutex<codex_capture::CaptureStatus>>,
    capture_enabled: Arc<AtomicBool>,
    codex_sessions_root: PathBuf,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct WorkspaceBinding {
    canonical_root: PathBuf,
    inventory: Vec<WorkspaceFile>,
    #[serde(default)]
    excluded_roots: Vec<String>,
    #[serde(default)]
    root_digest: String,
}

#[derive(serde::Serialize)]
struct KernelStatus {
    mode: &'static str,
    message: String,
    object_count: usize,
    event_count: usize,
    candidate_count: usize,
}

#[derive(serde::Serialize)]
struct DossierCandidate {
    id: String,
    evidence_id: String,
    state: CandidateState,
    history_event_count: usize,
}

#[derive(serde::Serialize)]
struct DossierSnapshot {
    object_count: usize,
    event_count: usize,
    candidates: Vec<DossierCandidate>,
    applications: Vec<AppliedRecord>,
}

#[derive(serde::Serialize)]
struct AppliedRecord {
    event_id: String,
    relative_path: String,
    preimage_object: Option<String>,
    postimage_object: Option<String>,
    rolled_back: bool,
}

#[derive(serde::Serialize)]
struct OwnerBriefItem {
    candidate_id: String,
    evidence_id: String,
    state: CandidateState,
}

#[derive(serde::Serialize)]
struct OwnerBrief {
    pending_decision_count: usize,
    visible_decisions: Vec<OwnerBriefItem>,
    truncated: bool,
}

#[derive(serde::Serialize)]
struct ForgeSnapshot {
    schema_version: u16,
    revision: String,
    read_only: bool,
    master_program: serde_json::Value,
    active_checkpoint: serde_json::Value,
    atlas: serde_json::Value,
    knowledge_records: Vec<forge_kernel::knowledge::KnowledgeRecord>,
}

#[derive(serde::Serialize)]
struct AuthorizationReceipt {
    action: &'static str,
    candidate_id: String,
    event_id: String,
}

const REFERENCE_STUDIO_SCHEMA_VERSION: u16 = 1;

#[derive(serde::Serialize)]
struct ReferenceStudioProjection {
    projection_schema_version: u16,
    requested_schema_version: u16,
    compatibility: &'static str,
    projection_source: &'static str,
    verified_at_ms: u128,
    read_only: bool,
    limitations: Vec<&'static str>,
    records: ReferenceStudioRecords,
}

#[tauri::command]
fn kernel_status(state: State<'_, AppState>) -> Result<KernelStatus, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    Ok(status_for(forge.kernel()))
}

#[tauri::command]
fn reference_viewport_snapshot() -> Result<reference_viewport::ViewportSnapshot, String> {
    reference_viewport::reference_snapshot().map_err(|error| error.to_string())
}

#[tauri::command]
fn reference_viewport_stimulus_bundle()
-> Result<viewport_stimulus::ControlledStimulusBundle, String> {
    viewport_stimulus::controlled_stimulus_bundle()
}

#[tauri::command]
fn record_reference_viewport_observation(
    input: viewport_stimulus::OwnerObservationInput,
) -> Result<viewport_stimulus::OwnerObservationReceipt, String> {
    viewport_stimulus::owner_observation_receipt(input)
}

#[tauri::command]
fn codex_capture_status(
    state: State<'_, AppState>,
) -> Result<codex_capture::CaptureStatus, String> {
    state
        .capture_status
        .lock()
        .map(|status| status.clone())
        .map_err(|_| "The Codex capture status lock is unavailable.".to_owned())
}

#[tauri::command]
fn pause_codex_capture(state: State<'_, AppState>) -> Result<codex_capture::CaptureStatus, String> {
    state.capture_enabled.store(false, Ordering::SeqCst);
    let mut status = state
        .capture_status
        .lock()
        .map_err(|_| "The Codex capture status lock is unavailable.".to_owned())?;
    status.state = "paused".into();
    Ok(status.clone())
}

#[tauri::command]
fn resume_codex_capture(
    state: State<'_, AppState>,
) -> Result<codex_capture::CaptureStatus, String> {
    state.capture_enabled.store(true, Ordering::SeqCst);
    scan_capture(
        &state.forge,
        &state.capture_status,
        &state.codex_sessions_root,
        &state.project_root,
    )?;
    codex_capture_status(state)
}

#[tauri::command]
fn rescan_codex_capture(
    state: State<'_, AppState>,
) -> Result<codex_capture::CaptureStatus, String> {
    scan_capture(
        &state.forge,
        &state.capture_status,
        &state.codex_sessions_root,
        &state.project_root,
    )?;
    codex_capture_status(state)
}

#[tauri::command]
fn import_labeled_transcript(
    source_id: String,
    transcript: String,
    state: State<'_, AppState>,
) -> Result<ImportReport, String> {
    import_transcript(&state.forge, source_id, transcript)
}

#[tauri::command]
fn ingest_codex_bridge_message(
    thread_id: String,
    message_id: String,
    role: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<BridgeReceipt, String> {
    let actor = match role.as_str() {
        "user" => ActorKind::ImportedContent,
        "assistant" => ActorKind::Assistant,
        _ => return Err("Bridge role must be user or assistant.".to_owned()),
    };
    let mut forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .ingest_codex_bridge_message(thread_id, message_id, actor, content.as_bytes())
        .map_err(|error| format!("Bridge message was not accepted: {error:?}"))
}

#[tauri::command]
fn dossier_snapshot(state: State<'_, AppState>) -> Result<DossierSnapshot, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    Ok(dossier_for(forge.kernel()))
}

#[tauri::command]
fn owner_brief(state: State<'_, AppState>) -> Result<OwnerBrief, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    Ok(owner_brief_for(forge.kernel()))
}

#[tauri::command]
fn reference_studio_snapshot(
    expected_schema_version: u16,
    state: State<'_, AppState>,
) -> Result<ReferenceStudioProjection, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System time is unavailable for inspection provenance.".to_owned())?
        .as_millis();
    reference_studio_for(&forge, expected_schema_version, now)
}

#[tauri::command]
fn authorize_candidate(
    action: String,
    candidate_id: String,
    confirmation: String,
    correction_evidence_id: Option<String>,
    replacement_candidate_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<AuthorizationReceipt, String> {
    authorize_candidate_for(
        &state.forge,
        &action,
        &candidate_id,
        &confirmation,
        correction_evidence_id.as_deref(),
        replacement_candidate_id.as_deref(),
    )
}

#[tauri::command]
fn create_local_backup(state: State<'_, AppState>) -> Result<BackupReceipt, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System time is unavailable for creating a unique backup name.".to_owned())?;
    let destination = state.backup_directory.join(format!(
        "forge-backup-{}-{}.sqlite3",
        now.as_secs(),
        now.subsec_nanos()
    ));
    create_backup_for(&state.forge, &destination)
}

#[tauri::command]
fn admit_pasted_code(
    source_id: String,
    relative_path: String,
    language: String,
    code: String,
    state: State<'_, AppState>,
) -> Result<CodeAdmissionReceipt, String> {
    let mut forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .admit_pasted_code(source_id, relative_path, language, code.as_bytes())
        .map_err(|error| format!("Code was not admitted: {error:?}"))
}

#[tauri::command]
fn preview_code_candidate(
    candidate_id: String,
    state: State<'_, AppState>,
) -> Result<CodePreview, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .preview_code_candidate(&candidate_id)
        .map_err(|error| format!("Code preview was not available: {error:?}"))
}

#[tauri::command]
fn apply_promoted_code(
    candidate_id: String,
    confirmation: String,
    state: State<'_, AppState>,
) -> Result<AppliedCodeReceipt, String> {
    let expected = format!("APPLY {candidate_id}");
    if confirmation.trim() != expected {
        return Err(format!("Confirmation must exactly equal: {expected}"));
    }
    let mut forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .apply_promoted_code(&candidate_id, &state.staging_directory)
        .map_err(|error| format!("Staging application was not accepted: {error:?}"))
}

#[tauri::command]
fn apply_to_approved_forge_workspace(
    candidate_id: String,
    confirmation: String,
    state: State<'_, AppState>,
) -> Result<AppliedCodeReceipt, String> {
    let expected = format!("APPLY-FORGE {candidate_id}");
    if confirmation.trim() != expected {
        return Err(format!("Confirmation must exactly equal: {expected}"));
    }
    let mut forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    apply_and_verify(
        &mut forge,
        &candidate_id,
        &state.project_root,
        run_full_verification,
    )
}

fn apply_and_verify(
    forge: &mut PersistentForge,
    candidate_id: &str,
    root: &Path,
    verifier: impl Fn(&Path) -> Result<(), String>,
) -> Result<AppliedCodeReceipt, String> {
    let receipt = forge
        .apply_promoted_code(candidate_id, root)
        .map_err(|error| format!("Forge workspace application was not accepted: {error:?}"))?;
    if let Err(error) = verifier(root) {
        let rollback = forge.rollback_application(&receipt.event_id, root);
        let _ = verifier(root);
        return Err(format!("{error} Automatic rollback result: {rollback:?}"));
    }
    Ok(receipt)
}

#[tauri::command]
fn rollback_application(
    application_event_id: String,
    confirmation: String,
    state: State<'_, AppState>,
) -> Result<AppliedCodeReceipt, String> {
    let expected = format!("ROLLBACK {application_event_id}");
    if confirmation.trim() != expected {
        return Err(format!("Confirmation must exactly equal: {expected}"));
    }
    let mut forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    let receipt = forge
        .rollback_application(&application_event_id, &state.project_root)
        .map_err(|error| format!("Rollback was not accepted: {error:?}"))?;
    run_full_verification(&state.project_root)?;
    Ok(receipt)
}

fn run_full_verification(project_root: &Path) -> Result<(), String> {
    let status = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "tools\\verify.ps1",
        ])
        .current_dir(project_root)
        .status()
        .map_err(|error| format!("Could not start full verification: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(
            "Full verification failed after application; rollback is pending implementation."
                .into(),
        )
    }
}

#[tauri::command]
fn staging_inventory(state: State<'_, AppState>) -> Result<Vec<WorkspaceFile>, String> {
    inventory_workspace(&state.staging_directory)
        .map_err(|error| format!("Workspace inventory was not available: {error:?}"))
}

#[tauri::command]
fn project_inventory(state: State<'_, AppState>) -> Result<Vec<WorkspaceFile>, String> {
    inventory_workspace(&state.project_root)
        .map_err(|error| format!("Approved Forge workspace inventory was not available: {error:?}"))
}

#[tauri::command]
fn workspace_binding(state: State<'_, AppState>) -> Result<WorkspaceBinding, String> {
    let binding_path = state
        .project_root
        .join(".local")
        .join("forge-workspace-binding.json");
    fs::read_to_string(&binding_path)
        .map_err(|error| format!("Workspace binding was not available: {error}"))
        .and_then(|text| serde_json::from_str(&text).map_err(|error| error.to_string()))
        .map_err(|error| format!("Workspace binding was invalid: {error}"))
}

#[tauri::command]
fn project_atlas(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    project_atlas_for(&state.project_root)
}

fn project_atlas_for(project_root: &Path) -> Result<serde_json::Value, String> {
    let mut atlas: serde_json::Value = serde_json::from_slice(
        &fs::read(project_root.join("docs/project-atlas/project-model.json"))
            .map_err(|error| format!("Project Atlas is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Project Atlas is invalid: {error}"))?;
    let master: serde_json::Value = serde_json::from_slice(
        &fs::read(project_root.join("docs/canonical-system/MASTER_PROGRAM.json"))
            .map_err(|error| format!("Master program is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Master program is invalid: {error}"))?;
    let items = master["items"]
        .as_array()
        .ok_or_else(|| "Master program items are invalid.".to_owned())?;
    let milestones = atlas["milestones"]
        .as_array_mut()
        .ok_or_else(|| "Project Atlas milestones are invalid.".to_owned())?;
    for milestone in milestones {
        let id = milestone["id"].as_str().unwrap_or_default();
        let matching: Vec<_> = items
            .iter()
            .filter(|item| item["milestone"].as_str() == Some(id))
            .collect();
        let status =
            if matching.is_empty() || matching.iter().all(|item| item["status"] == "complete") {
                "verified"
            } else if matching.iter().any(|item| item["status"] == "active") {
                "active"
            } else {
                "gated"
            };
        milestone["status"] = serde_json::Value::String(status.into());
    }
    Ok(atlas)
}

#[tauri::command]
fn operating_system_snapshot(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let policy = fs::read(state.project_root.join("governance/policy-registry.json"))
        .map_err(|error| format!("Policy registry is unavailable: {error}"))?;
    let checkpoint = fs::read(
        state
            .project_root
            .join("context/active/WORKER_BATCH_STATE.json"),
    )
    .map_err(|error| format!("Canonical active checkpoint is unavailable: {error}"))?;
    Ok(serde_json::json!({
        "policy": serde_json::from_slice::<serde_json::Value>(&policy).map_err(|error| format!("Policy registry is invalid: {error}"))?,
        "active_checkpoint": serde_json::from_slice::<serde_json::Value>(&checkpoint).map_err(|error| format!("Canonical active checkpoint is invalid: {error}"))?,
        "catalogue_available": state.project_root.join(".local/forge-bootstrap/EVIDENCE_CATALOG.json").exists(),
    }))
}

#[tauri::command]
fn owner_dashboard_snapshot(
    state: State<'_, AppState>,
) -> Result<owner_presentation::OwnerDashboardSnapshot, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    owner_presentation::owner_dashboard_for(&forge, &state.project_root)
}

#[tauri::command]
fn metrics_dashboard_snapshot(
    state: State<'_, AppState>,
    since_ms: Option<i64>,
) -> Result<serde_json::Value, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    let projection = forge
        .metrics_dashboard_projection_since(since_ms)
        .map_err(|error| format!("Metrics projection is unavailable: {error:?}"))?;
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &fs::read(
            state
                .project_root
                .join("context/active/WORKER_BATCH_STATE.json"),
        )
        .map_err(|error| format!("Active checkpoint is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Active checkpoint is invalid: {error}"))?;
    let criteria = checkpoint["exit_criteria"]
        .as_array()
        .cloned()
        .unwrap_or_else(|| {
            checkpoint["evidence_requirements"]
                .as_array()
                .map(|requirements| {
                    requirements
                        .iter()
                        .enumerate()
                        .map(|(index, label)| {
                            serde_json::json!({
                                "id": format!("legacy-criterion-{}", index + 1),
                                "label": label.as_str().unwrap_or("Unnamed criterion"),
                                "status": "planned",
                                "evidence_ids": [],
                            })
                        })
                        .collect()
                })
                .unwrap_or_default()
        });
    let verified = criteria
        .iter()
        .filter(|criterion| criterion["status"] == "verified")
        .count();
    Ok(serde_json::json!({
        "schema_version": 1,
        "read_only": true,
        "current": {
            "batch_id": checkpoint["batch_id"],
            "state": checkpoint["state"],
            "substage_id": checkpoint["substage_id"],
            "objective": checkpoint["objective"],
            "verified_criteria": verified,
            "total_criteria": criteria.len(),
            "criteria": criteria,
        },
        "projection": projection,
    }))
}

#[tauri::command]
fn forge_snapshot(state: State<'_, AppState>) -> Result<ForgeSnapshot, String> {
    let forge = state
        .forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge_snapshot_for(&forge, &state.project_root)
}

fn forge_snapshot_for(
    forge: &PersistentForge,
    project_root: &Path,
) -> Result<ForgeSnapshot, String> {
    let master_bytes = fs::read(project_root.join("docs/canonical-system/MASTER_PROGRAM.json"))
        .map_err(|error| format!("Master program is unavailable: {error}"))?;
    let checkpoint_bytes = fs::read(project_root.join("context/active/WORKER_BATCH_STATE.json"))
        .map_err(|error| format!("Active checkpoint is unavailable: {error}"))?;
    let atlas_bytes = fs::read(project_root.join("docs/project-atlas/project-model.json"))
        .map_err(|error| format!("Project Atlas is unavailable: {error}"))?;
    let knowledge_records = forge
        .knowledge_records()
        .map_err(|error| format!("Knowledge records are unavailable: {error:?}"))?;
    let knowledge_bytes = serde_json::to_vec(&knowledge_records)
        .map_err(|error| format!("Knowledge records are invalid: {error}"))?;
    let mut hasher = Sha256::new();
    for bytes in [
        &master_bytes,
        &checkpoint_bytes,
        &atlas_bytes,
        &knowledge_bytes,
    ] {
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    let revision = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok(ForgeSnapshot {
        schema_version: 1,
        revision,
        read_only: true,
        master_program: serde_json::from_slice(&master_bytes)
            .map_err(|error| format!("Master program is invalid: {error}"))?,
        active_checkpoint: serde_json::from_slice(&checkpoint_bytes)
            .map_err(|error| format!("Active checkpoint is invalid: {error}"))?,
        atlas: project_atlas_for(project_root)?,
        knowledge_records,
    })
}

fn import_transcript(
    forge: &Mutex<PersistentForge>,
    source_id: String,
    transcript: String,
) -> Result<ImportReport, String> {
    let mut forge = forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .ingest_labeled_transcript(source_id, transcript.as_bytes())
        .map_err(|error| format!("Transcript import was not accepted: {error:?}"))
}

fn status_for(kernel: &ForgeKernel) -> KernelStatus {
    KernelStatus {
        mode: "local-only",
        message: "Protected Kernel command boundary is active; local journal is verified on open."
            .to_owned(),
        object_count: kernel.object_count(),
        event_count: kernel.events().len(),
        candidate_count: kernel.candidate_count(),
    }
}

fn dossier_for(kernel: &ForgeKernel) -> DossierSnapshot {
    DossierSnapshot {
        object_count: kernel.object_count(),
        event_count: kernel.events().len(),
        candidates: kernel
            .candidates()
            .map(|candidate| DossierCandidate {
                id: candidate.id.clone(),
                evidence_id: candidate.evidence.clone(),
                state: candidate.state.clone(),
                history_event_count: candidate.evidence_events.len(),
            })
            .collect(),
        applications: kernel
            .events()
            .iter()
            .filter(|event| event.event_type == EventType::CodeApplied)
            .map(|event| {
                let payload = &event.payload;
                let rolled_back = kernel.events().iter().any(|rollback| {
                    rollback.event_type == EventType::CodeRolledBack
                        && rollback.prior_events.iter().any(|id| id == &event.id)
                });
                AppliedRecord {
                    event_id: event.id.clone(),
                    relative_path: payload["relative_path"]
                        .as_str()
                        .unwrap_or("invalid-path")
                        .to_owned(),
                    preimage_object: payload["preimage_object"].as_str().map(str::to_owned),
                    postimage_object: payload["postimage_object"].as_str().map(str::to_owned),
                    rolled_back,
                }
            })
            .collect(),
    }
}

fn owner_brief_for(kernel: &ForgeKernel) -> OwnerBrief {
    let mut pending: Vec<_> = kernel
        .candidates()
        .filter(|candidate| candidate.state == CandidateState::Proposed)
        .map(|candidate| OwnerBriefItem {
            candidate_id: candidate.id.clone(),
            evidence_id: candidate.evidence.clone(),
            state: candidate.state.clone(),
        })
        .collect();
    pending.sort_by(|left, right| left.candidate_id.cmp(&right.candidate_id));
    let pending_decision_count = pending.len();
    pending.truncate(5);
    OwnerBrief {
        pending_decision_count,
        truncated: pending_decision_count > pending.len(),
        visible_decisions: pending,
    }
}

fn reference_studio_for(
    forge: &PersistentForge,
    expected_schema_version: u16,
    verified_at_ms: u128,
) -> Result<ReferenceStudioProjection, String> {
    Ok(ReferenceStudioProjection {
        projection_schema_version: REFERENCE_STUDIO_SCHEMA_VERSION,
        requested_schema_version: expected_schema_version,
        compatibility: if expected_schema_version == REFERENCE_STUDIO_SCHEMA_VERSION {
            "compatible"
        } else {
            "version_mismatch"
        },
        projection_source: "verified-local-forge-sqlite",
        verified_at_ms,
        read_only: true,
        limitations: vec![
            "Projection data is informational and grants no approval, promotion, application, or execution authority.",
            "Raw capture content, arbitrary files, network sources, and runtime-engine objects are excluded.",
        ],
        records: forge
            .reference_studio_records()
            .map_err(|error| format!("Reference Studio records were not available: {error:?}"))?,
    })
}

fn create_backup_for(
    forge: &Mutex<PersistentForge>,
    destination: &Path,
) -> Result<BackupReceipt, String> {
    let mut forge = forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    forge
        .backup_to(destination)
        .map_err(|error| format!("Backup was not verified: {error:?}"))
}

fn authorize_candidate_for(
    forge: &Mutex<PersistentForge>,
    action: &str,
    candidate_id: &str,
    confirmation: &str,
    correction_evidence_id: Option<&str>,
    replacement_candidate_id: Option<&str>,
) -> Result<AuthorizationReceipt, String> {
    let correction_evidence_id = correction_evidence_id.filter(|value| !value.trim().is_empty());
    let replacement_candidate_id =
        replacement_candidate_id.filter(|value| !value.trim().is_empty());
    let expected_confirmation = match action {
        "approve" => format!("APPROVE {candidate_id}"),
        "promote" => format!("PROMOTE {candidate_id}"),
        "supersede" => {
            let correction = correction_evidence_id.ok_or_else(|| {
                "Supersession requires an existing correction evidence ID.".to_owned()
            })?;
            match replacement_candidate_id {
                Some(replacement) => {
                    format!("SUPERSEDE {candidate_id} USING {correction} WITH {replacement}")
                }
                None => format!("SUPERSEDE {candidate_id} USING {correction}"),
            }
        }
        _ => return Err("Action must be approve, promote, or supersede.".to_owned()),
    };
    if confirmation.trim() != expected_confirmation {
        return Err(format!(
            "Confirmation must exactly equal: {expected_confirmation}"
        ));
    }
    let mut forge = forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    let event_id = match action {
        "approve" => forge.kernel_mut().approve_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            candidate_id,
            "desktop:explicit-approval",
        ),
        "promote" => forge.kernel_mut().promote_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            candidate_id,
            "desktop:explicit-promotion",
        ),
        "supersede" => forge.kernel_mut().supersede_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            candidate_id,
            correction_evidence_id.expect("validated correction evidence"),
            replacement_candidate_id,
            "desktop:explicit-supersession",
        ),
        _ => unreachable!("validated action"),
    }
    .map_err(|error| format!("Authorization was not accepted: {error:?}"))?;
    forge
        .commit()
        .map_err(|error| format!("Authorization was not made durable: {error:?}"))?;
    Ok(AuthorizationReceipt {
        action: match action {
            "approve" => "approved",
            "promote" => "promoted",
            "supersede" => "superseded",
            _ => unreachable!("validated action"),
        },
        candidate_id: candidate_id.to_owned(),
        event_id,
    })
}

fn scan_capture(
    forge: &Arc<Mutex<PersistentForge>>,
    status: &Arc<Mutex<codex_capture::CaptureStatus>>,
    sessions_root: &Path,
    project_root: &Path,
) -> Result<(), String> {
    let mut forge = forge
        .lock()
        .map_err(|_| "The local Forge state lock is unavailable.".to_owned())?;
    let report = codex_capture::scan_all(&mut forge, sessions_root);
    codex_capture::write_bootstrap_pack(&forge, project_root, &report)?;
    let mut destination = status
        .lock()
        .map_err(|_| "The Codex capture status lock is unavailable.".to_owned())?;
    *destination = report.into();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let data_directory = app
                .path()
                .app_data_dir()
                .map_err(|error| std::io::Error::other(error.to_string()))?;
            std::fs::create_dir_all(&data_directory)?;
            let forge = PersistentForge::open(data_directory.join("forge.sqlite3"))
                .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
            forge
                .backfill_knowledge_records()
                .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
            let forge = Arc::new(Mutex::new(forge));
            let codex_sessions_root =
                codex_capture::default_sessions_root().map_err(std::io::Error::other)?;
            let capture_status = Arc::new(Mutex::new(codex_capture::CaptureStatus {
                state: "starting".into(),
                ..Default::default()
            }));
            let capture_enabled = Arc::new(AtomicBool::new(true));
            let project_root = fs::canonicalize(r"C:\Users\zakwm\Desktop\Mindwarp forge")?;
            let watcher_project_root = project_root.clone();
            let binding_root = project_root.clone();
            app.manage(AppState {
                forge: forge.clone(),
                backup_directory: data_directory.join("backups"),
                staging_directory: data_directory.join("staging-workspace"),
                project_root,
                capture_status: capture_status.clone(),
                capture_enabled: capture_enabled.clone(),
                codex_sessions_root: codex_sessions_root.clone(),
            });
            thread::spawn(move || {
                if let Ok(inventory) = inventory_managed_workspace(&binding_root) {
                    let binding = WorkspaceBinding {
                        canonical_root: binding_root.clone(),
                        inventory: inventory.files,
                        excluded_roots: inventory.excluded_roots,
                        root_digest: inventory.root_digest,
                    };
                    let binding_directory = binding_root.join(".local");
                    if fs::create_dir_all(&binding_directory).is_ok() {
                        if let Ok(bytes) = serde_json::to_vec_pretty(&binding) {
                            let _ = fs::write(
                                binding_directory.join("forge-workspace-binding.json"),
                                bytes,
                            );
                        }
                    }
                }
            });
            let watcher_forge = forge;
            let watcher_status = capture_status;
            thread::spawn(move || {
                loop {
                    if capture_enabled.load(Ordering::SeqCst) {
                        let _ = scan_capture(
                            &watcher_forge,
                            &watcher_status,
                            &codex_sessions_root,
                            &watcher_project_root,
                        );
                    }
                    if let Ok(forge) = watcher_forge.lock() {
                        let _ = run_metrics_capture::scan_inbox(&forge, &watcher_project_root);
                    }
                    thread::sleep(Duration::from_secs(2));
                }
            });
            app.autolaunch()
                .enable()
                .map_err(|error| std::io::Error::other(error.to_string()))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            kernel_status,
            reference_viewport_snapshot,
            reference_viewport_stimulus_bundle,
            record_reference_viewport_observation,
            codex_capture_status,
            pause_codex_capture,
            resume_codex_capture,
            rescan_codex_capture,
            ingest_codex_bridge_message,
            import_labeled_transcript,
            dossier_snapshot,
            create_local_backup,
            admit_pasted_code,
            owner_brief,
            reference_studio_snapshot,
            authorize_candidate,
            preview_code_candidate,
            apply_promoted_code,
            staging_inventory,
            project_inventory,
            project_atlas,
            operating_system_snapshot,
            owner_dashboard_snapshot,
            metrics_dashboard_snapshot,
            forge_snapshot,
            workspace_binding,
            apply_to_approved_forge_workspace,
            rollback_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running Mind Warp Forge desktop application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stellar_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::StellarOrbitalContract {
        derived_world_rules::compile_stellar_orbital(&derived_world_rules::StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn geological_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::GeologicalAtmosphericContract {
        derived_world_rules::compile_geological_atmospheric(
            &derived_world_rules::GeologicalAtmosphericInput {
                schema_version: 1,
                reconstruction_id,
                planetary_body_id: [4; 32],
                stellar_orbital: stellar_contract(reconstruction_id),
                planet_mass_milli_earth: 1_000,
                planet_radius_milli_earth: 1_000,
                internal_heat_flux_milli_w_m2: 87,
                solid_surface_fraction_permille: 600,
                atmospheric_column_mass_g_m2: 10_332_000,
                gas_transmission_rgb_permille: [800, 900, 950],
                aerosol_transmission_rgb_permille: [1_000; 3],
            },
        )
        .unwrap()
    }

    fn hydrological_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::HydrologicalContract {
        derived_world_rules::compile_hydrological(&derived_world_rules::HydrologicalInput {
            schema_version: 1,
            reconstruction_id,
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract(reconstruction_id),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap()
    }

    fn climate_contract(reconstruction_id: [u8; 32]) -> derived_world_rules::ClimateContract {
        derived_world_rules::compile_climate(&derived_world_rules::ClimateInput {
            schema_version: 1,
            reconstruction_id,
            climate_source_id: [6; 32],
            hydrological: hydrological_contract(reconstruction_id),
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }

    fn surface_material_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::SurfaceMaterialContract {
        derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id,
            material_source_id: [7; 32],
            climate: climate_contract(reconstruction_id),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap()
    }

    fn regional_environment_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::RegionalEnvironmentContract {
        derived_world_rules::compile_regional_environment(
            &derived_world_rules::RegionalEnvironmentInput {
                schema_version: 1,
                reconstruction_id,
                regional_source_id: [8; 32],
                field_recipe_bytes: field_basis::FieldRecipe::new(
                    vec![field_basis::Term::Constant(field_basis::ONE)],
                    0,
                )
                .unwrap()
                .encode_canonical()
                .unwrap(),
                moisture_source_id: [9; 32],
                moisture_field_recipe_bytes: field_basis::FieldRecipe::new(
                    vec![field_basis::Term::Constant(0)],
                    0,
                )
                .unwrap()
                .encode_canonical()
                .unwrap(),
                coordinate_q32_32: [0, 0],
            },
        )
        .unwrap()
    }

    #[test]
    fn unified_snapshot_is_revisioned_read_only_and_contains_typed_knowledge() {
        let root = std::env::temp_dir().join(format!(
            "forge-snapshot-fixture-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("docs/canonical-system")).unwrap();
        fs::create_dir_all(root.join("docs/project-atlas")).unwrap();
        fs::create_dir_all(root.join("context/active")).unwrap();
        fs::write(
            root.join("docs/canonical-system/MASTER_PROGRAM.json"),
            br#"{"schema_version":1,"items":[{"id":"F5-COHERENCE","status":"active"}]}"#,
        )
        .unwrap();
        fs::write(
            root.join("context/active/WORKER_BATCH_STATE.json"),
            br#"{"batch_id":"fixture","next_action":"verify"}"#,
        )
        .unwrap();
        fs::write(
            root.join("docs/project-atlas/project-model.json"),
            br#"{"project":{"name":"Forge","vision":"coherent"},"systems":[],"milestones":[]}"#,
        )
        .unwrap();
        let mut forge = PersistentForge::in_memory().unwrap();
        forge
            .ingest_codex_bridge_message(
                "snapshot-thread",
                "message-1",
                ActorKind::Assistant,
                b"Proposed plan: retain one canonical record.",
            )
            .unwrap();
        let first = forge_snapshot_for(&forge, &root).unwrap();
        let second = forge_snapshot_for(&forge, &root).unwrap();
        assert!(first.read_only);
        assert_eq!(first.revision, second.revision);
        assert_eq!(first.knowledge_records.len(), 1);
        assert_eq!(first.knowledge_records[0].authority_lane, "evidence_only");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn status_command_is_local_only() {
        let status = status_for(&ForgeKernel::default());
        assert_eq!(status.mode, "local-only");
        assert!(status.message.contains("Kernel"));
        assert_eq!(status.object_count, 0);
        assert_eq!(status.event_count, 0);
        assert_eq!(status.candidate_count, 0);
    }

    #[test]
    fn built_in_reference_viewport_is_data_only_and_stable() {
        let first = reference_viewport_snapshot().unwrap();
        let second = reference_viewport_snapshot().unwrap();
        assert_eq!(first, second);
        assert_eq!(first.mode, "built-in-data-only");
        assert!(first.read_only);
        assert!(
            first
                .limitations
                .iter()
                .any(|limit| limit.contains("no files"))
        );
    }

    #[test]
    fn controlled_viewport_stimuli_are_bound_and_owner_observations_remain_pending() {
        let bundle = reference_viewport_stimulus_bundle().unwrap();
        assert_eq!(bundle.status, "stimuli_ready_observations_pending");
        assert_eq!(bundle.stimuli.len(), 4);
        assert_eq!(bundle.observed_claim_count, 0);
        assert!(
            bundle
                .protocol_package
                .observations
                .observations
                .iter()
                .all(|item| item.outcome == perception_protocol::ReviewOutcome::NotObserved)
        );
    }

    #[test]
    fn direct_viewport_observation_returns_receipt_without_authority_effect() {
        let bundle = reference_viewport_stimulus_bundle().unwrap();
        let receipt =
            record_reference_viewport_observation(viewport_stimulus::OwnerObservationInput {
                expected_base_scene_fingerprint: bundle.base_scene_fingerprint,
                pair_index: 2,
                outcome: perception_protocol::ReviewOutcome::Indeterminate,
                confidence: 40,
            })
            .unwrap();
        assert_eq!(receipt.status, "validated_direct_owner_observation");
        assert_eq!(receipt.observed_claim_count, 1);
        assert_eq!(receipt.authority_effect, "none");
    }

    #[test]
    fn explicit_transcript_command_returns_a_receipt_without_auto_approval() {
        let forge = Mutex::new(PersistentForge::in_memory().unwrap());
        let receipt = import_transcript(
            &forge,
            "manual-test".into(),
            "Assistant: Preserve provenance.\nUser: Approved.".into(),
        )
        .unwrap();
        assert_eq!(receipt.message_count, 2);
        assert_eq!(receipt.candidate_count, 1);
        assert_eq!(receipt.approval_intents, 1);
        assert_eq!(forge.lock().unwrap().kernel().candidate_count(), 1);
    }

    #[test]
    fn dossier_projection_is_read_only_and_matches_candidate_state() {
        let mut forge = PersistentForge::in_memory().unwrap();
        forge
            .ingest_labeled_transcript("dossier-test", b"Assistant: Preserve evidence.")
            .unwrap();
        let before_events = forge.kernel().events().len();
        let snapshot = dossier_for(forge.kernel());
        assert_eq!(snapshot.object_count, forge.kernel().object_count());
        assert_eq!(snapshot.event_count, before_events);
        assert_eq!(snapshot.candidates.len(), 1);
        assert_eq!(snapshot.candidates[0].state, CandidateState::Proposed);
        assert!(snapshot.applications.is_empty());
        assert_eq!(forge.kernel().events().len(), before_events);
    }

    #[test]
    fn backup_command_returns_a_fixity_receipt() {
        let directory = std::env::temp_dir().join(format!(
            "mindwarp-forge-desktop-backup-{}",
            std::process::id()
        ));
        let destination = directory.join("verified.sqlite3");
        let forge = Mutex::new(PersistentForge::in_memory().unwrap());
        let receipt = create_backup_for(&forge, &destination).unwrap();
        assert_eq!(receipt.path, destination);
        assert_eq!(receipt.sha256.len(), 64);
        drop(forge);
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn code_admission_stays_a_candidate_after_durable_recording() {
        let mut forge = PersistentForge::in_memory().unwrap();
        let receipt = forge
            .admit_pasted_code("code-test", "src/demo.rs", "rust", b"fn demo() {}")
            .unwrap();
        assert!(!receipt.already_recorded);
        assert_eq!(forge.kernel().candidate_count(), 1);
    }

    #[test]
    fn owner_brief_is_bounded_and_read_only() {
        let mut forge = PersistentForge::in_memory().unwrap();
        for index in 0..6 {
            forge
                .ingest_labeled_transcript(
                    format!("brief-{index}"),
                    format!("Assistant: Candidate {index}.").as_bytes(),
                )
                .unwrap();
        }
        let events_before = forge.kernel().events().len();
        let brief = owner_brief_for(forge.kernel());
        assert_eq!(brief.pending_decision_count, 6);
        assert_eq!(brief.visible_decisions.len(), 5);
        assert!(brief.truncated);
        assert_eq!(forge.kernel().events().len(), events_before);
    }

    #[test]
    fn authorization_requires_exact_phrase_and_separate_promotion() {
        let forge = Mutex::new(PersistentForge::in_memory().unwrap());
        let candidate = forge
            .lock()
            .unwrap()
            .ingest_labeled_transcript("auth-test", b"Assistant: Preserve boundaries.")
            .unwrap();
        let candidate_id = forge
            .lock()
            .unwrap()
            .kernel()
            .candidates()
            .next()
            .unwrap()
            .id
            .clone();
        assert!(
            authorize_candidate_for(
                &forge,
                "approve",
                &candidate_id,
                "approve anything",
                None,
                None,
            )
            .is_err()
        );
        let approval = authorize_candidate_for(
            &forge,
            "approve",
            &candidate_id,
            &format!("APPROVE {candidate_id}"),
            None,
            None,
        )
        .unwrap();
        assert_eq!(approval.action, "approved");
        let promotion = authorize_candidate_for(
            &forge,
            "promote",
            &candidate_id,
            &format!("PROMOTE {candidate_id}"),
            None,
            None,
        )
        .unwrap();
        assert_eq!(promotion.action, "promoted");
        assert_eq!(candidate.candidate_count, 1);
    }

    #[test]
    fn supersession_requires_correction_binding_and_exact_distinct_phrase() {
        let forge = Mutex::new(PersistentForge::in_memory().unwrap());
        let candidate_id = {
            let mut forge = forge.lock().unwrap();
            forge
                .ingest_labeled_transcript("supersede-auth", b"Assistant: Withdrawable candidate.")
                .unwrap();
            let candidate_id = forge.kernel().candidates().next().unwrap().id.clone();
            forge
                .kernel_mut()
                .approve_candidate(
                    ActorKind::DirectProjectUser,
                    AuthorityBasis::ExplicitUserAuthorization,
                    &candidate_id,
                    "supersede-auth",
                )
                .unwrap();
            forge.commit().unwrap();
            candidate_id
        };
        let correction = forge
            .lock()
            .unwrap()
            .kernel_mut()
            .register_evidence(
                ActorKind::DirectProjectUser,
                b"Owner correction",
                "supersede-auth",
            )
            .unwrap();

        assert!(
            authorize_candidate_for(
                &forge,
                "supersede",
                &candidate_id,
                &format!("SUPERSEDE {candidate_id}"),
                None,
                None,
            )
            .is_err()
        );
        assert!(
            authorize_candidate_for(
                &forge,
                "supersede",
                &candidate_id,
                &format!("SUPERSEDE {candidate_id} USING wrong"),
                Some(&correction),
                None,
            )
            .is_err()
        );
        let receipt = authorize_candidate_for(
            &forge,
            "supersede",
            &candidate_id,
            &format!("SUPERSEDE {candidate_id} USING {correction}"),
            Some(&correction),
            None,
        )
        .unwrap();
        assert_eq!(receipt.action, "superseded");
        assert_eq!(
            forge
                .lock()
                .unwrap()
                .kernel()
                .candidate(&candidate_id)
                .unwrap()
                .state,
            CandidateState::Superseded
        );
    }

    #[test]
    fn code_preview_command_reads_but_does_not_mutate_the_ledger() {
        let mut forge = PersistentForge::in_memory().unwrap();
        let receipt = forge
            .admit_pasted_code("preview-test", "src/demo.rs", "rust", b"fn demo() {}")
            .unwrap();
        let events_before = forge.kernel().events().len();
        let preview = forge.preview_code_candidate(&receipt.candidate).unwrap();
        assert_eq!(preview.code, "fn demo() {}");
        assert_eq!(forge.kernel().events().len(), events_before);
    }

    #[test]
    fn failed_verification_automatically_rolls_back_a_new_file() {
        let directory = std::env::temp_dir().join(format!(
            "mindwarp-forge-auto-rollback-{}",
            std::process::id()
        ));
        let mut forge = PersistentForge::in_memory().unwrap();
        let candidate = forge
            .admit_pasted_code("auto", "src/demo.rs", "rust", b"new")
            .unwrap()
            .candidate;
        forge
            .kernel_mut()
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "test",
            )
            .unwrap();
        forge
            .kernel_mut()
            .promote_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "test",
            )
            .unwrap();
        assert!(
            apply_and_verify(&mut forge, &candidate, &directory, |_| Err(
                "forced failure".into()
            ))
            .is_err()
        );
        assert!(!directory.join("src/demo.rs").exists());
        assert!(
            forge
                .kernel()
                .events()
                .iter()
                .any(|event| event.event_type == EventType::CodeRolledBack)
        );
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn reference_studio_is_read_only_and_exposes_failures_blockers_and_source_gaps() {
        use forge_kernel::contracts::{
            BlockerRecord, GateReceiptRecord, NamedVersion, ProofMeasurement, ProofReceiptRecord,
            ResearchSourceRecord, WorkPackageRecord,
        };
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let package = WorkPackageRecord {
            id: "studio-package".into(),
            stage: "research".into(),
            dependencies: vec!["B2".into()],
            risk: "high".into(),
            evidence_requirements: vec!["local fixture".into()],
            verification_plan: vec!["mutation-negative".into()],
            authority_lane: "delegated".into(),
            next_action: "inspect only".into(),
        };
        forge.record_work_package(&package).unwrap();
        let failed = GateReceiptRecord {
            id: "studio-failure".into(),
            work_package_id: package.id.clone(),
            from_stage: "research".into(),
            to_stage: "research".into(),
            outcome: "failed".into(),
            evidence_ids: vec!["missing-source".into()],
            failure_reason: Some("source unavailable".into()),
            rollback_target: Some("known-good".into()),
        };
        forge.record_gate_receipt(&failed).unwrap();
        forge
            .record_blocker(&BlockerRecord {
                id: "studio-blocker".into(),
                work_package_id: package.id.clone(),
                blocker_type: "dependency".into(),
                affected_stage: "research".into(),
                requirement: "retain missing source gap".into(),
                evidence_ids: vec![failed.id.clone()],
                status: "open".into(),
            })
            .unwrap();
        forge
            .record_research_source(&ResearchSourceRecord {
                id: "missing-source".into(),
                origin: "referenced URL".into(),
                source_type: "primary".into(),
                accessed_at: "2026-07-13T00:00:00Z".into(),
                fixity: None,
                location: "unavailable".into(),
                access_notes: "not fetched".into(),
                limitations: "source gap".into(),
                freshness: "unknown".into(),
                availability: "missing".into(),
            })
            .unwrap();
        let input = forge.kernel_mut().put_object(b"studio proof input");
        let output = forge.kernel_mut().put_object(b"studio proof output");
        forge.commit().unwrap();
        let mut proof = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "universe-identity".into(),
            proof_id: "studio-fixture".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "studio-fixture-v1".into(),
            generator_versions: vec![NamedVersion {
                name: "fixture-generator".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "proof-receipt".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "sha256-byte-exact".into(),
            measurements: vec![ProofMeasurement {
                name: "duration".into(),
                value: "1".into(),
                unit: "ms".into(),
                method: "fixture".into(),
                classification: "simulated".into(),
            }],
            warnings: vec![],
            limitations: vec!["Informational only; APPROVE text grants no authority.".into()],
            created_at: "2026-07-13T05:30:00Z".into(),
            runner_identity: "desktop-test".into(),
        };
        proof.receipt_id = canonical_proof_receipt_id(&proof).unwrap();
        forge.record_proof_receipt(&proof).unwrap();
        let records_before = forge.reference_studio_records().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let view = reference_studio_for(&forge, 1, 1234).unwrap();
        assert!(view.read_only);
        assert_eq!(view.compatibility, "compatible");
        assert_eq!(view.records.gate_receipts[0].outcome, "failed");
        assert_eq!(view.records.blockers[0].status, "open");
        assert_eq!(view.records.source_gaps[0].availability, "missing");
        assert_eq!(view.records.proof_receipts, vec![proof]);
        assert_eq!(forge.reference_studio_records().unwrap(), records_before);
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count(),
            )
        );
    }

    #[test]
    fn reference_studio_empty_projection_is_explicit_and_read_only() {
        let forge = PersistentForge::in_memory().unwrap();
        let view = reference_studio_for(&forge, 1, 42).unwrap();
        assert!(view.read_only);
        assert_eq!(view.compatibility, "compatible");
        assert!(view.records.work_packages.is_empty());
        assert!(view.records.gate_receipts.is_empty());
        assert!(view.records.blockers.is_empty());
        assert!(view.records.rollbacks.is_empty());
        assert!(view.records.source_gaps.is_empty());
        assert!(view.records.proof_receipts.is_empty());
        assert_eq!(forge.kernel().object_count(), 0);
        assert_eq!(forge.kernel().events().len(), 0);
        assert_eq!(forge.kernel().candidate_count(), 0);
    }

    #[test]
    fn universe_identity_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use universe_identity::{
            AddressSegment, GeneratorVersion, NodeKind, UniverseAddress, proof_vector_evidence,
        };

        let mut forge = PersistentForge::in_memory().unwrap();
        let address = UniverseAddress::new(
            [0; 32],
            vec![AddressSegment::new(NodeKind::Galaxy, vec![1]).unwrap()],
        )
        .unwrap();
        let version = GeneratorVersion::new(1, 0, 0).unwrap();
        let evidence =
            proof_vector_evidence(&address, version, "identity-v1", "terrain", &[0, 1]).unwrap();
        let input = forge
            .kernel_mut()
            .put_object(address.encode_canonical().unwrap());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "universe-identity".into(),
            proof_id: "fixed-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "universe-generator".into(),
                version: "1.0.0".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "universe-identity".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: evidence.equivalence_method.clone(),
            measurements: vec![ProofMeasurement {
                name: "fixture_duration".into(),
                value: "0".into(),
                unit: "ms".into(),
                method: "deterministic-test-harness".into(),
                classification: evidence.measurement_classification.clone(),
            }],
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T05:47:00Z".into(),
            runner_identity: "forge-desktop-universe-identity-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        let view = reference_studio_for(&forge, 1, 9001).unwrap();
        assert_eq!(view.records.proof_receipts, vec![receipt]);
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count(),
            )
        );
    }

    #[test]
    fn field_basis_vector_persists_as_read_only_proof_receipt() {
        use field_basis::{FieldProofEvidence, FieldRecipe, ONE, Term, sample};
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let recipe = FieldRecipe::new(
            vec![
                Term::ValueLattice2 {
                    frequency: 2,
                    amplitude: ONE,
                    component: 7,
                },
                Term::Ridged { input: 0 },
            ],
            1,
        )
        .unwrap();
        let result = sample(&recipe, [3; 32], 123_456_789, -987_654_321).unwrap();
        let input = forge
            .kernel_mut()
            .put_object(recipe.encode_canonical().unwrap());
        let evidence = FieldProofEvidence {
            fixture_id: "field-fixed-v1".into(),
            exact: true,
            canonical: true,
            limitations: vec!["CPU fixed-point reference; not runtime performance".into()],
        };
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&(result, evidence.clone())).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "field-basis".into(),
            proof_id: "fixed-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "philox4x32".into(),
                version: "10-round-v1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "field-basis".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![ProofMeasurement {
                name: "fixture_duration".into(),
                value: "0".into(),
                unit: "ms".into(),
                method: "deterministic-test-harness".into(),
                classification: "simulated".into(),
            }],
            warnings: vec![],
            limitations: evidence.limitations,
            created_at: "2026-07-13T06:22:00Z".into(),
            runner_identity: "forge-desktop-field-basis-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9002)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn stellar_orbital_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let contract = stellar_contract([1; 32]);
        let input = forge
            .kernel_mut()
            .put_object(contract.input.to_bytes().unwrap());
        let output = forge
            .kernel_mut()
            .put_object(contract.state.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "stellar-orbital".into(),
            proof_id: "bounded-two-body-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "stellar-orbital-v1/earth-normalized".into(),
            generator_versions: vec![NamedVersion {
                name: "stellar-orbital-compiler".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "stellar-orbital".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes and input replay".into(),
            measurements: vec![ProofMeasurement {
                name: "mean_distance_irradiance".into(),
                value: contract
                    .state
                    .content
                    .irradiance_mean_distance_millionths_earth
                    .to_string(),
                unit: "millionths_earth_flux".into(),
                method: "bounded-integer-inverse-square-reference".into(),
                classification: "simulated".into(),
            }],
            warnings: vec![],
            limitations: contract.state.content.limitations.clone(),
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-stellar-orbital-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9002)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn geological_atmospheric_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let contract = geological_contract([1; 32]);
        let input = forge
            .kernel_mut()
            .put_object(contract.input.to_bytes().unwrap());
        let output = forge
            .kernel_mut()
            .put_object(contract.state.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "geological-atmospheric".into(),
            proof_id: "bounded-gravity-column-transmission-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "geological-atmospheric-v1/earth-normalized".into(),
            generator_versions: vec![NamedVersion {
                name: "geological-atmospheric-compiler".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "geological-atmospheric".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes and input replay".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "surface_gravity".into(),
                    value: contract.state.content.surface_gravity_mm_s2.to_string(),
                    unit: "millimetres_per_second_squared".into(),
                    method: "earth-normalized-spherical-gravity-reference".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "surface_pressure".into(),
                    value: contract.state.content.surface_pressure_pa.to_string(),
                    unit: "pascal".into(),
                    method: "column-mass-times-surface-gravity".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: contract.state.content.limitations.clone(),
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-geological-atmospheric-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9002)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn hydrological_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let contract = hydrological_contract([1; 32]);
        let input = forge
            .kernel_mut()
            .put_object(contract.input.to_bytes().unwrap());
        let output = forge
            .kernel_mut()
            .put_object(contract.state.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "hydrological-state".into(),
            proof_id: "bounded-water-inventory-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "hydrological-state-v1/declared-reservoirs".into(),
            generator_versions: vec![NamedVersion {
                name: "hydrological-state-compiler".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "hydrological-state".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes and input replay".into(),
            measurements: vec![ProofMeasurement {
                name: "surface_accessible_liquid_column".into(),
                value: contract
                    .state
                    .content
                    .surface_accessible_liquid_column_millionths_g_m2
                    .to_string(),
                unit: "millionths_gram_per_square_metre".into(),
                method: "declared-inventory-partition-and-access-product".into(),
                classification: "simulated".into(),
            }],
            warnings: vec![],
            limitations: contract.state.content.limitations.clone(),
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-hydrological-state-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9002)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn derived_world_vector_persists_as_read_only_proof_receipt() {
        use derived_world_rules::{
            SignalChannel, SignalPotential, WorldGenerationInput, compile_world,
        };
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let world_input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            surface_material: surface_material_contract([1; 32]),
            regional_environment: regional_environment_contract([1; 32]),
            signal_potentials: vec![SignalPotential {
                channel: SignalChannel::MagneticField,
                baseline_strength_permille: 800,
            }],
        };
        let packet = compile_world(&world_input).unwrap();
        let input = forge
            .kernel_mut()
            .put_object(world_input.to_bytes().unwrap());
        let output = forge.kernel_mut().put_object(packet.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "derived-world-rules".into(),
            proof_id: "causal-world-vector".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "derived-world-rules-v1/causal-world".into(),
            generator_versions: vec![NamedVersion {
                name: "derived-world-compiler".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "derived-world-rules".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![ProofMeasurement {
                name: "fixture_duration".into(),
                value: "0".into(),
                unit: "ms".into(),
                method: "deterministic-test-harness".into(),
                classification: "simulated".into(),
            }],
            warnings: vec![],
            limitations: packet.content.limitations.clone(),
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-derived-world-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9003)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn hierarchy_history_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use hierarchy_history::reference_proof_evidence;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.descriptor_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "world-history-ledger".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "hierarchy-history-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "hierarchy-history".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-sha256-and-replay".into(),
            measurements: evidence
                .measured_window_sizes
                .iter()
                .map(|(logical, returned, examined)| ProofMeasurement {
                    name: format!("window_{logical}"),
                    value: format!("{returned}/{examined}"),
                    unit: "returned/examined".into(),
                    method: "deterministic-test-harness".into(),
                    classification: evidence.measurement_classification.clone(),
                })
                .collect(),
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T07:18:00Z".into(),
            runner_identity: "forge-desktop-hierarchy-history-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9003)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn organism_niche_binding_vector_persists_as_read_only_proof_receipt() {
        use derived_world_rules::{
            SignalChannel, SignalPotential, WorldGenerationInput, compile_world,
        };
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use organism_niche_binding::build_distance_sensing_niche_package;
        use semantic_construction::validate_package;

        let mut forge = PersistentForge::in_memory().unwrap();
        let world_input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [5; 32],
            surface_material: surface_material_contract([5; 32]),
            regional_environment: regional_environment_contract([5; 32]),
            signal_potentials: vec![SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 900,
            }],
        };
        let packet = compile_world(&world_input).unwrap();
        let package = build_distance_sensing_niche_package(&world_input, &packet).unwrap();
        let report = validate_package(&package, 512);

        let input = forge.kernel_mut().put_object(packet.to_bytes().unwrap());
        let output = forge.kernel_mut().put_object(package.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let feasible_count = package
            .solutions
            .families
            .iter()
            .filter(|item| item.feasible)
            .count();
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "organism-ecology".into(),
            proof_id: "distance-sensing-environmental-support-prerequisite".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "organism-niche-binding-v1/distance-sensing-niche".into(),
            generator_versions: vec![NamedVersion {
                name: "organism-niche-binding".into(),
                version: "1".into(),
            }],
            contract_versions: vec![
                NamedVersion {
                    name: "derived-world-rules".into(),
                    version: "1".into(),
                },
                NamedVersion {
                    name: "semantic-construction".into(),
                    version: "1".into(),
                },
            ],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "environmentally_supported_candidate_count".into(),
                    value: feasible_count.to_string(),
                    unit: "mechanisms".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "validation_examined".into(),
                    value: report.examined.to_string(),
                    unit: "fixture_items".into(),
                    method: "bounded-integer-reference-validator".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: vec![
                "tiny disposable sensory-mechanism fixture vocabulary; not the Mind Warp content grammar".into(),
                "environmental support only; sensory physiology, person-form eligibility and applicable dimorphism remain open".into(),
                "no biological, physiological, or perceptual-realism claim".into(),
            ],
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-organism-niche-binding-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9003)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn niche_graph_binding_vector_persists_as_read_only_proof_receipt() {
        use derived_world_rules::{
            SignalChannel, SignalPotential, WorldGenerationInput, compile_world,
        };
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use niche_graph_binding::{
            build_environmental_opportunity_graph, validate_environmental_opportunity_graph,
        };

        let mut forge = PersistentForge::in_memory().unwrap();
        let world_input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [7; 32],
            surface_material: surface_material_contract([7; 32]),
            regional_environment: regional_environment_contract([7; 32]),
            signal_potentials: vec![
                SignalPotential {
                    channel: SignalChannel::VisibleRadiance,
                    baseline_strength_permille: 900,
                },
                SignalPotential {
                    channel: SignalChannel::ElectricField,
                    baseline_strength_permille: 800,
                },
            ],
        };
        let packet = compile_world(&world_input).unwrap();
        let graph = build_environmental_opportunity_graph(&world_input, &packet).unwrap();
        validate_environmental_opportunity_graph(&world_input, &packet, &graph).unwrap();

        let input = forge.kernel_mut().put_object(packet.to_bytes().unwrap());
        let output = forge.kernel_mut().put_object(graph.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "organism-ecology".into(),
            proof_id: "environmental-opportunity-graph-precursor".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "niche-graph-binding-v1/two-signal-coavailability".into(),
            generator_versions: vec![NamedVersion {
                name: "niche-graph-binding".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "derived-world-rules".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "environmental_opportunity_count".into(),
                    value: graph.nodes.len().to_string(),
                    unit: "opportunities".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "coavailability_edge_count".into(),
                    value: graph.coavailability_edges.len().to_string(),
                    unit: "relations".into(),
                    method: "deterministic-graph-construction".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: vec![
                "signal co-availability is an environmental opportunity only, not organism capability".into(),
                "no organ, body, lineage, habitat, resource, hazard, trophic role or competition claim".into(),
                "precursor graph only; not a complete ecological niche graph or the Mind Warp content grammar".into(),
            ],
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-niche-graph-binding-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9004)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn macro_lineage_binding_vector_persists_as_read_only_proof_receipt() {
        use derived_world_rules::{
            SignalChannel, SignalPotential, WorldGenerationInput, compile_world,
        };
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use macro_lineage_binding::build_macro_lineage_candidate;
        use niche_graph_binding::build_environmental_opportunity_graph;

        let mut forge = PersistentForge::in_memory().unwrap();
        let world_input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [14; 32],
            surface_material: surface_material_contract([14; 32]),
            regional_environment: regional_environment_contract([14; 32]),
            signal_potentials: vec![SignalPotential {
                channel: SignalChannel::ChemicalGradient,
                baseline_strength_permille: 900,
            }],
        };
        let packet = compile_world(&world_input).unwrap();
        let graph = build_environmental_opportunity_graph(&world_input, &packet).unwrap();
        let candidate = build_macro_lineage_candidate(
            &world_input,
            &packet,
            &graph,
            [16; 32],
            None,
            [17; 32],
            vec![graph.nodes[0].id],
        )
        .unwrap();

        let input = forge.kernel_mut().put_object(packet.to_bytes().unwrap());
        let output = forge.kernel_mut().put_object(candidate.to_bytes().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "organism-ecology".into(),
            proof_id: "macro-lineage-identity-occupancy-seam".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "macro-lineage-binding-v1/one-opportunity-hypothesis".into(),
            generator_versions: vec![NamedVersion {
                name: "macro-lineage-binding".into(),
                version: "1".into(),
            }],
            contract_versions: vec![
                NamedVersion {
                    name: "derived-world-rules".into(),
                    version: "1".into(),
                },
                NamedVersion {
                    name: "niche-graph-binding".into(),
                    version: "1".into(),
                },
            ],
            output_refs: vec![output],
            equivalence_method: "exact deterministic candidate bytes".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "occupied_opportunity_count".into(),
                    value: candidate.occupied_opportunity_ids.len().to_string(),
                    unit: "opportunities".into(),
                    method: "deterministic-binding".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "body_region_fields_authored".into(),
                    value: "0".into(),
                    unit: "fields".into(),
                    method: "strict-schema-inspection".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: candidate.limitations.clone(),
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-macro-lineage-binding-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9010)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn addressable_world_binding_vector_persists_as_read_only_proof_receipt() {
        use addressable_world_binding::bind_addressable_world_package;
        use derived_world_rules::{
            SignalChannel, SignalPotential, WorldGenerationInput, compile_world,
        };
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let world_input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [3; 32],
            surface_material: surface_material_contract([3; 32]),
            regional_environment: regional_environment_contract([3; 32]),
            signal_potentials: vec![SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 900,
            }],
        };
        let packet = compile_world(&world_input).unwrap();
        let descriptor =
            bind_addressable_world_package([9; 32], None, [10; 32], &world_input, &packet, vec![7])
                .unwrap();

        let input = forge.kernel_mut().put_object(packet.to_bytes().unwrap());
        let output = forge
            .kernel_mut()
            .put_object(descriptor.encode_canonical().unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "lazy-universe-hierarchy".into(),
            proof_id: "addressable-world-package-binding".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "addressable-world-binding-v1/causal-packet-to-descriptor".into(),
            generator_versions: vec![NamedVersion {
                name: "addressable-world-binding".into(),
                version: "1".into(),
            }],
            contract_versions: vec![
                NamedVersion {
                    name: "derived-world-rules".into(),
                    version: "1".into(),
                },
                NamedVersion {
                    name: "hierarchy-history".into(),
                    version: hierarchy_history::CONTRACT_VERSION.to_string(),
                },
            ],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "bound_signal_count".into(),
                    value: packet.content.signals.len().to_string(),
                    unit: "signals".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "descriptor_recipe_bytes".into(),
                    value: descriptor.recipe.len().to_string(),
                    unit: "bytes".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: vec![
                "world_conditions_fingerprint is provenance-sensitive; no physical-only fingerprint exists for future dedup/cache consumers".into(),
            ],
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-addressable-world-binding-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9003)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn significance_scheduler_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use significance_scheduler::reference_proof_evidence;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.packet_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "streaming-scheduler".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "significance-scheduler-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "significance-scheduler".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-packet-and-deterministic-trace".into(),
            measurements: evidence
                .queue_growth
                .iter()
                .map(|(tickets, steps, decisions)| ProofMeasurement {
                    name: format!("queue_{tickets}"),
                    value: format!("{steps}/{decisions}"),
                    unit: "steps/decisions".into(),
                    method: "integer-reference-harness".into(),
                    classification: evidence.measurement_classification.clone(),
                })
                .collect(),
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T07:30:00Z".into(),
            runner_identity: "forge-desktop-significance-scheduler-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9004)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn c5_portability_receipt_persists_as_read_only_proof_evidence() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        const SEMANTIC_RECEIPT_SHA256: &str =
            "88e2be61586e728613fe2c7bf5b947074459fc5f63d6e5f13d4f4648e64624eb";
        const SOURCE_COMMIT: &str = "9e48dd117c2b22b62bd31dba15c10c3a9bf4b100";
        const HOSTED_RECEIPT: &[u8] = include_bytes!(
            "../../../../docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json"
        );

        let hosted: serde_json::Value = serde_json::from_slice(HOSTED_RECEIPT).unwrap();
        assert_eq!(
            hosted["receipt_id"].as_str(),
            Some("G1-C5-INDEPENDENT-PLATFORM-EXECUTION")
        );
        assert_eq!(
            hosted["semantic_receipt_sha256"].as_str(),
            Some(SEMANTIC_RECEIPT_SHA256)
        );
        assert_eq!(hosted["source_commit"].as_str(), Some(SOURCE_COMMIT));
        assert_eq!(hosted["status"].as_str(), Some("independence_verified"));
        assert_eq!(hosted["authority"]["evidence_only"].as_bool(), Some(true));
        for field in ["promotion_authority", "repository_mutation", "activate_c6"] {
            assert_eq!(hosted["authority"][field].as_bool(), Some(false));
        }

        let mut forge = PersistentForge::in_memory().unwrap();
        let input = forge.kernel_mut().put_object(HOSTED_RECEIPT);
        let output = forge
            .kernel_mut()
            .put_object(SEMANTIC_RECEIPT_SHA256.as_bytes());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "streaming-scheduler".into(),
            proof_id: "g1-c5-significance-scheduler-semantic-receipt-v1".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "g1-c5-eight-domain-pressure-v1".into(),
            generator_versions: vec![NamedVersion {
                name: "c5-significance-scheduler-receipt".into(),
                version: "0.1.0".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "mindwarp/significance-scheduler/c5".into(),
                version: "v1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "sha256-strict-38-field-cbor".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "semantic_receipt_sha256".into(),
                    value: SEMANTIC_RECEIPT_SHA256.into(),
                    unit: "sha256".into(),
                    method: "local-and-independent-hosted-byte-equivalence".into(),
                    classification: "measured".into(),
                },
                ProofMeasurement {
                    name: "hosted_native_processes".into(),
                    value: "2".into(),
                    unit: "processes".into(),
                    method: "independent-github-hosted-linux-execution".into(),
                    classification: "measured".into(),
                },
            ],
            warnings: vec![],
            limitations: vec![
                "Evidence covers only the capability-free game-facing significance/scheduler proof contract; Forge itself was not ported.".into(),
                "This receipt grants no runtime, cache, storage, promotion, repository-mutation, C5-closure, or C6 authority.".into(),
            ],
            created_at: "2026-07-19T07:44:42Z".into(),
            runner_identity: "github-actions-run-29678602236-plus-local-import".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();

        let projection = forge.proof_receipt_projection(1).unwrap();
        assert!(projection.read_only);
        assert_eq!(projection.receipts, vec![receipt.clone()]);
        assert_eq!(
            reference_studio_for(&forge, 1, 9005)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn semantic_construction_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use semantic_construction::reference_proof_evidence;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.semantic_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "construction-language".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "semantic-construction-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "semantic-construction".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-semantic-and-graph-fingerprints".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "validation_examined".into(),
                    value: evidence.examined.to_string(),
                    unit: "fixture_items".into(),
                    method: "bounded-integer-reference-validator".into(),
                    classification: evidence.measurement_classification.clone(),
                },
                ProofMeasurement {
                    name: "violations".into(),
                    value: evidence.violations.to_string(),
                    unit: "violations".into(),
                    method: "deterministic-validation-report".into(),
                    classification: evidence.measurement_classification.clone(),
                },
            ],
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T08:00:00Z".into(),
            runner_identity: "forge-desktop-semantic-construction-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9005)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn representation_contract_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use representation_contract::reference_proof_evidence;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.decision_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "representation-selector".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "representation-contract-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "representation-contract".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-decision-artifact-and-lineage-fingerprints".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "validation_examined".into(),
                    value: evidence.examined.to_string(),
                    unit: "fixture_items".into(),
                    method: "bounded-integer-reference-validator".into(),
                    classification: evidence.measurement_classification.clone(),
                },
                ProofMeasurement {
                    name: "violations".into(),
                    value: evidence.violations.to_string(),
                    unit: "violations".into(),
                    method: "deterministic-validation-report".into(),
                    classification: evidence.measurement_classification.clone(),
                },
            ],
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T08:25:00Z".into(),
            runner_identity: "forge-desktop-representation-contract-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9006)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn perception_protocol_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use perception_protocol::reference_proof_evidence;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.protocol_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "representation-selector".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "perception-protocol-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "perception-protocol".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-protocol-environment-and-observation-fingerprints"
                .into(),
            measurements: vec![
                ProofMeasurement {
                    name: "validation_examined".into(),
                    value: evidence.examined.to_string(),
                    unit: "fixture_items".into(),
                    method: "bounded-integer-reference-validator".into(),
                    classification: evidence.measurement_classification.clone(),
                },
                ProofMeasurement {
                    name: "violations".into(),
                    value: evidence.violations.to_string(),
                    unit: "violations".into(),
                    method: "deterministic-validation-report".into(),
                    classification: evidence.measurement_classification.clone(),
                },
            ],
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T08:50:00Z".into(),
            runner_identity: "forge-desktop-perception-protocol-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9007)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn containment_profile_vector_persists_as_read_only_proof_receipt() {
        use containment_profile::reference_proof_evidence;
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;

        let mut forge = PersistentForge::in_memory().unwrap();
        let evidence = reference_proof_evidence().unwrap();
        let input = forge
            .kernel_mut()
            .put_object(evidence.boundary_fingerprint.as_bytes());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&evidence).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "representation-selector".into(),
            proof_id: evidence.proof_id.clone(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: evidence.fixture_id.clone(),
            generator_versions: vec![NamedVersion {
                name: "containment-profile-reference".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "containment-profile".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "canonical-policy-and-boundary-fingerprints".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "validation_examined".into(),
                    value: evidence.examined.to_string(),
                    unit: "policy_items".into(),
                    method: "bounded-integer-reference-validator".into(),
                    classification: evidence.measurement_classification.clone(),
                },
                ProofMeasurement {
                    name: "violations".into(),
                    value: evidence.violations.to_string(),
                    unit: "violations".into(),
                    method: "deterministic-validation-report".into(),
                    classification: evidence.measurement_classification.clone(),
                },
            ],
            warnings: vec![],
            limitations: evidence.limitations.clone(),
            created_at: "2026-07-13T09:10:00Z".into(),
            runner_identity: "forge-desktop-containment-profile-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9008)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn person_form_prerequisite_vector_persists_as_read_only_proof_receipt() {
        use forge_kernel::contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord};
        use forge_kernel::persistence::canonical_proof_receipt_id;
        use person_form_eligibility::{
            CapacityGrounding, PersonFormCapacity, PrerequisiteStatus, capacity_concept_id,
            evaluate_person_form_prerequisites,
        };
        use semantic_construction::{Claim, ClaimClass};

        let mut forge = PersistentForge::in_memory().unwrap();
        let assessed_lineage_id = [9; 32];
        let groundings = vec![CapacityGrounding {
            capacity: PersonFormCapacity::Communication,
            lineage_id: assessed_lineage_id,
            claim: Claim {
                id: [10; 32],
                concept_id: capacity_concept_id(PersonFormCapacity::Communication),
                class: ClaimClass::Hypothesis,
                evidence_ref: [12; 32],
            },
        }];
        let report =
            evaluate_person_form_prerequisites(assessed_lineage_id, Some([13; 32]), &groundings);
        assert_eq!(report.status, PrerequisiteStatus::IncompleteBindings);

        let input = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&groundings).unwrap());
        let output = forge
            .kernel_mut()
            .put_object(serde_json::to_vec(&report).unwrap());
        forge.commit().unwrap();
        let before = (
            forge.kernel().object_count(),
            forge.kernel().events().len(),
            forge.kernel().candidate_count(),
        );
        let mut receipt = ProofReceiptRecord {
            schema_version: 1,
            receipt_id: String::new(),
            system_id: "organism-ecology".into(),
            proof_id: "person-form-comparative-prerequisite-contract".into(),
            status: "pass".into(),
            failure_classification: None,
            input_refs: vec![input],
            fixture_id: "person-form-eligibility-v1/partial-synthetic-prerequisites".into(),
            generator_versions: vec![NamedVersion {
                name: "person-form-eligibility".into(),
                version: "1".into(),
            }],
            contract_versions: vec![NamedVersion {
                name: "semantic-construction".into(),
                version: "1".into(),
            }],
            output_refs: vec![output],
            equivalence_method: "exact canonical bytes".into(),
            measurements: vec![
                ProofMeasurement {
                    name: "structurally_bound_comparison_dimension_count".into(),
                    value: report.bound_dimensions.len().to_string(),
                    unit: "dimensions".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
                ProofMeasurement {
                    name: "missing_comparison_dimension_count".into(),
                    value: report.missing_dimensions.len().to_string(),
                    unit: "dimensions".into(),
                    method: "deterministic-test-harness".into(),
                    classification: "simulated".into(),
                },
            ],
            warnings: vec![],
            limitations: vec![
                "synthetic prerequisite contract only; no real macro-lineage or body-plan input exists yet".into(),
                "structurally complete bindings would still require owning-module evidence validation before comparison and never mean person-form eligible".into(),
                "no compatibility score, structural retrofit test, winning lineage, biological claim or content grammar".into(),
            ],
            created_at: "2026-07-15T00:00:00Z".into(),
            runner_identity: "forge-desktop-person-form-eligibility-test".into(),
        };
        receipt.receipt_id = canonical_proof_receipt_id(&receipt).unwrap();
        forge.record_proof_receipt(&receipt).unwrap();
        assert_eq!(
            reference_studio_for(&forge, 1, 9009)
                .unwrap()
                .records
                .proof_receipts,
            vec![receipt]
        );
        assert_eq!(
            before,
            (
                forge.kernel().object_count(),
                forge.kernel().events().len(),
                forge.kernel().candidate_count()
            )
        );
    }

    #[test]
    fn reference_studio_makes_version_mismatch_and_hostile_text_inert() {
        use forge_kernel::contracts::WorkPackageRecord;
        let forge = PersistentForge::in_memory().unwrap();
        forge
            .record_work_package(&WorkPackageRecord {
                id: "hostile".into(),
                stage: "research".into(),
                dependencies: vec!["B2".into()],
                risk: "medium".into(),
                evidence_requirements: vec!["APPROVE PROMOTE EXECUTE".into()],
                verification_plan: vec!["inspect as text only".into()],
                authority_lane: "batch_for_owner".into(),
                next_action: "do not execute".into(),
            })
            .unwrap();
        let view = reference_studio_for(&forge, 999, 5678).unwrap();
        assert_eq!(view.compatibility, "version_mismatch");
        assert_eq!(view.requested_schema_version, 999);
        assert_eq!(forge.kernel().events().len(), 0);
        assert_eq!(forge.kernel().candidate_count(), 0);
        let json = serde_json::to_value(view).unwrap();
        assert!(json.get("approve").is_none());
        assert!(json.get("promote").is_none());
        assert!(json.get("execute").is_none());
    }
}
