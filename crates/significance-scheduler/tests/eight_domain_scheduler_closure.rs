use significance_scheduler::*;

fn packet(epoch: u64, peak: u16) -> ImportancePacket {
    ImportancePacket::new(
        [7; 32],
        epoch,
        SignalVector {
            focus: peak,
            visibility: peak,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        0,
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
    id: u8,
    domain: ConsumerDomainV1,
    resource: ResourceClass,
    cost: u32,
    fallback: Option<u8>,
) -> WorkTicket {
    WorkTicket::new(
        [id; 32],
        [7; 32],
        1,
        domain.code(),
        1,
        resource,
        cost,
        vec![],
        DeadlineClass::QualityTarget,
        20,
        fallback.map(|value| [value; 32]),
        None,
        packet(1, 650).fingerprint().unwrap(),
        ImportanceTier::Critical,
    )
    .unwrap()
}

#[test]
fn eight_domains_are_closed_stable_and_share_one_derived_truth() {
    assert_eq!(ConsumerDomainV1::ALL.len(), 8);
    assert!(ConsumerDomainV1::try_from(0).is_err());
    assert!(ConsumerDomainV1::try_from(9).is_err());
    let maps = maps();
    let binding = ImportanceDecisionBindingV1::derive(
        &packet(1, 650),
        policy(),
        SignificanceState::default(),
        1,
        &maps,
    )
    .unwrap();
    assert_eq!(binding.tier, ImportanceTier::Critical);
    let mut distinct = std::collections::BTreeSet::new();
    for domain in ConsumerDomainV1::ALL {
        distinct.insert(maps.fidelity(domain, binding.tier));
        binding
            .verify_ticket(&ticket(
                domain.code() as u8,
                domain,
                ResourceClass::Cpu,
                1,
                None,
            ))
            .unwrap();
    }
    assert!(distinct.len() > 1);
    let mut forged = ticket(1, ConsumerDomainV1::Generation, ResourceClass::Cpu, 1, None);
    forged.importance_tier = ImportanceTier::Dormant;
    assert!(binding.verify_ticket(&forged).is_err());
    assert!(
        binding
            .verify(
                &packet(1, 650),
                HysteresisPolicy::new([101, 300, 600], [80, 250, 500], 2).unwrap(),
                &maps
            )
            .is_err()
    );
    assert!(
        ImportancePacket::new(
            [0; 32],
            1,
            SignalVector {
                focus: 1,
                visibility: 0,
                interaction: 0,
                threat: 0,
                prediction: 0
            },
            1,
            0
        )
        .is_err()
    );
}

#[test]
fn domain_map_packet_policy_and_budget_bytes_are_strict_and_bound() {
    let maps = maps();
    let map_bytes = maps.encode_canonical().unwrap();
    assert_eq!(
        DomainFidelityMapSetV1::decode_strict(&map_bytes).unwrap(),
        maps
    );
    let mut trailing = map_bytes;
    trailing.push(0);
    assert!(DomainFidelityMapSetV1::decode_strict(&trailing).is_err());
    assert_ne!(
        maps.fingerprint().unwrap(),
        DomainFidelityMapSetV1::new([[0, 0, 0, 0]; 8])
            .unwrap()
            .fingerprint()
            .unwrap()
    );
    assert_ne!(
        policy().fingerprint().unwrap(),
        HysteresisPolicy::new([101, 300, 600], [80, 250, 500], 2)
            .unwrap()
            .fingerprint()
            .unwrap()
    );
    let budget = BudgetEnvelope::new(1, [1, 2, 3, 4], [0, 1, 0, 0], 3).unwrap();
    let bytes = budget.encode_canonical().unwrap();
    assert_eq!(BudgetEnvelope::decode_strict(&bytes).unwrap(), budget);
    let mut corrupt = bytes;
    corrupt.push(0);
    assert!(BudgetEnvelope::decode_strict(&corrupt).is_err());
}

#[test]
fn admission_receipts_are_deterministic_and_bind_the_budget() {
    let tickets = vec![ticket(
        1,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        1,
        None,
    )];
    let budget = BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 3).unwrap();
    let accepted = AdmissionReceiptV1::evaluate(&tickets, budget);
    assert!(accepted.accepted);
    assert_eq!(accepted.budget_fingerprint, budget.fingerprint().unwrap());
    assert_eq!(accepted, AdmissionReceiptV1::evaluate(&tickets, budget));
    let rejected = AdmissionReceiptV1::evaluate(&[], budget);
    assert!(!rejected.accepted);
    assert_ne!(accepted.reason_code, rejected.reason_code);
    assert_eq!(
        ReferenceScheduler::new(
            tickets.clone(),
            BudgetEnvelope::new(2, [0, 1, 0, 0], [0; 4], 3).unwrap()
        )
        .unwrap_err(),
        SignificanceSchedulerError::AdmissionRejected
    );
    let binding = ImportanceDecisionBindingV1::derive(
        &packet(1, 650),
        policy(),
        SignificanceState::default(),
        1,
        &maps(),
    )
    .unwrap();
    assert!(AdmissionReceiptV1::evaluate_verified(&tickets, budget, &[binding]).accepted);
}

#[test]
fn fallback_must_preserve_domain_and_work_class() {
    let original = ticket(
        1,
        ConsumerDomainV1::Generation,
        ResourceClass::Cpu,
        2,
        Some(2),
    );
    let wrong_domain = ticket(2, ConsumerDomainV1::Simulation, ResourceClass::Cpu, 1, None);
    let budget = BudgetEnvelope::new(1, [0, 2, 0, 0], [0; 4], 3).unwrap();
    assert_eq!(
        ReferenceScheduler::new(vec![original.clone(), wrong_domain], budget).unwrap_err(),
        SignificanceSchedulerError::InvalidFallback
    );
    let mut wrong_class = ticket(2, ConsumerDomainV1::Generation, ResourceClass::Cpu, 1, None);
    wrong_class.work_class = 2;
    assert_eq!(
        ReferenceScheduler::new(vec![original, wrong_class], budget).unwrap_err(),
        SignificanceSchedulerError::InvalidFallback
    );
}

#[test]
fn completion_is_accepted_only_from_running_and_never_from_inactive_or_terminal() {
    let original = ticket(
        1,
        ConsumerDomainV1::Streaming,
        ResourceClass::Io,
        2,
        Some(2),
    );
    let fallback = ticket(2, ConsumerDomainV1::Streaming, ResourceClass::Io, 1, None);
    let budget = BudgetEnvelope::new(1, [0, 0, 0, 1], [0; 4], 3).unwrap();
    let mut scheduler = ReferenceScheduler::new(vec![original, fallback], budget).unwrap();
    assert_eq!(
        scheduler
            .record_external_completion([1; 32], 1)
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(
        scheduler
            .record_external_completion([2; 32], 1)
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    scheduler.step();
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Running);
    assert_eq!(
        scheduler
            .record_completion(CompletionReceiptV1::new([1; 32], 1, 1, [9; 32]).unwrap())
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    assert_eq!(scheduler.state([1; 32]).unwrap(), TicketState::Running);
    assert_eq!(
        scheduler
            .record_external_completion([1; 32], 1)
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
    scheduler
        .record_completion(CompletionReceiptV1::new([1; 32], 1, 2, [9; 32]).unwrap())
        .unwrap();
    assert_eq!(
        scheduler.state([1; 32]).unwrap(),
        TicketState::CompletedAccepted
    );
    assert_eq!(
        scheduler
            .record_external_completion([1; 32], 1)
            .unwrap_err(),
        SignificanceSchedulerError::InvalidTransition
    );
}

#[test]
fn strict_trace_has_domain_budget_and_stable_code_identity() {
    let tickets: Vec<WorkTicket> = ConsumerDomainV1::ALL
        .into_iter()
        .enumerate()
        .map(|(index, domain)| {
            ticket(
                (index + 1) as u8,
                domain,
                ResourceClass::ALL[index % 4],
                1,
                None,
            )
        })
        .collect();
    let budget = BudgetEnvelope::new(1, [2, 2, 2, 2], [0; 4], 3).unwrap();
    let mut left = ReferenceScheduler::new(tickets.clone(), budget).unwrap();
    let mut right = ReferenceScheduler::new(tickets, budget).unwrap();
    let a = left.step_strict().unwrap();
    let b = right.step_strict().unwrap();
    assert_eq!(a, b);
    assert_eq!(a.budget_fingerprint, budget.fingerprint().unwrap());
    assert!(
        a.decisions
            .iter()
            .all(|decision| ConsumerDomainV1::try_from(decision.domain_code).is_ok())
    );
    let bytes = a.encode_canonical().unwrap();
    assert_eq!(PressureTraceV2::decode_strict(&bytes).unwrap(), a);
}

#[test]
fn starvation_is_diagnosed_without_forging_significance() {
    let work = ticket(1, ConsumerDomainV1::Simulation, ResourceClass::Cpu, 2, None);
    let budget = BudgetEnvelope::new(1, [1, 0, 0, 0], [0; 4], 2).unwrap();
    let mut scheduler = ReferenceScheduler::new(vec![work], budget).unwrap();
    assert!(
        !scheduler
            .step()
            .decisions
            .iter()
            .any(|d| d.kind == DecisionKind::StarvationDiagnosed)
    );
    assert!(
        scheduler
            .step()
            .decisions
            .iter()
            .any(|d| d.kind == DecisionKind::StarvationDiagnosed)
    );
    assert_eq!(
        scheduler.effective_priority([1; 32]).unwrap().1,
        ImportanceTier::Critical
    );
}

#[test]
fn residency_intents_are_streaming_only_bounded_and_strict() {
    let intent = ResidencyIntentV1::new(
        [7; 32],
        1,
        ConsumerDomainV1::Streaming,
        8,
        ResidencyDisposition::Request,
    )
    .unwrap();
    let bytes = intent.encode_canonical().unwrap();
    assert_eq!(ResidencyIntentV1::decode_strict(&bytes).unwrap(), intent);
    assert!(
        ResidencyIntentV1::new(
            [7; 32],
            1,
            ConsumerDomainV1::Rendering,
            8,
            ResidencyDisposition::Request
        )
        .is_err()
    );
    assert!(
        ResidencyIntentV1::new(
            [7; 32],
            1,
            ConsumerDomainV1::Streaming,
            0,
            ResidencyDisposition::Request
        )
        .is_err()
    );
    assert!(
        ResidencyIntentV1::new(
            [7; 32],
            1,
            ConsumerDomainV1::Streaming,
            257,
            ResidencyDisposition::Request
        )
        .is_err()
    );
}

#[test]
fn frozen_hostile_registry_is_complete_and_unique() {
    assert_eq!(C5_HOSTILE_IDS.len(), 92);
    let unique: std::collections::BTreeSet<_> = C5_HOSTILE_IDS.iter().collect();
    assert_eq!(unique.len(), C5_HOSTILE_IDS.len());
}
