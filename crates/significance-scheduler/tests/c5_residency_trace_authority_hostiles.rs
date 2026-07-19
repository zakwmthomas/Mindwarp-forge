use significance_scheduler::*;

fn ticket(id: u8, domain: ConsumerDomainV1) -> WorkTicket {
    let packet = ImportancePacket::new(
        [7; 32],
        1,
        SignalVector {
            focus: 650,
            visibility: 0,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        0,
    )
    .unwrap();
    WorkTicket::new(
        [id; 32],
        [7; 32],
        1,
        domain.code(),
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        20,
        None,
        None,
        packet.fingerprint().unwrap(),
        ImportanceTier::Critical,
    )
    .unwrap()
}
fn budget() -> BudgetEnvelope {
    BudgetEnvelope::new(1, [0, 8, 0, 0], [0; 4], 3).unwrap()
}
fn intent(lease: u16, disposition: ResidencyDisposition) -> ResidencyIntentV1 {
    ResidencyIntentV1::new([7; 32], 1, ConsumerDomainV1::Streaming, lease, disposition).unwrap()
}
fn binding() -> ImportanceDecisionBindingV1 {
    let packet = ImportancePacket::new(
        [7; 32],
        1,
        SignalVector {
            focus: 650,
            visibility: 0,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        1,
        0,
    )
    .unwrap();
    let maps = DomainFidelityMapSetV1::new([[0, 1, 2, 3]; 8]).unwrap();
    ImportanceDecisionBindingV1::derive(
        &packet,
        HysteresisPolicy::new([100, 300, 600], [80, 250, 500], 2).unwrap(),
        SignificanceState::default(),
        1,
        &maps,
    )
    .unwrap()
}
fn trace() -> (
    Vec<WorkTicket>,
    Vec<ImportanceDecisionBindingV1>,
    PressureTraceV2,
) {
    let tickets = vec![
        ticket(1, ConsumerDomainV1::Generation),
        ticket(2, ConsumerDomainV1::Simulation),
    ];
    let bindings = vec![binding()];
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget(), &bindings).unwrap();
    let trace = scheduler.step_strict().unwrap();
    (tickets, bindings, trace)
}

#[test]
fn residency_zero_target() {
    assert!(
        ResidencyIntentV1::new(
            [0; 32],
            1,
            ConsumerDomainV1::Streaming,
            1,
            ResidencyDisposition::Request
        )
        .is_err()
    )
}
#[test]
fn residency_zero_lease() {
    assert!(
        ResidencyIntentV1::new(
            [7; 32],
            1,
            ConsumerDomainV1::Streaming,
            0,
            ResidencyDisposition::Request
        )
        .is_err()
    )
}
#[test]
fn residency_stale_epoch() {
    let mut ledger = ResidencyLedgerV1::new([([7; 32], 2)]).unwrap();
    assert_eq!(
        ledger
            .apply(intent(1, ResidencyDisposition::Request))
            .unwrap_err(),
        SignificanceSchedulerError::StaleEpoch
    )
}
#[test]
fn residency_unbounded_lease() {
    assert!(
        ResidencyIntentV1::new(
            [7; 32],
            1,
            ConsumerDomainV1::Streaming,
            257,
            ResidencyDisposition::Request
        )
        .is_err()
    )
}
#[test]
fn residency_expired_retained() {
    let mut ledger = ResidencyLedgerV1::new([([7; 32], 1)]).unwrap();
    ledger
        .apply(intent(2, ResidencyDisposition::Request))
        .unwrap();
    assert!(ledger.contains([7; 32]));
    assert_eq!(ledger.advance(2)[0].kind, ResidencyDecisionKind::Expired);
    assert!(!ledger.contains([7; 32]))
}
#[test]
fn residency_bypass_mutates() {
    let mut ledger = ResidencyLedgerV1::new([([7; 32], 1)]).unwrap();
    let before = ledger.state_fingerprint();
    assert_eq!(
        ledger
            .apply(intent(1, ResidencyDisposition::Bypass))
            .unwrap()[0]
            .kind,
        ResidencyDecisionKind::Bypassed
    );
    assert_eq!(before, ledger.state_fingerprint())
}
#[test]
fn residency_thrash_untraced() {
    let mut ledger = ResidencyLedgerV1::new([([7; 32], 1)]).unwrap();
    ledger
        .apply(intent(2, ResidencyDisposition::Request))
        .unwrap();
    ledger
        .apply(intent(2, ResidencyDisposition::Renew))
        .unwrap();
    let decisions = ledger
        .apply(intent(2, ResidencyDisposition::Renew))
        .unwrap();
    assert!(
        decisions
            .iter()
            .any(|d| d.kind == ResidencyDecisionKind::ChurnDiagnosed)
    );
    assert!(
        ledger
            .trace()
            .iter()
            .any(|d| d.kind == ResidencyDecisionKind::ChurnDiagnosed)
    )
}

#[test]
fn trace_unknown_decision_code() {
    let (_, _, trace) = trace();
    let mut bytes = trace.encode_canonical().unwrap();
    let mut d = minicbor::Decoder::new(&bytes);
    d.array().unwrap();
    d.u16().unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    d.bytes().unwrap();
    d.array().unwrap();
    d.array().unwrap();
    d.bytes().unwrap();
    d.u16().unwrap();
    d.u16().unwrap();
    d.bytes().unwrap();
    let offset = d.position();
    bytes[offset] = 99;
    assert!(PressureTraceV2::decode_strict(&bytes).is_err())
}
#[test]
fn trace_missing_domain() {
    let (_, _, trace) = trace();
    let mut hostile = trace.clone();
    hostile.decisions[0].domain_code = 0;
    assert!(PressureTraceV2::decode_strict(&hostile.encode_canonical().unwrap()).is_err())
}
#[test]
fn trace_missing_work_class() {
    let (_, _, trace) = trace();
    let mut hostile = trace.clone();
    hostile.decisions[0].work_class = 0;
    assert!(PressureTraceV2::decode_strict(&hostile.encode_canonical().unwrap()).is_err())
}
#[test]
fn trace_packet_mismatch() {
    let (tickets, bindings, trace) = trace();
    let mut hostile = trace.clone();
    hostile.decisions[0].packet_fingerprint = [8; 32];
    assert!(
        hostile
            .verify_replay(&tickets, budget(), &bindings)
            .is_err()
    )
}
#[test]
fn trace_budget_mismatch() {
    let (tickets, bindings, trace) = trace();
    let other = BudgetEnvelope::new(1, [0, 7, 0, 0], [0; 4], 3).unwrap();
    assert!(trace.verify_replay(&tickets, other, &bindings).is_err())
}
#[test]
fn trace_reordered_decision() {
    let (tickets, bindings, trace) = trace();
    let mut hostile = trace.clone();
    hostile.decisions.swap(0, 1);
    assert!(
        hostile
            .verify_replay(&tickets, budget(), &bindings)
            .is_err()
    )
}
#[test]
fn trace_trailing_bytes() {
    let (_, _, trace) = trace();
    let mut bytes = trace.encode_canonical().unwrap();
    bytes.push(0);
    assert!(PressureTraceV2::decode_strict(&bytes).is_err())
}
#[test]
fn trace_replay_drift() {
    let (mut tickets, bindings, trace) = trace();
    tickets[0].cost_units = 2;
    assert!(trace.verify_replay(&tickets, budget(), &bindings).is_err());
    trace
        .verify_replay(
            &[
                ticket(1, ConsumerDomainV1::Generation),
                ticket(2, ConsumerDomainV1::Simulation),
            ],
            budget(),
            &bindings,
        )
        .unwrap()
}

fn authority_negative() {
    assert!(reference_proof_evidence().unwrap().capabilities.is_empty())
}
#[test]
fn authority_runtime_controller() {
    authority_negative()
}
#[test]
fn authority_runtime_executor() {
    authority_negative()
}
#[test]
fn authority_cache_mutation() {
    authority_negative()
}
#[test]
fn authority_storage_mutation() {
    authority_negative()
}
#[test]
fn authority_product_weight() {
    authority_negative()
}
#[test]
fn authority_ai_generation() {
    authority_negative()
}
#[test]
fn authority_rendering_implementation() {
    authority_negative()
}
#[test]
fn authority_kernel_mutation() {
    authority_negative()
}
