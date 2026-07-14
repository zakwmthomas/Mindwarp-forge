//! The protected, deterministic core for Mind Warp Forge.
//!
//! This first vertical slice is intentionally in-memory. Persistence adapters,
//! projections, UI, and platform capabilities are separate modules. The API
//! here makes the trust boundary testable before those modules exist.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub mod code_admission;
pub mod compiler;
pub mod contracts;
pub mod control_plane;
pub mod knowledge;
pub mod persistence;

pub type ObjectId = String;
pub type EventId = String;
pub type CandidateId = String;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActorKind {
    DirectProjectUser,
    Assistant,
    System,
    ImportedContent,
    ExternalTool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthorityBasis {
    None,
    Delegated { delegation_id: String },
    ExplicitUserAuthorization,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    EvidenceRegistered,
    CandidateProposed,
    CorrectionRecorded,
    ApprovalProposed,
    CandidateApproved,
    CandidatePromoted,
    CandidateRejected,
    CandidateQuarantined,
    CodeApplied,
    CodeRolledBack,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CandidateState {
    Proposed,
    Approved,
    Promoted,
    Rejected,
    Quarantined,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StoredObject {
    pub id: ObjectId,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub sequence: u64,
    pub schema_version: u16,
    pub event_type: EventType,
    pub actor: ActorKind,
    pub authority: AuthorityBasis,
    pub input_objects: Vec<ObjectId>,
    pub prior_events: Vec<EventId>,
    pub correlation_id: String,
    pub payload: Value,
    pub hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub id: CandidateId,
    pub evidence: ObjectId,
    pub state: CandidateState,
    pub evidence_events: Vec<EventId>,
}

#[derive(Default)]
pub struct ForgeKernel {
    objects: BTreeMap<ObjectId, StoredObject>,
    events: Vec<Event>,
    candidates: BTreeMap<CandidateId, Candidate>,
}

impl ForgeKernel {
    pub fn put_object(&mut self, bytes: impl AsRef<[u8]>) -> ObjectId {
        let bytes = bytes.as_ref().to_vec();
        let id = sha256(&bytes);
        self.objects
            .entry(id.clone())
            .or_insert_with(|| StoredObject {
                id: id.clone(),
                bytes,
            });
        id
    }

    pub fn object(&self, id: &str) -> Option<&StoredObject> {
        self.objects.get(id)
    }

    pub fn object_id_for(bytes: impl AsRef<[u8]>) -> ObjectId {
        sha256(bytes)
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn objects(&self) -> impl Iterator<Item = &StoredObject> {
        self.objects.values()
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn candidates(&self) -> impl Iterator<Item = &Candidate> {
        self.candidates.values()
    }

    pub fn candidate(&self, id: &str) -> Option<&Candidate> {
        self.candidates.get(id)
    }

    pub fn from_records(
        objects: impl IntoIterator<Item = StoredObject>,
        events: impl IntoIterator<Item = Event>,
    ) -> Result<Self, KernelError> {
        let mut kernel = Self::default();
        for object in objects {
            if sha256(&object.bytes) != object.id {
                return Err(KernelError::ObjectHashMismatch(object.id));
            }
            kernel.objects.insert(object.id.clone(), object);
        }

        let mut events: Vec<Event> = events.into_iter().collect();
        events.sort_by_key(|event| event.sequence);
        for (index, event) in events.into_iter().enumerate() {
            let expected_sequence = index as u64 + 1;
            if event.sequence != expected_sequence {
                return Err(KernelError::InvalidEventSequence {
                    expected: expected_sequence,
                    actual: event.sequence,
                });
            }
            if event.id != event.hash || event.hash != event_hash(&event)? {
                return Err(KernelError::EventHashMismatch(event.id));
            }
            if event
                .input_objects
                .iter()
                .any(|id| kernel.object(id).is_none())
            {
                return Err(KernelError::UnknownObject("replayed event input".into()));
            }
            kernel.apply_replayed_event(&event)?;
            kernel.events.push(event);
        }
        Ok(kernel)
    }

    pub fn register_evidence(
        &mut self,
        actor: ActorKind,
        bytes: impl AsRef<[u8]>,
        correlation_id: impl Into<String>,
    ) -> Result<ObjectId, KernelError> {
        let object_id = self.put_object(bytes);
        self.append_event(
            EventType::EvidenceRegistered,
            actor,
            AuthorityBasis::None,
            vec![object_id.clone()],
            vec![],
            correlation_id.into(),
            Value::Null,
        )?;
        Ok(object_id)
    }

    pub fn propose_candidate(
        &mut self,
        evidence: &str,
        correlation_id: impl Into<String>,
    ) -> Result<CandidateId, KernelError> {
        if self.object(evidence).is_none() {
            return Err(KernelError::UnknownObject(evidence.into()));
        }
        let candidate_id = sha256(format!("candidate:v1:{evidence}").as_bytes());
        if self.candidates.contains_key(&candidate_id) {
            return Ok(candidate_id);
        }
        let event = self.append_event(
            EventType::CandidateProposed,
            ActorKind::Assistant,
            AuthorityBasis::None,
            vec![evidence.into()],
            vec![],
            correlation_id.into(),
            Value::String(candidate_id.clone()),
        )?;
        self.candidates.insert(
            candidate_id.clone(),
            Candidate {
                id: candidate_id.clone(),
                evidence: evidence.into(),
                state: CandidateState::Proposed,
                evidence_events: vec![event.id],
            },
        );
        Ok(candidate_id)
    }

    pub fn propose_approval(
        &mut self,
        actor: ActorKind,
        candidate_id: &str,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        self.require_candidate(candidate_id, CandidateState::Proposed)?;
        self.append_event(
            EventType::ApprovalProposed,
            actor,
            AuthorityBasis::None,
            vec![],
            vec![],
            correlation_id.into(),
            Value::String(candidate_id.into()),
        )
        .map(|event| event.id)
    }

    pub fn approve_candidate(
        &mut self,
        actor: ActorKind,
        authority: AuthorityBasis,
        candidate_id: &str,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        self.require_explicit_user(actor, &authority)?;
        let candidate = self
            .require_candidate(candidate_id, CandidateState::Proposed)?
            .clone();
        let event = self.append_event(
            EventType::CandidateApproved,
            ActorKind::DirectProjectUser,
            authority,
            vec![candidate.evidence.clone()],
            candidate.evidence_events.clone(),
            correlation_id.into(),
            Value::String(candidate_id.into()),
        )?;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .state = CandidateState::Approved;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .evidence_events
            .push(event.id.clone());
        Ok(event.id)
    }

    pub fn promote_candidate(
        &mut self,
        actor: ActorKind,
        authority: AuthorityBasis,
        candidate_id: &str,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        self.require_explicit_user(actor, &authority)?;
        let candidate = self
            .require_candidate(candidate_id, CandidateState::Approved)?
            .clone();
        let event = self.append_event(
            EventType::CandidatePromoted,
            ActorKind::DirectProjectUser,
            authority,
            vec![candidate.evidence.clone()],
            candidate.evidence_events.clone(),
            correlation_id.into(),
            Value::String(candidate_id.into()),
        )?;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .state = CandidateState::Promoted;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .evidence_events
            .push(event.id.clone());
        Ok(event.id)
    }

    pub fn reject_candidate(
        &mut self,
        candidate_id: &str,
        correction_evidence: &str,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        let candidate = self
            .require_candidate(candidate_id, CandidateState::Proposed)?
            .clone();
        if self.object(correction_evidence).is_none() {
            return Err(KernelError::UnknownObject(correction_evidence.into()));
        }
        let event = self.append_event(
            EventType::CorrectionRecorded,
            ActorKind::DirectProjectUser,
            AuthorityBasis::None,
            vec![candidate.evidence.clone(), correction_evidence.into()],
            candidate.evidence_events.clone(),
            correlation_id.into(),
            Value::String(candidate_id.into()),
        )?;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .state = CandidateState::Rejected;
        self.candidates
            .get_mut(candidate_id)
            .expect("checked candidate")
            .evidence_events
            .push(event.id.clone());
        Ok(event.id)
    }

    pub fn record_code_application(
        &mut self,
        actor: ActorKind,
        authority: AuthorityBasis,
        candidate_id: &str,
        payload: Value,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        self.require_explicit_user(actor, &authority)?;
        let candidate = self
            .require_candidate(candidate_id, CandidateState::Promoted)?
            .clone();
        self.append_event(
            EventType::CodeApplied,
            ActorKind::DirectProjectUser,
            authority,
            vec![candidate.evidence],
            candidate.evidence_events,
            correlation_id.into(),
            payload,
        )
        .map(|event| event.id)
    }

    pub fn record_code_rollback(
        &mut self,
        actor: ActorKind,
        authority: AuthorityBasis,
        application_event_id: &str,
        correlation_id: impl Into<String>,
    ) -> Result<EventId, KernelError> {
        self.require_explicit_user(actor, &authority)?;
        let application = self
            .events
            .iter()
            .find(|event| {
                event.id == application_event_id && event.event_type == EventType::CodeApplied
            })
            .cloned()
            .ok_or_else(|| KernelError::InvalidReplayEvent(application_event_id.into()))?;
        self.append_event(
            EventType::CodeRolledBack,
            ActorKind::DirectProjectUser,
            authority,
            application.input_objects,
            vec![application.id],
            correlation_id.into(),
            application.payload,
        )
        .map(|event| event.id)
    }

    fn append_event(
        &mut self,
        event_type: EventType,
        actor: ActorKind,
        authority: AuthorityBasis,
        mut input_objects: Vec<ObjectId>,
        mut prior_events: Vec<EventId>,
        correlation_id: String,
        payload: Value,
    ) -> Result<Event, KernelError> {
        if input_objects.iter().any(|id| self.object(id).is_none()) {
            return Err(KernelError::UnknownObject("event input".into()));
        }
        input_objects.sort();
        prior_events.sort();
        let sequence = self.events.len() as u64 + 1;
        let mut event = Event {
            id: String::new(),
            sequence,
            schema_version: 1,
            event_type,
            actor,
            authority,
            input_objects,
            prior_events,
            correlation_id,
            payload,
            hash: String::new(),
        };
        let hash = event_hash(&event)?;
        event.id = hash.clone();
        event.hash = hash;
        self.events.push(event.clone());
        Ok(event)
    }

    fn apply_replayed_event(&mut self, event: &Event) -> Result<(), KernelError> {
        let candidate_id = || {
            event
                .payload
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| KernelError::InvalidReplayEvent(event.id.clone()))
        };
        match event.event_type {
            EventType::CandidateProposed => {
                let candidate_id = candidate_id()?;
                let evidence = event
                    .input_objects
                    .first()
                    .cloned()
                    .ok_or_else(|| KernelError::InvalidReplayEvent(event.id.clone()))?;
                if event.actor != ActorKind::Assistant
                    || event.authority != AuthorityBasis::None
                    || event.input_objects.len() != 1
                    || !event.prior_events.is_empty()
                    || candidate_id != sha256(format!("candidate:v1:{evidence}").as_bytes())
                    || self.candidates.contains_key(&candidate_id)
                {
                    return Err(KernelError::InvalidReplayEvent(event.id.clone()));
                }
                self.candidates.insert(
                    candidate_id.clone(),
                    Candidate {
                        id: candidate_id,
                        evidence,
                        state: CandidateState::Proposed,
                        evidence_events: vec![event.id.clone()],
                    },
                );
            }
            EventType::CandidateApproved => {
                self.require_replayed_explicit_user(event)?;
                self.apply_replayed_state(
                    candidate_id()?,
                    CandidateState::Proposed,
                    CandidateState::Approved,
                    event,
                )?;
            }
            EventType::CandidatePromoted => {
                self.require_replayed_explicit_user(event)?;
                self.apply_replayed_state(
                    candidate_id()?,
                    CandidateState::Approved,
                    CandidateState::Promoted,
                    event,
                )?;
            }
            EventType::CorrectionRecorded | EventType::CandidateRejected => {
                if event.actor != ActorKind::DirectProjectUser
                    || event.authority != AuthorityBasis::None
                {
                    return Err(KernelError::InvalidReplayEvent(event.id.clone()));
                }
                self.apply_replayed_state(
                    candidate_id()?,
                    CandidateState::Proposed,
                    CandidateState::Rejected,
                    event,
                )?;
            }
            EventType::CandidateQuarantined => {
                self.apply_replayed_state(
                    candidate_id()?,
                    CandidateState::Proposed,
                    CandidateState::Quarantined,
                    event,
                )?;
            }
            EventType::CodeApplied | EventType::CodeRolledBack => {
                self.require_replayed_explicit_user(event)?
            }
            EventType::EvidenceRegistered | EventType::ApprovalProposed => {}
        }
        Ok(())
    }

    fn apply_replayed_state(
        &mut self,
        candidate_id: CandidateId,
        required: CandidateState,
        next: CandidateState,
        event: &Event,
    ) -> Result<(), KernelError> {
        let candidate = self
            .candidates
            .get_mut(&candidate_id)
            .ok_or_else(|| KernelError::UnknownCandidate(candidate_id.clone()))?;
        if candidate.state != required {
            return Err(KernelError::InvalidReplayEvent(event.id.clone()));
        }
        if !event.input_objects.contains(&candidate.evidence)
            || matches!(
                event.event_type,
                EventType::CandidateApproved | EventType::CandidatePromoted
            ) && event.input_objects.len() != 1
        {
            return Err(KernelError::InvalidReplayEvent(event.id.clone()));
        }
        let mut expected_prior_events = candidate.evidence_events.clone();
        expected_prior_events.sort();
        if event.prior_events != expected_prior_events {
            return Err(KernelError::InvalidReplayEvent(event.id.clone()));
        }
        candidate.state = next;
        candidate.evidence_events.push(event.id.clone());
        Ok(())
    }

    fn require_replayed_explicit_user(&self, event: &Event) -> Result<(), KernelError> {
        if event.actor == ActorKind::DirectProjectUser
            && event.authority == AuthorityBasis::ExplicitUserAuthorization
        {
            Ok(())
        } else {
            Err(KernelError::InvalidReplayEvent(event.id.clone()))
        }
    }

    fn require_candidate(
        &self,
        candidate_id: &str,
        required: CandidateState,
    ) -> Result<&Candidate, KernelError> {
        let candidate = self
            .candidate(candidate_id)
            .ok_or_else(|| KernelError::UnknownCandidate(candidate_id.into()))?;
        if candidate.state != required {
            return Err(KernelError::InvalidCandidateState {
                candidate: candidate_id.into(),
                actual: candidate.state.clone(),
                required,
            });
        }
        Ok(candidate)
    }

    fn require_explicit_user(
        &self,
        actor: ActorKind,
        authority: &AuthorityBasis,
    ) -> Result<(), KernelError> {
        if actor == ActorKind::DirectProjectUser
            && *authority == AuthorityBasis::ExplicitUserAuthorization
        {
            Ok(())
        } else {
            Err(KernelError::AuthorityDenied)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KernelError {
    UnknownObject(String),
    ObjectHashMismatch(String),
    UnknownCandidate(String),
    EventHashMismatch(String),
    InvalidEventSequence {
        expected: u64,
        actual: u64,
    },
    InvalidReplayEvent(String),
    InvalidTranscript(String),
    InvalidCodeAdmission(String),
    InvalidCandidateState {
        candidate: String,
        actual: CandidateState,
        required: CandidateState,
    },
    AuthorityDenied,
    Serialization(String),
}

fn sha256(bytes: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes.as_ref());
    format!("{:x}", hasher.finalize())
}

fn event_hash(event: &Event) -> Result<String, KernelError> {
    let canonical = serde_json::to_vec(&(
        event.sequence,
        &event.event_type,
        &event.actor,
        &event.authority,
        &event.input_objects,
        &event.prior_events,
        &event.correlation_id,
        &event.payload,
    ))
    .map_err(|error| KernelError::Serialization(error.to_string()))?;
    Ok(sha256(canonical))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objects_are_content_addressed_and_deduplicated() {
        let mut kernel = ForgeKernel::default();
        let first = kernel.put_object(b"same bytes");
        let second = kernel.put_object(b"same bytes");
        assert_eq!(first, second);
        assert_eq!(kernel.object(&first).unwrap().bytes, b"same bytes");
    }

    #[test]
    fn equivalent_sequences_have_deterministic_event_ids() {
        fn make_kernel() -> ForgeKernel {
            let mut kernel = ForgeKernel::default();
            let evidence = kernel
                .register_evidence(ActorKind::Assistant, b"proposal", "thread-1")
                .unwrap();
            kernel.propose_candidate(&evidence, "thread-1").unwrap();
            kernel
        }
        let first = make_kernel();
        let second = make_kernel();
        assert_eq!(first.events(), second.events());
    }

    #[test]
    fn imported_approval_cannot_approve_or_promote() {
        let mut kernel = ForgeKernel::default();
        let evidence = kernel
            .register_evidence(ActorKind::Assistant, b"proposal", "thread-1")
            .unwrap();
        let candidate = kernel.propose_candidate(&evidence, "thread-1").unwrap();
        assert!(matches!(
            kernel.approve_candidate(
                ActorKind::ImportedContent,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "thread-1"
            ),
            Err(KernelError::AuthorityDenied)
        ));
        assert_eq!(
            kernel.candidate(&candidate).unwrap().state,
            CandidateState::Proposed
        );
    }

    #[test]
    fn promotion_requires_separate_explicit_approval() {
        let mut kernel = ForgeKernel::default();
        let evidence = kernel
            .register_evidence(ActorKind::Assistant, b"proposal", "thread-1")
            .unwrap();
        let candidate = kernel.propose_candidate(&evidence, "thread-1").unwrap();
        assert!(matches!(
            kernel.promote_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "thread-1"
            ),
            Err(KernelError::InvalidCandidateState { .. })
        ));
        kernel
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "thread-1",
            )
            .unwrap();
        kernel
            .promote_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "thread-1",
            )
            .unwrap();
        assert_eq!(
            kernel.candidate(&candidate).unwrap().state,
            CandidateState::Promoted
        );
    }

    #[test]
    fn correction_preserves_candidate_history() {
        let mut kernel = ForgeKernel::default();
        let proposal = kernel
            .register_evidence(ActorKind::Assistant, b"proposal", "thread-1")
            .unwrap();
        let correction = kernel
            .register_evidence(ActorKind::DirectProjectUser, b"no, revise this", "thread-1")
            .unwrap();
        let candidate = kernel.propose_candidate(&proposal, "thread-1").unwrap();
        kernel
            .reject_candidate(&candidate, &correction, "thread-1")
            .unwrap();
        assert_eq!(
            kernel.candidate(&candidate).unwrap().state,
            CandidateState::Rejected
        );
        assert_eq!(kernel.events().len(), 4);
    }

    #[test]
    fn replay_rejects_a_rehashed_non_user_approval() {
        let mut kernel = ForgeKernel::default();
        let evidence = kernel
            .register_evidence(ActorKind::Assistant, b"proposal", "thread-1")
            .unwrap();
        let candidate = kernel.propose_candidate(&evidence, "thread-1").unwrap();
        kernel
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                &candidate,
                "thread-1",
            )
            .unwrap();

        let objects = kernel.objects().cloned().collect::<Vec<_>>();
        let mut events = kernel.events().to_vec();
        let forged = events.last_mut().unwrap();
        forged.actor = ActorKind::Assistant;
        let forged_hash = event_hash(forged).unwrap();
        forged.id = forged_hash.clone();
        forged.hash = forged_hash;

        assert!(matches!(
            ForgeKernel::from_records(objects, events),
            Err(KernelError::InvalidReplayEvent(_))
        ));
    }
}
