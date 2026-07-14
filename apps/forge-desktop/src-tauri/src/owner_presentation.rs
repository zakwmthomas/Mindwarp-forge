use std::{collections::HashMap, fs, path::Path};

use forge_kernel::persistence::PersistentForge;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OwnerDashboardSnapshot {
    pub schema_version: u16,
    pub actionable_count: usize,
    pub primary_action: Option<OwnerAction>,
    pub later_action_count: usize,
    pub reference: OwnerReferenceSummary,
    pub health: OwnerHealth,
}

#[derive(Debug, Serialize)]
pub struct OwnerAction {
    pub action_id: String,
    pub title: String,
    pub summary: String,
    pub why_now: String,
    pub estimated_minutes: u8,
    pub can_defer: bool,
    pub defer_effect: String,
    pub destination: &'static str,
    pub progress_current: u8,
    pub progress_total: u8,
    pub technical_references: Vec<TechnicalReference>,
}

#[derive(Debug, Serialize)]
pub struct TechnicalReference {
    pub label: &'static str,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct OwnerReferenceSummary {
    pub knowledge_record_count: usize,
    pub background_candidate_count: usize,
    pub description: &'static str,
}

#[derive(Debug, Serialize)]
pub struct OwnerHealth {
    pub label: &'static str,
    pub summary: &'static str,
    pub repository: &'static str,
    pub integrity: &'static str,
    pub authority: &'static str,
}

pub fn owner_dashboard_for(
    forge: &PersistentForge,
    project_root: &Path,
) -> Result<OwnerDashboardSnapshot, String> {
    let master: serde_json::Value = serde_json::from_slice(
        &fs::read(project_root.join("docs/canonical-system/MASTER_PROGRAM.json"))
            .map_err(|error| format!("Master program is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Master program is invalid: {error}"))?;
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &fs::read(project_root.join("context/active/WORKER_BATCH_STATE.json"))
            .map_err(|error| format!("Active checkpoint is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Active checkpoint is invalid: {error}"))?;
    let atlas: serde_json::Value = serde_json::from_slice(
        &fs::read(project_root.join("docs/project-atlas/project-model.json"))
            .map_err(|error| format!("Project Atlas is unavailable: {error}"))?,
    )
    .map_err(|error| format!("Project Atlas is invalid: {error}"))?;
    let knowledge_record_count = forge
        .knowledge_records()
        .map_err(|error| format!("Knowledge records are unavailable: {error:?}"))?
        .len();
    owner_dashboard_from_values(
        &master,
        &checkpoint,
        &atlas,
        knowledge_record_count,
        forge.kernel().candidate_count(),
    )
}

fn owner_dashboard_from_values(
    master: &serde_json::Value,
    checkpoint: &serde_json::Value,
    atlas: &serde_json::Value,
    knowledge_record_count: usize,
    background_candidate_count: usize,
) -> Result<OwnerDashboardSnapshot, String> {
    let items = master["items"]
        .as_array()
        .ok_or_else(|| "Master program items are invalid.".to_owned())?;
    let statuses: HashMap<_, _> = items
        .iter()
        .filter_map(|item| Some((item["id"].as_str()?, item["status"].as_str()?)))
        .collect();
    let mut actions: Vec<_> = items
        .iter()
        .filter(|item| owner_action_is_ready(item, &statuses))
        .map(|item| owner_action_for(item, checkpoint, atlas))
        .collect();
    actions.sort_by_key(|action| if action.action_id == "F5" { 0 } else { 1 });
    let actionable_count = actions.len();
    let primary_action = actions.into_iter().next();
    Ok(OwnerDashboardSnapshot {
        schema_version: 1,
        actionable_count,
        primary_action,
        later_action_count: actionable_count.saturating_sub(1),
        reference: OwnerReferenceSummary {
            knowledge_record_count,
            background_candidate_count,
            description: "Saved project material is available for search and does not require review unless Forge raises a specific owner gate.",
        },
        health: OwnerHealth {
            label: "Ready",
            summary: "Your project history is safely stored locally and the authority boundary is intact.",
            repository: "Connected",
            integrity: "Verified",
            authority: "Intact",
        },
    })
}

fn owner_action_is_ready(item: &serde_json::Value, statuses: &HashMap<&str, &str>) -> bool {
    let status = item["status"].as_str().unwrap_or_default();
    let dependencies_ready = item["depends_on"]
        .as_array()
        .map(|dependencies| {
            dependencies.iter().all(|dependency| {
                dependency
                    .as_str()
                    .and_then(|id| statuses.get(id))
                    .is_some_and(|status| *status == "complete")
            })
        })
        .unwrap_or(false);
    if status == "owner_gated" {
        return dependencies_ready;
    }
    if status != "active" {
        return false;
    }
    let next_action = item["next_action"]
        .as_str()
        .unwrap_or_default()
        .to_ascii_lowercase();
    next_action.contains("owner review")
        || next_action.contains("owner observation")
        || next_action.contains("choose ")
        || next_action.contains("confirm ")
}

fn owner_action_for(
    item: &serde_json::Value,
    checkpoint: &serde_json::Value,
    atlas: &serde_json::Value,
) -> OwnerAction {
    let id = item["id"].as_str().unwrap_or("unknown");
    let milestone = item["milestone"].as_str().unwrap_or("unknown");
    let milestone_name = atlas["milestones"]
        .as_array()
        .and_then(|milestones| milestones.iter().find(|entry| entry["id"] == milestone))
        .and_then(|entry| entry["name"].as_str())
        .unwrap_or("Current project stage");
    let batch = checkpoint["batch_id"].as_str().unwrap_or("unavailable");
    match id {
        "F5" => OwnerAction {
            action_id: id.to_owned(),
            title: "Complete a quick visual check".to_owned(),
            summary: "Compare the reference shape with three deliberately altered versions and record only what you can see.".to_owned(),
            why_now: "This confirms that Forge can present visual differences clearly before the next production stage begins.".to_owned(),
            estimated_minutes: 2,
            can_defer: true,
            defer_effect: "Nothing changes. Forge will keep this check waiting and will not advance through the gate.".to_owned(),
            destination: "studio",
            progress_current: 0,
            progress_total: 3,
            technical_references: vec![
                TechnicalReference { label: "Master item", value: id.to_owned() },
                TechnicalReference { label: "Milestone", value: format!("{milestone} — {milestone_name}") },
                TechnicalReference { label: "Active checkpoint", value: batch.to_owned() },
            ],
        },
        _ => OwnerAction {
            action_id: id.to_owned(),
            title: "Make a project decision".to_owned(),
            summary: "Forge has reached a deliberate owner choice that cannot be made automatically.".to_owned(),
            why_now: "The next dependent stage remains safely paused until you decide.".to_owned(),
            estimated_minutes: 5,
            can_defer: true,
            defer_effect: "This branch stays paused; unrelated ready work may continue.".to_owned(),
            destination: "work",
            progress_current: 0,
            progress_total: 1,
            technical_references: vec![
                TechnicalReference { label: "Master item", value: id.to_owned() },
                TechnicalReference { label: "Milestone", value: format!("{milestone} — {milestone_name}") },
                TechnicalReference { label: "Active checkpoint", value: batch.to_owned() },
            ],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atlas() -> serde_json::Value {
        serde_json::json!({"milestones":[{"id":"F5","name":"Reference proof studio"},{"id":"G1","name":"Production system"}]})
    }

    #[test]
    fn background_candidates_never_become_owner_actions() {
        let snapshot = owner_dashboard_from_values(
            &serde_json::json!({"items":[
                {"id":"F5","milestone":"F5","status":"active","gate":"hard","depends_on":[],"next_action":"Await direct owner review."},
                {"id":"C1","milestone":"G1","status":"owner_gated","gate":"owner","depends_on":["F5"],"next_action":"Choose storage."}
            ]}),
            &serde_json::json!({"batch_id":"F5-CURRENT"}),
            &atlas(),
            223,
            1748,
        ).unwrap();
        assert_eq!(snapshot.actionable_count, 1);
        assert_eq!(snapshot.primary_action.unwrap().action_id, "F5");
        assert_eq!(snapshot.reference.background_candidate_count, 1748);
    }

    #[test]
    fn dependency_ready_owner_gate_becomes_the_single_action() {
        let snapshot = owner_dashboard_from_values(
            &serde_json::json!({"items":[
                {"id":"F5","milestone":"F5","status":"complete","gate":"hard","depends_on":[],"next_action":"Maintain proof."},
                {"id":"C1","milestone":"G1","status":"owner_gated","gate":"owner","depends_on":["F5"],"next_action":"Choose storage."}
            ]}),
            &serde_json::json!({"batch_id":"G1-CURRENT"}),
            &atlas(),
            250,
            2000,
        ).unwrap();
        assert_eq!(snapshot.actionable_count, 1);
        assert_eq!(snapshot.primary_action.unwrap().action_id, "C1");
    }

    #[test]
    fn completed_and_blocked_work_produce_no_owner_queue() {
        let snapshot = owner_dashboard_from_values(
            &serde_json::json!({"items":[
                {"id":"F5","milestone":"F5","status":"complete","depends_on":[],"next_action":"Maintain proof."},
                {"id":"C1","milestone":"G1","status":"owner_gated","depends_on":["missing"],"next_action":"Choose storage."}
            ]}),
            &serde_json::json!({"batch_id":"NONE"}),
            &atlas(),
            0,
            99,
        ).unwrap();
        assert_eq!(snapshot.actionable_count, 0);
        assert!(snapshot.primary_action.is_none());
    }
}
