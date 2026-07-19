use hierarchy_history::{
    AddressPresence, BaselineManifest, DeltaEnvelope, DependencyRef, DescriptorOrigin,
    HierarchyDescriptor, HierarchyHistoryError, HistoryStream, RecoveryFailureKind,
    ReferenceOperation, Snapshot, UnsupportedTopologyOperation, deterministic_cost_evidence,
    dynamic_instance_logical_id, identity_reference_chain, recover_known_good_prefix,
    reference_operation_schema, validate_identity_reference_chain,
};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn descriptor(marker: u8) -> HierarchyDescriptor {
    HierarchyDescriptor::new(
        [marker; 32],
        None,
        [2; 32],
        [3; 32],
        [4; 32],
        DescriptorOrigin::Procedural,
        vec![marker],
    )
    .unwrap()
}

fn baseline(marker: u8) -> BaselineManifest {
    let descriptor = descriptor(marker);
    BaselineManifest::new(
        descriptor.logical_id,
        descriptor.fingerprint().unwrap(),
        vec![
            DependencyRef {
                kind: 1,
                fingerprint: [11; 32],
            },
            DependencyRef {
                kind: 2,
                fingerprint: [12; 32],
            },
        ],
    )
    .unwrap()
}

fn migration_baseline(marker: u8) -> BaselineManifest {
    let descriptor = descriptor(marker);
    BaselineManifest::new(
        [1; 32],
        descriptor.fingerprint().unwrap(),
        vec![
            DependencyRef {
                kind: 1,
                fingerprint: [marker.wrapping_add(10); 32],
            },
            DependencyRef {
                kind: 2,
                fingerprint: [12; 32],
            },
        ],
    )
    .unwrap()
}

fn event(stream: &HistoryStream, sequence: u64, marker: u8) -> DeltaEnvelope {
    DeltaEnvelope::new(
        stream.baseline_key(),
        stream.baseline().logical_id,
        sequence,
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

#[test]
fn dynamic_identity_and_four_presence_states_are_strict() {
    let id = dynamic_instance_logical_id([1; 32], [2; 32]).unwrap();
    assert_eq!(id, dynamic_instance_logical_id([1; 32], [2; 32]).unwrap());
    assert_ne!(id, dynamic_instance_logical_id([1; 32], [3; 32]).unwrap());
    assert!(
        dynamic_instance_logical_id([0; 32], [2; 32]).is_err(),
        "identity.dynamic-zero-parent"
    );
    assert!(
        dynamic_instance_logical_id([1; 32], [0; 32]).is_err(),
        "identity.dynamic-zero-instance"
    );
    assert_eq!(
        hex(&id),
        "a934a928fe8936e4a8a9f028cd8cfd3c80208dfcc13ec15d11a03d5e70daeef6",
        "identity.dynamic-vector-drift"
    );
    assert_eq!(
        hex(&dynamic_instance_logical_id([1; 32], [3; 32]).unwrap()),
        "94b8f59ffb72848a7a21dd243449ddf978be50801c7475e8528dfba41fc8cde0",
        "identity.dynamic-domain-drift"
    );
    let states = [
        AddressPresence::NeverObserved,
        AddressPresence::Absent {
            address_fingerprint: [1; 32],
        },
        AddressPresence::Present {
            descriptor_fingerprint: [2; 32],
        },
        AddressPresence::Tombstoned {
            prior_descriptor_fingerprint: [2; 32],
            tombstone_delta: [3; 32],
        },
    ];
    let mut fingerprints = Vec::new();
    for state in states {
        let bytes = state.encode_canonical().unwrap();
        assert_eq!(AddressPresence::decode_strict(&bytes).unwrap(), state);
        fingerprints.push(state.fingerprint().unwrap());
    }
    fingerprints.sort();
    fingerprints.dedup();
    assert_eq!(fingerprints.len(), 4);
    let absent = AddressPresence::Absent {
        address_fingerprint: [1; 32],
    };
    let mut unknown = absent.encode_canonical().unwrap();
    unknown[4] = 9;
    assert!(
        AddressPresence::decode_strict(&unknown).is_err(),
        "presence.unknown-tag"
    );
    assert!(
        AddressPresence::Absent {
            address_fingerprint: [0; 32]
        }
        .encode_canonical()
        .is_err(),
        "presence.zero-fingerprint"
    );
    let mut trailing = absent.encode_canonical().unwrap();
    trailing.push(0);
    assert!(
        AddressPresence::decode_strict(&trailing).is_err(),
        "presence.trailing-bytes"
    );
    let substituted = AddressPresence::Present {
        descriptor_fingerprint: [1; 32],
    };
    assert_ne!(
        absent.fingerprint().unwrap(),
        substituted.fingerprint().unwrap(),
        "presence.state-substitution"
    );
}

#[test]
fn dependency_availability_is_exact() {
    let manifest = baseline(1);
    let exact = manifest.output_dependencies.clone();
    manifest.verify_available_dependencies(&exact).unwrap();
    assert_eq!(
        manifest.verify_available_dependencies(&exact[..1]),
        Err(HierarchyHistoryError::MissingDependency(2)),
        "dependency.missing"
    );
    let mut wrong = exact.clone();
    wrong[1].fingerprint = [99; 32];
    assert_eq!(
        manifest.verify_available_dependencies(&wrong),
        Err(HierarchyHistoryError::DependencyFingerprintMismatch(2)),
        "dependency.fingerprint-mismatch"
    );
    let mut extra = exact.clone();
    extra.push(DependencyRef {
        kind: 3,
        fingerprint: [13; 32],
    });
    assert_eq!(
        manifest.verify_available_dependencies(&extra),
        Err(HierarchyHistoryError::InvalidDependencyAvailability),
        "dependency.extra"
    );
    let mut c3b = exact.clone();
    c3b.push(DependencyRef {
        kind: 0x0c3b,
        fingerprint: [33; 32],
    });
    assert!(
        manifest.verify_available_dependencies(&c3b).is_err(),
        "dependency.c3b-extra"
    );
    let mut unsorted = exact.clone();
    unsorted.swap(0, 1);
    assert!(
        manifest.verify_available_dependencies(&unsorted).is_err(),
        "dependency.unsorted"
    );
    let duplicate = vec![exact[0], exact[0]];
    assert!(
        manifest.verify_available_dependencies(&duplicate).is_err(),
        "dependency.duplicate"
    );
    let zero = vec![DependencyRef {
        kind: 0,
        fingerprint: [1; 32],
    }];
    assert!(
        manifest.verify_available_dependencies(&zero).is_err(),
        "dependency.zero-kind"
    );
    let mut invalid = manifest.clone();
    invalid.output_dependencies = unsorted;
    assert!(
        invalid.verify_available_dependencies(&exact).is_err(),
        "dependency.manifest-invalid"
    );
}

#[test]
fn corrupt_tail_recovers_only_semantically_replayable_prefix() {
    let baseline = baseline(1);
    let mut source = HistoryStream::new(baseline.clone()).unwrap();
    let first = event(&source, 1, 1);
    source.append(first.clone()).unwrap();
    let second = event(&source, 2, 2);
    let mut corrupt = second.encode_canonical().unwrap();
    corrupt[3] ^= 0xff;
    let records = vec![
        first.encode_canonical().unwrap(),
        corrupt,
        event(&source, 2, 3).encode_canonical().unwrap(),
    ];
    let recovered = recover_known_good_prefix(baseline.clone(), &records).unwrap();
    assert_eq!(
        recovered.accepted_records, 1,
        "history.recovery-past-prefix"
    );
    assert_eq!(
        recovered.first_failure,
        Some(RecoveryFailureKind::CorruptContent),
        "history.corrupt-envelope"
    );
    assert_eq!(recovered.stream.head(), Some(first.content_id));
    let mut truncated = first.encode_canonical().unwrap();
    truncated.pop();
    assert!(
        DeltaEnvelope::decode_strict(&truncated).is_err(),
        "history.truncated-envelope"
    );
    let mut trailing = first.encode_canonical().unwrap();
    trailing.push(0);
    assert!(
        DeltaEnvelope::decode_strict(&trailing).is_err(),
        "history.trailing-envelope"
    );
    assert!(
        matches!(
            recover_known_good_prefix(baseline.clone(), &vec![Vec::new(); 1025]),
            Err(HierarchyHistoryError::RecoveryBoundExceeded)
        ),
        "history.recovery-bound-overflow"
    );
    assert!(
        matches!(
            recover_known_good_prefix(baseline, &[vec![0; 16 * 1024 * 1024 + 1]]),
            Err(HierarchyHistoryError::RecoveryBoundExceeded)
        ),
        "history.recovery-bound-overflow"
    );
}

#[test]
fn history_scope_order_and_semantics_fail_closed() {
    let base = baseline(1);
    let mut stream = HistoryStream::new(base.clone()).unwrap();
    let mut wrong_baseline = event(&stream, 1, 1);
    wrong_baseline.baseline_key = [9; 32];
    assert_eq!(
        stream.append(wrong_baseline),
        Err(HierarchyHistoryError::WrongBaseline),
        "history.wrong-baseline"
    );
    let mut wrong_target = event(&stream, 1, 1);
    wrong_target.target_logical_id = [9; 32];
    assert_eq!(
        stream.append(wrong_target),
        Err(HierarchyHistoryError::WrongTarget),
        "history.wrong-target"
    );
    let gap = event(&stream, 2, 2);
    assert_eq!(
        stream.append(gap),
        Err(HierarchyHistoryError::Gap),
        "history.gap"
    );
    let first = event(&stream, 1, 1);
    stream.append(first.clone()).unwrap();
    let mut forged_retry = first.clone();
    forged_retry.operation = ReferenceOperation::Set { key: 99, value: 99 }
        .encode_canonical()
        .unwrap();
    assert_eq!(
        stream.append(forged_retry),
        Err(HierarchyHistoryError::CorruptContent),
        "history.command-conflict"
    );
    let conflict = DeltaEnvelope::new(
        stream.baseline_key(),
        stream.baseline().logical_id,
        2,
        stream.head(),
        [1; 32],
        reference_operation_schema(),
        ReferenceOperation::Set { key: 2, value: 2 }
            .encode_canonical()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        stream.append(conflict),
        Err(HierarchyHistoryError::CommandConflict),
        "history.command-conflict"
    );
    let fork = DeltaEnvelope::new(
        stream.baseline_key(),
        stream.baseline().logical_id,
        2,
        Some([99; 32]),
        [2; 32],
        reference_operation_schema(),
        ReferenceOperation::Set { key: 2, value: 2 }
            .encode_canonical()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        stream.append(fork),
        Err(HierarchyHistoryError::ForkConflict),
        "history.fork"
    );
    let mut semantic = HistoryStream::new(base.clone()).unwrap();
    let unknown = DeltaEnvelope::new(
        semantic.baseline_key(),
        semantic.baseline().logical_id,
        1,
        None,
        [3; 32],
        [88; 32],
        ReferenceOperation::Set { key: 1, value: 1 }
            .encode_canonical()
            .unwrap(),
    )
    .unwrap();
    semantic.append(unknown).unwrap();
    assert_eq!(
        semantic.replay_reference(),
        Err(HierarchyHistoryError::UnknownOperationSchema),
        "history.unknown-schema"
    );
    let recovered = recover_known_good_prefix(
        base.clone(),
        &semantic
            .events()
            .iter()
            .map(|item| item.encode_canonical().unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(
        recovered.first_failure,
        Some(RecoveryFailureKind::UnknownOperationSchema)
    );
    let mut cross = HistoryStream::new(base).unwrap();
    let cross_event = DeltaEnvelope::new(
        cross.baseline_key(),
        cross.baseline().logical_id,
        1,
        None,
        [4; 32],
        reference_operation_schema(),
        ReferenceOperation::CrossTarget {
            other_target: [4; 32],
        }
        .encode_canonical()
        .unwrap(),
    )
    .unwrap();
    cross.append(cross_event).unwrap();
    assert_eq!(
        cross.replay_reference(),
        Err(HierarchyHistoryError::UnsupportedCrossTarget),
        "history.cross-target"
    );
}

#[test]
fn stale_head_and_zero_sequence_fail_without_panicking_or_mutating() {
    let mut stream = HistoryStream::new(baseline(1)).unwrap();
    let first = event(&stream, 1, 1);
    stream.append(first.clone()).unwrap();
    let second = event(&stream, 2, 2);
    stream.append(second).unwrap();
    let before = (stream.head(), stream.events().len());
    let stale = DeltaEnvelope::new(
        stream.baseline_key(),
        stream.baseline().logical_id,
        3,
        Some(first.content_id),
        [9; 32],
        reference_operation_schema(),
        ReferenceOperation::Set { key: 9, value: 9 }
            .encode_canonical()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        stream.append(stale),
        Err(HierarchyHistoryError::StaleHead),
        "history.stale-head"
    );
    let mut zero = event(&stream, 3, 3);
    zero.sequence = 0;
    assert_eq!(
        stream.append(zero),
        Err(HierarchyHistoryError::CorruptContent)
    );
    assert_eq!((stream.head(), stream.events().len()), before);
}

#[test]
fn topology_changes_fail_without_mutation() {
    let mut stream = HistoryStream::new(baseline(1)).unwrap();
    let before = (stream.baseline_key(), stream.head(), stream.events().len());
    for (id, operation) in [
        ("history.reparent", UnsupportedTopologyOperation::Reparent),
        ("history.split", UnsupportedTopologyOperation::Split),
        ("history.merge", UnsupportedTopologyOperation::Merge),
    ] {
        assert_eq!(
            stream.reject_topology_change(operation),
            Err(HierarchyHistoryError::UnsupportedTopology),
            "{id}"
        );
        assert_eq!(
            (stream.baseline_key(), stream.head(), stream.events().len()),
            before,
            "{id}"
        );
    }
}

#[test]
fn snapshot_every_bound_field_is_anchored() {
    let mut stream = HistoryStream::new(baseline(1)).unwrap();
    stream.append(event(&stream, 1, 1)).unwrap();
    let expected = stream.replay_reference().unwrap();
    let builder = [7; 32];
    let snapshot = Snapshot::build_reference(&stream, builder).unwrap();
    snapshot
        .verify_reference_with_builder(&stream, &expected, builder)
        .unwrap();
    let mut cases = Vec::new();
    let mut value = snapshot.clone();
    value.baseline_key = [9; 32];
    cases.push(("snapshot.wrong-baseline", value));
    let mut value = snapshot.clone();
    value.covered_head = None;
    cases.push(("snapshot.wrong-head", value));
    let mut value = snapshot.clone();
    value.sequence = 9;
    cases.push(("snapshot.wrong-sequence", value));
    let mut value = snapshot.clone();
    value.reducer_fingerprint = [9; 32];
    cases.push(("snapshot.wrong-reducer", value));
    let mut value = snapshot.clone();
    value.state_bytes.push(0);
    cases.push(("snapshot.wrong-state", value));
    let mut value = snapshot.clone();
    value.state_hash = [9; 32];
    cases.push(("snapshot.wrong-hash", value));
    for (id, value) in cases {
        assert_eq!(
            value.verify_reference_with_builder(&stream, &expected, builder),
            Err(HierarchyHistoryError::SnapshotMismatch),
            "{id}"
        );
    }
    let rebuilt = Snapshot::build_reference(&stream, [8; 32]).unwrap();
    assert_eq!(
        rebuilt.verify_reference_with_builder(&stream, &expected, builder),
        Err(HierarchyHistoryError::SnapshotMismatch),
        "snapshot.wrong-builder"
    );
}

#[test]
fn two_hop_migration_is_atomic_and_validated() {
    let source = HistoryStream::new(baseline(1)).unwrap();
    let targets = [migration_baseline(2), migration_baseline(3)];
    let receipts = identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]]).unwrap();
    assert_eq!(receipts.len(), 2);
    for receipt in &receipts {
        receipt.validate_content_id().unwrap();
    }
    assert_eq!(receipts[0].to_baseline, receipts[1].from_baseline);
    validate_identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]], &receipts).unwrap();
    let mut reordered = receipts.clone();
    reordered.swap(0, 1);
    assert_eq!(
        validate_identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]], &reordered),
        Err(HierarchyHistoryError::MigrationChainInvalid),
        "migration.reordered-hop"
    );
    assert_eq!(
        identity_reference_chain(&source, &targets, &[[21; 32]]),
        Err(HierarchyHistoryError::MigrationChainInvalid),
        "migration.missing-adapter"
    );
    assert!(
        identity_reference_chain(&source, &targets, &[[21; 32], [0; 32]]).is_err(),
        "migration.zero-adapter"
    );
    assert!(
        identity_reference_chain(&source, &targets, &[[21; 32], [21; 32]]).is_err(),
        "migration.duplicate-adapter"
    );
    assert!(
        identity_reference_chain(&source, &[baseline(2)], &[[21; 32]]).is_err(),
        "migration.wrong-logical-id"
    );
    assert!(
        identity_reference_chain(&source, &[baseline(1)], &[[21; 32]]).is_err(),
        "migration.same-baseline"
    );
    assert!(
        identity_reference_chain(
            &source,
            &[
                migration_baseline(2),
                migration_baseline(3),
                migration_baseline(4)
            ],
            &[[21; 32], [22; 32], [23; 32]]
        )
        .is_err(),
        "migration.overbound"
    );
    let cycle = [migration_baseline(2), baseline(1)];
    assert_eq!(
        identity_reference_chain(&source, &cycle, &[[21; 32], [22; 32]]),
        Err(HierarchyHistoryError::MigrationChainInvalid),
        "migration.failed-hop"
    );
    let direct = identity_reference_chain(&source, &[migration_baseline(3)], &[[22; 32]]).unwrap();
    let noncontiguous = vec![receipts[0].clone(), direct[0].clone()];
    assert!(
        validate_identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]], &noncontiguous)
            .is_err(),
        "migration.noncontiguous-hop"
    );
    assert!(
        validate_identity_reference_chain(&source, &targets, &[[21; 32], [23; 32]], &receipts)
            .is_err(),
        "migration.changed-retry"
    );
    let mut tampered = receipts.clone();
    tampered[1].content_id[0] ^= 1;
    assert!(
        validate_identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]], &tampered)
            .is_err(),
        "migration.receipt-tamper"
    );
    let before = (
        source.baseline_key(),
        source.head(),
        source.events().len(),
        source.replay_reference().unwrap(),
    );
    let _ = identity_reference_chain(&source, &targets, &[[21; 32], [22; 32]]).unwrap();
    assert_eq!(
        (
            source.baseline_key(),
            source.head(),
            source.events().len(),
            source.replay_reference().unwrap()
        ),
        before
    );
    let mut altered = source.clone();
    let altered_event = event(&altered, 1, 7);
    altered.append(altered_event).unwrap();
    assert!(
        validate_identity_reference_chain(&altered, &targets, &[[21; 32], [22; 32]], &receipts)
            .is_err(),
        "migration.altered-source"
    );
}

#[test]
fn deterministic_cost_rows_are_exact_and_repeatable() {
    let first = deterministic_cost_evidence().unwrap();
    let second = deterministic_cost_evidence().unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first
            .0
            .iter()
            .map(|row| (
                row.requested,
                row.returned,
                row.examined,
                row.canonical_bytes
            ))
            .collect::<Vec<_>>(),
        vec![
            (0, 0, 0, 43),
            (1, 1, 1, 113),
            (16, 16, 16, 1163),
            (256, 256, 256, 18201)
        ]
    );
    assert_eq!(
        first
            .1
            .iter()
            .map(|row| (
                row.event_count,
                row.encoded_delta_bytes,
                row.full_stream_bytes,
                row.decoded_operation_count,
                row.snapshot_bytes
            ))
            .collect::<Vec<_>>(),
        vec![
            (0, 0, 148, 0, 179),
            (1, 182, 332, 1, 215),
            (16, 3407, 3587, 16, 261),
            (64, 13848, 14125, 64, 489),
            (256, 55705, 56367, 256, 1451)
        ]
    );
}
