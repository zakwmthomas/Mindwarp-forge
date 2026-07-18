#![deny(warnings)]
//! Capability-free provenance for a bounded four-symbol optical phase-space cell.

use fixed_interval_arithmetic::Signed512;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, fmt};

pub const MAX_DEPTH: u8 = 12;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str =
    "phase_space_partition_evidence_only_no_coupling_or_physical_correctness_claim";
const ROOT_CAP: usize = 16 * 1024;
const CELL_CAP: usize = 32 * 1024;
const SPLIT_QUERY_CAP: usize = 32 * 1024;
const SPLIT_RECEIPT_CAP: usize = 64 * 1024;
const PROJECTION_QUERY_CAP: usize = 32 * 1024;
const PROJECTION_RECEIPT_CAP: usize = 16 * 1024;
const ROOT_BITS: u16 = 192;
const LIVE_BITS: u16 = 368;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpticalPhaseSpaceCellError {
    InvalidSchema,
    InvalidProvenance,
    NoncanonicalDecimal,
    NoncanonicalRational,
    NoncanonicalForm,
    ReversedRemainder,
    IdentityMismatch,
    DepthLimit {
        retained_measure: PositiveRationalV1,
    },
    ProjectionOutOfRange,
    ByteCeiling,
    ResourceCeiling,
    ArithmeticShieldExceeded,
    ArithmeticDefect,
    CodecDefect,
}

impl fmt::Display for OpticalPhaseSpaceCellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for OpticalPhaseSpaceCellError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseSpaceParameterizationV1 {
    TransverseAreaDirection4d,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseSpaceOutputRoleV1 {
    PointX,
    PointY,
    PointZ,
    DirectionX,
    DirectionY,
    DirectionZ,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseSpaceParameterAxisV1 {
    U0,
    U1,
    U2,
    U3,
}
impl PhaseSpaceParameterAxisV1 {
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseSpaceSplitSideV1 {
    Lower,
    Upper,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpticalProjectionTargetV1 {
    ExistingOpticalIntervalSeamV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PositiveRationalV1 {
    pub numerator: String,
    pub denominator: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelatedAffineOutputV1 {
    pub role: PhaseSpaceOutputRoleV1,
    pub center_numerator: String,
    pub coefficient_numerators: [String; 4],
    pub remainder_lower_numerator: String,
    pub remainder_upper_numerator: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceRootInputV1 {
    pub schema_version: u16,
    pub source_id: [u8; 32],
    pub scope_id: [u8; 32],
    pub reconstruction_id: [u8; 32],
    pub source_revision: u32,
    pub parameterization: PhaseSpaceParameterizationV1,
    pub measure: PositiveRationalV1,
    pub form_denominator: String,
    pub forms: [CorrelatedAffineOutputV1; 6],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseSpaceSplitStepV1 {
    pub axis: PhaseSpaceParameterAxisV1,
    pub side: PhaseSpaceSplitSideV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceCellV1 {
    pub schema_version: u16,
    pub source_id: [u8; 32],
    pub scope_id: [u8; 32],
    pub reconstruction_id: [u8; 32],
    pub source_revision: u32,
    pub parameterization: PhaseSpaceParameterizationV1,
    pub root_id: [u8; 32],
    pub parent_id: Option<[u8; 32]>,
    pub depth: u8,
    pub path: Vec<PhaseSpaceSplitStepV1>,
    pub measure: PositiveRationalV1,
    pub form_denominator: String,
    pub forms: [CorrelatedAffineOutputV1; 6],
    pub cell_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceSplitQueryV1 {
    pub schema_version: u16,
    pub cell: OpticalPhaseSpaceCellV1,
    pub axis: PhaseSpaceParameterAxisV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseSpaceArithmeticReceiptV1 {
    pub maximum_live_magnitude_bits: u16,
    pub shifts: u16,
    pub additions_subtractions: u16,
    pub gcd_checks: u16,
    pub directed_divisions: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceSplitReceiptV1 {
    pub schema_version: u16,
    pub parent_id: [u8; 32],
    pub axis: PhaseSpaceParameterAxisV1,
    pub children: [OpticalPhaseSpaceCellV1; 2],
    pub parent_measure: PositiveRationalV1,
    pub child_measures: [PositiveRationalV1; 2],
    pub arithmetic_receipt: PhaseSpaceArithmeticReceiptV1,
    pub split_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceProjectionQueryV1 {
    pub schema_version: u16,
    pub cell: OpticalPhaseSpaceCellV1,
    pub target: OpticalProjectionTargetV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectedFixedIntervalV1 {
    pub fractional_bits: u16,
    pub lower: String,
    pub upper: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalPhaseSpaceProjectionReceiptV1 {
    pub schema_version: u16,
    pub cell_id: [u8; 32],
    pub target: OpticalProjectionTargetV1,
    pub position_intervals: [DirectedFixedIntervalV1; 3],
    pub direction_intervals: [DirectedFixedIntervalV1; 3],
    pub form_denominator_hash: [u8; 32],
    pub arithmetic_receipt: PhaseSpaceArithmeticReceiptV1,
    pub projection_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BigNat(Vec<u8>);
impl BigNat {
    fn parse(value: &str) -> Option<Self> {
        let digits = value.strip_prefix('-').unwrap_or(value);
        if digits.is_empty()
            || (digits.len() > 1 && digits.starts_with('0'))
            || !digits.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        Some(Self(digits.bytes().map(|b| b - b'0').collect()))
    }
    fn zero() -> Self {
        Self(vec![0])
    }
    fn one() -> Self {
        Self(vec![1])
    }
    fn is_zero(&self) -> bool {
        self.0 == [0]
    }
    fn trim(mut self) -> Self {
        while self.0.len() > 1 && self.0[0] == 0 {
            self.0.remove(0);
        }
        self
    }
    fn cmp_nat(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
    fn sub(&self, other: &Self) -> Self {
        let mut out = self.0.clone();
        let mut borrow = 0_i16;
        let offset = out.len() - other.0.len();
        for index in (0..out.len()).rev() {
            let rhs = if index >= offset {
                i16::from(other.0[index - offset])
            } else {
                0
            };
            let mut value = i16::from(out[index]) - rhs - borrow;
            if value < 0 {
                value += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out[index] = value as u8;
        }
        Self(out).trim()
    }
    fn mul10_add(&self, digit: u8) -> Self {
        let mut out = if self.is_zero() {
            Vec::new()
        } else {
            self.0.clone()
        };
        out.push(digit);
        Self(out).trim()
    }
    fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        let mut quotient = Vec::with_capacity(self.0.len());
        let mut remainder = Self::zero();
        for digit in &self.0 {
            remainder = remainder.mul10_add(*digit);
            let mut q = 0;
            while remainder.cmp_nat(divisor) != Ordering::Less {
                remainder = remainder.sub(divisor);
                q += 1;
            }
            quotient.push(q);
        }
        (Self(quotient).trim(), remainder)
    }
    fn remainder(&self, divisor: &Self) -> Self {
        self.div_rem(divisor).1
    }
    fn decimal(&self) -> String {
        self.0.iter().map(|d| char::from(b'0' + d)).collect()
    }
}

fn gcd(mut left: BigNat, mut right: BigNat) -> BigNat {
    while !right.is_zero() {
        let next = left.remainder(&right);
        left = right;
        right = next;
    }
    left
}
fn collective_gcd(denominator: &str, forms: &[CorrelatedAffineOutputV1; 6]) -> BigNat {
    let mut value = BigNat::parse(denominator).expect("validated denominator");
    for form in forms {
        for scalar in std::iter::once(&form.center_numerator)
            .chain(form.coefficient_numerators.iter())
            .chain([
                &form.remainder_lower_numerator,
                &form.remainder_upper_numerator,
            ])
        {
            value = gcd(value, BigNat::parse(scalar).expect("validated scalar"));
            if value == BigNat::one() {
                return value;
            }
        }
    }
    value
}
fn divide_decimal(value: &str, divisor: &BigNat) -> String {
    let negative = value.starts_with('-');
    let magnitude = BigNat::parse(value).expect("validated decimal");
    let (quotient, remainder) = magnitude.div_rem(divisor);
    debug_assert!(remainder.is_zero());
    let text = quotient.decimal();
    if negative && text != "0" {
        format!("-{text}")
    } else {
        text
    }
}

fn signed(value: &str, bits: u16) -> Result<Signed512, OpticalPhaseSpaceCellError> {
    let parsed = Signed512::from_canonical_decimal(value)
        .map_err(|_| OpticalPhaseSpaceCellError::NoncanonicalDecimal)?;
    if parsed.maximum_magnitude_bits() > bits {
        return Err(if bits == ROOT_BITS {
            OpticalPhaseSpaceCellError::ResourceCeiling
        } else {
            OpticalPhaseSpaceCellError::ArithmeticShieldExceeded
        });
    }
    Ok(parsed)
}
fn positive(value: &str, bits: u16) -> Result<Signed512, OpticalPhaseSpaceCellError> {
    let parsed = signed(value, bits)?;
    if parsed.is_negative() || parsed == Signed512::zero() {
        return Err(OpticalPhaseSpaceCellError::NoncanonicalDecimal);
    }
    Ok(parsed)
}
fn validate_rational(
    value: &PositiveRationalV1,
    bits: u16,
) -> Result<(), OpticalPhaseSpaceCellError> {
    positive(&value.numerator, bits)
        .map_err(|_| OpticalPhaseSpaceCellError::NoncanonicalRational)?;
    positive(&value.denominator, bits)
        .map_err(|_| OpticalPhaseSpaceCellError::NoncanonicalRational)?;
    if gcd(
        BigNat::parse(&value.numerator).unwrap(),
        BigNat::parse(&value.denominator).unwrap(),
    ) != BigNat::one()
    {
        return Err(OpticalPhaseSpaceCellError::NoncanonicalRational);
    }
    Ok(())
}
fn validate_forms(
    denominator: &str,
    forms: &[CorrelatedAffineOutputV1; 6],
    bits: u16,
) -> Result<(), OpticalPhaseSpaceCellError> {
    positive(denominator, bits)?;
    let roles = [
        PhaseSpaceOutputRoleV1::PointX,
        PhaseSpaceOutputRoleV1::PointY,
        PhaseSpaceOutputRoleV1::PointZ,
        PhaseSpaceOutputRoleV1::DirectionX,
        PhaseSpaceOutputRoleV1::DirectionY,
        PhaseSpaceOutputRoleV1::DirectionZ,
    ];
    for (form, role) in forms.iter().zip(roles) {
        if form.role != role {
            return Err(OpticalPhaseSpaceCellError::NoncanonicalForm);
        }
        let center = signed(&form.center_numerator, bits)?;
        let _ = center;
        for coefficient in &form.coefficient_numerators {
            signed(coefficient, bits)?;
        }
        let lower = signed(&form.remainder_lower_numerator, bits)?;
        let upper = signed(&form.remainder_upper_numerator, bits)?;
        if lower > upper {
            return Err(OpticalPhaseSpaceCellError::ReversedRemainder);
        }
    }
    if collective_gcd(denominator, forms) != BigNat::one() {
        return Err(OpticalPhaseSpaceCellError::NoncanonicalForm);
    }
    Ok(())
}
fn provenance(
    source: &[u8; 32],
    scope: &[u8; 32],
    reconstruction: &[u8; 32],
    revision: u32,
) -> Result<(), OpticalPhaseSpaceCellError> {
    if source.iter().all(|b| *b == 0)
        || scope.iter().all(|b| *b == 0)
        || reconstruction.iter().all(|b| *b == 0)
        || revision == 0
    {
        Err(OpticalPhaseSpaceCellError::InvalidProvenance)
    } else {
        Ok(())
    }
}
fn hash(domain: &str, bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    h.update([0]);
    h.update(bytes);
    h.finalize().into()
}
fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
    serde_json::to_vec(value).map_err(|_| OpticalPhaseSpaceCellError::CodecDefect)
}

#[derive(Serialize)]
struct RootIdentity<'a> {
    input: &'a OpticalPhaseSpaceRootInputV1,
    limitations: &'a str,
    authority_effect: &'a str,
}
fn root_id(input: &OpticalPhaseSpaceRootInputV1) -> Result<[u8; 32], OpticalPhaseSpaceCellError> {
    Ok(hash(
        "mindwarp.optical-phase-space.root.v1",
        &json(&RootIdentity {
            input,
            limitations: LIMITATIONS_V1,
            authority_effect: AUTHORITY_EFFECT_NONE,
        })?,
    ))
}
fn child_id(cell: &OpticalPhaseSpaceCellV1) -> Result<[u8; 32], OpticalPhaseSpaceCellError> {
    let mut payload = cell.clone();
    payload.cell_id = [0; 32];
    Ok(hash(
        "mindwarp.optical-phase-space.cell.v1",
        &json(&payload)?,
    ))
}

pub fn compile_optical_phase_space_root(
    input: &OpticalPhaseSpaceRootInputV1,
) -> Result<OpticalPhaseSpaceCellV1, OpticalPhaseSpaceCellError> {
    if json(input)?.len() > ROOT_CAP {
        return Err(OpticalPhaseSpaceCellError::ByteCeiling);
    }
    if input.schema_version != 1 {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    provenance(
        &input.source_id,
        &input.scope_id,
        &input.reconstruction_id,
        input.source_revision,
    )?;
    validate_rational(&input.measure, ROOT_BITS)?;
    validate_forms(&input.form_denominator, &input.forms, ROOT_BITS)?;
    let identity = root_id(input)?;
    Ok(OpticalPhaseSpaceCellV1 {
        schema_version: 1,
        source_id: input.source_id,
        scope_id: input.scope_id,
        reconstruction_id: input.reconstruction_id,
        source_revision: input.source_revision,
        parameterization: input.parameterization,
        root_id: identity,
        parent_id: None,
        depth: 0,
        path: Vec::new(),
        measure: input.measure.clone(),
        form_denominator: input.form_denominator.clone(),
        forms: input.forms.clone(),
        cell_id: identity,
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    })
}

fn validate_cell(cell: &OpticalPhaseSpaceCellV1) -> Result<(), OpticalPhaseSpaceCellError> {
    if cell.schema_version != 1 {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    provenance(
        &cell.source_id,
        &cell.scope_id,
        &cell.reconstruction_id,
        cell.source_revision,
    )?;
    if cell.depth > MAX_DEPTH || cell.path.len() != usize::from(cell.depth) {
        return Err(OpticalPhaseSpaceCellError::ResourceCeiling);
    }
    if cell.limitations != LIMITATIONS_V1 || cell.authority_effect != AUTHORITY_EFFECT_NONE {
        return Err(OpticalPhaseSpaceCellError::InvalidProvenance);
    }
    validate_rational(&cell.measure, LIVE_BITS)?;
    validate_forms(&cell.form_denominator, &cell.forms, LIVE_BITS)?;
    if cell.depth == 0 {
        if cell.parent_id.is_some() || cell.cell_id != cell.root_id {
            return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
        }
        let input = OpticalPhaseSpaceRootInputV1 {
            schema_version: 1,
            source_id: cell.source_id,
            scope_id: cell.scope_id,
            reconstruction_id: cell.reconstruction_id,
            source_revision: cell.source_revision,
            parameterization: cell.parameterization,
            measure: cell.measure.clone(),
            form_denominator: cell.form_denominator.clone(),
            forms: cell.forms.clone(),
        };
        if root_id(&input)? != cell.root_id {
            return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
        }
    } else if cell.parent_id.is_none() || child_id(cell)? != cell.cell_id {
        return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
    }
    Ok(())
}

fn normalize_forms(
    mut denominator: String,
    mut forms: [CorrelatedAffineOutputV1; 6],
) -> (String, [CorrelatedAffineOutputV1; 6]) {
    let divisor = collective_gcd(&denominator, &forms);
    if divisor != BigNat::one() {
        denominator = divide_decimal(&denominator, &divisor);
        for form in &mut forms {
            form.center_numerator = divide_decimal(&form.center_numerator, &divisor);
            for coefficient in &mut form.coefficient_numerators {
                *coefficient = divide_decimal(coefficient, &divisor);
            }
            form.remainder_lower_numerator =
                divide_decimal(&form.remainder_lower_numerator, &divisor);
            form.remainder_upper_numerator =
                divide_decimal(&form.remainder_upper_numerator, &divisor);
        }
    }
    (denominator, forms)
}
fn half_measure(
    value: &PositiveRationalV1,
) -> Result<PositiveRationalV1, OpticalPhaseSpaceCellError> {
    let numerator = BigNat::parse(&value.numerator).unwrap();
    let denominator = BigNat::parse(&value.denominator)
        .unwrap()
        .mul10_add(0)
        .div_rem(&BigNat::parse("5").unwrap())
        .0;
    let divisor = gcd(numerator.clone(), denominator.clone());
    Ok(PositiveRationalV1 {
        numerator: numerator.div_rem(&divisor).0.decimal(),
        denominator: denominator.div_rem(&divisor).0.decimal(),
    })
}

pub fn split_optical_phase_space_cell(
    query: &OpticalPhaseSpaceSplitQueryV1,
) -> Result<OpticalPhaseSpaceSplitReceiptV1, OpticalPhaseSpaceCellError> {
    if query.schema_version != 1 {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    validate_cell(&query.cell)?;
    if query.cell.depth == MAX_DEPTH {
        return Err(OpticalPhaseSpaceCellError::DepthLimit {
            retained_measure: query.cell.measure.clone(),
        });
    }
    let axis = query.axis.index();
    let denominator = positive(&query.cell.form_denominator, LIVE_BITS)?
        .checked_shl(1)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
        .canonical_decimal();
    let measure = half_measure(&query.cell.measure)?;
    let mut children = Vec::with_capacity(2);
    let mut maximum = 0;
    for side in [PhaseSpaceSplitSideV1::Lower, PhaseSpaceSplitSideV1::Upper] {
        let mut forms = query.cell.forms.clone();
        for form in &mut forms {
            let center = signed(&form.center_numerator, LIVE_BITS)?
                .checked_shl(1)
                .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
            let selected = signed(&form.coefficient_numerators[axis], LIVE_BITS)?;
            let derived = if side == PhaseSpaceSplitSideV1::Lower {
                center.checked_sub(&selected)
            } else {
                center.checked_add(&selected)
            }
            .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
            form.center_numerator = derived.canonical_decimal();
            maximum = maximum.max(derived.maximum_magnitude_bits());
            for (index, coefficient) in form.coefficient_numerators.iter_mut().enumerate() {
                if index != axis {
                    *coefficient = signed(coefficient, LIVE_BITS)?
                        .checked_shl(1)
                        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
                        .canonical_decimal();
                }
            }
            form.remainder_lower_numerator = signed(&form.remainder_lower_numerator, LIVE_BITS)?
                .checked_shl(1)
                .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
                .canonical_decimal();
            form.remainder_upper_numerator = signed(&form.remainder_upper_numerator, LIVE_BITS)?
                .checked_shl(1)
                .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
                .canonical_decimal();
        }
        let (child_denominator, child_forms) = normalize_forms(denominator.clone(), forms);
        validate_forms(&child_denominator, &child_forms, LIVE_BITS)?;
        let mut path = query.cell.path.clone();
        path.push(PhaseSpaceSplitStepV1 {
            axis: query.axis,
            side,
        });
        let mut child = OpticalPhaseSpaceCellV1 {
            schema_version: 1,
            source_id: query.cell.source_id,
            scope_id: query.cell.scope_id,
            reconstruction_id: query.cell.reconstruction_id,
            source_revision: query.cell.source_revision,
            parameterization: query.cell.parameterization,
            root_id: query.cell.root_id,
            parent_id: Some(query.cell.cell_id),
            depth: query.cell.depth + 1,
            path,
            measure: measure.clone(),
            form_denominator: child_denominator,
            forms: child_forms,
            cell_id: [0; 32],
            limitations: LIMITATIONS_V1.into(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        };
        child.cell_id = child_id(&child)?;
        children.push(child);
    }
    let children: [OpticalPhaseSpaceCellV1; 2] = children
        .try_into()
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let arithmetic_receipt = PhaseSpaceArithmeticReceiptV1 {
        maximum_live_magnitude_bits: maximum,
        shifts: 74,
        additions_subtractions: 12,
        gcd_checks: 86,
        directed_divisions: 0,
    };
    let payload = (
        &query.cell.cell_id,
        query.axis,
        [&children[0].cell_id, &children[1].cell_id],
        &query.cell.measure,
        [&measure, &measure],
        &arithmetic_receipt,
        LIMITATIONS_V1,
        AUTHORITY_EFFECT_NONE,
    );
    let split_id = hash("mindwarp.optical-phase-space.split.v1", &json(&payload)?);
    Ok(OpticalPhaseSpaceSplitReceiptV1 {
        schema_version: 1,
        parent_id: query.cell.cell_id,
        axis: query.axis,
        children,
        parent_measure: query.cell.measure.clone(),
        child_measures: [measure.clone(), measure],
        arithmetic_receipt,
        split_id,
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    })
}

fn interval(
    form: &CorrelatedAffineOutputV1,
    denominator: &Signed512,
    fractional_bits: u16,
) -> Result<(DirectedFixedIntervalV1, u16), OpticalPhaseSpaceCellError> {
    let center = signed(&form.center_numerator, LIVE_BITS)?;
    let mut radius = Signed512::zero();
    for coefficient in &form.coefficient_numerators {
        let value = signed(coefficient, LIVE_BITS)?;
        let magnitude = if value.is_negative() {
            value.checked_neg()
        } else {
            value
        };
        radius = radius
            .checked_add(&magnitude)
            .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    }
    let remainder_lower = signed(&form.remainder_lower_numerator, LIVE_BITS)?;
    let remainder_upper = signed(&form.remainder_upper_numerator, LIVE_BITS)?;
    let lower = center
        .checked_sub(&radius)
        .and_then(|v| v.checked_add(&remainder_lower))
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let upper = center
        .checked_add(&radius)
        .and_then(|v| v.checked_add(&remainder_upper))
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let shifted_lower = lower
        .checked_shl(fractional_bits)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let shifted_upper = upper
        .checked_shl(fractional_bits)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let maximum = shifted_lower
        .maximum_magnitude_bits()
        .max(shifted_upper.maximum_magnitude_bits());
    if maximum > LIVE_BITS {
        return Err(OpticalPhaseSpaceCellError::ArithmeticShieldExceeded);
    }
    let lower = shifted_lower
        .div_floor(denominator)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let upper = shifted_upper
        .div_ceil(denominator)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    Ok((
        DirectedFixedIntervalV1 {
            fractional_bits,
            lower: lower.canonical_decimal(),
            upper: upper.canonical_decimal(),
        },
        maximum,
    ))
}

pub fn project_optical_phase_space_cell(
    query: &OpticalPhaseSpaceProjectionQueryV1,
) -> Result<OpticalPhaseSpaceProjectionReceiptV1, OpticalPhaseSpaceCellError> {
    if query.schema_version != 1 {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    validate_cell(&query.cell)?;
    let denominator = positive(&query.cell.form_denominator, LIVE_BITS)?;
    let mut intervals = Vec::with_capacity(6);
    let mut maximum = 0;
    for (index, form) in query.cell.forms.iter().enumerate() {
        let bits = if index < 3 { 160 } else { 62 };
        let (value, used) = interval(form, &denominator, bits)?;
        maximum = maximum.max(used);
        intervals.push(value);
    }
    let position_intervals = intervals[..3]
        .to_vec()
        .try_into()
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let direction_intervals: [DirectedFixedIntervalV1; 3] = intervals[3..]
        .to_vec()
        .try_into()
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let bound = Signed512::one()
        .checked_shl(62)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    for value in &direction_intervals {
        if signed(&value.lower, LIVE_BITS)? < bound.checked_neg()
            || signed(&value.upper, LIVE_BITS)? > bound
        {
            return Err(OpticalPhaseSpaceCellError::ProjectionOutOfRange);
        }
    }
    let form_denominator_hash = Sha256::digest(query.cell.form_denominator.as_bytes()).into();
    let arithmetic_receipt = PhaseSpaceArithmeticReceiptV1 {
        maximum_live_magnitude_bits: maximum,
        shifts: 6,
        additions_subtractions: 42,
        gcd_checks: 0,
        directed_divisions: 12,
    };
    let payload = (
        &query.cell.cell_id,
        query.target,
        &position_intervals,
        &direction_intervals,
        form_denominator_hash,
        &arithmetic_receipt,
        LIMITATIONS_V1,
        AUTHORITY_EFFECT_NONE,
    );
    let projection_id = hash(
        "mindwarp.optical-phase-space.projection.v1",
        &json(&payload)?,
    );
    Ok(OpticalPhaseSpaceProjectionReceiptV1 {
        schema_version: 1,
        cell_id: query.cell.cell_id,
        target: query.target,
        position_intervals,
        direction_intervals,
        form_denominator_hash,
        arithmetic_receipt,
        projection_id,
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    })
}

pub fn correlated_difference_interval(
    cell: &OpticalPhaseSpaceCellV1,
    left: usize,
    right: usize,
) -> Result<(String, String), OpticalPhaseSpaceCellError> {
    validate_cell(cell)?;
    let a = cell
        .forms
        .get(left)
        .ok_or(OpticalPhaseSpaceCellError::ResourceCeiling)?;
    let b = cell
        .forms
        .get(right)
        .ok_or(OpticalPhaseSpaceCellError::ResourceCeiling)?;
    let mut center = signed(&a.center_numerator, LIVE_BITS)?
        .checked_sub(&signed(&b.center_numerator, LIVE_BITS)?)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let mut radius = Signed512::zero();
    for index in 0..4 {
        let coefficient = signed(&a.coefficient_numerators[index], LIVE_BITS)?
            .checked_sub(&signed(&b.coefficient_numerators[index], LIVE_BITS)?)
            .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
        let magnitude = if coefficient.is_negative() {
            coefficient.checked_neg()
        } else {
            coefficient
        };
        radius = radius
            .checked_add(&magnitude)
            .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    }
    let lower_remainder = signed(&a.remainder_lower_numerator, LIVE_BITS)?
        .checked_sub(&signed(&b.remainder_upper_numerator, LIVE_BITS)?)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let upper_remainder = signed(&a.remainder_upper_numerator, LIVE_BITS)?
        .checked_sub(&signed(&b.remainder_lower_numerator, LIVE_BITS)?)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let denominator = positive(&cell.form_denominator, LIVE_BITS)?;
    let lower = center
        .checked_sub(&radius)
        .and_then(|v| v.checked_add(&lower_remainder))
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
        .div_floor(&denominator)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    center = center
        .checked_add(&radius)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    let upper = center
        .checked_add(&upper_remainder)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?
        .div_ceil(&denominator)
        .map_err(|_| OpticalPhaseSpaceCellError::ArithmeticDefect)?;
    Ok((lower.canonical_decimal(), upper.canonical_decimal()))
}

impl OpticalPhaseSpaceCellV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        validate_cell(self)?;
        let bytes = json(self)?;
        if bytes.len() > CELL_CAP {
            Err(OpticalPhaseSpaceCellError::ByteCeiling)
        } else {
            Ok(bytes)
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        if bytes.len() > CELL_CAP {
            return Err(OpticalPhaseSpaceCellError::ByteCeiling);
        }
        let value: Self =
            serde_json::from_slice(bytes).map_err(|_| OpticalPhaseSpaceCellError::CodecDefect)?;
        validate_cell(&value)?;
        if json(&value)? != bytes {
            return Err(OpticalPhaseSpaceCellError::CodecDefect);
        }
        Ok(value)
    }
}

fn strict_decode<T: for<'de> Deserialize<'de> + Serialize>(
    bytes: &[u8],
    cap: usize,
) -> Result<T, OpticalPhaseSpaceCellError> {
    if bytes.len() > cap {
        return Err(OpticalPhaseSpaceCellError::ByteCeiling);
    }
    let value =
        serde_json::from_slice(bytes).map_err(|_| OpticalPhaseSpaceCellError::CodecDefect)?;
    if json(&value)? != bytes {
        return Err(OpticalPhaseSpaceCellError::CodecDefect);
    }
    Ok(value)
}

fn capped_json<T: Serialize>(value: &T, cap: usize) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
    let bytes = json(value)?;
    if bytes.len() > cap {
        Err(OpticalPhaseSpaceCellError::ByteCeiling)
    } else {
        Ok(bytes)
    }
}

impl OpticalPhaseSpaceRootInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        compile_optical_phase_space_root(self)?;
        capped_json(self, ROOT_CAP)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        let value: Self = strict_decode(bytes, ROOT_CAP)?;
        compile_optical_phase_space_root(&value)?;
        Ok(value)
    }
}

impl OpticalPhaseSpaceSplitQueryV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        if self.schema_version != 1 {
            return Err(OpticalPhaseSpaceCellError::InvalidSchema);
        }
        validate_cell(&self.cell)?;
        capped_json(self, SPLIT_QUERY_CAP)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        let value: Self = strict_decode(bytes, SPLIT_QUERY_CAP)?;
        if value.schema_version != 1 {
            return Err(OpticalPhaseSpaceCellError::InvalidSchema);
        }
        validate_cell(&value.cell)?;
        Ok(value)
    }
}

fn validate_split_receipt(
    value: &OpticalPhaseSpaceSplitReceiptV1,
) -> Result<(), OpticalPhaseSpaceCellError> {
    if value.schema_version != 1
        || value.limitations != LIMITATIONS_V1
        || value.authority_effect != AUTHORITY_EFFECT_NONE
    {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    for child in &value.children {
        validate_cell(child)?;
        if child.parent_id != Some(value.parent_id) {
            return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
        }
    }
    if value.children[0]
        .path
        .last()
        .map(|step| (step.axis, step.side))
        != Some((value.axis, PhaseSpaceSplitSideV1::Lower))
        || value.children[1]
            .path
            .last()
            .map(|step| (step.axis, step.side))
            != Some((value.axis, PhaseSpaceSplitSideV1::Upper))
        || value.child_measures
            != [
                value.children[0].measure.clone(),
                value.children[1].measure.clone(),
            ]
    {
        return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
    }
    let payload = (
        &value.parent_id,
        value.axis,
        [&value.children[0].cell_id, &value.children[1].cell_id],
        &value.parent_measure,
        [&value.child_measures[0], &value.child_measures[1]],
        &value.arithmetic_receipt,
        LIMITATIONS_V1,
        AUTHORITY_EFFECT_NONE,
    );
    if hash("mindwarp.optical-phase-space.split.v1", &json(&payload)?) != value.split_id {
        return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
    }
    Ok(())
}

impl OpticalPhaseSpaceSplitReceiptV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        validate_split_receipt(self)?;
        capped_json(self, SPLIT_RECEIPT_CAP)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        let value: Self = strict_decode(bytes, SPLIT_RECEIPT_CAP)?;
        validate_split_receipt(&value)?;
        Ok(value)
    }
}

impl OpticalPhaseSpaceProjectionQueryV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        if self.schema_version != 1 {
            return Err(OpticalPhaseSpaceCellError::InvalidSchema);
        }
        validate_cell(&self.cell)?;
        capped_json(self, PROJECTION_QUERY_CAP)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        let value: Self = strict_decode(bytes, PROJECTION_QUERY_CAP)?;
        if value.schema_version != 1 {
            return Err(OpticalPhaseSpaceCellError::InvalidSchema);
        }
        validate_cell(&value.cell)?;
        Ok(value)
    }
}

fn validate_projection_receipt(
    value: &OpticalPhaseSpaceProjectionReceiptV1,
) -> Result<(), OpticalPhaseSpaceCellError> {
    if value.schema_version != 1
        || value.limitations != LIMITATIONS_V1
        || value.authority_effect != AUTHORITY_EFFECT_NONE
    {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    for interval in value
        .position_intervals
        .iter()
        .chain(value.direction_intervals.iter())
    {
        let lower = signed(&interval.lower, LIVE_BITS)?;
        let upper = signed(&interval.upper, LIVE_BITS)?;
        if lower > upper {
            return Err(OpticalPhaseSpaceCellError::ReversedRemainder);
        }
    }
    if value
        .position_intervals
        .iter()
        .any(|v| v.fractional_bits != 160)
        || value
            .direction_intervals
            .iter()
            .any(|v| v.fractional_bits != 62)
    {
        return Err(OpticalPhaseSpaceCellError::InvalidSchema);
    }
    let payload = (
        &value.cell_id,
        value.target,
        &value.position_intervals,
        &value.direction_intervals,
        value.form_denominator_hash,
        &value.arithmetic_receipt,
        LIMITATIONS_V1,
        AUTHORITY_EFFECT_NONE,
    );
    if hash(
        "mindwarp.optical-phase-space.projection.v1",
        &json(&payload)?,
    ) != value.projection_id
    {
        return Err(OpticalPhaseSpaceCellError::IdentityMismatch);
    }
    Ok(())
}

impl OpticalPhaseSpaceProjectionReceiptV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalPhaseSpaceCellError> {
        validate_projection_receipt(self)?;
        capped_json(self, PROJECTION_RECEIPT_CAP)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalPhaseSpaceCellError> {
        let value: Self = strict_decode(bytes, PROJECTION_RECEIPT_CAP)?;
        validate_projection_receipt(&value)?;
        Ok(value)
    }
}
