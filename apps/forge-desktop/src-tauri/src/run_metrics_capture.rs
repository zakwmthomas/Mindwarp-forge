//! Local, metadata-only intake for measured Forge run receipts.

use std::{fs, path::Path};

use forge_kernel::{contracts::BatchEventRecord, persistence::PersistentForge};

const MAX_RECEIPT_BYTES: u64 = 256 * 1024;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RunMetricsScanReport {
    pub ingested: usize,
    pub rejected: usize,
    pub last_error: Option<String>,
}

pub fn scan_inbox(forge: &PersistentForge, project_root: &Path) -> RunMetricsScanReport {
    let inbox = project_root
        .join(".local")
        .join("forge-metrics")
        .join("inbox");
    let mut report = RunMetricsScanReport::default();
    let Ok(entries) = fs::read_dir(&inbox) else {
        return report;
    };
    let mut paths = entries
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        let result = (|| -> Result<(), String> {
            let metadata = fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
            if !metadata.is_file()
                || metadata.file_type().is_symlink()
                || metadata.len() > MAX_RECEIPT_BYTES
                || path.extension().is_none_or(|extension| extension != "json")
            {
                return Err("run metric receipt is not a bounded regular JSON file".into());
            }
            let bytes = fs::read(&path).map_err(|error| error.to_string())?;
            let event: BatchEventRecord =
                serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
            if event.schema_version != 2
                || event.sequence != 0
                || event.privacy_class != "metadata_only"
                || event.cardinality_class != "bounded"
                || !matches!(
                    event.event_type.as_str(),
                    "routine_run_completed"
                        | "metric_observed"
                        | "criterion_completed"
                        | "criterion_verified"
                )
            {
                return Err("run metric receipt violates the bounded v2 intake contract".into());
            }
            forge
                .record_batch_event_auto_sequence(&event)
                .map_err(|error| format!("{error:?}"))?;
            fs::remove_file(&path).map_err(|error| error.to_string())?;
            Ok(())
        })();
        match result {
            Ok(()) => report.ingested += 1,
            Err(error) => {
                report.rejected += 1;
                report.last_error = Some(error);
            }
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_kernel::contracts::{BatchEventRecord, MetricDimension};

    #[test]
    fn bounded_receipt_is_ingested_idempotently_without_authority_effect() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("forge-run-metrics-{}-{nonce}", std::process::id()));
        let inbox = root.join(".local/forge-metrics/inbox");
        fs::create_dir_all(&inbox).unwrap();
        let forge = PersistentForge::in_memory().unwrap();
        let event = BatchEventRecord {
            schema_version: 2,
            id: "run-receipt-1".into(),
            sequence: 0,
            trace_id: "run-1".into(),
            parent_event_id: None,
            event_type: "routine_run_completed".into(),
            started_at_ms: 10,
            ended_at_ms: 20,
            route_system: "forge-dashboard".into(),
            route_group: "B4".into(),
            route_contract: "batch-event-v2".into(),
            work_package_id: "G1-FORGE-METRICS-DASHBOARD-V1".into(),
            batch_id: "batch-1".into(),
            outcome: "passed".into(),
            evidence_ids: vec!["run-definition:fixture".into()],
            privacy_class: "metadata_only".into(),
            cardinality_class: "bounded".into(),
            metric_name: Some("wall_duration_ms".into()),
            metric_value: Some(10),
            metric_unit: Some("ms".into()),
            metric_dimensions: vec![MetricDimension {
                name: "run_definition".into(),
                value: "fixture-v1".into(),
            }],
        };
        fs::write(
            inbox.join("receipt.json"),
            serde_json::to_vec_pretty(&event).unwrap(),
        )
        .unwrap();
        let report = scan_inbox(&forge, &root);
        assert_eq!(report.ingested, 1);
        assert_eq!(forge.batch_events(10).unwrap().len(), 1);
        assert_eq!(forge.kernel().candidate_count(), 0);
        fs::remove_dir_all(root).unwrap();
    }
}
