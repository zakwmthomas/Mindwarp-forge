use significance_scheduler::{
    BudgetEnvelope, CompletionReceiptV1, ConsumerDomainV1, DeadlineClass, DecisionKind,
    HysteresisPolicy, ImportancePacket, ImportanceTier, ReferenceScheduler, ResourceClass,
    SignalVector, SignificanceSchedulerError, SignificanceState, TicketState, WorkTicket,
};

#[allow(clippy::too_many_arguments)]
fn ticket(
    id: u8,
    domain: ConsumerDomainV1,
    resource: ResourceClass,
    cost: u32,
    dependencies: Vec<u8>,
    class: DeadlineClass,
    due: u64,
    tier: ImportanceTier,
    fallback: Option<u8>,
    cancellation_parent: Option<u8>,
) -> WorkTicket {
    WorkTicket::new(
        [id; 32],
        [7; 32],
        1,
        domain.code(),
        domain.code(),
        resource,
        cost,
        dependencies.into_iter().map(|value| [value; 32]).collect(),
        class,
        due,
        fallback.map(|value| [value; 32]),
        cancellation_parent.map(|value| [value; 32]),
        [9; 32],
        tier,
    )
    .unwrap()
}

fn budget(units: [u32; 4], reserve: [u32; 4], debt: u16) -> BudgetEnvelope {
    BudgetEnvelope::new(1, units, reserve, debt).unwrap()
}

fn completion(id: u8, epoch: u64, units: u32) -> CompletionReceiptV1 {
    CompletionReceiptV1::new([id; 32], epoch, units, [8; 32]).unwrap()
}

fn ordinary(id: u8, cost: u32) -> WorkTicket {
    ticket(
        id,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        cost,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Background,
        None,
        None,
    )
}

#[test]
fn dispatch_nondeterministic_tie() {
    // Hostile ID: dispatch.nondeterministic-tie
    let one = ordinary(1, 1);
    let two = ordinary(2, 1);
    let envelope = budget([0, 1, 0, 0], [0; 4], 3);
    let mut left = ReferenceScheduler::new(vec![two.clone(), one.clone()], envelope).unwrap();
    let mut right = ReferenceScheduler::new(vec![one, two], envelope).unwrap();
    let left_trace = left.step_strict().unwrap();
    let right_trace = right.step_strict().unwrap();
    assert_eq!(left_trace, right_trace);
    assert_eq!(left_trace.decisions[0].ticket_id, [1; 32]);
}

#[test]
fn dispatch_dependency_before_ready() {
    // Hostile ID: dispatch.dependency-before-ready
    let dependency = ordinary(1, 2);
    let dependent = ticket(
        2,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        1,
        vec![1],
        DeadlineClass::VisibleMinimum,
        10,
        ImportanceTier::Visible,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![dependent, dependency], budget([0, 1, 0, 0], [0; 4], 3))
            .unwrap();
    let trace = scheduler.step();
    assert!(
        trace
            .decisions
            .iter()
            .all(|decision| decision.ticket_id != [2; 32])
    );
    assert_eq!(scheduler.state([2; 32]).unwrap(), TicketState::Pending);
}

fn donated_scheduler() -> ReferenceScheduler {
    let dependency = ticket(
        1,
        ConsumerDomainV1::Ai,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Dormant,
        None,
        None,
    );
    let dependent = ticket(
        2,
        ConsumerDomainV1::Ai,
        ResourceClass::Cpu,
        1,
        vec![1],
        DeadlineClass::InteractionSafety,
        2,
        ImportanceTier::Critical,
        None,
        None,
    );
    ReferenceScheduler::new(
        vec![dependency, dependent],
        budget([0, 2, 0, 0], [0, 1, 0, 0], 3),
    )
    .unwrap()
}

#[test]
fn dispatch_donation_persisted() {
    // Hostile ID: dispatch.donation-persisted
    let mut scheduler = donated_scheduler();
    assert_eq!(
        scheduler.effective_priority([1; 32]).unwrap(),
        (
            DeadlineClass::InteractionSafety,
            ImportanceTier::Critical,
            2
        )
    );
    scheduler.request_cancel([2; 32], 1).unwrap();
    assert_eq!(
        scheduler.effective_priority([1; 32]).unwrap(),
        (DeadlineClass::QualityTarget, ImportanceTier::Dormant, 100)
    );
}

#[test]
fn dispatch_donation_after_cancel() {
    // Hostile ID: dispatch.donation-after-cancel
    let mut scheduler = donated_scheduler();
    scheduler.request_cancel([2; 32], 1).unwrap();
    let trace = scheduler.step();
    assert!(
        trace
            .decisions
            .iter()
            .all(|decision| decision.ticket_id != [2; 32])
    );
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::CancelRequested
    );
}

#[test]
fn dispatch_resource_cross_charge() {
    // Hostile ID: dispatch.resource-cross-charge
    let cpu = ordinary(1, 1);
    let gpu = ticket(
        2,
        ConsumerDomainV1::Rendering,
        ResourceClass::Gpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Background,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![cpu, gpu], budget([0, 0, 1, 0], [0; 4], 3)).unwrap();
    scheduler.step();
    assert_eq!(scheduler.remaining([1; 32]).unwrap(), 1);
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::CompletedAccepted
    );
}

#[test]
fn fairness_background_starved() {
    // Hostile ID: fairness.background-starved
    let urgent = ticket(
        1,
        ConsumerDomainV1::Simulation,
        ResourceClass::Cpu,
        10,
        vec![],
        DeadlineClass::VisibleMinimum,
        100,
        ImportanceTier::Critical,
        None,
        None,
    );
    let background = ticket(
        2,
        ConsumerDomainV1::Simulation,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Dormant,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![urgent, background], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    for _ in 0..4 {
        scheduler.step();
    }
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::CompletedAccepted
    );
}

#[test]
fn fairness_debt_overflow() {
    // Hostile ID: fairness.debt-overflow
    let stranded = ticket(
        1,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        u64::MAX,
        ImportanceTier::Background,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![stranded], budget([1, 0, 0, 0], [0; 4], u16::MAX)).unwrap();
    let mut diagnoses = 0;
    for _ in 0..=u16::MAX {
        diagnoses += scheduler
            .step()
            .decisions
            .iter()
            .filter(|decision| decision.kind == DecisionKind::StarvationDiagnosed)
            .count();
    }
    assert_eq!(diagnoses, 1);
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Pending);
}

#[test]
fn fairness_domain_monopoly() {
    // Hostile ID: fairness.domain-monopoly
    let tickets = ConsumerDomainV1::ALL
        .into_iter()
        .enumerate()
        .map(|(index, domain)| {
            ticket(
                (index + 1) as u8,
                domain,
                ResourceClass::Cpu,
                1,
                vec![],
                DeadlineClass::QualityTarget,
                100,
                ImportanceTier::Background,
                None,
                None,
            )
        })
        .collect();
    let mut scheduler = ReferenceScheduler::new(tickets, budget([0, 1, 0, 0], [0; 4], 2)).unwrap();
    for _ in 0..8 {
        scheduler.step();
    }
    for id in 1..=8 {
        assert_eq!(
            scheduler.state([id; 32]).unwrap(),
            TicketState::CompletedAccepted
        );
    }
}

#[test]
fn fairness_diagnosis_missing() {
    // Hostile ID: fairness.diagnosis-missing
    let stranded = ordinary(1, 1);
    let mut scheduler =
        ReferenceScheduler::new(vec![stranded], budget([1, 0, 0, 0], [0; 4], 2)).unwrap();
    scheduler.step();
    let trace = scheduler.step_strict().unwrap();
    assert!(
        trace
            .decisions
            .iter()
            .any(|decision| decision.kind == DecisionKind::StarvationDiagnosed)
    );
}

fn packet(focus: u16) -> ImportancePacket {
    ImportancePacket::new(
        [7; 32],
        1,
        SignalVector {
            focus,
            visibility: 0,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        0,
    )
    .unwrap()
}

#[test]
fn thrash_focus_oscillation() {
    // Hostile ID: thrash.focus-oscillation
    let policy = HysteresisPolicy::new([100, 500, 900], [50, 400, 800], 3).unwrap();
    let mut state = SignificanceState::default();
    assert_eq!(
        state.advance(&packet(510), policy, 1).unwrap(),
        ImportanceTier::Visible
    );
    assert_eq!(
        state.advance(&packet(390), policy, 2).unwrap(),
        ImportanceTier::Visible
    );
    assert_eq!(
        state.advance(&packet(510), policy, 3).unwrap(),
        ImportanceTier::Visible
    );
}

#[test]
fn thrash_route_reversal_stale_work() {
    // Hostile ID: thrash.route-reversal-stale-work
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.step();
    scheduler.advance_target_epoch([7; 32], 2).unwrap();
    let trace = scheduler.step_strict().unwrap();
    assert!(
        trace
            .decisions
            .iter()
            .any(|decision| decision.kind == DecisionKind::EpochAdvanced)
    );
    let late = scheduler.record_completion(completion(1, 1, 2)).unwrap();
    assert_eq!(late.kind, DecisionKind::CompletedDiscarded);
}

#[test]
fn cancel_stale_epoch() {
    // Hostile ID: cancel.stale-epoch
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 1)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    assert_eq!(
        scheduler.request_cancel([1; 32], 2).unwrap_err(),
        SignificanceSchedulerError::StaleEpoch
    );
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Pending);
}

#[test]
fn cancel_child_cancels_parent() {
    // Hostile ID: cancel.child-cancels-parent
    let parent = ordinary(1, 2);
    let child = ticket(
        2,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Background,
        None,
        Some(1),
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![parent, child], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.request_cancel([2; 32], 1).unwrap();
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Pending);
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::CancelRequested
    );
}

#[test]
fn cancel_missing_acknowledgement() {
    // Hostile ID: cancel.missing-acknowledgement
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 1)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.request_cancel([1; 32], 1).unwrap();
    assert_eq!(
        scheduler.settle_cancel([1; 32]).unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
}

#[test]
fn cancel_settle_before_acknowledge() {
    // Hostile ID: cancel.settle-before-acknowledge
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 1)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    assert_eq!(
        scheduler.settle_cancel([1; 32]).unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Pending);
}

#[test]
fn cancel_epoch_advance_untraced() {
    // Hostile ID: cancel.epoch-advance-untraced
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.advance_target_epoch([7; 32], 2).unwrap();
    let trace = scheduler.step_strict().unwrap();
    assert!(
        trace
            .decisions
            .iter()
            .any(|decision| decision.kind == DecisionKind::EpochAdvanced)
    );
}

#[test]
fn completion_pending_accepted() {
    // Hostile ID: completion.pending-accepted
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    assert_eq!(
        scheduler
            .record_completion(completion(1, 1, 2))
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Pending);
}

#[test]
fn completion_inactive_fallback_accepted() {
    // Hostile ID: completion.inactive-fallback-accepted
    let original = ticket(
        1,
        ConsumerDomainV1::Streaming,
        ResourceClass::Io,
        2,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Background,
        Some(2),
        None,
    );
    let fallback = ticket(
        2,
        ConsumerDomainV1::Streaming,
        ResourceClass::Io,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        100,
        ImportanceTier::Background,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![original, fallback], budget([0, 0, 0, 1], [0; 4], 3)).unwrap();
    assert_eq!(
        scheduler
            .record_completion(completion(2, 1, 1))
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::InactiveFallback
    );
}

#[test]
fn completion_rejected_accepted() {
    // Hostile ID: completion.rejected-accepted
    let expiring = ticket(
        1,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        2,
        vec![],
        DeadlineClass::QualityTarget,
        1,
        ImportanceTier::Background,
        None,
        None,
    );
    let mut scheduler =
        ReferenceScheduler::new(vec![expiring], budget([1, 0, 0, 0], [0; 4], 3)).unwrap();
    scheduler.step();
    scheduler.step();
    scheduler.step();
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Rejected);
    assert_eq!(
        scheduler
            .record_completion(completion(1, 1, 2))
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
}

#[test]
fn completion_cancelled_accepted() {
    // Hostile ID: completion.cancelled-accepted
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.request_cancel([1; 32], 1).unwrap();
    let decision = scheduler.record_completion(completion(1, 1, 2)).unwrap();
    assert_eq!(decision.kind, DecisionKind::CompletedDiscarded);
    assert_ne!(
        scheduler.state([1; 32]).unwrap(),
        TicketState::CompletedAccepted
    );
}

#[test]
fn completion_stale_epoch_accepted() {
    // Hostile ID: completion.stale-epoch-accepted
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.advance_target_epoch([7; 32], 2).unwrap();
    let decision = scheduler.record_completion(completion(1, 1, 2)).unwrap();
    assert_eq!(decision.kind, DecisionKind::CompletedDiscarded);
}

#[test]
fn completion_duplicate_accepted() {
    // Hostile ID: completion.duplicate-accepted
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 1)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.step();
    assert_eq!(
        scheduler.state([1; 32]).unwrap(),
        TicketState::CompletedAccepted
    );
    assert_eq!(
        scheduler
            .record_completion(completion(1, 1, 1))
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
}

#[test]
fn completion_partial_output_accepted() {
    // Hostile ID: completion.partial-output-accepted
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 3)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.step();
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Running);
    assert_eq!(
        scheduler
            .record_completion(completion(1, 1, 2))
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Running);
    assert_eq!(scheduler.remaining([1; 32]).unwrap(), 2);
}

#[test]
fn completion_terminal_rewrite() {
    // Hostile ID: completion.terminal-rewrite
    let mut scheduler =
        ReferenceScheduler::new(vec![ordinary(1, 2)], budget([0, 1, 0, 0], [0; 4], 3)).unwrap();
    scheduler.request_cancel([1; 32], 1).unwrap();
    scheduler.acknowledge_cancel([1; 32]).unwrap();
    scheduler.settle_cancel([1; 32]).unwrap();
    assert_eq!(
        scheduler
            .record_completion(completion(1, 1, 2))
            .unwrap()
            .kind,
        DecisionKind::CompletedDiscarded
    );
    assert_eq!(
        scheduler.state([1; 32]).unwrap(),
        TicketState::CancelSettled
    );
}
