#![deny(warnings)]
//! Capability-free immutable-origin transport certificates for exact optical phase-space cells.

use fixed_interval_arithmetic::{FixedArithmeticError, Signed512};
use optical_phase_space_cell_binding::{
    CorrelatedAffineOutputV1, OpticalPhaseSpaceCellV1, OpticalPhaseSpaceProjectionQueryV1,
    OpticalProjectionTargetV1, PhaseSpaceOutputRoleV1, PhaseSpaceParameterizationV1,
    project_optical_phase_space_cell,
};
use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, ConditionalIntervalCellStepEventV1,
    ConditionalIntervalCellStepInputV1, ConditionalIntervalCellStepOutcomeV1,
    ConditionalIntervalEvidenceKindV1, IntervalFaceAxisV1, IntervalFaceSideV1, IntervalFaceV1,
    PhysicalVolumeRecipeV1, PhysicalVolumeV1, SignedDecimalIntervalV1, build_physical_cell,
    compile_conditional_interval_cell_step, validate_physical_volume,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, fmt};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAXIMUM_STEPS: u8 = 64;
pub const IMMUTABLE_ORIGIN_SCALAR_BITS: u16 = 64;
pub const DERIVED_MAXIMUM_LIVE_BITS: u16 = 490;
pub const STORAGE_BITS: u16 = 512;
pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_FACE_STEP_BYTES: usize = 256 * 1024;
pub const MAX_CERTIFICATE_BYTES: usize = 20 * 1024 * 1024;
pub const MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 40 * 1024 * 1024;
pub const MAX_DIRECT_FORM_SCALARS: u16 = 1_024;
pub const MAX_CHECKED_INTEGER_OPERATIONS: u32 = 24_576;
pub const MAX_DIRECTED_PROJECTIONS: u16 = 768;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str = "same_medium_origin_anchored_transport_evidence_only_no_interface_coupling_arrival_power_visibility_runtime_or_promotion_claim";

const INPUT_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.transport.input.v1";
const FORM_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.transport.form.v1";
const STEP_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.transport.step.v1";
const CERTIFICATE_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.transport.certificate.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransportCertificateError {
    InvalidSchema,
    InvalidInput(&'static str),
    Dependency(&'static str),
    ByteCeiling,
    ResourceCeiling,
    ArithmeticShieldExceeded,
    ProjectionOutOfRange,
    IdentityMismatch,
    CodecDefect,
}

impl fmt::Display for TransportCertificateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for TransportCertificateError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginAnchoredTransportInputV1 {
    pub schema_version: u16,
    pub cell: OpticalPhaseSpaceCellV1,
    pub physical_volume_recipe: PhysicalVolumeRecipeV1,
    pub physical_volume: PhysicalVolumeV1,
    pub current_cell: CellIndex3V1,
    pub band_time_id: [u8; 32],
    pub maximum_steps: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRationalV1 {
    pub numerator: String,
    pub denominator: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportAffineFormV1 {
    pub role: PhaseSpaceOutputRoleV1,
    pub center: ExactRationalV1,
    pub coefficients: [ExactRationalV1; 4],
    pub remainder_lower: ExactRationalV1,
    pub remainder_upper: ExactRationalV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportProjectionV1 {
    pub position_q160: [SignedDecimalIntervalV1; 3],
    pub direction_q1_62: [SignedDecimalIntervalV1; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportArithmeticReceiptV1 {
    pub storage_bits: u16,
    pub derived_maximum_live_bits: u16,
    pub observed_maximum_live_bits: u16,
    pub checked_integer_operations: u32,
    pub directed_divisions: u16,
    pub directed_projections: u16,
    pub direct_form_scalars: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginAnchoredTransportTerminalV1 {
    InterfaceRequired,
    OuterDomainExit,
    UnavailableNeighbor,
    AmbiguousNextFace,
    NoForwardProgress,
    ArithmeticShieldExceeded,
    ProjectionOutOfRange,
    WorkExhausted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginAnchoredFaceStepV1 {
    pub step_index: u8,
    pub current_cell: CellIndex3V1,
    pub certified_face: IntervalFaceV1,
    pub physical_input: ConditionalIntervalCellStepInputV1,
    pub physical_event: ConditionalIntervalCellStepEventV1,
    pub successor_cell: Option<CellIndex3V1>,
    pub direct_forms: [TransportAffineFormV1; 3],
    pub projection: TransportProjectionV1,
    pub direct_form_id: [u8; 32],
    pub arithmetic_receipt: TransportArithmeticReceiptV1,
    pub step_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginAnchoredTransportCertificateV1 {
    pub schema_version: u16,
    pub input_id: [u8; 32],
    pub immutable_origin_cell_id: [u8; 32],
    pub physical_volume_recipe_id: [u8; 32],
    pub physical_volume_id: [u8; 32],
    pub band_time_id: [u8; 32],
    pub steps: Vec<OriginAnchoredFaceStepV1>,
    pub terminal: OriginAnchoredTransportTerminalV1,
    pub aggregate_receipt: TransportArithmeticReceiptV1,
    pub certificate_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rational {
    numerator: Signed512,
    denominator: Signed512,
}

#[derive(Default)]
struct Work {
    observed: u16,
    operations: u32,
    divisions: u16,
    projections: u16,
    scalars: u16,
}

impl Work {
    fn see(&mut self, values: &[&Signed512]) -> Result<(), CalcError> {
        for value in values {
            self.observed = self.observed.max(value.maximum_magnitude_bits());
        }
        if self.observed > DERIVED_MAXIMUM_LIVE_BITS {
            return Err(CalcError::Shield);
        }
        Ok(())
    }
    fn operation(&mut self, count: u32) -> Result<(), CalcError> {
        self.operations = self
            .operations
            .checked_add(count)
            .ok_or(CalcError::Resource)?;
        if self.operations > MAX_CHECKED_INTEGER_OPERATIONS {
            Err(CalcError::Resource)
        } else {
            Ok(())
        }
    }
    fn scalar(&mut self) -> Result<(), CalcError> {
        self.scalars = self.scalars.checked_add(1).ok_or(CalcError::Resource)?;
        if self.scalars > MAX_DIRECT_FORM_SCALARS {
            Err(CalcError::Resource)
        } else {
            Ok(())
        }
    }
    fn receipt(&self) -> TransportArithmeticReceiptV1 {
        TransportArithmeticReceiptV1 {
            storage_bits: STORAGE_BITS,
            derived_maximum_live_bits: DERIVED_MAXIMUM_LIVE_BITS,
            observed_maximum_live_bits: self.observed,
            checked_integer_operations: self.operations,
            directed_divisions: self.divisions,
            directed_projections: self.projections,
            direct_form_scalars: self.scalars,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CalcError {
    Shield,
    Projection,
    Resource,
    Invalid,
}

fn arithmetic<T>(value: Result<T, FixedArithmeticError>) -> Result<T, CalcError> {
    value.map_err(|error| match error {
        FixedArithmeticError::StorageOverflow => CalcError::Shield,
        _ => CalcError::Invalid,
    })
}

fn parse(value: &str) -> Result<Signed512, CalcError> {
    arithmetic(Signed512::from_canonical_decimal(value))
}

fn absolute(value: &Signed512) -> Signed512 {
    if value.is_negative() {
        value.checked_neg()
    } else {
        value.clone()
    }
}

fn add(left: &Signed512, right: &Signed512, work: &mut Work) -> Result<Signed512, CalcError> {
    let value = arithmetic(left.checked_add(right))?;
    work.operation(1)?;
    work.see(&[left, right, &value])?;
    Ok(value)
}
fn sub(left: &Signed512, right: &Signed512, work: &mut Work) -> Result<Signed512, CalcError> {
    let value = arithmetic(left.checked_sub(right))?;
    work.operation(1)?;
    work.see(&[left, right, &value])?;
    Ok(value)
}
fn mul(left: &Signed512, right: &Signed512, work: &mut Work) -> Result<Signed512, CalcError> {
    let value = arithmetic(left.checked_mul(right))?;
    work.operation(1)?;
    work.see(&[left, right, &value])?;
    Ok(value)
}
fn shl(value: &Signed512, bits: u16, work: &mut Work) -> Result<Signed512, CalcError> {
    let shifted = arithmetic(value.checked_shl(bits))?;
    work.operation(1)?;
    work.see(&[value, &shifted])?;
    Ok(shifted)
}

fn gcd(mut left: Signed512, mut right: Signed512, work: &mut Work) -> Result<Signed512, CalcError> {
    left = absolute(&left);
    right = absolute(&right);
    while right != Signed512::zero() {
        let quotient = arithmetic(left.div_floor(&right))?;
        let product = arithmetic(quotient.checked_mul(&right))?;
        let remainder = arithmetic(left.checked_sub(&product))?;
        work.see(&[&left, &right, &quotient, &product, &remainder])?;
        left = right;
        right = remainder;
    }
    work.operation(1)?;
    Ok(left)
}

impl Rational {
    fn new(
        mut numerator: Signed512,
        mut denominator: Signed512,
        work: &mut Work,
    ) -> Result<Self, CalcError> {
        if denominator == Signed512::zero() {
            return Err(CalcError::Invalid);
        }
        if denominator.is_negative() {
            numerator = numerator.checked_neg();
            denominator = denominator.checked_neg();
        }
        work.see(&[&numerator, &denominator])?;
        let divisor = gcd(numerator.clone(), denominator.clone(), work)?;
        let numerator = arithmetic(numerator.div_floor(&divisor))?;
        let denominator = arithmetic(denominator.div_floor(&divisor))?;
        work.see(&[&numerator, &denominator])?;
        Ok(Self {
            numerator,
            denominator,
        })
    }
    fn zero(work: &mut Work) -> Result<Self, CalcError> {
        Self::new(Signed512::zero(), Signed512::one(), work)
    }
    fn add(&self, other: &Self, work: &mut Work) -> Result<Self, CalcError> {
        let left = mul(&self.numerator, &other.denominator, work)?;
        let right = mul(&other.numerator, &self.denominator, work)?;
        let numerator = add(&left, &right, work)?;
        let denominator = mul(&self.denominator, &other.denominator, work)?;
        Self::new(numerator, denominator, work)
    }
    fn sub(&self, other: &Self, work: &mut Work) -> Result<Self, CalcError> {
        let mut negative = other.clone();
        negative.numerator = negative.numerator.checked_neg();
        self.add(&negative, work)
    }
    fn absolute(&self) -> Self {
        Self {
            numerator: absolute(&self.numerator),
            denominator: self.denominator.clone(),
        }
    }
    fn compare(&self, other: &Self, work: &mut Work) -> Result<Ordering, CalcError> {
        let left = mul(&self.numerator, &other.denominator, work)?;
        let right = mul(&other.numerator, &self.denominator, work)?;
        Ok(left.cmp(&right))
    }
    fn project(
        &self,
        fractional_bits: u16,
        work: &mut Work,
    ) -> Result<(Signed512, Signed512), CalcError> {
        work.projections = work.projections.checked_add(1).ok_or(CalcError::Resource)?;
        if work.projections > MAX_DIRECTED_PROJECTIONS {
            return Err(CalcError::Resource);
        }
        let shifted = shl(&self.numerator, fractional_bits, work)?;
        let lower = arithmetic(shifted.div_floor(&self.denominator))?;
        let upper = arithmetic(shifted.div_ceil(&self.denominator))?;
        work.operation(2)?;
        work.divisions = work.divisions.checked_add(2).ok_or(CalcError::Resource)?;
        work.see(&[&lower, &upper])?;
        Ok((lower, upper))
    }
    fn public(&self, work: &mut Work) -> Result<ExactRationalV1, CalcError> {
        work.scalar()?;
        Ok(ExactRationalV1 {
            numerator: self.numerator.canonical_decimal(),
            denominator: self.denominator.canonical_decimal(),
        })
    }
}

fn integer_extent(
    form: &CorrelatedAffineOutputV1,
    work: &mut Work,
) -> Result<(Signed512, Signed512), CalcError> {
    let center = parse(&form.center_numerator)?;
    let mut radius = Signed512::zero();
    for coefficient in &form.coefficient_numerators {
        radius = add(&radius, &absolute(&parse(coefficient)?), work)?;
    }
    let lower = add(
        &sub(&center, &radius, work)?,
        &parse(&form.remainder_lower_numerator)?,
        work,
    )?;
    let upper = add(
        &add(&center, &radius, work)?,
        &parse(&form.remainder_upper_numerator)?,
        work,
    )?;
    Ok((lower, upper))
}

fn rational_extent(
    form: &[Rational; 7],
    work: &mut Work,
) -> Result<(Rational, Rational), CalcError> {
    let mut radius = Rational::zero(work)?;
    for coefficient in &form[1..5] {
        radius = radius.add(&coefficient.absolute(), work)?;
    }
    let lower = form[0].sub(&radius, work)?.add(&form[5], work)?;
    let upper = form[0].add(&radius, work)?.add(&form[6], work)?;
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

const RESIDUAL_VARIABLES: usize = 8;
type AffinePolynomial = [Signed512; RESIDUAL_VARIABLES + 1];

struct QuadraticPolynomial {
    constant: Signed512,
    linear: [Signed512; RESIDUAL_VARIABLES],
    quadratic: [[Signed512; RESIDUAL_VARIABLES]; RESIDUAL_VARIABLES],
}

fn zero_affine() -> AffinePolynomial {
    std::array::from_fn(|_| Signed512::zero())
}

fn form_affine(
    form: &CorrelatedAffineOutputV1,
    remainder_variable: usize,
) -> Result<AffinePolynomial, CalcError> {
    let mut value = zero_affine();
    value[0] = parse(&form.center_numerator)?;
    for index in 0..4 {
        value[index + 1] = parse(&form.coefficient_numerators[index])?;
    }
    value[1 + 4 + remainder_variable] = Signed512::one();
    Ok(value)
}

fn multiply_affine(
    left: &AffinePolynomial,
    right: &AffinePolynomial,
    work: &mut Work,
) -> Result<QuadraticPolynomial, CalcError> {
    let constant = mul(&left[0], &right[0], work)?;
    let mut linear: [Signed512; RESIDUAL_VARIABLES] = std::array::from_fn(|_| Signed512::zero());
    let mut quadratic: [[Signed512; RESIDUAL_VARIABLES]; RESIDUAL_VARIABLES] =
        std::array::from_fn(|_| std::array::from_fn(|_| Signed512::zero()));
    for index in 0..RESIDUAL_VARIABLES {
        linear[index] = add(
            &mul(&left[0], &right[index + 1], work)?,
            &mul(&left[index + 1], &right[0], work)?,
            work,
        )?;
        for other in 0..RESIDUAL_VARIABLES {
            quadratic[index][other] = mul(&left[index + 1], &right[other + 1], work)?;
        }
    }
    Ok(QuadraticPolynomial {
        constant,
        linear,
        quadratic,
    })
}

fn scale_polynomial(
    value: &QuadraticPolynomial,
    factor: &Signed512,
    work: &mut Work,
) -> Result<QuadraticPolynomial, CalcError> {
    let constant = mul(&value.constant, factor, work)?;
    let mut linear: [Signed512; RESIDUAL_VARIABLES] = std::array::from_fn(|_| Signed512::zero());
    let mut quadratic: [[Signed512; RESIDUAL_VARIABLES]; RESIDUAL_VARIABLES] =
        std::array::from_fn(|_| std::array::from_fn(|_| Signed512::zero()));
    for index in 0..RESIDUAL_VARIABLES {
        linear[index] = mul(&value.linear[index], factor, work)?;
        for other in 0..RESIDUAL_VARIABLES {
            quadratic[index][other] = mul(&value.quadratic[index][other], factor, work)?;
        }
    }
    Ok(QuadraticPolynomial {
        constant,
        linear,
        quadratic,
    })
}

fn combine_polynomial(
    left: &QuadraticPolynomial,
    right: &QuadraticPolynomial,
    subtract_right: bool,
    work: &mut Work,
) -> Result<QuadraticPolynomial, CalcError> {
    let combine = |left: &Signed512, right: &Signed512, work: &mut Work| {
        if subtract_right {
            sub(left, right, work)
        } else {
            add(left, right, work)
        }
    };
    let constant = combine(&left.constant, &right.constant, work)?;
    let mut linear: [Signed512; RESIDUAL_VARIABLES] = std::array::from_fn(|_| Signed512::zero());
    let mut quadratic: [[Signed512; RESIDUAL_VARIABLES]; RESIDUAL_VARIABLES] =
        std::array::from_fn(|_| std::array::from_fn(|_| Signed512::zero()));
    for index in 0..RESIDUAL_VARIABLES {
        linear[index] = combine(&left.linear[index], &right.linear[index], work)?;
        for other in 0..RESIDUAL_VARIABLES {
            quadratic[index][other] = combine(
                &left.quadratic[index][other],
                &right.quadratic[index][other],
                work,
            )?;
        }
    }
    Ok(QuadraticPolynomial {
        constant,
        linear,
        quadratic,
    })
}

fn product_bounds(
    coefficient: &Signed512,
    left: &(Signed512, Signed512),
    right: &(Signed512, Signed512),
    work: &mut Work,
) -> Result<(Signed512, Signed512), CalcError> {
    let values = [
        mul(&mul(coefficient, &left.0, work)?, &right.0, work)?,
        mul(&mul(coefficient, &left.0, work)?, &right.1, work)?,
        mul(&mul(coefficient, &left.1, work)?, &right.0, work)?,
        mul(&mul(coefficient, &left.1, work)?, &right.1, work)?,
    ];
    Ok((
        values.iter().min().expect("four values").clone(),
        values.iter().max().expect("four values").clone(),
    ))
}

fn polynomial_bounds(
    value: &QuadraticPolynomial,
    ranges: &[(Signed512, Signed512); RESIDUAL_VARIABLES],
    work: &mut Work,
) -> Result<(Signed512, Signed512), CalcError> {
    let mut lower = value.constant.clone();
    let mut upper = value.constant.clone();
    for index in 0..RESIDUAL_VARIABLES {
        let term = product_bounds(
            &value.linear[index],
            &ranges[index],
            &(Signed512::one(), Signed512::one()),
            work,
        )?;
        lower = add(&lower, &term.0, work)?;
        upper = add(&upper, &term.1, work)?;
        for other in 0..RESIDUAL_VARIABLES {
            let term = product_bounds(
                &value.quadratic[index][other],
                &ranges[index],
                &ranges[other],
                work,
            )?;
            lower = add(&lower, &term.0, work)?;
            upper = add(&upper, &term.1, work)?;
        }
    }
    Ok((lower, upper))
}

fn direct_face(
    cell: &OpticalPhaseSpaceCellV1,
    face: IntervalFaceV1,
    height_q32_32: i64,
    direction_projection: [SignedDecimalIntervalV1; 3],
    work: &mut Work,
) -> Result<([TransportAffineFormV1; 3], TransportProjectionV1), CalcError> {
    let axis = match face.axis {
        IntervalFaceAxisV1::X => 0,
        IntervalFaceAxisV1::Y => 1,
        IntervalFaceAxisV1::Z => 2,
    };
    let mut ordered: [Option<&CorrelatedAffineOutputV1>; 6] = [None; 6];
    for form in &cell.forms {
        ordered[role_index(form.role)] = Some(form);
    }
    let forms: [&CorrelatedAffineOutputV1; 6] = ordered
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(CalcError::Invalid)?
        .try_into()
        .map_err(|_| CalcError::Invalid)?;
    let denominator = parse(&cell.form_denominator)?;
    let scale = shl(&Signed512::one(), 32, work)?;
    let height = Signed512::from_i64(height_q32_32);
    let p_axis = forms[axis];
    let v_axis = forms[axis + 3];
    let (v_lower, v_upper) = integer_extent(v_axis, work)?;
    if v_lower <= Signed512::zero() && v_upper >= Signed512::zero() {
        return Err(CalcError::Projection);
    }
    let b = parse(&v_axis.center_numerator)?;
    let height_d = mul(&height, &denominator, work)?;
    let p_axis_scaled = mul(&parse(&p_axis.center_numerator)?, &scale, work)?;
    let a = sub(&height_d, &p_axis_scaled, work)?;
    let b_squared = mul(&b, &b, work)?;
    let mut public_forms = Vec::with_capacity(3);
    let mut projected = Vec::with_capacity(3);
    for index in 0..3 {
        let raw: [Rational; 7] = if index == axis {
            let centre = Rational::new(height.clone(), scale.clone(), work)?;
            [
                centre,
                Rational::zero(work)?,
                Rational::zero(work)?,
                Rational::zero(work)?,
                Rational::zero(work)?,
                Rational::zero(work)?,
                Rational::zero(work)?,
            ]
        } else {
            let position = forms[index];
            let direction = forms[index + 3];
            let center_left = mul(
                &mul(&parse(&position.center_numerator)?, &scale, work)?,
                &b,
                work,
            )?;
            let center_right = mul(&a, &parse(&direction.center_numerator)?, work)?;
            let center_numerator = add(&center_left, &center_right, work)?;
            let center_denominator = mul(&mul(&denominator, &scale, work)?, &b, work)?;
            let centre = Rational::new(center_numerator.clone(), center_denominator, work)?;
            let coefficient_denominator = mul(&mul(&denominator, &scale, work)?, &b_squared, work)?;
            let mut coefficient_numerators = Vec::with_capacity(4);
            let mut coefficients = Vec::with_capacity(4);
            for symbol in 0..4 {
                let p_axis_coefficient = parse(&p_axis.coefficient_numerators[symbol])?;
                let v_axis_coefficient = parse(&v_axis.coefficient_numerators[symbol])?;
                let position_coefficient = parse(&position.coefficient_numerators[symbol])?;
                let direction_coefficient = parse(&direction.coefficient_numerators[symbol])?;
                let dt_first = mul(
                    &mul(&p_axis_coefficient.checked_neg(), &b, work)?,
                    &scale,
                    work,
                )?;
                let dt_second = mul(&a, &v_axis_coefficient, work)?;
                let dt = sub(&dt_first, &dt_second, work)?;
                let first = mul(
                    &mul(&mul(&position_coefficient, &scale, work)?, &b, work)?,
                    &b,
                    work,
                )?;
                let second = mul(&dt, &parse(&direction.center_numerator)?, work)?;
                let third = mul(&mul(&a, &direction_coefficient, work)?, &b, work)?;
                let numerator = add(&add(&first, &second, work)?, &third, work)?;
                coefficient_numerators.push(numerator.clone());
                coefficients.push(Rational::new(
                    numerator,
                    coefficient_denominator.clone(),
                    work,
                )?);
            }
            let affine_center_n = mul(&center_numerator, &b, work)?;
            let v_axis_affine = form_affine(v_axis, 1)?;
            let position_affine = form_affine(position, 2)?;
            let direction_affine = form_affine(direction, 3)?;
            let mut a_affine = zero_affine();
            a_affine[0] = a.clone();
            for symbol in 0..4 {
                a_affine[symbol + 1] = mul(
                    &parse(&p_axis.coefficient_numerators[symbol])?.checked_neg(),
                    &scale,
                    work,
                )?;
            }
            a_affine[1 + 4] = scale.checked_neg();
            let position_velocity = scale_polynomial(
                &multiply_affine(&position_affine, &v_axis_affine, work)?,
                &scale,
                work,
            )?;
            let axis_velocity = multiply_affine(&a_affine, &direction_affine, work)?;
            let exact_numerator =
                combine_polynomial(&position_velocity, &axis_velocity, false, work)?;
            let exact_scaled = scale_polynomial(&exact_numerator, &b_squared, work)?;
            let mut affine_numerator = zero_affine();
            affine_numerator[0] = affine_center_n;
            for symbol in 0..4 {
                affine_numerator[symbol + 1] = coefficient_numerators[symbol].clone();
            }
            let affine_scaled = multiply_affine(&affine_numerator, &v_axis_affine, work)?;
            let residual_numerator = combine_polynomial(&exact_scaled, &affine_scaled, true, work)?;
            let mut ranges: [(Signed512, Signed512); RESIDUAL_VARIABLES] =
                std::array::from_fn(|_| (Signed512::zero(), Signed512::zero()));
            for range in &mut ranges[..4] {
                *range = (Signed512::one().checked_neg(), Signed512::one());
            }
            for (slot, form) in [p_axis, v_axis, position, direction].iter().enumerate() {
                ranges[4 + slot] = (
                    parse(&form.remainder_lower_numerator)?,
                    parse(&form.remainder_upper_numerator)?,
                );
            }
            let residual_bounds = polynomial_bounds(&residual_numerator, &ranges, work)?;
            let denominator_factor =
                mul(&mul(&mul(&denominator, &scale, work)?, &b, work)?, &b, work)?;
            let mut candidates = Vec::with_capacity(4);
            for numerator in [&residual_bounds.0, &residual_bounds.1] {
                for velocity in [&v_lower, &v_upper] {
                    candidates.push(Rational::new(
                        numerator.clone(),
                        mul(&denominator_factor, velocity, work)?,
                        work,
                    )?);
                }
            }
            let mut remainder_lower = candidates[0].clone();
            let mut remainder_upper = candidates[0].clone();
            for candidate in candidates.into_iter().skip(1) {
                if candidate.compare(&remainder_lower, work)? == Ordering::Less {
                    remainder_lower = candidate.clone();
                }
                if candidate.compare(&remainder_upper, work)? == Ordering::Greater {
                    remainder_upper = candidate;
                }
            }
            [
                centre,
                coefficients[0].clone(),
                coefficients[1].clone(),
                coefficients[2].clone(),
                coefficients[3].clone(),
                remainder_lower,
                remainder_upper,
            ]
        };
        let extent = rational_extent(&raw, work)?;
        let (lower, _) = extent.0.project(160, work)?;
        let (_, upper) = extent.1.project(160, work)?;
        projected.push(SignedDecimalIntervalV1 {
            fractional_bits: 160,
            lower: lower.canonical_decimal(),
            upper: upper.canonical_decimal(),
        });
        public_forms.push(TransportAffineFormV1 {
            role: [
                PhaseSpaceOutputRoleV1::PointX,
                PhaseSpaceOutputRoleV1::PointY,
                PhaseSpaceOutputRoleV1::PointZ,
            ][index],
            center: raw[0].public(work)?,
            coefficients: [
                raw[1].public(work)?,
                raw[2].public(work)?,
                raw[3].public(work)?,
                raw[4].public(work)?,
            ],
            remainder_lower: raw[5].public(work)?,
            remainder_upper: raw[6].public(work)?,
        });
    }
    Ok((
        public_forms.try_into().map_err(|_| CalcError::Invalid)?,
        TransportProjectionV1 {
            position_q160: projected.try_into().map_err(|_| CalcError::Invalid)?,
            direction_q1_62: direction_projection,
        },
    ))
}

fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, TransportCertificateError> {
    serde_json::to_vec(value).map_err(|_| TransportCertificateError::CodecDefect)
}
fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
fn bit_cap(value: &str) -> Result<(), TransportCertificateError> {
    let parsed = Signed512::from_canonical_decimal(value)
        .map_err(|_| TransportCertificateError::InvalidInput("noncanonical origin scalar"))?;
    if parsed.maximum_magnitude_bits() > IMMUTABLE_ORIGIN_SCALAR_BITS {
        Err(TransportCertificateError::ArithmeticShieldExceeded)
    } else {
        Ok(())
    }
}

fn validate_input(input: &OriginAnchoredTransportInputV1) -> Result<(), TransportCertificateError> {
    if input.schema_version != CONTRACT_VERSION
        || !(1..=MAXIMUM_STEPS).contains(&input.maximum_steps)
        || input.band_time_id == [0; 32]
    {
        return Err(TransportCertificateError::InvalidSchema);
    }
    let cell_bytes = input.cell.to_bytes().map_err(|_| {
        TransportCertificateError::Dependency("optical phase-space cell replay failed")
    })?;
    if OpticalPhaseSpaceCellV1::from_bytes(&cell_bytes).map_err(|_| {
        TransportCertificateError::Dependency("optical phase-space cell replay failed")
    })? != input.cell
    {
        return Err(TransportCertificateError::Dependency(
            "optical phase-space cell drift",
        ));
    }
    if input.cell.parameterization != PhaseSpaceParameterizationV1::TransverseAreaDirection4d {
        return Err(TransportCertificateError::InvalidInput(
            "unsupported parameterization",
        ));
    }
    bit_cap(&input.cell.form_denominator)?;
    for form in &input.cell.forms {
        bit_cap(&form.center_numerator)?;
        for coefficient in &form.coefficient_numerators {
            bit_cap(coefficient)?;
        }
        bit_cap(&form.remainder_lower_numerator)?;
        bit_cap(&form.remainder_upper_numerator)?;
    }
    validate_physical_volume(&input.physical_volume_recipe, &input.physical_volume)
        .map_err(|_| TransportCertificateError::Dependency("physical volume replay failed"))?;
    if input.cell.scope_id != input.physical_volume_recipe.input.scope_id
        || input.cell.reconstruction_id != input.physical_volume_recipe.input.reconstruction_id
    {
        return Err(TransportCertificateError::InvalidInput(
            "cell and physical scope mismatch",
        ));
    }
    build_physical_cell(
        &input.physical_volume_recipe,
        &input.physical_volume,
        input.current_cell,
    )
    .map_err(|_| TransportCertificateError::InvalidInput("current cell unavailable"))?;
    Ok(())
}

impl OriginAnchoredTransportInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, TransportCertificateError> {
        let bytes = json(self)?;
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(TransportCertificateError::ByteCeiling);
        }
        validate_input(self)?;
        Ok(bytes)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, TransportCertificateError> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(TransportCertificateError::ByteCeiling);
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|_| TransportCertificateError::CodecDefect)?;
        if value.to_bytes()? != bytes {
            return Err(TransportCertificateError::CodecDefect);
        }
        Ok(value)
    }
}

fn evidence_equal(left: &CellEvidenceV1, right: &CellEvidenceV1) -> bool {
    left == right
}

fn containment(
    inner: &[SignedDecimalIntervalV1; 3],
    outer: &[SignedDecimalIntervalV1; 3],
) -> Result<bool, TransportCertificateError> {
    for axis in 0..3 {
        if inner[axis].fractional_bits != 160 || outer[axis].fractional_bits != 160 {
            return Err(TransportCertificateError::ProjectionOutOfRange);
        }
        let inner_lower = Signed512::from_canonical_decimal(&inner[axis].lower)
            .map_err(|_| TransportCertificateError::ProjectionOutOfRange)?;
        let inner_upper = Signed512::from_canonical_decimal(&inner[axis].upper)
            .map_err(|_| TransportCertificateError::ProjectionOutOfRange)?;
        let outer_lower = Signed512::from_canonical_decimal(&outer[axis].lower)
            .map_err(|_| TransportCertificateError::ProjectionOutOfRange)?;
        let outer_upper = Signed512::from_canonical_decimal(&outer[axis].upper)
            .map_err(|_| TransportCertificateError::ProjectionOutOfRange)?;
        if inner_lower < outer_lower || inner_upper > outer_upper {
            return Ok(false);
        }
    }
    Ok(true)
}

fn terminal_from_calc(
    error: CalcError,
) -> Result<OriginAnchoredTransportTerminalV1, TransportCertificateError> {
    match error {
        CalcError::Shield => Ok(OriginAnchoredTransportTerminalV1::ArithmeticShieldExceeded),
        CalcError::Projection => Ok(OriginAnchoredTransportTerminalV1::ProjectionOutOfRange),
        CalcError::Resource => Err(TransportCertificateError::ResourceCeiling),
        CalcError::Invalid => Err(TransportCertificateError::InvalidInput(
            "direct-origin arithmetic defect",
        )),
    }
}

pub fn compile_origin_anchored_transport(
    input: &OriginAnchoredTransportInputV1,
) -> Result<OriginAnchoredTransportCertificateV1, TransportCertificateError> {
    let input_bytes = input.to_bytes()?;
    let input_id = domain_hash(INPUT_DOMAIN, &input_bytes);
    let origin_projection = project_optical_phase_space_cell(&OpticalPhaseSpaceProjectionQueryV1 {
        schema_version: 1,
        cell: input.cell.clone(),
        target: OpticalProjectionTargetV1::ExistingOpticalIntervalSeamV1,
    })
    .map_err(|_| TransportCertificateError::Dependency("origin projection replay failed"))?;
    let direction: [SignedDecimalIntervalV1; 3] =
        origin_projection
            .direction_intervals
            .map(|value| SignedDecimalIntervalV1 {
                fractional_bits: value.fractional_bits,
                lower: value.lower,
                upper: value.upper,
            });
    let mut projection = TransportProjectionV1 {
        position_q160: origin_projection
            .position_intervals
            .map(|value| SignedDecimalIntervalV1 {
                fractional_bits: value.fractional_bits,
                lower: value.lower,
                upper: value.upper,
            }),
        direction_q1_62: direction.clone(),
    };
    let mut current = input.current_cell;
    let mut steps = Vec::with_capacity(usize::from(input.maximum_steps));
    let mut work = Work::default();
    let mut terminal = OriginAnchoredTransportTerminalV1::WorkExhausted;
    for step_index in 0..input.maximum_steps {
        let physical_input = ConditionalIntervalCellStepInputV1 {
            schema_version: 1,
            state_source_id: input.cell.source_id,
            scope_id: input.cell.scope_id,
            reconstruction_id: input.cell.reconstruction_id,
            state_revision: input.cell.source_revision,
            evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
            physical_volume_recipe_id: input.physical_volume_recipe.physical_volume_recipe_id,
            physical_volume_id: input.physical_volume.physical_volume_id,
            current_cell: current,
            point_q160: projection.position_q160.clone(),
            direction_q1_62: direction.clone(),
        };
        let event = compile_conditional_interval_cell_step(
            &input.physical_volume_recipe,
            &input.physical_volume,
            &physical_input,
        )
        .map_err(|_| TransportCertificateError::Dependency("physical interval step failed"))?;
        let (certified, successor, event_terminal) = match &event.outcome {
            ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace => {
                terminal = OriginAnchoredTransportTerminalV1::AmbiguousNextFace;
                break;
            }
            ConditionalIntervalCellStepOutcomeV1::NoForwardProgress => {
                terminal = OriginAnchoredTransportTerminalV1::NoForwardProgress;
                break;
            }
            ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } => {
                (certified, certified.neighbor, None)
            }
            ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { certified } => (
                certified,
                None,
                Some(OriginAnchoredTransportTerminalV1::OuterDomainExit),
            ),
            ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { certified } => (
                certified,
                certified.neighbor,
                Some(OriginAnchoredTransportTerminalV1::UnavailableNeighbor),
            ),
        };
        let physical_cell = build_physical_cell(
            &input.physical_volume_recipe,
            &input.physical_volume,
            current,
        )
        .map_err(|_| TransportCertificateError::Dependency("physical cell replay failed"))?;
        let axis = match certified.face.axis {
            IntervalFaceAxisV1::X => 0,
            IntervalFaceAxisV1::Y => 1,
            IntervalFaceAxisV1::Z => 2,
        };
        let height = match certified.face.side {
            IntervalFaceSideV1::Minimum => physical_cell.min_q32_32[axis],
            IntervalFaceSideV1::Maximum => physical_cell.max_q32_32[axis],
        };
        let (forms, next_projection) = match direct_face(
            &input.cell,
            certified.face,
            height,
            direction.clone(),
            &mut work,
        ) {
            Ok(value) => value,
            Err(error) => {
                terminal = terminal_from_calc(error)?;
                break;
            }
        };
        if !containment(&next_projection.position_q160, &certified.point_q160)? {
            terminal = OriginAnchoredTransportTerminalV1::ProjectionOutOfRange;
            break;
        }
        let direct_form_id = domain_hash(
            FORM_DOMAIN,
            &json(&(
                input.cell.cell_id,
                step_index,
                certified.face,
                &forms,
                &next_projection,
            ))?,
        );
        let mut step = OriginAnchoredFaceStepV1 {
            step_index,
            current_cell: current,
            certified_face: certified.face,
            physical_input,
            physical_event: event,
            successor_cell: successor,
            direct_forms: forms,
            projection: next_projection.clone(),
            direct_form_id,
            arithmetic_receipt: work.receipt(),
            step_id: [0; 32],
            limitations: LIMITATIONS_V1.into(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        };
        step.step_id = domain_hash(STEP_DOMAIN, &json(&step)?);
        if json(&step)?.len() > MAX_FACE_STEP_BYTES {
            return Err(TransportCertificateError::ByteCeiling);
        }
        steps.push(step);
        if let Some(outcome) = event_terminal {
            terminal = outcome;
            break;
        }
        let Some(next) = successor else {
            return Err(TransportCertificateError::Dependency(
                "certified successor missing",
            ));
        };
        let next_evidence =
            build_physical_cell(&input.physical_volume_recipe, &input.physical_volume, next)
                .map_err(|_| TransportCertificateError::Dependency("successor replay failed"))?
                .evidence;
        if !evidence_equal(&physical_cell.evidence, &next_evidence) {
            terminal = OriginAnchoredTransportTerminalV1::InterfaceRequired;
            break;
        }
        current = next;
        projection = next_projection;
    }
    let mut certificate = OriginAnchoredTransportCertificateV1 {
        schema_version: 1,
        input_id,
        immutable_origin_cell_id: input.cell.cell_id,
        physical_volume_recipe_id: input.physical_volume_recipe.physical_volume_recipe_id,
        physical_volume_id: input.physical_volume.physical_volume_id,
        band_time_id: input.band_time_id,
        steps,
        terminal,
        aggregate_receipt: work.receipt(),
        certificate_id: [0; 32],
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    };
    certificate.certificate_id = domain_hash(CERTIFICATE_DOMAIN, &json(&certificate)?);
    let output_bytes = json(&certificate)?;
    if output_bytes.len() > MAX_CERTIFICATE_BYTES
        || input_bytes.len().saturating_add(output_bytes.len()) > MAX_AGGREGATE_LIVE_CANONICAL_BYTES
    {
        return Err(TransportCertificateError::ByteCeiling);
    }
    Ok(certificate)
}

pub fn validate_origin_anchored_transport_certificate(
    input: &OriginAnchoredTransportInputV1,
    certificate: &OriginAnchoredTransportCertificateV1,
) -> Result<(), TransportCertificateError> {
    let expected = compile_origin_anchored_transport(input)?;
    if certificate == &expected {
        Ok(())
    } else {
        Err(TransportCertificateError::IdentityMismatch)
    }
}

impl OriginAnchoredTransportCertificateV1 {
    pub fn to_bytes(
        &self,
        input: &OriginAnchoredTransportInputV1,
    ) -> Result<Vec<u8>, TransportCertificateError> {
        validate_origin_anchored_transport_certificate(input, self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_CERTIFICATE_BYTES {
            Err(TransportCertificateError::ByteCeiling)
        } else {
            Ok(bytes)
        }
    }
    pub fn from_bytes(
        input: &OriginAnchoredTransportInputV1,
        bytes: &[u8],
    ) -> Result<Self, TransportCertificateError> {
        if bytes.len() > MAX_CERTIFICATE_BYTES {
            return Err(TransportCertificateError::ByteCeiling);
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|_| TransportCertificateError::CodecDefect)?;
        if value.to_bytes(input)? != bytes {
            return Err(TransportCertificateError::CodecDefect);
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use optical_phase_space_cell_binding::{
        OpticalPhaseSpaceRootInputV1, PositiveRationalV1, compile_optical_phase_space_root,
    };
    use physical_path_substrate::{
        AdjacencyV1, BoundaryModeV1, CoordinateFrameV1, PhysicalVolumeRecipeInputV1,
        compile_physical_volume, compile_physical_volume_recipe,
    };

    fn form(role: PhaseSpaceOutputRoleV1, center: u128, small: u128) -> CorrelatedAffineOutputV1 {
        CorrelatedAffineOutputV1 {
            role,
            center_numerator: center.to_string(),
            coefficient_numerators: [small, small - 1, small - 2, small - 3]
                .map(|value| value.to_string()),
            remainder_lower_numerator: "0".into(),
            remainder_upper_numerator: "0".into(),
        }
    }

    fn input_with_bits(maximum_steps: u8, bits: u32) -> OriginAnchoredTransportInputV1 {
        let denominator = (1_u128 << bits) - 1;
        let small = denominator / 16_384;
        let scope = [2; 32];
        let reconstruction = [3; 32];
        let cell = compile_optical_phase_space_root(&OpticalPhaseSpaceRootInputV1 {
            schema_version: 1,
            source_id: [1; 32],
            scope_id: scope,
            reconstruction_id: reconstruction,
            source_revision: 1,
            parameterization: PhaseSpaceParameterizationV1::TransverseAreaDirection4d,
            measure: PositiveRationalV1 {
                numerator: "1".into(),
                denominator: "1".into(),
            },
            form_denominator: denominator.to_string(),
            forms: [
                form(PhaseSpaceOutputRoleV1::PointX, denominator / 5, small),
                form(PhaseSpaceOutputRoleV1::PointY, denominator / 5, small),
                form(PhaseSpaceOutputRoleV1::PointZ, denominator / 5, small),
                form(
                    PhaseSpaceOutputRoleV1::DirectionX,
                    denominator * 9 / 10,
                    small / 8,
                ),
                form(
                    PhaseSpaceOutputRoleV1::DirectionY,
                    denominator * 3 / 5,
                    small / 8,
                ),
                form(
                    PhaseSpaceOutputRoleV1::DirectionZ,
                    denominator / 3,
                    small / 8,
                ),
            ],
        })
        .expect("cell");
        let recipe = compile_physical_volume_recipe(&PhysicalVolumeRecipeInputV1 {
            schema_version: 1,
            recipe_source_id: [4; 32],
            scope_id: scope,
            reconstruction_id: reconstruction,
            recipe_revision: 1,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            origin_q32_32: [0; 3],
            cell_step_q32_32: 1_i64 << 32,
            extent: [4, 4, 4],
            boundary_mode: BoundaryModeV1::BoundedAbsent,
            adjacency: AdjacencyV1::SharedFace6,
            default_evidence: CellEvidenceV1::Vacuum,
            column_runs: Vec::new(),
        })
        .expect("recipe");
        let volume = compile_physical_volume(&recipe).expect("volume");
        OriginAnchoredTransportInputV1 {
            schema_version: 1,
            cell,
            physical_volume_recipe: recipe,
            physical_volume: volume,
            current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
            band_time_id: [5; 32],
            maximum_steps,
        }
    }

    fn input(maximum_steps: u8) -> OriginAnchoredTransportInputV1 {
        input_with_bits(maximum_steps, 64)
    }

    #[test]
    fn three_face_origin_anchored_run_is_deterministic_and_bounded() {
        let input = input(3);
        let first = compile_origin_anchored_transport(&input).expect("certificate");
        assert_eq!(
            first.certificate_id,
            [
                0x3e, 0x56, 0xfe, 0xc3, 0x95, 0x1c, 0x34, 0xf7, 0xaa, 0x54, 0xdc, 0x3d, 0xc4, 0x69,
                0xbf, 0x5b, 0xa6, 0x2d, 0x28, 0x9d, 0xa4, 0x78, 0x5b, 0x03, 0xea, 0x63, 0xe3, 0x9b,
                0x4f, 0x69, 0x4f, 0x29,
            ]
        );
        let second = compile_origin_anchored_transport(&input).expect("certificate");
        assert_eq!(first, second);
        assert_eq!(
            first.terminal,
            OriginAnchoredTransportTerminalV1::WorkExhausted
        );
        assert_eq!(first.steps.len(), 3);
        assert_eq!(
            first
                .steps
                .iter()
                .map(|step| step.certified_face)
                .collect::<Vec<_>>(),
            vec![
                IntervalFaceV1 {
                    axis: IntervalFaceAxisV1::X,
                    side: IntervalFaceSideV1::Maximum
                },
                IntervalFaceV1 {
                    axis: IntervalFaceAxisV1::Y,
                    side: IntervalFaceSideV1::Maximum
                },
                IntervalFaceV1 {
                    axis: IntervalFaceAxisV1::X,
                    side: IntervalFaceSideV1::Maximum
                },
            ]
        );
        assert!(first.aggregate_receipt.observed_maximum_live_bits <= DERIVED_MAXIMUM_LIVE_BITS);
        assert_eq!(
            OriginAnchoredTransportCertificateV1::from_bytes(
                &input,
                &first.to_bytes(&input).expect("bytes")
            )
            .expect("decode"),
            first
        );
    }

    #[test]
    fn forged_identity_authority_and_order_fail_replay() {
        let input = input(3);
        let certificate = compile_origin_anchored_transport(&input).expect("certificate");
        let mut forged = certificate.clone();
        forged.authority_effect = "arrival_authorized".into();
        assert_eq!(
            validate_origin_anchored_transport_certificate(&input, &forged),
            Err(TransportCertificateError::IdentityMismatch)
        );
        let mut forged = certificate.clone();
        forged.steps.swap(0, 1);
        assert_eq!(
            validate_origin_anchored_transport_certificate(&input, &forged),
            Err(TransportCertificateError::IdentityMismatch)
        );
        let mut forged = certificate;
        forged.certificate_id = [9; 32];
        assert_eq!(
            validate_origin_anchored_transport_certificate(&input, &forged),
            Err(TransportCertificateError::IdentityMismatch)
        );
    }

    #[test]
    fn scalar_cap_and_codec_are_fail_closed() {
        let mut over = input(1);
        over.cell.forms[0].center_numerator = (1_u128 << 64).to_string();
        assert_eq!(
            over.to_bytes(),
            Err(TransportCertificateError::Dependency(
                "optical phase-space cell replay failed"
            ))
        );
        let valid = input(1);
        let mut bytes = valid.to_bytes().expect("bytes");
        bytes.extend_from_slice(b" ");
        assert_eq!(
            OriginAnchoredTransportInputV1::from_bytes(&bytes),
            Err(TransportCertificateError::CodecDefect)
        );
        for bits in 65..=70 {
            assert_eq!(
                input_with_bits(1, bits).to_bytes(),
                Err(TransportCertificateError::ArithmeticShieldExceeded)
            );
        }
    }

    #[test]
    fn same_medium_change_stops_at_typed_interface() {
        let mut input = input(2);
        input
            .physical_volume_recipe
            .input
            .column_runs
            .push(physical_path_substrate::ColumnRunV1 {
                x_index: 1,
                y_index: 0,
                z_start: 0,
                length: 1,
                evidence: CellEvidenceV1::Gas {
                    substance_source_id: [7; 32],
                },
            });
        input.physical_volume_recipe =
            compile_physical_volume_recipe(&input.physical_volume_recipe.input).expect("recipe");
        input.physical_volume =
            compile_physical_volume(&input.physical_volume_recipe).expect("volume");
        let certificate = compile_origin_anchored_transport(&input).expect("certificate");
        assert_eq!(
            certificate.terminal,
            OriginAnchoredTransportTerminalV1::InterfaceRequired
        );
        assert_eq!(certificate.steps.len(), 1);
    }
}
