//! Engine-neutral C4V stable-stop persistence reference proof.

use addressable_world_binding::{
    bind_addressable_world_package, world_conditions_contract_fingerprint,
    world_conditions_packet_fingerprint,
};
use derived_world_rules::{CausalWorldPacket, WorldGenerationInput};
use hierarchy_history::{
    AppendOutcome, BaselineManifest, DeltaEnvelope, DependencyRef, HierarchyDescriptor,
    HistoryStream,
};
use mindwarp_gameplay_foundation::{
    BaseLoopActionV1, BaseLoopStateV1, LoopWorldContextV1, SessionRecordV1, apply_base_loop_action,
    bind_validated_c3a_world,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use universe_identity::{NodeKind, UniverseAddress};

pub const MAX_EVENTS: usize = 256;
pub const MAX_ACTIONS: usize = 2;
pub const MAX_ACTION_BYTES: usize = 8 * 1024;
pub const MAX_COMMAND_BYTES: usize = 16 * 1024;
pub const MAX_STATE_BYTES: usize = 256 * 1024;
pub const MAX_LOG_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_SNAPSHOT_BYTES: usize = 512 * 1024;
pub const MAX_RECEIPT_BYTES: usize = 64 * 1024;

const GP1_CONTRACT: &[u8] = b"mindwarp/gp1/base-loop-contract/v1\0";
const CODEC_V1: &[u8] = b"mindwarp/c4v/codec/v1\0";
const CODEC_V2: &[u8] = b"mindwarp/c4v/codec/v2\0";
const OPERATION_SCHEMA: &[u8] = b"mindwarp/c4v/vertical-command-operation-schema/v1\0";
const IDENTITY_DOMAIN: &[u8] = b"mindwarp/c4v/identity/v1\0";
const ENCOUNTER_DOMAIN: &[u8] = b"mindwarp/c4v/encounter-id/v1\0";
const CONSEQUENCE_DOMAIN: &[u8] = b"mindwarp/c4v/consequence/v1\0";
const LOG_DOMAIN: &[u8] = b"mindwarp/c4v/log/v1\0";
const SNAPSHOT_DOMAIN: &[u8] = b"mindwarp/c4v/snapshot/v1\0";
const MIGRATION_DOMAIN: &[u8] = b"mindwarp/c4v/migration/v1\0";
const RECEIPT_DOMAIN: &[u8] = b"mindwarp/c4v/receipt/v1\0";
const ADAPTER_DOMAIN: &[u8] = b"mindwarp/c4v/v1-to-v2-adapter/v1\0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerticalError {
    Invalid(&'static str),
    RetryConflict,
    Stale,
    Gap,
    Fork,
    Terminal,
    Codec,
    Upstream,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalIdentityV1 {
    pub hub_address_bytes: Vec<u8>,
    pub place_address_bytes: Vec<u8>,
    pub player_address_bytes: Vec<u8>,
    pub descriptor_bytes: Vec<u8>,
    pub hub_id: [u8; 32],
    pub place_id: [u8; 32],
    pub player_id: [u8; 32],
    pub encounter_id: [u8; 32],
    pub session_id: String,
    pub run_id: String,
}

impl VerticalIdentityV1 {
    pub fn new(
        hub: &UniverseAddress,
        place: &UniverseAddress,
        player: &UniverseAddress,
        descriptor: &HierarchyDescriptor,
        session_id: &str,
        run_id: &str,
    ) -> Result<Self, VerticalError> {
        validate_id(session_id)?;
        validate_id(run_id)?;
        let hub_id = hub
            .logical_fingerprint()
            .map_err(|_| VerticalError::Upstream)?;
        let place_id = place
            .logical_fingerprint()
            .map_err(|_| VerticalError::Upstream)?;
        let player_id = player
            .logical_fingerprint()
            .map_err(|_| VerticalError::Upstream)?;
        if hub.path.last().map(|item| item.kind) != Some(NodeKind::Site)
            || place.path.last().map(|item| item.kind) != Some(NodeKind::Site)
            || player.path.last().map(|item| item.kind) != Some(NodeKind::Entity)
            || hub.universe_seed != place.universe_seed
            || hub.universe_seed != player.universe_seed
            || hub_id == place_id
            || hub_id == player_id
            || place_id == player_id
            || descriptor.logical_id != place_id
        {
            return Err(VerticalError::Invalid("crossed vertical identity"));
        }
        let gp1 = domain_hash(GP1_CONTRACT, &[]);
        let encounter_id = framed_hash(
            ENCOUNTER_DOMAIN,
            &[
                &hub_id,
                &place_id,
                &player_id,
                session_id.as_bytes(),
                run_id.as_bytes(),
                &gp1,
            ],
        );
        let identity = Self {
            hub_address_bytes: hub
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
            place_address_bytes: place
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
            player_address_bytes: player
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
            descriptor_bytes: descriptor
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
            hub_id,
            place_id,
            player_id,
            encounter_id,
            session_id: session_id.into(),
            run_id: run_id.into(),
        };
        Ok(identity)
    }

    pub fn validate(&self) -> Result<(), VerticalError> {
        let hub = UniverseAddress::decode_canonical(&self.hub_address_bytes)
            .map_err(|_| VerticalError::Invalid("hub address"))?;
        let place = UniverseAddress::decode_canonical(&self.place_address_bytes)
            .map_err(|_| VerticalError::Invalid("place address"))?;
        let player = UniverseAddress::decode_canonical(&self.player_address_bytes)
            .map_err(|_| VerticalError::Invalid("player address"))?;
        let descriptor = HierarchyDescriptor::decode_strict(&self.descriptor_bytes)
            .map_err(|_| VerticalError::Invalid("place descriptor"))?;
        if Self::new(
            &hub,
            &place,
            &player,
            &descriptor,
            &self.session_id,
            &self.run_id,
        )? != *self
        {
            return Err(VerticalError::Invalid("vertical identity mismatch"));
        }
        Ok(())
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], VerticalError> {
        Ok(framed_hash(IDENTITY_DOMAIN, &[&canonical(self)?]))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        self.validate()?;
        canonical(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VerticalError> {
        let value: Self = strict_json(bytes)?;
        value.validate()?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalCommandBatchV1 {
    pub actor_player_id: [u8; 32],
    pub command_id: [u8; 32],
    pub expected_revision: u64,
    pub sequence: u64,
    pub expected_parent: Option<[u8; 32]>,
    pub actions: Vec<BaseLoopActionV1>,
}

impl VerticalCommandBatchV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        if self.actions.is_empty() || self.actions.len() > MAX_ACTIONS || self.sequence == 0 {
            return Err(VerticalError::Invalid("command shape"));
        }
        for action in &self.actions {
            if canonical(action)?.len() > MAX_ACTION_BYTES {
                return Err(VerticalError::Invalid("action byte bound"));
            }
        }
        let bytes = canonical(self)?;
        if bytes.len() > MAX_COMMAND_BYTES {
            return Err(VerticalError::Invalid("command byte bound"));
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VerticalError> {
        if bytes.len() > MAX_COMMAND_BYTES {
            return Err(VerticalError::Invalid("command byte bound"));
        }
        let value: Self = strict_json(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(VerticalError::Codec);
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredEventV1 {
    command_bytes: Vec<u8>,
    consequence_bytes: Vec<u8>,
    consequence_hash: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalLogV1 {
    pub schema_version: u16,
    pub codec_version: u16,
    pub identity: VerticalIdentityV1,
    pub baseline_bytes: Vec<u8>,
    pub initial_state_bytes: Vec<u8>,
    pub delta_bytes: Vec<Vec<u8>>,
    pub final_state_bytes: Vec<u8>,
    pub log_hash: [u8; 32],
}

impl VerticalLogV1 {
    pub fn initialize(
        identity: VerticalIdentityV1,
        initial_state: &BaseLoopStateV1,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<Self, VerticalError> {
        identity.validate()?;
        validate_authority(&identity, initial_state, record, input, packet)?;
        if initial_state.stable_stop.resume_action.is_none() || initial_state.stable_stop.terminal {
            return Err(VerticalError::Invalid(
                "initial state is not resumable stable stop",
            ));
        }
        let initial_state_bytes = initial_state
            .to_bytes(record)
            .map_err(|_| VerticalError::Upstream)?;
        if initial_state_bytes.len() > MAX_STATE_BYTES {
            return Err(VerticalError::Invalid("initial state byte bound"));
        }
        let descriptor = HierarchyDescriptor::decode_strict(&identity.descriptor_bytes)
            .map_err(|_| VerticalError::Upstream)?;
        let baseline = build_baseline(&identity, &descriptor, packet, &initial_state_bytes, 1)?;
        let mut value = Self {
            schema_version: 1,
            codec_version: 1,
            identity,
            baseline_bytes: baseline
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
            initial_state_bytes: initial_state_bytes.clone(),
            delta_bytes: vec![],
            final_state_bytes: initial_state_bytes,
            log_hash: [0; 32],
        };
        value.log_hash = value.compute_hash()?;
        Ok(value)
    }

    pub fn revision(&self) -> u64 {
        self.delta_bytes.len() as u64
    }
    pub fn head(&self) -> Result<Option<[u8; 32]>, VerticalError> {
        Ok(self.stream()?.head())
    }

    pub fn append(
        &self,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
        command: &VerticalCommandBatchV1,
    ) -> Result<(Self, BaseLoopStateV1, AppendOutcome), VerticalError> {
        self.validate_static(record, input, packet)?;
        self.validate_semantics(record, input, packet)?;
        if command.actor_player_id != self.identity.player_id {
            return Err(VerticalError::Invalid("actor mismatch"));
        }
        let command_bytes = command.to_bytes()?;
        for delta_bytes in &self.delta_bytes {
            let delta =
                DeltaEnvelope::decode_strict(delta_bytes).map_err(|_| VerticalError::Codec)?;
            if delta.command_id == command.command_id {
                if delta.operation_schema != operation_schema() {
                    return Err(VerticalError::Invalid("operation schema"));
                }
                let stored = decode_stored(&delta.operation)?;
                if framed_hash(CONSEQUENCE_DOMAIN, &[&stored.consequence_bytes])
                    != stored.consequence_hash
                {
                    return Err(VerticalError::Invalid("retry consequence hash"));
                }
                if stored.command_bytes == command_bytes {
                    let state = decode_state(record, input, packet, &stored.consequence_bytes)?;
                    return Ok((
                        self.clone(),
                        state,
                        AppendOutcome::Idempotent(delta.content_id),
                    ));
                }
                return Err(VerticalError::RetryConflict);
            }
        }
        if command.expected_revision != self.revision() {
            return Err(VerticalError::Stale);
        }
        if command.sequence > self.revision() + 1 {
            return Err(VerticalError::Gap);
        }
        if command.sequence < self.revision() + 1 {
            return Err(VerticalError::Stale);
        }
        if command.expected_parent != self.head()? {
            return Err(VerticalError::Fork);
        }
        if self.revision() as usize >= MAX_EVENTS {
            return Err(VerticalError::Invalid("event bound"));
        }
        let current = decode_state(record, input, packet, &self.final_state_bytes)?;
        if current.stable_stop.terminal {
            return Err(VerticalError::Terminal);
        }
        validate_batch_actions(&command.actions)?;
        let mut next = current;
        for action in &command.actions {
            next = apply_base_loop_action(record, &next, action)
                .map_err(|_| VerticalError::Upstream)?;
        }
        if next.stable_stop.resume_action.is_none() && !next.stable_stop.terminal {
            return Err(VerticalError::Invalid("batch ends outside stable stop"));
        }
        let consequence_bytes = next.to_bytes(record).map_err(|_| VerticalError::Upstream)?;
        if consequence_bytes.len() > MAX_STATE_BYTES {
            return Err(VerticalError::Invalid("consequence byte bound"));
        }
        let stored = StoredEventV1 {
            command_bytes,
            consequence_hash: framed_hash(CONSEQUENCE_DOMAIN, &[&consequence_bytes]),
            consequence_bytes: consequence_bytes.clone(),
        };
        let baseline = BaselineManifest::decode_strict(&self.baseline_bytes)
            .map_err(|_| VerticalError::Codec)?;
        let mut stream = self.stream()?;
        let delta = DeltaEnvelope::new(
            baseline.key().map_err(|_| VerticalError::Upstream)?,
            self.identity.place_id,
            command.sequence,
            command.expected_parent,
            command.command_id,
            operation_schema(),
            canonical(&stored)?,
        )
        .map_err(map_history)?;
        let outcome = stream.append(delta.clone()).map_err(map_history)?;
        let mut next_log = self.clone();
        next_log.delta_bytes.push(
            delta
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?,
        );
        next_log.final_state_bytes = consequence_bytes;
        next_log.log_hash = next_log.compute_hash()?;
        if canonical(&next_log)?.len() > MAX_LOG_BYTES {
            return Err(VerticalError::Invalid("log byte bound"));
        }
        Ok((next_log, next, outcome))
    }

    pub fn restart(
        &self,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<BaseLoopStateV1, VerticalError> {
        self.validate_static(record, input, packet)?;
        self.validate_semantics(record, input, packet)?;
        decode_state(record, input, packet, &self.final_state_bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        self.validate_shape()?;
        let bytes = canonical(self)?;
        if bytes.len() > MAX_LOG_BYTES {
            return Err(VerticalError::Invalid("log byte bound"));
        }
        Ok(bytes)
    }

    pub fn from_bytes(
        bytes: &[u8],
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<Self, VerticalError> {
        if bytes.len() > MAX_LOG_BYTES {
            return Err(VerticalError::Invalid("log byte bound"));
        }
        let value: Self = strict_json(bytes)?;
        value.restart(record, input, packet)?;
        if value.to_bytes()? != bytes {
            return Err(VerticalError::Codec);
        }
        Ok(value)
    }

    fn validate_shape(&self) -> Result<(), VerticalError> {
        if self.schema_version != 1
            || !matches!(self.codec_version, 1 | 2)
            || self.delta_bytes.len() > MAX_EVENTS
            || self.initial_state_bytes.len() > MAX_STATE_BYTES
            || self.final_state_bytes.len() > MAX_STATE_BYTES
        {
            return Err(VerticalError::Invalid("log shape"));
        }
        self.identity.validate()?;
        if self.compute_hash()? != self.log_hash {
            return Err(VerticalError::Invalid("log hash"));
        }
        Ok(())
    }

    fn validate_static(
        &self,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<(), VerticalError> {
        self.validate_shape()?;
        if record.session_id != self.identity.session_id {
            return Err(VerticalError::Invalid("session mismatch"));
        }
        let initial = decode_state(record, input, packet, &self.initial_state_bytes)?;
        validate_authority(&self.identity, &initial, record, input, packet)?;
        let descriptor = HierarchyDescriptor::decode_strict(&self.identity.descriptor_bytes)
            .map_err(|_| VerticalError::Codec)?;
        let expected = build_baseline(
            &self.identity,
            &descriptor,
            packet,
            &self.initial_state_bytes,
            self.codec_version,
        )?;
        if expected
            .encode_canonical()
            .map_err(|_| VerticalError::Upstream)?
            != self.baseline_bytes
        {
            return Err(VerticalError::Invalid("baseline mismatch"));
        }
        // This is deliberately part of static validation, before retry lookup.
        // A retry may be idempotent only against a fully retained, baseline- and
        // target-bound history; a rehashed crossed log must never return an old
        // consequence.
        self.stream()?;
        Ok(())
    }

    fn stream(&self) -> Result<HistoryStream, VerticalError> {
        let baseline = BaselineManifest::decode_strict(&self.baseline_bytes)
            .map_err(|_| VerticalError::Codec)?;
        let mut stream = HistoryStream::new(baseline).map_err(|_| VerticalError::Upstream)?;
        for bytes in &self.delta_bytes {
            let delta = DeltaEnvelope::decode_strict(bytes).map_err(|_| VerticalError::Codec)?;
            if delta.operation_schema != operation_schema() {
                return Err(VerticalError::Invalid("operation schema"));
            }
            stream.append(delta).map_err(map_history)?;
        }
        Ok(stream)
    }

    fn validate_semantics(
        &self,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<(), VerticalError> {
        let mut state = decode_state(record, input, packet, &self.initial_state_bytes)?;
        validate_authority(&self.identity, &state, record, input, packet)?;
        let baseline_key = BaselineManifest::decode_strict(&self.baseline_bytes)
            .map_err(|_| VerticalError::Codec)?
            .key()
            .map_err(|_| VerticalError::Upstream)?;
        let mut parent = None;
        for (index, delta_bytes) in self.delta_bytes.iter().enumerate() {
            let delta =
                DeltaEnvelope::decode_strict(delta_bytes).map_err(|_| VerticalError::Codec)?;
            let stored = decode_stored(&delta.operation)?;
            let command = VerticalCommandBatchV1::from_bytes(&stored.command_bytes)?;
            if command.actor_player_id != self.identity.player_id
                || command.expected_revision != index as u64
                || command.sequence != index as u64 + 1
                || command.expected_parent != parent
            {
                return Err(VerticalError::Invalid("retained command authority"));
            }
            if state.stable_stop.terminal {
                return Err(VerticalError::Invalid("retained command after terminal"));
            }
            validate_batch_actions(&command.actions)?;
            let mut consequence = state;
            for action in &command.actions {
                consequence = apply_base_loop_action(record, &consequence, action)
                    .map_err(|_| VerticalError::Upstream)?;
            }
            if consequence.stable_stop.resume_action.is_none() && !consequence.stable_stop.terminal
            {
                return Err(VerticalError::Invalid("retained batch outside stable stop"));
            }
            let consequence_bytes = consequence
                .to_bytes(record)
                .map_err(|_| VerticalError::Upstream)?;
            if consequence_bytes != stored.consequence_bytes {
                return Err(VerticalError::Invalid("retained consequence mismatch"));
            }
            let expected = DeltaEnvelope::new(
                baseline_key,
                self.identity.place_id,
                command.sequence,
                command.expected_parent,
                command.command_id,
                operation_schema(),
                canonical(&stored)?,
            )
            .map_err(map_history)?;
            if expected
                .encode_canonical()
                .map_err(|_| VerticalError::Upstream)?
                != *delta_bytes
            {
                return Err(VerticalError::Invalid("retained delta mismatch"));
            }
            parent = Some(expected.content_id);
            state = consequence;
        }
        if state
            .to_bytes(record)
            .map_err(|_| VerticalError::Upstream)?
            != self.final_state_bytes
        {
            return Err(VerticalError::Invalid("retained final state mismatch"));
        }
        Ok(())
    }

    fn compute_hash(&self) -> Result<[u8; 32], VerticalError> {
        let mut copy = self.clone();
        copy.log_hash = [0; 32];
        Ok(framed_hash(LOG_DOMAIN, &[&canonical(&copy)?]))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalSnapshotV1 {
    pub baseline_key: [u8; 32],
    pub identity_hash: [u8; 32],
    pub head: Option<[u8; 32]>,
    pub revision: u64,
    pub reducer_fingerprint: [u8; 32],
    pub codec_fingerprint: [u8; 32],
    pub state_bytes: Vec<u8>,
    pub state_hash: [u8; 32],
    pub content_id: [u8; 32],
}

impl VerticalSnapshotV1 {
    pub fn build(
        log: &VerticalLogV1,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<Self, VerticalError> {
        let state = log.restart(record, input, packet)?;
        let state_bytes = state
            .to_bytes(record)
            .map_err(|_| VerticalError::Upstream)?;
        let mut value = Self {
            baseline_key: BaselineManifest::decode_strict(&log.baseline_bytes)
                .map_err(|_| VerticalError::Codec)?
                .key()
                .map_err(|_| VerticalError::Upstream)?,
            identity_hash: log.identity.fingerprint()?,
            head: log.head()?,
            revision: log.revision(),
            reducer_fingerprint: domain_hash(GP1_CONTRACT, &[]),
            codec_fingerprint: codec_fingerprint(log.codec_version)?,
            state_hash: framed_hash(CONSEQUENCE_DOMAIN, &[&state_bytes]),
            state_bytes,
            content_id: [0; 32],
        };
        value.content_id = value.hash()?;
        if canonical(&value)?.len() > MAX_SNAPSHOT_BYTES {
            return Err(VerticalError::Invalid("snapshot byte bound"));
        }
        Ok(value)
    }
    pub fn verify(
        &self,
        log: &VerticalLogV1,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<(), VerticalError> {
        if Self::build(log, record, input, packet)? != *self {
            return Err(VerticalError::Invalid("snapshot mismatch"));
        }
        Ok(())
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        let bytes = canonical(self)?;
        if bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(VerticalError::Invalid("snapshot byte bound"));
        }
        if self.hash()? != self.content_id {
            return Err(VerticalError::Invalid("snapshot content id"));
        }
        Ok(bytes)
    }
    pub fn from_bytes(
        bytes: &[u8],
        log: &VerticalLogV1,
        record: &SessionRecordV1,
        input: &WorldGenerationInput,
        packet: &CausalWorldPacket,
    ) -> Result<Self, VerticalError> {
        if bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(VerticalError::Invalid("snapshot byte bound"));
        }
        let value: Self = strict_json(bytes)?;
        value.verify(log, record, input, packet)?;
        if value.to_bytes()? != bytes {
            return Err(VerticalError::Codec);
        }
        Ok(value)
    }
    fn hash(&self) -> Result<[u8; 32], VerticalError> {
        let mut copy = self.clone();
        copy.content_id = [0; 32];
        Ok(framed_hash(SNAPSHOT_DOMAIN, &[&canonical(&copy)?]))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalMigrationReceiptV1 {
    pub from_baseline: [u8; 32],
    pub to_baseline: [u8; 32],
    pub source_head: Option<[u8; 32]>,
    pub target_head: Option<[u8; 32]>,
    pub adapter_id: [u8; 32],
    pub command_ids: Vec<[u8; 32]>,
    pub final_state_hash: [u8; 32],
    pub content_id: [u8; 32],
}

fn migrate_log_v1_to_v2(
    log: &VerticalLogV1,
    record: &SessionRecordV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<(VerticalLogV1, VerticalMigrationReceiptV1), VerticalError> {
    if log.codec_version != 1 {
        return Err(VerticalError::Invalid("migration source codec"));
    }
    let final_state = log.restart(record, input, packet)?;
    let descriptor = HierarchyDescriptor::decode_strict(&log.identity.descriptor_bytes)
        .map_err(|_| VerticalError::Codec)?;
    let target_baseline = build_baseline(
        &log.identity,
        &descriptor,
        packet,
        &log.initial_state_bytes,
        2,
    )?;
    let mut target = VerticalLogV1 {
        schema_version: 1,
        codec_version: 2,
        identity: log.identity.clone(),
        baseline_bytes: target_baseline
            .encode_canonical()
            .map_err(|_| VerticalError::Upstream)?,
        initial_state_bytes: log.initial_state_bytes.clone(),
        delta_bytes: vec![],
        final_state_bytes: log.initial_state_bytes.clone(),
        log_hash: [0; 32],
    };
    target.log_hash = target.compute_hash()?;
    let mut ids = vec![];
    for bytes in &log.delta_bytes {
        let delta = DeltaEnvelope::decode_strict(bytes).map_err(|_| VerticalError::Codec)?;
        let stored = decode_stored(&delta.operation)?;
        let mut command: VerticalCommandBatchV1 = strict_json(&stored.command_bytes)?;
        command.expected_parent = target.head()?;
        let (next, _, _) = target.append(record, input, packet, &command)?;
        ids.push(command.command_id);
        target = next;
    }
    if target.final_state_bytes
        != final_state
            .to_bytes(record)
            .map_err(|_| VerticalError::Upstream)?
    {
        return Err(VerticalError::Invalid("migration semantic drift"));
    }
    let mut receipt = VerticalMigrationReceiptV1 {
        from_baseline: BaselineManifest::decode_strict(&log.baseline_bytes)
            .map_err(|_| VerticalError::Codec)?
            .key()
            .map_err(|_| VerticalError::Upstream)?,
        to_baseline: target_baseline.key().map_err(|_| VerticalError::Upstream)?,
        source_head: log.head()?,
        target_head: target.head()?,
        adapter_id: domain_hash(ADAPTER_DOMAIN, &[]),
        command_ids: ids,
        final_state_hash: framed_hash(CONSEQUENCE_DOMAIN, &[&target.final_state_bytes]),
        content_id: [0; 32],
    };
    receipt.content_id = framed_hash(MIGRATION_DOMAIN, &[&canonical(&receipt)?]);
    if canonical(&receipt)?.len() > MAX_RECEIPT_BYTES {
        return Err(VerticalError::Invalid("migration receipt bound"));
    }
    Ok((target, receipt))
}

impl VerticalMigrationReceiptV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        let bytes = canonical(self)?;
        if bytes.len() > MAX_RECEIPT_BYTES {
            return Err(VerticalError::Invalid("migration receipt bound"));
        }
        let mut copy = self.clone();
        copy.content_id = [0; 32];
        if framed_hash(MIGRATION_DOMAIN, &[&canonical(&copy)?]) != self.content_id {
            return Err(VerticalError::Invalid("migration receipt content id"));
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VerticalError> {
        if bytes.len() > MAX_RECEIPT_BYTES {
            return Err(VerticalError::Invalid("migration receipt bound"));
        }
        let value: Self = strict_json(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(VerticalError::Codec);
        }
        Ok(value)
    }
}

/// Frozen byte adapter boundary: an eight-byte unsigned big-endian length
/// followed by the exact canonical log payload.
pub fn frame_log(log: &VerticalLogV1) -> Result<Vec<u8>, VerticalError> {
    frame_payload(&log.to_bytes()?, MAX_LOG_BYTES)
}

pub fn migrate_v1_to_v2(
    framed_v1: &[u8],
    record: &SessionRecordV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<(Vec<u8>, VerticalMigrationReceiptV1), VerticalError> {
    let source_bytes = unframe_payload(framed_v1, MAX_LOG_BYTES)?;
    let source = VerticalLogV1::from_bytes(source_bytes, record, input, packet)?;
    let (target, receipt) = migrate_log_v1_to_v2(&source, record, input, packet)?;
    Ok((frame_log(&target)?, receipt))
}

/// Rollback never rewrites the V1 artifact: after strict reopening it returns
/// the caller's exact framed bytes.
pub fn rollback_v1(
    framed_v1: &[u8],
    record: &SessionRecordV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<Vec<u8>, VerticalError> {
    let source_bytes = unframe_payload(framed_v1, MAX_LOG_BYTES)?;
    let source = VerticalLogV1::from_bytes(source_bytes, record, input, packet)?;
    if source.codec_version != 1 {
        return Err(VerticalError::Invalid("rollback source codec"));
    }
    Ok(framed_v1.to_vec())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalPersistenceReceiptV1 {
    pub system_id: String,
    pub proof_id: String,
    pub log_hash: [u8; 32],
    pub head: Option<[u8; 32]>,
    pub revision: u64,
    pub evidence_only: bool,
    pub mutation_authority: bool,
    pub content_id: [u8; 32],
}
pub fn persistence_receipt(
    log: &VerticalLogV1,
) -> Result<VerticalPersistenceReceiptV1, VerticalError> {
    let mut value = VerticalPersistenceReceiptV1 {
        system_id: "world-history-ledger".into(),
        proof_id: "c4v-vertical-persistence-seam-v1".into(),
        log_hash: log.log_hash,
        head: log.head()?,
        revision: log.revision(),
        evidence_only: true,
        mutation_authority: false,
        content_id: [0; 32],
    };
    value.content_id = framed_hash(RECEIPT_DOMAIN, &[&canonical(&value)?]);
    if canonical(&value)?.len() > MAX_RECEIPT_BYTES {
        return Err(VerticalError::Invalid("receipt bound"));
    }
    Ok(value)
}

impl VerticalPersistenceReceiptV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VerticalError> {
        let bytes = canonical(self)?;
        if bytes.len() > MAX_RECEIPT_BYTES {
            return Err(VerticalError::Invalid("receipt bound"));
        }
        let mut copy = self.clone();
        copy.content_id = [0; 32];
        if framed_hash(RECEIPT_DOMAIN, &[&canonical(&copy)?]) != self.content_id {
            return Err(VerticalError::Invalid("receipt content id"));
        }
        if self.system_id != "world-history-ledger"
            || self.proof_id != "c4v-vertical-persistence-seam-v1"
            || !self.evidence_only
            || self.mutation_authority
        {
            return Err(VerticalError::Invalid("receipt authority"));
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VerticalError> {
        if bytes.len() > MAX_RECEIPT_BYTES {
            return Err(VerticalError::Invalid("receipt bound"));
        }
        let value: Self = strict_json(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(VerticalError::Codec);
        }
        Ok(value)
    }
}

fn validate_authority(
    identity: &VerticalIdentityV1,
    state: &BaseLoopStateV1,
    record: &SessionRecordV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<(), VerticalError> {
    if state.run_id != identity.run_id
        || state.session_id != identity.session_id
        || record.session_id != identity.session_id
    {
        return Err(VerticalError::Invalid("GP1 identity mismatch"));
    }
    let expected = bind_validated_c3a_world(input, packet).map_err(|_| VerticalError::Upstream)?;
    if state.world_context != LoopWorldContextV1::ValidatedC3A(expected) {
        return Err(VerticalError::Invalid("crossed C3A context"));
    }
    state
        .validate_against(record)
        .map_err(|_| VerticalError::Upstream)?;
    let descriptor = HierarchyDescriptor::decode_strict(&identity.descriptor_bytes)
        .map_err(|_| VerticalError::Codec)?;
    let rebuilt = bind_addressable_world_package(
        identity.place_id,
        Some(identity.hub_id),
        descriptor.reconstruction_fingerprint,
        input,
        packet,
        descriptor.recipe.clone(),
    )
    .map_err(|_| VerticalError::Upstream)?;
    if rebuilt
        .encode_canonical()
        .map_err(|_| VerticalError::Upstream)?
        != identity.descriptor_bytes
    {
        return Err(VerticalError::Invalid("descriptor authority mismatch"));
    }
    Ok(())
}
fn build_baseline(
    identity: &VerticalIdentityV1,
    descriptor: &HierarchyDescriptor,
    packet: &CausalWorldPacket,
    initial: &[u8],
    codec: u16,
) -> Result<BaselineManifest, VerticalError> {
    BaselineManifest::new(
        identity.place_id,
        descriptor
            .fingerprint()
            .map_err(|_| VerticalError::Upstream)?,
        vec![
            DependencyRef {
                kind: 1,
                fingerprint: world_conditions_contract_fingerprint(packet.content.schema_version),
            },
            DependencyRef {
                kind: 2,
                fingerprint: world_conditions_packet_fingerprint(packet)
                    .map_err(|_| VerticalError::Upstream)?,
            },
            DependencyRef {
                kind: 3,
                fingerprint: domain_hash(GP1_CONTRACT, &[]),
            },
            DependencyRef {
                kind: 4,
                fingerprint: identity.fingerprint()?,
            },
            DependencyRef {
                kind: 5,
                fingerprint: framed_hash(CONSEQUENCE_DOMAIN, &[initial]),
            },
            DependencyRef {
                kind: 6,
                fingerprint: codec_fingerprint(codec)?,
            },
        ],
    )
    .map_err(|_| VerticalError::Upstream)
}
fn validate_batch_actions(actions: &[BaseLoopActionV1]) -> Result<(), VerticalError> {
    if actions.len() == 2 {
        let valid = matches!(
            (&actions[0], &actions[1]),
            (
                BaseLoopActionV1::Depart,
                BaseLoopActionV1::ChooseOutcome { .. } | BaseLoopActionV1::FailEncounter { .. }
            ) | (
                BaseLoopActionV1::Recover,
                BaseLoopActionV1::ChooseOutcome { .. } | BaseLoopActionV1::FailEncounter { .. }
            )
        );
        if !valid {
            return Err(VerticalError::Invalid("unsupported two-action batch"));
        }
    } else if actions.len() != 1 {
        return Err(VerticalError::Invalid("command action count"));
    }
    if actions.len() == 1
        && matches!(
            actions[0],
            BaseLoopActionV1::Depart | BaseLoopActionV1::Recover
        )
    {
        return Err(VerticalError::Invalid("unsafe action persisted alone"));
    }
    Ok(())
}
fn decode_state(
    record: &SessionRecordV1,
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    bytes: &[u8],
) -> Result<BaseLoopStateV1, VerticalError> {
    if bytes.len() > MAX_STATE_BYTES {
        return Err(VerticalError::Invalid("state byte bound"));
    }
    let context = LoopWorldContextV1::ValidatedC3A(
        bind_validated_c3a_world(input, packet).map_err(|_| VerticalError::Upstream)?,
    );
    BaseLoopStateV1::from_bytes(record, &context, bytes).map_err(|_| VerticalError::Upstream)
}
fn decode_stored(bytes: &[u8]) -> Result<StoredEventV1, VerticalError> {
    let value: StoredEventV1 = strict_json(bytes)?;
    if value.command_bytes.len() > MAX_COMMAND_BYTES
        || value.consequence_bytes.len() > MAX_STATE_BYTES
        || framed_hash(CONSEQUENCE_DOMAIN, &[&value.consequence_bytes]) != value.consequence_hash
    {
        return Err(VerticalError::Invalid("stored event"));
    }
    Ok(value)
}
fn codec_fingerprint(version: u16) -> Result<[u8; 32], VerticalError> {
    match version {
        1 => Ok(domain_hash(CODEC_V1, &[])),
        2 => Ok(domain_hash(CODEC_V2, &[])),
        _ => Err(VerticalError::Invalid("codec version")),
    }
}
fn operation_schema() -> [u8; 32] {
    domain_hash(OPERATION_SCHEMA, &[])
}
fn validate_id(value: &str) -> Result<(), VerticalError> {
    if value.is_empty()
        || value.len() > 96
        || !value.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
        })
    {
        return Err(VerticalError::Invalid("invalid GP1 identifier"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use addressable_world_binding::bind_addressable_world_package;
    use derived_world_rules::compile_world;
    use mindwarp_gameplay_foundation::{
        BaseLoopLedgerV1, PreparationV1, fixed_sessions, start_authored_base_loop,
        start_c3a_base_loop,
    };
    use universe_identity::AddressSegment;

    mod world_support {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../mindwarp-gameplay-foundation/tests/world_support/mod.rs"
        ));
    }

    struct Fixture {
        record: SessionRecordV1,
        input: WorldGenerationInput,
        packet: CausalWorldPacket,
        identity: VerticalIdentityV1,
        log: VerticalLogV1,
    }

    fn address(seed: [u8; 32], kind: NodeKind, payload: &[u8]) -> UniverseAddress {
        UniverseAddress::new(seed, vec![AddressSegment::new(kind, payload).unwrap()]).unwrap()
    }

    fn fixture() -> Fixture {
        let record = fixed_sessions()[0].clone();
        let input = world_support::world_input([71; 32]);
        let packet = compile_world(&input).unwrap();
        let hub = address([9; 32], NodeKind::Site, b"hub");
        let place = address([9; 32], NodeKind::Site, b"place");
        let player = address([9; 32], NodeKind::Entity, b"player");
        let place_id = place.logical_fingerprint().unwrap();
        let descriptor = bind_addressable_world_package(
            place_id,
            Some(hub.logical_fingerprint().unwrap()),
            input.reconstruction_id,
            &input,
            &packet,
            b"c4v-place-v1".to_vec(),
        )
        .unwrap();
        let identity = VerticalIdentityV1::new(
            &hub,
            &place,
            &player,
            &descriptor,
            &record.session_id,
            "run-c4v",
        )
        .unwrap();
        let initial = start_c3a_base_loop(
            &record,
            "run-c4v",
            BaseLoopLedgerV1::empty(),
            &input,
            &packet,
        )
        .unwrap();
        let log = VerticalLogV1::initialize(identity.clone(), &initial, &record, &input, &packet)
            .unwrap();
        Fixture {
            record,
            input,
            packet,
            identity,
            log,
        }
    }

    fn command(
        fixture: &Fixture,
        log: &VerticalLogV1,
        id: u8,
        actions: Vec<BaseLoopActionV1>,
    ) -> VerticalCommandBatchV1 {
        VerticalCommandBatchV1 {
            actor_player_id: fixture.identity.player_id,
            command_id: [id; 32],
            expected_revision: log.revision(),
            sequence: log.revision() + 1,
            expected_parent: log.head().unwrap(),
            actions,
        }
    }

    fn prepare(fixture: &Fixture) -> BaseLoopActionV1 {
        BaseLoopActionV1::Prepare(PreparationV1 {
            session_id: fixture.record.session_id.clone(),
            intent_id: "restore-shared-cause".into(),
            tool_id: "fitting-field-tool".into(),
            divert_threat: false,
        })
    }

    fn complete_log(f: &Fixture) -> VerticalLogV1 {
        let (log, _, _) = f
            .log
            .append(
                &f.record,
                &f.input,
                &f.packet,
                &command(f, &f.log, 1, vec![prepare(f)]),
            )
            .unwrap();
        let outcome = f
            .record
            .outcomes
            .iter()
            .find(|item| {
                !matches!(
                    item.trigger,
                    mindwarp_gameplay_foundation::OutcomeTrigger::Retreat
                )
            })
            .unwrap();
        let (log2, _, _) = log
            .append(
                &f.record,
                &f.input,
                &f.packet,
                &command(
                    f,
                    &log,
                    2,
                    vec![
                        BaseLoopActionV1::Depart,
                        BaseLoopActionV1::ChooseOutcome {
                            outcome_id: outcome.outcome_id.clone(),
                        },
                    ],
                ),
            )
            .unwrap();
        let (log3, _, _) = log2
            .append(
                &f.record,
                &f.input,
                &f.packet,
                &command(f, &log2, 3, vec![BaseLoopActionV1::BeginReturn]),
            )
            .unwrap();
        log3.append(
            &f.record,
            &f.input,
            &f.packet,
            &command(
                f,
                &log3,
                4,
                vec![BaseLoopActionV1::RecordRememberedResponse],
            ),
        )
        .unwrap()
        .0
    }

    #[test]
    fn stable_stop_batches_retry_conflict_stale_gap_and_fork_are_exact() {
        let f = fixture();
        let first = command(&f, &f.log, 1, vec![prepare(&f)]);
        let (log, state, appended) = f
            .log
            .append(&f.record, &f.input, &f.packet, &first)
            .unwrap();
        assert!(matches!(appended, AppendOutcome::Appended(_)));
        assert_eq!(log.revision(), 1);
        let (same, retry_state, retry) =
            log.append(&f.record, &f.input, &f.packet, &first).unwrap();
        assert_eq!(same, log);
        assert_eq!(retry_state, state);
        assert!(matches!(retry, AppendOutcome::Idempotent(_)));
        let mut changed = first.clone();
        changed.actions = vec![BaseLoopActionV1::BeginReturn];
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &changed),
            Err(VerticalError::RetryConflict)
        );
        let mut stale = command(&f, &log, 2, vec![BaseLoopActionV1::BeginReturn]);
        stale.expected_revision = 0;
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &stale),
            Err(VerticalError::Stale)
        );
        let mut gap = command(&f, &log, 3, vec![BaseLoopActionV1::BeginReturn]);
        gap.sequence = 3;
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &gap),
            Err(VerticalError::Gap)
        );
        let mut fork = command(&f, &log, 4, vec![BaseLoopActionV1::BeginReturn]);
        fork.expected_parent = Some([99; 32]);
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &fork),
            Err(VerticalError::Fork)
        );
        let unsafe_depart = command(&f, &log, 5, vec![BaseLoopActionV1::Depart]);
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &unsafe_depart),
            Err(VerticalError::Invalid("unsafe action persisted alone"))
        );
    }

    #[test]
    fn restart_is_byte_exact_and_rejects_corruption_and_terminal_append() {
        let f = fixture();
        let log = complete_log(&f);
        let bytes = log.to_bytes().unwrap();
        assert_eq!(
            VerticalLogV1::from_bytes(&bytes, &f.record, &f.input, &f.packet).unwrap(),
            log
        );
        let mut trailing = bytes.clone();
        trailing.push(b' ');
        assert_eq!(
            VerticalLogV1::from_bytes(&trailing, &f.record, &f.input, &f.packet),
            Err(VerticalError::Codec)
        );
        let terminal = command(&f, &log, 9, vec![BaseLoopActionV1::BeginReturn]);
        assert_eq!(
            log.append(&f.record, &f.input, &f.packet, &terminal),
            Err(VerticalError::Terminal)
        );

        let mut corrupt = log.clone();
        let delta = DeltaEnvelope::decode_strict(&corrupt.delta_bytes[0]).unwrap();
        let mut stored = decode_stored(&delta.operation).unwrap();
        stored.consequence_hash = [0; 32];
        let replacement = DeltaEnvelope::new(
            delta.baseline_key,
            delta.target_logical_id,
            delta.sequence,
            delta.expected_parent,
            delta.command_id,
            delta.operation_schema,
            canonical(&stored).unwrap(),
        )
        .unwrap();
        corrupt.delta_bytes[0] = replacement.encode_canonical().unwrap();
        corrupt.log_hash = corrupt.compute_hash().unwrap();
        assert!(corrupt.restart(&f.record, &f.input, &f.packet).is_err());
    }

    #[test]
    fn snapshot_migration_and_read_only_receipt_are_verifiable() {
        let f = fixture();
        let log = complete_log(&f);
        let snapshot = VerticalSnapshotV1::build(&log, &f.record, &f.input, &f.packet).unwrap();
        snapshot
            .verify(&log, &f.record, &f.input, &f.packet)
            .unwrap();
        let mut poisoned = snapshot.clone();
        poisoned.state_hash[0] ^= 1;
        assert_eq!(
            poisoned.verify(&log, &f.record, &f.input, &f.packet),
            Err(VerticalError::Invalid("snapshot mismatch"))
        );
        let framed_v1 = frame_log(&log).unwrap();
        let (framed_v2, receipt) =
            migrate_v1_to_v2(&framed_v1, &f.record, &f.input, &f.packet).unwrap();
        let v2 = VerticalLogV1::from_bytes(
            unframe_payload(&framed_v2, MAX_LOG_BYTES).unwrap(),
            &f.record,
            &f.input,
            &f.packet,
        )
        .unwrap();
        assert_eq!(v2.codec_version, 2);
        assert_eq!(v2.final_state_bytes, log.final_state_bytes);
        assert_eq!(
            receipt.command_ids,
            vec![[1; 32], [2; 32], [3; 32], [4; 32]]
        );
        assert_eq!(
            log.restart(&f.record, &f.input, &f.packet).unwrap(),
            v2.restart(&f.record, &f.input, &f.packet).unwrap()
        );
        assert_eq!(
            rollback_v1(&framed_v1, &f.record, &f.input, &f.packet).unwrap(),
            framed_v1
        );
        assert_eq!(
            VerticalMigrationReceiptV1::from_bytes(&receipt.to_bytes().unwrap()).unwrap(),
            receipt
        );
        assert_eq!(
            VerticalSnapshotV1::from_bytes(
                &snapshot.to_bytes().unwrap(),
                &log,
                &f.record,
                &f.input,
                &f.packet
            )
            .unwrap(),
            snapshot
        );
        let proof = persistence_receipt(&log).unwrap();
        assert_eq!(proof.system_id, "world-history-ledger");
        assert_eq!(proof.proof_id, "c4v-vertical-persistence-seam-v1");
        assert!(proof.evidence_only && !proof.mutation_authority);
        assert_eq!(
            VerticalPersistenceReceiptV1::from_bytes(&proof.to_bytes().unwrap()).unwrap(),
            proof
        );
    }

    #[test]
    fn crossed_identity_packet_actor_and_authored_context_fail_closed() {
        let f = fixture();
        let mut actor = command(&f, &f.log, 1, vec![prepare(&f)]);
        actor.actor_player_id = [44; 32];
        assert_eq!(
            f.log.append(&f.record, &f.input, &f.packet, &actor),
            Err(VerticalError::Invalid("actor mismatch"))
        );
        let foreign_input = world_support::world_input([72; 32]);
        let foreign_packet = compile_world(&foreign_input).unwrap();
        assert!(
            f.log
                .restart(&f.record, &foreign_input, &foreign_packet)
                .is_err()
        );
        let authored =
            start_authored_base_loop(&f.record, "run-c4v", BaseLoopLedgerV1::empty()).unwrap();
        assert_eq!(
            VerticalLogV1::initialize(
                f.identity.clone(),
                &authored,
                &f.record,
                &f.input,
                &f.packet
            ),
            Err(VerticalError::Invalid("crossed C3A context"))
        );
        let mut crossed = f.identity.clone();
        crossed.player_id = crossed.place_id;
        assert_eq!(
            crossed.validate(),
            Err(VerticalError::Invalid("vertical identity mismatch"))
        );
    }

    #[test]
    fn all_five_sessions_initialize_and_persist_from_exact_c3a_stable_stops() {
        let seed = fixture();
        let s1_terminal = complete_log(&seed)
            .restart(&seed.record, &seed.input, &seed.packet)
            .unwrap();
        for (index, record) in fixed_sessions().iter().enumerate() {
            let hub = address([9; 32], NodeKind::Site, format!("hub-{index}").as_bytes());
            let place = address([9; 32], NodeKind::Site, format!("place-{index}").as_bytes());
            let player = address(
                [9; 32],
                NodeKind::Entity,
                format!("player-{index}").as_bytes(),
            );
            let descriptor = bind_addressable_world_package(
                place.logical_fingerprint().unwrap(),
                Some(hub.logical_fingerprint().unwrap()),
                seed.input.reconstruction_id,
                &seed.input,
                &seed.packet,
                format!("c4v-place-{index}").into_bytes(),
            )
            .unwrap();
            let run_id = format!("run-c4v-{index}");
            let identity = VerticalIdentityV1::new(
                &hub,
                &place,
                &player,
                &descriptor,
                &record.session_id,
                &run_id,
            )
            .unwrap();
            let ledger = if index == 4 {
                s1_terminal.ledger_after.clone()
            } else {
                BaseLoopLedgerV1::empty()
            };
            let initial =
                start_c3a_base_loop(record, &run_id, ledger, &seed.input, &seed.packet).unwrap();
            let log = VerticalLogV1::initialize(
                identity.clone(),
                &initial,
                record,
                &seed.input,
                &seed.packet,
            )
            .unwrap();
            let action = BaseLoopActionV1::Prepare(PreparationV1 {
                session_id: record.session_id.clone(),
                intent_id: "restore-shared-cause".into(),
                tool_id: "fitting-field-tool".into(),
                divert_threat: record.threat_contribution.is_some(),
            });
            let command = VerticalCommandBatchV1 {
                actor_player_id: identity.player_id,
                command_id: [(20 + index) as u8; 32],
                expected_revision: 0,
                sequence: 1,
                expected_parent: None,
                actions: vec![action],
            };
            assert_eq!(
                log.append(record, &seed.input, &seed.packet, &command)
                    .unwrap()
                    .0
                    .revision(),
                1
            );
        }
    }

    #[test]
    fn retained_history_reorder_truncate_wrong_binding_and_fabrication_fail_closed() {
        let f = fixture();
        let first = command(&f, &f.log, 1, vec![prepare(&f)]);
        let one = f
            .log
            .append(&f.record, &f.input, &f.packet, &first)
            .unwrap()
            .0;
        let delta = DeltaEnvelope::decode_strict(&one.delta_bytes[0]).unwrap();
        for crossed in [
            DeltaEnvelope::new(
                [88; 32],
                delta.target_logical_id,
                delta.sequence,
                delta.expected_parent,
                delta.command_id,
                delta.operation_schema,
                delta.operation.clone(),
            )
            .unwrap(),
            DeltaEnvelope::new(
                delta.baseline_key,
                [89; 32],
                delta.sequence,
                delta.expected_parent,
                delta.command_id,
                delta.operation_schema,
                delta.operation.clone(),
            )
            .unwrap(),
        ] {
            let mut poisoned = one.clone();
            poisoned.delta_bytes[0] = crossed.encode_canonical().unwrap();
            poisoned.log_hash = poisoned.compute_hash().unwrap();
            assert!(
                poisoned
                    .append(&f.record, &f.input, &f.packet, &first)
                    .is_err()
            );
        }

        let mut semantic_poison = one.clone();
        let mut stored = decode_stored(&delta.operation).unwrap();
        stored.consequence_bytes = one.initial_state_bytes.clone();
        stored.consequence_hash = framed_hash(CONSEQUENCE_DOMAIN, &[&stored.consequence_bytes]);
        let poisoned_delta = DeltaEnvelope::new(
            delta.baseline_key,
            delta.target_logical_id,
            delta.sequence,
            delta.expected_parent,
            delta.command_id,
            delta.operation_schema,
            canonical(&stored).unwrap(),
        )
        .unwrap();
        semantic_poison.delta_bytes[0] = poisoned_delta.encode_canonical().unwrap();
        semantic_poison.final_state_bytes = semantic_poison.initial_state_bytes.clone();
        semantic_poison.log_hash = semantic_poison.compute_hash().unwrap();
        assert_eq!(
            semantic_poison.append(&f.record, &f.input, &f.packet, &first),
            Err(VerticalError::Invalid("retained consequence mismatch"))
        );

        let complete = complete_log(&f);
        let mut reordered = complete.clone();
        reordered.delta_bytes.swap(0, 1);
        reordered.log_hash = reordered.compute_hash().unwrap();
        assert!(reordered.restart(&f.record, &f.input, &f.packet).is_err());
        let mut truncated = complete.clone();
        truncated.delta_bytes.pop();
        truncated.log_hash = truncated.compute_hash().unwrap();
        assert!(truncated.restart(&f.record, &f.input, &f.packet).is_err());

        let mut fabricated = start_c3a_base_loop(
            &f.record,
            "run-c4v",
            BaseLoopLedgerV1::empty(),
            &f.input,
            &f.packet,
        )
        .unwrap();
        fabricated.trace.push(BaseLoopActionV1::BeginReturn);
        assert!(
            VerticalLogV1::initialize(
                f.identity.clone(),
                &fabricated,
                &f.record,
                &f.input,
                &f.packet,
            )
            .is_err()
        );
    }

    #[test]
    fn crossed_hub_place_descriptor_and_poisoned_adapter_are_rejected() {
        let f = fixture();
        let initial = start_c3a_base_loop(
            &f.record,
            "run-c4v",
            BaseLoopLedgerV1::empty(),
            &f.input,
            &f.packet,
        )
        .unwrap();
        let mut crossed_hub = f.identity.clone();
        crossed_hub.hub_address_bytes = crossed_hub.place_address_bytes.clone();
        let mut crossed_place = f.identity.clone();
        crossed_place.place_address_bytes = crossed_place.hub_address_bytes.clone();
        for crossed in [crossed_hub, crossed_place] {
            assert!(
                VerticalLogV1::initialize(crossed, &initial, &f.record, &f.input, &f.packet,)
                    .is_err()
            );
        }

        let foreign_input = world_support::world_input([73; 32]);
        let foreign_packet = compile_world(&foreign_input).unwrap();
        let hub = UniverseAddress::decode_canonical(&f.identity.hub_address_bytes).unwrap();
        let place = UniverseAddress::decode_canonical(&f.identity.place_address_bytes).unwrap();
        let player = UniverseAddress::decode_canonical(&f.identity.player_address_bytes).unwrap();
        let foreign_descriptor = bind_addressable_world_package(
            f.identity.place_id,
            Some(f.identity.hub_id),
            foreign_input.reconstruction_id,
            &foreign_input,
            &foreign_packet,
            b"foreign-place".to_vec(),
        )
        .unwrap();
        let foreign_identity = VerticalIdentityV1::new(
            &hub,
            &place,
            &player,
            &foreign_descriptor,
            &f.record.session_id,
            "run-c4v",
        )
        .unwrap();
        assert!(
            VerticalLogV1::initialize(foreign_identity, &initial, &f.record, &f.input, &f.packet,)
                .is_err()
        );

        let log = complete_log(&f);
        let mut framed = frame_log(&log).unwrap();
        framed[7] ^= 1;
        assert!(migrate_v1_to_v2(&framed, &f.record, &f.input, &f.packet).is_err());
    }
}

fn frame_payload(payload: &[u8], maximum: usize) -> Result<Vec<u8>, VerticalError> {
    if payload.len() > maximum {
        return Err(VerticalError::Invalid("framed payload bound"));
    }
    let mut framed = Vec::with_capacity(8 + payload.len());
    framed.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    framed.extend_from_slice(payload);
    Ok(framed)
}

fn unframe_payload(framed: &[u8], maximum: usize) -> Result<&[u8], VerticalError> {
    let length_bytes: [u8; 8] = framed
        .get(..8)
        .ok_or(VerticalError::Codec)?
        .try_into()
        .map_err(|_| VerticalError::Codec)?;
    let length =
        usize::try_from(u64::from_be_bytes(length_bytes)).map_err(|_| VerticalError::Codec)?;
    if length > maximum || framed.len() != length + 8 {
        return Err(VerticalError::Invalid("framed payload length"));
    }
    Ok(&framed[8..])
}

fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, VerticalError> {
    serde_json::to_vec(value).map_err(|_| VerticalError::Codec)
}
fn strict_json<T>(bytes: &[u8]) -> Result<T, VerticalError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let value: T = serde_json::from_slice(bytes).map_err(|_| VerticalError::Codec)?;
    if canonical(&value)? != bytes {
        return Err(VerticalError::Codec);
    }
    Ok(value)
}
fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update((domain.len() as u32).to_be_bytes());
    h.update(domain);
    h.update((bytes.len() as u64).to_be_bytes());
    h.update(bytes);
    h.finalize().into()
}
fn framed_hash(domain: &[u8], fields: &[&[u8]]) -> [u8; 32] {
    let mut bytes = vec![];
    for field in fields {
        bytes.extend_from_slice(&(field.len() as u64).to_be_bytes());
        bytes.extend_from_slice(field);
    }
    domain_hash(domain, &bytes)
}
fn map_history(error: hierarchy_history::HierarchyHistoryError) -> VerticalError {
    match error {
        hierarchy_history::HierarchyHistoryError::Gap => VerticalError::Gap,
        hierarchy_history::HierarchyHistoryError::StaleHead => VerticalError::Stale,
        hierarchy_history::HierarchyHistoryError::ForkConflict => VerticalError::Fork,
        hierarchy_history::HierarchyHistoryError::CommandConflict => VerticalError::RetryConflict,
        _ => VerticalError::Upstream,
    }
}
