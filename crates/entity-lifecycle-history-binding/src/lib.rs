//! Capability-free G1-C4 integration proof: selected-entity lifecycle deltas
//! replayed through `hierarchy-history`'s already-proven generic delta/
//! replay machinery, and recovered without any continuous simulation.
//!
//! `entity-lifecycle` defines the pure transition table; this crate proves
//! only that its state-changing events can be mapped onto
//! `hierarchy_history::ReferenceOperation` and reconstructed by replaying
//! stored (encoded-and-decoded) deltas, with no new operation schema, no
//! reducer changes, and no wall clock.

use entity_lifecycle::{
    AgeCohort, LifecycleError, LifecycleEvent, LifecycleMode, LifecycleState,
    apply as lifecycle_apply, validate_state,
};
use hierarchy_history::{
    BaselineManifest, DeltaEnvelope, DependencyRef, HierarchyHistoryError, HistoryStream,
    ReferenceOperation, ReferenceState, reference_operation_schema,
};
use sha2::{Digest, Sha256};

const COHORT_BINDING_DOMAIN: &[u8] = b"mindwarp/entity-lifecycle/ambient-cohort-binding/v1\0";
const COHORT_BINDING_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AmbientCohortBindingV1 {
    entity_logical_id: [u8; 32],
    assignment_contract_fingerprint: [u8; 32],
    cohort: AgeCohort,
}

impl AmbientCohortBindingV1 {
    pub fn new(
        entity_logical_id: [u8; 32],
        assignment_contract_fingerprint: [u8; 32],
        cohort: AgeCohort,
    ) -> Result<Self, BindingError> {
        if entity_logical_id == [0; 32] || assignment_contract_fingerprint == [0; 32] {
            return Err(BindingError::InvalidCohortBinding);
        }
        Ok(Self {
            entity_logical_id,
            assignment_contract_fingerprint,
            cohort,
        })
    }

    pub fn encode_canonical(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(67);
        bytes.extend_from_slice(&COHORT_BINDING_VERSION.to_be_bytes());
        bytes.extend_from_slice(&self.entity_logical_id);
        bytes.extend_from_slice(&self.assignment_contract_fingerprint);
        bytes.push(self.cohort as u8);
        bytes
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, BindingError> {
        if bytes.len() != 67 || u16::from_be_bytes([bytes[0], bytes[1]]) != COHORT_BINDING_VERSION {
            return Err(BindingError::InvalidCohortBinding);
        }
        let entity_logical_id = bytes[2..34]
            .try_into()
            .map_err(|_| BindingError::InvalidCohortBinding)?;
        let assignment_contract_fingerprint = bytes[34..66]
            .try_into()
            .map_err(|_| BindingError::InvalidCohortBinding)?;
        let cohort = match bytes[66] {
            0 => AgeCohort::Young,
            1 => AgeCohort::Juvenile,
            2 => AgeCohort::Adult,
            3 => AgeCohort::Elderly,
            _ => return Err(BindingError::InvalidCohortBinding),
        };
        let value = Self::new(entity_logical_id, assignment_contract_fingerprint, cohort)?;
        if value.encode_canonical() != bytes {
            return Err(BindingError::InvalidCohortBinding);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(COHORT_BINDING_DOMAIN);
        hasher.update(self.encode_canonical());
        hasher.finalize().into()
    }

    pub fn verify_expected(
        &self,
        entity_logical_id: [u8; 32],
        assignment_contract_fingerprint: [u8; 32],
        cohort: AgeCohort,
    ) -> Result<(), BindingError> {
        let expected = Self::new(entity_logical_id, assignment_contract_fingerprint, cohort)?;
        if *self != expected {
            return Err(BindingError::InvalidCohortBinding);
        }
        Ok(())
    }
}

pub const KEY_MODE: u16 = 1;
pub const KEY_MATURITY: u16 = 2;
pub const KEY_ELDER: u16 = 3;
pub const KEY_APPEARANCE_LOCK: u16 = 4;

#[derive(Debug)]
pub enum BindingError {
    Lifecycle(LifecycleError),
    History(HierarchyHistoryError),
    InvalidStoredState(&'static str),
    InvalidCohortBinding,
}

impl From<HierarchyHistoryError> for BindingError {
    fn from(value: HierarchyHistoryError) -> Self {
        Self::History(value)
    }
}

/// Maps one state-changing lifecycle event to the single generic delta
/// operation it produces. `Reload` produces none, because it changes
/// nothing and must not pollute the delta ledger.
fn event_operation(next: &LifecycleState, event: LifecycleEvent) -> Option<ReferenceOperation> {
    match event {
        LifecycleEvent::Reload => None,
        LifecycleEvent::BeginTracking => Some(ReferenceOperation::Set {
            key: KEY_MODE,
            value: 1,
        }),
        LifecycleEvent::AdvanceMaturity { .. } => Some(ReferenceOperation::Set {
            key: KEY_MATURITY,
            value: next.maturity_permille as i64,
        }),
        LifecycleEvent::AdvanceElder { .. } => Some(ReferenceOperation::Set {
            key: KEY_ELDER,
            value: next.elder_permille as i64,
        }),
        LifecycleEvent::SetAppearanceLock { locked } => Some(ReferenceOperation::Set {
            key: KEY_APPEARANCE_LOCK,
            value: locked as i64,
        }),
    }
}

/// Reconstructs a `LifecycleState` from an ambient baseline plus whatever
/// keys the replayed reference state contains. Keys absent from the
/// replayed state simply keep the baseline's original value.
pub fn reconstruct_from_reference_state(
    baseline: LifecycleState,
    state: &ReferenceState,
) -> Result<LifecycleState, BindingError> {
    let mut result = baseline;
    if let Some(&mode) = state.values.get(&KEY_MODE) {
        result.mode = match mode {
            0 => LifecycleMode::Ambient,
            1 => LifecycleMode::Tracked,
            _ => return Err(BindingError::InvalidStoredState("invalid lifecycle mode")),
        };
    }
    if let Some(&maturity) = state.values.get(&KEY_MATURITY) {
        result.maturity_permille = u16::try_from(maturity)
            .map_err(|_| BindingError::InvalidStoredState("invalid maturity value"))?;
    }
    if let Some(&elder) = state.values.get(&KEY_ELDER) {
        result.elder_permille = u16::try_from(elder)
            .map_err(|_| BindingError::InvalidStoredState("invalid elder value"))?;
    }
    if let Some(&lock) = state.values.get(&KEY_APPEARANCE_LOCK) {
        result.appearance_lock = match lock {
            0 => false,
            1 => true,
            _ => {
                return Err(BindingError::InvalidStoredState(
                    "invalid appearance-lock value",
                ));
            }
        };
    }
    validate_state(&result).map_err(BindingError::Lifecycle)?;
    if matches!(result.mode, LifecycleMode::Ambient) && result.appearance_lock {
        return Err(BindingError::InvalidStoredState(
            "ambient lifecycle cannot use appearance lock",
        ));
    }
    Ok(result)
}

pub fn demo_baseline_manifest(
    logical_id: [u8; 32],
    descriptor_fingerprint: [u8; 32],
) -> Result<BaselineManifest, BindingError> {
    Ok(BaselineManifest::new(
        logical_id,
        descriptor_fingerprint,
        vec![DependencyRef {
            kind: 1,
            fingerprint: [0; 32],
        }],
    )?)
}

/// Applies each event to `initial` via the proven pure transition table
/// *and* appends the corresponding delta (encoded to bytes and decoded
/// back, simulating a storage round-trip) onto `stream`. Returns the
/// direct in-memory result for comparison against a later pure replay.
pub fn drive(
    stream: &mut HistoryStream,
    mut state: LifecycleState,
    events: &[LifecycleEvent],
) -> Result<LifecycleState, BindingError> {
    let baseline_key = stream.baseline_key();
    let target = stream.baseline().logical_id;
    let mut delta_sequence = stream.events().len() as u64;
    for &event in events {
        let next = lifecycle_apply(state, event).map_err(BindingError::Lifecycle)?;
        if next != state
            && let Some(operation) = event_operation(&next, event)
        {
            delta_sequence += 1;
            let operation_bytes = operation.encode_canonical()?;
            let envelope = DeltaEnvelope::new(
                baseline_key,
                target,
                delta_sequence,
                stream.head(),
                command_id(delta_sequence),
                reference_operation_schema(),
                operation_bytes,
            )?;
            let stored = envelope.encode_canonical()?;
            let reloaded = DeltaEnvelope::decode_strict(&stored)?;
            stream.append(reloaded)?;
        }
        state = next;
    }
    Ok(state)
}

fn command_id(sequence: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&sequence.to_be_bytes());
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity_lifecycle::AgeCohort;

    fn new_stream() -> HistoryStream {
        let baseline = demo_baseline_manifest([1; 32], [2; 32]).unwrap();
        HistoryStream::new(baseline).unwrap()
    }

    #[test]
    fn recovery_from_stored_deltas_matches_direct_lifecycle_application() {
        let ambient = LifecycleState::ambient(AgeCohort::Adult);
        let events = [
            LifecycleEvent::BeginTracking,
            LifecycleEvent::SetAppearanceLock { locked: true },
            LifecycleEvent::AdvanceElder {
                delta_permille: 250,
            },
            LifecycleEvent::AdvanceElder {
                delta_permille: 250,
            },
            LifecycleEvent::SetAppearanceLock { locked: false },
        ];

        let mut stream = new_stream();
        let direct_result = drive(&mut stream, ambient, &events).unwrap();

        // Recovery: reconstruct purely by replaying stored deltas through
        // hierarchy-history's own reducer, never touching entity_lifecycle
        // events again and never simulating forward.
        let replayed = stream.replay_reference().unwrap();
        let recovered = reconstruct_from_reference_state(ambient, &replayed).unwrap();

        assert_eq!(recovered, direct_result);
        assert_eq!(
            entity_lifecycle::present(&recovered),
            entity_lifecycle::PresentedStage::Elderly
        );
    }

    #[test]
    fn reload_produces_no_delta_and_never_pollutes_the_ledger() {
        let ambient = LifecycleState::ambient(AgeCohort::Young);
        let events = [
            LifecycleEvent::BeginTracking,
            LifecycleEvent::Reload,
            LifecycleEvent::AdvanceMaturity {
                delta_permille: 500,
            },
            LifecycleEvent::Reload,
        ];
        let mut stream = new_stream();
        drive(&mut stream, ambient, &events).unwrap();
        assert_eq!(
            stream.events().len(),
            2,
            "only the two state-changing events should append a delta"
        );
    }

    #[test]
    fn gapped_lifecycle_delta_is_rejected_by_the_existing_machinery() {
        let mut stream = new_stream();
        let operation = ReferenceOperation::Set {
            key: KEY_MODE,
            value: 1,
        }
        .encode_canonical()
        .unwrap();
        let first = DeltaEnvelope::new(
            stream.baseline_key(),
            stream.baseline().logical_id,
            1,
            None,
            command_id(1),
            reference_operation_schema(),
            operation.clone(),
        )
        .unwrap();
        stream.append(first).unwrap();

        let skipped = DeltaEnvelope::new(
            stream.baseline_key(),
            stream.baseline().logical_id,
            3,
            stream.head(),
            command_id(3),
            reference_operation_schema(),
            operation,
        )
        .unwrap();
        assert_eq!(stream.append(skipped), Err(HierarchyHistoryError::Gap));
    }

    #[test]
    fn multiple_drive_batches_continue_sequence_and_command_identity() {
        let ambient = LifecycleState::ambient(AgeCohort::Adult);
        let mut stream = new_stream();
        let tracked = drive(&mut stream, ambient, &[LifecycleEvent::BeginTracking]).unwrap();
        let locked = drive(
            &mut stream,
            tracked,
            &[LifecycleEvent::SetAppearanceLock { locked: true }],
        )
        .unwrap();
        let aged = drive(
            &mut stream,
            locked,
            &[LifecycleEvent::AdvanceElder {
                delta_permille: 250,
            }],
        )
        .unwrap();
        assert_eq!(stream.events().len(), 3);
        assert_eq!(
            reconstruct_from_reference_state(ambient, &stream.replay_reference().unwrap()).unwrap(),
            aged
        );
    }

    #[test]
    fn hostile_reference_values_fail_before_lifecycle_reconstruction() {
        let ambient = LifecycleState::ambient(AgeCohort::Adult);
        let mut state = ReferenceState::default();
        state.values.insert(KEY_MATURITY, -1);
        assert!(matches!(
            reconstruct_from_reference_state(ambient, &state),
            Err(BindingError::InvalidStoredState("invalid maturity value"))
        ));
    }

    #[test]
    fn zero_delta_is_not_recorded_as_a_state_change() {
        let ambient = LifecycleState::ambient(AgeCohort::Young);
        let mut stream = new_stream();
        let tracked = drive(&mut stream, ambient, &[LifecycleEvent::BeginTracking]).unwrap();
        drive(
            &mut stream,
            tracked,
            &[LifecycleEvent::AdvanceMaturity { delta_permille: 0 }],
        )
        .unwrap();
        assert_eq!(stream.events().len(), 1);
    }
}
