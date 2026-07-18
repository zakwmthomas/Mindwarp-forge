//! Exact-ray strict-interior receiver arrival over validated optical lineage evidence.
//! Geometry only: no source magnitude, detector, visibility, runtime, or authority.

use core::cmp::Ordering;
use fixed_interval_arithmetic::Signed512;
use optical_lineage_binding::{
    OpticalLaneManifestV1, OpticalLineageBundleInputV1, OpticalLineageDispositionV1,
    OpticalLineageTerminalV1, validate_optical_lane_manifest,
};
use physical_path_substrate::{
    ConditionalIntervalCellStepOutcomeV1, CoordinateFrameV1, Id, SignedDecimalIntervalV1,
    compile_physical_volume, compile_physical_volume_recipe,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_RECEIVER_INPUT_BYTES: usize = 18 * 1024 * 1024;
pub const MAX_RECEIVER_OUTPUT_BYTES: usize = 256 * 1024;
pub const MAX_VALIDATION_LIVE_CANONICAL_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_RECEIVER_STEPS: usize = 64;
pub const PARAMETER_FRACTIONAL_BITS: u16 = 160;
pub const MAXIMUM_LIVE_BITS: u16 = 414;
pub const MAX_DIRECTED_DIVISIONS: u16 = 384;
pub const MAX_BOUND_COMPARISONS: u16 = 768;
pub const MAX_INTERSECTIONS: u16 = 64;
const RECEIVER_DOMAIN: &[u8] = b"mindwarp.receiver-arrival.aabb.v1";
const RESULT_DOMAIN: &[u8] = b"mindwarp.receiver-arrival.result.v1";
const TRANSCRIPT_DOMAIN: &[u8] = b"mindwarp.receiver-arrival.transcript.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReceiverArrivalError {
    Invalid(&'static str),
    Codec(String),
    Arithmetic,
}
impl core::fmt::Display for ReceiverArrivalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Invalid(m) => f.write_str(m),
            Self::Codec(m) => write!(f, "codec: {m}"),
            Self::Arithmetic => f.write_str("checked receiver arithmetic failed"),
        }
    }
}
impl std::error::Error for ReceiverArrivalError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverAabbV1 {
    pub schema_version: u16,
    pub receiver_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub receiver_revision: u32,
    pub coordinate_frame: CoordinateFrameV1,
    pub minimum_q160: [String; 3],
    pub maximum_q160: [String; 3],
    pub receiver_id: Id,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverArrivalGeometryInputV1 {
    pub schema_version: u16,
    pub bundle: OpticalLineageBundleInputV1,
    pub manifest: OpticalLaneManifestV1,
    pub receiver: ReceiverAabbV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverContactReceiptV1 {
    pub step_ordinal: u8,
    pub parameter_infimum_q160: String,
    pub parameter_supremum_q160: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReceiverArrivalOutcomeV1 {
    ArrivalAtStart {
        step_ordinal: u8,
    },
    CertifiedStrictInteriorArrival {
        step_ordinal: u8,
        parameter_infimum_q160: String,
        parameter_supremum_q160: String,
    },
    UnsupportedConditionalEvidence {
        first_unsupported_ordinal: u8,
    },
    UpstreamTerminalWithoutFace {
        step_ordinal: u8,
        terminal: OpticalLineageTerminalV1,
    },
    NoArrivalBeforeLineageTerminal {
        terminal: OpticalLineageTerminalV1,
    },
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverArrivalArithmeticReceiptV1 {
    pub parameter_fractional_bits: u16,
    pub storage_bits: u16,
    pub maximum_live_bits: u16,
    pub observed_maximum_live_bits: u16,
    pub directed_divisions: u16,
    pub bound_comparisons: u16,
    pub intersections: u16,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverArrivalGeometryV1 {
    pub schema_version: u16,
    pub bundle_sha256: Id,
    pub receiver: ReceiverAabbV1,
    pub lane_id: Id,
    pub lineage_transcript_id: Id,
    pub contacts: Vec<ReceiverContactReceiptV1>,
    pub outcome: ReceiverArrivalOutcomeV1,
    pub arithmetic_receipt: ReceiverArrivalArithmeticReceiptV1,
    pub result_id: Id,
    pub transcript_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl ReceiverAabbV1 {
    pub fn compile(
        receiver_source_id: Id,
        scope_id: Id,
        reconstruction_id: Id,
        receiver_revision: u32,
        minimum_q160: [String; 3],
        maximum_q160: [String; 3],
    ) -> Result<Self, ReceiverArrivalError> {
        let receiver_id = domain_hash(
            RECEIVER_DOMAIN,
            &encode(&(
                CONTRACT_VERSION,
                receiver_source_id,
                scope_id,
                reconstruction_id,
                receiver_revision,
                CoordinateFrameV1::CartesianQ32_32Volume3dV1,
                &minimum_q160,
                &maximum_q160,
            ))?,
        );
        Ok(Self {
            schema_version: CONTRACT_VERSION,
            receiver_source_id,
            scope_id,
            reconstruction_id,
            receiver_revision,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            minimum_q160,
            maximum_q160,
            receiver_id,
        })
    }
}
impl ReceiverArrivalGeometryInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ReceiverArrivalError> {
        let result = compile_receiver_arrival_geometry(self)?;
        let bytes = encode_capped(
            self,
            MAX_RECEIVER_INPUT_BYTES,
            "receiver input byte ceiling",
        )?;
        enforce_live(bytes.len(), result.to_bytes(self)?.len())?;
        Ok(bytes)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ReceiverArrivalError> {
        if bytes.len() > MAX_RECEIVER_INPUT_BYTES {
            return Err(ReceiverArrivalError::Invalid("receiver input byte ceiling"));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(ReceiverArrivalError::Invalid(
                "receiver input canonical drift",
            ));
        }
        Ok(value)
    }
}
impl ReceiverArrivalGeometryV1 {
    pub fn to_bytes(
        &self,
        input: &ReceiverArrivalGeometryInputV1,
    ) -> Result<Vec<u8>, ReceiverArrivalError> {
        validate_receiver_arrival_geometry(input, self)?;
        encode_capped(
            self,
            MAX_RECEIVER_OUTPUT_BYTES,
            "receiver output byte ceiling",
        )
    }
    pub fn from_bytes(
        bytes: &[u8],
        input: &ReceiverArrivalGeometryInputV1,
    ) -> Result<Self, ReceiverArrivalError> {
        if bytes.len() > MAX_RECEIVER_OUTPUT_BYTES {
            return Err(ReceiverArrivalError::Invalid(
                "receiver output byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(input)? != bytes {
            return Err(ReceiverArrivalError::Invalid(
                "receiver output canonical drift",
            ));
        }
        Ok(value)
    }
}
pub fn validate_receiver_arrival_geometry(
    input: &ReceiverArrivalGeometryInputV1,
    value: &ReceiverArrivalGeometryV1,
) -> Result<(), ReceiverArrivalError> {
    if &compile_receiver_arrival_geometry(input)? != value {
        return Err(ReceiverArrivalError::Invalid("receiver result drift"));
    }
    Ok(())
}

pub fn compile_receiver_arrival_geometry(
    input: &ReceiverArrivalGeometryInputV1,
) -> Result<ReceiverArrivalGeometryV1, ReceiverArrivalError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(ReceiverArrivalError::Invalid(
            "receiver input schema version",
        ));
    }
    validate_optical_lane_manifest(&input.bundle, &input.manifest)
        .map_err(|_| ReceiverArrivalError::Invalid("receiver lineage replay"))?;
    if input.bundle.steps.len() > MAX_RECEIVER_STEPS {
        return Err(ReceiverArrivalError::Invalid("receiver step ceiling"));
    }
    let bundle_bytes = input
        .bundle
        .to_bytes()
        .map_err(|_| ReceiverArrivalError::Invalid("receiver bundle replay"))?;
    let manifest_bytes = input
        .manifest
        .to_bytes(&input.bundle)
        .map_err(|_| ReceiverArrivalError::Invalid("receiver manifest replay"))?;
    enforce_live(bundle_bytes.len(), manifest_bytes.len())?;
    let (minimum, maximum) = validate_receiver(input)?;
    let mut work = Work::default();
    let mut contacts = Vec::new();
    let mut outcome = None;
    for (ordinal, evidence) in input.bundle.steps.iter().enumerate() {
        let terminal = match input.manifest.steps[ordinal].disposition {
            OpticalLineageDispositionV1::Terminal { terminal } => Some(terminal),
            _ => None,
        };
        let step = &evidence.bulk_query.interval_cell_step_input;
        let point = match exact_vector(&step.point_q160, 160) {
            Ok(v) => v,
            Err(ReceiverArrivalError::Invalid("conditional evidence")) => {
                outcome = Some(ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
                    first_unsupported_ordinal: ordinal as u8,
                });
                break;
            }
            Err(e) => return Err(e),
        };
        let raw_direction = match exact_vector(&step.direction_q1_62, 62) {
            Ok(v) => v,
            Err(ReceiverArrivalError::Invalid("conditional evidence")) => {
                outcome = Some(ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
                    first_unsupported_ordinal: ordinal as u8,
                });
                break;
            }
            Err(e) => return Err(e),
        };
        if matches!(
            terminal,
            Some(
                OpticalLineageTerminalV1::UnavailableCurrent
                    | OpticalLineageTerminalV1::AmbiguousNextFace
                    | OpticalLineageTerminalV1::NoForwardProgress
            )
        ) {
            outcome = Some(ReceiverArrivalOutcomeV1::UpstreamTerminalWithoutFace {
                step_ordinal: ordinal as u8,
                terminal: terminal.expect("matched"),
            });
            break;
        }
        let certified = match &evidence.bulk_query.interval_cell_step_event.outcome {
            ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. }
            | ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { certified }
            | ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { certified } => certified,
            _ => {
                return Err(ReceiverArrivalError::Invalid(
                    "receiver terminal/event mismatch",
                ));
            }
        };
        let face_time = match exact_interval(&certified.time_q160, 160) {
            Ok(v) => v,
            Err(ReceiverArrivalError::Invalid("conditional evidence")) => {
                outcome = Some(ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
                    first_unsupported_ordinal: ordinal as u8,
                });
                break;
            }
            Err(e) => return Err(e),
        };
        match exact_vector(&certified.point_q160, 160) {
            Ok(_) => {}
            Err(ReceiverArrivalError::Invalid("conditional evidence")) => {
                outcome = Some(ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
                    first_unsupported_ordinal: ordinal as u8,
                });
                break;
            }
            Err(e) => return Err(e),
        }
        if point
            .iter()
            .enumerate()
            .all(|(a, v)| v > &minimum[a] && v < &maximum[a])
        {
            outcome = Some(ReceiverArrivalOutcomeV1::ArrivalAtStart {
                step_ordinal: ordinal as u8,
            });
            break;
        }
        // Lifting Q1.62 to Q160 multiplies the denominator by 2^98. Cancel that
        // common power against the 2^160 numerator before exact cross-products.
        let direction = raw_direction;
        match classify_step(
            &point, &direction, &minimum, &maximum, &face_time, &mut work,
        )? {
            StepClassification::Strict(lower, upper) => {
                outcome = Some(ReceiverArrivalOutcomeV1::CertifiedStrictInteriorArrival {
                    step_ordinal: ordinal as u8,
                    parameter_infimum_q160: project_floor(&lower)?,
                    parameter_supremum_q160: project_ceil(&upper)?,
                });
                break;
            }
            StepClassification::Contact(lower, upper) => contacts.push(ReceiverContactReceiptV1 {
                step_ordinal: ordinal as u8,
                parameter_infimum_q160: project_floor(&lower)?,
                parameter_supremum_q160: project_ceil(&upper)?,
            }),
            StepClassification::None => {}
        }
    }
    let outcome = outcome.unwrap_or(ReceiverArrivalOutcomeV1::NoArrivalBeforeLineageTerminal {
        terminal: input.manifest.final_terminal,
    });
    work.require_caps()?;
    let arithmetic_receipt = ReceiverArrivalArithmeticReceiptV1 {
        parameter_fractional_bits: 160,
        storage_bits: 512,
        maximum_live_bits: 414,
        observed_maximum_live_bits: work.observed,
        directed_divisions: work.divisions,
        bound_comparisons: work.comparisons,
        intersections: work.intersections,
    };
    let limitations = limitations();
    let authority_effect = "none_evidence_only".to_owned();
    let bundle_sha256 = Sha256::digest(&bundle_bytes).into();
    let result_id = domain_hash(
        RESULT_DOMAIN,
        &encode(&(
            bundle_sha256,
            input.receiver.receiver_id,
            input.manifest.lane_id,
            input.manifest.transcript_id,
            &contacts,
            &outcome,
            arithmetic_receipt,
            &limitations,
            &authority_effect,
        ))?,
    );
    let transcript_id = domain_hash(
        TRANSCRIPT_DOMAIN,
        &encode(&(
            result_id,
            input.manifest.transcript_id,
            input.receiver.receiver_id,
            &contacts,
            &outcome,
            arithmetic_receipt,
        ))?,
    );
    Ok(ReceiverArrivalGeometryV1 {
        schema_version: 1,
        bundle_sha256,
        receiver: input.receiver.clone(),
        lane_id: input.manifest.lane_id,
        lineage_transcript_id: input.manifest.transcript_id,
        contacts,
        outcome,
        arithmetic_receipt,
        result_id,
        transcript_id,
        limitations,
        authority_effect,
    })
}

fn validate_receiver(
    input: &ReceiverArrivalGeometryInputV1,
) -> Result<([Signed512; 3], [Signed512; 3]), ReceiverArrivalError> {
    let receiver = &input.receiver;
    let profile = &input.bundle.profile.input;
    if receiver.schema_version != 1
        || receiver.receiver_source_id == [0; 32]
        || receiver.receiver_revision == 0
    {
        return Err(ReceiverArrivalError::Invalid("receiver provenance"));
    }
    if receiver.scope_id != profile.scope_id
        || receiver.reconstruction_id != profile.reconstruction_id
        || receiver.coordinate_frame != CoordinateFrameV1::CartesianQ32_32Volume3dV1
    {
        return Err(ReceiverArrivalError::Invalid("receiver physical binding"));
    }
    let expected = ReceiverAabbV1::compile(
        receiver.receiver_source_id,
        receiver.scope_id,
        receiver.reconstruction_id,
        receiver.receiver_revision,
        receiver.minimum_q160.clone(),
        receiver.maximum_q160.clone(),
    )?;
    if &expected != receiver {
        return Err(ReceiverArrivalError::Invalid("receiver identity drift"));
    }
    let recipe = compile_physical_volume_recipe(&profile.physical_volume_recipe_input)
        .map_err(|_| ReceiverArrivalError::Invalid("receiver recipe replay"))?;
    compile_physical_volume(&recipe)
        .map_err(|_| ReceiverArrivalError::Invalid("receiver volume replay"))?;
    let minimum = parse_vector(&receiver.minimum_q160)?;
    let maximum = parse_vector(&receiver.maximum_q160)?;
    for axis in 0..3 {
        if minimum[axis] >= maximum[axis] {
            return Err(ReceiverArrivalError::Invalid("receiver positive volume"));
        }
        let volume_min =
            checked(Signed512::from_i64(recipe.input.origin_q32_32[axis]).checked_shl(128))?;
        let span =
            i128::from(recipe.input.cell_step_q32_32) * i128::from(recipe.input.extent[axis]);
        let volume_max = checked(
            Signed512::from_i128(i128::from(recipe.input.origin_q32_32[axis]) + span)
                .checked_shl(128),
        )?;
        if minimum[axis] < volume_min || maximum[axis] > volume_max {
            return Err(ReceiverArrivalError::Invalid(
                "receiver outside physical volume",
            ));
        }
    }
    Ok((minimum, maximum))
}

#[derive(Clone)]
struct Ratio {
    numerator: Signed512,
    denominator: Signed512,
}
impl Ratio {
    fn integer(v: Signed512) -> Self {
        Self {
            numerator: v,
            denominator: Signed512::one(),
        }
    }
}
#[derive(Default)]
struct Work {
    observed: u16,
    divisions: u16,
    comparisons: u16,
    intersections: u16,
}
impl Work {
    fn see(&mut self, values: &[&Signed512]) -> Result<(), ReceiverArrivalError> {
        for value in values {
            let bits = value.maximum_magnitude_bits();
            self.observed = self.observed.max(bits);
            if bits > 414 {
                return Err(ReceiverArrivalError::Invalid(
                    "receiver 414-bit arithmetic shield exceeded",
                ));
            }
        }
        Ok(())
    }
    fn compare(&mut self, a: &Ratio, b: &Ratio) -> Result<Ordering, ReceiverArrivalError> {
        let left = checked(a.numerator.checked_mul(&b.denominator))?;
        let right = checked(b.numerator.checked_mul(&a.denominator))?;
        self.see(&[&left, &right])?;
        self.comparisons += 1;
        Ok(left.cmp(&right))
    }
    fn require_caps(&self) -> Result<(), ReceiverArrivalError> {
        if self.divisions > 384 || self.comparisons > 768 || self.intersections > 64 {
            Err(ReceiverArrivalError::Invalid("receiver operation ceiling"))
        } else {
            Ok(())
        }
    }
}
enum StepClassification {
    Strict(Ratio, Ratio),
    Contact(Ratio, Ratio),
    None,
}
fn classify_step(
    point: &[Signed512; 3],
    direction: &[Signed512; 3],
    minimum: &[Signed512; 3],
    maximum: &[Signed512; 3],
    face_time: &Signed512,
    work: &mut Work,
) -> Result<StepClassification, ReceiverArrivalError> {
    work.intersections += 1;
    let mut lower = Ratio::integer(Signed512::zero());
    let mut upper = Ratio::integer(face_time.clone());
    let mut strict = true;
    let mut closed = true;
    for axis in 0..3 {
        if direction[axis] == Signed512::zero() {
            work.comparisons += 2;
            if point[axis] <= minimum[axis] || point[axis] >= maximum[axis] {
                strict = false
            }
            if point[axis] < minimum[axis] || point[axis] > maximum[axis] {
                closed = false
            }
            continue;
        }
        let mut a = boundary_ratio(&minimum[axis], &point[axis], &direction[axis], work)?;
        let mut b = boundary_ratio(&maximum[axis], &point[axis], &direction[axis], work)?;
        if work.compare(&a, &b)? == Ordering::Greater {
            core::mem::swap(&mut a, &mut b)
        }
        if work.compare(&a, &lower)? == Ordering::Greater {
            lower = a
        }
        if work.compare(&b, &upper)? == Ordering::Less {
            upper = b
        }
    }
    if !closed || work.compare(&lower, &upper)? == Ordering::Greater {
        return Ok(StepClassification::None);
    }
    if strict && work.compare(&lower, &upper)? == Ordering::Less {
        Ok(StepClassification::Strict(lower, upper))
    } else {
        Ok(StepClassification::Contact(lower, upper))
    }
}
fn boundary_ratio(
    bound: &Signed512,
    point: &Signed512,
    direction: &Signed512,
    work: &mut Work,
) -> Result<Ratio, ReceiverArrivalError> {
    let delta = checked(bound.checked_sub(point))?;
    let mut numerator = checked(delta.checked_shl(62))?;
    let mut denominator = direction.clone();
    if denominator.is_negative() {
        numerator = numerator.checked_neg();
        denominator = denominator.checked_neg()
    }
    work.see(&[&delta, &numerator, &denominator])?;
    work.divisions += 1;
    Ok(Ratio {
        numerator,
        denominator,
    })
}
fn project_floor(v: &Ratio) -> Result<String, ReceiverArrivalError> {
    Ok(checked(v.numerator.div_floor(&v.denominator))?.canonical_decimal())
}
fn project_ceil(v: &Ratio) -> Result<String, ReceiverArrivalError> {
    Ok(checked(v.numerator.div_ceil(&v.denominator))?.canonical_decimal())
}
fn exact_interval(
    v: &SignedDecimalIntervalV1,
    bits: u16,
) -> Result<Signed512, ReceiverArrivalError> {
    if v.fractional_bits != bits {
        return Err(ReceiverArrivalError::Invalid("receiver interval scale"));
    }
    let a = parse(&v.lower)?;
    let b = parse(&v.upper)?;
    if a != b {
        return Err(ReceiverArrivalError::Invalid("conditional evidence"));
    }
    Ok(a)
}
fn exact_vector(
    v: &[SignedDecimalIntervalV1; 3],
    bits: u16,
) -> Result<[Signed512; 3], ReceiverArrivalError> {
    Ok([
        exact_interval(&v[0], bits)?,
        exact_interval(&v[1], bits)?,
        exact_interval(&v[2], bits)?,
    ])
}
fn parse_vector(v: &[String; 3]) -> Result<[Signed512; 3], ReceiverArrivalError> {
    Ok([parse(&v[0])?, parse(&v[1])?, parse(&v[2])?])
}
fn parse(v: &str) -> Result<Signed512, ReceiverArrivalError> {
    checked(Signed512::from_canonical_decimal(v))
}
fn checked<T>(v: Result<Signed512, T>) -> Result<Signed512, ReceiverArrivalError> {
    let v = v.map_err(|_| ReceiverArrivalError::Arithmetic)?;
    if v.maximum_magnitude_bits() > 414 {
        Err(ReceiverArrivalError::Invalid(
            "receiver 414-bit arithmetic shield exceeded",
        ))
    } else {
        Ok(v)
    }
}
fn limitations() -> Vec<String> {
    vec![
        "exact_degenerate_ray_evidence_only".into(),
        "strict_interior_aabb_arrival_only".into(),
        "contact_is_not_arrival".into(),
        "receiver_face_tie_is_not_current_step_arrival".into(),
        "no_source_magnitude_spreading_detector_visibility_or_perception_claim".into(),
        "bounded_reference_not_runtime_authority".into(),
    ]
}
fn enforce_live(a: usize, b: usize) -> Result<(), ReceiverArrivalError> {
    if a.checked_add(b)
        .ok_or(ReceiverArrivalError::Invalid("receiver live byte overflow"))?
        > 32 * 1024 * 1024
    {
        Err(ReceiverArrivalError::Invalid(
            "receiver live canonical byte ceiling",
        ))
    } else {
        Ok(())
    }
}
fn encode<T: Serialize + ?Sized>(v: &T) -> Result<Vec<u8>, ReceiverArrivalError> {
    serde_json::to_vec(v).map_err(|e| ReceiverArrivalError::Codec(e.to_string()))
}
fn encode_capped<T: Serialize + ?Sized>(
    v: &T,
    cap: usize,
    message: &'static str,
) -> Result<Vec<u8>, ReceiverArrivalError> {
    let b = encode(v)?;
    if b.len() > cap {
        Err(ReceiverArrivalError::Invalid(message))
    } else {
        Ok(b)
    }
}
fn decode<T: DeserializeOwned>(b: &[u8]) -> Result<T, ReceiverArrivalError> {
    let mut s = serde_json::Deserializer::from_slice(b).into_iter::<T>();
    let v = s
        .next()
        .ok_or_else(|| ReceiverArrivalError::Codec("missing JSON value".into()))?
        .map_err(|e| ReceiverArrivalError::Codec(e.to_string()))?;
    if s.byte_offset() != b.len() {
        return Err(ReceiverArrivalError::Invalid("trailing bytes"));
    }
    Ok(v)
}
fn domain_hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut h = Sha256::new();
    h.update(domain);
    h.update(bytes);
    h.finalize().into()
}
