use std::collections::{BTreeMap, BTreeSet};

use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{CONTRACT_VERSION, HierarchyHistoryError, codec, hash};

const BASELINE_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/baseline/v1\0";
const DELTA_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/delta/v1\0";
const STATE_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/reference-state/v1\0";
const OPERATION_SCHEMA_DOMAIN: &[u8] =
    b"mindwarp/hierarchy-history/reference-operation-schema/v1\0";
const SNAPSHOT_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/snapshot/v1\0";
const MIGRATION_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/migration/v1\0";
const MAX_DEPENDENCIES: usize = 32;
const MAX_OPERATION_BYTES: usize = 65_536;
const MAX_RECOVERY_RECORDS: usize = 1024;
const MAX_RECOVERY_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DependencyRef {
    pub kind: u16,
    pub fingerprint: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BaselineManifest {
    pub logical_id: [u8; 32],
    pub descriptor_fingerprint: [u8; 32],
    pub output_dependencies: Vec<DependencyRef>,
}

impl BaselineManifest {
    pub fn new(
        logical_id: [u8; 32],
        descriptor_fingerprint: [u8; 32],
        output_dependencies: Vec<DependencyRef>,
    ) -> Result<Self, HierarchyHistoryError> {
        if output_dependencies.is_empty() || output_dependencies.len() > MAX_DEPENDENCIES {
            return Err(HierarchyHistoryError::Invalid("baseline dependencies"));
        }
        let unique: BTreeSet<u16> = output_dependencies.iter().map(|item| item.kind).collect();
        if output_dependencies.iter().any(|item| item.kind == 0)
            || unique.len() != output_dependencies.len()
            || !output_dependencies
                .windows(2)
                .all(|pair| pair[0].kind < pair[1].kind)
        {
            return Err(HierarchyHistoryError::Invalid("dependency order"));
        }
        Ok(Self {
            logical_id,
            descriptor_fingerprint,
            output_dependencies,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(4)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.logical_id))
            .and_then(|e| e.bytes(&self.descriptor_fingerprint))
            .and_then(|e| e.array(self.output_dependencies.len() as u64))
            .map_err(codec)?;
        for dependency in &self.output_dependencies {
            encoder
                .array(2)
                .and_then(|e| e.u16(dependency.kind))
                .and_then(|e| e.bytes(&dependency.fingerprint))
                .map_err(codec)?;
        }
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(4)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("baseline envelope"));
        }
        let logical_id = bytes32(decoder.bytes().map_err(codec)?)?;
        let descriptor_fingerprint = bytes32(decoder.bytes().map_err(codec)?)?;
        let count = decoder
            .array()
            .map_err(codec)?
            .ok_or(HierarchyHistoryError::Invalid("indefinite dependencies"))?
            as usize;
        if count == 0 || count > MAX_DEPENDENCIES {
            return Err(HierarchyHistoryError::Invalid("baseline dependencies"));
        }
        let mut output_dependencies = Vec::with_capacity(count);
        for _ in 0..count {
            if decoder.array().map_err(codec)? != Some(2) {
                return Err(HierarchyHistoryError::Invalid("dependency envelope"));
            }
            output_dependencies.push(DependencyRef {
                kind: decoder.u16().map_err(codec)?,
                fingerprint: bytes32(decoder.bytes().map_err(codec)?)?,
            });
        }
        let manifest = Self::new(logical_id, descriptor_fingerprint, output_dependencies)?;
        if decoder.position() != bytes.len() || manifest.encode_canonical()? != bytes {
            return Err(HierarchyHistoryError::Invalid("noncanonical baseline"));
        }
        Ok(manifest)
    }

    pub fn key(&self) -> Result<[u8; 32], HierarchyHistoryError> {
        Ok(hash(BASELINE_DOMAIN, &self.encode_canonical()?))
    }

    pub fn verify_available_dependencies(
        &self,
        available: &[DependencyRef],
    ) -> Result<(), HierarchyHistoryError> {
        let rebuilt = Self::new(
            self.logical_id,
            self.descriptor_fingerprint,
            self.output_dependencies.clone(),
        )?;
        if &rebuilt != self
            || available.is_empty()
            || available.len() > MAX_DEPENDENCIES
            || available.len() > self.output_dependencies.len()
            || available.iter().any(|item| item.kind == 0)
            || !available.windows(2).all(|pair| pair[0].kind < pair[1].kind)
        {
            return Err(HierarchyHistoryError::InvalidDependencyAvailability);
        }
        for required in &self.output_dependencies {
            let Some(actual) = available.iter().find(|item| item.kind == required.kind) else {
                return Err(HierarchyHistoryError::MissingDependency(required.kind));
            };
            if actual.fingerprint != required.fingerprint {
                return Err(HierarchyHistoryError::DependencyFingerprintMismatch(
                    required.kind,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeltaEnvelope {
    pub baseline_key: [u8; 32],
    pub target_logical_id: [u8; 32],
    pub sequence: u64,
    pub expected_parent: Option<[u8; 32]>,
    pub command_id: [u8; 32],
    pub operation_schema: [u8; 32],
    pub operation: Vec<u8>,
    pub content_id: [u8; 32],
}

impl DeltaEnvelope {
    pub fn new(
        baseline_key: [u8; 32],
        target_logical_id: [u8; 32],
        sequence: u64,
        expected_parent: Option<[u8; 32]>,
        command_id: [u8; 32],
        operation_schema: [u8; 32],
        operation: Vec<u8>,
    ) -> Result<Self, HierarchyHistoryError> {
        if sequence == 0 || operation.is_empty() || operation.len() > MAX_OPERATION_BYTES {
            return Err(HierarchyHistoryError::Invalid("delta sequence/operation"));
        }
        let mut envelope = Self {
            baseline_key,
            target_logical_id,
            sequence,
            expected_parent,
            command_id,
            operation_schema,
            operation,
            content_id: [0; 32],
        };
        envelope.content_id = hash(DELTA_DOMAIN, &envelope.encode_body()?);
        Ok(envelope)
    }

    fn encode_body(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(8)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.baseline_key))
            .and_then(|e| e.bytes(&self.target_logical_id))
            .and_then(|e| e.u64(self.sequence))
            .map_err(codec)?;
        match self.expected_parent {
            Some(parent) => encoder.bytes(&parent).map_err(codec)?,
            None => encoder.null().map_err(codec)?,
        };
        encoder
            .bytes(&self.command_id)
            .and_then(|e| e.bytes(&self.operation_schema))
            .and_then(|e| e.bytes(&self.operation))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let body = self.encode_body()?;
        let mut out = Vec::new();
        Encoder::new(&mut out)
            .array(2)
            .and_then(|e| e.bytes(&body))
            .and_then(|e| e.bytes(&self.content_id))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut outer = Decoder::new(bytes);
        if outer.array().map_err(codec)? != Some(2) {
            return Err(HierarchyHistoryError::Invalid("delta wire envelope"));
        }
        let body = outer.bytes().map_err(codec)?;
        let content_id = bytes32(outer.bytes().map_err(codec)?)?;
        if outer.position() != bytes.len() || hash(DELTA_DOMAIN, body) != content_id {
            return Err(HierarchyHistoryError::CorruptContent);
        }
        let mut decoder = Decoder::new(body);
        if decoder.array().map_err(codec)? != Some(8)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("delta body"));
        }
        let baseline_key = bytes32(decoder.bytes().map_err(codec)?)?;
        let target_logical_id = bytes32(decoder.bytes().map_err(codec)?)?;
        let sequence = decoder.u64().map_err(codec)?;
        let expected_parent = match decoder.datatype().map_err(codec)? {
            minicbor::data::Type::Null => {
                decoder.null().map_err(codec)?;
                None
            }
            minicbor::data::Type::Bytes => Some(bytes32(decoder.bytes().map_err(codec)?)?),
            _ => return Err(HierarchyHistoryError::Invalid("delta parent")),
        };
        let decoded = Self::new(
            baseline_key,
            target_logical_id,
            sequence,
            expected_parent,
            bytes32(decoder.bytes().map_err(codec)?)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            decoder.bytes().map_err(codec)?.to_vec(),
        )?;
        if decoder.position() != body.len()
            || decoded.content_id != content_id
            || decoded.encode_canonical()? != bytes
        {
            return Err(HierarchyHistoryError::CorruptContent);
        }
        Ok(decoded)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum AppendOutcome {
    Appended([u8; 32]),
    Idempotent([u8; 32]),
}

#[derive(Clone, Debug)]
pub struct HistoryStream {
    baseline: BaselineManifest,
    baseline_key: [u8; 32],
    events: Vec<DeltaEnvelope>,
    commands: BTreeMap<[u8; 32], [u8; 32]>,
}

impl HistoryStream {
    pub fn new(baseline: BaselineManifest) -> Result<Self, HierarchyHistoryError> {
        let baseline_key = baseline.key()?;
        Ok(Self {
            baseline,
            baseline_key,
            events: Vec::new(),
            commands: BTreeMap::new(),
        })
    }

    pub fn baseline(&self) -> &BaselineManifest {
        &self.baseline
    }

    pub fn baseline_key(&self) -> [u8; 32] {
        self.baseline_key
    }

    pub fn head(&self) -> Option<[u8; 32]> {
        self.events.last().map(|event| event.content_id)
    }

    pub fn events(&self) -> &[DeltaEnvelope] {
        &self.events
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let baseline_bytes = self.baseline.encode_canonical()?;
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(3)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&baseline_bytes))
            .and_then(|e| e.array(self.events.len() as u64))
            .map_err(codec)?;
        for event in &self.events {
            encoder.bytes(&event.encode_canonical()?).map_err(codec)?;
        }
        Ok(out)
    }

    pub fn append(&mut self, event: DeltaEnvelope) -> Result<AppendOutcome, HierarchyHistoryError> {
        if event.sequence == 0 {
            return Err(HierarchyHistoryError::CorruptContent);
        }
        if event.baseline_key != self.baseline_key {
            return Err(HierarchyHistoryError::WrongBaseline);
        }
        if event.target_logical_id != self.baseline.logical_id {
            return Err(HierarchyHistoryError::WrongTarget);
        }
        let event = DeltaEnvelope::decode_strict(&event.encode_canonical()?)?;
        if let Some(existing) = self.commands.get(&event.command_id) {
            return if existing == &event.content_id {
                Ok(AppendOutcome::Idempotent(event.content_id))
            } else {
                Err(HierarchyHistoryError::CommandConflict)
            };
        }
        let expected_sequence = self.events.len() as u64 + 1;
        if event.sequence > expected_sequence {
            return Err(HierarchyHistoryError::Gap);
        }
        if event.sequence < expected_sequence {
            return if self.events.get(event.sequence as usize - 1).is_some() {
                Err(HierarchyHistoryError::ForkConflict)
            } else {
                Err(HierarchyHistoryError::StaleHead)
            };
        }
        if event.expected_parent != self.head() {
            return if event.expected_parent.is_some_and(|parent| {
                self.events
                    .iter()
                    .any(|existing| existing.content_id == parent)
            }) {
                Err(HierarchyHistoryError::StaleHead)
            } else {
                Err(HierarchyHistoryError::ForkConflict)
            };
        }
        self.commands.insert(event.command_id, event.content_id);
        self.events.push(event.clone());
        Ok(AppendOutcome::Appended(event.content_id))
    }

    pub fn replay_reference(&self) -> Result<ReferenceState, HierarchyHistoryError> {
        let schema = reference_operation_schema();
        let mut state = ReferenceState::default();
        for event in &self.events {
            if event.operation_schema != schema {
                return Err(HierarchyHistoryError::UnknownOperationSchema);
            }
            let operation = ReferenceOperation::decode_strict(&event.operation)?;
            if matches!(operation, ReferenceOperation::CrossTarget { .. }) {
                return Err(HierarchyHistoryError::UnsupportedCrossTarget);
            }
            operation.apply(&mut state);
        }
        Ok(state)
    }

    pub fn reject_topology_change(
        &mut self,
        _operation: UnsupportedTopologyOperation,
    ) -> Result<(), HierarchyHistoryError> {
        Err(HierarchyHistoryError::UnsupportedTopology)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UnsupportedTopologyOperation {
    Reparent,
    Split,
    Merge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum RecoveryFailureKind {
    CorruptContent,
    InvalidEnvelope,
    WrongBaseline,
    WrongTarget,
    Gap,
    StaleHead,
    ForkConflict,
    CommandConflict,
    UnknownOperationSchema,
    UnsupportedCrossTarget,
}

#[derive(Clone, Debug)]
pub struct KnownGoodPrefixRecovery {
    pub stream: HistoryStream,
    pub accepted_records: usize,
    pub first_failure: Option<RecoveryFailureKind>,
}

pub fn recover_known_good_prefix(
    baseline: BaselineManifest,
    encoded_records: &[Vec<u8>],
) -> Result<KnownGoodPrefixRecovery, HierarchyHistoryError> {
    if encoded_records.len() > MAX_RECOVERY_RECORDS {
        return Err(HierarchyHistoryError::RecoveryBoundExceeded);
    }
    let total = encoded_records.iter().try_fold(0_usize, |sum, bytes| {
        sum.checked_add(bytes.len())
            .ok_or(HierarchyHistoryError::RecoveryBoundExceeded)
    })?;
    if total > MAX_RECOVERY_BYTES {
        return Err(HierarchyHistoryError::RecoveryBoundExceeded);
    }
    let mut stream = HistoryStream::new(baseline)?;
    for (index, bytes) in encoded_records.iter().enumerate() {
        let result = (|| {
            let event = DeltaEnvelope::decode_strict(bytes)?;
            let mut candidate = stream.clone();
            candidate.append(event)?;
            candidate.replay_reference()?;
            stream = candidate;
            Ok::<(), HierarchyHistoryError>(())
        })();
        if let Err(error) = result {
            return Ok(KnownGoodPrefixRecovery {
                stream,
                accepted_records: index,
                first_failure: Some(recovery_failure_kind(&error)),
            });
        }
    }
    Ok(KnownGoodPrefixRecovery {
        accepted_records: encoded_records.len(),
        stream,
        first_failure: None,
    })
}

fn recovery_failure_kind(error: &HierarchyHistoryError) -> RecoveryFailureKind {
    match error {
        HierarchyHistoryError::CorruptContent => RecoveryFailureKind::CorruptContent,
        HierarchyHistoryError::WrongBaseline => RecoveryFailureKind::WrongBaseline,
        HierarchyHistoryError::WrongTarget => RecoveryFailureKind::WrongTarget,
        HierarchyHistoryError::Gap => RecoveryFailureKind::Gap,
        HierarchyHistoryError::StaleHead => RecoveryFailureKind::StaleHead,
        HierarchyHistoryError::ForkConflict => RecoveryFailureKind::ForkConflict,
        HierarchyHistoryError::CommandConflict => RecoveryFailureKind::CommandConflict,
        HierarchyHistoryError::UnknownOperationSchema => {
            RecoveryFailureKind::UnknownOperationSchema
        }
        HierarchyHistoryError::UnsupportedCrossTarget => {
            RecoveryFailureKind::UnsupportedCrossTarget
        }
        _ => RecoveryFailureKind::InvalidEnvelope,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceOperation {
    Set { key: u16, value: i64 },
    Remove { key: u16 },
    CrossTarget { other_target: [u8; 32] },
}

impl ReferenceOperation {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        match self {
            Self::Set { key, value } => encoder
                .array(3)
                .and_then(|e| e.u8(0))
                .and_then(|e| e.u16(*key))
                .and_then(|e| e.i64(*value))
                .map_err(codec)?,
            Self::Remove { key } => encoder
                .array(2)
                .and_then(|e| e.u8(1))
                .and_then(|e| e.u16(*key))
                .map_err(codec)?,
            Self::CrossTarget { other_target } => encoder
                .array(2)
                .and_then(|e| e.u8(2))
                .and_then(|e| e.bytes(other_target))
                .map_err(codec)?,
        };
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        let len = decoder
            .array()
            .map_err(codec)?
            .ok_or(HierarchyHistoryError::Invalid("operation envelope"))?;
        let operation = match (decoder.u8().map_err(codec)?, len) {
            (0, 3) => Self::Set {
                key: decoder.u16().map_err(codec)?,
                value: decoder.i64().map_err(codec)?,
            },
            (1, 2) => Self::Remove {
                key: decoder.u16().map_err(codec)?,
            },
            (2, 2) => Self::CrossTarget {
                other_target: bytes32(decoder.bytes().map_err(codec)?)?,
            },
            _ => return Err(HierarchyHistoryError::Invalid("operation tag/length")),
        };
        if decoder.position() != bytes.len() || operation.encode_canonical()? != bytes {
            return Err(HierarchyHistoryError::Invalid("noncanonical operation"));
        }
        Ok(operation)
    }

    fn apply(self, state: &mut ReferenceState) {
        match self {
            Self::Set { key, value } => {
                state.values.insert(key, value);
            }
            Self::Remove { key } => {
                state.values.remove(&key);
            }
            Self::CrossTarget { .. } => {}
        }
    }
}

pub fn reference_operation_schema() -> [u8; 32] {
    hash(
        OPERATION_SCHEMA_DOMAIN,
        b"set-i64/remove-u16;cross-target-rejected",
    )
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct ReferenceState {
    pub values: BTreeMap<u16, i64>,
}

impl ReferenceState {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder.array(self.values.len() as u64).map_err(codec)?;
        for (key, value) in &self.values {
            encoder
                .array(2)
                .and_then(|e| e.u16(*key))
                .and_then(|e| e.i64(*value))
                .map_err(codec)?;
        }
        Ok(out)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], HierarchyHistoryError> {
        Ok(hash(STATE_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Snapshot {
    pub baseline_key: [u8; 32],
    pub covered_head: Option<[u8; 32]>,
    pub sequence: u64,
    pub reducer_fingerprint: [u8; 32],
    pub builder_fingerprint: [u8; 32],
    pub state_bytes: Vec<u8>,
    pub state_hash: [u8; 32],
    pub content_id: [u8; 32],
}

impl Snapshot {
    pub fn build_reference(
        stream: &HistoryStream,
        builder_fingerprint: [u8; 32],
    ) -> Result<Self, HierarchyHistoryError> {
        let state_bytes = stream.replay_reference()?.encode_canonical()?;
        let state_hash = hash(STATE_DOMAIN, &state_bytes);
        let mut snapshot = Self {
            baseline_key: stream.baseline_key(),
            covered_head: stream.head(),
            sequence: stream.events().len() as u64,
            reducer_fingerprint: reference_operation_schema(),
            builder_fingerprint,
            state_bytes,
            state_hash,
            content_id: [0; 32],
        };
        snapshot.content_id = hash(SNAPSHOT_DOMAIN, &snapshot.body_bytes()?);
        Ok(snapshot)
    }

    fn body_bytes(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(7)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.baseline_key))
            .map_err(codec)?;
        match self.covered_head {
            Some(head) => encoder.bytes(&head).map_err(codec)?,
            None => encoder.null().map_err(codec)?,
        };
        encoder
            .u64(self.sequence)
            .and_then(|e| e.bytes(&self.reducer_fingerprint))
            .and_then(|e| e.bytes(&self.builder_fingerprint))
            .and_then(|e| e.bytes(&self.state_bytes))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let body = self.body_bytes()?;
        let mut out = Vec::new();
        Encoder::new(&mut out)
            .array(3)
            .and_then(|e| e.bytes(&body))
            .and_then(|e| e.bytes(&self.state_hash))
            .and_then(|e| e.bytes(&self.content_id))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn verify_reference(
        &self,
        stream: &HistoryStream,
        expected: &ReferenceState,
    ) -> Result<(), HierarchyHistoryError> {
        let replayed = stream.replay_reference()?;
        if self.baseline_key != stream.baseline_key()
            || self.covered_head != stream.head()
            || self.sequence != stream.events().len() as u64
            || self.reducer_fingerprint != reference_operation_schema()
            || self.state_bytes != replayed.encode_canonical()?
            || self.state_hash != replayed.fingerprint()?
            || &replayed != expected
            || self.content_id != hash(SNAPSHOT_DOMAIN, &self.body_bytes()?)
        {
            return Err(HierarchyHistoryError::SnapshotMismatch);
        }
        Ok(())
    }

    pub fn verify_reference_with_builder(
        &self,
        stream: &HistoryStream,
        expected: &ReferenceState,
        expected_builder_fingerprint: [u8; 32],
    ) -> Result<(), HierarchyHistoryError> {
        if self.builder_fingerprint != expected_builder_fingerprint {
            return Err(HierarchyHistoryError::SnapshotMismatch);
        }
        self.verify_reference(stream, expected)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MigrationReceipt {
    pub from_baseline: [u8; 32],
    pub to_baseline: [u8; 32],
    pub source_head: Option<[u8; 32]>,
    pub adapter_fingerprint: [u8; 32],
    pub input_state_hash: [u8; 32],
    pub output_state_hash: [u8; 32],
    pub content_id: [u8; 32],
}

impl MigrationReceipt {
    pub fn identity_reference(
        source: &HistoryStream,
        target: &BaselineManifest,
        adapter_fingerprint: [u8; 32],
    ) -> Result<Self, HierarchyHistoryError> {
        if source.baseline().logical_id != target.logical_id
            || source.baseline_key() == target.key()?
        {
            return Err(HierarchyHistoryError::UnsupportedMigration);
        }
        let state_hash = source.replay_reference()?.fingerprint()?;
        Self::identity_between(
            source.baseline_key(),
            target.key()?,
            source.head(),
            adapter_fingerprint,
            state_hash,
        )
    }

    fn identity_between(
        from_baseline: [u8; 32],
        to_baseline: [u8; 32],
        source_head: Option<[u8; 32]>,
        adapter_fingerprint: [u8; 32],
        state_hash: [u8; 32],
    ) -> Result<Self, HierarchyHistoryError> {
        if from_baseline == to_baseline || adapter_fingerprint == [0; 32] {
            return Err(HierarchyHistoryError::MigrationChainInvalid);
        }
        let mut preimage = Vec::new();
        preimage.extend_from_slice(&from_baseline);
        preimage.extend_from_slice(&to_baseline);
        preimage.extend_from_slice(&source_head.unwrap_or([0; 32]));
        preimage.extend_from_slice(&adapter_fingerprint);
        preimage.extend_from_slice(&state_hash);
        preimage.extend_from_slice(&state_hash);
        Ok(Self {
            from_baseline,
            to_baseline,
            source_head,
            adapter_fingerprint,
            input_state_hash: state_hash,
            output_state_hash: state_hash,
            content_id: hash(MIGRATION_DOMAIN, &preimage),
        })
    }

    pub fn validate_content_id(&self) -> Result<(), HierarchyHistoryError> {
        let expected = Self::identity_between(
            self.from_baseline,
            self.to_baseline,
            self.source_head,
            self.adapter_fingerprint,
            self.input_state_hash,
        )?;
        if self.input_state_hash != self.output_state_hash || self.content_id != expected.content_id
        {
            return Err(HierarchyHistoryError::MigrationChainInvalid);
        }
        Ok(())
    }
}

pub fn identity_reference_chain(
    source: &HistoryStream,
    targets: &[BaselineManifest],
    adapter_fingerprints: &[[u8; 32]],
) -> Result<Vec<MigrationReceipt>, HierarchyHistoryError> {
    if targets.is_empty()
        || targets.len() > 2
        || targets.len() != adapter_fingerprints.len()
        || adapter_fingerprints.contains(&[0; 32])
        || adapter_fingerprints
            .windows(2)
            .any(|pair| pair[0] == pair[1])
    {
        return Err(HierarchyHistoryError::MigrationChainInvalid);
    }
    let mut keys = Vec::with_capacity(targets.len());
    let mut visited = BTreeSet::from([source.baseline_key()]);
    for target in targets {
        if target.logical_id != source.baseline().logical_id
            || BaselineManifest::decode_strict(&target.encode_canonical()?)? != *target
        {
            return Err(HierarchyHistoryError::MigrationChainInvalid);
        }
        let key = target.key()?;
        if !visited.insert(key) {
            return Err(HierarchyHistoryError::MigrationChainInvalid);
        }
        keys.push(key);
    }
    let state_hash = source.replay_reference()?.fingerprint()?;
    let mut receipts = Vec::with_capacity(keys.len());
    let mut from = source.baseline_key();
    for (to, adapter) in keys.into_iter().zip(adapter_fingerprints.iter().copied()) {
        receipts.push(MigrationReceipt::identity_between(
            from,
            to,
            source.head(),
            adapter,
            state_hash,
        )?);
        from = to;
    }
    Ok(receipts)
}

pub fn validate_identity_reference_chain(
    source: &HistoryStream,
    targets: &[BaselineManifest],
    adapter_fingerprints: &[[u8; 32]],
    received: &[MigrationReceipt],
) -> Result<(), HierarchyHistoryError> {
    if received.is_empty()
        || received.len() > 2
        || received.len() != targets.len()
        || received.len() != adapter_fingerprints.len()
    {
        return Err(HierarchyHistoryError::MigrationChainInvalid);
    }
    for receipt in received {
        receipt.validate_content_id()?;
    }
    if identity_reference_chain(source, targets, adapter_fingerprints)? != received {
        return Err(HierarchyHistoryError::MigrationChainInvalid);
    }
    Ok(())
}

fn bytes32(bytes: &[u8]) -> Result<[u8; 32], HierarchyHistoryError> {
    bytes
        .try_into()
        .map_err(|_| HierarchyHistoryError::Invalid("expected 32 bytes"))
}
