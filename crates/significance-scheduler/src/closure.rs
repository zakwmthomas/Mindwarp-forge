use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{
    BudgetEnvelope, CONTRACT_VERSION, DecisionKind, HysteresisPolicy, ImportancePacket,
    ImportanceTier, ResourceClass, SignificanceSchedulerError, SignificanceState, WorkTicket,
    bytes32, codec, hash,
};

const MAP_DOMAIN: &[u8] = b"mindwarp/significance/domain-fidelity-map-set/v1\0";
const BINDING_DOMAIN: &[u8] = b"mindwarp/significance/decision-binding/v1\0";
const BUDGET_DOMAIN: &[u8] = b"mindwarp/scheduler/budget/v1\0";
const ADMISSION_DOMAIN: &[u8] = b"mindwarp/scheduler/admission-receipt/v1\0";
const TRACE_V2_DOMAIN: &[u8] = b"mindwarp/scheduler/pressure-trace/v2\0";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(u16)]
pub enum ConsumerDomainV1 {
    Generation = 1,
    Simulation = 2,
    Ai = 3,
    Physics = 4,
    Animation = 5,
    Audio = 6,
    Rendering = 7,
    Streaming = 8,
}
impl ConsumerDomainV1 {
    pub const ALL: [Self; 8] = [
        Self::Generation,
        Self::Simulation,
        Self::Ai,
        Self::Physics,
        Self::Animation,
        Self::Audio,
        Self::Rendering,
        Self::Streaming,
    ];
    pub const fn code(self) -> u16 {
        self as u16
    }
}
impl TryFrom<u16> for ConsumerDomainV1 {
    type Error = SignificanceSchedulerError;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Generation),
            2 => Ok(Self::Simulation),
            3 => Ok(Self::Ai),
            4 => Ok(Self::Physics),
            5 => Ok(Self::Animation),
            6 => Ok(Self::Audio),
            7 => Ok(Self::Rendering),
            8 => Ok(Self::Streaming),
            _ => Err(SignificanceSchedulerError::Invalid("consumer domain")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DomainFidelityMapSetV1 {
    levels: [[u8; 4]; 8],
}
impl DomainFidelityMapSetV1 {
    pub fn new(levels: [[u8; 4]; 8]) -> Result<Self, SignificanceSchedulerError> {
        if levels
            .iter()
            .any(|m| m.iter().any(|v| *v > 16) || !m.windows(2).all(|p| p[0] <= p[1]))
        {
            return Err(SignificanceSchedulerError::Invalid("domain fidelity map"));
        }
        Ok(Self { levels })
    }
    pub fn fidelity(&self, domain: ConsumerDomainV1, tier: ImportanceTier) -> u8 {
        self.levels[(domain.code() - 1) as usize][tier as usize]
    }
    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(3)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.array(8))
            .map_err(codec)?;
        for (i, m) in self.levels.iter().enumerate() {
            e.array(2)
                .and_then(|e| e.u16((i + 1) as u16))
                .and_then(|e| e.array(4))
                .map_err(codec)?;
            for v in m {
                e.u8(*v).map_err(codec)?;
            }
        }
        Ok(out)
    }
    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(3)
            || d.u16().map_err(codec)? != CONTRACT_VERSION
            || d.array().map_err(codec)? != Some(8)
        {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let mut levels = [[0; 4]; 8];
        for (i, m) in levels.iter_mut().enumerate() {
            if d.array().map_err(codec)? != Some(2)
                || d.u16().map_err(codec)? != (i + 1) as u16
                || d.array().map_err(codec)? != Some(4)
            {
                return Err(SignificanceSchedulerError::NonCanonical);
            }
            for v in m {
                *v = d.u8().map_err(codec)?;
            }
        }
        let value = Self::new(levels)?;
        if d.position() != bytes.len() || value.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(value)
    }
    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        Ok(hash(MAP_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImportanceDecisionBindingV1 {
    target_descriptor: [u8; 32],
    request_epoch: u64,
    packet_fingerprint: [u8; 32],
    policy_fingerprint: [u8; 32],
    map_set_fingerprint: [u8; 32],
    step: u64,
    prior_state: SignificanceState,
    resulting_state: SignificanceState,
    pub tier: ImportanceTier,
}
impl ImportanceDecisionBindingV1 {
    pub fn derive(
        packet: &ImportancePacket,
        policy: HysteresisPolicy,
        prior_state: SignificanceState,
        step: u64,
        maps: &DomainFidelityMapSetV1,
    ) -> Result<Self, SignificanceSchedulerError> {
        if step == 0 {
            return Err(SignificanceSchedulerError::Invalid("binding step"));
        }
        let mut resulting_state = prior_state;
        let tier = resulting_state.advance(packet, policy, step)?;
        Ok(Self {
            target_descriptor: packet.target_descriptor,
            request_epoch: packet.request_epoch,
            packet_fingerprint: packet.fingerprint()?,
            policy_fingerprint: policy.fingerprint()?,
            map_set_fingerprint: maps.fingerprint()?,
            step,
            prior_state,
            resulting_state,
            tier,
        })
    }
    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.target_descriptor);
        bytes.extend_from_slice(&self.request_epoch.to_be_bytes());
        bytes.extend_from_slice(&self.packet_fingerprint);
        bytes.extend_from_slice(&self.policy_fingerprint);
        bytes.extend_from_slice(&self.map_set_fingerprint);
        bytes.extend_from_slice(&self.step.to_be_bytes());
        bytes.push(self.prior_state.tier as u8);
        bytes.extend_from_slice(&self.prior_state.since_step.to_be_bytes());
        bytes.extend_from_slice(&self.prior_state.last_step.to_be_bytes());
        bytes.push(self.resulting_state.tier as u8);
        bytes.extend_from_slice(&self.resulting_state.since_step.to_be_bytes());
        bytes.extend_from_slice(&self.resulting_state.last_step.to_be_bytes());
        Ok(hash(BINDING_DOMAIN, &bytes))
    }
    pub fn packet_fingerprint(&self) -> [u8; 32] {
        self.packet_fingerprint
    }
    pub fn verify(
        &self,
        packet: &ImportancePacket,
        policy: HysteresisPolicy,
        maps: &DomainFidelityMapSetV1,
    ) -> Result<(), SignificanceSchedulerError> {
        let expected = Self::derive(packet, policy, self.prior_state, self.step, maps)?;
        if &expected != self {
            return Err(SignificanceSchedulerError::Invalid(
                "importance binding mismatch",
            ));
        }
        Ok(())
    }
    pub fn verify_ticket(&self, ticket: &WorkTicket) -> Result<(), SignificanceSchedulerError> {
        ConsumerDomainV1::try_from(ticket.consumer)?;
        if ticket.target_descriptor != self.target_descriptor
            || ticket.request_epoch != self.request_epoch
            || ticket.importance_packet != self.packet_fingerprint
            || ticket.importance_tier != self.tier
        {
            return Err(SignificanceSchedulerError::Invalid(
                "importance binding mismatch",
            ));
        }
        Ok(())
    }
}

impl BudgetEnvelope {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(5)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.u64(self.epoch))
            .and_then(|e| e.array(4))
            .map_err(codec)?;
        for v in self.units {
            e.u32(v).map_err(codec)?;
        }
        e.array(4).map_err(codec)?;
        for v in self.safety_reserve {
            e.u32(v).map_err(codec)?;
        }
        e.u16(self.max_service_debt).map_err(codec)?;
        Ok(out)
    }
    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(5) || d.u16().map_err(codec)? != CONTRACT_VERSION {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let epoch = d.u64().map_err(codec)?;
        if d.array().map_err(codec)? != Some(4) {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let units = [
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
        ];
        if d.array().map_err(codec)? != Some(4) {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let reserves = [
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
            d.u32().map_err(codec)?,
        ];
        let value = Self::new(epoch, units, reserves, d.u16().map_err(codec)?)?;
        if d.position() != bytes.len() || value.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(value)
    }
    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        Ok(hash(BUDGET_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdmissionReceiptV1 {
    pub accepted: bool,
    pub reason_code: u16,
    pub ticket_fingerprints: Vec<[u8; 32]>,
    pub graph_fingerprint: [u8; 32],
    pub budget_fingerprint: [u8; 32],
}
impl AdmissionReceiptV1 {
    pub fn evaluate(tickets: &[WorkTicket], budget: BudgetEnvelope) -> Self {
        let result = crate::ReferenceScheduler::new(tickets.to_vec(), budget);
        Self::from_result(tickets, budget, result)
    }
    pub fn evaluate_verified(
        tickets: &[WorkTicket],
        budget: BudgetEnvelope,
        bindings: &[ImportanceDecisionBindingV1],
    ) -> Self {
        let result = crate::ReferenceScheduler::new_verified(tickets.to_vec(), budget, bindings);
        Self::from_result(tickets, budget, result)
    }
    fn from_result(
        tickets: &[WorkTicket],
        budget: BudgetEnvelope,
        result: Result<crate::ReferenceScheduler, SignificanceSchedulerError>,
    ) -> Self {
        let mut ticket_fingerprints: Vec<[u8; 32]> = tickets
            .iter()
            .map(|ticket| ticket.fingerprint().expect("constructed ticket"))
            .collect();
        ticket_fingerprints.sort();
        let mut bytes = Vec::with_capacity(8 + ticket_fingerprints.len() * 32);
        bytes.extend_from_slice(&(ticket_fingerprints.len() as u64).to_be_bytes());
        for fingerprint in &ticket_fingerprints {
            bytes.extend_from_slice(fingerprint)
        }
        let graph_fingerprint = hash(ADMISSION_DOMAIN, &bytes);
        let reason_code = match &result {
            Ok(_) => 1,
            Err(SignificanceSchedulerError::Invalid(_)) => 2,
            Err(SignificanceSchedulerError::DuplicateTicket) => 3,
            Err(SignificanceSchedulerError::UnknownDependency) => 4,
            Err(SignificanceSchedulerError::DependencyCycle) => 5,
            Err(SignificanceSchedulerError::UnknownFallback) => 6,
            Err(SignificanceSchedulerError::InvalidFallback) => 7,
            Err(SignificanceSchedulerError::InvalidCancellationTree) => 8,
            Err(SignificanceSchedulerError::AdmissionRejected) => 9,
            Err(_) => 10,
        };
        Self {
            accepted: result.is_ok(),
            reason_code,
            ticket_fingerprints,
            graph_fingerprint,
            budget_fingerprint: budget.fingerprint().expect("validated budget"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct CompletionReceiptV1 {
    pub ticket_id: [u8; 32],
    pub request_epoch: u64,
    pub completed_units: u32,
    pub output_fingerprint: [u8; 32],
}
impl CompletionReceiptV1 {
    pub fn new(
        ticket_id: [u8; 32],
        request_epoch: u64,
        completed_units: u32,
        output_fingerprint: [u8; 32],
    ) -> Result<Self, SignificanceSchedulerError> {
        if ticket_id == [0; 32]
            || request_epoch == 0
            || completed_units == 0
            || output_fingerprint == [0; 32]
        {
            return Err(SignificanceSchedulerError::Invalid("completion receipt"));
        }
        Ok(Self {
            ticket_id,
            request_epoch,
            completed_units,
            output_fingerprint,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StrictSchedulerDecisionV2 {
    pub ticket_id: [u8; 32],
    pub domain_code: u16,
    pub work_class: u16,
    pub packet_fingerprint: [u8; 32],
    pub kind: DecisionKind,
    pub resource: ResourceClass,
    pub units: u32,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PressureTraceV2 {
    pub step: u64,
    pub budget_epoch: u64,
    pub budget_fingerprint: [u8; 32],
    pub decisions: Vec<StrictSchedulerDecisionV2>,
    pub remaining_tickets: usize,
    pub fingerprint: [u8; 32],
}
impl PressureTraceV2 {
    pub(crate) fn build(
        step: u64,
        budget: BudgetEnvelope,
        decisions: Vec<StrictSchedulerDecisionV2>,
        remaining: usize,
    ) -> Result<Self, SignificanceSchedulerError> {
        let mut value = Self {
            step,
            budget_epoch: budget.epoch,
            budget_fingerprint: budget.fingerprint()?,
            decisions,
            remaining_tickets: remaining,
            fingerprint: [0; 32],
        };
        value.fingerprint = hash(TRACE_V2_DOMAIN, &value.encode_body()?);
        Ok(value)
    }
    fn encode_body(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(6)
            .and_then(|e| e.u16(2))
            .and_then(|e| e.u64(self.step))
            .and_then(|e| e.u64(self.budget_epoch))
            .and_then(|e| e.bytes(&self.budget_fingerprint))
            .and_then(|e| e.array(self.decisions.len() as u64))
            .map_err(codec)?;
        for d in &self.decisions {
            e.array(7)
                .and_then(|e| e.bytes(&d.ticket_id))
                .and_then(|e| e.u16(d.domain_code))
                .and_then(|e| e.u16(d.work_class))
                .and_then(|e| e.bytes(&d.packet_fingerprint))
                .and_then(|e| e.u8(d.kind.stable_code()))
                .and_then(|e| e.u8(d.resource as u8))
                .and_then(|e| e.u32(d.units))
                .map_err(codec)?;
        }
        e.u64(self.remaining_tickets as u64).map_err(codec)?;
        Ok(out)
    }
    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(7)
            .and_then(|e| e.u16(2))
            .and_then(|e| e.u64(self.step))
            .and_then(|e| e.u64(self.budget_epoch))
            .and_then(|e| e.bytes(&self.budget_fingerprint))
            .and_then(|e| e.array(self.decisions.len() as u64))
            .map_err(codec)?;
        for d in &self.decisions {
            e.array(7)
                .and_then(|e| e.bytes(&d.ticket_id))
                .and_then(|e| e.u16(d.domain_code))
                .and_then(|e| e.u16(d.work_class))
                .and_then(|e| e.bytes(&d.packet_fingerprint))
                .and_then(|e| e.u8(d.kind.stable_code()))
                .and_then(|e| e.u8(d.resource as u8))
                .and_then(|e| e.u32(d.units))
                .map_err(codec)?;
        }
        e.u64(self.remaining_tickets as u64)
            .and_then(|e| e.bytes(&self.fingerprint))
            .map_err(codec)?;
        Ok(out)
    }
    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(7) || d.u16().map_err(codec)? != 2 {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let step = d.u64().map_err(codec)?;
        let epoch = d.u64().map_err(codec)?;
        let budget_fingerprint = bytes32(d.bytes().map_err(codec)?)?;
        let count = d
            .array()
            .map_err(codec)?
            .ok_or(SignificanceSchedulerError::NonCanonical)? as usize;
        let mut decisions = Vec::with_capacity(count);
        for _ in 0..count {
            if d.array().map_err(codec)? != Some(7) {
                return Err(SignificanceSchedulerError::NonCanonical);
            }
            let ticket_id = bytes32(d.bytes().map_err(codec)?)?;
            let domain_code = d.u16().map_err(codec)?;
            ConsumerDomainV1::try_from(domain_code)?;
            let work_class = d.u16().map_err(codec)?;
            if !(1..=8).contains(&work_class) {
                return Err(SignificanceSchedulerError::NonCanonical);
            }
            decisions.push(StrictSchedulerDecisionV2 {
                ticket_id,
                domain_code,
                work_class,
                packet_fingerprint: bytes32(d.bytes().map_err(codec)?)?,
                kind: DecisionKind::try_from(d.u8().map_err(codec)?)?,
                resource: ResourceClass::try_from(d.u8().map_err(codec)?)?,
                units: d.u32().map_err(codec)?,
            });
        }
        let remaining = d.u64().map_err(codec)? as usize;
        let fp = bytes32(d.bytes().map_err(codec)?)?;
        let value = Self {
            step,
            budget_epoch: epoch,
            budget_fingerprint,
            decisions,
            remaining_tickets: remaining,
            fingerprint: fp,
        };
        if d.position() != bytes.len()
            || hash(TRACE_V2_DOMAIN, &value.encode_body()?) != fp
            || value.encode_canonical()? != bytes
        {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum ResidencyDisposition {
    Request = 1,
    Renew = 2,
    Expire = 3,
    Bypass = 4,
}
impl TryFrom<u8> for ResidencyDisposition {
    type Error = SignificanceSchedulerError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Request),
            2 => Ok(Self::Renew),
            3 => Ok(Self::Expire),
            4 => Ok(Self::Bypass),
            _ => Err(SignificanceSchedulerError::Invalid("residency disposition")),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResidencyIntentV1 {
    pub target_descriptor: [u8; 32],
    pub request_epoch: u64,
    pub domain: ConsumerDomainV1,
    pub lease_steps: u16,
    pub disposition: ResidencyDisposition,
}
impl ResidencyIntentV1 {
    pub fn new(
        target: [u8; 32],
        epoch: u64,
        domain: ConsumerDomainV1,
        lease: u16,
        disposition: ResidencyDisposition,
    ) -> Result<Self, SignificanceSchedulerError> {
        if target == [0; 32]
            || epoch == 0
            || domain != ConsumerDomainV1::Streaming
            || lease == 0
            || lease > 256
        {
            return Err(SignificanceSchedulerError::Invalid("residency intent"));
        }
        Ok(Self {
            target_descriptor: target,
            request_epoch: epoch,
            domain,
            lease_steps: lease,
            disposition,
        })
    }
    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        Encoder::new(&mut out)
            .array(6)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.target_descriptor))
            .and_then(|e| e.u64(self.request_epoch))
            .and_then(|e| e.u16(self.domain.code()))
            .and_then(|e| e.u16(self.lease_steps))
            .and_then(|e| e.u8(self.disposition as u8))
            .map_err(codec)?;
        Ok(out)
    }
    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(6) || d.u16().map_err(codec)? != CONTRACT_VERSION {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let value = Self::new(
            bytes32(d.bytes().map_err(codec)?)?,
            d.u64().map_err(codec)?,
            ConsumerDomainV1::try_from(d.u16().map_err(codec)?)?,
            d.u16().map_err(codec)?,
            ResidencyDisposition::try_from(d.u8().map_err(codec)?)?,
        )?;
        if d.position() != bytes.len() || value.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(value)
    }
}

pub const C5_HOSTILE_IDS: [&str; 92] = [
    "domain.unknown-code",
    "domain.zero-code",
    "domain.missing-required",
    "domain.duplicate-required",
    "domain.swapped-map",
    "domain.map-nonmonotone",
    "domain.private-score",
    "truth.packet-zero",
    "truth.packet-mismatch",
    "truth.packet-tier-forged",
    "truth.packet-epoch-mismatch",
    "truth.packet-target-mismatch",
    "truth.policy-mismatch",
    "truth.domain-map-set-mismatch",
    "truth.protection-erased",
    "truth.cross-domain-interference",
    "ticket.unknown-domain",
    "ticket.zero-id",
    "ticket.duplicate-id",
    "ticket.conflicting-id",
    "ticket.unknown-work-class",
    "ticket.unknown-dependency",
    "ticket.self-dependency",
    "ticket.dependency-cycle",
    "ticket.cancellation-cycle",
    "ticket.oversized-graph",
    "ticket.oversized-dependencies",
    "fallback.missing",
    "fallback.same-cost",
    "fallback.more-expensive",
    "fallback.cross-target",
    "fallback.cross-epoch",
    "fallback.cross-domain",
    "fallback.cross-work-class",
    "fallback.cross-resource",
    "fallback.nested",
    "admission.zero-budget",
    "admission.reserve-over-budget",
    "admission.budget-epoch-mismatch",
    "admission.impossible-safety",
    "admission.deadline-zero",
    "admission.cost-overflow",
    "admission.rejection-unreceipted",
    "budget.noncanonical",
    "budget.fingerprint-mismatch",
    "dispatch.nondeterministic-tie",
    "dispatch.dependency-before-ready",
    "dispatch.donation-persisted",
    "dispatch.donation-after-cancel",
    "dispatch.resource-cross-charge",
    "fairness.background-starved",
    "fairness.debt-overflow",
    "fairness.domain-monopoly",
    "fairness.diagnosis-missing",
    "thrash.focus-oscillation",
    "thrash.route-reversal-stale-work",
    "cancel.stale-epoch",
    "cancel.child-cancels-parent",
    "cancel.missing-acknowledgement",
    "cancel.settle-before-acknowledge",
    "cancel.epoch-advance-untraced",
    "completion.pending-accepted",
    "completion.inactive-fallback-accepted",
    "completion.rejected-accepted",
    "completion.cancelled-accepted",
    "completion.stale-epoch-accepted",
    "completion.duplicate-accepted",
    "completion.terminal-rewrite",
    "completion.partial-output-accepted",
    "residency.zero-target",
    "residency.zero-lease",
    "residency.stale-epoch",
    "residency.unbounded-lease",
    "residency.expired-retained",
    "residency.bypass-mutates",
    "residency.thrash-untraced",
    "trace.unknown-decision-code",
    "trace.missing-domain",
    "trace.missing-work-class",
    "trace.packet-mismatch",
    "trace.budget-mismatch",
    "trace.reordered-decision",
    "trace.trailing-bytes",
    "trace.replay-drift",
    "authority.runtime-controller",
    "authority.runtime-executor",
    "authority.cache-mutation",
    "authority.storage-mutation",
    "authority.product-weight",
    "authority.ai-generation",
    "authority.rendering-implementation",
    "authority.kernel-mutation",
];
