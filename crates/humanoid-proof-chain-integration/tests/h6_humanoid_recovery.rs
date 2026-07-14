use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

use forge_kernel::{
    contracts::{NamedVersion, ProofMeasurement, ProofReceiptRecord},
    persistence::{PersistentForge, canonical_proof_receipt_id},
};
use humanoid_proof_chain::{reference_h5_decision, reference_manifest};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn exact_h1_h5_chain_and_receipt_survive_backup_reopen() {
    let suffix = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "mindwarp-h6-humanoid-recovery-{}-{suffix}",
        std::process::id()
    ));
    fs::create_dir(&directory).unwrap();
    let database = directory.join("forge.sqlite3");
    let backup = directory.join("backup.sqlite3");

    let manifest = reference_manifest().unwrap();
    let manifest_bytes = manifest.to_bytes().unwrap();
    let h5 = reference_h5_decision().unwrap();
    let h5_bytes = h5.to_bytes().unwrap();
    let stage_bytes: Vec<_> = manifest
        .content
        .stages
        .iter()
        .map(|stage| serde_json::to_vec(stage).unwrap())
        .collect();

    let mut forge = PersistentForge::open(&database).unwrap();
    let mut input_refs: Vec<_> = stage_bytes
        .iter()
        .map(|bytes| forge.kernel_mut().put_object(bytes))
        .collect();
    input_refs.push(forge.kernel_mut().put_object(&h5_bytes));
    let manifest_ref = forge.kernel_mut().put_object(&manifest_bytes);
    forge.commit().unwrap();

    let mut proof = ProofReceiptRecord {
        schema_version: 1,
        receipt_id: String::new(),
        system_id: "asset-factory".into(),
        proof_id: "h6-h1-h5-proof-chain-recovery".into(),
        status: "pass".into(),
        failure_classification: None,
        input_refs: input_refs.clone(),
        fixture_id: "h6-neutral-humanoid-proof-chain-v1".into(),
        generator_versions: vec![NamedVersion {
            name: "humanoid-proof-chain".into(),
            version: "0.1.0".into(),
        }],
        contract_versions: vec![NamedVersion {
            name: "h6-humanoid-reproduction-recovery".into(),
            version: "1".into(),
        }],
        output_refs: vec![manifest_ref.clone()],
        equivalence_method: "strict-canonical-json-and-domain-separated-sha256".into(),
        measurements: vec![
            ProofMeasurement {
                name: "stage_count".into(),
                value: "5".into(),
                unit: "stages".into(),
                method: "exact_manifest_count".into(),
                classification: "simulated".into(),
            },
            ProofMeasurement {
                name: "hostile_case_count".into(),
                value: "9".into(),
                unit: "cases".into(),
                method: "table_driven_strict_decode".into(),
                classification: "simulated".into(),
            },
        ],
        warnings: vec![],
        limitations: vec![
            "Engine-neutral proof continuity only; no asset, rig, runtime, graphics, or device-cost proof."
                .into(),
            "Receipt and recovery grant no approval, promotion, import, execution, or protected-Kernel authority."
                .into(),
        ],
        created_at: "2026-07-14T00:00:00Z".into(),
        runner_identity: "forge-kernel-h6-test".into(),
    };
    proof.receipt_id = canonical_proof_receipt_id(&proof).unwrap();
    forge.record_proof_receipt(&proof).unwrap();

    let backup_receipt = forge.backup_to(&backup).unwrap();
    assert_eq!(backup_receipt.object_count, 7);
    PersistentForge::verify_backup(&backup_receipt).unwrap();
    drop(forge);

    for path in [&database, &backup] {
        let reopened = PersistentForge::open(path).unwrap();
        assert_eq!(reopened.kernel().object_count(), 7);
        assert_eq!(
            reopened.kernel().object(&manifest_ref).unwrap().bytes,
            manifest_bytes
        );
        for (object_id, expected_bytes) in input_refs
            .iter()
            .zip(stage_bytes.iter().chain(std::iter::once(&h5_bytes)))
        {
            assert_eq!(
                reopened.kernel().object(object_id).unwrap().bytes,
                *expected_bytes
            );
        }
        assert_eq!(
            reopened.proof_receipt_projection(1).unwrap().receipts,
            vec![proof.clone()]
        );
        assert!(reopened.proof_receipt_projection(1).unwrap().read_only);
        drop(reopened);
    }

    fs::remove_file(backup).unwrap();
    fs::remove_file(database).unwrap();
    fs::remove_dir(directory).unwrap();
}
