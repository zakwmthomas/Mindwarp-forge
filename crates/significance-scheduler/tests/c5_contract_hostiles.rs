use significance_scheduler::*;

fn id(value: u16) -> [u8; 32] {
    let mut id = [0; 32];
    id[..2].copy_from_slice(&value.to_be_bytes());
    id
}

fn packet(epoch: u64, target: [u8; 32], protection_flags: u8) -> ImportancePacket {
    ImportancePacket::new(
        target,
        epoch,
        SignalVector {
            focus: 650,
            visibility: 650,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        protection_flags,
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

#[allow(clippy::too_many_arguments)]
fn ticket(
    ticket_id: u16,
    target: [u8; 32],
    epoch: u64,
    domain: ConsumerDomainV1,
    work_class: u16,
    resource: ResourceClass,
    cost: u32,
    dependencies: Vec<[u8; 32]>,
    fallback: Option<[u8; 32]>,
    cancellation_parent: Option<[u8; 32]>,
) -> WorkTicket {
    WorkTicket::new(
        id(ticket_id),
        target,
        epoch,
        domain.code(),
        work_class,
        resource,
        cost,
        dependencies,
        DeadlineClass::QualityTarget,
        20,
        fallback,
        cancellation_parent,
        packet(epoch, target, 0).fingerprint().unwrap(),
        ImportanceTier::Critical,
    )
    .unwrap()
}

fn budget(epoch: u64) -> BudgetEnvelope {
    BudgetEnvelope::new(epoch, [2, 2, 2, 2], [0; 4], 3).unwrap()
}

#[test]
fn hostile_domain_and_truth_contracts_fail_closed() {
    // domain.zero-code, domain.unknown-code
    assert!(ConsumerDomainV1::try_from(0).is_err());
    assert!(ConsumerDomainV1::try_from(9).is_err());

    // domain.map-nonmonotone
    let mut invalid_levels = [[0, 1, 2, 3]; 8];
    invalid_levels[4] = [0, 3, 2, 4];
    assert!(DomainFidelityMapSetV1::new(invalid_levels).is_err());
    // domain.missing-required, domain.duplicate-required, domain.swapped-map
    let encoded = maps().encode_canonical().unwrap();
    let mut missing = encoded.clone();
    missing[2] = 0x87;
    assert!(DomainFidelityMapSetV1::decode_strict(&missing).is_err());
    let mut duplicate = encoded.clone();
    duplicate[11] = 1;
    assert!(DomainFidelityMapSetV1::decode_strict(&duplicate).is_err());
    let mut swapped = encoded;
    swapped.swap(4, 11);
    assert!(DomainFidelityMapSetV1::decode_strict(&swapped).is_err());
    // domain.private-score: the typed projection exposes only the fixed maps.
    let projection = serde_json::to_string(&maps()).unwrap();
    assert!(!projection.contains("score"));

    let target = [7; 32];
    let protected_packet = packet(1, target, PROTECT_INTERACTION);
    let maps = maps();
    let binding = ImportanceDecisionBindingV1::derive(
        &protected_packet,
        policy(),
        SignificanceState::default(),
        1,
        &maps,
    )
    .unwrap();
    let valid = ticket(
        1,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        None,
        None,
    );

    // truth.packet-zero, truth.packet-mismatch, truth.packet-tier-forged
    let mut hostile = valid.clone();
    hostile.importance_packet = [0; 32];
    assert!(binding.verify_ticket(&hostile).is_err());
    hostile.importance_packet = [8; 32];
    assert!(binding.verify_ticket(&hostile).is_err());
    hostile = valid.clone();
    hostile.importance_tier = ImportanceTier::Dormant;
    assert!(binding.verify_ticket(&hostile).is_err());

    // truth.packet-epoch-mismatch, truth.packet-target-mismatch
    hostile = valid.clone();
    hostile.request_epoch = 2;
    assert!(binding.verify_ticket(&hostile).is_err());
    hostile = valid.clone();
    hostile.target_descriptor = [6; 32];
    assert!(binding.verify_ticket(&hostile).is_err());

    // truth.policy-mismatch, truth.domain-map-set-mismatch, truth.protection-erased
    assert!(
        binding
            .verify(
                &protected_packet,
                HysteresisPolicy::new([101, 300, 600], [80, 250, 500], 2).unwrap(),
                &maps,
            )
            .is_err()
    );
    assert!(
        binding
            .verify(
                &protected_packet,
                policy(),
                &DomainFidelityMapSetV1::new([[0, 0, 0, 0]; 8]).unwrap(),
            )
            .is_err()
    );
    assert!(
        binding
            .verify(&packet(1, target, 0), policy(), &maps)
            .is_err()
    );

    // truth.cross-domain-interference: one domain lookup cannot alter another.
    let audio_before = maps.fidelity(ConsumerDomainV1::Audio, ImportanceTier::Critical);
    let _ = maps.fidelity(ConsumerDomainV1::Generation, ImportanceTier::Dormant);
    assert_eq!(
        maps.fidelity(ConsumerDomainV1::Audio, ImportanceTier::Critical),
        audio_before
    );
}

#[test]
fn hostile_ticket_and_graph_contracts_fail_closed() {
    let target = [7; 32];
    let valid = ticket(
        1,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        None,
        None,
    );

    // ticket.zero-id, ticket.unknown-domain, ticket.unknown-work-class,
    // ticket.self-dependency, ticket.oversized-dependencies
    let mut args = valid.clone();
    args.id = [0; 32];
    assert!(WorkTicket::decode_strict(&args.encode_canonical().unwrap()).is_err());
    args = valid.clone();
    args.consumer = 9;
    assert!(WorkTicket::decode_strict(&args.encode_canonical().unwrap()).is_err());
    args = valid.clone();
    args.work_class = 9;
    assert!(WorkTicket::decode_strict(&args.encode_canonical().unwrap()).is_err());
    args = valid.clone();
    args.dependencies = vec![args.id];
    assert!(WorkTicket::decode_strict(&args.encode_canonical().unwrap()).is_err());
    args = valid.clone();
    args.dependencies = (2..=18).map(id).collect();
    assert!(WorkTicket::decode_strict(&args.encode_canonical().unwrap()).is_err());

    // ticket.duplicate-id, ticket.conflicting-id
    assert_eq!(
        ReferenceScheduler::new(vec![valid.clone(), valid.clone()], budget(1)).unwrap_err(),
        SignificanceSchedulerError::DuplicateTicket
    );
    let mut conflicting = valid.clone();
    conflicting.cost_units = 2;
    assert_eq!(
        ReferenceScheduler::new(vec![valid.clone(), conflicting], budget(1)).unwrap_err(),
        SignificanceSchedulerError::DuplicateTicket
    );

    // ticket.unknown-dependency
    let unknown = ticket(
        2,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![id(99)],
        None,
        None,
    );
    assert_eq!(
        ReferenceScheduler::new(vec![unknown], budget(1)).unwrap_err(),
        SignificanceSchedulerError::UnknownDependency
    );

    // ticket.dependency-cycle
    let left = ticket(
        1,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![id(2)],
        None,
        None,
    );
    let right = ticket(
        2,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![id(1)],
        None,
        None,
    );
    assert_eq!(
        ReferenceScheduler::new(vec![left, right], budget(1)).unwrap_err(),
        SignificanceSchedulerError::DependencyCycle
    );

    // ticket.cancellation-cycle
    let left = ticket(
        1,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        None,
        Some(id(2)),
    );
    let right = ticket(
        2,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        None,
        Some(id(1)),
    );
    assert_eq!(
        ReferenceScheduler::new(vec![left, right], budget(1)).unwrap_err(),
        SignificanceSchedulerError::InvalidCancellationTree
    );

    // ticket.oversized-graph
    let oversized: Vec<_> = (1..=257)
        .map(|n| {
            ticket(
                n,
                target,
                1,
                ConsumerDomainV1::Generation,
                1,
                ResourceClass::Cpu,
                1,
                vec![],
                None,
                None,
            )
        })
        .collect();
    assert!(ReferenceScheduler::new(oversized, budget(1)).is_err());
}

#[test]
fn hostile_fallback_contracts_fail_closed() {
    let target = [7; 32];
    let original = |fallback| {
        ticket(
            1,
            target,
            1,
            ConsumerDomainV1::Generation,
            1,
            ResourceClass::Cpu,
            2,
            vec![],
            fallback,
            None,
        )
    };
    let candidate = |target, epoch, domain, class, resource, cost, fallback| {
        ticket(
            2,
            target,
            epoch,
            domain,
            class,
            resource,
            cost,
            vec![],
            fallback,
            None,
        )
    };

    // fallback.missing
    assert_eq!(
        ReferenceScheduler::new(vec![original(Some(id(2)))], budget(1)).unwrap_err(),
        SignificanceSchedulerError::UnknownFallback
    );

    let assert_invalid = |fallback: WorkTicket| {
        assert_eq!(
            ReferenceScheduler::new(vec![original(Some(id(2))), fallback], budget(1)).unwrap_err(),
            SignificanceSchedulerError::InvalidFallback
        );
    };
    // fallback.same-cost, fallback.more-expensive, fallback.cross-target,
    // fallback.cross-domain, fallback.cross-work-class, fallback.cross-resource,
    // fallback.nested
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        2,
        None,
    ));
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        3,
        None,
    ));
    assert_invalid(candidate(
        [8; 32],
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        None,
    ));
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Simulation,
        1,
        ResourceClass::Cpu,
        1,
        None,
    ));
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Generation,
        2,
        ResourceClass::Cpu,
        1,
        None,
    ));
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Gpu,
        1,
        None,
    ));
    assert_invalid(candidate(
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        Some(id(3)),
    ));

    // fallback.cross-epoch is rejected at admission before fallback validation.
    let cross_epoch = candidate(
        target,
        2,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        None,
    );
    assert_eq!(
        ReferenceScheduler::new(vec![original(Some(id(2))), cross_epoch], budget(1)).unwrap_err(),
        SignificanceSchedulerError::AdmissionRejected
    );
}

#[test]
fn hostile_admission_and_budget_contracts_fail_closed_and_are_receipted() {
    // admission.zero-budget, admission.reserve-over-budget
    assert!(BudgetEnvelope::new(1, [0; 4], [0; 4], 3).is_err());
    assert!(BudgetEnvelope::new(1, [1, 0, 0, 0], [2, 0, 0, 0], 3).is_err());
    // admission.cost-overflow: even the maximum bounded graph cannot overflow u64 accumulation.
    assert!(256u64 * u64::from(u32::MAX) < u64::MAX);

    let target = [7; 32];
    let work = ticket(
        1,
        target,
        1,
        ConsumerDomainV1::Generation,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        None,
        None,
    );

    // admission.budget-epoch-mismatch
    assert_eq!(
        ReferenceScheduler::new(vec![work.clone()], budget(2)).unwrap_err(),
        SignificanceSchedulerError::AdmissionRejected
    );

    // admission.deadline-zero
    let mut zero_deadline = work.clone();
    zero_deadline.due_step = 0;
    assert!(WorkTicket::decode_strict(&zero_deadline.encode_canonical().unwrap()).is_err());

    // admission.impossible-safety
    let mut safety = work.clone();
    safety.deadline_class = DeadlineClass::InteractionSafety;
    safety.due_step = 1;
    safety.cost_units = 3;
    assert_eq!(
        ReferenceScheduler::new(vec![safety], budget(1)).unwrap_err(),
        SignificanceSchedulerError::AdmissionRejected
    );

    // admission.rejection-unreceipted: every evaluated rejection has a stable receipt.
    let rejected = AdmissionReceiptV1::evaluate(&[], budget(1));
    assert!(!rejected.accepted);
    assert_ne!(rejected.reason_code, 0);
    assert_ne!(rejected.graph_fingerprint, [0; 32]);
    assert_eq!(
        rejected.budget_fingerprint,
        budget(1).fingerprint().unwrap()
    );
    let receipt_bytes = rejected.encode_canonical().unwrap();
    assert_eq!(
        AdmissionReceiptV1::decode_strict(&receipt_bytes).unwrap(),
        rejected
    );
    rejected.verify(&[], budget(1)).unwrap();
    let mut forged = rejected.clone();
    forged.budget_fingerprint = [3; 32];
    assert!(forged.verify(&[], budget(1)).is_err());

    // budget.noncanonical, budget.fingerprint-mismatch
    let encoded = budget(1).encode_canonical().unwrap();
    let mut trailing = encoded.clone();
    trailing.push(0);
    assert!(BudgetEnvelope::decode_strict(&trailing).is_err());
    assert_ne!(
        budget(1).fingerprint().unwrap(),
        budget(2).fingerprint().unwrap()
    );
    assert_eq!(BudgetEnvelope::decode_strict(&encoded).unwrap(), budget(1));
}
