use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{CONTRACT_VERSION, ImportanceTier, SignificanceSchedulerError, bytes32, codec, hash};

const TICKET_DOMAIN: &[u8] = b"mindwarp/scheduler/ticket/v1\0";
const TRACE_DOMAIN: &[u8] = b"mindwarp/scheduler/trace/v1\0";
const MAX_TICKETS: usize = 256;
const MAX_DEPENDENCIES: usize = 16;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(u8)]
pub enum ResourceClass {
    Main = 0,
    Cpu = 1,
    Gpu = 2,
    Io = 3,
}

impl ResourceClass {
    pub const ALL: [Self; 4] = [Self::Main, Self::Cpu, Self::Gpu, Self::Io];
}

impl TryFrom<u8> for ResourceClass {
    type Error = SignificanceSchedulerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Main),
            1 => Ok(Self::Cpu),
            2 => Ok(Self::Gpu),
            3 => Ok(Self::Io),
            _ => Err(SignificanceSchedulerError::Invalid("resource class")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(u8)]
pub enum DeadlineClass {
    QualityTarget = 0,
    VisibleMinimum = 1,
    InteractionSafety = 2,
}

impl TryFrom<u8> for DeadlineClass {
    type Error = SignificanceSchedulerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::QualityTarget),
            1 => Ok(Self::VisibleMinimum),
            2 => Ok(Self::InteractionSafety),
            _ => Err(SignificanceSchedulerError::Invalid("deadline class")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorkTicket {
    pub id: [u8; 32],
    pub target_descriptor: [u8; 32],
    pub request_epoch: u64,
    pub consumer: u16,
    pub work_class: u16,
    pub resource: ResourceClass,
    pub cost_units: u32,
    pub dependencies: Vec<[u8; 32]>,
    pub deadline_class: DeadlineClass,
    pub due_step: u64,
    pub fallback: Option<[u8; 32]>,
    pub cancellation_parent: Option<[u8; 32]>,
    pub importance_packet: [u8; 32],
    pub importance_tier: ImportanceTier,
}

impl WorkTicket {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: [u8; 32],
        target_descriptor: [u8; 32],
        request_epoch: u64,
        consumer: u16,
        work_class: u16,
        resource: ResourceClass,
        cost_units: u32,
        dependencies: Vec<[u8; 32]>,
        deadline_class: DeadlineClass,
        due_step: u64,
        fallback: Option<[u8; 32]>,
        cancellation_parent: Option<[u8; 32]>,
        importance_packet: [u8; 32],
        importance_tier: ImportanceTier,
    ) -> Result<Self, SignificanceSchedulerError> {
        if id == [0; 32]
            || request_epoch == 0
            || consumer == 0
            || work_class == 0
            || cost_units == 0
            || due_step == 0
            || dependencies.len() > MAX_DEPENDENCIES
            || dependencies.iter().any(|dependency| dependency == &id)
            || !dependencies.windows(2).all(|pair| pair[0] < pair[1])
            || fallback == Some(id)
            || cancellation_parent == Some(id)
        {
            return Err(SignificanceSchedulerError::Invalid("work ticket"));
        }
        Ok(Self {
            id,
            target_descriptor,
            request_epoch,
            consumer,
            work_class,
            resource,
            cost_units,
            dependencies,
            deadline_class,
            due_step,
            fallback,
            cancellation_parent,
            importance_packet,
            importance_tier,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(15)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.id))
            .and_then(|e| e.bytes(&self.target_descriptor))
            .and_then(|e| e.u64(self.request_epoch))
            .and_then(|e| e.u16(self.consumer))
            .and_then(|e| e.u16(self.work_class))
            .and_then(|e| e.u8(self.resource as u8))
            .and_then(|e| e.u32(self.cost_units))
            .and_then(|e| e.array(self.dependencies.len() as u64))
            .map_err(codec)?;
        for dependency in &self.dependencies {
            encoder.bytes(dependency).map_err(codec)?;
        }
        encoder
            .u8(self.deadline_class as u8)
            .and_then(|e| e.u64(self.due_step))
            .map_err(codec)?;
        encode_optional_id(&mut encoder, self.fallback)?;
        encode_optional_id(&mut encoder, self.cancellation_parent)?;
        encoder
            .bytes(&self.importance_packet)
            .and_then(|e| e.u8(self.importance_tier as u8))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(15)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let id = bytes32(decoder.bytes().map_err(codec)?)?;
        let target_descriptor = bytes32(decoder.bytes().map_err(codec)?)?;
        let request_epoch = decoder.u64().map_err(codec)?;
        let consumer = decoder.u16().map_err(codec)?;
        let work_class = decoder.u16().map_err(codec)?;
        let resource = ResourceClass::try_from(decoder.u8().map_err(codec)?)?;
        let cost_units = decoder.u32().map_err(codec)?;
        let count = decoder
            .array()
            .map_err(codec)?
            .ok_or(SignificanceSchedulerError::NonCanonical)? as usize;
        if count > MAX_DEPENDENCIES {
            return Err(SignificanceSchedulerError::Invalid("dependency count"));
        }
        let mut dependencies = Vec::with_capacity(count);
        for _ in 0..count {
            dependencies.push(bytes32(decoder.bytes().map_err(codec)?)?);
        }
        let ticket = Self::new(
            id,
            target_descriptor,
            request_epoch,
            consumer,
            work_class,
            resource,
            cost_units,
            dependencies,
            DeadlineClass::try_from(decoder.u8().map_err(codec)?)?,
            decoder.u64().map_err(codec)?,
            decode_optional_id(&mut decoder)?,
            decode_optional_id(&mut decoder)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            ImportanceTier::try_from(decoder.u8().map_err(codec)?)?,
        )?;
        if decoder.position() != bytes.len() || ticket.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(ticket)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        Ok(hash(TICKET_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct BudgetEnvelope {
    pub epoch: u64,
    pub units: [u32; 4],
    pub safety_reserve: [u32; 4],
    pub max_service_debt: u16,
}

impl BudgetEnvelope {
    pub fn new(
        epoch: u64,
        units: [u32; 4],
        safety_reserve: [u32; 4],
        max_service_debt: u16,
    ) -> Result<Self, SignificanceSchedulerError> {
        if epoch == 0
            || max_service_debt == 0
            || units.iter().all(|unit| *unit == 0)
            || safety_reserve
                .iter()
                .zip(units)
                .any(|(reserve, unit)| *reserve > unit)
        {
            return Err(SignificanceSchedulerError::Invalid("budget envelope"));
        }
        Ok(Self {
            epoch,
            units,
            safety_reserve,
            max_service_debt,
        })
    }

    pub fn units_for(self, resource: ResourceClass) -> u32 {
        self.units[resource as usize]
    }

    pub fn reserve_for(self, resource: ResourceClass) -> u32 {
        self.safety_reserve[resource as usize]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum TicketState {
    InactiveFallback,
    Pending,
    Running,
    CompletedAccepted,
    CancelRequested,
    CancelAcknowledged,
    CancelSettled,
    CompletedDiscarded,
    Rejected,
}

impl TicketState {
    fn is_failed_terminal(self) -> bool {
        matches!(
            self,
            Self::CancelSettled | Self::CompletedDiscarded | Self::Rejected
        )
    }

    fn is_finished(self) -> bool {
        matches!(
            self,
            Self::CompletedAccepted
                | Self::CancelSettled
                | Self::CompletedDiscarded
                | Self::Rejected
        )
    }
}

#[derive(Clone, Debug)]
struct RuntimeTicket {
    ticket: WorkTicket,
    remaining: u32,
    service_debt: u16,
    state: TicketState,
    effective_class: DeadlineClass,
    effective_tier: ImportanceTier,
    effective_due: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DecisionKind {
    Executed,
    CompletedAccepted,
    CompletedDiscarded,
    CancelRequested,
    CancelAcknowledged,
    CancelSettled,
    FallbackActivated,
    DeadlineRejected,
    DependencyRejected,
}

impl DecisionKind {
    fn stable_code(self) -> u8 {
        match self {
            Self::Executed => 1,
            Self::CompletedAccepted => 2,
            Self::CompletedDiscarded => 3,
            Self::CancelRequested => 4,
            Self::CancelAcknowledged => 5,
            Self::CancelSettled => 6,
            Self::FallbackActivated => 7,
            Self::DeadlineRejected => 8,
            Self::DependencyRejected => 9,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SchedulerDecision {
    pub ticket_id: [u8; 32],
    pub kind: DecisionKind,
    pub resource: ResourceClass,
    pub units: u32,
    pub reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PressureTrace {
    pub step: u64,
    pub budget_epoch: u64,
    pub decisions: Vec<SchedulerDecision>,
    pub remaining_tickets: usize,
    pub fingerprint: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct ReferenceScheduler {
    budget: BudgetEnvelope,
    tickets: BTreeMap<[u8; 32], RuntimeTicket>,
    dependency_order: Vec<[u8; 32]>,
    cancellation_children: BTreeMap<[u8; 32], Vec<[u8; 32]>>,
    current_epochs: BTreeMap<[u8; 32], u64>,
    step: u64,
}

impl ReferenceScheduler {
    pub fn new(
        tickets: Vec<WorkTicket>,
        budget: BudgetEnvelope,
    ) -> Result<Self, SignificanceSchedulerError> {
        if tickets.is_empty() || tickets.len() > MAX_TICKETS {
            return Err(SignificanceSchedulerError::Invalid("ticket count"));
        }
        let mut by_id = BTreeMap::new();
        for ticket in tickets {
            if by_id.insert(ticket.id, ticket).is_some() {
                return Err(SignificanceSchedulerError::DuplicateTicket);
            }
        }
        validate_links(&by_id)?;
        let order = topological_order(&by_id, false)?;
        topological_order(&by_id, true)?;
        let fallback_ids: BTreeSet<[u8; 32]> = by_id.values().filter_map(|t| t.fallback).collect();
        let mut runtime: BTreeMap<[u8; 32], RuntimeTicket> = by_id
            .into_iter()
            .map(|(id, ticket)| {
                let state = if fallback_ids.contains(&id) {
                    TicketState::InactiveFallback
                } else {
                    TicketState::Pending
                };
                (
                    id,
                    RuntimeTicket {
                        remaining: ticket.cost_units,
                        service_debt: 0,
                        state,
                        effective_class: ticket.deadline_class,
                        effective_tier: ticket.importance_tier,
                        effective_due: ticket.due_step,
                        ticket,
                    },
                )
            })
            .collect();
        for dependent_id in order.iter().rev() {
            let dependent = runtime.get(dependent_id).unwrap().clone();
            for dependency_id in &dependent.ticket.dependencies {
                let dependency = runtime.get_mut(dependency_id).unwrap();
                dependency.effective_class =
                    dependency.effective_class.max(dependent.effective_class);
                dependency.effective_tier = dependency.effective_tier.max(dependent.effective_tier);
                dependency.effective_due = dependency.effective_due.min(dependent.effective_due);
            }
        }
        validate_safety_admission(&runtime, budget)?;
        let mut cancellation_children: BTreeMap<[u8; 32], Vec<[u8; 32]>> = BTreeMap::new();
        let mut current_epochs: BTreeMap<[u8; 32], u64> = BTreeMap::new();
        for runtime_ticket in runtime.values() {
            if let Some(parent) = runtime_ticket.ticket.cancellation_parent {
                cancellation_children
                    .entry(parent)
                    .or_default()
                    .push(runtime_ticket.ticket.id);
            }
            current_epochs
                .entry(runtime_ticket.ticket.target_descriptor)
                .and_modify(|epoch| *epoch = (*epoch).max(runtime_ticket.ticket.request_epoch))
                .or_insert(runtime_ticket.ticket.request_epoch);
        }
        for children in cancellation_children.values_mut() {
            children.sort();
        }
        for runtime_ticket in runtime.values_mut() {
            if runtime_ticket.ticket.request_epoch
                < current_epochs[&runtime_ticket.ticket.target_descriptor]
                && runtime_ticket.state != TicketState::InactiveFallback
            {
                runtime_ticket.state = TicketState::CompletedDiscarded;
            }
        }
        Ok(Self {
            budget,
            tickets: runtime,
            dependency_order: order,
            cancellation_children,
            current_epochs,
            step: 0,
        })
    }

    pub fn state(&self, id: [u8; 32]) -> Result<TicketState, SignificanceSchedulerError> {
        self.tickets
            .get(&id)
            .map(|ticket| ticket.state)
            .ok_or(SignificanceSchedulerError::UnknownTicket)
    }

    pub fn remaining(&self, id: [u8; 32]) -> Result<u32, SignificanceSchedulerError> {
        self.tickets
            .get(&id)
            .map(|ticket| ticket.remaining)
            .ok_or(SignificanceSchedulerError::UnknownTicket)
    }

    pub fn effective_priority(
        &self,
        id: [u8; 32],
    ) -> Result<(DeadlineClass, ImportanceTier, u64), SignificanceSchedulerError> {
        let ticket = self
            .tickets
            .get(&id)
            .ok_or(SignificanceSchedulerError::UnknownTicket)?;
        Ok((
            ticket.effective_class,
            ticket.effective_tier,
            ticket.effective_due,
        ))
    }

    pub fn request_cancel(
        &mut self,
        id: [u8; 32],
        epoch: u64,
    ) -> Result<Vec<SchedulerDecision>, SignificanceSchedulerError> {
        let ticket = self
            .tickets
            .get(&id)
            .ok_or(SignificanceSchedulerError::UnknownTicket)?;
        if ticket.ticket.request_epoch != epoch {
            return Err(SignificanceSchedulerError::StaleEpoch);
        }
        let mut queue = VecDeque::from([id]);
        let mut decisions = Vec::new();
        while let Some(current) = queue.pop_front() {
            if let Some(runtime) = self.tickets.get_mut(&current) {
                if matches!(runtime.state, TicketState::Pending | TicketState::Running) {
                    runtime.state = TicketState::CancelRequested;
                    decisions.push(decision(
                        runtime,
                        DecisionKind::CancelRequested,
                        0,
                        "explicit cancellation",
                    ));
                }
            }
            if let Some(children) = self.cancellation_children.get(&current) {
                queue.extend(children.iter().copied());
            }
        }
        self.recompute_effective_priorities();
        Ok(decisions)
    }

    pub fn acknowledge_cancel(
        &mut self,
        id: [u8; 32],
    ) -> Result<SchedulerDecision, SignificanceSchedulerError> {
        let runtime = self
            .tickets
            .get_mut(&id)
            .ok_or(SignificanceSchedulerError::UnknownTicket)?;
        if runtime.state != TicketState::CancelRequested {
            return Err(SignificanceSchedulerError::InvalidTransition);
        }
        runtime.state = TicketState::CancelAcknowledged;
        Ok(decision(
            runtime,
            DecisionKind::CancelAcknowledged,
            0,
            "worker acknowledged cancellation",
        ))
    }

    pub fn settle_cancel(
        &mut self,
        id: [u8; 32],
    ) -> Result<Vec<SchedulerDecision>, SignificanceSchedulerError> {
        let fallback = {
            let runtime = self
                .tickets
                .get_mut(&id)
                .ok_or(SignificanceSchedulerError::UnknownTicket)?;
            if runtime.state != TicketState::CancelAcknowledged {
                return Err(SignificanceSchedulerError::InvalidTransition);
            }
            runtime.state = TicketState::CancelSettled;
            runtime.ticket.fallback
        };
        let mut decisions = vec![decision(
            self.tickets.get(&id).unwrap(),
            DecisionKind::CancelSettled,
            0,
            "cancelled output settled and discarded",
        )];
        if let Some(fallback) = fallback {
            if let Some(activated) = self.activate_fallback(fallback) {
                decisions.push(activated);
            }
        }
        Ok(decisions)
    }

    pub fn advance_target_epoch(
        &mut self,
        target: [u8; 32],
        epoch: u64,
    ) -> Result<(), SignificanceSchedulerError> {
        let current = self.current_epochs.entry(target).or_insert(epoch);
        if epoch <= *current {
            return Err(SignificanceSchedulerError::StaleEpoch);
        }
        *current = epoch;
        for runtime in self.tickets.values_mut() {
            if runtime.ticket.target_descriptor == target
                && runtime.ticket.request_epoch < epoch
                && matches!(runtime.state, TicketState::Pending | TicketState::Running)
            {
                runtime.state = TicketState::CancelRequested;
            }
        }
        Ok(())
    }

    pub fn record_external_completion(
        &mut self,
        id: [u8; 32],
        epoch: u64,
    ) -> Result<SchedulerDecision, SignificanceSchedulerError> {
        let current_epochs = &self.current_epochs;
        let runtime = self
            .tickets
            .get_mut(&id)
            .ok_or(SignificanceSchedulerError::UnknownTicket)?;
        let stale = epoch != runtime.ticket.request_epoch
            || current_epochs[&runtime.ticket.target_descriptor] != epoch
            || matches!(
                runtime.state,
                TicketState::CancelRequested
                    | TicketState::CancelAcknowledged
                    | TicketState::CancelSettled
            );
        runtime.remaining = 0;
        runtime.state = if stale {
            TicketState::CompletedDiscarded
        } else {
            TicketState::CompletedAccepted
        };
        Ok(decision(
            runtime,
            if stale {
                DecisionKind::CompletedDiscarded
            } else {
                DecisionKind::CompletedAccepted
            },
            0,
            if stale {
                "stale or cancelled completion"
            } else {
                "completion accepted"
            },
        ))
    }

    pub fn step(&mut self) -> PressureTrace {
        let mut decisions = Vec::new();
        self.reject_failed_dependencies(&mut decisions);
        self.expire_overdue(&mut decisions);
        self.recompute_effective_priorities();
        let runnable: Vec<[u8; 32]> = self
            .tickets
            .iter()
            .filter(|(id, runtime)| {
                runtime.effective_class != DeadlineClass::InteractionSafety
                    && self.is_runnable(**id)
            })
            .map(|(id, _)| *id)
            .collect();
        for id in runnable {
            let runtime = self.tickets.get_mut(&id).unwrap();
            runtime.service_debt = runtime
                .service_debt
                .saturating_add(1)
                .min(self.budget.max_service_debt);
        }
        for resource in ResourceClass::ALL {
            let reserve = self.budget.reserve_for(resource);
            let safety_used = self.dispatch(resource, reserve, true, &mut decisions);
            let general_budget = self.budget.units_for(resource).saturating_sub(safety_used);
            self.dispatch(resource, general_budget, false, &mut decisions);
        }
        let remaining_tickets = self
            .tickets
            .values()
            .filter(|ticket| {
                !ticket.state.is_finished() && ticket.state != TicketState::InactiveFallback
            })
            .count();
        let fingerprint =
            trace_fingerprint(self.step, self.budget.epoch, &decisions, remaining_tickets);
        let trace = PressureTrace {
            step: self.step,
            budget_epoch: self.budget.epoch,
            decisions,
            remaining_tickets,
            fingerprint,
        };
        self.step = self.step.saturating_add(1);
        trace
    }

    fn is_runnable(&self, id: [u8; 32]) -> bool {
        let runtime = &self.tickets[&id];
        matches!(runtime.state, TicketState::Pending | TicketState::Running)
            && runtime
                .ticket
                .dependencies
                .iter()
                .all(|dependency| self.tickets[dependency].state == TicketState::CompletedAccepted)
    }

    fn recompute_effective_priorities(&mut self) {
        for runtime in self.tickets.values_mut() {
            runtime.effective_class = runtime.ticket.deadline_class;
            runtime.effective_tier = runtime.ticket.importance_tier;
            runtime.effective_due = runtime.ticket.due_step;
        }
        for dependent_id in self.dependency_order.iter().rev() {
            let dependent = self.tickets.get(dependent_id).unwrap().clone();
            if !matches!(dependent.state, TicketState::Pending | TicketState::Running) {
                continue;
            }
            for dependency_id in &dependent.ticket.dependencies {
                let dependency = self.tickets.get_mut(dependency_id).unwrap();
                if matches!(
                    dependency.state,
                    TicketState::Pending | TicketState::Running
                ) {
                    dependency.effective_class =
                        dependency.effective_class.max(dependent.effective_class);
                    dependency.effective_tier =
                        dependency.effective_tier.max(dependent.effective_tier);
                    dependency.effective_due =
                        dependency.effective_due.min(dependent.effective_due);
                }
            }
        }
    }

    fn dispatch(
        &mut self,
        resource: ResourceClass,
        mut budget: u32,
        safety_only: bool,
        decisions: &mut Vec<SchedulerDecision>,
    ) -> u32 {
        let initial = budget;
        while budget > 0 {
            let mut candidates: Vec<[u8; 32]> = self
                .tickets
                .iter()
                .filter(|(id, runtime)| {
                    runtime.ticket.resource == resource
                        && self.is_runnable(**id)
                        && if safety_only {
                            runtime.effective_class == DeadlineClass::InteractionSafety
                        } else {
                            runtime.effective_class != DeadlineClass::InteractionSafety
                        }
                })
                .map(|(id, _)| *id)
                .collect();
            if candidates.is_empty() {
                break;
            }
            candidates.sort_by(|left, right| self.compare_candidates(*left, *right, safety_only));
            let id = candidates[0];
            let runtime = self.tickets.get_mut(&id).unwrap();
            let units = runtime.remaining.min(budget);
            runtime.remaining -= units;
            runtime.service_debt = 0;
            runtime.state = TicketState::Running;
            budget -= units;
            decisions.push(decision(
                runtime,
                DecisionKind::Executed,
                units,
                if safety_only {
                    "bounded safety reserve"
                } else {
                    "general fair dispatch"
                },
            ));
            if runtime.remaining == 0 {
                runtime.state = TicketState::CompletedAccepted;
                decisions.push(decision(
                    runtime,
                    DecisionKind::CompletedAccepted,
                    0,
                    "reference work completed",
                ));
            }
        }
        initial - budget
    }

    fn compare_candidates(&self, left: [u8; 32], right: [u8; 32], safety: bool) -> Ordering {
        let left = &self.tickets[&left];
        let right = &self.tickets[&right];
        if safety {
            left.effective_due
                .cmp(&right.effective_due)
                .then_with(|| right.effective_tier.cmp(&left.effective_tier))
                .then_with(|| left.ticket.id.cmp(&right.ticket.id))
        } else {
            let left_starved = left.service_debt >= self.budget.max_service_debt;
            let right_starved = right.service_debt >= self.budget.max_service_debt;
            right_starved
                .cmp(&left_starved)
                .then_with(|| right.effective_class.cmp(&left.effective_class))
                .then_with(|| left.effective_due.cmp(&right.effective_due))
                .then_with(|| right.effective_tier.cmp(&left.effective_tier))
                .then_with(|| right.service_debt.cmp(&left.service_debt))
                .then_with(|| left.ticket.id.cmp(&right.ticket.id))
        }
    }

    fn reject_failed_dependencies(&mut self, decisions: &mut Vec<SchedulerDecision>) {
        let failed: Vec<[u8; 32]> = self
            .tickets
            .iter()
            .filter(|(_, runtime)| {
                matches!(runtime.state, TicketState::Pending | TicketState::Running)
                    && runtime
                        .ticket
                        .dependencies
                        .iter()
                        .any(|dependency| self.tickets[dependency].state.is_failed_terminal())
            })
            .map(|(id, _)| *id)
            .collect();
        for id in failed {
            let fallback = self.tickets[&id].ticket.fallback;
            let runtime = self.tickets.get_mut(&id).unwrap();
            runtime.state = TicketState::Rejected;
            decisions.push(decision(
                runtime,
                DecisionKind::DependencyRejected,
                0,
                "dependency failed",
            ));
            if let Some(fallback) = fallback {
                if let Some(activated) = self.activate_fallback(fallback) {
                    decisions.push(activated);
                }
            }
        }
    }

    fn expire_overdue(&mut self, decisions: &mut Vec<SchedulerDecision>) {
        let overdue: Vec<[u8; 32]> = self
            .tickets
            .iter()
            .filter(|(_, runtime)| {
                matches!(runtime.state, TicketState::Pending | TicketState::Running)
                    && self.step > runtime.ticket.due_step
            })
            .map(|(id, _)| *id)
            .collect();
        for id in overdue {
            let fallback = self.tickets[&id].ticket.fallback;
            let runtime = self.tickets.get_mut(&id).unwrap();
            runtime.state = TicketState::Rejected;
            decisions.push(decision(
                runtime,
                DecisionKind::DeadlineRejected,
                0,
                "deadline expired before completion",
            ));
            if let Some(fallback) = fallback {
                if let Some(activated) = self.activate_fallback(fallback) {
                    decisions.push(activated);
                }
            }
        }
    }

    fn activate_fallback(&mut self, id: [u8; 32]) -> Option<SchedulerDecision> {
        let runtime = self.tickets.get_mut(&id)?;
        if runtime.state != TicketState::InactiveFallback {
            return None;
        }
        runtime.state = TicketState::Pending;
        Some(decision(
            runtime,
            DecisionKind::FallbackActivated,
            0,
            "validated cheaper fallback",
        ))
    }
}

fn validate_links(
    tickets: &BTreeMap<[u8; 32], WorkTicket>,
) -> Result<(), SignificanceSchedulerError> {
    for ticket in tickets.values() {
        for dependency in &ticket.dependencies {
            let target = tickets
                .get(dependency)
                .ok_or(SignificanceSchedulerError::UnknownDependency)?;
            if target.target_descriptor != ticket.target_descriptor
                || target.request_epoch != ticket.request_epoch
            {
                return Err(SignificanceSchedulerError::UnknownDependency);
            }
        }
        if let Some(fallback_id) = ticket.fallback {
            let fallback = tickets
                .get(&fallback_id)
                .ok_or(SignificanceSchedulerError::UnknownFallback)?;
            if fallback.target_descriptor != ticket.target_descriptor
                || fallback.request_epoch != ticket.request_epoch
                || fallback.resource != ticket.resource
                || fallback.cost_units >= ticket.cost_units
                || fallback.fallback.is_some()
                || !fallback.dependencies.is_empty()
            {
                return Err(SignificanceSchedulerError::InvalidFallback);
            }
        }
        if let Some(parent) = ticket.cancellation_parent {
            let parent = tickets
                .get(&parent)
                .ok_or(SignificanceSchedulerError::InvalidCancellationTree)?;
            if parent.target_descriptor != ticket.target_descriptor
                || parent.request_epoch != ticket.request_epoch
            {
                return Err(SignificanceSchedulerError::InvalidCancellationTree);
            }
        }
    }
    Ok(())
}

fn topological_order(
    tickets: &BTreeMap<[u8; 32], WorkTicket>,
    cancellation: bool,
) -> Result<Vec<[u8; 32]>, SignificanceSchedulerError> {
    let mut indegree: BTreeMap<[u8; 32], usize> = tickets.keys().map(|id| (*id, 0)).collect();
    let mut children: BTreeMap<[u8; 32], Vec<[u8; 32]>> = BTreeMap::new();
    for ticket in tickets.values() {
        let parents: Vec<[u8; 32]> = if cancellation {
            ticket.cancellation_parent.into_iter().collect()
        } else {
            ticket.dependencies.clone()
        };
        for parent in parents {
            *indegree.get_mut(&ticket.id).unwrap() += 1;
            children.entry(parent).or_default().push(ticket.id);
        }
    }
    let mut ready: BTreeSet<[u8; 32]> = indegree
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut order = Vec::with_capacity(tickets.len());
    while let Some(id) = ready.pop_first() {
        order.push(id);
        if let Some(next) = children.get(&id) {
            for child in next {
                let count = indegree.get_mut(child).unwrap();
                *count -= 1;
                if *count == 0 {
                    ready.insert(*child);
                }
            }
        }
    }
    if order.len() != tickets.len() {
        return Err(if cancellation {
            SignificanceSchedulerError::InvalidCancellationTree
        } else {
            SignificanceSchedulerError::DependencyCycle
        });
    }
    Ok(order)
}

fn validate_safety_admission(
    tickets: &BTreeMap<[u8; 32], RuntimeTicket>,
    budget: BudgetEnvelope,
) -> Result<(), SignificanceSchedulerError> {
    for resource in ResourceClass::ALL {
        let reserve = u64::from(budget.reserve_for(resource));
        let mut deadlines: Vec<u64> = tickets
            .values()
            .filter(|runtime| {
                runtime.ticket.resource == resource
                    && runtime.effective_class == DeadlineClass::InteractionSafety
                    && runtime.state != TicketState::InactiveFallback
            })
            .map(|runtime| runtime.effective_due)
            .collect();
        deadlines.sort_unstable();
        deadlines.dedup();
        for deadline in deadlines {
            let required: u64 = tickets
                .values()
                .filter(|runtime| {
                    runtime.ticket.resource == resource
                        && runtime.effective_class == DeadlineClass::InteractionSafety
                        && runtime.effective_due <= deadline
                        && runtime.state != TicketState::InactiveFallback
                })
                .map(|runtime| u64::from(runtime.ticket.cost_units))
                .sum();
            let available = reserve.saturating_mul(deadline.saturating_add(1));
            if reserve == 0 || required > available {
                return Err(SignificanceSchedulerError::AdmissionRejected);
            }
        }
    }
    Ok(())
}

fn decision(
    runtime: &RuntimeTicket,
    kind: DecisionKind,
    units: u32,
    reason: &'static str,
) -> SchedulerDecision {
    SchedulerDecision {
        ticket_id: runtime.ticket.id,
        kind,
        resource: runtime.ticket.resource,
        units,
        reason,
    }
}

fn trace_fingerprint(
    step: u64,
    budget_epoch: u64,
    decisions: &[SchedulerDecision],
    remaining: usize,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&step.to_be_bytes());
    bytes.extend_from_slice(&budget_epoch.to_be_bytes());
    bytes.extend_from_slice(&(remaining as u64).to_be_bytes());
    for decision in decisions {
        bytes.extend_from_slice(&decision.ticket_id);
        bytes.push(decision.resource as u8);
        bytes.extend_from_slice(&decision.units.to_be_bytes());
        bytes.push(decision.kind.stable_code());
        bytes.extend_from_slice(decision.reason.as_bytes());
    }
    hash(TRACE_DOMAIN, &bytes)
}

fn encode_optional_id(
    encoder: &mut Encoder<&mut Vec<u8>>,
    id: Option<[u8; 32]>,
) -> Result<(), SignificanceSchedulerError> {
    match id {
        Some(id) => encoder.bytes(&id).map_err(codec)?,
        None => encoder.null().map_err(codec)?,
    };
    Ok(())
}

fn decode_optional_id(
    decoder: &mut Decoder<'_>,
) -> Result<Option<[u8; 32]>, SignificanceSchedulerError> {
    match decoder.datatype().map_err(codec)? {
        minicbor::data::Type::Null => {
            decoder.null().map_err(codec)?;
            Ok(None)
        }
        minicbor::data::Type::Bytes => Ok(Some(bytes32(decoder.bytes().map_err(codec)?)?)),
        _ => Err(SignificanceSchedulerError::NonCanonical),
    }
}
