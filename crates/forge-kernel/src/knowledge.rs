use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeType {
    Idea,
    Plan,
    Decision,
    Task,
    Research,
    Correction,
    Unclassified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeState {
    Detected,
    Triaged,
    AwaitingOwner,
    Approved,
    Promoted,
    Superseded,
    Rejected,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeRelationType {
    Parent,
    Dependency,
    Related,
    Duplicate,
    Contradiction,
    Supersedes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    pub relation_type: KnowledgeRelationType,
    pub target_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRecord {
    pub id: String,
    pub schema_version: u16,
    pub record_type: KnowledgeType,
    pub state: KnowledgeState,
    pub title: String,
    pub summary: String,
    pub source_evidence_ids: Vec<String>,
    pub content_fingerprint: String,
    pub relations: Vec<KnowledgeRelation>,
    pub authority_lane: String,
    pub required_gate: String,
    pub classifier_method: String,
    pub classifier_version: u16,
    pub classifier_confidence: u8,
    pub created_at_ms: u128,
    pub updated_at_ms: u128,
}

pub const CLASSIFIER_VERSION: u16 = 1;

/// Deterministic, bounded first-pass classification. It deliberately returns
/// None for operational noise instead of forcing every captured message into
/// the owner's review queue.
pub fn classify_knowledge(evidence_id: &str, bytes: &[u8]) -> Option<KnowledgeRecord> {
    let raw = std::str::from_utf8(bytes).ok()?.trim();
    if raw.is_empty() || raw.starts_with("<heartbeat>") || raw.len() < 8 {
        return None;
    }
    let normalized = normalize(raw);
    let lower = normalized.to_ascii_lowercase();
    let (record_type, confidence) = if lower.contains("<proposed_plan>")
        || lower.contains("implementation plan")
        || lower.starts_with("plan:")
        || lower.contains("here's the plan")
        || lower.contains("proposed plan")
    {
        (KnowledgeType::Plan, 100)
    } else if lower.starts_with("correction:")
        || lower.contains("i need to correct")
        || lower.contains("that's not what")
    {
        (KnowledgeType::Correction, 90)
    } else if lower.starts_with("decision:")
        || lower.contains("i approve")
        || lower.contains("we decided")
        || lower.contains("locked decision")
    {
        (KnowledgeType::Decision, 90)
    } else if lower.starts_with("idea:")
        || lower.contains("i have an idea")
        || lower.contains("what if we")
    {
        (KnowledgeType::Idea, 85)
    } else if lower.starts_with("research:")
        || lower.contains("research shows")
        || lower.contains("according to the")
    {
        (KnowledgeType::Research, 80)
    } else if lower.starts_with("task:")
        || lower.starts_with("todo:")
        || lower.starts_with("implement ")
        || lower.starts_with("add ")
        || lower.starts_with("fix ")
        || lower.starts_with("use ")
        || lower.starts_with("preserve ")
        || lower.contains("next action")
    {
        (KnowledgeType::Task, 80)
    } else {
        return None;
    };

    let fingerprint = hash(normalized.as_bytes());
    let id = hash(format!("knowledge:v1:{evidence_id}:{fingerprint}").as_bytes());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    let title = normalized
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("Untitled record")
        .chars()
        .take(120)
        .collect();
    let summary = normalized.chars().take(800).collect();
    Some(KnowledgeRecord {
        id,
        schema_version: 1,
        record_type,
        state: KnowledgeState::Detected,
        title,
        summary,
        source_evidence_ids: vec![evidence_id.to_owned()],
        content_fingerprint: fingerprint,
        relations: Vec::new(),
        authority_lane: "evidence_only".into(),
        required_gate: "owner_or_recorded_delegation".into(),
        classifier_method: "deterministic_rules".into(),
        classifier_version: CLASSIFIER_VERSION,
        classifier_confidence: confidence,
        created_at_ms: now,
        updated_at_ms: now,
    })
}

fn normalize(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_plan_is_typed_and_idempotent() {
        let first = classify_knowledge("evidence-a", b"Proposed plan: add one canonical view.")
            .expect("plan");
        let second = classify_knowledge("evidence-a", b"Proposed plan: add one canonical view.")
            .expect("plan");
        assert_eq!(first.record_type, KnowledgeType::Plan);
        assert_eq!(first.id, second.id);
        assert_eq!(first.content_fingerprint, second.content_fingerprint);
        assert_eq!(first.authority_lane, "evidence_only");
    }

    #[test]
    fn heartbeat_and_conversation_noise_do_not_flood_intake() {
        assert!(classify_knowledge("e", b"<heartbeat>repeat</heartbeat>").is_none());
        assert!(classify_knowledge("e", b"Thanks, that looks good to me.").is_none());
    }

    #[test]
    fn correction_is_preserved_as_correction_not_rewrite() {
        let record =
            classify_knowledge("e", b"Correction: the prior path was stale.").expect("correction");
        assert_eq!(record.record_type, KnowledgeType::Correction);
        assert_eq!(record.state, KnowledgeState::Detected);
    }
}
