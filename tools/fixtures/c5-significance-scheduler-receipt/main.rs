use minicbor::{Decoder, Encoder};
use sha2::{Digest, Sha256};
use significance_scheduler::*;
use std::{
    env, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const RECEIPT_DOMAIN: &[u8] = b"mindwarp/c5-semantic-receipt/v1\0";
const HOSTILE_DOMAIN: &[u8] = b"mindwarp/c5-hostile-registry/v1\0";
const PRESSURE_DOMAIN: &[u8] = b"mindwarp/c5-pressure-suite/v1\0";
const C4_SHA: [u8; 32] = hex32("263a7c274c5bbfb5a48f0a7ccf3462eb35ddc7c96c1c92ff01d8ef37a40f6996");

const fn hex_nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("invalid hex"),
    }
}
const fn hex32(value: &str) -> [u8; 32] {
    let bytes = value.as_bytes();
    let mut out = [0; 32];
    let mut i = 0;
    while i < 32 {
        out[i] = (hex_nibble(bytes[i * 2]) << 4) | hex_nibble(bytes[i * 2 + 1]);
        i += 1;
    }
    out
}
fn sha(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
fn domain_sha(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(bytes);
    hash.finalize().into()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn field<F>(write: F) -> Vec<u8>
where
    F: FnOnce(&mut Encoder<Vec<u8>>),
{
    let mut encoder = Encoder::new(Vec::new());
    write(&mut encoder);
    encoder.into_writer()
}
fn f_u8(value: u8) -> Vec<u8> {
    field(|e| {
        e.u8(value).unwrap();
    })
}
fn f_u16(value: u16) -> Vec<u8> {
    field(|e| {
        e.u16(value).unwrap();
    })
}
fn f_text(value: &str) -> Vec<u8> {
    field(|e| {
        e.str(value).unwrap();
    })
}
fn f_bytes(value: &[u8]) -> Vec<u8> {
    field(|e| {
        e.bytes(value).unwrap();
    })
}
fn f_bool(value: bool) -> Vec<u8> {
    field(|e| {
        e.bool(value).unwrap();
    })
}

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
    prior: SignificanceState,
    step: u64,
) -> ImportanceDecisionBindingV1 {
    ImportanceDecisionBindingV1::derive(&packet(signal, protection), policy(), prior, step, &maps())
        .unwrap()
}
#[allow(clippy::too_many_arguments)]
fn ticket(
    id: u8,
    domain: ConsumerDomainV1,
    resource: ResourceClass,
    cost: u32,
    class: DeadlineClass,
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
        dependencies.into_iter().map(|v| [v; 32]).collect(),
        class,
        20,
        fallback.map(|v| [v; 32]),
        None,
        binding.packet_fingerprint(),
        binding.tier(),
    )
    .unwrap()
}
fn base_fixture() -> (
    ImportancePacket,
    HysteresisPolicy,
    DomainFidelityMapSetV1,
    ImportanceDecisionBindingV1,
    Vec<WorkTicket>,
    BudgetEnvelope,
) {
    let packet = packet(650, 0);
    let policy = policy();
    let maps = maps();
    let binding = ImportanceDecisionBindingV1::derive(
        &packet,
        policy,
        SignificanceState::default(),
        1,
        &maps,
    )
    .unwrap();
    let tickets = ConsumerDomainV1::ALL
        .into_iter()
        .enumerate()
        .map(|(index, domain)| {
            ticket(
                (index + 1) as u8,
                domain,
                ResourceClass::ALL[index % 4],
                1,
                DeadlineClass::QualityTarget,
                None,
                vec![],
                &binding,
            )
        })
        .collect();
    let budget = BudgetEnvelope::new(1, [2; 4], [0; 4], 3).unwrap();
    (packet, policy, maps, binding, tickets, budget)
}

fn decision_bytes(decisions: &[SchedulerDecision]) -> Vec<u8> {
    field(|e| {
        e.array(decisions.len() as u64).unwrap();
        for d in decisions {
            e.array(4)
                .unwrap()
                .bytes(&d.ticket_id)
                .unwrap()
                .u8(d.kind.stable_code())
                .unwrap()
                .u8(d.resource as u8)
                .unwrap()
                .u32(d.units)
                .unwrap();
        }
    })
}
fn residency_bytes(decisions: &[ResidencyDecisionV1]) -> Vec<u8> {
    field(|e| {
        e.array(decisions.len() as u64).unwrap();
        for d in decisions {
            e.array(4)
                .unwrap()
                .bytes(&d.target_descriptor)
                .unwrap()
                .u64(d.request_epoch)
                .unwrap()
                .u8(d.kind as u8)
                .unwrap()
                .u64(d.step)
                .unwrap();
        }
    })
}
fn transcript(records: &[(u8, Vec<u8>)]) -> Vec<u8> {
    field(|e| {
        e.array(records.len() as u64).unwrap();
        for (code, payload) in records {
            e.array(2)
                .unwrap()
                .u8(*code)
                .unwrap()
                .bytes(payload)
                .unwrap();
        }
    })
}
fn admission_record(
    tickets: &[WorkTicket],
    budget: BudgetEnvelope,
    bindings: &[ImportanceDecisionBindingV1],
) -> (u8, Vec<u8>) {
    let receipt = AdmissionReceiptV1::evaluate_verified(tickets, budget, bindings);
    receipt.verify_verified(tickets, budget, bindings).unwrap();
    (1, receipt.encode_canonical().unwrap())
}
fn strict_record(scheduler: &mut ReferenceScheduler) -> (u8, Vec<u8>) {
    (
        2,
        scheduler.step_strict().unwrap().encode_canonical().unwrap(),
    )
}

fn pressure_rows() -> Vec<(u8, u16, [u8; 32])> {
    let b = binding(650, 0, SignificanceState::default(), 1);
    let mut rows = Vec::new();

    let (_, _, _, base_binding, base_tickets, base_budget) = base_fixture();
    let mut scheduler = ReferenceScheduler::new_verified(
        base_tickets.clone(),
        base_budget,
        std::slice::from_ref(&base_binding),
    )
    .unwrap();
    let records = vec![
        admission_record(
            &base_tickets,
            base_budget,
            std::slice::from_ref(&base_binding),
        ),
        strict_record(&mut scheduler),
    ];
    rows.push((1, records.len() as u16, sha(&transcript(&records))));

    let second = binding(490, 0, b.resulting_state(), 2);
    let third = binding(510, 0, second.resulting_state(), 3);
    let mut records = Vec::new();
    for current in [b.clone(), second, third] {
        let work = ticket(
            1,
            ConsumerDomainV1::Ai,
            ResourceClass::Cpu,
            1,
            DeadlineClass::QualityTarget,
            None,
            vec![],
            &current,
        );
        let budget = BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap();
        records.push(admission_record(
            std::slice::from_ref(&work),
            budget,
            std::slice::from_ref(&current),
        ));
        let mut scheduler =
            ReferenceScheduler::new_verified(vec![work], budget, &[current]).unwrap();
        records.push(strict_record(&mut scheduler));
    }
    rows.push((2, records.len() as u16, sha(&transcript(&records))));

    let protected = binding(0, PROTECT_INTERACTION, SignificanceState::default(), 1);
    let tickets = vec![
        ticket(
            1,
            ConsumerDomainV1::Physics,
            ResourceClass::Cpu,
            1,
            DeadlineClass::InteractionSafety,
            None,
            vec![],
            &protected,
        ),
        ticket(
            2,
            ConsumerDomainV1::Generation,
            ResourceClass::Cpu,
            1,
            DeadlineClass::QualityTarget,
            None,
            vec![],
            &protected,
        ),
    ];
    let budget = BudgetEnvelope::new(1, [0, 2, 0, 0], [0, 1, 0, 0], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&protected))
            .unwrap();
    let records = vec![
        admission_record(&tickets, budget, std::slice::from_ref(&protected)),
        strict_record(&mut scheduler),
    ];
    rows.push((3, records.len() as u16, sha(&transcript(&records))));

    let tickets: Vec<_> = (1..=4)
        .map(|id| {
            ticket(
                id,
                ConsumerDomainV1::Simulation,
                ResourceClass::Cpu,
                2,
                DeadlineClass::QualityTarget,
                None,
                vec![],
                &b,
            )
        })
        .collect();
    let budget = BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
            .unwrap();
    let mut records = vec![admission_record(&tickets, budget, std::slice::from_ref(&b))];
    for _ in 0..8 {
        records.push(strict_record(&mut scheduler));
    }
    rows.push((4, records.len() as u16, sha(&transcript(&records))));

    let tickets = vec![
        ticket(
            1,
            ConsumerDomainV1::Streaming,
            ResourceClass::Io,
            2,
            DeadlineClass::QualityTarget,
            Some(2),
            vec![],
            &b,
        ),
        ticket(
            2,
            ConsumerDomainV1::Streaming,
            ResourceClass::Io,
            1,
            DeadlineClass::QualityTarget,
            None,
            vec![],
            &b,
        ),
    ];
    let budget = BudgetEnvelope::new(1, [0, 0, 0, 1], [0; 4], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
            .unwrap();
    let mut records = vec![admission_record(&tickets, budget, std::slice::from_ref(&b))];
    records.push((
        6,
        decision_bytes(&scheduler.advance_target_epoch([7; 32], 2).unwrap()),
    ));
    records.push(strict_record(&mut scheduler));
    rows.push((5, records.len() as u16, sha(&transcript(&records))));

    let work = ticket(
        1,
        ConsumerDomainV1::Animation,
        ResourceClass::Cpu,
        2,
        DeadlineClass::QualityTarget,
        None,
        vec![],
        &b,
    );
    let budget = BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(vec![work.clone()], budget, std::slice::from_ref(&b))
            .unwrap();
    let mut records = vec![
        admission_record(
            std::slice::from_ref(&work),
            budget,
            std::slice::from_ref(&b),
        ),
        strict_record(&mut scheduler),
    ];
    records.push((
        3,
        decision_bytes(&scheduler.request_cancel([1; 32], 1).unwrap()),
    ));
    records.push((
        4,
        decision_bytes(std::slice::from_ref(
            &scheduler.acknowledge_cancel([1; 32]).unwrap(),
        )),
    ));
    records.push((
        5,
        decision_bytes(&scheduler.settle_cancel([1; 32]).unwrap()),
    ));
    let late = scheduler
        .record_completion(CompletionReceiptV1::new([1; 32], 1, 2, [9; 32]).unwrap())
        .unwrap();
    records.push((7, decision_bytes(&[late])));
    rows.push((6, records.len() as u16, sha(&transcript(&records))));

    let tickets = vec![
        ticket(
            1,
            ConsumerDomainV1::Audio,
            ResourceClass::Cpu,
            1,
            DeadlineClass::QualityTarget,
            None,
            vec![],
            &b,
        ),
        ticket(
            2,
            ConsumerDomainV1::Audio,
            ResourceClass::Cpu,
            2,
            DeadlineClass::QualityTarget,
            Some(3),
            vec![1],
            &b,
        ),
        ticket(
            3,
            ConsumerDomainV1::Audio,
            ResourceClass::Cpu,
            1,
            DeadlineClass::QualityTarget,
            None,
            vec![],
            &b,
        ),
    ];
    let budget = BudgetEnvelope::new(1, [0, 1, 0, 0], [0; 4], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
            .unwrap();
    let mut records = vec![admission_record(&tickets, budget, std::slice::from_ref(&b))];
    records.push((
        3,
        decision_bytes(&scheduler.request_cancel([1; 32], 1).unwrap()),
    ));
    records.push((
        4,
        decision_bytes(std::slice::from_ref(
            &scheduler.acknowledge_cancel([1; 32]).unwrap(),
        )),
    ));
    records.push((
        5,
        decision_bytes(&scheduler.settle_cancel([1; 32]).unwrap()),
    ));
    records.push(strict_record(&mut scheduler));
    rows.push((7, records.len() as u16, sha(&transcript(&records))));

    let tickets: Vec<_> = ResourceClass::ALL
        .into_iter()
        .enumerate()
        .map(|(i, resource)| {
            ticket(
                (i + 1) as u8,
                ConsumerDomainV1::ALL[i],
                resource,
                2,
                DeadlineClass::QualityTarget,
                None,
                vec![],
                &b,
            )
        })
        .collect();
    let budget = BudgetEnvelope::new(1, [1; 4], [0; 4], 2).unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&b))
            .unwrap();
    let records = vec![
        admission_record(&tickets, budget, std::slice::from_ref(&b)),
        strict_record(&mut scheduler),
    ];
    rows.push((8, records.len() as u16, sha(&transcript(&records))));

    let mut ledger = ResidencyLedgerV1::new([([7; 32], 1)]).unwrap();
    let make = |disposition| {
        ResidencyIntentV1::new([7; 32], 1, ConsumerDomainV1::Streaming, 2, disposition).unwrap()
    };
    let mut records = Vec::new();
    for disposition in [
        ResidencyDisposition::Bypass,
        ResidencyDisposition::Request,
        ResidencyDisposition::Renew,
        ResidencyDisposition::Renew,
    ] {
        let intent = make(disposition);
        let mut payload = intent.encode_canonical().unwrap();
        payload.extend_from_slice(&residency_bytes(&ledger.apply(intent).unwrap()));
        records.push((8, payload));
    }
    records.push((9, residency_bytes(&ledger.advance(2))));
    rows.push((9, records.len() as u16, sha(&transcript(&records))));

    let mut payload = field(|e| {
        e.array(8).unwrap();
        for domain in ConsumerDomainV1::ALL {
            e.array(3)
                .unwrap()
                .u16(domain.code())
                .unwrap()
                .bytes(&b.packet_fingerprint())
                .unwrap()
                .u8(maps().fidelity(domain, b.tier()))
                .unwrap();
        }
    });
    let receipt = AdmissionReceiptV1::evaluate_verified(
        &base_tickets,
        base_budget,
        std::slice::from_ref(&base_binding),
    );
    payload.extend_from_slice(&receipt.encode_canonical().unwrap());
    let records = vec![(1, payload)];
    rows.push((10, records.len() as u16, sha(&transcript(&records))));
    rows
}

fn expected_fields() -> Vec<Vec<u8>> {
    let (packet, policy, maps, binding, tickets, budget) = base_fixture();
    let packet_bytes = packet.encode_canonical().unwrap();
    let policy_bytes = policy.encode_canonical().unwrap();
    let map_bytes = maps.encode_canonical().unwrap();
    let admission =
        AdmissionReceiptV1::evaluate_verified(&tickets, budget, std::slice::from_ref(&binding));
    admission
        .verify_verified(&tickets, budget, std::slice::from_ref(&binding))
        .unwrap();
    let admission_bytes = admission.encode_canonical().unwrap();
    assert_eq!(
        AdmissionReceiptV1::decode_strict(&admission_bytes).unwrap(),
        admission
    );
    let budget_bytes = budget.encode_canonical().unwrap();
    let mut scheduler =
        ReferenceScheduler::new_verified(tickets.clone(), budget, std::slice::from_ref(&binding))
            .unwrap();
    let trace = scheduler.step_strict().unwrap();
    let trace_bytes = trace.encode_canonical().unwrap();
    assert_eq!(PressureTraceV2::decode_strict(&trace_bytes).unwrap(), trace);
    trace
        .verify_replay(&tickets, budget, std::slice::from_ref(&binding))
        .unwrap();
    let pressure = pressure_rows();
    assert_eq!(pressure.len(), 10);
    let pressure_field = field(|e| {
        e.array(10).unwrap();
        for (code, count, digest) in &pressure {
            e.array(3)
                .unwrap()
                .u8(*code)
                .unwrap()
                .u16(*count)
                .unwrap()
                .bytes(digest)
                .unwrap();
        }
    });
    let hostile_digest = domain_sha(HOSTILE_DOMAIN, C5_HOSTILE_IDS.join("\n").as_bytes());
    let domain_field = field(|e| {
        e.array(8).unwrap();
        for (index, domain) in ConsumerDomainV1::ALL.into_iter().enumerate() {
            e.array(5)
                .unwrap()
                .u16(domain.code())
                .unwrap()
                .u8(maps.fidelity(domain, binding.tier()))
                .unwrap()
                .u8(ResourceClass::ALL[index % 4] as u8)
                .unwrap()
                .u16(1)
                .unwrap()
                .bytes(&tickets[index].fingerprint().unwrap())
                .unwrap();
        }
    });
    let pressure_digest = domain_sha(PRESSURE_DOMAIN, &pressure_field);
    let mut fields = vec![
        f_u16(1),
        f_text("g1-c5-significance-scheduler-semantic-receipt-v1"),
        f_text("mindwarp/significance-scheduler/c5/v1"),
        f_text("g1-c5-eight-domain-pressure-v1"),
        f_bytes(&C4_SHA),
        f_bytes(&sha(&packet_bytes)),
        f_bytes(&packet.fingerprint().unwrap()),
        f_bytes(&sha(&policy_bytes)),
        f_bytes(&policy.fingerprint().unwrap()),
        f_bytes(&sha(&map_bytes)),
        f_bytes(&maps.fingerprint().unwrap()),
        f_bytes(&binding.fingerprint().unwrap()),
        f_u8(binding.tier() as u8),
        domain_field,
        f_bytes(&sha(&budget_bytes)),
        f_bytes(&budget.fingerprint().unwrap()),
        f_bytes(&sha(&admission_bytes)),
        f_bytes(&admission.fingerprint()),
        f_bytes(&admission.graph_fingerprint),
        f_bytes(&sha(&trace_bytes)),
        f_bytes(&trace.fingerprint),
        f_u16(trace.decisions.len().try_into().unwrap()),
        f_u16(trace.remaining_tickets.try_into().unwrap()),
        pressure_field,
        f_bytes(&pressure_digest),
        f_bytes(&hostile_digest),
        f_u16(92),
    ];
    fields.extend((0..10).map(|_| f_bool(false)));
    assert_eq!(fields.len(), 37);
    fields
}

fn assemble(fields: &[Vec<u8>], hash_receipt: bool) -> Vec<u8> {
    let mut hash = Sha256::new();
    hash.update(RECEIPT_DOMAIN);
    for value in fields {
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    let digest: [u8; 32] = hash.finalize().into();
    let mut out = field(|e| {
        e.array((fields.len() + 1) as u64).unwrap();
    });
    for value in fields {
        out.extend_from_slice(value);
    }
    out.extend_from_slice(&f_bytes(if hash_receipt { &digest } else { &[0; 32] }));
    out
}
fn expected() -> Vec<u8> {
    assemble(&expected_fields(), true)
}

fn arr(decoder: &mut Decoder<'_>, count: u64) -> Result<(), ()> {
    (decoder.array().map_err(|_| ())? == Some(count))
        .then_some(())
        .ok_or(())
}
fn b32(decoder: &mut Decoder<'_>) -> Result<(), ()> {
    (decoder.bytes().map_err(|_| ())?.len() == 32)
        .then_some(())
        .ok_or(())
}
fn parse_field(index: usize, decoder: &mut Decoder<'_>) -> Result<(), ()> {
    match index {
        0 | 21 | 22 | 26 => {
            decoder.u16().map_err(|_| ())?;
        }
        1..=3 => {
            if decoder.str().map_err(|_| ())?.len() > 64 {
                return Err(());
            }
        }
        4..=11 | 14..=20 | 24..=25 => b32(decoder)?,
        12 => {
            decoder.u8().map_err(|_| ())?;
        }
        13 => {
            arr(decoder, 8)?;
            for expected_domain in 1..=8 {
                arr(decoder, 5)?;
                if decoder.u16().map_err(|_| ())? != expected_domain {
                    return Err(());
                }
                decoder.u8().map_err(|_| ())?;
                decoder.u8().map_err(|_| ())?;
                decoder.u16().map_err(|_| ())?;
                b32(decoder)?;
            }
        }
        23 => {
            arr(decoder, 10)?;
            for expected_scenario in 1..=10 {
                arr(decoder, 3)?;
                if decoder.u8().map_err(|_| ())? != expected_scenario {
                    return Err(());
                }
                decoder.u16().map_err(|_| ())?;
                b32(decoder)?;
            }
        }
        27..=36 => {
            decoder.bool().map_err(|_| ())?;
        }
        _ => return Err(()),
    }
    Ok(())
}
fn validate(bytes: &[u8]) -> Result<(), ()> {
    if bytes.len() > 65_536 {
        return Err(());
    }
    let expected_fields = expected_fields();
    let mut decoder = Decoder::new(bytes);
    arr(&mut decoder, 38)?;
    let mut received = Vec::new();
    for (index, expected_field) in expected_fields.iter().enumerate() {
        let start = decoder.position();
        parse_field(index, &mut decoder)?;
        let actual = bytes[start..decoder.position()].to_vec();
        if &actual != expected_field {
            return Err(());
        }
        received.push(actual);
    }
    b32(&mut decoder)?;
    if decoder.position() != bytes.len() || assemble(&received, true) != bytes {
        return Err(());
    }
    Ok(())
}

fn self_test() {
    let receipt = expected();
    validate(&receipt).unwrap();
    let fields = expected_fields();
    let mut cases: Vec<(&str, Vec<u8>)> = Vec::new();
    let mut unknown = receipt.clone();
    unknown[1] = 39;
    unknown.push(0xf6);
    cases.push(("receipt.unknown-field", unknown));
    cases.push(("receipt.missing-field", assemble(&fields[..36], true)));
    let mut changed = fields.clone();
    changed.swap(5, 6);
    cases.push(("receipt.field-reorder", assemble(&changed, true)));
    let mut changed = fields.clone();
    changed[0] = f_text("1");
    cases.push(("receipt.type-coercion", assemble(&changed, true)));
    let mut changed = fields.clone();
    let last = changed[11].len() - 1;
    changed[11][last] ^= 1;
    cases.push(("receipt.proof-drift", assemble(&changed, true)));
    let mut changed = fields.clone();
    let last = changed[23].len() - 1;
    changed[23][last] ^= 1;
    cases.push(("receipt.transcript-drift", assemble(&changed, true)));
    let mut changed = fields.clone();
    changed[27] = f_bool(true);
    cases.push(("receipt.authority-flip", assemble(&changed, true)));
    let mut changed = receipt.clone();
    let last = changed.len() - 1;
    changed[last] ^= 1;
    cases.push(("receipt.hash-drift", changed));
    for (id, hostile) in cases {
        assert!(validate(&hostile).is_err(), "{id}");
    }
    for index in 0..fields.len() {
        let mut changed = fields.clone();
        let last = changed[index].len() - 1;
        changed[index][last] ^= 1;
        assert!(
            validate(&assemble(&changed, true)).is_err(),
            "top-level field {index} admitted"
        );
    }
    println!(
        "C5 semantic receipt self-test passed: strict 38-field CBOR, 8 receipt hostiles, 10 pressure transcripts, 92-ID registry, and 10 authority-negative flags."
    );
}
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--self-test") {
        self_test();
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--start-at-unix-ms") {
        let target = args
            .get(index + 1)
            .expect("missing start time")
            .parse::<u128>()
            .expect("invalid start time");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before epoch")
            .as_millis();
        if target > now {
            thread::sleep(Duration::from_millis((target - now) as u64));
        }
    }
    println!("{}", hex(&expected()));
}
