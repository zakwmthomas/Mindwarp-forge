#![deny(warnings)]
//! Capability-free whole-cell receiver-before-face evidence.

use fixed_interval_arithmetic::Signed512;
use optical_phase_space_cell_binding::{
    CorrelatedAffineOutputV1, DirectedFixedIntervalV1, OpticalPhaseSpaceProjectionQueryV1,
    OpticalProjectionTargetV1, PhaseSpaceOutputRoleV1, PositiveRationalV1,
    project_optical_phase_space_cell,
};
use optical_phase_space_transport_certificate::{
    OriginAnchoredTransportCertificateV1, OriginAnchoredTransportInputV1,
    validate_origin_anchored_transport_certificate,
};
use physical_path_substrate::{
    CoordinateFrameV1, IntervalFaceAxisV1, IntervalFaceSideV1, IntervalFaceV1,
    SignedDecimalIntervalV1, build_physical_cell,
};
use receiver_arrival_geometry_binding::ReceiverAabbV1;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, fmt};

pub const CONTRACT_VERSION: u16 = 1;
pub const STORAGE_BITS: u16 = 512;
pub const MAXIMUM_LIVE_BITS: u16 = 391;
pub const MAX_CHECKED_INTEGER_OPERATIONS: u32 = 16_384;
pub const MAX_BOUND_COMPARISONS: u16 = 4_096;
pub const MAX_INPUT_BYTES: usize = 40 * 1024 * 1024;
pub const MAX_RESULT_BYTES: usize = 256 * 1024;
pub const MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 64 * 1024 * 1024;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str = "whole_cell_receiver_before_face_evidence_only_no_partial_fraction_source_power_detector_visibility_runtime_promotion_or_c3_closure";

const INPUT_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.receiver-coupling.input.v1";
const RESULT_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.receiver-coupling.result.v1";
const VARIABLES: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WholeCellReceiverCouplingError {
    InvalidSchema,
    InvalidInput(&'static str),
    Dependency(&'static str),
    ByteCeiling,
    ResourceCeiling,
    ArithmeticShieldExceeded,
    IdentityMismatch,
    CodecDefect,
}
impl fmt::Display for WholeCellReceiverCouplingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for WholeCellReceiverCouplingError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverFaceSideV1 {
    Minimum,
    Maximum,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WholeCellFullProofV1 {
    StartInside,
    ReceiverFace,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnresolvedReceiverCouplingReasonV1 {
    MixedReceiverFaceOrder,
    DirectionSignChange,
    PartialCrossAxisOverlap,
    TangencyOrFaceCoincidence,
    UnsupportedEvidence,
    ArithmeticShield,
    WorkExhausted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WholeCellReceiverCouplingOutcomeV1 {
    CertifiedFullBeforeFace {
        proof: WholeCellFullProofV1,
        receiver_axis: Option<u8>,
        receiver_side: Option<ReceiverFaceSideV1>,
    },
    CertifiedZeroBeforeFace {
        separating_axis: u8,
        receiver_side: ReceiverFaceSideV1,
    },
    UnresolvedReceiverCoupling {
        reason: UnresolvedReceiverCouplingReasonV1,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellReceiverCouplingArithmeticReceiptV1 {
    pub storage_bits: u16,
    pub maximum_live_bits: u16,
    pub observed_maximum_live_bits: u16,
    pub checked_integer_operations: u32,
    pub bound_comparisons: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellReceiverCouplingInputV1 {
    pub schema_version: u16,
    pub transport_input: OriginAnchoredTransportInputV1,
    pub transport_certificate: OriginAnchoredTransportCertificateV1,
    pub selected_step_index: u8,
    pub receiver: ReceiverAabbV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellReceiverCouplingV1 {
    pub schema_version: u16,
    pub input_id: [u8; 32],
    pub cell_id: [u8; 32],
    pub transport_certificate_id: [u8; 32],
    pub selected_step_id: [u8; 32],
    pub receiver_id: [u8; 32],
    pub outcome: WholeCellReceiverCouplingOutcomeV1,
    pub accepted_measure: PositiveRationalV1,
    pub zero_measure: PositiveRationalV1,
    pub unresolved_measure: PositiveRationalV1,
    pub arithmetic_receipt: WholeCellReceiverCouplingArithmeticReceiptV1,
    pub result_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Default)]
struct Work {
    observed: u16,
    operations: u32,
    comparisons: u16,
}
impl Work {
    fn see(&mut self, value: &Signed512) -> Result<(), WholeCellReceiverCouplingError> {
        self.observed = self.observed.max(value.maximum_magnitude_bits());
        if self.observed > MAXIMUM_LIVE_BITS {
            return Err(WholeCellReceiverCouplingError::ArithmeticShieldExceeded);
        }
        Ok(())
    }
    fn operation(&mut self, value: Signed512) -> Result<Signed512, WholeCellReceiverCouplingError> {
        self.operations = self.operations.saturating_add(1);
        if self.operations > MAX_CHECKED_INTEGER_OPERATIONS {
            return Err(WholeCellReceiverCouplingError::ResourceCeiling);
        }
        self.see(&value)?;
        Ok(value)
    }
    fn compare_zero(
        &mut self,
        lower: &Signed512,
    ) -> Result<Ordering, WholeCellReceiverCouplingError> {
        self.comparisons = self.comparisons.saturating_add(1);
        if self.comparisons > MAX_BOUND_COMPARISONS {
            return Err(WholeCellReceiverCouplingError::ResourceCeiling);
        }
        Ok(lower.cmp(&Signed512::zero()))
    }
}

#[derive(Clone)]
struct Affine {
    center: Signed512,
    coefficients: [Signed512; VARIABLES],
    remainder_lower: Signed512,
    remainder_upper: Signed512,
}

#[derive(Clone)]
struct Quadratic {
    constant: Signed512,
    linear: [Signed512; VARIABLES],
    quadratic: [[Signed512; VARIABLES]; VARIABLES],
    remainder_lower: Signed512,
    remainder_upper: Signed512,
}

fn parse(value: &str, work: &mut Work) -> Result<Signed512, WholeCellReceiverCouplingError> {
    let parsed = Signed512::from_canonical_decimal(value)
        .map_err(|_| WholeCellReceiverCouplingError::InvalidInput("noncanonical decimal"))?;
    work.see(&parsed)?;
    Ok(parsed)
}
fn add(
    a: &Signed512,
    b: &Signed512,
    work: &mut Work,
) -> Result<Signed512, WholeCellReceiverCouplingError> {
    work.operation(
        a.checked_add(b)
            .map_err(|_| WholeCellReceiverCouplingError::ArithmeticShieldExceeded)?,
    )
}
fn sub(
    a: &Signed512,
    b: &Signed512,
    work: &mut Work,
) -> Result<Signed512, WholeCellReceiverCouplingError> {
    work.operation(
        a.checked_sub(b)
            .map_err(|_| WholeCellReceiverCouplingError::ArithmeticShieldExceeded)?,
    )
}
fn mul(
    a: &Signed512,
    b: &Signed512,
    work: &mut Work,
) -> Result<Signed512, WholeCellReceiverCouplingError> {
    work.operation(
        a.checked_mul(b)
            .map_err(|_| WholeCellReceiverCouplingError::ArithmeticShieldExceeded)?,
    )
}
fn shl(
    a: &Signed512,
    bits: u16,
    work: &mut Work,
) -> Result<Signed512, WholeCellReceiverCouplingError> {
    work.operation(
        a.checked_shl(bits)
            .map_err(|_| WholeCellReceiverCouplingError::ArithmeticShieldExceeded)?,
    )
}
fn abs(value: &Signed512) -> Signed512 {
    if value.is_negative() {
        value.checked_neg()
    } else {
        value.clone()
    }
}
fn min_value(a: Signed512, b: Signed512) -> Signed512 {
    if a <= b { a } else { b }
}
fn max_value(a: Signed512, b: Signed512) -> Signed512 {
    if a >= b { a } else { b }
}

fn affine_from(
    form: &CorrelatedAffineOutputV1,
    work: &mut Work,
) -> Result<Affine, WholeCellReceiverCouplingError> {
    let mut coefficients = std::array::from_fn(|_| Signed512::zero());
    for (index, coefficient) in coefficients.iter_mut().enumerate() {
        *coefficient = parse(&form.coefficient_numerators[index], work)?;
    }
    Ok(Affine {
        center: parse(&form.center_numerator, work)?,
        coefficients,
        remainder_lower: parse(&form.remainder_lower_numerator, work)?,
        remainder_upper: parse(&form.remainder_upper_numerator, work)?,
    })
}

fn affine_bounds(
    value: &Affine,
    work: &mut Work,
) -> Result<(Signed512, Signed512), WholeCellReceiverCouplingError> {
    let mut radius = Signed512::zero();
    for coefficient in &value.coefficients {
        radius = add(&radius, &abs(coefficient), work)?;
    }
    Ok((
        add(
            &sub(&value.center, &radius, work)?,
            &value.remainder_lower,
            work,
        )?,
        add(
            &add(&value.center, &radius, work)?,
            &value.remainder_upper,
            work,
        )?,
    ))
}

fn affine_scale(
    value: &Affine,
    factor: &Signed512,
    work: &mut Work,
) -> Result<Affine, WholeCellReceiverCouplingError> {
    let a = mul(&value.remainder_lower, factor, work)?;
    let b = mul(&value.remainder_upper, factor, work)?;
    let mut coefficients = std::array::from_fn(|_| Signed512::zero());
    for (index, coefficient) in coefficients.iter_mut().enumerate() {
        *coefficient = mul(&value.coefficients[index], factor, work)?;
    }
    Ok(Affine {
        center: mul(&value.center, factor, work)?,
        coefficients,
        remainder_lower: min_value(a.clone(), b.clone()),
        remainder_upper: max_value(a, b),
    })
}

fn plane_numerator(
    coordinate: &Signed512,
    denominator: &Signed512,
    point: &Affine,
    fractional_bits: u16,
    orientation: i64,
    work: &mut Work,
) -> Result<Affine, WholeCellReceiverCouplingError> {
    let oriented = Signed512::from_i64(orientation);
    let coordinate_d = mul(coordinate, denominator, work)?;
    let point_scaled = affine_scale(point, &shl(&Signed512::one(), fractional_bits, work)?, work)?;
    let raw = Affine {
        center: sub(&coordinate_d, &point_scaled.center, work)?,
        coefficients: std::array::from_fn(|index| point_scaled.coefficients[index].checked_neg()),
        remainder_lower: point_scaled.remainder_upper.checked_neg(),
        remainder_upper: point_scaled.remainder_lower.checked_neg(),
    };
    affine_scale(&raw, &oriented, work)
}

fn interval_mul(
    left: &(Signed512, Signed512),
    right: &(Signed512, Signed512),
    work: &mut Work,
) -> Result<(Signed512, Signed512), WholeCellReceiverCouplingError> {
    let values = [
        mul(&left.0, &right.0, work)?,
        mul(&left.0, &right.1, work)?,
        mul(&left.1, &right.0, work)?,
        mul(&left.1, &right.1, work)?,
    ];
    let mut lower = values[0].clone();
    let mut upper = values[0].clone();
    for value in values.into_iter().skip(1) {
        lower = min_value(lower, value.clone());
        upper = max_value(upper, value);
    }
    Ok((lower, upper))
}

fn affine_core_bounds(
    value: &Affine,
    work: &mut Work,
) -> Result<(Signed512, Signed512), WholeCellReceiverCouplingError> {
    let zero_remainder = Affine {
        center: value.center.clone(),
        coefficients: value.coefficients.clone(),
        remainder_lower: Signed512::zero(),
        remainder_upper: Signed512::zero(),
    };
    affine_bounds(&zero_remainder, work)
}

fn product(
    left: &Affine,
    right: &Affine,
    work: &mut Work,
) -> Result<Quadratic, WholeCellReceiverCouplingError> {
    let constant = mul(&left.center, &right.center, work)?;
    let mut linear: [Signed512; VARIABLES] = std::array::from_fn(|_| Signed512::zero());
    let mut quadratic: [[Signed512; VARIABLES]; VARIABLES] =
        std::array::from_fn(|_| std::array::from_fn(|_| Signed512::zero()));
    for index in 0..VARIABLES {
        linear[index] = add(
            &mul(&left.center, &right.coefficients[index], work)?,
            &mul(&left.coefficients[index], &right.center, work)?,
            work,
        )?;
        for other in 0..VARIABLES {
            quadratic[index][other] =
                mul(&left.coefficients[index], &right.coefficients[other], work)?;
        }
    }
    let left_core = affine_core_bounds(left, work)?;
    let right_core = affine_core_bounds(right, work)?;
    let left_remainder = (left.remainder_lower.clone(), left.remainder_upper.clone());
    let right_remainder = (right.remainder_lower.clone(), right.remainder_upper.clone());
    let ar_b = interval_mul(&left_core, &right_remainder, work)?;
    let br_a = interval_mul(&right_core, &left_remainder, work)?;
    let ra_rb = interval_mul(&left_remainder, &right_remainder, work)?;
    Ok(Quadratic {
        constant,
        linear,
        quadratic,
        remainder_lower: add(&add(&ar_b.0, &br_a.0, work)?, &ra_rb.0, work)?,
        remainder_upper: add(&add(&ar_b.1, &br_a.1, work)?, &ra_rb.1, work)?,
    })
}

fn quadratic_add(
    left: &Quadratic,
    right: &Quadratic,
    work: &mut Work,
) -> Result<Quadratic, WholeCellReceiverCouplingError> {
    let mut value = Quadratic {
        constant: add(&left.constant, &right.constant, work)?,
        linear: std::array::from_fn(|_| Signed512::zero()),
        quadratic: std::array::from_fn(|_| std::array::from_fn(|_| Signed512::zero())),
        remainder_lower: add(&left.remainder_lower, &right.remainder_lower, work)?,
        remainder_upper: add(&left.remainder_upper, &right.remainder_upper, work)?,
    };
    for i in 0..VARIABLES {
        value.linear[i] = add(&left.linear[i], &right.linear[i], work)?;
        for j in 0..VARIABLES {
            value.quadratic[i][j] = add(&left.quadratic[i][j], &right.quadratic[i][j], work)?;
        }
    }
    Ok(value)
}
fn quadratic_neg(value: &Quadratic) -> Quadratic {
    Quadratic {
        constant: value.constant.checked_neg(),
        linear: std::array::from_fn(|i| value.linear[i].checked_neg()),
        quadratic: std::array::from_fn(|i| {
            std::array::from_fn(|j| value.quadratic[i][j].checked_neg())
        }),
        remainder_lower: value.remainder_upper.checked_neg(),
        remainder_upper: value.remainder_lower.checked_neg(),
    }
}
fn quadratic_scale_shift(
    value: &Quadratic,
    bits: u16,
    work: &mut Work,
) -> Result<Quadratic, WholeCellReceiverCouplingError> {
    let scale = shl(&Signed512::one(), bits, work)?;
    let mut result = value.clone();
    result.constant = mul(&result.constant, &scale, work)?;
    for i in 0..VARIABLES {
        result.linear[i] = mul(&result.linear[i], &scale, work)?;
        for j in 0..VARIABLES {
            result.quadratic[i][j] = mul(&result.quadratic[i][j], &scale, work)?;
        }
    }
    result.remainder_lower = mul(&result.remainder_lower, &scale, work)?;
    result.remainder_upper = mul(&result.remainder_upper, &scale, work)?;
    Ok(result)
}
fn quadratic_bounds(
    value: &Quadratic,
    work: &mut Work,
) -> Result<(Signed512, Signed512), WholeCellReceiverCouplingError> {
    let mut lower = add(&value.constant, &value.remainder_lower, work)?;
    let mut upper = add(&value.constant, &value.remainder_upper, work)?;
    for i in 0..VARIABLES {
        let magnitude = abs(&value.linear[i]);
        lower = sub(&lower, &magnitude, work)?;
        upper = add(&upper, &magnitude, work)?;
        for j in 0..VARIABLES {
            let magnitude = abs(&value.quadratic[i][j]);
            lower = sub(&lower, &magnitude, work)?;
            upper = add(&upper, &magnitude, work)?;
        }
    }
    Ok((lower, upper))
}

fn role_index(role: PhaseSpaceOutputRoleV1) -> usize {
    match role {
        PhaseSpaceOutputRoleV1::PointX => 0,
        PhaseSpaceOutputRoleV1::PointY => 1,
        PhaseSpaceOutputRoleV1::PointZ => 2,
        PhaseSpaceOutputRoleV1::DirectionX => 3,
        PhaseSpaceOutputRoleV1::DirectionY => 4,
        PhaseSpaceOutputRoleV1::DirectionZ => 5,
    }
}
fn axis_index(axis: IntervalFaceAxisV1) -> usize {
    match axis {
        IntervalFaceAxisV1::X => 0,
        IntervalFaceAxisV1::Y => 1,
        IntervalFaceAxisV1::Z => 2,
    }
}
fn face_orientation(side: IntervalFaceSideV1) -> i64 {
    match side {
        IntervalFaceSideV1::Minimum => -1,
        IntervalFaceSideV1::Maximum => 1,
    }
}

fn physical_face_height(
    input: &OriginAnchoredTransportInputV1,
    step: usize,
) -> Result<(IntervalFaceV1, i64), WholeCellReceiverCouplingError> {
    let certificate =
        optical_phase_space_transport_certificate::compile_origin_anchored_transport(input)
            .map_err(|_| WholeCellReceiverCouplingError::Dependency("transport replay failed"))?;
    let selected =
        certificate
            .steps
            .get(step)
            .ok_or(WholeCellReceiverCouplingError::InvalidInput(
                "selected step absent",
            ))?;
    let cell = build_physical_cell(
        &input.physical_volume_recipe,
        &input.physical_volume,
        selected.current_cell,
    )
    .map_err(|_| WholeCellReceiverCouplingError::Dependency("physical cell replay failed"))?;
    let axis = axis_index(selected.certified_face.axis);
    let height = match selected.certified_face.side {
        IntervalFaceSideV1::Minimum => cell.min_q32_32[axis],
        IntervalFaceSideV1::Maximum => cell.max_q32_32[axis],
    };
    Ok((selected.certified_face, height))
}

fn q160_vector(
    values: &[String; 3],
    work: &mut Work,
) -> Result<[Signed512; 3], WholeCellReceiverCouplingError> {
    let mut parsed: [Signed512; 3] = std::array::from_fn(|_| Signed512::zero());
    for index in 0..3 {
        parsed[index] = parse(&values[index], work)?;
    }
    Ok(parsed)
}
fn projection_bounds(
    values: &[SignedDecimalIntervalV1; 3],
    work: &mut Work,
) -> Result<[(Signed512, Signed512); 3], WholeCellReceiverCouplingError> {
    let mut result = std::array::from_fn(|_| (Signed512::zero(), Signed512::zero()));
    for axis in 0..3 {
        if values[axis].fractional_bits != 160 {
            return Err(WholeCellReceiverCouplingError::InvalidInput(
                "projection scale",
            ));
        }
        result[axis] = (
            parse(&values[axis].lower, work)?,
            parse(&values[axis].upper, work)?,
        );
    }
    Ok(result)
}
fn directed_projection_bounds(
    values: &[DirectedFixedIntervalV1; 3],
    work: &mut Work,
) -> Result<[(Signed512, Signed512); 3], WholeCellReceiverCouplingError> {
    let mut result = std::array::from_fn(|_| (Signed512::zero(), Signed512::zero()));
    for axis in 0..3 {
        if values[axis].fractional_bits != 160 {
            return Err(WholeCellReceiverCouplingError::InvalidInput(
                "projection scale",
            ));
        }
        result[axis] = (
            parse(&values[axis].lower, work)?,
            parse(&values[axis].upper, work)?,
        );
    }
    Ok(result)
}

fn zero_measure() -> PositiveRationalV1 {
    PositiveRationalV1 {
        numerator: "0".into(),
        denominator: "1".into(),
    }
}

fn validate_receiver(
    input: &WholeCellReceiverCouplingInputV1,
    work: &mut Work,
) -> Result<([Signed512; 3], [Signed512; 3]), WholeCellReceiverCouplingError> {
    let receiver = &input.receiver;
    let transport = &input.transport_input;
    if receiver.schema_version != 1
        || receiver.receiver_source_id == [0; 32]
        || receiver.receiver_revision == 0
        || receiver.scope_id != transport.cell.scope_id
        || receiver.reconstruction_id != transport.cell.reconstruction_id
        || receiver.coordinate_frame != CoordinateFrameV1::CartesianQ32_32Volume3dV1
    {
        return Err(WholeCellReceiverCouplingError::InvalidInput(
            "receiver binding",
        ));
    }
    let expected = ReceiverAabbV1::compile(
        receiver.receiver_source_id,
        receiver.scope_id,
        receiver.reconstruction_id,
        receiver.receiver_revision,
        receiver.minimum_q160.clone(),
        receiver.maximum_q160.clone(),
    )
    .map_err(|_| WholeCellReceiverCouplingError::Dependency("receiver replay failed"))?;
    if expected != *receiver {
        return Err(WholeCellReceiverCouplingError::IdentityMismatch);
    }
    let minimum = q160_vector(&receiver.minimum_q160, work)?;
    let maximum = q160_vector(&receiver.maximum_q160, work)?;
    for axis in 0..3 {
        if minimum[axis] >= maximum[axis] {
            return Err(WholeCellReceiverCouplingError::InvalidInput(
                "receiver positive volume",
            ));
        }
        let volume_min = shl(
            &Signed512::from_i64(transport.physical_volume_recipe.input.origin_q32_32[axis]),
            128,
            work,
        )?;
        let span = i128::from(transport.physical_volume_recipe.input.cell_step_q32_32)
            * i128::from(transport.physical_volume_recipe.input.extent[axis]);
        let volume_max = shl(
            &Signed512::from_i128(
                i128::from(transport.physical_volume_recipe.input.origin_q32_32[axis]) + span,
            ),
            128,
            work,
        )?;
        if minimum[axis] < volume_min || maximum[axis] > volume_max {
            return Err(WholeCellReceiverCouplingError::InvalidInput(
                "receiver outside physical volume",
            ));
        }
    }
    Ok((minimum, maximum))
}

fn validate_input(
    input: &WholeCellReceiverCouplingInputV1,
    work: &mut Work,
) -> Result<([Signed512; 3], [Signed512; 3]), WholeCellReceiverCouplingError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(WholeCellReceiverCouplingError::InvalidSchema);
    }
    let transport_bytes = input
        .transport_input
        .to_bytes()
        .map_err(|_| WholeCellReceiverCouplingError::Dependency("transport input replay failed"))?;
    let certificate_bytes = input
        .transport_certificate
        .to_bytes(&input.transport_input)
        .map_err(|_| {
            WholeCellReceiverCouplingError::Dependency("transport certificate replay failed")
        })?;
    if transport_bytes
        .len()
        .saturating_add(certificate_bytes.len())
        > MAX_AGGREGATE_LIVE_CANONICAL_BYTES
    {
        return Err(WholeCellReceiverCouplingError::ByteCeiling);
    }
    validate_origin_anchored_transport_certificate(
        &input.transport_input,
        &input.transport_certificate,
    )
    .map_err(|_| WholeCellReceiverCouplingError::IdentityMismatch)?;
    if usize::from(input.selected_step_index) >= input.transport_certificate.steps.len() {
        return Err(WholeCellReceiverCouplingError::InvalidInput(
            "selected step absent",
        ));
    }
    validate_receiver(input, work)
}

fn classify(
    input: &WholeCellReceiverCouplingInputV1,
    minimum: &[Signed512; 3],
    maximum: &[Signed512; 3],
    work: &mut Work,
) -> Result<WholeCellReceiverCouplingOutcomeV1, WholeCellReceiverCouplingError> {
    let cell = &input.transport_input.cell;
    let mut ordered: [Option<&CorrelatedAffineOutputV1>; 6] = [None; 6];
    for form in &cell.forms {
        ordered[role_index(form.role)] = Some(form);
    }
    let forms: [&CorrelatedAffineOutputV1; 6] = ordered
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(WholeCellReceiverCouplingError::InvalidInput("cell roles"))?
        .try_into()
        .map_err(|_| WholeCellReceiverCouplingError::InvalidInput("cell roles"))?;
    let points: [Affine; 3] = (0..3)
        .map(|axis| affine_from(forms[axis], work))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| WholeCellReceiverCouplingError::InvalidInput("cell point roles"))?;
    let directions: [Affine; 3] = (0..3)
        .map(|axis| affine_from(forms[axis + 3], work))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| WholeCellReceiverCouplingError::InvalidInput("cell direction roles"))?;
    let denominator = parse(&cell.form_denominator, work)?;
    let step_index = usize::from(input.selected_step_index);
    let selected = &input.transport_certificate.steps[step_index];
    let origin_projection = project_optical_phase_space_cell(&OpticalPhaseSpaceProjectionQueryV1 {
        schema_version: 1,
        cell: cell.clone(),
        target: OpticalProjectionTargetV1::ExistingOpticalIntervalSeamV1,
    })
    .map_err(|_| WholeCellReceiverCouplingError::Dependency("origin projection failed"))?;
    let start_bounds = if step_index == 0 {
        directed_projection_bounds(&origin_projection.position_intervals, work)?
    } else {
        projection_bounds(
            &input.transport_certificate.steps[step_index - 1]
                .projection
                .position_q160,
            work,
        )?
    };
    let end_projection = &selected.projection.position_q160;
    let end_bounds = projection_bounds(end_projection, work)?;
    if (0..3)
        .all(|axis| start_bounds[axis].0 > minimum[axis] && start_bounds[axis].1 < maximum[axis])
    {
        return Ok(
            WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace {
                proof: WholeCellFullProofV1::StartInside,
                receiver_axis: None,
                receiver_side: None,
            },
        );
    }
    let (end_face, end_height) = physical_face_height(&input.transport_input, step_index)?;
    let end_axis = axis_index(end_face.axis);
    let end_numerator = plane_numerator(
        &Signed512::from_i64(end_height),
        &denominator,
        &points[end_axis],
        32,
        face_orientation(end_face.side),
        work,
    )?;
    let end_denominator = affine_scale(
        &directions[end_axis],
        &Signed512::from_i64(face_orientation(end_face.side)),
        work,
    )?;
    let start_parameter = if step_index == 0 {
        None
    } else {
        let (face, height) = physical_face_height(&input.transport_input, step_index - 1)?;
        let axis = axis_index(face.axis);
        Some((
            plane_numerator(
                &Signed512::from_i64(height),
                &denominator,
                &points[axis],
                32,
                face_orientation(face.side),
                work,
            )?,
            affine_scale(
                &directions[axis],
                &Signed512::from_i64(face_orientation(face.side)),
                work,
            )?,
        ))
    };
    for receiver_axis in 0..3 {
        for side in [ReceiverFaceSideV1::Minimum, ReceiverFaceSideV1::Maximum] {
            let (coordinate, orientation) = match side {
                ReceiverFaceSideV1::Minimum => (&minimum[receiver_axis], 1),
                ReceiverFaceSideV1::Maximum => (&maximum[receiver_axis], -1),
            };
            let receiver_numerator = plane_numerator(
                coordinate,
                &denominator,
                &points[receiver_axis],
                160,
                orientation,
                work,
            )?;
            let receiver_denominator = affine_scale(
                &directions[receiver_axis],
                &Signed512::from_i64(orientation),
                work,
            )?;
            let receiver_direction_lower = affine_bounds(&receiver_denominator, work)?.0;
            if work.compare_zero(&receiver_direction_lower)? != Ordering::Greater {
                continue;
            }
            let after_start = if let Some((start_numerator, start_denominator)) = &start_parameter {
                let delta = quadratic_add(
                    &quadratic_scale_shift(
                        &product(&receiver_numerator, start_denominator, work)?,
                        32,
                        work,
                    )?,
                    &quadratic_neg(&quadratic_scale_shift(
                        &product(start_numerator, &receiver_denominator, work)?,
                        160,
                        work,
                    )?),
                    work,
                )?;
                quadratic_bounds(&delta, work)?.0 >= Signed512::zero()
            } else {
                affine_bounds(&receiver_numerator, work)?.0 >= Signed512::zero()
            };
            if !after_start {
                continue;
            }
            let before_end = quadratic_add(
                &quadratic_scale_shift(
                    &product(&end_numerator, &receiver_denominator, work)?,
                    160,
                    work,
                )?,
                &quadratic_neg(&quadratic_scale_shift(
                    &product(&receiver_numerator, &end_denominator, work)?,
                    32,
                    work,
                )?),
                work,
            )?;
            if quadratic_bounds(&before_end, work)?.0 <= Signed512::zero() {
                continue;
            }
            let mut cross_ok = true;
            for axis in 0..3 {
                if axis == receiver_axis {
                    continue;
                }
                let lower_plane =
                    plane_numerator(&minimum[axis], &denominator, &points[axis], 160, -1, work)?;
                let lower = quadratic_add(
                    &product(&lower_plane, &receiver_denominator, work)?,
                    &product(&receiver_numerator, &directions[axis], work)?,
                    work,
                )?;
                let upper_plane =
                    plane_numerator(&maximum[axis], &denominator, &points[axis], 160, 1, work)?;
                let upper = quadratic_add(
                    &product(&upper_plane, &receiver_denominator, work)?,
                    &quadratic_neg(&product(&receiver_numerator, &directions[axis], work)?),
                    work,
                )?;
                if quadratic_bounds(&lower, work)?.0 <= Signed512::zero()
                    || quadratic_bounds(&upper, work)?.0 <= Signed512::zero()
                {
                    cross_ok = false;
                    break;
                }
            }
            if cross_ok {
                return Ok(
                    WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace {
                        proof: WholeCellFullProofV1::ReceiverFace,
                        receiver_axis: Some(receiver_axis as u8),
                        receiver_side: Some(side),
                    },
                );
            }
        }
    }
    for axis in 0..3 {
        let swept_upper = max_value(start_bounds[axis].1.clone(), end_bounds[axis].1.clone());
        if swept_upper < minimum[axis] {
            return Ok(
                WholeCellReceiverCouplingOutcomeV1::CertifiedZeroBeforeFace {
                    separating_axis: axis as u8,
                    receiver_side: ReceiverFaceSideV1::Minimum,
                },
            );
        }
        let swept_lower = min_value(start_bounds[axis].0.clone(), end_bounds[axis].0.clone());
        if swept_lower > maximum[axis] {
            return Ok(
                WholeCellReceiverCouplingOutcomeV1::CertifiedZeroBeforeFace {
                    separating_axis: axis as u8,
                    receiver_side: ReceiverFaceSideV1::Maximum,
                },
            );
        }
    }
    Ok(
        WholeCellReceiverCouplingOutcomeV1::UnresolvedReceiverCoupling {
            reason: UnresolvedReceiverCouplingReasonV1::MixedReceiverFaceOrder,
        },
    )
}

pub fn compile_whole_cell_receiver_coupling(
    input: &WholeCellReceiverCouplingInputV1,
) -> Result<WholeCellReceiverCouplingV1, WholeCellReceiverCouplingError> {
    let input_bytes = input.to_bytes()?;
    let input_id = domain_hash(INPUT_DOMAIN, &input_bytes);
    let mut work = Work::default();
    let (minimum, maximum) = validate_input(input, &mut work)?;
    let outcome = classify(input, &minimum, &maximum, &mut work)?;
    let measure = input.transport_input.cell.measure.clone();
    let (accepted_measure, zero, unresolved) = match outcome {
        WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace { .. } => {
            (measure, zero_measure(), zero_measure())
        }
        WholeCellReceiverCouplingOutcomeV1::CertifiedZeroBeforeFace { .. } => {
            (zero_measure(), measure, zero_measure())
        }
        WholeCellReceiverCouplingOutcomeV1::UnresolvedReceiverCoupling { .. } => {
            (zero_measure(), zero_measure(), measure)
        }
    };
    let mut result = WholeCellReceiverCouplingV1 {
        schema_version: 1,
        input_id,
        cell_id: input.transport_input.cell.cell_id,
        transport_certificate_id: input.transport_certificate.certificate_id,
        selected_step_id: input.transport_certificate.steps[usize::from(input.selected_step_index)]
            .step_id,
        receiver_id: input.receiver.receiver_id,
        outcome,
        accepted_measure,
        zero_measure: zero,
        unresolved_measure: unresolved,
        arithmetic_receipt: WholeCellReceiverCouplingArithmeticReceiptV1 {
            storage_bits: STORAGE_BITS,
            maximum_live_bits: MAXIMUM_LIVE_BITS,
            observed_maximum_live_bits: work.observed,
            checked_integer_operations: work.operations,
            bound_comparisons: work.comparisons,
        },
        result_id: [0; 32],
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    };
    result.result_id = domain_hash(RESULT_DOMAIN, &json(&result)?);
    if json(&result)?.len() > MAX_RESULT_BYTES {
        return Err(WholeCellReceiverCouplingError::ByteCeiling);
    }
    Ok(result)
}

pub fn validate_whole_cell_receiver_coupling(
    input: &WholeCellReceiverCouplingInputV1,
    result: &WholeCellReceiverCouplingV1,
) -> Result<(), WholeCellReceiverCouplingError> {
    if &compile_whole_cell_receiver_coupling(input)? == result {
        Ok(())
    } else {
        Err(WholeCellReceiverCouplingError::IdentityMismatch)
    }
}

impl WholeCellReceiverCouplingInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, WholeCellReceiverCouplingError> {
        let bytes = json(self)?;
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(WholeCellReceiverCouplingError::ByteCeiling);
        }
        let mut work = Work::default();
        validate_input(self, &mut work)?;
        Ok(bytes)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WholeCellReceiverCouplingError> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(WholeCellReceiverCouplingError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? == bytes {
            Ok(value)
        } else {
            Err(WholeCellReceiverCouplingError::CodecDefect)
        }
    }
}
impl WholeCellReceiverCouplingV1 {
    pub fn to_bytes(
        &self,
        input: &WholeCellReceiverCouplingInputV1,
    ) -> Result<Vec<u8>, WholeCellReceiverCouplingError> {
        validate_whole_cell_receiver_coupling(input, self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_RESULT_BYTES {
            Err(WholeCellReceiverCouplingError::ByteCeiling)
        } else {
            Ok(bytes)
        }
    }
    pub fn from_bytes(
        input: &WholeCellReceiverCouplingInputV1,
        bytes: &[u8],
    ) -> Result<Self, WholeCellReceiverCouplingError> {
        if bytes.len() > MAX_RESULT_BYTES {
            return Err(WholeCellReceiverCouplingError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(input)? == bytes {
            Ok(value)
        } else {
            Err(WholeCellReceiverCouplingError::CodecDefect)
        }
    }
}

fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, WholeCellReceiverCouplingError> {
    serde_json::to_vec(value).map_err(|_| WholeCellReceiverCouplingError::CodecDefect)
}
fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WholeCellReceiverCouplingError> {
    serde_json::from_slice(bytes).map_err(|_| WholeCellReceiverCouplingError::CodecDefect)
}
fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update([0]);
    hash.update(bytes);
    hash.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shield_and_correlated_monomial_cancellation_are_exact() {
        let mut work = Work::default();
        let within = Signed512::one().checked_shl(390).expect("391-bit value");
        assert_eq!(work.see(&within), Ok(()));
        let outside = Signed512::one().checked_shl(391).expect("392-bit value");
        assert_eq!(
            work.see(&outside),
            Err(WholeCellReceiverCouplingError::ArithmeticShieldExceeded)
        );

        let u = Affine {
            center: Signed512::zero(),
            coefficients: [
                Signed512::one(),
                Signed512::zero(),
                Signed512::zero(),
                Signed512::zero(),
            ],
            remainder_lower: Signed512::zero(),
            remainder_upper: Signed512::zero(),
        };
        let mut work = Work::default();
        let square = product(&u, &u, &mut work).expect("u squared");
        let cancelled = quadratic_add(&square, &quadratic_neg(&square), &mut work)
            .expect("correlated cancellation");
        assert_eq!(
            quadratic_bounds(&cancelled, &mut work).expect("bounds"),
            (Signed512::zero(), Signed512::zero())
        );
    }
}
