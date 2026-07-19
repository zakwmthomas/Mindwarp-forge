use addressable_world_binding::bind_addressable_world_package;
use derived_world_rules::compile_world;
use entity_lifecycle::AgeCohort;
use entity_lifecycle_history_binding::AmbientCohortBindingV1;
use hierarchy_history::{
    AddressPresence, BaselineManifest, DeltaEnvelope, DependencyRef, HistoryStream,
    RecoveryFailureKind, ReferenceOperation, Snapshot, deterministic_cost_evidence,
    dynamic_instance_logical_id, identity_reference_chain, recover_known_good_prefix,
    reference_operation_schema,
};
use minicbor::{Decoder, Encoder};
use sha2::{Digest, Sha256};
use std::{
    env, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod world_support {
    include!(concat!(
        env!("FORGE_ROOT"),
        "/crates/mindwarp-gameplay-foundation/tests/world_support/mod.rs"
    ));
}

const DOMAIN: &[u8] = b"mindwarp/c4-semantic-receipt/v1\0";
const HOSTILE_DOMAIN: &[u8] = b"mindwarp/c4-hostile-registry/v1\0";
const HOSTILES: [&str; 74] = [
    "identity.dynamic-zero-parent",
    "identity.dynamic-zero-instance",
    "identity.dynamic-domain-drift",
    "identity.dynamic-vector-drift",
    "presence.unknown-tag",
    "presence.state-substitution",
    "presence.zero-fingerprint",
    "presence.trailing-bytes",
    "cohort.zero-entity",
    "cohort.zero-contract",
    "cohort.entity-drift",
    "cohort.contract-drift",
    "cohort.value-drift",
    "cohort.reroll",
    "cohort.trailing-bytes",
    "dependency.manifest-invalid",
    "dependency.missing",
    "dependency.fingerprint-mismatch",
    "dependency.extra",
    "dependency.c3b-extra",
    "dependency.unsorted",
    "dependency.duplicate",
    "dependency.zero-kind",
    "history.wrong-baseline",
    "history.wrong-target",
    "history.gap",
    "history.stale-head",
    "history.fork",
    "history.command-conflict",
    "history.unknown-schema",
    "history.cross-target",
    "history.reparent",
    "history.split",
    "history.merge",
    "history.corrupt-envelope",
    "history.truncated-envelope",
    "history.trailing-envelope",
    "history.recovery-past-prefix",
    "history.recovery-bound-overflow",
    "snapshot.wrong-baseline",
    "snapshot.wrong-head",
    "snapshot.wrong-sequence",
    "snapshot.wrong-reducer",
    "snapshot.wrong-builder",
    "snapshot.wrong-state",
    "snapshot.wrong-hash",
    "migration.missing-adapter",
    "migration.zero-adapter",
    "migration.duplicate-adapter",
    "migration.wrong-logical-id",
    "migration.same-baseline",
    "migration.reordered-hop",
    "migration.noncontiguous-hop",
    "migration.failed-hop",
    "migration.overbound",
    "migration.altered-source",
    "migration.changed-retry",
    "migration.receipt-tamper",
    "receipt.unknown-field",
    "receipt.missing-field",
    "receipt.dependency-reorder",
    "receipt.type-coercion",
    "receipt.proof-drift",
    "receipt.source-drift",
    "receipt.authority-flip",
    "receipt.hash-drift",
    "portability.single-process",
    "portability.stdout-mismatch",
    "portability.source-mismatch",
    "portability.compile-as-execution",
    "portability.same-host-as-independent",
    "portability.same-platform-remote",
    "portability.target-drift",
    "portability.runner-drift",
];

fn sha(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(text: &str) -> [u8; 32] {
    assert_eq!(text.len(), 64);
    let mut out = [0; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap();
    }
    out
}
fn field<F>(write: F) -> Vec<u8>
where
    F: FnOnce(&mut Encoder<Vec<u8>>),
{
    let mut e = Encoder::new(Vec::new());
    write(&mut e);
    e.into_writer()
}
fn f_u16(v: u16) -> Vec<u8> {
    field(|e| {
        e.u16(v).unwrap();
    })
}
fn f_text(v: &str) -> Vec<u8> {
    field(|e| {
        e.str(v).unwrap();
    })
}
fn f_bytes(v: &[u8]) -> Vec<u8> {
    field(|e| {
        e.bytes(v).unwrap();
    })
}
fn f_bool(v: bool) -> Vec<u8> {
    field(|e| {
        e.bool(v).unwrap();
    })
}
fn f_bytes_array(values: &[[u8; 32]]) -> Vec<u8> {
    field(|e| {
        e.array(values.len() as u64).unwrap();
        for value in values {
            e.bytes(value).unwrap();
        }
    })
}
fn recovery_tag(value: Option<RecoveryFailureKind>) -> u8 {
    match value {
        Some(RecoveryFailureKind::CorruptContent) => 0,
        Some(RecoveryFailureKind::InvalidEnvelope) => 1,
        Some(RecoveryFailureKind::WrongBaseline) => 2,
        Some(RecoveryFailureKind::WrongTarget) => 3,
        Some(RecoveryFailureKind::Gap) => 4,
        Some(RecoveryFailureKind::StaleHead) => 5,
        Some(RecoveryFailureKind::ForkConflict) => 6,
        Some(RecoveryFailureKind::CommandConflict) => 7,
        Some(RecoveryFailureKind::UnknownOperationSchema) => 8,
        Some(RecoveryFailureKind::UnsupportedCrossTarget) => 9,
        None => 255,
    }
}

fn event(stream: &HistoryStream, marker: u8) -> DeltaEnvelope {
    DeltaEnvelope::new(
        stream.baseline_key(),
        stream.baseline().logical_id,
        stream.events().len() as u64 + 1,
        stream.head(),
        [marker; 32],
        reference_operation_schema(),
        ReferenceOperation::Set {
            key: marker as u16,
            value: marker as i64,
        }
        .encode_canonical()
        .unwrap(),
    )
    .unwrap()
}

fn expected_fields() -> Vec<Vec<u8>> {
    let input = world_support::world_input([0x4a; 32]);
    let input_bytes = input.to_bytes().unwrap();
    assert_eq!(
        hex(&sha(&input_bytes)),
        "5f54137fa9de4b06514dbfde509ef5faf65a23b885a24288ed5cb51bbcee07ca"
    );
    let packet = compile_world(&input).unwrap();
    let packet_bytes = packet.to_bytes().unwrap();
    assert_eq!(
        packet.packet_id,
        "947a0564c7a08115d4ee63ff89bfbdafdc9303ecd7f86c846b4945c7e305492b"
    );
    assert_eq!(
        hex(&sha(&packet_bytes)),
        "e3479b36a3e7085ae892a358ba7e5e6415688ef0d82e0338b9226ae71c46576f"
    );
    let descriptor = bind_addressable_world_package(
        [1; 32],
        None,
        [2; 32],
        &input,
        &packet,
        b"c4-semantic-v1".to_vec(),
    )
    .unwrap();
    let descriptor_digest = descriptor.fingerprint().unwrap();
    let dynamic_ids = [
        dynamic_instance_logical_id(descriptor.logical_id, [1; 32]).unwrap(),
        dynamic_instance_logical_id(descriptor.logical_id, [2; 32]).unwrap(),
    ];
    let presence = [
        AddressPresence::NeverObserved.fingerprint().unwrap(),
        AddressPresence::Absent {
            address_fingerprint: [1; 32],
        }
        .fingerprint()
        .unwrap(),
        AddressPresence::Present {
            descriptor_fingerprint: descriptor_digest,
        }
        .fingerprint()
        .unwrap(),
        AddressPresence::Tombstoned {
            prior_descriptor_fingerprint: descriptor_digest,
            tombstone_delta: [3; 32],
        }
        .fingerprint()
        .unwrap(),
    ];
    let cohort = AmbientCohortBindingV1::new(descriptor.logical_id, [0xa4; 32], AgeCohort::Adult)
        .unwrap()
        .fingerprint();
    let baselines: [BaselineManifest; 3] = [11_u8, 12, 13].map(|v| {
        BaselineManifest::new(
            descriptor.logical_id,
            descriptor_digest,
            vec![
                DependencyRef {
                    kind: 1,
                    fingerprint: [v; 32],
                },
                DependencyRef {
                    kind: 2,
                    fingerprint: descriptor.world_conditions_fingerprint,
                },
            ],
        )
        .unwrap()
    });
    let mut streams: Vec<HistoryStream> = baselines
        .iter()
        .cloned()
        .map(|b| HistoryStream::new(b).unwrap())
        .collect();
    for (i, stream) in streams.iter_mut().enumerate() {
        let value = event(stream, i as u8 + 1);
        stream.append(value).unwrap();
    }
    let baseline_keys = [
        streams[0].baseline_key(),
        streams[1].baseline_key(),
        streams[2].baseline_key(),
    ];
    let heads = [
        streams[0].head().unwrap(),
        streams[1].head().unwrap(),
        streams[2].head().unwrap(),
    ];
    let snapshot = Snapshot::build_reference(&streams[0], [0x42; 32]).unwrap();
    let mut recovery_records = streams[0]
        .events()
        .iter()
        .map(|v| v.encode_canonical().unwrap())
        .collect::<Vec<_>>();
    let mut corrupt = event(&streams[0], 9).encode_canonical().unwrap();
    corrupt[3] ^= 0xff;
    recovery_records.push(corrupt);
    let recovered = recover_known_good_prefix(baselines[0].clone(), &recovery_records).unwrap();
    assert_eq!(recovered.accepted_records, 1);
    assert_eq!(
        recovered.first_failure,
        Some(RecoveryFailureKind::CorruptContent)
    );
    let migrations =
        identity_reference_chain(&streams[0], &baselines[1..], &[[0x21; 32], [0x22; 32]]).unwrap();
    let rollback_sha = sha(&streams[0].encode_canonical().unwrap());
    let (window_rows, history_rows) = deterministic_cost_evidence().unwrap();
    let hostile_digest = sha(&[HOSTILE_DOMAIN, HOSTILES.join("\n").as_bytes()].concat());
    assert_eq!(
        hex(&hostile_digest),
        "4d4b7cb792f5b410092d247354bac62a5b8f3dc880fcb2a6ad61ffafadff127c"
    );

    let c2 = field(|e| {
        e.array(3).unwrap();
        e.u8(2).unwrap();
        e.u8(1).unwrap();
        e.bytes(&unhex(
            "bbd80968996612cca154ad29e324d9fdeb50072f38fd41d2c43699bacdb2da3d",
        ))
        .unwrap();
    });
    let c3 = field(|e| {
        e.array(5).unwrap();
        e.bytes(&sha(&input_bytes)).unwrap();
        e.bytes(&unhex(&packet.packet_id)).unwrap();
        e.bytes(&sha(&packet_bytes)).unwrap();
        e.bytes(&descriptor_digest).unwrap();
        e.bytes(&descriptor.world_conditions_fingerprint).unwrap();
    });
    let recovery = field(|e| {
        e.array(5).unwrap();
        e.bytes(&recovered.stream.baseline_key()).unwrap();
        e.u16(recovered.accepted_records as u16).unwrap();
        e.bytes(&recovered.stream.head().unwrap()).unwrap();
        e.u8(recovery_tag(recovered.first_failure)).unwrap();
        let mut framed = Encoder::new(Vec::new());
        framed.array(recovery_records.len() as u64).unwrap();
        for row in &recovery_records {
            framed.bytes(row).unwrap();
        }
        e.bytes(&sha(&framed.into_writer())).unwrap();
    });
    let window = field(|e| {
        e.array(window_rows.len() as u64).unwrap();
        for r in &window_rows {
            e.array(4).unwrap();
            e.u16(r.requested).unwrap();
            e.u16(r.returned).unwrap();
            e.u16(r.examined).unwrap();
            e.u64(r.canonical_bytes).unwrap();
        }
    });
    let history = field(|e| {
        e.array(history_rows.len() as u64).unwrap();
        for r in &history_rows {
            e.array(5).unwrap();
            e.u16(r.event_count).unwrap();
            e.u64(r.encoded_delta_bytes).unwrap();
            e.u64(r.full_stream_bytes).unwrap();
            e.u16(r.decoded_operation_count).unwrap();
            e.u64(r.snapshot_bytes).unwrap();
        }
    });
    vec![
        f_u16(1),
        f_text("G1-C4-HIERARCHY-HISTORY"),
        f_text("hierarchy-history-v1"),
        f_text("c4-fixed-c3a-v1"),
        c2,
        c3,
        f_bytes(&descriptor_digest),
        f_bytes_array(&dynamic_ids),
        f_bytes_array(&presence),
        f_bytes(&cohort),
        f_bytes_array(&baseline_keys),
        f_bytes_array(&heads),
        f_bytes(&snapshot.content_id),
        recovery,
        f_bytes_array(&[migrations[0].content_id, migrations[1].content_id]),
        f_bytes(&migrations[1].output_state_hash),
        f_bytes(&rollback_sha),
        window,
        history,
        f_bytes(&hostile_digest),
        f_u16(74),
        f_bool(false),
        f_bool(false),
        f_bool(false),
        f_bool(false),
        f_bool(false),
    ]
}

fn assemble(fields: &[Vec<u8>], recompute_hash: bool) -> Vec<u8> {
    let mut hash = Sha256::new();
    hash.update(DOMAIN);
    for value in fields {
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    let digest: [u8; 32] = hash.finalize().into();
    let mut out = vec![0x98, 27];
    for value in fields {
        out.extend_from_slice(value);
    }
    let chosen = if recompute_hash { digest } else { [0; 32] };
    out.extend_from_slice(&f_bytes(&chosen));
    out
}
fn expected() -> Vec<u8> {
    assemble(&expected_fields(), true)
}
fn arr(d: &mut Decoder<'_>, n: u64) -> Result<(), ()> {
    if d.array().map_err(|_| ())? != Some(n) {
        Err(())
    } else {
        Ok(())
    }
}
fn b32(d: &mut Decoder<'_>) -> Result<(), ()> {
    if d.bytes().map_err(|_| ())?.len() != 32 {
        Err(())
    } else {
        Ok(())
    }
}
fn parse_field(index: usize, d: &mut Decoder<'_>) -> Result<(), ()> {
    match index {
        0 | 20 => {
            d.u16().map_err(|_| ())?;
        }
        1..=3 => {
            if d.str().map_err(|_| ())?.len() > 64 {
                return Err(());
            }
        }
        4 => {
            arr(d, 3)?;
            d.u8().map_err(|_| ())?;
            d.u8().map_err(|_| ())?;
            b32(d)?;
        }
        5 => {
            arr(d, 5)?;
            for _ in 0..5 {
                b32(d)?;
            }
        }
        6 | 9 | 12 | 15 | 16 | 19 => b32(d)?,
        7 | 14 => {
            arr(d, 2)?;
            b32(d)?;
            b32(d)?;
        }
        8 => {
            arr(d, 4)?;
            for _ in 0..4 {
                b32(d)?;
            }
        }
        10 | 11 => {
            arr(d, 3)?;
            for _ in 0..3 {
                b32(d)?;
            }
        }
        13 => {
            arr(d, 5)?;
            b32(d)?;
            d.u16().map_err(|_| ())?;
            b32(d)?;
            let tag = d.u8().map_err(|_| ())?;
            if tag > 9 {
                return Err(());
            }
            b32(d)?;
        }
        17 => {
            arr(d, 4)?;
            for _ in 0..4 {
                arr(d, 4)?;
                d.u16().map_err(|_| ())?;
                d.u16().map_err(|_| ())?;
                d.u16().map_err(|_| ())?;
                d.u64().map_err(|_| ())?;
            }
        }
        18 => {
            arr(d, 5)?;
            for _ in 0..5 {
                arr(d, 5)?;
                d.u16().map_err(|_| ())?;
                d.u64().map_err(|_| ())?;
                d.u64().map_err(|_| ())?;
                d.u16().map_err(|_| ())?;
                d.u64().map_err(|_| ())?;
            }
        }
        21..=25 => {
            d.bool().map_err(|_| ())?;
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
    let mut d = Decoder::new(bytes);
    arr(&mut d, 27)?;
    let mut received = Vec::new();
    for (index, expected_field) in expected_fields.iter().enumerate() {
        let start = d.position();
        parse_field(index, &mut d)?;
        let actual = bytes[start..d.position()].to_vec();
        if &actual != expected_field {
            return Err(());
        }
        received.push(actual);
    }
    let received_hash = d.bytes().map_err(|_| ())?;
    if received_hash.len() != 32 || d.position() != bytes.len() {
        return Err(());
    }
    let rebuilt = assemble(&received, true);
    if rebuilt != bytes {
        return Err(());
    }
    Ok(())
}
fn self_test() {
    let expected = expected();
    validate(&expected).unwrap();
    let fields = expected_fields();
    let mut cases: Vec<(&str, Vec<u8>)> = Vec::new();
    let mut value = expected.clone();
    value[1] = 28;
    value.push(0xf6);
    cases.push(("receipt.unknown-field", value));
    cases.push(("receipt.missing-field", assemble(&fields[..25], true)));
    let mut changed = fields.clone();
    changed.swap(4, 5);
    cases.push(("receipt.dependency-reorder", assemble(&changed, true)));
    let mut changed = fields.clone();
    changed[0] = f_text("1");
    cases.push(("receipt.type-coercion", assemble(&changed, true)));
    let mut changed = fields.clone();
    let last = changed[4].len() - 1;
    changed[4][last] ^= 1;
    cases.push(("receipt.proof-drift", assemble(&changed, true)));
    let mut changed = fields.clone();
    let last = changed[5].len() - 1;
    changed[5][last] ^= 1;
    cases.push(("receipt.source-drift", assemble(&changed, true)));
    let mut changed = fields.clone();
    changed[25] = f_bool(true);
    cases.push(("receipt.authority-flip", assemble(&changed, true)));
    let mut value = expected.clone();
    let last = value.len() - 1;
    value[last] ^= 1;
    cases.push(("receipt.hash-drift", value));
    for (id, bytes) in cases {
        assert!(validate(&bytes).is_err(), "{id}");
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
        "C4 semantic receipt self-test passed: 8 receipt hostiles, 74-ID registry, exact C2+C3A replay and authority-negative bytes."
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
    println!("{}", hex(&expected()))
}
