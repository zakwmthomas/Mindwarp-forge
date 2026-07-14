use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ActorKind;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeType {
    Idea,
    Plan,
    Decision,
    Task,
    Research,
    Correction,
    Philosophy,
    Requirement,
    Constraint,
    Preference,
    Risk,
    Question,
    Observation,
    Context,
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
    #[serde(default = "unknown_source_actor")]
    pub source_actor: String,
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

pub const CLASSIFIER_VERSION: u16 = 2;

fn unknown_source_actor() -> String {
    "legacy_unknown".into()
}

/// Deterministic, bounded first-pass material classification.
///
/// One captured message may express several durable concerns, so v2 emits one
/// evidence-linked facet per detected type. Every non-noise message receives
/// at least a `Context` facet. This makes intake searchable without pretending
/// that deterministic phrase rules understand or promote canonical truth.
pub fn classify_knowledge(
    evidence_id: &str,
    actor: &ActorKind,
    bytes: &[u8],
) -> Vec<KnowledgeRecord> {
    let Some(raw) = std::str::from_utf8(bytes).ok().map(str::trim) else {
        return Vec::new();
    };
    if raw.is_empty() || raw.len() < 8 {
        return Vec::new();
    }
    let normalized = normalize(raw);
    let lower = normalized.to_ascii_lowercase();
    if is_operational_noise(&lower) {
        return Vec::new();
    }

    let mut facets = Vec::new();
    detect(
        &mut facets,
        KnowledgeType::Plan,
        95,
        contains_any(
            &lower,
            &[
                "<proposed_plan>",
                "implementation plan",
                "plan:",
                "here's the plan",
                "proposed plan",
                "master plan",
                "roadmap",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Correction,
        92,
        contains_any(
            &lower,
            &[
                "correction:",
                "i need to correct",
                "that's not what",
                "to clarify",
                "what i meant",
                "one distinction",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Decision,
        88,
        contains_any(
            &lower,
            &[
                "decision:",
                "i approve",
                "we decided",
                "locked decision",
                "i've decided",
                "we're going to",
                "we are going to",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Idea,
        82,
        contains_any(
            &lower,
            &[
                "idea:",
                "i have an idea",
                "what if we",
                "my idea is",
                "i was thinking",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Research,
        80,
        contains_any(
            &lower,
            &[
                "research:",
                "research shows",
                "according to the",
                "source says",
                "documentation says",
                "we found that",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Philosophy,
        86,
        contains_any(
            &lower,
            &[
                "philosophy",
                "philosophies",
                "north star",
                "principle",
                "important to me",
                "i care about",
                "i value",
                "our way of",
                "operating rule",
                "efficiency is",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Requirement,
        82,
        contains_any(
            &lower,
            &[
                "i need",
                "we need",
                "need to",
                "must ",
                "should ",
                "has to",
                "have to",
                "make sure",
                "i want",
                "we want",
                "require",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Constraint,
        82,
        contains_any(
            &lower,
            &[
                "must not",
                "cannot",
                "can't ",
                "do not ",
                "don't ",
                "only ",
                "without ",
                "low poly",
                "on a phone",
                "phone-class",
                "budget",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Preference,
        80,
        contains_any(
            &lower,
            &[
                "i prefer",
                "we prefer",
                "i'd rather",
                "i would rather",
                "i like",
                "i don't like",
                "i kind of want",
                "i was hoping",
                "style",
                "stylized",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Risk,
        88,
        contains_any(
            &lower,
            &[
                "i'm worried",
                "i am worried",
                "worried",
                "concerned",
                "risk",
                "failure",
                "mess this up",
                "start again",
                "cost ineffective",
                "too expensive",
                "burned through",
                "forget your context",
                "context gets lost",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Question,
        76,
        lower.contains('?')
            || starts_with_any(
                &lower,
                &[
                    "are we ", "is that ", "what ", "why ", "how ", "do we ", "can we ",
                ],
            ),
    );
    detect(
        &mut facets,
        KnowledgeType::Observation,
        72,
        contains_any(
            &lower,
            &[
                "i noticed",
                "we observed",
                "it seems",
                "it looks",
                "currently",
                "right now",
                "the result",
                "passed",
                "failed",
            ],
        ),
    );
    detect(
        &mut facets,
        KnowledgeType::Task,
        80,
        starts_with_any(
            &lower,
            &[
                "task:",
                "todo:",
                "implement ",
                "add ",
                "fix ",
                "use ",
                "preserve ",
                "continue ",
            ],
        ) || contains_any(
            &lower,
            &["next action", "let's ", "we should ", "make sure"],
        ),
    );

    if facets.is_empty() {
        facets.push((KnowledgeType::Context, 60));
    }
    let fingerprint = hash(normalized.as_bytes());
    let title: String = normalized
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("Untitled record")
        .chars()
        .take(120)
        .collect();
    let summary: String = normalized.chars().take(1200).collect();
    let source_actor = actor_name(actor).to_owned();
    facets
        .into_iter()
        .map(|(record_type, confidence)| {
            let type_name = serde_json::to_string(&record_type).unwrap_or_default();
            let id =
                hash(format!("knowledge:v2:{evidence_id}:{fingerprint}:{type_name}").as_bytes());
            KnowledgeRecord {
                id,
                schema_version: 2,
                record_type,
                state: KnowledgeState::Detected,
                title: title.clone(),
                summary: summary.clone(),
                source_evidence_ids: vec![evidence_id.to_owned()],
                source_actor: source_actor.clone(),
                content_fingerprint: fingerprint.clone(),
                relations: Vec::new(),
                authority_lane: "evidence_only".into(),
                required_gate: "canonical_triage_or_recorded_delegation".into(),
                classifier_method: "deterministic_multi_facet_rules".into(),
                classifier_version: CLASSIFIER_VERSION,
                classifier_confidence: confidence,
                created_at_ms: 0,
                updated_at_ms: 0,
            }
        })
        .collect()
}

fn detect(
    facets: &mut Vec<(KnowledgeType, u8)>,
    record_type: KnowledgeType,
    confidence: u8,
    detected: bool,
) {
    if detected {
        facets.push((record_type, confidence));
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn starts_with_any(value: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| value.starts_with(prefix))
}

fn is_operational_noise(lower: &str) -> bool {
    lower.starts_with("<heartbeat>")
        || lower.starts_with("bootstrap receipt")
        || lower.starts_with("**bootstrap receipt**")
        || matches!(
            lower,
            "thanks"
                | "thank you"
                | "awesome"
                | "okay"
                | "ok"
                | "sounds good"
                | "that looks good"
                | "thanks, that looks good to me."
        )
}

fn actor_name(actor: &ActorKind) -> &'static str {
    match actor {
        ActorKind::DirectProjectUser => "direct_project_user",
        ActorKind::Assistant => "assistant",
        ActorKind::System => "system",
        ActorKind::ImportedContent => "captured_user",
        ActorKind::ExternalTool => "external_tool",
    }
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

    fn types(value: &str) -> Vec<KnowledgeType> {
        classify_knowledge("evidence-a", &ActorKind::ImportedContent, value.as_bytes())
            .into_iter()
            .map(|record| record.record_type)
            .collect()
    }

    #[test]
    fn explicit_plan_is_typed_and_idempotent() {
        let first = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"Proposed plan: add one canonical view.",
        );
        let second = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"Proposed plan: add one canonical view.",
        );
        assert!(
            first
                .iter()
                .any(|record| record.record_type == KnowledgeType::Plan)
        );
        assert_eq!(first, second);
        assert!(
            first
                .iter()
                .all(|record| record.authority_lane == "evidence_only")
        );
    }

    #[test]
    fn ordinary_owner_language_yields_multiple_material_facets() {
        let found = types(
            "Efficiency is really important to me. We need to test cheaply because I am concerned about wasting the weekly allowance.",
        );
        assert!(found.contains(&KnowledgeType::Philosophy));
        assert!(found.contains(&KnowledgeType::Requirement));
        assert!(found.contains(&KnowledgeType::Risk));
    }

    #[test]
    fn continuity_request_retains_philosophy_requirement_risk_and_task() {
        let found = types(
            "Make sure our philosophies are categorized and stored. I get worried that you forget your context, so we should verify the system before continuing.",
        );
        assert!(found.contains(&KnowledgeType::Philosophy));
        assert!(found.contains(&KnowledgeType::Requirement));
        assert!(found.contains(&KnowledgeType::Risk));
        assert!(found.contains(&KnowledgeType::Task));
    }

    #[test]
    fn style_and_phone_language_retain_preference_and_constraint() {
        let found =
            types("I kind of want mature stylized rendering, but it has to run on a phone.");
        assert!(found.contains(&KnowledgeType::Preference));
        assert!(found.contains(&KnowledgeType::Constraint));
        assert!(found.contains(&KnowledgeType::Requirement));
    }

    #[test]
    fn distinction_and_question_are_not_lost() {
        let found =
            types("One distinction: old age must not cause death. Is that correctly stored?");
        assert!(found.contains(&KnowledgeType::Correction));
        assert!(found.contains(&KnowledgeType::Constraint));
        assert!(found.contains(&KnowledgeType::Question));
    }

    #[test]
    fn meaningful_fallback_is_context_and_actor_is_retained() {
        let records = classify_knowledge(
            "e",
            &ActorKind::Assistant,
            b"The small blue component sits beside the larger green component.",
        );
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].record_type, KnowledgeType::Context);
        assert_eq!(records[0].source_actor, "assistant");
    }

    #[test]
    fn acknowledgements_and_operational_receipts_do_not_flood_intake() {
        assert!(classify_knowledge("e", &ActorKind::ImportedContent, b"Awesome").is_empty());
        assert!(
            classify_knowledge("e", &ActorKind::Assistant, b"BOOTSTRAP RECEIPT\npassed").is_empty()
        );
    }
}
