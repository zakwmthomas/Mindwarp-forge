//! Neutral records shared across Forge module boundaries.

use crate::{CandidateId, ObjectId};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct SourceGapReceipt {
    pub state: &'static str,
    pub reason: Option<String>,
}

/// Immutable, data-only result evidence for an engine-neutral proof. Receipt
/// text and status are informational and carry no approval or promotion
/// authority.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProofReceiptRecord {
    pub schema_version: u16,
    pub receipt_id: String,
    pub system_id: String,
    pub proof_id: String,
    pub status: String,
    pub failure_classification: Option<String>,
    pub input_refs: Vec<String>,
    pub fixture_id: String,
    pub generator_versions: Vec<NamedVersion>,
    pub contract_versions: Vec<NamedVersion>,
    pub output_refs: Vec<String>,
    pub equivalence_method: String,
    pub measurements: Vec<ProofMeasurement>,
    pub warnings: Vec<String>,
    pub limitations: Vec<String>,
    pub created_at: String,
    pub runner_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NamedVersion {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProofMeasurement {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub method: String,
    pub classification: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ProofReceiptProjection {
    pub projection_schema_version: u16,
    pub requested_schema_version: u16,
    pub compatibility: &'static str,
    pub read_only: bool,
    pub receipts: Vec<ProofReceiptRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ImportReport {
    pub source_id: String,
    pub message_count: usize,
    pub candidate_count: usize,
    pub correction_intents: usize,
    pub approval_intents: usize,
    pub already_recorded: bool,
    pub message_evidence: Vec<ObjectId>,
    pub source_gap: SourceGapReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct BridgeReceipt {
    pub thread_id: String,
    pub message_id: String,
    pub evidence: ObjectId,
    pub candidate: Option<CandidateId>,
    pub already_recorded: bool,
}

/// Provenance-only research evidence. This cannot carry approval, promotion,
/// or execution authority.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ResearchSourceRecord {
    pub id: String,
    pub origin: String,
    pub source_type: String,
    pub accessed_at: String,
    pub fixity: Option<String>,
    pub location: String,
    pub access_notes: String,
    pub limitations: String,
    pub freshness: String,
    pub availability: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ResearchClaimRecord {
    pub id: String,
    pub source_id: String,
    pub source_span: String,
    pub claim: String,
    pub confidence: String,
    pub limitations: String,
    pub affected_systems: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ResearchContradictionRecord {
    pub id: String,
    pub left_claim_id: String,
    pub right_claim_id: String,
    pub scope_difference: String,
    pub unresolved_question: String,
    pub discriminating_evidence: String,
    pub status: String,
}

/// Immutable orchestration records. They describe lifecycle evidence but never
/// carry approval, promotion, application, or execution authority.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageRecord {
    pub id: String,
    pub stage: String,
    pub dependencies: Vec<String>,
    pub risk: String,
    pub evidence_requirements: Vec<String>,
    pub verification_plan: Vec<String>,
    pub authority_lane: String,
    pub next_action: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GateReceiptRecord {
    pub id: String,
    pub work_package_id: String,
    pub from_stage: String,
    pub to_stage: String,
    pub outcome: String,
    pub evidence_ids: Vec<String>,
    pub failure_reason: Option<String>,
    pub rollback_target: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BlockerRecord {
    pub id: String,
    pub work_package_id: String,
    pub blocker_type: String,
    pub affected_stage: String,
    pub requirement: String,
    pub evidence_ids: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RollbackRecord {
    pub id: String,
    pub work_package_id: String,
    pub gate_receipt_id: String,
    pub previous_standard: String,
    pub affected_artifact: String,
    pub restore_evidence_ids: Vec<String>,
    pub reason: String,
    pub follow_up: String,
}

/// Append-only worker telemetry. Identifiers and evidence remain references;
/// metric dimensions are separately bounded and cannot carry authority.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BatchEventRecord {
    pub schema_version: u16,
    pub id: String,
    pub sequence: u64,
    pub trace_id: String,
    pub parent_event_id: Option<String>,
    pub event_type: String,
    pub started_at_ms: i64,
    pub ended_at_ms: i64,
    pub route_system: String,
    pub route_group: String,
    pub route_contract: String,
    pub work_package_id: String,
    pub batch_id: String,
    pub outcome: String,
    pub evidence_ids: Vec<String>,
    pub privacy_class: String,
    pub cardinality_class: String,
    pub metric_name: Option<String>,
    pub metric_value: Option<i64>,
    pub metric_unit: Option<String>,
    pub metric_dimensions: Vec<MetricDimension>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MetricDimension {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct BatchMetricProjection {
    pub event_count: usize,
    pub completed_batches: usize,
    pub verified_batches: usize,
    pub failed_or_blocked_batches: usize,
    pub rework_events: usize,
    pub verified_closure_percent: Option<u32>,
    pub sample_state: String,
    pub recommendation: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ImprovementExperimentRecord {
    pub schema_version: u16,
    pub id: String,
    pub module_id: String,
    pub method_scope: String,
    pub input_contract: String,
    pub metric_name: String,
    pub metric_unit: String,
    pub metric_denominator: String,
    pub validity_rule: String,
    pub baseline_evidence_ids: Vec<String>,
    pub fixture_ids: Vec<String>,
    pub hypothesis: String,
    pub expected_gain: i64,
    pub implementation_cost_budget: u64,
    pub operating_cost_budget: u64,
    pub uncertainty: String,
    pub regression_guard: String,
    pub falsifier: String,
    pub promotion_threshold: String,
    pub rollback_trigger: String,
    pub stop_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ImprovementResultRecord {
    pub schema_version: u16,
    pub id: String,
    pub experiment_id: String,
    pub module_id: String,
    pub outcome: String,
    pub observed_gain: i64,
    pub uncertainty: String,
    pub regression_detected: bool,
    pub evidence_ids: Vec<String>,
    pub limitations: String,
    pub shared_projection_available: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ImprovementDecisionRecord {
    pub schema_version: u16,
    pub id: String,
    pub result_id: String,
    pub decision: String,
    pub evidence_ids: Vec<String>,
    pub counterexamples: Vec<String>,
    pub non_applicable_scope: Vec<String>,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransferCandidateRecord {
    pub schema_version: u16,
    pub id: String,
    pub source_module_id: String,
    pub source_experiment_id: String,
    pub source_result_id: String,
    pub method_scope: String,
    pub counterexamples: Vec<String>,
    pub non_applicable_scope: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransferGateRecord {
    pub schema_version: u16,
    pub id: String,
    pub candidate_id: String,
    pub target_module_id: String,
    pub target_experiment_id: String,
    pub target_result_id: Option<String>,
    pub decision: String,
    pub reason: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TransferAssessment {
    pub candidate_id: String,
    pub successful_modules: Vec<String>,
    pub regressed_modules: Vec<String>,
    pub state: String,
}
