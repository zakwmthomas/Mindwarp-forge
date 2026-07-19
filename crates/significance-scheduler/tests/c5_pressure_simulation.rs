use significance_scheduler::*;

fn packet(signal: u16, protection: u8) -> ImportancePacket {
    ImportancePacket::new(
        [7; 32],
        1,
        SignalVector {
            focus: signal,
            visibility: signal,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        protection,
    )
    .unwrap()
}
fn policy() -> HysteresisPolicy {
    HysteresisPolicy::new([100, 300, 600], [80, 250, 500], 2).unwrap()
}
fn maps() -> DomainFidelityMapSetV1 {
    DomainFidelityMapSetV1::new([
        [0, 2, 8, 16],
        [0, 1, 6, 14],
        [2, 8, 12, 16],
        [0, 2, 10, 16],
        [0, 0, 6, 16],
        [0, 4, 4, 10],
        [1, 5, 10, 16],
        [0, 3, 9, 15],
    ])
    .unwrap()
}
fn binding(
    signal: u16,
    protection: u8,
    state: SignificanceState,
    step: u64,
) -> ImportanceDecisionBindingV1 {
    ImportanceDecisionBindingV1::derive(&packet(signal, protection), policy(), state, step, &maps())
        .unwrap()
}
#[allow(clippy::too_many_arguments)]
fn ticket(
    id: u8,
    domain: ConsumerDomainV1,
    resource: ResourceClass,
    cost: u32,
    class: DeadlineClass,
    due: u64,
    fallback: Option<u8>,
    dependencies: Vec<u8>,
    binding: &ImportanceDecisionBindingV1,
) -> WorkTicket {
    WorkTicket::new(
        [id; 32],
        [7; 32],
        1,
        domain.code(),
        1,
        resource,
        cost,
        dependencies.into_iter().map(|id| [id; 32]).collect(),
        class,
        due,
        fallback.map(|id| [id; 32]),
        None,
        binding.packet_fingerprint(),
        binding.tier(),
    )
    .unwrap()
}

#[test]
fn pressure_stable_focus_replays_byte_exact() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let tickets: Vec<_> = ConsumerDomainV1::ALL
        .into_iter()
        .enumerate()
        .map(|(i, d)| {
            ticket(
                (i + 1) as u8,
                d,
                ResourceClass::ALL[i % 4],
                1,
                DeadlineClass::QualityTarget,
                20,
                None,
                vec![],
                &b,
            )
        })
        .collect();
    let budget = BudgetEnvelope::new(1, [2; 4], [0; 4], 3).unwrap();
    let mut a = ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
        .unwrap();
    let mut c = ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
        .unwrap();
    let left = a.step_strict().unwrap();
    let right = c.step_strict().unwrap();
    assert_eq!(
        left.encode_canonical().unwrap(),
        right.encode_canonical().unwrap()
    );
    left.verify_replay(&tickets, budget, &[b]).unwrap()
}

#[test]
fn pressure_focus_oscillation_does_not_flap_dispatch_truth() {
    let first = binding(650, 0, SignificanceState::default(), 1);
    let second = binding(490, 0, first.resulting_state(), 2);
    let third = binding(510, 0, second.resulting_state(), 3);
    assert_eq!(first.tier(), ImportanceTier::Critical);
    assert_eq!(second.tier(), ImportanceTier::Critical);
    assert_eq!(third.tier(), ImportanceTier::Critical);
    for b in [first, second, third] {
        let work = ticket(
            1,
            ConsumerDomainV1::Ai,
            ResourceClass::Cpu,
            1,
            DeadlineClass::QualityTarget,
            20,
            None,
            vec![],
            &b,
        );
        let mut scheduler = ReferenceScheduler::new_verified(
            vec![work],
            BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap(),
            &[b],
        )
        .unwrap();
        assert!(
            scheduler
                .step()
                .decisions
                .iter()
                .any(|d| d.kind == DecisionKind::Executed)
        );
    }
}

#[test]
fn pressure_protected_interaction_uses_reserve_first() {
    let b = binding(0, PROTECT_INTERACTION, SignificanceState::default(), 1);
    let safety = ticket(
        1,
        ConsumerDomainV1::Physics,
        ResourceClass::Cpu,
        1,
        DeadlineClass::InteractionSafety,
        1,
        None,
        vec![],
        &b,
    );
    let general = ticket(
        2,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        1,
        DeadlineClass::QualityTarget,
        20,
        None,
        vec![],
        &b,
    );
    let mut scheduler = ReferenceScheduler::new_verified(
        vec![general, safety],
        BudgetEnvelope::new(1, [0, 2, 0, 0], [0, 1, 0, 0], 2).unwrap(),
        &[b],
    )
    .unwrap();
    let trace = scheduler.step();
    let executed: Vec<_> = trace
        .decisions
        .iter()
        .filter(|d| d.kind == DecisionKind::Executed)
        .collect();
    assert_eq!(executed[0].ticket_id, [1; 32]);
}

#[test]
fn pressure_overload_emits_starvation_before_bounded_service() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let tickets = (1..=4)
        .map(|id| {
            ticket(
                id,
                ConsumerDomainV1::Simulation,
                ResourceClass::Cpu,
                2,
                DeadlineClass::QualityTarget,
                50,
                None,
                vec![],
                &b,
            )
        })
        .collect();
    let mut scheduler = ReferenceScheduler::new_verified(
        tickets,
        BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap(),
        &[b],
    )
    .unwrap();
    let mut diagnosed = false;
    for _ in 0..8 {
        diagnosed |= scheduler
            .step()
            .decisions
            .iter()
            .any(|d| d.kind == DecisionKind::StarvationDiagnosed);
    }
    assert!(diagnosed)
}

#[test]
fn pressure_route_reversal_quarantines_primary_and_fallback() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let original = ticket(
        1,
        ConsumerDomainV1::Streaming,
        ResourceClass::Io,
        2,
        DeadlineClass::QualityTarget,
        20,
        Some(2),
        vec![],
        &b,
    );
    let fallback = ticket(
        2,
        ConsumerDomainV1::Streaming,
        ResourceClass::Io,
        1,
        DeadlineClass::QualityTarget,
        20,
        None,
        vec![],
        &b,
    );
    let mut scheduler = ReferenceScheduler::new_verified(
        vec![original, fallback],
        BudgetEnvelope::new(1, [0, 0, 0, 1], [0; 4], 2).unwrap(),
        &[b],
    )
    .unwrap();
    scheduler.advance_target_epoch([7; 32], 2).unwrap();
    assert_eq!(
        scheduler.state([2; 32]).unwrap(),
        TicketState::CompletedDiscarded
    );
    assert!(
        !scheduler
            .step()
            .decisions
            .iter()
            .any(|d| d.kind == DecisionKind::Executed)
    )
}

#[test]
fn pressure_cancellation_requires_ack_and_discards_late_output() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let work = ticket(
        1,
        ConsumerDomainV1::Animation,
        ResourceClass::Cpu,
        2,
        DeadlineClass::QualityTarget,
        20,
        None,
        vec![],
        &b,
    );
    let mut scheduler = ReferenceScheduler::new_verified(
        vec![work],
        BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap(),
        &[b],
    )
    .unwrap();
    scheduler.step();
    scheduler.request_cancel([1; 32], 1).unwrap();
    assert!(scheduler.settle_cancel([1; 32]).is_err());
    scheduler.acknowledge_cancel([1; 32]).unwrap();
    scheduler.settle_cancel([1; 32]).unwrap();
    assert_eq!(
        scheduler
            .record_completion(CompletionReceiptV1::new([1; 32], 1, 2, [9; 32]).unwrap())
            .unwrap()
            .kind,
        DecisionKind::CompletedDiscarded
    )
}

#[test]
fn pressure_failed_dependency_activates_only_validated_fallback() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let dep = ticket(
        1,
        ConsumerDomainV1::Audio,
        ResourceClass::Cpu,
        1,
        DeadlineClass::QualityTarget,
        20,
        None,
        vec![],
        &b,
    );
    let original = ticket(
        2,
        ConsumerDomainV1::Audio,
        ResourceClass::Cpu,
        2,
        DeadlineClass::QualityTarget,
        20,
        Some(3),
        vec![1],
        &b,
    );
    let fallback = ticket(
        3,
        ConsumerDomainV1::Audio,
        ResourceClass::Cpu,
        1,
        DeadlineClass::QualityTarget,
        20,
        None,
        vec![],
        &b,
    );
    let mut scheduler = ReferenceScheduler::new_verified(
        vec![dep, original, fallback],
        BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap(),
        &[b],
    )
    .unwrap();
    scheduler.request_cancel([1; 32], 1).unwrap();
    scheduler.acknowledge_cancel([1; 32]).unwrap();
    scheduler.settle_cancel([1; 32]).unwrap();
    scheduler.step();
    assert!(matches!(
        scheduler.state([3; 32]).unwrap(),
        TicketState::Running | TicketState::CompletedAccepted
    ))
}

#[test]
fn pressure_exhausts_each_resource_without_cross_charge() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let tickets: Vec<_> = ResourceClass::ALL
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            ticket(
                (i + 1) as u8,
                ConsumerDomainV1::ALL[i],
                r,
                2,
                DeadlineClass::QualityTarget,
                20,
                None,
                vec![],
                &b,
            )
        })
        .collect();
    let mut scheduler = ReferenceScheduler::new_verified(
        tickets,
        BudgetEnvelope::new(1, [1; 4], [0; 4], 2).unwrap(),
        &[b],
    )
    .unwrap();
    let trace = scheduler.step();
    for resource in ResourceClass::ALL {
        assert_eq!(
            trace
                .decisions
                .iter()
                .filter(|d| d.kind == DecisionKind::Executed && d.resource == resource)
                .map(|d| d.units)
                .sum::<u32>(),
            1
        )
    }
}

#[test]
fn pressure_residency_churn_expires_without_canonical_mutation() {
    let mut ledger = ResidencyLedgerV1::new([([7; 32], 1)]).unwrap();
    let request =
        |d| ResidencyIntentV1::new([7; 32], 1, ConsumerDomainV1::Streaming, 2, d).unwrap();
    let before = ledger.state_fingerprint();
    ledger.apply(request(ResidencyDisposition::Bypass)).unwrap();
    assert_eq!(before, ledger.state_fingerprint());
    ledger
        .apply(request(ResidencyDisposition::Request))
        .unwrap();
    ledger.apply(request(ResidencyDisposition::Renew)).unwrap();
    assert!(
        ledger
            .apply(request(ResidencyDisposition::Renew))
            .unwrap()
            .iter()
            .any(|d| d.kind == ResidencyDecisionKind::ChurnDiagnosed)
    );
    ledger.advance(2);
    assert!(!ledger.contains([7; 32]))
}

#[test]
fn pressure_eight_domains_share_truth_but_keep_distinct_fidelity() {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let fingerprints: std::collections::BTreeSet<_> = ConsumerDomainV1::ALL
        .into_iter()
        .map(|domain| {
            (
                b.packet_fingerprint(),
                b.tier(),
                maps().fidelity(domain, b.tier()),
            )
        })
        .collect();
    assert_eq!(
        fingerprints
            .iter()
            .map(|v| v.0)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        1
    );
    assert!(
        fingerprints
            .iter()
            .map(|v| v.2)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            > 1
    )
}
