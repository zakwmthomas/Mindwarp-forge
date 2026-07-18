//! Authority-negative project, workstream and session-routing records.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectRecord {
    pub schema_version: u16,
    pub id: String,
    pub revision: u64,
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub repository_uri: String,
    pub bootstrap_uri: String,
    pub authority_boundary: String,
    pub status: String,
    pub related_project_ids: Vec<String>,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkstreamLease {
    pub holder: String,
    pub expires_unix_ms: u64,
}

pub const WRITER_LEASE_MAX_TTL_MS: u64 = 30 * 60 * 1_000;
const CHECKPOINT_HASH_PREFIX: &str = "checkpoint-sha256:";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkstreamRecord {
    pub schema_version: u16,
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub stage: String,
    pub status: String,
    pub authority_lane: String,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
    pub checkpoint_uri: String,
    pub next_action: String,
    pub revision: u64,
    pub lease: Option<WorkstreamLease>,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRouteReceipt {
    pub schema_version: u16,
    pub session_id: String,
    pub revision: u64,
    pub state: String,
    pub candidate_project_ids: Vec<String>,
    pub project_id: Option<String>,
    pub workstream_id: Option<String>,
    pub method: String,
    pub confidence: u8,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossProjectLink {
    pub schema_version: u16,
    pub id: String,
    pub left_project_id: String,
    pub right_project_id: String,
    pub relation: String,
    pub state: String,
    pub required_gate: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FederationError {
    InvalidProject(String),
    InvalidWorkstream(String),
    InvalidSessionRoute(String),
    InvalidCrossProjectLink(String),
    StaleRevision { expected: u64, actual: u64 },
    LeaseConflict(String),
}

pub fn claim_writer_lease(
    current: &WorkstreamRecord,
    holder: &str,
    checkpoint_sha256: &str,
    now_unix_ms: u64,
    ttl_ms: u64,
) -> Result<WorkstreamRecord, FederationError> {
    validate_workstream(current)?;
    if holder.trim().is_empty()
        || !valid_sha256(checkpoint_sha256)
        || ttl_ms == 0
        || ttl_ms > WRITER_LEASE_MAX_TTL_MS
    {
        return Err(FederationError::InvalidWorkstream(current.id.clone()));
    }
    if current.status != "active" {
        return Err(FederationError::LeaseConflict(current.id.clone()));
    }
    if current
        .lease
        .as_ref()
        .is_some_and(|lease| lease.expires_unix_ms > now_unix_ms && lease.holder != holder)
    {
        return Err(FederationError::LeaseConflict(current.id.clone()));
    }
    let expires_unix_ms = now_unix_ms
        .checked_add(ttl_ms)
        .ok_or_else(|| FederationError::InvalidWorkstream(current.id.clone()))?;
    let mut next = current.clone();
    next.revision = current.revision.saturating_add(1);
    next.lease = Some(WorkstreamLease {
        holder: holder.to_string(),
        expires_unix_ms,
    });
    next.evidence_ids
        .retain(|evidence| !evidence.starts_with(CHECKPOINT_HASH_PREFIX));
    next.evidence_ids.push(format!(
        "{CHECKPOINT_HASH_PREFIX}{}",
        checkpoint_sha256.to_ascii_lowercase()
    ));
    next.evidence_ids.sort();
    next.evidence_ids.dedup();
    validate_workstream_successor(current, &next, now_unix_ms)?;
    Ok(next)
}

pub fn assert_writer_lease(
    current: &WorkstreamRecord,
    holder: &str,
    checkpoint_sha256: &str,
    now_unix_ms: u64,
) -> Result<(), FederationError> {
    validate_workstream(current)?;
    let expected_hash = format!(
        "{CHECKPOINT_HASH_PREFIX}{}",
        checkpoint_sha256.to_ascii_lowercase()
    );
    let hash_matches = valid_sha256(checkpoint_sha256)
        && current
            .evidence_ids
            .iter()
            .filter(|evidence| evidence.starts_with(CHECKPOINT_HASH_PREFIX))
            .eq(std::iter::once(&expected_hash));
    let lease_matches = current
        .lease
        .as_ref()
        .is_some_and(|lease| lease.holder == holder && lease.expires_unix_ms > now_unix_ms);
    if current.status != "active" || holder.trim().is_empty() || !hash_matches || !lease_matches {
        return Err(FederationError::LeaseConflict(current.id.clone()));
    }
    Ok(())
}

pub fn release_writer_lease(
    current: &WorkstreamRecord,
    holder: &str,
    now_unix_ms: u64,
) -> Result<WorkstreamRecord, FederationError> {
    validate_workstream(current)?;
    let Some(lease) = &current.lease else {
        return Err(FederationError::LeaseConflict(current.id.clone()));
    };
    if holder.trim().is_empty() || lease.holder != holder {
        return Err(FederationError::LeaseConflict(current.id.clone()));
    }
    let mut next = current.clone();
    next.revision = current.revision.saturating_add(1);
    next.lease = Some(WorkstreamLease {
        holder: holder.to_string(),
        expires_unix_ms: now_unix_ms.max(1),
    });
    validate_workstream_successor(current, &next, now_unix_ms)?;
    Ok(next)
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn normalize_alias(value: &str) -> String {
    value
        .bytes()
        .filter(|byte| byte.is_ascii_alphanumeric())
        .map(|byte| char::from(byte.to_ascii_lowercase()))
        .collect()
}

pub fn project_id_for(canonical_name: &str, repository_uri: &str) -> String {
    let mut sha = Sha256::new();
    sha.update(b"forge-project:v1\0");
    sha.update(normalize_alias(canonical_name).as_bytes());
    sha.update(b"\0");
    sha.update(repository_uri.trim().as_bytes());
    format!("project-{:x}", sha.finalize())
}

pub fn validate_project(record: &ProjectRecord) -> Result<(), FederationError> {
    let normalized_name = normalize_alias(&record.canonical_name);
    let normalized_aliases = record
        .aliases
        .iter()
        .map(|alias| normalize_alias(alias))
        .collect::<Vec<_>>();
    let aliases: BTreeSet<_> = normalized_aliases.iter().cloned().collect();
    if record.schema_version != 1
        || record.revision == 0
        || record.id != project_id_for(&record.canonical_name, &record.repository_uri)
        || normalized_name.is_empty()
        || !aliases.contains(&normalized_name)
        || aliases.iter().any(String::is_empty)
        || aliases.len() != record.aliases.len()
        || normalized_aliases.windows(2).any(|pair| pair[0] >= pair[1])
        || record.repository_uri.trim().is_empty()
        || record.bootstrap_uri.trim().is_empty()
        || record.authority_boundary.trim().is_empty()
        || !matches!(record.status.as_str(), "active" | "paused" | "archived")
        || !canonical_strings(&record.evidence_ids, false)
        || !canonical_strings(&record.related_project_ids, true)
        || record.related_project_ids.iter().any(|id| id == &record.id)
        || !bounded_record(record)
    {
        return Err(FederationError::InvalidProject(record.id.clone()));
    }
    Ok(())
}

pub fn validate_project_successor(
    current: &ProjectRecord,
    next: &ProjectRecord,
) -> Result<(), FederationError> {
    validate_project(next)?;
    let status_valid = matches!(
        (current.status.as_str(), next.status.as_str()),
        ("active", "active" | "paused" | "archived")
            | ("paused", "paused" | "active" | "archived")
            | ("archived", "archived")
    );
    if current.id != next.id
        || next.revision != current.revision.saturating_add(1)
        || current.canonical_name != next.canonical_name
        || current.aliases != next.aliases
        || current.repository_uri != next.repository_uri
        || current.bootstrap_uri != next.bootstrap_uri
        || current.authority_boundary != next.authority_boundary
        || !status_valid
        || !is_subset(&current.related_project_ids, &next.related_project_ids)
        || !is_subset(&current.evidence_ids, &next.evidence_ids)
    {
        return Err(FederationError::InvalidProject(next.id.clone()));
    }
    Ok(())
}

pub fn validate_workstream(record: &WorkstreamRecord) -> Result<(), FederationError> {
    if record.schema_version != 1
        || record.id.trim().is_empty()
        || record.project_id.trim().is_empty()
        || record.title.trim().is_empty()
        || !matches!(
            record.stage.as_str(),
            "research"
                | "design"
                | "readiness"
                | "implementation"
                | "verification"
                | "promotion"
                | "monitoring"
        )
        || !matches!(
            record.status.as_str(),
            "active" | "blocked" | "complete" | "paused"
        )
        || !matches!(
            record.authority_lane.as_str(),
            "automatic" | "delegated" | "batch_for_owner" | "immediate_authorization"
        )
        || record
            .dependencies
            .iter()
            .any(|dependency| dependency == &record.id)
        || !canonical_strings(&record.dependencies, true)
        || !canonical_strings(&record.blockers, true)
        || record.checkpoint_uri.trim().is_empty()
        || record.next_action.trim().is_empty()
        || record.revision == 0
        || !canonical_strings(&record.evidence_ids, false)
        || record
            .lease
            .as_ref()
            .is_some_and(|lease| lease.holder.trim().is_empty() || lease.expires_unix_ms == 0)
        || !bounded_record(record)
    {
        return Err(FederationError::InvalidWorkstream(record.id.clone()));
    }
    Ok(())
}

pub fn validate_workstream_successor(
    current: &WorkstreamRecord,
    next: &WorkstreamRecord,
    now_unix_ms: u64,
) -> Result<(), FederationError> {
    validate_workstream(next)?;
    if current.id != next.id || current.project_id != next.project_id {
        return Err(FederationError::InvalidWorkstream(next.id.clone()));
    }
    let expected = current.revision.saturating_add(1);
    if next.revision != expected {
        return Err(FederationError::StaleRevision {
            expected,
            actual: next.revision,
        });
    }
    if let Some(lease) = &current.lease {
        if lease.expires_unix_ms > now_unix_ms
            && next
                .lease
                .as_ref()
                .is_none_or(|candidate| candidate.holder != lease.holder)
        {
            return Err(FederationError::LeaseConflict(current.id.clone()));
        }
    }
    Ok(())
}

pub fn validate_session_route(record: &SessionRouteReceipt) -> Result<(), FederationError> {
    let assignment_valid = match (record.state.as_str(), record.method.as_str()) {
        (
            "routed",
            "explicit_owner_task_binding" | "registered_repository_root" | "explicit_project_alias",
        ) => record.project_id.is_some(),
        ("ambiguous", "deterministic_suggestion") | ("unrouted", "unrouted_inbox") => {
            record.project_id.is_none() && record.workstream_id.is_none()
        }
        _ => false,
    };
    if record.schema_version != 1
        || record.session_id.trim().is_empty()
        || record.revision == 0
        || !assignment_valid
        || record.workstream_id.is_some() && record.project_id.is_none()
        || record.confidence > 100
        || !canonical_strings(&record.candidate_project_ids, true)
        || !canonical_strings(&record.evidence_ids, false)
        || record
            .project_id
            .as_ref()
            .is_some_and(|id| !record.candidate_project_ids.contains(id))
        || !bounded_record(record)
    {
        return Err(FederationError::InvalidSessionRoute(
            record.session_id.clone(),
        ));
    }
    Ok(())
}

pub fn cross_project_link_id_for(left: &str, right: &str, relation: &str) -> String {
    let mut sha = Sha256::new();
    sha.update(b"forge-cross-project-link:v1\0");
    sha.update(left.as_bytes());
    sha.update(b"\0");
    sha.update(right.as_bytes());
    sha.update(b"\0");
    sha.update(relation.as_bytes());
    format!("cross-project-{:x}", sha.finalize())
}

pub fn validate_cross_project_link(record: &CrossProjectLink) -> Result<(), FederationError> {
    if record.schema_version != 1
        || record.id
            != cross_project_link_id_for(
                &record.left_project_id,
                &record.right_project_id,
                &record.relation,
            )
        || record.left_project_id.trim().is_empty()
        || record.right_project_id.trim().is_empty()
        || record.left_project_id == record.right_project_id
        || !matches!(
            record.relation.as_str(),
            "reuse_candidate" | "dependency" | "contradiction" | "transfer_proposal" | "related"
        )
        || !matches!(
            record.state.as_str(),
            "evidence_only" | "rejected" | "promoted"
        )
        || record.required_gate.trim().is_empty()
        || !canonical_strings(&record.evidence_ids, false)
        || !bounded_record(record)
    {
        return Err(FederationError::InvalidCrossProjectLink(record.id.clone()));
    }
    Ok(())
}

fn canonical_strings(values: &[String], allow_empty: bool) -> bool {
    (allow_empty || !values.is_empty())
        && values.len() <= 1_024
        && values
            .iter()
            .all(|value| !value.trim().is_empty() && value.len() <= 4_096)
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn is_subset(current: &[String], next: &[String]) -> bool {
    current
        .iter()
        .all(|value| next.binary_search(value).is_ok())
}

fn bounded_record<T: Serialize>(record: &T) -> bool {
    serde_json::to_vec(record).is_ok_and(|bytes| bytes.len() <= 4 * 1024 * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greenfield() -> ProjectRecord {
        let canonical_name = "Greenfield".to_string();
        let repository_uri = r"C:\Users\zakwm\CascadeProjects\greenfield".to_string();
        ProjectRecord {
            schema_version: 1,
            id: project_id_for(&canonical_name, &repository_uri),
            revision: 1,
            canonical_name,
            aliases: vec!["Greenfeld".into(), "Greenfield".into()],
            repository_uri,
            bootstrap_uri: "AGENTS.md".into(),
            authority_boundary: "Independent product; Forge reuse is evidence only.".into(),
            status: "active".into(),
            related_project_ids: vec!["mindwarp-forge".into()],
            evidence_ids: vec!["evidence-greenfield".into()],
        }
    }

    #[test]
    fn explicit_aliases_join_greenfeld_and_greenfield_without_fuzzy_guessing() {
        let project = greenfield();
        validate_project(&project).unwrap();
        let aliases: BTreeSet<_> = project
            .aliases
            .iter()
            .map(|item| normalize_alias(item))
            .collect();
        assert!(aliases.contains(&normalize_alias("Greenfeld")));
        assert!(aliases.contains(&normalize_alias("Greenfield")));
        assert!(!aliases.contains(&normalize_alias("Greenfields")));
    }

    #[test]
    fn ambiguous_routes_fail_closed_without_a_project_assignment() {
        let mut route = SessionRouteReceipt {
            schema_version: 1,
            session_id: "session-a".into(),
            revision: 1,
            state: "ambiguous".into(),
            candidate_project_ids: vec!["greenfield".into(), "mindwarp-forge".into()],
            project_id: None,
            workstream_id: None,
            method: "deterministic_suggestion".into(),
            confidence: 50,
            evidence_ids: vec!["evidence-route".into()],
        };
        validate_session_route(&route).unwrap();
        route.project_id = Some("greenfield".into());
        assert!(validate_session_route(&route).is_err());
    }

    #[test]
    fn active_lease_and_revision_prevent_parallel_overwrite() {
        let current = WorkstreamRecord {
            schema_version: 1,
            id: "greenfield-release".into(),
            project_id: "greenfield".into(),
            title: "Android release readiness".into(),
            stage: "implementation".into(),
            status: "active".into(),
            authority_lane: "delegated".into(),
            dependencies: vec![],
            blockers: vec![],
            checkpoint_uri: "docs/RELEASE_AUDIT.md".into(),
            next_action: "Resolve release blockers.".into(),
            revision: 1,
            lease: Some(WorkstreamLease {
                holder: "worker-a".into(),
                expires_unix_ms: 200,
            }),
            evidence_ids: vec!["evidence-workstream".into()],
        };
        let mut next = current.clone();
        next.revision = 2;
        validate_workstream_successor(&current, &next, 100).unwrap();
        next.lease.as_mut().unwrap().holder = "worker-b".into();
        assert_eq!(
            validate_workstream_successor(&current, &next, 100),
            Err(FederationError::LeaseConflict("greenfield-release".into()))
        );
        assert!(validate_workstream_successor(&current, &next, 250).is_ok());
    }

    #[test]
    fn writer_claim_is_bounded_checkpoint_bound_and_releasable() {
        let current = WorkstreamRecord {
            schema_version: 1,
            id: "forge-live-mainline".into(),
            project_id: "mindwarp-forge".into(),
            title: "Forge live mainline writer".into(),
            stage: "implementation".into(),
            status: "active".into(),
            authority_lane: "delegated".into(),
            dependencies: vec![],
            blockers: vec![],
            checkpoint_uri: "context/active/WORKER_BATCH_STATE.json".into(),
            next_action: "Continue the sole canonical writer route.".into(),
            revision: 1,
            lease: None,
            evidence_ids: vec!["evidence-workstream".into()],
        };
        let hash = "a".repeat(64);
        let claimed = claim_writer_lease(&current, "session-a", &hash, 100, 60_000).unwrap();
        assert_writer_lease(&claimed, "session-a", &hash, 101).unwrap();
        assert!(assert_writer_lease(&claimed, "session-b", &hash, 101).is_err());
        assert!(assert_writer_lease(&claimed, "session-a", &"b".repeat(64), 101).is_err());
        assert!(claim_writer_lease(&claimed, "session-b", &hash, 101, 60_000).is_err());
        assert!(
            claim_writer_lease(
                &current,
                "session-a",
                &hash,
                100,
                WRITER_LEASE_MAX_TTL_MS + 1
            )
            .is_err()
        );

        let released = release_writer_lease(&claimed, "session-a", 200).unwrap();
        assert!(assert_writer_lease(&released, "session-a", &hash, 200).is_err());
        let reclaimed = claim_writer_lease(&released, "session-b", &hash, 201, 60_000).unwrap();
        assert_writer_lease(&reclaimed, "session-b", &hash, 202).unwrap();
    }

    #[test]
    fn cross_project_link_never_implies_transfer_authority() {
        let link = CrossProjectLink {
            schema_version: 1,
            id: cross_project_link_id_for("greenfield", "mindwarp-forge", "reuse_candidate"),
            left_project_id: "greenfield".into(),
            right_project_id: "mindwarp-forge".into(),
            relation: "reuse_candidate".into(),
            state: "evidence_only".into(),
            required_gate: "target-local-design-and-owner-approval".into(),
            evidence_ids: vec!["evidence-link".into()],
        };
        validate_cross_project_link(&link).unwrap();
    }

    #[test]
    fn forged_project_identity_and_alias_order_fail_closed() {
        let mut project = greenfield();
        project.id = "forged".into();
        assert!(validate_project(&project).is_err());
        project = greenfield();
        project.aliases.swap(0, 1);
        assert!(validate_project(&project).is_err());
    }

    #[test]
    fn project_successor_preserves_identity_and_append_only_evidence() {
        let current = greenfield();
        let mut next = current.clone();
        next.revision = 2;
        next.status = "paused".into();
        next.evidence_ids.push("evidence-later".into());
        next.evidence_ids.sort();
        validate_project_successor(&current, &next).unwrap();
        next.aliases.push("Greenfields".into());
        assert!(validate_project_successor(&current, &next).is_err());
    }

    #[test]
    fn suggestion_method_cannot_silently_route() {
        let project_id = greenfield().id;
        let mut route = SessionRouteReceipt {
            schema_version: 1,
            session_id: "session-suggestion".into(),
            revision: 1,
            state: "ambiguous".into(),
            candidate_project_ids: vec![project_id.clone()],
            project_id: None,
            workstream_id: None,
            method: "deterministic_suggestion".into(),
            confidence: 90,
            evidence_ids: vec!["evidence-route".into()],
        };
        validate_session_route(&route).unwrap();
        route.state = "routed".into();
        route.project_id = Some(project_id);
        assert!(validate_session_route(&route).is_err());
    }
}
