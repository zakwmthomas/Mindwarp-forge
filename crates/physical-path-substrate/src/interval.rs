use fixed_interval_arithmetic::{FixedArithmeticError, Signed512};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    CellEvidenceV1, CellIndex3V1, Id, PhysicalPathError, PhysicalVolumeRecipeV1, PhysicalVolumeV1,
    build_physical_cell, validate_physical_volume,
};

pub const INTERVAL_CELL_STEP_CONTRACT_VERSION: u16 = 1;
pub const INTERVAL_CELL_STEP_FRACTIONAL_BITS: u16 = 160;
pub const MAX_INTERVAL_CELL_STEP_INPUT_BYTES: usize = 16 * 1024;
pub const MAX_INTERVAL_CELL_STEP_EVENT_BYTES: usize = 32 * 1024;
pub const INTERVAL_CELL_STEP_DERIVED_MAXIMUM_LIVE_BITS: u16 = 414;

const INPUT_DOMAIN: &[u8] = b"mindwarp.physical-path.interval-cell-step-input.v1";
const EVENT_DOMAIN: &[u8] = b"mindwarp.physical-path.interval-cell-step-event.v1";
const DIRECTION_FRACTIONAL_BITS: u16 = 62;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionalIntervalEvidenceKindV1 {
    DeclaredConditionalPointDirectionBox,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedDecimalIntervalV1 {
    pub fractional_bits: u16,
    pub lower: String,
    pub upper: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalIntervalCellStepInputV1 {
    pub schema_version: u16,
    pub state_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub state_revision: u32,
    pub evidence_kind: ConditionalIntervalEvidenceKindV1,
    pub physical_volume_recipe_id: Id,
    pub physical_volume_id: Id,
    pub current_cell: CellIndex3V1,
    pub point_q160: [SignedDecimalIntervalV1; 3],
    pub direction_q1_62: [SignedDecimalIntervalV1; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalFaceAxisV1 {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalFaceSideV1 {
    Minimum,
    Maximum,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalFaceV1 {
    pub axis: IntervalFaceAxisV1,
    pub side: IntervalFaceSideV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedIntervalCellFaceV1 {
    pub face: IntervalFaceV1,
    pub neighbor: Option<CellIndex3V1>,
    pub time_q160: SignedDecimalIntervalV1,
    pub point_q160: [SignedDecimalIntervalV1; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConditionalIntervalCellStepOutcomeV1 {
    CertifiedNextFace {
        certified: CertifiedIntervalCellFaceV1,
        neighbor_evidence: CellEvidenceV1,
    },
    AmbiguousNextFace,
    NoForwardProgress,
    OuterDomainExit {
        certified: CertifiedIntervalCellFaceV1,
    },
    UnavailableNeighbor {
        certified: CertifiedIntervalCellFaceV1,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalCellStepArithmeticReceiptV1 {
    pub fractional_bits: u16,
    pub storage_bits: u16,
    pub derived_maximum_live_bits: u16,
    pub observed_maximum_live_bits: u16,
    pub possible_face_count: u8,
    pub directed_division_count: u8,
    pub strict_face_order_comparison_count: u8,
    pub propagated_tangential_axes: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalIntervalCellStepEventV1 {
    pub schema_version: u16,
    pub interval_cell_step_input_id: Id,
    pub physical_volume_recipe_id: Id,
    pub physical_volume_id: Id,
    pub current_cell: CellIndex3V1,
    pub outcome: ConditionalIntervalCellStepOutcomeV1,
    pub arithmetic_receipt: IntervalCellStepArithmeticReceiptV1,
    pub interval_cell_step_event_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

trait PhysicalSigned512: Sized {
    fn bits(&self) -> u16;
    fn neg(&self) -> Self;
    fn add(&self, other: &Self) -> Result<Self, PhysicalPathError>;
    fn sub(&self, other: &Self) -> Result<Self, PhysicalPathError>;
    fn mul(&self, other: &Self) -> Result<Self, PhysicalPathError>;
    fn shl(&self, bits: u16) -> Result<Self, PhysicalPathError>;
    fn physical_div_floor(&self, denominator: &Self) -> Result<Self, PhysicalPathError>;
    fn physical_div_ceil(&self, denominator: &Self) -> Result<Self, PhysicalPathError>;
    fn decimal(&self) -> String;
}

impl PhysicalSigned512 for Signed512 {
    fn bits(&self) -> u16 {
        self.maximum_magnitude_bits()
    }
    fn neg(&self) -> Self {
        self.checked_neg()
    }
    fn add(&self, other: &Self) -> Result<Self, PhysicalPathError> {
        shield(self.checked_add(other).map_err(map_arithmetic_error)?)
    }
    fn sub(&self, other: &Self) -> Result<Self, PhysicalPathError> {
        shield(self.checked_sub(other).map_err(map_arithmetic_error)?)
    }
    fn mul(&self, other: &Self) -> Result<Self, PhysicalPathError> {
        shield(self.checked_mul(other).map_err(map_arithmetic_error)?)
    }
    fn shl(&self, bits: u16) -> Result<Self, PhysicalPathError> {
        shield(self.checked_shl(bits).map_err(map_arithmetic_error)?)
    }
    fn physical_div_floor(&self, denominator: &Self) -> Result<Self, PhysicalPathError> {
        shield(self.div_floor(denominator).map_err(map_arithmetic_error)?)
    }
    fn physical_div_ceil(&self, denominator: &Self) -> Result<Self, PhysicalPathError> {
        shield(self.div_ceil(denominator).map_err(map_arithmetic_error)?)
    }
    fn decimal(&self) -> String {
        self.canonical_decimal()
    }
}

fn map_arithmetic_error(error: FixedArithmeticError) -> PhysicalPathError {
    match error {
        FixedArithmeticError::DivisionByZero => {
            PhysicalPathError::Invalid("interval directed division by zero")
        }
        FixedArithmeticError::InvalidDecimal => {
            PhysicalPathError::Invalid("noncanonical interval decimal")
        }
        FixedArithmeticError::StorageOverflow => {
            PhysicalPathError::Invalid("interval arithmetic overflow")
        }
        _ => PhysicalPathError::Invalid("interval arithmetic defect"),
    }
}

fn shield(value: Signed512) -> Result<Signed512, PhysicalPathError> {
    if value.bits() > INTERVAL_CELL_STEP_DERIVED_MAXIMUM_LIVE_BITS {
        Err(PhysicalPathError::Invalid(
            "interval 414-bit arithmetic shield exceeded",
        ))
    } else {
        Ok(value)
    }
}

fn parse_decimal(value: &str) -> Result<Signed512, PhysicalPathError> {
    shield(Signed512::from_canonical_decimal(value).map_err(map_arithmetic_error)?)
}

#[derive(Clone)]
struct RawInterval {
    lower: Signed512,
    upper: Signed512,
}
impl RawInterval {
    fn new(lower: Signed512, upper: Signed512) -> Result<Self, PhysicalPathError> {
        if lower > upper {
            Err(PhysicalPathError::Invalid("reversed interval"))
        } else {
            Ok(Self { lower, upper })
        }
    }
    fn public(&self, fractional_bits: u16) -> SignedDecimalIntervalV1 {
        SignedDecimalIntervalV1 {
            fractional_bits,
            lower: self.lower.decimal(),
            upper: self.upper.decimal(),
        }
    }
}

#[derive(Clone)]
struct Candidate {
    axis: usize,
    maximum: bool,
    lower: Signed512,
    upper: Option<Signed512>,
}

struct Work {
    observed: u16,
    divisions: u8,
    comparisons: u8,
}
impl Work {
    fn new() -> Self {
        Self {
            observed: 0,
            divisions: 0,
            comparisons: 0,
        }
    }
    fn see(&mut self, values: &[&Signed512]) {
        for value in values {
            self.observed = self.observed.max(value.bits());
        }
    }
}

impl ConditionalIntervalCellStepInputV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
    ) -> Result<Vec<u8>, PhysicalPathError> {
        validate_input(recipe, volume, self)?;
        let bytes = serde_json::to_vec(self)
            .map_err(|error| PhysicalPathError::Codec(error.to_string()))?;
        if bytes.len() > MAX_INTERVAL_CELL_STEP_INPUT_BYTES {
            return Err(PhysicalPathError::Invalid(
                "interval input byte ceiling exceeded",
            ));
        }
        Ok(bytes)
    }
    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPathError> {
        if bytes.len() > MAX_INTERVAL_CELL_STEP_INPUT_BYTES {
            return Err(PhysicalPathError::Invalid(
                "interval input byte ceiling exceeded",
            ));
        }
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| PhysicalPathError::Codec(error.to_string()))?;
        if value.to_bytes(recipe, volume)? != bytes {
            return Err(PhysicalPathError::Invalid(
                "noncanonical interval input bytes",
            ));
        }
        Ok(value)
    }
}

impl ConditionalIntervalCellStepEventV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        input: &ConditionalIntervalCellStepInputV1,
    ) -> Result<Vec<u8>, PhysicalPathError> {
        validate_conditional_interval_cell_step_event(recipe, volume, input, self)?;
        let bytes = serde_json::to_vec(self)
            .map_err(|error| PhysicalPathError::Codec(error.to_string()))?;
        if bytes.len() > MAX_INTERVAL_CELL_STEP_EVENT_BYTES {
            return Err(PhysicalPathError::Invalid(
                "interval event byte ceiling exceeded",
            ));
        }
        Ok(bytes)
    }
    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        input: &ConditionalIntervalCellStepInputV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPathError> {
        if bytes.len() > MAX_INTERVAL_CELL_STEP_EVENT_BYTES {
            return Err(PhysicalPathError::Invalid(
                "interval event byte ceiling exceeded",
            ));
        }
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| PhysicalPathError::Codec(error.to_string()))?;
        if value.to_bytes(recipe, volume, input)? != bytes {
            return Err(PhysicalPathError::Invalid(
                "noncanonical interval event bytes",
            ));
        }
        Ok(value)
    }
}

fn validate_input(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &ConditionalIntervalCellStepInputV1,
) -> Result<(), PhysicalPathError> {
    validate_physical_volume(recipe, volume)?;
    if input.schema_version != INTERVAL_CELL_STEP_CONTRACT_VERSION
        || input.state_revision == 0
        || input.state_source_id == [0; 32]
        || input.scope_id != recipe.input.scope_id
        || input.reconstruction_id != recipe.input.reconstruction_id
        || input.physical_volume_recipe_id != recipe.physical_volume_recipe_id
        || input.physical_volume_id != volume.physical_volume_id
    {
        return Err(PhysicalPathError::Invalid(
            "interval input provenance mismatch",
        ));
    }
    let cell = build_physical_cell(recipe, volume, input.current_cell)?;
    for axis in 0..3 {
        let point = decode_interval(&input.point_q160[axis], INTERVAL_CELL_STEP_FRACTIONAL_BITS)?;
        let minimum = Signed512::from_i64(cell.min_q32_32[axis]).shl(128)?;
        let maximum = Signed512::from_i64(cell.max_q32_32[axis]).shl(128)?;
        if point.lower < minimum || point.upper > maximum {
            return Err(PhysicalPathError::Invalid(
                "interval point lies outside current cell",
            ));
        }
        let direction = decode_interval(&input.direction_q1_62[axis], DIRECTION_FRACTIONAL_BITS)?;
        let limit = Signed512::one().shl(DIRECTION_FRACTIONAL_BITS)?;
        if direction.lower < limit.neg() || direction.upper > limit {
            return Err(PhysicalPathError::Invalid(
                "interval direction outside Q1.62 range",
            ));
        }
    }
    Ok(())
}

fn decode_interval(
    value: &SignedDecimalIntervalV1,
    bits: u16,
) -> Result<RawInterval, PhysicalPathError> {
    if value.fractional_bits != bits {
        return Err(PhysicalPathError::Invalid("interval scale mismatch"));
    }
    RawInterval::new(parse_decimal(&value.lower)?, parse_decimal(&value.upper)?)
}

pub fn compile_conditional_interval_cell_step(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &ConditionalIntervalCellStepInputV1,
) -> Result<ConditionalIntervalCellStepEventV1, PhysicalPathError> {
    let input_bytes = input.to_bytes(recipe, volume)?;
    let input_id = domain_hash(INPUT_DOMAIN, &input_bytes);
    let cell = build_physical_cell(recipe, volume, input.current_cell)?;
    let mut points = Vec::with_capacity(3);
    let mut directions = Vec::with_capacity(3);
    for axis in 0..3 {
        points.push(decode_interval(&input.point_q160[axis], 160)?);
        let raw = decode_interval(&input.direction_q1_62[axis], 62)?;
        directions.push(RawInterval::new(raw.lower.shl(98)?, raw.upper.shl(98)?)?);
    }
    let scale = Signed512::one().shl(160)?;
    let mut candidates = Vec::with_capacity(6);
    let mut work = Work::new();
    for axis in 0..3 {
        let lower_face = Signed512::from_i64(cell.min_q32_32[axis]).shl(128)?;
        let upper_face = Signed512::from_i64(cell.max_q32_32[axis]).shl(128)?;
        let direction = &directions[axis];
        if direction.upper > Signed512::zero() {
            let n_lower = upper_face.sub(&points[axis].upper)?;
            let n_upper = upper_face.sub(&points[axis].lower)?;
            let scaled_lower = n_lower.mul(&scale)?;
            let lower = scaled_lower.physical_div_floor(&direction.upper)?;
            work.divisions += 1;
            let upper = if direction.lower > Signed512::zero() {
                work.divisions += 1;
                Some(n_upper.mul(&scale)?.physical_div_ceil(&direction.lower)?)
            } else {
                None
            };
            work.see(&[&scaled_lower, &lower]);
            candidates.push(Candidate {
                axis,
                maximum: true,
                lower,
                upper,
            });
        }
        if direction.lower < Signed512::zero() {
            let n_lower = points[axis].lower.sub(&lower_face)?;
            let n_upper = points[axis].upper.sub(&lower_face)?;
            let speed_max = direction.lower.neg();
            let scaled_lower = n_lower.mul(&scale)?;
            let lower = scaled_lower.physical_div_floor(&speed_max)?;
            work.divisions += 1;
            let upper = if direction.upper < Signed512::zero() {
                work.divisions += 1;
                Some(
                    n_upper
                        .mul(&scale)?
                        .physical_div_ceil(&direction.upper.neg())?,
                )
            } else {
                None
            };
            work.see(&[&scaled_lower, &lower]);
            candidates.push(Candidate {
                axis,
                maximum: false,
                lower,
                upper,
            });
        }
    }
    let outcome = if candidates.is_empty()
        || candidates
            .iter()
            .any(|candidate| candidate.lower == Signed512::zero())
    {
        ConditionalIntervalCellStepOutcomeV1::NoForwardProgress
    } else {
        let mut winners = Vec::new();
        for (index, candidate) in candidates.iter().enumerate() {
            let Some(upper) = &candidate.upper else {
                continue;
            };
            let mut wins = true;
            for (other_index, other) in candidates.iter().enumerate() {
                if index == other_index {
                    continue;
                }
                work.comparisons += 1;
                if upper >= &other.lower {
                    wins = false;
                    break;
                }
            }
            if wins {
                winners.push(index);
            }
        }
        if winners.len() != 1 {
            ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace
        } else {
            certify(
                recipe,
                volume,
                input.current_cell,
                &cell,
                &points,
                &directions,
                &candidates[winners[0]],
                &scale,
                &mut work,
            )?
        }
    };
    let propagated = match outcome {
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
        | ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. }
        | ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { .. } => 2,
        _ => 0,
    };
    let receipt = IntervalCellStepArithmeticReceiptV1 {
        fractional_bits: 160,
        storage_bits: 512,
        derived_maximum_live_bits: 414,
        observed_maximum_live_bits: work.observed,
        possible_face_count: candidates.len() as u8,
        directed_division_count: work.divisions,
        strict_face_order_comparison_count: work.comparisons,
        propagated_tangential_axes: propagated,
    };
    let mut event = ConditionalIntervalCellStepEventV1 {
        schema_version: 1,
        interval_cell_step_input_id: input_id,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: input.current_cell,
        outcome,
        arithmetic_receipt: receipt,
        interval_cell_step_event_id: [0; 32],
        limitations: limitations(),
        authority_effect: "none_evidence_only".into(),
    };
    let identity_bytes =
        serde_json::to_vec(&event).map_err(|error| PhysicalPathError::Codec(error.to_string()))?;
    event.interval_cell_step_event_id = domain_hash(EVENT_DOMAIN, &identity_bytes);
    Ok(event)
}

fn certify(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    current: CellIndex3V1,
    cell: &crate::PhysicalCellV1,
    points: &[RawInterval],
    directions: &[RawInterval],
    candidate: &Candidate,
    scale: &Signed512,
    work: &mut Work,
) -> Result<ConditionalIntervalCellStepOutcomeV1, PhysicalPathError> {
    let time = RawInterval::new(
        candidate.lower.clone(),
        candidate.upper.clone().expect("winner is finite"),
    )?;
    let mut hit = Vec::with_capacity(3);
    for axis in 0..3 {
        if axis == candidate.axis {
            let raw = Signed512::from_i64(if candidate.maximum {
                cell.max_q32_32[axis]
            } else {
                cell.min_q32_32[axis]
            })
            .shl(128)?;
            hit.push(RawInterval::new(raw.clone(), raw)?);
        } else {
            let products = [
                directions[axis].lower.mul(&time.lower)?,
                directions[axis].lower.mul(&time.upper)?,
                directions[axis].upper.mul(&time.lower)?,
                directions[axis].upper.mul(&time.upper)?,
            ];
            work.see(&products.iter().collect::<Vec<_>>());
            let product_lower = products
                .iter()
                .min()
                .expect("four products")
                .physical_div_floor(scale)?;
            let product_upper = products
                .iter()
                .max()
                .expect("four products")
                .physical_div_ceil(scale)?;
            let lower = points[axis]
                .lower
                .add(&product_lower)?
                .max(Signed512::from_i64(cell.min_q32_32[axis]).shl(128)?);
            let upper = points[axis]
                .upper
                .add(&product_upper)?
                .min(Signed512::from_i64(cell.max_q32_32[axis]).shl(128)?);
            hit.push(
                RawInterval::new(lower, upper)
                    .map_err(|_| PhysicalPathError::Invalid("empty certified face intersection"))?,
            );
        }
    }
    let mut next = current;
    let extent = recipe.input.extent;
    let inside = match (candidate.axis, candidate.maximum) {
        (0, true) if current.x + 1 < extent[0] => {
            next.x += 1;
            true
        }
        (0, false) if current.x > 0 => {
            next.x -= 1;
            true
        }
        (1, true) if current.y + 1 < extent[1] => {
            next.y += 1;
            true
        }
        (1, false) if current.y > 0 => {
            next.y -= 1;
            true
        }
        (2, true) if current.z + 1 < extent[2] => {
            next.z += 1;
            true
        }
        (2, false) if current.z > 0 => {
            next.z -= 1;
            true
        }
        _ => false,
    };
    let certified = CertifiedIntervalCellFaceV1 {
        face: IntervalFaceV1 {
            axis: [
                IntervalFaceAxisV1::X,
                IntervalFaceAxisV1::Y,
                IntervalFaceAxisV1::Z,
            ][candidate.axis],
            side: if candidate.maximum {
                IntervalFaceSideV1::Maximum
            } else {
                IntervalFaceSideV1::Minimum
            },
        },
        neighbor: inside.then_some(next),
        time_q160: time.public(160),
        point_q160: [hit[0].public(160), hit[1].public(160), hit[2].public(160)],
    };
    if !inside {
        return Ok(ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { certified });
    }
    let evidence = build_physical_cell(recipe, volume, next)?.evidence;
    if evidence == CellEvidenceV1::Unavailable {
        Ok(ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { certified })
    } else {
        Ok(ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace {
            certified,
            neighbor_evidence: evidence,
        })
    }
}

pub fn validate_conditional_interval_cell_step_event(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &ConditionalIntervalCellStepInputV1,
    event: &ConditionalIntervalCellStepEventV1,
) -> Result<(), PhysicalPathError> {
    let expected = compile_conditional_interval_cell_step(recipe, volume, input)?;
    if event != &expected {
        Err(PhysicalPathError::Invalid(
            "conditional interval cell-step event drift",
        ))
    } else {
        Ok(())
    }
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
fn limitations() -> Vec<String> {
    vec![
        "conditional local interval evidence only".into(),
        "no optical lineage endpoint arrival or bulk transfer claim".into(),
        "no collision passage navigation biome planet runtime or authority claim".into(),
    ]
}
