//! Runtime-independent GP4 Signal Anchor proof bundle.

use addressable_world_binding::bind_addressable_world_package;
use derived_world_rules::{
    CausalWorldPacket, ClimateContract, ClimateInput, GeologicalAtmosphericContract,
    GeologicalAtmosphericInput, HydrologicalContract, HydrologicalInput,
    RegionalEnvironmentContract, RegionalEnvironmentInput, SignalChannel, SignalPotential,
    StellarOrbitalContract, StellarOrbitalInput, WorldGenerationInput, compile_world,
};
use field_basis::{FieldRecipe, ONE, Term};
use mindwarp_gameplay_foundation::{
    BaseLoopActionV1, BaseLoopLedgerV1, BaseLoopStateV1, EncounterFailureV1, EncounterSituationV1,
    LoopPhaseV1, LoopWorldContextV1, PreparationV1, ProgressionLedgerV1, SessionRecordV1,
    SessionState, StableStopV1, apply_base_loop_action, apply_progression, fixed_encounter_grammar,
    fixed_sessions, start_authored_base_loop, start_c3a_base_loop,
};
use mindwarp_vertical_persistence::{
    VerticalCommandBatchV1, VerticalIdentityV1, VerticalLogV1, VerticalPersistenceReceiptV1,
    VerticalSnapshotV1, persistence_receipt,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;
use universe_identity::{AddressSegment, NodeKind, UniverseAddress};

pub const MAX_BUNDLE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalAnchorError {
    Bound,
    Authority,
    Dependency,
    Replay,
    Registry,
    Digest,
    Codec,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AdapterRequirementClassV1 {
    Hard,
    Compare,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AdapterRequirementStatusV1 {
    Unmeasured,
    Measured,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPresentationSlotV1 {
    pub slot_id: String,
    pub source_ids: Vec<String>,
    pub source_id_list_digest: [u8; 32],
    pub text_equivalent: String,
    pub non_color_cue: String,
    pub reduced_motion_equivalent: String,
    pub screen_reader_label: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRequirementV1 {
    pub requirement_id: String,
    pub class: AdapterRequirementClassV1,
    pub status: AdapterRequirementStatusV1,
    pub question: String,
    pub required_evidence: String,
    pub method: String,
    pub target: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseLoopSemanticProjectionV1 {
    pub schema_version: u16,
    pub run_id: String,
    pub session_id: String,
    pub phase: LoopPhaseV1,
    pub preparation: Option<PreparationV1>,
    pub predecessor_outcome_id: Option<String>,
    pub session_state: SessionState,
    pub ledger_before: BaseLoopLedgerV1,
    pub ledger_after: BaseLoopLedgerV1,
    pub failure: Option<EncounterFailureV1>,
    pub recoveries_used: u8,
    pub stable_stop: StableStopV1,
    pub trace: Vec<BaseLoopActionV1>,
}

impl BaseLoopSemanticProjectionV1 {
    pub const FIELD_NAMES: [&'static str; 13] = [
        "schema_version",
        "run_id",
        "session_id",
        "phase",
        "preparation",
        "predecessor_outcome_id",
        "session_state",
        "ledger_before",
        "ledger_after",
        "failure",
        "recoveries_used",
        "stable_stop",
        "trace",
    ];
    fn from_state(state: &BaseLoopStateV1) -> Self {
        let BaseLoopStateV1 {
            schema_version,
            run_id,
            session_id,
            world_context: _,
            phase,
            preparation,
            predecessor_outcome_id,
            session_state,
            ledger_before,
            ledger_after,
            failure,
            recoveries_used,
            stable_stop,
            trace,
        } = state.clone();
        Self {
            schema_version,
            run_id,
            session_id,
            phase,
            preparation,
            predecessor_outcome_id,
            session_state,
            ledger_before,
            ledger_after,
            failure,
            recoveries_used,
            stable_stop,
            trace,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignalAnchorBundleV1 {
    pub schema_version: u16,
    pub bundle_id: String,
    pub session_bytes: Vec<u8>,
    pub c3a_input_bytes: Vec<u8>,
    pub c3a_packet_bytes: Vec<u8>,
    pub c4v_log_bytes: Vec<u8>,
    pub return_prefix_snapshot_bytes: Vec<u8>,
    pub final_snapshot_bytes: Vec<u8>,
    pub persistence_receipt_bytes: Vec<u8>,
    pub command_ids: Vec<[u8; 32]>,
    pub authored_shadow_state_bytes: Vec<u8>,
    pub common_semantic_digest: [u8; 32],
    pub gp3_situation_bytes: Vec<u8>,
    pub gp4_approach_ref_digest: [u8; 32],
    pub gp3_threat_digest: String,
    pub gp4_threat_ref_digest: [u8; 32],
    pub threat_selected: bool,
    pub progression_ledger_bytes: Vec<u8>,
    pub presentation_slots: Vec<SemanticPresentationSlotV1>,
    pub adapter_requirements: Vec<AdapterRequirementV1>,
    pub bundle_digest: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalAnchorCommandRowV1 {
    pub actor_player_id: [u8; 32],
    pub command_id: [u8; 32],
    pub sequence: u64,
    pub expected_revision: u64,
    pub expected_parent: Option<[u8; 32]>,
    pub action_count: usize,
    pub action_labels: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalAnchorInspectionV1 {
    pub c3a_projection: BaseLoopSemanticProjectionV1,
    pub authored_projection: BaseLoopSemanticProjectionV1,
    pub c3a_ledger_before_bytes: Vec<u8>,
    pub authored_ledger_before_bytes: Vec<u8>,
    pub c3a_ledger_after_bytes: Vec<u8>,
    pub authored_ledger_after_bytes: Vec<u8>,
    pub c3a_world_is_validated: bool,
    pub shadow_world_is_authored: bool,
    pub authored_c4v_rejected: bool,
    pub command_rows: Vec<SignalAnchorCommandRowV1>,
    pub prefix_restart_verified: bool,
    pub final_restart_verified: bool,
    pub stored_state_distrusted: bool,
    pub situation_id: String,
    pub approach_id: String,
    pub tool_id: String,
    pub risk_id: String,
    pub threat_id: String,
    pub outcome_id: String,
    pub intervention_step_ids: Vec<String>,
    pub progression_rule_id: String,
    pub progression_decision_id: String,
    pub progression_emitted_ids: Vec<String>,
    pub progression_transition_ids: Vec<String>,
    pub progression_lane_ids: Vec<String>,
    pub progression_cross_identity_rejected: bool,
    pub progression_duplicate_rejected: bool,
    pub c3a_progression_rejected: bool,
}

impl SignalAnchorInspectionV1 {
    pub fn semantic_field_bytes(&self, name: &str) -> Result<Vec<u8>, SignalAnchorError> {
        let value =
            serde_json::to_value(&self.c3a_projection).map_err(|_| SignalAnchorError::Codec)?;
        value
            .get(name)
            .ok_or(SignalAnchorError::Registry)
            .and_then(|v| serde_json::to_vec(v).map_err(|_| SignalAnchorError::Codec))
    }
}

#[derive(Clone)]
struct Built {
    bundle: SignalAnchorBundleV1,
    inspection: SignalAnchorInspectionV1,
}

pub fn build_signal_anchor_bundle(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<SignalAnchorBundleV1, SignalAnchorError> {
    Ok(build_all(input, packet)?.bundle)
}

pub fn inspect_signal_anchor_bundle(
    bundle: &SignalAnchorBundleV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<SignalAnchorInspectionV1, SignalAnchorError> {
    bundle.validate_against(input, packet)?;
    Ok(build_all(input, packet)?.inspection)
}

impl SignalAnchorBundleV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SignalAnchorError> {
        check_bounds(self)?;
        if digest_bundle(self)? != self.bundle_digest {
            return Err(SignalAnchorError::Digest);
        }
        let bytes = serde_json::to_vec(self).map_err(|_| SignalAnchorError::Codec)?;
        if bytes.len() > MAX_BUNDLE_BYTES {
            return Err(SignalAnchorError::Bound);
        }
        Ok(bytes)
    }
    pub fn from_bytes(
        bytes: &[u8],
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<Self, SignalAnchorError> {
        if bytes.len() > MAX_BUNDLE_BYTES {
            return Err(SignalAnchorError::Bound);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| SignalAnchorError::Codec)?;
        if serde_json::to_vec(&value).map_err(|_| SignalAnchorError::Codec)? != bytes {
            return Err(SignalAnchorError::Codec);
        }
        value.validate_against(input, packet)?;
        Ok(value)
    }
    pub fn validate_against(
        &self,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<(), SignalAnchorError> {
        check_bounds(self)?;
        if digest_bundle(self)? != self.bundle_digest {
            return Err(SignalAnchorError::Digest);
        }
        let expected = build_all(input, packet)?;
        validate_received_memoized(self, &expected.bundle, input, packet)?;
        if self.session_bytes != expected.bundle.session_bytes
            || self.c3a_input_bytes != expected.bundle.c3a_input_bytes
            || self.c3a_packet_bytes != expected.bundle.c3a_packet_bytes
            || self.persistence_receipt_bytes != expected.bundle.persistence_receipt_bytes
            || self.gp3_situation_bytes != expected.bundle.gp3_situation_bytes
            || self.progression_ledger_bytes != expected.bundle.progression_ledger_bytes
        {
            return Err(SignalAnchorError::Dependency);
        }
        if self.c4v_log_bytes != expected.bundle.c4v_log_bytes
            || self.return_prefix_snapshot_bytes != expected.bundle.return_prefix_snapshot_bytes
            || self.final_snapshot_bytes != expected.bundle.final_snapshot_bytes
            || self.authored_shadow_state_bytes != expected.bundle.authored_shadow_state_bytes
            || self.command_ids != expected.bundle.command_ids
            || self.common_semantic_digest != expected.bundle.common_semantic_digest
            || self.threat_selected != expected.bundle.threat_selected
        {
            return Err(SignalAnchorError::Replay);
        }
        let mut left = self.clone();
        left.bundle_digest = [0; 32];
        let mut right = expected.bundle;
        right.bundle_digest = [0; 32];
        if left != right {
            return Err(SignalAnchorError::Registry);
        }
        Ok(())
    }
}

fn validate_received_memoized(
    v: &SignalAnchorBundleV1,
    canonical: &SignalAnchorBundleV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<(), SignalAnchorError> {
    static CANONICAL_RECEIVED: OnceLock<()> = OnceLock::new();
    let same = v.session_bytes == canonical.session_bytes
        && v.c3a_input_bytes == canonical.c3a_input_bytes
        && v.c3a_packet_bytes == canonical.c3a_packet_bytes
        && v.c4v_log_bytes == canonical.c4v_log_bytes
        && v.return_prefix_snapshot_bytes == canonical.return_prefix_snapshot_bytes
        && v.final_snapshot_bytes == canonical.final_snapshot_bytes
        && v.persistence_receipt_bytes == canonical.persistence_receipt_bytes
        && v.authored_shadow_state_bytes == canonical.authored_shadow_state_bytes
        && v.gp3_situation_bytes == canonical.gp3_situation_bytes
        && v.progression_ledger_bytes == canonical.progression_ledger_bytes
        && v.threat_selected == canonical.threat_selected;
    if same && CANONICAL_RECEIVED.get().is_some() {
        return Ok(());
    }
    validate_received(v, input, packet)?;
    if same {
        let _ = CANONICAL_RECEIVED.set(());
    }
    Ok(())
}

fn validate_received(
    v: &SignalAnchorBundleV1,
    expected_input: &WorldGenerationInput,
    expected_packet: &CausalWorldPacket,
) -> Result<(), SignalAnchorError> {
    let record: SessionRecordV1 =
        serde_json::from_slice(&v.session_bytes).map_err(|_| SignalAnchorError::Dependency)?;
    record
        .validate()
        .map_err(|_| SignalAnchorError::Dependency)?;
    if serde_json::to_vec(&record).map_err(|_| SignalAnchorError::Codec)? != v.session_bytes {
        return Err(SignalAnchorError::Dependency);
    }
    let input = WorldGenerationInput::from_bytes(&v.c3a_input_bytes)
        .map_err(|_| SignalAnchorError::Dependency)?;
    let packet = CausalWorldPacket::from_bytes(&v.c3a_packet_bytes)
        .map_err(|_| SignalAnchorError::Dependency)?;
    if &input != expected_input || &packet != expected_packet {
        return Err(SignalAnchorError::Dependency);
    }
    let log = VerticalLogV1::from_bytes(&v.c4v_log_bytes, &record, &input, &packet)
        .map_err(|_| SignalAnchorError::Replay)?;
    let terminal = log
        .restart(&record, &input, &packet)
        .map_err(|_| SignalAnchorError::Replay)?;
    let prefix_log = log_prefix(&log, 3)?;
    let prefix = VerticalSnapshotV1::from_bytes(
        &v.return_prefix_snapshot_bytes,
        &prefix_log,
        &record,
        &input,
        &packet,
    )
    .map_err(|_| SignalAnchorError::Replay)?;
    if prefix.revision != 3 {
        return Err(SignalAnchorError::Replay);
    }
    let final_snapshot =
        VerticalSnapshotV1::from_bytes(&v.final_snapshot_bytes, &log, &record, &input, &packet)
            .map_err(|_| SignalAnchorError::Replay)?;
    if final_snapshot.revision != 4
        || final_snapshot.state_bytes
            != terminal
                .to_bytes(&record)
                .map_err(|_| SignalAnchorError::Replay)?
    {
        return Err(SignalAnchorError::Replay);
    }
    let receipt = VerticalPersistenceReceiptV1::from_bytes(&v.persistence_receipt_bytes)
        .map_err(|_| SignalAnchorError::Dependency)?;
    if receipt != persistence_receipt(&log).map_err(|_| SignalAnchorError::Dependency)? {
        return Err(SignalAnchorError::Dependency);
    }
    let shadow_value: serde_json::Value = serde_json::from_slice(&v.authored_shadow_state_bytes)
        .map_err(|_| SignalAnchorError::Replay)?;
    let authored_value = serde_json::to_value(LoopWorldContextV1::AuthoredFixture)
        .map_err(|_| SignalAnchorError::Codec)?;
    if shadow_value.get("world_context") != Some(&authored_value) {
        return Err(SignalAnchorError::Authority);
    }
    let shadow = BaseLoopStateV1::from_bytes(
        &record,
        &LoopWorldContextV1::AuthoredFixture,
        &v.authored_shadow_state_bytes,
    )
    .map_err(|_| SignalAnchorError::Replay)?;
    if BaseLoopSemanticProjectionV1::from_state(&terminal)
        != BaseLoopSemanticProjectionV1::from_state(&shadow)
        || terminal
            .ledger_before
            .to_bytes()
            .map_err(|_| SignalAnchorError::Replay)?
            != shadow
                .ledger_before
                .to_bytes()
                .map_err(|_| SignalAnchorError::Replay)?
        || terminal
            .ledger_after
            .to_bytes()
            .map_err(|_| SignalAnchorError::Replay)?
            != shadow
                .ledger_after
                .to_bytes()
                .map_err(|_| SignalAnchorError::Replay)?
    {
        return Err(SignalAnchorError::Replay);
    }
    let diverted = shadow
        .trace
        .iter()
        .any(|a| matches!(a,BaseLoopActionV1::Prepare(p) if p.divert_threat));
    if diverted != v.threat_selected {
        return Err(SignalAnchorError::Replay);
    }
    EncounterSituationV1::from_bytes(&v.gp3_situation_bytes)
        .map_err(|_| SignalAnchorError::Dependency)?;
    ProgressionLedgerV1::from_bytes(&shadow.ledger_after, &v.progression_ledger_bytes)
        .map_err(|_| SignalAnchorError::Dependency)?;
    Ok(())
}

fn log_prefix(log: &VerticalLogV1, count: usize) -> Result<VerticalLogV1, SignalAnchorError> {
    let mut value = log.clone();
    value.delta_bytes.truncate(count);
    let last = if count == 0 {
        value.initial_state_bytes.clone()
    } else {
        let delta = hierarchy_history::DeltaEnvelope::decode_strict(&value.delta_bytes[count - 1])
            .map_err(|_| SignalAnchorError::Replay)?;
        let stored: serde_json::Value =
            serde_json::from_slice(&delta.operation).map_err(|_| SignalAnchorError::Replay)?;
        serde_json::from_value(
            stored
                .get("consequence_bytes")
                .cloned()
                .ok_or(SignalAnchorError::Replay)?,
        )
        .map_err(|_| SignalAnchorError::Replay)?
    };
    value.final_state_bytes = last;
    // Reopening a truncated prefix through the public codec requires its authentic log hash.
    // The C4V type intentionally keeps hash recomputation private, so reconstruct by replaying
    // the exact first commands from the full strict log instead.
    let record = fixed_sessions()
        .into_iter()
        .find(|r| r.session_id == value.identity.session_id)
        .ok_or(SignalAnchorError::Replay)?;
    let input = fixed_world_input([0x4a; 32]);
    let packet = compile_world(&input).map_err(|_| SignalAnchorError::Replay)?;
    let initial = start_c3a_base_loop(
        &record,
        &value.identity.run_id,
        BaseLoopLedgerV1::empty(),
        &input,
        &packet,
    )
    .map_err(|_| SignalAnchorError::Replay)?;
    let mut rebuilt =
        VerticalLogV1::initialize(value.identity.clone(), &initial, &record, &input, &packet)
            .map_err(|_| SignalAnchorError::Replay)?;
    for raw in log.delta_bytes.iter().take(count) {
        let delta = hierarchy_history::DeltaEnvelope::decode_strict(raw)
            .map_err(|_| SignalAnchorError::Replay)?;
        let stored: serde_json::Value =
            serde_json::from_slice(&delta.operation).map_err(|_| SignalAnchorError::Replay)?;
        let command_bytes: Vec<u8> = serde_json::from_value(
            stored
                .get("command_bytes")
                .cloned()
                .ok_or(SignalAnchorError::Replay)?,
        )
        .map_err(|_| SignalAnchorError::Replay)?;
        let command = VerticalCommandBatchV1::from_bytes(&command_bytes)
            .map_err(|_| SignalAnchorError::Replay)?;
        rebuilt = rebuilt
            .append(&record, &input, &packet, &command)
            .map_err(|_| SignalAnchorError::Replay)?
            .0;
    }
    Ok(rebuilt)
}

fn check_bounds(v: &SignalAnchorBundleV1) -> Result<(), SignalAnchorError> {
    for (n, max) in [
        (v.session_bytes.len(), 262144),
        (v.c3a_input_bytes.len(), 262144),
        (v.c3a_packet_bytes.len(), 262144),
        (v.c4v_log_bytes.len(), 4194304),
        (v.return_prefix_snapshot_bytes.len(), 524288),
        (v.final_snapshot_bytes.len(), 524288),
        (v.persistence_receipt_bytes.len(), 65536),
        (v.authored_shadow_state_bytes.len(), 262144),
        (v.gp3_situation_bytes.len(), 32768),
        (v.progression_ledger_bytes.len(), 1048576),
    ] {
        if n > max {
            return Err(SignalAnchorError::Bound);
        }
    }
    if v.command_ids.len() != 4
        || v.presentation_slots.len() != 25
        || v.adapter_requirements.len() != 29
    {
        return Err(SignalAnchorError::Bound);
    }
    for row in &v.presentation_slots {
        if row.source_ids.is_empty() || row.source_ids.len() > 16 {
            return Err(SignalAnchorError::Bound);
        }
        for s in std::iter::once(&row.slot_id).chain(row.source_ids.iter()) {
            if s.is_empty() || s.len() > 128 {
                return Err(SignalAnchorError::Bound);
            }
        }
        for s in [
            &row.text_equivalent,
            &row.non_color_cue,
            &row.reduced_motion_equivalent,
            &row.screen_reader_label,
        ] {
            if s.is_empty() || s.len() > 512 {
                return Err(SignalAnchorError::Bound);
            }
        }
    }
    for row in &v.adapter_requirements {
        if row.requirement_id.is_empty() || row.requirement_id.len() > 128 {
            return Err(SignalAnchorError::Bound);
        }
        for s in [
            &row.question,
            &row.required_evidence,
            &row.method,
            &row.target,
        ] {
            if s.is_empty() || s.len() > 512 {
                return Err(SignalAnchorError::Bound);
            }
        }
    }
    Ok(())
}

fn digest_bundle(v: &SignalAnchorBundleV1) -> Result<[u8; 32], SignalAnchorError> {
    let mut copy = v.clone();
    copy.bundle_digest = [0; 32];
    let body = serde_json::to_vec(&copy).map_err(|_| SignalAnchorError::Codec)?;
    let mut h = Sha256::new();
    h.update(b"mindwarp.gp4.signal-anchor.bundle.v1\0");
    h.update((body.len() as u64).to_be_bytes());
    h.update(body);
    Ok(h.finalize().into())
}

fn build_all(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<Built, SignalAnchorError> {
    static FIXED: OnceLock<Result<Built, SignalAnchorError>> = OnceLock::new();
    let built = FIXED
        .get_or_init(|| {
            let input = fixed_world_input([0x4a; 32]);
            let packet = compile_world(&input).map_err(|_| SignalAnchorError::Dependency)?;
            build_uncached(&input, &packet)
        })
        .clone()?;
    if input.to_bytes().map_err(|_| SignalAnchorError::Authority)? != built.bundle.c3a_input_bytes
        || packet
            .to_bytes()
            .map_err(|_| SignalAnchorError::Authority)?
            != built.bundle.c3a_packet_bytes
    {
        return Err(SignalAnchorError::Authority);
    }
    Ok(built)
}

fn build_uncached(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<Built, SignalAnchorError> {
    let fixed_input = fixed_world_input([0x4a; 32]);
    let fixed_packet = compile_world(&fixed_input).map_err(|_| SignalAnchorError::Dependency)?;
    if input != &fixed_input || packet != &fixed_packet {
        return Err(SignalAnchorError::Authority);
    }
    let record = fixed_sessions()
        .into_iter()
        .find(|r| r.session_id == "gp0.s4.signal-anchor")
        .ok_or(SignalAnchorError::Dependency)?;
    let grammar = fixed_encounter_grammar().map_err(|_| SignalAnchorError::Dependency)?;
    let situation = grammar
        .situations
        .iter()
        .find(|s| s.situation_id == "gp3.s4.signal-anchor")
        .ok_or(SignalAnchorError::Dependency)?;
    let approach = situation
        .approaches
        .iter()
        .find(|a| a.approach_id == "s4.approach.temporary")
        .ok_or(SignalAnchorError::Dependency)?;
    let threat = situation
        .threat_ref
        .as_ref()
        .ok_or(SignalAnchorError::Dependency)?;
    let hub = address([0x0e; 32], NodeKind::Site, b"signal-hub")?;
    let place = address([0x0e; 32], NodeKind::Site, b"signal-anchor")?;
    let player = address([0x0e; 32], NodeKind::Entity, b"signal-player")?;
    let descriptor = bind_addressable_world_package(
        place
            .logical_fingerprint()
            .map_err(|_| SignalAnchorError::Dependency)?,
        Some(
            hub.logical_fingerprint()
                .map_err(|_| SignalAnchorError::Dependency)?,
        ),
        input.reconstruction_id,
        input,
        packet,
        b"gp4-signal-anchor-place-v1".to_vec(),
    )
    .map_err(|_| SignalAnchorError::Dependency)?;
    let identity = VerticalIdentityV1::new(
        &hub,
        &place,
        &player,
        &descriptor,
        &record.session_id,
        "gp4.signal-anchor.vertical-1",
    )
    .map_err(|_| SignalAnchorError::Dependency)?;
    let initial = start_c3a_base_loop(
        &record,
        "gp4.signal-anchor.vertical-1",
        BaseLoopLedgerV1::empty(),
        input,
        packet,
    )
    .map_err(|_| SignalAnchorError::Dependency)?;
    let mut log = VerticalLogV1::initialize(identity.clone(), &initial, &record, input, packet)
        .map_err(|_| SignalAnchorError::Dependency)?;
    let batches = vec![
        vec![BaseLoopActionV1::Prepare(PreparationV1 {
            session_id: record.session_id.clone(),
            intent_id: "rescue-before-anchor-collapse".into(),
            tool_id: "temporary-brace-kit".into(),
            divert_threat: true,
        })],
        vec![
            BaseLoopActionV1::Depart,
            BaseLoopActionV1::ChooseOutcome {
                outcome_id: "s4.temporary-rescue".into(),
            },
        ],
        vec![BaseLoopActionV1::BeginReturn],
        vec![BaseLoopActionV1::RecordRememberedResponse],
    ];
    let mut command_ids = vec![];
    let mut command_rows = vec![];
    let mut states = vec![];
    let mut prefix_snapshot = None;
    let mut prefix_log_state = None;
    for (index, actions) in batches.into_iter().enumerate() {
        let sequence = index as u64 + 1;
        let revision = log.revision();
        let parent = log.head().map_err(|_| SignalAnchorError::Replay)?;
        let id = command_id(
            identity
                .fingerprint()
                .map_err(|_| SignalAnchorError::Dependency)?,
            parent,
            sequence,
            revision,
            &actions,
        )?;
        let labels = actions
            .iter()
            .map(action_label)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        command_rows.push(SignalAnchorCommandRowV1 {
            actor_player_id: identity.player_id,
            command_id: id,
            sequence,
            expected_revision: revision,
            expected_parent: parent,
            action_count: actions.len(),
            action_labels: labels,
        });
        command_ids.push(id);
        let command = VerticalCommandBatchV1 {
            actor_player_id: identity.player_id,
            command_id: id,
            expected_revision: revision,
            sequence,
            expected_parent: parent,
            actions,
        };
        let (next, state, _) = log
            .append(&record, input, packet, &command)
            .map_err(|_| SignalAnchorError::Replay)?;
        log = next;
        states.push(state);
        if sequence == 3 {
            let restarted = log
                .restart(&record, input, packet)
                .map_err(|_| SignalAnchorError::Replay)?;
            if restarted != states[2] {
                return Err(SignalAnchorError::Replay);
            }
            prefix_snapshot = Some(
                VerticalSnapshotV1::build(&log, &record, input, packet)
                    .map_err(|_| SignalAnchorError::Replay)?,
            );
            prefix_log_state = Some(log.clone());
        }
    }
    validate_signal_anchor_command_rows(&command_rows)?;
    let c3a = log
        .restart(&record, input, packet)
        .map_err(|_| SignalAnchorError::Replay)?;
    if c3a != states[3] {
        return Err(SignalAnchorError::Replay);
    }
    let final_snapshot = VerticalSnapshotV1::build(&log, &record, input, packet)
        .map_err(|_| SignalAnchorError::Replay)?;
    let prefix_restart_verified = prefix_snapshot
        .as_ref()
        .unwrap()
        .verify(prefix_log_state.as_ref().unwrap(), &record, input, packet)
        .is_ok();
    let final_restart_verified = final_snapshot.verify(&log, &record, input, packet).is_ok();
    let mut shadow = start_authored_base_loop(&record, &c3a.run_id, c3a.ledger_before.clone())
        .map_err(|_| SignalAnchorError::Replay)?;
    for action in &c3a.trace {
        shadow = apply_base_loop_action(&record, &shadow, action)
            .map_err(|_| SignalAnchorError::Replay)?;
    }
    let c3a_projection = BaseLoopSemanticProjectionV1::from_state(&c3a);
    let authored_projection = BaseLoopSemanticProjectionV1::from_state(&shadow);
    if c3a_projection != authored_projection {
        return Err(SignalAnchorError::Replay);
    }
    let c3a_before = c3a
        .ledger_before
        .to_bytes()
        .map_err(|_| SignalAnchorError::Replay)?;
    let authored_before = shadow
        .ledger_before
        .to_bytes()
        .map_err(|_| SignalAnchorError::Replay)?;
    let c3a_after = c3a
        .ledger_after
        .to_bytes()
        .map_err(|_| SignalAnchorError::Replay)?;
    let authored_after = shadow
        .ledger_after
        .to_bytes()
        .map_err(|_| SignalAnchorError::Replay)?;
    if c3a_before != authored_before || c3a_after != authored_after {
        return Err(SignalAnchorError::Replay);
    }
    let projection_bytes =
        serde_json::to_vec(&c3a_projection).map_err(|_| SignalAnchorError::Codec)?;
    let common_semantic_digest =
        framed_hash(b"mindwarp.gp4.base-loop-semantics.v1\0", &projection_bytes);
    let prior = ProgressionLedgerV1::from_base_loop(&shadow.ledger_before)
        .map_err(|_| SignalAnchorError::Dependency)?;
    let progression =
        apply_progression(&record, &shadow, &prior).map_err(|_| SignalAnchorError::Dependency)?;
    let receipt = &progression.processed_receipts[0];
    let progression_duplicate_rejected = apply_progression(&record, &shadow, &progression).is_err();
    let foreign_record = fixed_sessions()
        .into_iter()
        .find(|r| r.session_id != record.session_id)
        .ok_or(SignalAnchorError::Dependency)?;
    let progression_cross_identity_rejected =
        apply_progression(&foreign_record, &shadow, &prior).is_err();
    let authored_c4v_rejected =
        VerticalLogV1::initialize(identity.clone(), &shadow, &record, input, packet).is_err();
    let mut distrusted = log.clone();
    if let Some(first) = distrusted.final_state_bytes.first_mut() {
        *first ^= 1;
    }
    let stored_state_distrusted = distrusted.restart(&record, input, packet).is_err();
    let approach_ref = reference_digest(
        b"mindwarp.gp4.gp3-approach-ref.v1\0",
        situation.situation_id.as_bytes(),
        &serde_json::to_vec(approach).map_err(|_| SignalAnchorError::Codec)?,
    );
    let threat_ref = reference_digest(
        b"mindwarp.gp4.gp3-threat-ref.v1\0",
        situation.situation_id.as_bytes(),
        &serde_json::to_vec(threat).map_err(|_| SignalAnchorError::Codec)?,
    );
    let mut bundle = SignalAnchorBundleV1 {
        schema_version: 1,
        bundle_id: "gp4.signal-anchor.bundle-v1".into(),
        session_bytes: serde_json::to_vec(&record).map_err(|_| SignalAnchorError::Codec)?,
        c3a_input_bytes: input
            .to_bytes()
            .map_err(|_| SignalAnchorError::Dependency)?,
        c3a_packet_bytes: packet
            .to_bytes()
            .map_err(|_| SignalAnchorError::Dependency)?,
        c4v_log_bytes: log.to_bytes().map_err(|_| SignalAnchorError::Replay)?,
        return_prefix_snapshot_bytes: prefix_snapshot
            .unwrap()
            .to_bytes()
            .map_err(|_| SignalAnchorError::Replay)?,
        final_snapshot_bytes: final_snapshot
            .to_bytes()
            .map_err(|_| SignalAnchorError::Replay)?,
        persistence_receipt_bytes: persistence_receipt(&log)
            .map_err(|_| SignalAnchorError::Dependency)?
            .to_bytes()
            .map_err(|_| SignalAnchorError::Dependency)?,
        command_ids,
        authored_shadow_state_bytes: shadow
            .to_bytes(&record)
            .map_err(|_| SignalAnchorError::Replay)?,
        common_semantic_digest,
        gp3_situation_bytes: situation
            .to_bytes()
            .map_err(|_| SignalAnchorError::Dependency)?,
        gp4_approach_ref_digest: approach_ref,
        gp3_threat_digest: threat.canonical_digest.clone(),
        gp4_threat_ref_digest: threat_ref,
        threat_selected: true,
        progression_ledger_bytes: progression
            .to_bytes()
            .map_err(|_| SignalAnchorError::Dependency)?,
        presentation_slots: presentation_slots(&identity, approach_ref, threat)?,
        adapter_requirements: adapter_requirements()?,
        bundle_digest: [0; 32],
    };
    bundle.bundle_digest = digest_bundle(&bundle)?;
    let mut lane = vec![];
    lane.extend(progression.knowledge.iter().map(|x| x.record_id.clone()));
    lane.extend(
        progression
            .relationship_events
            .iter()
            .map(|x| x.event_id.clone()),
    );
    lane.extend(
        progression
            .constructions
            .iter()
            .map(|x| x.record_id.clone()),
    );
    lane.extend(progression.named_assets.iter().map(|x| x.asset_id.clone()));
    lane.extend(progression.liabilities.iter().map(|x| x.record_id.clone()));
    let inspection = SignalAnchorInspectionV1 {
        c3a_projection,
        authored_projection,
        c3a_ledger_before_bytes: c3a_before,
        authored_ledger_before_bytes: authored_before,
        c3a_ledger_after_bytes: c3a_after,
        authored_ledger_after_bytes: authored_after,
        c3a_world_is_validated: matches!(c3a.world_context, LoopWorldContextV1::ValidatedC3A(_)),
        shadow_world_is_authored: matches!(
            shadow.world_context,
            LoopWorldContextV1::AuthoredFixture
        ),
        authored_c4v_rejected,
        command_rows,
        prefix_restart_verified,
        final_restart_verified,
        stored_state_distrusted,
        situation_id: situation.situation_id.clone(),
        approach_id: approach.approach_id.clone(),
        tool_id: approach
            .prepared_tool_id
            .clone()
            .ok_or(SignalAnchorError::Dependency)?,
        risk_id: approach
            .risk_dispositions
            .first()
            .ok_or(SignalAnchorError::Dependency)?
            .risk_id
            .clone(),
        threat_id: threat.threat_id.clone(),
        outcome_id: approach.outcome_id.clone(),
        intervention_step_ids: approach
            .intervention_steps
            .iter()
            .map(|x| x.step_id.clone())
            .collect(),
        progression_rule_id: receipt.rule_id.clone(),
        progression_decision_id: receipt.opened_decision_id.clone(),
        progression_emitted_ids: receipt.emitted_record_ids.clone(),
        progression_transition_ids: receipt.world_transition_ids.clone(),
        progression_lane_ids: lane,
        progression_cross_identity_rejected,
        progression_duplicate_rejected,
        c3a_progression_rejected: apply_progression(&record, &c3a, &prior).is_err(),
    };
    Ok(Built { bundle, inspection })
}

pub fn validate_signal_anchor_command_rows(
    rows: &[SignalAnchorCommandRowV1],
) -> Result<(), SignalAnchorError> {
    let actor = hex32("f7930c4ac3776c4aa4f7400c1b1050bc03a8c4edd82358925fd874d564822e2d")?;
    let ids = [
        "6fa6a6d429003d91fb4f577486a34ff4bf174e16e659c966ccf3327e8dd2cc15",
        "287ccf55549997ecd90f3fd8fc202bd7c92be386923e1954a9324554b343fda1",
        "42a8db36764d01b533e7adf62c87fb226685471ca2b2f23ac3253cec330b9da1",
        "d3dd8df1f02284cebeb677d1136ebaa1ceb095edb3704aedf7e6fe9a57344fff",
    ];
    let parents = [
        None,
        Some(hex32(
            "b5f7b4d62d529354ae0de94469521cdef175a1ca8458dbcf0a78958bca02a66f",
        )?),
        Some(hex32(
            "95bd51fd8aa14ba9e67f9ad19193dee7ae613751c37d46973e51fe6e03a5d7e8",
        )?),
        Some(hex32(
            "66e5b1d83cd5d0ad86dab09bcef57b72350437c50662a7587830a0965c0045ea",
        )?),
    ];
    let labels = [
        vec!["Prepare"],
        vec!["Depart", "ChooseOutcome"],
        vec!["BeginReturn"],
        vec!["RecordRememberedResponse"],
    ];
    if rows.len() != 4 {
        return Err(SignalAnchorError::Replay);
    }
    for i in 0..4 {
        let r = &rows[i];
        if r.actor_player_id != actor
            || r.command_id != hex32(ids[i])?
            || r.sequence != i as u64 + 1
            || r.expected_revision != i as u64
            || r.expected_parent != parents[i]
            || r.action_count != labels[i].len()
            || r.action_labels
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                != labels[i]
        {
            return Err(SignalAnchorError::Replay);
        }
    }
    Ok(())
}

fn action_label(a: &BaseLoopActionV1) -> &'static str {
    match a {
        BaseLoopActionV1::Prepare(_) => "Prepare",
        BaseLoopActionV1::Depart => "Depart",
        BaseLoopActionV1::ChooseOutcome { .. } => "ChooseOutcome",
        BaseLoopActionV1::BeginReturn => "BeginReturn",
        BaseLoopActionV1::RecordRememberedResponse => "RecordRememberedResponse",
        _ => "Unexpected",
    }
}
fn address(
    seed: [u8; 32],
    kind: NodeKind,
    payload: &[u8],
) -> Result<UniverseAddress, SignalAnchorError> {
    UniverseAddress::new(
        seed,
        vec![AddressSegment::new(kind, payload).map_err(|_| SignalAnchorError::Dependency)?],
    )
    .map_err(|_| SignalAnchorError::Dependency)
}
fn command_id(
    identity: [u8; 32],
    parent: Option<[u8; 32]>,
    sequence: u64,
    revision: u64,
    actions: &[BaseLoopActionV1],
) -> Result<[u8; 32], SignalAnchorError> {
    let bundle = b"gp4.signal-anchor.bundle-v1";
    let run = b"gp4.signal-anchor.vertical-1";
    let action_bytes = serde_json::to_vec(actions).map_err(|_| SignalAnchorError::Codec)?;
    let mut h = Sha256::new();
    h.update(b"mindwarp.gp4.signal-anchor.command.v1\0");
    h.update(identity);
    h.update((bundle.len() as u32).to_be_bytes());
    h.update(bundle);
    h.update((run.len() as u32).to_be_bytes());
    h.update(run);
    h.update(sequence.to_be_bytes());
    h.update(revision.to_be_bytes());
    match parent {
        None => h.update([0]),
        Some(p) => {
            h.update([1]);
            h.update(p)
        }
    }
    h.update((action_bytes.len() as u64).to_be_bytes());
    h.update(action_bytes);
    Ok(h.finalize().into())
}
fn reference_digest(domain: &[u8], situation: &[u8], entity: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain);
    h.update((situation.len() as u32).to_be_bytes());
    h.update(situation);
    h.update((entity.len() as u64).to_be_bytes());
    h.update(entity);
    h.finalize().into()
}
fn framed_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain);
    h.update((bytes.len() as u64).to_be_bytes());
    h.update(bytes);
    h.finalize().into()
}
fn hex32(s: &str) -> Result<[u8; 32], SignalAnchorError> {
    if s.len() != 64 {
        return Err(SignalAnchorError::Registry);
    }
    let mut out = [0; 32];
    for i in 0..32 {
        out[i] =
            u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).map_err(|_| SignalAnchorError::Registry)?
    }
    Ok(out)
}

fn presentation_slots(
    identity: &VerticalIdentityV1,
    approach_ref: [u8; 32],
    threat: &mindwarp_gameplay_foundation::EncounterThreatRefV1,
) -> Result<Vec<SemanticPresentationSlotV1>, SignalAnchorError> {
    let sources: Vec<Vec<String>> = vec![
        vec![
            format!("c2.hub.{}", hex(identity.hub_id)),
            "gp0.s4.signal-anchor:caravan-leader".into(),
        ],
        vec![
            format!("c2.player.{}", hex(identity.player_id)),
            "player".into(),
        ],
        vec!["gp0.s4.signal-anchor:problem:iven-stranded".into()],
        vec![
            "gp0.s4.signal-anchor:problem".into(),
            "gp0.s4.signal-anchor:core-tension".into(),
        ],
        vec!["gp0.s4.signal-anchor:problem:anchor-broken".into()],
        vec!["s4.timing".into()],
        vec!["wire-scavengers".into(), threat.canonical_digest.clone()],
        vec!["anchor-collapse".into()],
        vec!["temporary-brace-kit".into()],
        vec!["s4.approach.temporary".into(), hex(approach_ref)],
        vec![
            "s4.approach.temporary.step.1".into(),
            "s4.approach.temporary.step.2".into(),
        ],
        vec!["wire-scavengers:mutation.0".into()],
        vec!["s4.temporary-rescue:mutation.0".into()],
        vec!["s4.temporary-rescue:mutation.1".into()],
        vec!["s4.temporary-rescue:mutation.2".into()],
        vec!["s4.temporary-rescue:mutation.3".into()],
        vec!["s4.temporary-rescue:mutation.4".into()],
        vec!["s4.temporary-rescue:mutation.5".into()],
        vec![
            "s4.temporary-rescue:opportunity_cost.0".into(),
            "liability.s4-temporary-rescue.0".into(),
        ],
        vec![
            "s4.temporary-rescue:memory.0".into(),
            "c4v.revision.4.ledger_after.history".into(),
        ],
        vec![
            "s4.temporary-rescue:named_decision.0".into(),
            "s4.rescue-next".into(),
        ],
        vec!["c4v.revision.1.stable_stop".into()],
        vec!["c4v.revision.2.stable_stop".into()],
        vec!["c4v.revision.3.stable_stop".into()],
        vec!["c4v.revision.4.stable_stop".into()],
    ];
    let data="hub-status|Fixed hub frame: the caravan leader is waiting for a safe crossing.|square hub marker
player-actor|The player is the sole actor for this vertical.|solid actor ring
iven-absent|Iven is stranded beyond the broken anchor.|empty person outline
signal-anchor-opportunity|Rescue, signal evidence and permanent repair cannot all be completed in the window.|three-way fork glyph
anchor-broken-state|The Signal Anchor is broken.|split anchor shape
signal-window-evidence|The signal window is three actions; permanent repair needs four and a temporary brace needs two.|three ticks beside four ticks
wire-scavenger-threat|Wire scavengers block the work area but cannot resolve the rescue.|toothed obstacle outline
anchor-collapse-risk|Loading the failed anchor can cause collapse.|descending crack chevron
temporary-brace-tool|Prepared tool: temporary brace kit.|brace tool silhouette
temporary-rescue-choice|Choose temporary rescue and signal capture.|selected fork notch
temporary-brace-intervention|Fit the brace, cross once, return Iven and record the signal.|two numbered step blocks
work-area-safe|The diverted work area is safe; this is world-only threat evidence.|cleared obstacle outline
anchor-brace-temporary|The anchor brace is temporary.|temporary brace hatch
temporary-crossing|One crossing was completed.|single crossing bar
iven-returned|Iven returned.|filled person outline
signal-coordinate-recorded|The signal coordinate was recorded.|pinned signal cross
caravan-delayed|The caravan remains delayed.|paused caravan bars
brace-expired|The temporary brace expired on return.|crossed brace outline
permanent-repair-incomplete|Permanent anchor repair was not completed.|open repair bracket
remembered-response|Iven remembers that rescue and evidence were chosen over permanent repair.|memory knot
next-decision|Next decision: pursue the signal or return with a permanent repair crew.|two-arrow decision fork
rev1-prepared-stop|Stable stop after preparation; depart is next.|stop marker one
rev2-consequence-stop|Stable stop after consequence; begin return is next.|stop marker two
rev3-return-prefix|Restarted stable return prefix; record remembered response is next.|stop marker three
rev4-terminal|Final restarted terminal remembered response.|terminal stop marker";
    let rows = data.lines().collect::<Vec<_>>();
    if rows.len() != sources.len() {
        return Err(SignalAnchorError::Registry);
    }
    let mut out = vec![];
    for (line, ids) in rows.into_iter().zip(sources) {
        let p = line.split('|').collect::<Vec<_>>();
        let id = p[0].to_owned();
        let text = p[1].to_owned();
        out.push(SemanticPresentationSlotV1 {
            slot_id: id.clone(),
            source_id_list_digest: source_digest(&ids),
            source_ids: ids,
            text_equivalent: text.clone(),
            non_color_cue: p[2].into(),
            reduced_motion_equivalent: text.clone(),
            screen_reader_label: format!("{id}: {text}"),
        });
    }
    Ok(out)
}

fn source_digest(ids: &[String]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"mindwarp.gp4.presentation-source.v1\0");
    h.update((ids.len() as u32).to_be_bytes());
    for id in ids {
        h.update((id.len() as u32).to_be_bytes());
        h.update(id.as_bytes())
    }
    h.finalize().into()
}
fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

fn adapter_requirements() -> Result<Vec<AdapterRequirementV1>, SignalAnchorError> {
    let data="hard.strict-bundle-roundtrip|Hard|Does the adapter preserve strict bundle bytes and every digest?|canonical encode/decode and hostile codec receipt|byte comparison|pass required
hard.exact-dependency-digests|Hard|Does the adapter authenticate every pinned dependency digest?|C3A GP3 GP2 and C4V digest comparison receipt|fixed-vector comparison|pass required
hard.c2-c3a-identity|Hard|Does the adapter preserve exact C2 identity and C3A world authority?|identity and validated packet binding receipt|typed authority replay|pass required
hard.gp1-action-stable-order|Hard|Does the adapter preserve five GP1 actions in four stable C4V batches?|ordered action and stable-stop trace|deterministic trace comparison|pass required
hard.gp3-approach-evidence-risk|Hard|Does the adapter preserve the exact GP3 approach evidence risk and threat mapping?|fixed situation approach risk and threat receipt|GP3 registry resolution|pass required
hard.c4v-append-restart|Hard|Does the adapter preserve C4V append prefix restart and final restart semantics?|revision three and revision four replay receipts|semantic restart comparison|pass required
hard.gp2-authored-shadow-isolation|Hard|Is GP2 restricted to the authority-lowering authored shadow?|rejected C3A GP2 attempt and accepted shadow receipt|authority-negative test|pass required
hard.no-duplicate-memory-progression|Hard|Are memory and progression records emitted exactly once?|exact receipt emission and history cardinality|set and order comparison|pass required
hard.semantic-slot-coverage|Hard|Are all twenty-five decision-relevant semantic slots present?|exact fixed slot registry comparison|typed row equality|pass required
hard.accessibility-equivalence|Hard|Do text non-colour reduced-motion and screen-reader forms preserve each slot meaning?|per-slot equivalence review receipt|semantic equivalence review|pass required
hard.no-canonical-mutation|Hard|Does adapter execution leave canonical Forge and gameplay records unchanged?|before and after canonical hashes|mutation-negative comparison|pass required
hard.no-ambient-authority|Hard|Does the adapter avoid filesystem network process and hidden runtime authority?|capability and side-effect denial receipt|containment audit|pass required
hard.headless-deterministic-tests|Hard|Does the complete vertical replay byte-identically without presentation?|repeated isolated headless receipts|clean-process replay|pass required
hard.clean-target-build|Hard|Does the adapter build from a clean isolated target?|clean target build receipt|isolated build|pass required
hard.runtime-provenance-licensing|Hard|Are runtime and dependency provenance and licenses acceptable?|source license and dependency inventory|provenance review|owner approval required
hard.containment-teardown|Hard|Can the runtime trial be contained stopped and removed without residue?|launch boundary and teardown receipt|containment exercise|pass required
compare.cold-build-import|Compare|What is cold build and initial import cost?|measured clean build and import trace|timed clean trial|owner-set after measurement
compare.incremental-iteration|Compare|What is edit to verified incremental iteration cost?|measured incremental build and test trace|timed incremental trial|owner-set after measurement
compare.bundle-validation-restart-latency|Compare|What are bundle validation prefix restart and final restart latencies?|measured validation and both restart traces|monotonic timing|owner-set after measurement
compare.input-semantic-feedback-latency|Compare|What is input to semantic feedback latency?|measured input and semantic projection timestamps|event trace timing|owner-set after measurement
compare.cpu-gpu-frame-pacing|Compare|What CPU GPU and frame pacing cost does presentation add?|measured CPU GPU and frame pacing trace|representative scene profile|owner-set after measurement
compare.peak-steady-memory|Compare|What are peak and steady memory use?|measured peak and steady allocation trace|memory profile|owner-set after measurement
compare.binary-asset-project-size|Compare|What binary asset and project size does the adapter add?|measured clean artifact inventory|size inventory|owner-set after measurement
compare.mobile-battery-thermal|Compare|What mobile battery and thermal cost occurs?|measured supported-device battery and thermal trace|bounded device trial|owner-set after measurement
compare.adapter-dependency-surface|Compare|How large is the adapter and dependency surface?|counted public adapter and dependency inventory|interface inventory|owner-set after measurement
compare.debugging-profiling|Compare|How effective are debugging and profiling workflows?|timed fault isolation and profile exercise|controlled defect exercise|owner-set after measurement
compare.platform-export-coverage|Compare|Which target exports pass the exact vertical?|per-target build run and replay receipts|platform matrix|owner-set after measurement
compare.upgrade-maintenance-risk|Compare|What upgrade and maintenance risk is observed?|dependency update and migration exercise|bounded upgrade trial|owner-set after measurement
compare.owner-play-comprehension|Compare|Does the owner understand and enjoy the fixed vertical?|explicit owner-authored play observation|bounded owner play check|owner decision required";
    let mut out = vec![];
    for line in data.lines() {
        let p = line.split('|').collect::<Vec<_>>();
        if p.len() != 6 {
            return Err(SignalAnchorError::Registry);
        }
        out.push(AdapterRequirementV1 {
            requirement_id: p[0].into(),
            class: if p[1] == "Hard" {
                AdapterRequirementClassV1::Hard
            } else {
                AdapterRequirementClassV1::Compare
            },
            status: AdapterRequirementStatusV1::Unmeasured,
            question: p[2].into(),
            required_evidence: p[3].into(),
            method: p[4].into(),
            target: p[5].into(),
        })
    }
    Ok(out)
}

fn fixed_world_input(r: [u8; 32]) -> WorldGenerationInput {
    WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id: r,
        surface_material: surface_contract(r),
        regional_environment: regional_contract(r),
        signal_potentials: vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }],
    }
}
fn regional_contract(r: [u8; 32]) -> RegionalEnvironmentContract {
    derived_world_rules::compile_regional_environment(&RegionalEnvironmentInput {
        schema_version: 1,
        reconstruction_id: r,
        regional_source_id: [8; 32],
        field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(ONE)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        moisture_source_id: [9; 32],
        moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        coordinate_q32_32: [0, 0],
    })
    .unwrap()
}
fn stellar_contract(r: [u8; 32]) -> StellarOrbitalContract {
    derived_world_rules::compile_stellar_orbital(&StellarOrbitalInput {
        schema_version: 1,
        reconstruction_id: r,
        stellar_source_id: [3; 32],
        primary_mass_milli_solar: 1000,
        stellar_luminosity_millionths_solar: 1_000_000,
        stellar_spectrum_rgb_permille: [400, 350, 250],
        semi_major_axis_milli_au: 1000,
        eccentricity_millionths: 0,
    })
    .unwrap()
}
fn geological_contract(r: [u8; 32]) -> GeologicalAtmosphericContract {
    derived_world_rules::compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id: r,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar_contract(r),
        planet_mass_milli_earth: 1000,
        planet_radius_milli_earth: 1000,
        internal_heat_flux_milli_w_m2: 87,
        solid_surface_fraction_permille: 600,
        atmospheric_column_mass_g_m2: 10_332_000,
        gas_transmission_rgb_permille: [800, 900, 950],
        aerosol_transmission_rgb_permille: [1000; 3],
    })
    .unwrap()
}
fn hydrological_contract(r: [u8; 32]) -> HydrologicalContract {
    derived_world_rules::compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id: r,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological_contract(r),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}
fn climate_contract(r: [u8; 32]) -> ClimateContract {
    derived_world_rules::compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id: r,
        climate_source_id: [6; 32],
        hydrological: hydrological_contract(r),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}
fn surface_contract(r: [u8; 32]) -> derived_world_rules::SurfaceMaterialContract {
    derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
        schema_version: 1,
        reconstruction_id: r,
        material_source_id: [7; 32],
        climate: climate_contract(r),
        dominant_surface_reflectance_rgb_permille: [500, 400, 300],
    })
    .unwrap()
}
