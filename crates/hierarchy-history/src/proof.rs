use serde::Serialize;

use crate::{
    BaselineManifest, CONTRACT_VERSION, DependencyRef, DescriptorOrigin, HierarchyDescriptor,
    HierarchyHistoryError, hex, observe_fixture_window,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HierarchyHistoryProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub contract_version: String,
    pub descriptor_fingerprint: String,
    pub baseline_key: String,
    pub measured_window_sizes: Vec<(u64, usize, u16)>,
    pub measurement_classification: String,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

pub fn reference_proof_evidence() -> Result<HierarchyHistoryProofEvidence, HierarchyHistoryError> {
    let descriptor = HierarchyDescriptor::new(
        [1; 32],
        None,
        [2; 32],
        [3; 32],
        [4; 32],
        DescriptorOrigin::Procedural,
        b"reference-root-v1".to_vec(),
    )?;
    let baseline = BaselineManifest::new(
        descriptor.logical_id,
        descriptor.fingerprint()?,
        vec![
            DependencyRef {
                kind: 1,
                fingerprint: descriptor.reconstruction_fingerprint,
            },
            DependencyRef {
                kind: 2,
                fingerprint: descriptor.world_conditions_fingerprint,
            },
        ],
    )?;
    let measured_window_sizes = [0_u64, 16, 256, 1024]
        .into_iter()
        .map(|logical_count| {
            let limit = logical_count.min(256).max(1) as u16;
            let window = observe_fixture_window(&descriptor, 1, None, limit, logical_count, limit)?;
            Ok((logical_count, window.children.len(), window.examined))
        })
        .collect::<Result<Vec<_>, HierarchyHistoryError>>()?;
    Ok(HierarchyHistoryProofEvidence {
        schema_version: 1,
        system_ids: vec![
            "lazy-universe-hierarchy".into(),
            "world-history-ledger".into(),
        ],
        proof_id: "bounded-sparse-reference".into(),
        fixture_id: "hierarchy-history-v1/core".into(),
        contract_version: format!("hierarchy-history-v{CONTRACT_VERSION}"),
        descriptor_fingerprint: hex(&descriptor.fingerprint()?),
        baseline_key: hex(&baseline.key()?),
        measured_window_sizes,
        measurement_classification: "simulated".into(),
        capabilities: Vec::new(),
        limitations: vec![
            "Reference fixture, not a production generator or storage engine.".into(),
            "Cross-target transactions, reparenting, splitting, and merging are unsupported."
                .into(),
            "Evidence grants no approval, promotion, execution, spending, or publishing authority."
                .into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AppendOutcome, ChildCursor, DeltaEnvelope, HistoryStream, MaterializationReceipt,
        MigrationReceipt, ReferenceOperation, ReferenceState, ResidencyState, Snapshot,
        reference_operation_schema,
    };
    use std::collections::BTreeMap;

    fn descriptor(marker: u8, origin: DescriptorOrigin) -> HierarchyDescriptor {
        HierarchyDescriptor::new(
            [marker; 32],
            None,
            [marker.wrapping_add(1); 32],
            [marker.wrapping_add(2); 32],
            [marker.wrapping_add(3); 32],
            origin,
            vec![marker, 0, 1],
        )
        .unwrap()
    }

    fn baseline(descriptor: &HierarchyDescriptor, version: u8) -> BaselineManifest {
        BaselineManifest::new(
            descriptor.logical_id,
            descriptor.fingerprint().unwrap(),
            vec![
                DependencyRef {
                    kind: 1,
                    fingerprint: [version; 32],
                },
                DependencyRef {
                    kind: 2,
                    fingerprint: descriptor.world_conditions_fingerprint,
                },
            ],
        )
        .unwrap()
    }

    fn event(
        stream: &HistoryStream,
        sequence: u64,
        command: u8,
        operation: ReferenceOperation,
    ) -> DeltaEnvelope {
        DeltaEnvelope::new(
            stream.baseline_key(),
            stream.baseline().logical_id,
            sequence,
            stream.head(),
            [command; 32],
            reference_operation_schema(),
            operation.encode_canonical().unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn descriptors_are_strict_stable_and_origin_separated() {
        let procedural = descriptor(7, DescriptorOrigin::Procedural);
        let bytes = procedural.encode_canonical().unwrap();
        assert_eq!(
            HierarchyDescriptor::decode_strict(&bytes).unwrap(),
            procedural
        );
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(HierarchyDescriptor::decode_strict(&trailing).is_err());
        let dynamic = descriptor(7, DescriptorOrigin::Dynamic);
        assert_ne!(
            procedural.fingerprint().unwrap(),
            dynamic.fingerprint().unwrap()
        );
    }

    #[test]
    fn child_windows_are_stable_bounded_and_cursor_checked() {
        let parent = descriptor(11, DescriptorOrigin::Procedural);
        let first = observe_fixture_window(&parent, 3, None, 2, 5, 2).unwrap();
        assert_eq!(
            (first.children.len(), first.examined, first.has_more),
            (2, 2, true)
        );
        let second = observe_fixture_window(&parent, 3, first.next_cursor, 3, 5, 3).unwrap();
        assert_eq!(
            second.children.iter().map(|c| c.index).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
        assert!(!second.has_more);
        assert_eq!(
            observe_fixture_window(&parent, 3, None, 2, 5, 1),
            Err(HierarchyHistoryError::Cancelled)
        );
        let stale = ChildCursor {
            parent_descriptor: [0; 32],
            child_kind: 3,
            next_index: 0,
        };
        assert_eq!(
            observe_fixture_window(&parent, 3, Some(stale), 1, 5, 1),
            Err(HierarchyHistoryError::StaleCursor)
        );
        assert!(observe_fixture_window(&parent, 3, None, 257, 1000, 257).is_err());
        let repeat = observe_fixture_window(&parent, 3, None, 2, 5, 2).unwrap();
        assert_eq!(first.children, repeat.children);
    }

    #[test]
    fn empty_and_boundary_windows_do_not_eagerly_scan() {
        let parent = descriptor(12, DescriptorOrigin::Procedural);
        let empty = observe_fixture_window(&parent, 1, None, 1, 0, 0).unwrap();
        assert!(empty.children.is_empty());
        let maximum = observe_fixture_window(&parent, 1, None, 256, 10_000_000, 256).unwrap();
        assert_eq!((maximum.children.len(), maximum.examined), (256, 256));
    }

    #[test]
    fn residency_never_changes_canonical_identity() {
        let item = descriptor(13, DescriptorOrigin::Procedural);
        let receipts = [
            ResidencyState::Cold,
            ResidencyState::Warm,
            ResidencyState::Evicted,
        ]
        .map(|state| MaterializationReceipt::for_descriptor(&item, state, 5).unwrap());
        assert!(
            receipts
                .windows(2)
                .all(|pair| pair[0].descriptor_fingerprint == pair[1].descriptor_fingerprint)
        );
    }

    #[test]
    fn baseline_coexistence_allows_quality_improvement_without_rewriting_old_worlds() {
        let item = descriptor(14, DescriptorOrigin::Procedural);
        let old = baseline(&item, 1);
        let new = baseline(&item, 2);
        assert_ne!(old.key().unwrap(), new.key().unwrap());
        assert_eq!(
            BaselineManifest::decode_strict(&old.encode_canonical().unwrap()).unwrap(),
            old
        );
        assert_eq!(
            BaselineManifest::decode_strict(&new.encode_canonical().unwrap()).unwrap(),
            new
        );
        assert!(
            BaselineManifest::new(
                item.logical_id,
                item.fingerprint().unwrap(),
                vec![
                    DependencyRef {
                        kind: 2,
                        fingerprint: [2; 32]
                    },
                    DependencyRef {
                        kind: 1,
                        fingerprint: [1; 32]
                    },
                ]
            )
            .is_err()
        );
    }

    #[test]
    fn sparse_history_is_ordered_idempotent_and_conflict_checked() {
        let item = descriptor(20, DescriptorOrigin::Procedural);
        let mut stream = HistoryStream::new(baseline(&item, 1)).unwrap();
        let one = event(&stream, 1, 1, ReferenceOperation::Set { key: 8, value: 42 });
        assert!(matches!(
            stream.append(one.clone()).unwrap(),
            AppendOutcome::Appended(_)
        ));
        assert!(matches!(
            stream.append(one).unwrap(),
            AppendOutcome::Idempotent(_)
        ));
        let conflicting_command = DeltaEnvelope::new(
            stream.baseline_key(),
            item.logical_id,
            2,
            stream.head(),
            [1; 32],
            reference_operation_schema(),
            ReferenceOperation::Remove { key: 8 }
                .encode_canonical()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            stream.append(conflicting_command),
            Err(HierarchyHistoryError::CommandConflict)
        );
        let gap = event(&stream, 3, 3, ReferenceOperation::Remove { key: 8 });
        assert_eq!(stream.append(gap), Err(HierarchyHistoryError::Gap));
        let mut fork = event(&stream, 2, 2, ReferenceOperation::Remove { key: 8 });
        fork.expected_parent = Some([9; 32]);
        assert_eq!(
            stream.append(fork),
            Err(HierarchyHistoryError::ForkConflict)
        );
        let two = event(&stream, 2, 2, ReferenceOperation::Remove { key: 8 });
        stream.append(two).unwrap();
        assert_eq!(
            stream.replay_reference().unwrap(),
            ReferenceState::default()
        );
    }

    #[test]
    fn wrong_scope_unknown_schema_and_cross_target_fail_closed() {
        let item = descriptor(21, DescriptorOrigin::Procedural);
        let mut stream = HistoryStream::new(baseline(&item, 1)).unwrap();
        let mut wrong = event(&stream, 1, 1, ReferenceOperation::Set { key: 1, value: 1 });
        wrong.target_logical_id = [99; 32];
        assert_eq!(
            stream.append(wrong),
            Err(HierarchyHistoryError::WrongTarget)
        );
        let mut unknown = event(&stream, 1, 2, ReferenceOperation::Set { key: 1, value: 1 });
        unknown.operation_schema = [88; 32];
        stream.append(unknown).unwrap();
        assert_eq!(
            stream.replay_reference(),
            Err(HierarchyHistoryError::UnknownOperationSchema)
        );

        let mut cross_stream = HistoryStream::new(baseline(&item, 1)).unwrap();
        let cross = event(
            &cross_stream,
            1,
            3,
            ReferenceOperation::CrossTarget {
                other_target: [3; 32],
            },
        );
        cross_stream.append(cross).unwrap();
        assert_eq!(
            cross_stream.replay_reference(),
            Err(HierarchyHistoryError::UnsupportedCrossTarget)
        );
    }

    #[test]
    fn delta_corruption_and_truncation_are_rejected() {
        let item = descriptor(22, DescriptorOrigin::Procedural);
        let stream = HistoryStream::new(baseline(&item, 1)).unwrap();
        let delta = event(&stream, 1, 1, ReferenceOperation::Set { key: 1, value: -5 });
        let bytes = delta.encode_canonical().unwrap();
        assert_eq!(DeltaEnvelope::decode_strict(&bytes).unwrap(), delta);
        assert!(DeltaEnvelope::decode_strict(&bytes[..bytes.len() - 1]).is_err());
        let mut corrupt = bytes;
        let index = corrupt.len() / 2;
        corrupt[index] ^= 1;
        assert!(DeltaEnvelope::decode_strict(&corrupt).is_err());
    }

    #[test]
    fn snapshots_are_verified_against_replay_not_trusted() {
        let item = descriptor(23, DescriptorOrigin::Procedural);
        let mut stream = HistoryStream::new(baseline(&item, 1)).unwrap();
        let one = event(&stream, 1, 1, ReferenceOperation::Set { key: 5, value: 9 });
        stream.append(one).unwrap();
        let mut expected = ReferenceState {
            values: BTreeMap::new(),
        };
        expected.values.insert(5, 9);
        let snapshot = Snapshot::build_reference(&stream, [7; 32]).unwrap();
        snapshot.verify_reference(&stream, &expected).unwrap();
        let mut poisoned = snapshot.clone();
        poisoned.state_bytes.push(0);
        assert_eq!(
            poisoned.verify_reference(&stream, &expected),
            Err(HierarchyHistoryError::SnapshotMismatch)
        );
        expected.values.insert(6, 10);
        assert_eq!(
            snapshot.verify_reference(&stream, &expected),
            Err(HierarchyHistoryError::SnapshotMismatch)
        );
    }

    #[test]
    fn migration_is_explicit_and_preserves_source() {
        let item = descriptor(24, DescriptorOrigin::Procedural);
        let mut source = HistoryStream::new(baseline(&item, 1)).unwrap();
        let one = event(&source, 1, 1, ReferenceOperation::Set { key: 1, value: 2 });
        source.append(one).unwrap();
        let original_head = source.head();
        let target = baseline(&item, 2);
        let receipt = MigrationReceipt::identity_reference(&source, &target, [4; 32]).unwrap();
        assert_eq!(
            (receipt.source_head, source.head()),
            (original_head, original_head)
        );
        assert_eq!(receipt.input_state_hash, receipt.output_state_hash);
        assert_eq!(
            MigrationReceipt::identity_reference(&source, source.baseline(), [4; 32]),
            Err(HierarchyHistoryError::UnsupportedMigration)
        );
    }

    #[test]
    fn proof_is_bounded_and_authority_negative() {
        let evidence = reference_proof_evidence().unwrap();
        assert_eq!(
            evidence.measured_window_sizes,
            vec![(0, 0, 0), (16, 16, 16), (256, 256, 256), (1024, 256, 256)]
        );
        assert!(evidence.capabilities.is_empty());
        let value = serde_json::to_value(evidence).unwrap();
        for forbidden in [
            "approve",
            "promote",
            "execute",
            "publish",
            "spend",
            "credential",
        ] {
            assert!(value.get(forbidden).is_none());
        }
    }
}
