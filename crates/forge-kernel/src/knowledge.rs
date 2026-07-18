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
pub struct KnowledgeSourceSpan {
    pub evidence_id: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRecord {
    pub id: String,
    pub schema_version: u16,
    pub record_type: KnowledgeType,
    #[serde(default)]
    pub facet_types: Vec<KnowledgeType>,
    #[serde(default)]
    pub system_refs: Vec<String>,
    /// Explicit federation scope. Empty means not yet routed, never "Forge" by default.
    #[serde(default)]
    pub project_refs: Vec<String>,
    #[serde(default)]
    pub workstream_refs: Vec<String>,
    #[serde(default)]
    pub entity_refs: Vec<String>,
    #[serde(default)]
    pub source_session_id: Option<String>,
    #[serde(default)]
    pub source_spans: Vec<KnowledgeSourceSpan>,
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

pub const CLASSIFIER_VERSION: u16 = 4;

fn unknown_source_actor() -> String {
    "legacy_unknown".into()
}

/// Deterministic, bounded first-pass material classification.
///
/// v3 stores each bounded source span once. Role facets and Atlas-system scopes
/// are references on that record, not duplicate copies of its text. Every
/// non-noise span receives at least `Context`; uncertain classification remains
/// searchable without pretending deterministic rules understand canonical truth.
pub fn classify_knowledge(
    evidence_id: &str,
    actor: &ActorKind,
    bytes: &[u8],
) -> Vec<KnowledgeRecord> {
    classify_knowledge_scoped(evidence_id, actor, bytes, None, &[], &[])
}

/// Classify evidence while retaining its explicit project/workstream route.
/// Scope is supplied by the caller's routing receipt; text keywords cannot
/// silently assign a project.
pub fn classify_knowledge_scoped(
    evidence_id: &str,
    actor: &ActorKind,
    bytes: &[u8],
    source_session_id: Option<&str>,
    project_refs: &[String],
    workstream_refs: &[String],
) -> Vec<KnowledgeRecord> {
    let Some(raw) = std::str::from_utf8(bytes).ok().map(str::trim) else {
        return Vec::new();
    };
    if raw.is_empty() || raw.len() < 8 {
        return Vec::new();
    }
    split_source_spans(raw)
        .into_iter()
        .filter_map(|(start_byte, end_byte)| {
            classify_span(
                evidence_id,
                actor,
                raw,
                start_byte,
                end_byte,
                source_session_id,
                project_refs,
                workstream_refs,
            )
        })
        .collect()
}

fn classify_span(
    evidence_id: &str,
    actor: &ActorKind,
    raw: &str,
    start_byte: usize,
    end_byte: usize,
    source_session_id: Option<&str>,
    project_refs: &[String],
    workstream_refs: &[String],
) -> Option<KnowledgeRecord> {
    let normalized = normalize(&raw[start_byte..end_byte]);
    if normalized.len() < 8 {
        return None;
    }
    let lower = normalized.to_ascii_lowercase();
    if is_operational_noise(&lower) {
        return None;
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
        84,
        contains_any(
            &lower,
            &[
                "north star",
                "my philosophy is",
                "our philosophy is",
                "the philosophy is",
                "as a principle",
                "guiding principle",
                "i value that",
                "we value that",
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
                "i'm seeing",
                "i am seeing",
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
    facets.sort_by(|left, right| left.0.cmp(&right.0));
    facets.dedup_by(|left, right| left.0 == right.0);
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
    let facet_types = facets
        .iter()
        .map(|(kind, _)| kind.clone())
        .collect::<Vec<_>>();
    let record_type = facet_types
        .first()
        .cloned()
        .unwrap_or(KnowledgeType::Context);
    let confidence = facets.iter().map(|(_, value)| *value).max().unwrap_or(60);
    Some(KnowledgeRecord {
        id: hash(
            format!("knowledge:v4:{evidence_id}:{start_byte}:{end_byte}:{fingerprint}").as_bytes(),
        ),
        schema_version: 4,
        record_type,
        facet_types,
        system_refs: classify_system_refs(&lower),
        project_refs: project_refs.to_vec(),
        workstream_refs: workstream_refs.to_vec(),
        entity_refs: Vec::new(),
        source_session_id: source_session_id.map(str::to_owned),
        source_spans: vec![KnowledgeSourceSpan {
            evidence_id: evidence_id.to_owned(),
            start_byte,
            end_byte,
        }],
        state: KnowledgeState::Detected,
        title,
        summary,
        source_evidence_ids: vec![evidence_id.to_owned()],
        source_actor,
        content_fingerprint: fingerprint,
        relations: Vec::new(),
        authority_lane: "evidence_only".into(),
        required_gate: "canonical_triage_or_recorded_delegation".into(),
        classifier_method: "deterministic_atomic_multi_reference_rules".into(),
        classifier_version: CLASSIFIER_VERSION,
        classifier_confidence: confidence,
        created_at_ms: 0,
        updated_at_ms: 0,
    })
}

fn split_source_spans(raw: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = 0;
    for (index, character) in raw.char_indices() {
        if !matches!(character, '.' | '?' | '!' | '\n') {
            continue;
        }
        let end = index + character.len_utf8();
        if character == '.' {
            let previous = raw[..index].chars().next_back();
            let next = raw[end..].chars().next();
            if previous.is_some_and(|value| value.is_ascii_digit())
                && next.is_some_and(|value| value.is_ascii_digit())
            {
                continue;
            }
            if next.is_some_and(|value| !value.is_whitespace()) {
                continue;
            }
        }
        push_trimmed_span(raw, start, end, &mut spans);
        start = end;
    }
    push_trimmed_span(raw, start, raw.len(), &mut spans);
    spans
}

fn push_trimmed_span(raw: &str, mut start: usize, mut end: usize, spans: &mut Vec<(usize, usize)>) {
    while start < end && raw[start..].chars().next().is_some_and(char::is_whitespace) {
        start += raw[start..].chars().next().unwrap().len_utf8();
    }
    while start < end
        && raw[..end]
            .chars()
            .next_back()
            .is_some_and(char::is_whitespace)
    {
        end -= raw[..end].chars().next_back().unwrap().len_utf8();
    }
    if end.saturating_sub(start) >= 8 {
        spans.push((start, end));
    }
}

fn classify_system_refs(lower: &str) -> Vec<String> {
    let mut refs = Vec::new();
    add_ref(
        &mut refs,
        "forge-kernel",
        contains_any(
            lower,
            &["forge", "stored", "storage", "knowledge", "record"],
        ),
    );
    add_ref(
        &mut refs,
        "conversation-capture",
        contains_any(
            lower,
            &[
                "conversation",
                "chat",
                "transcript",
                "capture",
                "classifier",
                "classification",
                "category",
                "categories",
            ],
        ),
    );
    add_ref(
        &mut refs,
        "task-bootstrap",
        contains_any(
            lower,
            &[
                "handoff",
                "bootstrap",
                "context",
                "continue work",
                "next action",
            ],
        ),
    );
    add_ref(
        &mut refs,
        "forge-dashboard",
        contains_any(
            lower,
            &["dashboard", "library", "search", "filter", "interface"],
        ),
    );
    add_ref(
        &mut refs,
        "canonical-production-system",
        contains_any(
            lower,
            &[
                "planet",
                "world",
                "biome",
                "climate",
                "terrain",
                "organism",
                "creature",
                "ecology",
                "generation",
            ],
        ),
    );
    add_ref(
        &mut refs,
        "mindwarp-game",
        contains_any(
            lower,
            &[
                "game",
                "player",
                "mobile",
                "phone",
                "pc",
                "playstation",
                "xbox",
                "runtime",
            ],
        ),
    );
    refs.sort();
    refs.dedup();
    refs
}

fn add_ref(refs: &mut Vec<String>, value: &str, detected: bool) {
    if detected {
        refs.push(value.to_owned());
    }
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
            .flat_map(|record| record.facet_types)
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
        assert_eq!(first.len(), 1);
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
    fn multiple_facets_reference_one_atomic_span_instead_of_copying_it() {
        let records = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"Efficiency is important to me. We need cheap tests because I am concerned about waste.",
        );
        assert_eq!(records.len(), 2);
        let second = &records[1];
        assert!(second.facet_types.contains(&KnowledgeType::Requirement));
        assert!(second.facet_types.contains(&KnowledgeType::Risk));
        assert_eq!(second.source_spans.len(), 1);
        assert_eq!(
            second.summary,
            "We need cheap tests because I am concerned about waste."
        );
    }

    #[test]
    fn mentioning_philosophies_does_not_create_a_philosophy_record() {
        let records = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"I am seeing that philosophies are not necessarily philosophies in the Forge library.",
        );
        assert_eq!(records.len(), 1);
        assert!(!records[0].facet_types.contains(&KnowledgeType::Philosophy));
        assert!(records[0].facet_types.contains(&KnowledgeType::Observation));
        assert!(records[0].system_refs.contains(&"forge-kernel".to_owned()));
    }

    #[test]
    fn one_record_can_reference_multiple_atlas_systems() {
        let records = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"Make the Forge library searchable for planet and biome records.",
        );
        assert_eq!(records.len(), 1);
        assert!(records[0].system_refs.contains(&"forge-kernel".to_owned()));
        assert!(
            records[0]
                .system_refs
                .contains(&"canonical-production-system".to_owned())
        );
        assert!(
            records[0]
                .system_refs
                .contains(&"forge-dashboard".to_owned())
        );
    }

    #[test]
    fn decimal_points_do_not_split_atomic_source_spans() {
        let records = classify_knowledge(
            "evidence-a",
            &ActorKind::ImportedContent,
            b"The measured value is 1.5 and it passed.",
        );
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].summary,
            "The measured value is 1.5 and it passed."
        );
    }

    #[test]
    fn continuity_request_does_not_confuse_category_name_with_philosophy() {
        let found = types(
            "Make sure our philosophies are categorized and stored. I get worried that you forget your context, so we should verify the system before continuing.",
        );
        assert!(!found.contains(&KnowledgeType::Philosophy));
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
