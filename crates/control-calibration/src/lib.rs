//! Capability-free H4 calibration of the three Forge-owned structural controls.

use std::{collections::BTreeSet, fmt};

use humanoid_generation::reference_receipt as h3_receipt;
use reference_viewport::{
    NegativeControlKind, ProjectedView, ViewKind, ViewportSnapshot, negative_control_snapshots,
    reference_snapshot,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SCHEMA_VERSION: u16 = 1;
const RECEIPT_DOMAIN: &[u8] = b"mindwarp.control-calibration.receipt.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalibrationError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
}

impl fmt::Display for CalibrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for CalibrationError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricKind {
    EdgeDeficit,
    RestFrontSpanLoss,
    FrameOneHandVerticalDisplacement,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ControlMetricSample {
    pub control: NegativeControlKind,
    pub snapshot_fingerprint: String,
    pub edge_deficit: u16,
    pub rest_front_span_loss: i32,
    pub frame_one_hand_vertical_displacement: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetricDiscrimination {
    pub metric: MetricKind,
    pub detects: NegativeControlKind,
    pub does_not_detect: Vec<NegativeControlKind>,
    pub reference_value: i32,
    pub control_delta: i32,
    pub decision_rule: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibrationReceipt {
    pub schema_version: u16,
    pub calibration_id: String,
    pub h3_candidate_fingerprint: String,
    pub reference_snapshot_fingerprint: String,
    pub samples: Vec<ControlMetricSample>,
    pub discrimination: Vec<MetricDiscrimination>,
    pub limitations: Vec<String>,
}

impl CalibrationReceipt {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CalibrationError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CalibrationError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| CalibrationError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(CalibrationError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<String, CalibrationError> {
        Ok(hex(&hash(RECEIPT_DOMAIN, &self.to_bytes()?)))
    }
}

pub fn reference_calibration() -> Result<CalibrationReceipt, CalibrationError> {
    let reference = reference_snapshot()
        .map_err(|_| CalibrationError::Invalid("reference snapshot unavailable"))?;
    let h3 = h3_receipt().map_err(|_| CalibrationError::Invalid("H3 receipt unavailable"))?;
    let controls = negative_control_snapshots()
        .map_err(|_| CalibrationError::Invalid("control snapshots unavailable"))?;
    let mut samples = Vec::new();
    for controlled in controls {
        samples.push(sample(
            &reference,
            controlled.control,
            &controlled.snapshot,
        )?);
    }
    samples.sort_by_key(|item| item.control);
    let receipt = CalibrationReceipt {
        schema_version: SCHEMA_VERSION,
        calibration_id: "h4-orthogonal-structural-controls-v1".into(),
        h3_candidate_fingerprint: h3.candidate_fingerprint,
        reference_snapshot_fingerprint: reference.scene_fingerprint,
        samples,
        discrimination: required_discrimination(),
        limitations: required_limitations(),
    };
    validate_calibration(&receipt)?;
    Ok(receipt)
}

pub fn validate_calibration(receipt: &CalibrationReceipt) -> Result<(), CalibrationError> {
    let expected = expected_receipt_without_validation()?;
    if receipt != &expected {
        return Err(CalibrationError::Invalid(
            "calibration drifted, became cross-sensitive, or exceeded its claims",
        ));
    }
    let controls: BTreeSet<_> = receipt.samples.iter().map(|item| item.control).collect();
    if controls
        != BTreeSet::from([
            NegativeControlKind::BrokenConnection,
            NegativeControlKind::SilhouetteCollapse,
            NegativeControlKind::ArticulationDrift,
        ])
    {
        return Err(CalibrationError::Invalid("control set is incomplete"));
    }
    Ok(())
}

fn expected_receipt_without_validation() -> Result<CalibrationReceipt, CalibrationError> {
    let reference = reference_snapshot()
        .map_err(|_| CalibrationError::Invalid("reference snapshot unavailable"))?;
    let h3 = h3_receipt().map_err(|_| CalibrationError::Invalid("H3 receipt unavailable"))?;
    let controls = negative_control_snapshots()
        .map_err(|_| CalibrationError::Invalid("control snapshots unavailable"))?;
    let mut samples: Vec<_> = controls
        .into_iter()
        .map(|controlled| sample(&reference, controlled.control, &controlled.snapshot))
        .collect::<Result<_, _>>()?;
    samples.sort_by_key(|item| item.control);
    Ok(CalibrationReceipt {
        schema_version: SCHEMA_VERSION,
        calibration_id: "h4-orthogonal-structural-controls-v1".into(),
        h3_candidate_fingerprint: h3.candidate_fingerprint,
        reference_snapshot_fingerprint: reference.scene_fingerprint,
        samples,
        discrimination: required_discrimination(),
        limitations: required_limitations(),
    })
}

fn sample(
    reference: &ViewportSnapshot,
    control: NegativeControlKind,
    snapshot: &ViewportSnapshot,
) -> Result<ControlMetricSample, CalibrationError> {
    let reference_span = front_span(reference, 0)?;
    let control_span = front_span(snapshot, 0)?;
    Ok(ControlMetricSample {
        control,
        snapshot_fingerprint: snapshot.scene_fingerprint.clone(),
        edge_deficit: reference
            .edges
            .len()
            .checked_sub(snapshot.edges.len())
            .ok_or(CalibrationError::Invalid("control added undeclared links"))?
            as u16,
        rest_front_span_loss: reference_span - control_span,
        frame_one_hand_vertical_displacement: hand_vertical_displacement(reference, snapshot)?,
    })
}

fn front_span(snapshot: &ViewportSnapshot, frame: usize) -> Result<i32, CalibrationError> {
    let view = snapshot
        .frames
        .get(frame)
        .and_then(|item| item.views.iter().find(|view| view.view == ViewKind::Front))
        .ok_or(CalibrationError::Invalid("front view is missing"))?;
    let minimum = view
        .points
        .iter()
        .map(|point| point.x)
        .min()
        .ok_or(CalibrationError::Invalid("front view is empty"))?;
    let maximum = view.points.iter().map(|point| point.x).max().unwrap();
    Ok(maximum - minimum)
}

fn hand_vertical_displacement(
    reference: &ViewportSnapshot,
    control: &ViewportSnapshot,
) -> Result<i32, CalibrationError> {
    let reference =
        front_view(reference, 1).ok_or(CalibrationError::Invalid("reference pose missing"))?;
    let control =
        front_view(control, 1).ok_or(CalibrationError::Invalid("control pose missing"))?;
    let mut total = 0;
    for id in ["hand_left", "hand_right"] {
        let expected = reference
            .points
            .iter()
            .find(|point| point.id == id)
            .ok_or(CalibrationError::Invalid("reference hand missing"))?;
        let actual = control
            .points
            .iter()
            .find(|point| point.id == id)
            .ok_or(CalibrationError::Invalid("control hand missing"))?;
        total += (actual.y - expected.y).abs();
    }
    Ok(total)
}

fn front_view(snapshot: &ViewportSnapshot, frame: usize) -> Option<&ProjectedView> {
    snapshot
        .frames
        .get(frame)
        .and_then(|item| item.views.iter().find(|view| view.view == ViewKind::Front))
}

fn required_discrimination() -> Vec<MetricDiscrimination> {
    vec![
        MetricDiscrimination {
            metric: MetricKind::EdgeDeficit,
            detects: NegativeControlKind::BrokenConnection,
            does_not_detect: vec![
                NegativeControlKind::SilhouetteCollapse,
                NegativeControlKind::ArticulationDrift,
            ],
            reference_value: 16,
            control_delta: 1,
            decision_rule: "exact_integer_difference_from_bound_reference".into(),
        },
        MetricDiscrimination {
            metric: MetricKind::RestFrontSpanLoss,
            detects: NegativeControlKind::SilhouetteCollapse,
            does_not_detect: vec![
                NegativeControlKind::BrokenConnection,
                NegativeControlKind::ArticulationDrift,
            ],
            reference_value: 600,
            control_delta: 480,
            decision_rule: "exact_integer_difference_from_bound_reference".into(),
        },
        MetricDiscrimination {
            metric: MetricKind::FrameOneHandVerticalDisplacement,
            detects: NegativeControlKind::ArticulationDrift,
            does_not_detect: vec![
                NegativeControlKind::BrokenConnection,
                NegativeControlKind::SilhouetteCollapse,
            ],
            reference_value: 0,
            control_delta: 480,
            decision_rule: "exact_integer_difference_from_bound_reference".into(),
        },
    ]
}

fn required_limitations() -> Vec<String> {
    vec![
        "Metrics distinguish only the three exact bound synthetic controls.".into(),
        "No learned, perceptual, anatomical, aesthetic, or production-quality threshold is defined."
            .into(),
        "Non-detection outside the named metric-control pairs is recorded, not treated as general validity."
            .into(),
        "Calibration grants no owner judgement, approval, promotion, engine, or protected-Kernel authority."
            .into(),
    ]
}

fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, CalibrationError> {
    serde_json::to_vec(value).map_err(|error| CalibrationError::Codec(error.to_string()))
}

fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_controls_are_orthogonal_and_deterministic() {
        let first = reference_calibration().unwrap();
        let second = reference_calibration().unwrap();
        assert_eq!(first, second);
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            first.fingerprint().unwrap(),
            "774a790aa963bb7ed329394d869fda4f5530697cce4d4d029a23d31e6d575f4d"
        );
        assert_eq!(first.samples.len(), 3);
        assert_eq!(first.samples[0].edge_deficit, 1);
        assert_eq!(first.samples[1].rest_front_span_loss, 480);
        assert_eq!(first.samples[2].frame_one_hand_vertical_displacement, 480);
    }

    #[test]
    fn missing_control_fails_closed() {
        let mut receipt = reference_calibration().unwrap();
        receipt.samples.pop();
        assert!(validate_calibration(&receipt).is_err());
    }

    #[test]
    fn cross_sensitive_or_posthoc_metric_claim_fails_closed() {
        let mut receipt = reference_calibration().unwrap();
        receipt.discrimination[0].does_not_detect.pop();
        assert!(validate_calibration(&receipt).is_err());
        let mut receipt = reference_calibration().unwrap();
        receipt.discrimination[0].decision_rule = "score_above_0.8".into();
        assert!(validate_calibration(&receipt).is_err());
    }

    #[test]
    fn stale_h3_or_snapshot_binding_fails_closed() {
        let mut receipt = reference_calibration().unwrap();
        receipt.h3_candidate_fingerprint.replace_range(..1, "0");
        assert!(validate_calibration(&receipt).is_err());
        let mut receipt = reference_calibration().unwrap();
        receipt.samples[0]
            .snapshot_fingerprint
            .replace_range(..1, "0");
        assert!(validate_calibration(&receipt).is_err());
    }

    #[test]
    fn strict_codec_rejects_unknown_and_noncanonical_data() {
        let receipt = reference_calibration().unwrap();
        let mut value: serde_json::Value =
            serde_json::from_slice(&receipt.to_bytes().unwrap()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("visual_quality_score".into(), 1.into());
        assert!(CalibrationReceipt::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        assert_eq!(
            CalibrationReceipt::from_bytes(&serde_json::to_vec_pretty(&receipt).unwrap()),
            Err(CalibrationError::NonCanonical)
        );
    }
}
