//! Additive conditional interval-incident interface evidence.
//!
//! This module binds a caller-declared Q1.62 direction box to separately
//! replayed physical recipe and volume objects. It does not reconstruct a path
//! or continue one, and it never consults reference precision at runtime.

use crate::arithmetic::{FixedInterval, Signed512, checked_u512_product};
use crate::{
    BandInterfaceEventV1, DIRECTION_SCALE_BITS, DecimalIntervalV1, FaceInteractionEvidenceV1,
    FixedScaleV1, InterfaceModelV1, POWER_SCALE_BITS, VisibleRadianceInterfaceError,
    public_interval,
};
use crypto_bigint::{CheckedAdd, U512};
use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, Id, PhysicalVolumeRecipeV1, PhysicalVolumeV1,
    build_physical_cell, validate_physical_volume, validate_physical_volume_recipe,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub const INTERVAL_CONTRACT_VERSION: u16 = 1;
pub const INTERVAL_FRACTIONAL_BITS: u16 = 160;
pub const MAX_INTERVAL_INPUT_BYTES: usize = 16 * 1024;
pub const MAX_INTERVAL_EVENT_BYTES: usize = 64 * 1024;
pub const INTERVAL_DERIVED_MAXIMUM_LIVE_BITS: u16 = 452;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalEvidenceKindV1 {
    DeclaredConditionalDirectionBox,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceIntervalInterfaceInputV1 {
    pub schema_version: u16,
    pub incident_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub incident_revision: u32,
    pub evidence_kind: IntervalEvidenceKindV1,
    pub physical_volume_recipe_id: Id,
    pub physical_volume_id: Id,
    pub source_cell: CellIndex3V1,
    pub target_cell: CellIndex3V1,
    pub face_interaction: FaceInteractionEvidenceV1,
    pub incident_direction_xyz: [DecimalIntervalV1; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalUniformBranchV1 {
    AllTir,
    AllTransmit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum IntervalBandOutcomeV1 {
    BoundedEnclosure {
        branch: IntervalUniformBranchV1,
        event: BandInterfaceEventV1,
    },
    AmbiguousInterfaceBranch,
    NonconvergentEnclosure {
        reason_code: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalArithmeticReceiptV1 {
    pub exact_branch_classifications: u8,
    pub evaluated_band_count: u8,
    pub fractional_bits: u16,
    pub fractional_bit_work_units: u16,
    pub maximum_stored_endpoint_bits: u16,
    pub storage_bits: u16,
    pub derived_maximum_live_bits: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum IntervalInterfaceOutcomeV1 {
    Evaluated {
        bands_rgb: [IntervalBandOutcomeV1; 3],
        arithmetic_receipt: IntervalArithmeticReceiptV1,
    },
    UnsupportedInterfaceModel,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceIntervalInterfaceEventV1 {
    pub schema_version: u16,
    pub interval_interface_input_id: Id,
    pub event_id: Id,
    pub outcome: IntervalInterfaceOutcomeV1,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Copy, Debug)]
struct RawBox {
    lower: [i64; 3],
    upper: [i64; 3],
    axis: usize,
    normal_sign: i8,
}

#[derive(Clone, Debug)]
struct BranchBounds {
    branch: Option<IntervalUniformBranchV1>,
    tangent_min: U512,
    tangent_max: U512,
    normal_min: U512,
    normal_max: U512,
}

impl VisibleRadianceIntervalInterfaceInputV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
    ) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
        validate_interval_input(recipe, volume, self)?;
        encode_capped(
            self,
            MAX_INTERVAL_INPUT_BYTES,
            "interval input byte ceiling exceeded",
        )
    }

    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        bytes: &[u8],
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        if bytes.len() > MAX_INTERVAL_INPUT_BYTES {
            return Err(VisibleRadianceInterfaceError::Invalid(
                "interval input byte ceiling exceeded",
            ));
        }
        let value: Self = decode(bytes)?;
        validate_interval_input(recipe, volume, &value)?;
        require_canonical(&value, bytes, "noncanonical interval input bytes")?;
        Ok(value)
    }
}

impl VisibleRadianceIntervalInterfaceEventV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        input: &VisibleRadianceIntervalInterfaceInputV1,
    ) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
        validate_visible_radiance_interval_interface_event(recipe, volume, input, self)?;
        encode_capped(
            self,
            MAX_INTERVAL_EVENT_BYTES,
            "interval event byte ceiling exceeded",
        )
    }

    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        input: &VisibleRadianceIntervalInterfaceInputV1,
        bytes: &[u8],
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        if bytes.len() > MAX_INTERVAL_EVENT_BYTES {
            return Err(VisibleRadianceInterfaceError::Invalid(
                "interval event byte ceiling exceeded",
            ));
        }
        let value: Self = decode(bytes)?;
        validate_visible_radiance_interval_interface_event(recipe, volume, input, &value)?;
        require_canonical(&value, bytes, "noncanonical interval event bytes")?;
        Ok(value)
    }
}

pub fn compile_visible_radiance_interval_interface_event(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &VisibleRadianceIntervalInterfaceInputV1,
) -> Result<VisibleRadianceIntervalInterfaceEventV1, VisibleRadianceInterfaceError> {
    let raw = validate_interval_input(recipe, volume, input)?;
    let input_bytes = encode_capped(
        input,
        MAX_INTERVAL_INPUT_BYTES,
        "interval input byte ceiling exceeded",
    )?;
    let interval_interface_input_id = crate::hash(
        b"forge-visible-radiance-interval-interface-input-v1",
        &input_bytes,
    );
    let outcome = compile_interval_outcome(&raw, input)?;
    validate_interval_outcome(&outcome)?;
    let limitations = vec![
        "declared conditional local direction-box evidence; no prior path or endpoint arrival claim"
            .into(),
        "no composer coefficient catalogue persistence runtime perception rendering passage biome planet terrain approval or promotion claim"
            .into(),
    ];
    let authority_effect = "none".to_owned();
    let event_id = crate::hash(
        b"forge-visible-radiance-interval-interface-event-v1",
        &crate::encode(&(
            interval_interface_input_id,
            &outcome,
            &limitations,
            &authority_effect,
        ))?,
    );
    let event = VisibleRadianceIntervalInterfaceEventV1 {
        schema_version: INTERVAL_CONTRACT_VERSION,
        interval_interface_input_id,
        event_id,
        outcome,
        limitations,
        authority_effect,
    };
    encode_capped(
        &event,
        MAX_INTERVAL_EVENT_BYTES,
        "interval event byte ceiling exceeded",
    )?;
    Ok(event)
}

pub fn validate_visible_radiance_interval_interface_event(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &VisibleRadianceIntervalInterfaceInputV1,
    event: &VisibleRadianceIntervalInterfaceEventV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    let expected = compile_visible_radiance_interval_interface_event(recipe, volume, input)?;
    if &expected != event {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval interface event replay mismatch",
        ));
    }
    validate_interval_outcome(&event.outcome)
}

fn validate_interval_input(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: &VisibleRadianceIntervalInterfaceInputV1,
) -> Result<RawBox, VisibleRadianceInterfaceError> {
    validate_physical_volume_recipe(recipe)?;
    validate_physical_volume(recipe, volume)?;
    if input.schema_version != INTERVAL_CONTRACT_VERSION
        || input.incident_source_id == [0; 32]
        || input.scope_id == [0; 32]
        || input.reconstruction_id == [0; 32]
        || input.incident_revision == 0
        || input.physical_volume_recipe_id != recipe.physical_volume_recipe_id
        || input.physical_volume_id != volume.physical_volume_id
        || input.reconstruction_id != volume.reconstruction_id
        || input.scope_id != recipe.input.scope_id
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid interval input provenance",
        ));
    }
    let axis = shared_face_axis(input.source_cell, input.target_cell).ok_or(
        VisibleRadianceInterfaceError::Invalid("interval cells do not share one face"),
    )?;
    let source = build_physical_cell(recipe, volume, input.source_cell)?;
    let target = build_physical_cell(recipe, volume, input.target_cell)?;
    let face = &input.face_interaction;
    if face.scope_id != input.scope_id
        || face.reconstruction_id != input.reconstruction_id
        || face.interaction_source_id == [0; 32]
        || face.interaction_revision == 0
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid interval face provenance",
        ));
    }
    let (cell_a, cell_b, medium_a, medium_b) = if input.source_cell < input.target_cell {
        (
            input.source_cell,
            input.target_cell,
            &source.evidence,
            &target.evidence,
        )
    } else {
        (
            input.target_cell,
            input.source_cell,
            &target.evidence,
            &source.evidence,
        )
    };
    if face.cell_a != cell_a
        || face.cell_b != cell_b
        || &face.medium_a != medium_a
        || &face.medium_b != medium_b
        || source.evidence == target.evidence
        || matches!(source.evidence, CellEvidenceV1::Unavailable)
        || matches!(target.evidence, CellEvidenceV1::Unavailable)
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval face reconstruction mismatch",
        ));
    }
    validate_model(&face.model)?;
    let mut lower = [0_i64; 3];
    let mut upper = [0_i64; 3];
    for index in 0..3 {
        lower[index] = parse_q62(&input.incident_direction_xyz[index].lower)?;
        upper[index] = parse_q62(&input.incident_direction_xyz[index].upper)?;
        if input.incident_direction_xyz[index].scale != FixedScaleV1::Q1_62
            || lower[index] > upper[index]
        {
            return Err(VisibleRadianceInterfaceError::Invalid(
                "invalid interval direction component",
            ));
        }
    }
    if (0..3).all(|index| lower[index] <= 0 && upper[index] >= 0) {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval direction box contains zero vector",
        ));
    }
    let one_squared = 1_u128 << 124;
    let minimum = (0..3).try_fold(0_u128, |sum, index| {
        sum.checked_add(square_min(lower[index], upper[index]))
    });
    let maximum = (0..3).try_fold(0_u128, |sum, index| {
        sum.checked_add(square_max(lower[index], upper[index]))
    });
    if minimum.is_none_or(|value| value > one_squared)
        || maximum.is_none_or(|value| value < one_squared)
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval direction box misses unit sphere",
        ));
    }
    let normal_sign = if coordinate(input.target_cell, axis) > coordinate(input.source_cell, axis) {
        1
    } else {
        -1
    };
    if (normal_sign > 0 && lower[axis] <= 0) || (normal_sign < 0 && upper[axis] >= 0) {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval direction lacks strict face orientation",
        ));
    }
    Ok(RawBox {
        lower,
        upper,
        axis,
        normal_sign,
    })
}

fn validate_model(model: &InterfaceModelV1) -> Result<(), VisibleRadianceInterfaceError> {
    match model {
        InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb } => {
            for band in bands_rgb {
                if band.eta_a_q16_48 < (1_u64 << 46)
                    || band.eta_a_q16_48 > (16_u64 << 48)
                    || band.eta_b_q16_48 < (1_u64 << 46)
                    || band.eta_b_q16_48 > (16_u64 << 48)
                {
                    return Err(VisibleRadianceInterfaceError::Invalid(
                        "interval refractive index outside admitted range",
                    ));
                }
            }
        }
        InterfaceModelV1::Unsupported { model_source_id } if *model_source_id == [0; 32] => {
            return Err(VisibleRadianceInterfaceError::Invalid(
                "zero unsupported interval model identity",
            ));
        }
        InterfaceModelV1::Unsupported { .. } => {}
    }
    Ok(())
}

fn compile_interval_outcome(
    raw: &RawBox,
    input: &VisibleRadianceIntervalInterfaceInputV1,
) -> Result<IntervalInterfaceOutcomeV1, VisibleRadianceInterfaceError> {
    let InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb } =
        &input.face_interaction.model
    else {
        return Ok(IntervalInterfaceOutcomeV1::UnsupportedInterfaceModel);
    };
    let forward = input.source_cell == input.face_interaction.cell_a;
    let mut evaluated_band_count = 0_u8;
    let mut maximum_stored_endpoint_bits = 0_u16;
    let mut outputs = Vec::with_capacity(3);
    for band in bands_rgb {
        let (eta_i, eta_t) = if forward {
            (band.eta_a_q16_48, band.eta_b_q16_48)
        } else {
            (band.eta_b_q16_48, band.eta_a_q16_48)
        };
        let bounds = classify_branch(raw, eta_i, eta_t)?;
        match bounds.branch {
            None => outputs.push(IntervalBandOutcomeV1::AmbiguousInterfaceBranch),
            Some(branch) => {
                evaluated_band_count += 1;
                match evaluate_band(raw, &bounds, eta_i, eta_t, branch)? {
                    Some((event, stored_bits)) => {
                        maximum_stored_endpoint_bits =
                            maximum_stored_endpoint_bits.max(stored_bits);
                        outputs.push(IntervalBandOutcomeV1::BoundedEnclosure { branch, event });
                    }
                    None => outputs.push(IntervalBandOutcomeV1::NonconvergentEnclosure {
                        reason_code: "finite_enclosure_unavailable_at_fixed_160".into(),
                    }),
                }
            }
        }
    }
    let bands_rgb: [IntervalBandOutcomeV1; 3] = outputs.try_into().map_err(|_| {
        VisibleRadianceInterfaceError::ArithmeticDefect("interval band count drift")
    })?;
    Ok(IntervalInterfaceOutcomeV1::Evaluated {
        bands_rgb,
        arithmetic_receipt: IntervalArithmeticReceiptV1 {
            exact_branch_classifications: 3,
            evaluated_band_count,
            fractional_bits: INTERVAL_FRACTIONAL_BITS,
            fractional_bit_work_units: u16::from(evaluated_band_count) * INTERVAL_FRACTIONAL_BITS,
            maximum_stored_endpoint_bits,
            storage_bits: 512,
            derived_maximum_live_bits: INTERVAL_DERIVED_MAXIMUM_LIVE_BITS,
        },
    })
}

fn classify_branch(
    raw: &RawBox,
    eta_i: u64,
    eta_t: u64,
) -> Result<BranchBounds, VisibleRadianceInterfaceError> {
    let mut minima = [0_u128; 3];
    let mut maxima = [0_u128; 3];
    for index in 0..3 {
        minima[index] = square_min(raw.lower[index], raw.upper[index]);
        maxima[index] = square_max(raw.lower[index], raw.upper[index]);
    }
    let normal_min = minima[raw.axis];
    let normal_max = maxima[raw.axis];
    let tangent_min = minima
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != raw.axis)
        .try_fold(0_u128, |sum, (_, value)| sum.checked_add(*value))
        .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
            "interval tangent minimum overflow",
        ))?;
    let tangent_max = maxima
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != raw.axis)
        .try_fold(0_u128, |sum, (_, value)| sum.checked_add(*value))
        .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
            "interval tangent maximum overflow",
        ))?;
    let eta_i_sq = Signed512::from_i128(i128::from(eta_i))
        .checked_mul(&Signed512::from_i128(i128::from(eta_i)))?;
    let eta_t_sq = Signed512::from_i128(i128::from(eta_t))
        .checked_mul(&Signed512::from_i128(i128::from(eta_t)))?;
    let coefficient = eta_i_sq.checked_sub(&eta_t_sq)?;
    let target_squared = eta_t_sq;
    let value = |coefficient_value: u128, normal_value: u128| {
        coefficient
            .checked_mul(&Signed512::new(false, U512::from(coefficient_value)))?
            .checked_sub(
                &target_squared.checked_mul(&Signed512::new(false, U512::from(normal_value)))?,
            )
    };
    let (discriminator_min, discriminator_max) =
        if coefficient.is_negative() || coefficient == Signed512::zero() {
            (
                value(tangent_max, normal_max)?,
                value(tangent_min, normal_min)?,
            )
        } else {
            (
                value(tangent_min, normal_max)?,
                value(tangent_max, normal_min)?,
            )
        };
    let branch = if discriminator_min >= Signed512::zero() {
        Some(IntervalUniformBranchV1::AllTir)
    } else if discriminator_max < Signed512::zero() {
        Some(IntervalUniformBranchV1::AllTransmit)
    } else {
        None
    };
    Ok(BranchBounds {
        branch,
        tangent_min: U512::from(tangent_min),
        tangent_max: U512::from(tangent_max),
        normal_min: U512::from(normal_min),
        normal_max: U512::from(normal_max),
    })
}

fn evaluate_band(
    raw: &RawBox,
    bounds: &BranchBounds,
    eta_i: u64,
    eta_t: u64,
    branch: IntervalUniformBranchV1,
) -> Result<Option<(BandInterfaceEventV1, u16)>, VisibleRadianceInterfaceError> {
    let bits = INTERVAL_FRACTIONAL_BITS;
    let zero = FixedInterval::integer(0, bits)?;
    let one = FixedInterval::integer(1, bits)?;
    let two = FixedInterval::integer(2, bits)?;
    let mut components = raw_components(raw)?;
    if raw.normal_sign < 0 {
        components[raw.axis] = negate(&components[raw.axis])?;
    }
    let squared = sum_squares(&components, &zero)?;
    let norm = squared.sqrt()?;
    if norm.lower <= Signed512::zero() {
        return Ok(None);
    }
    let incident = map_components(|index| components[index].div(&norm))?;
    let cos_i = incident[raw.axis].clone();
    let reflected_oriented = map_components(|index| {
        let factor = if index == raw.axis { 2 } else { 0 };
        let normal = cos_i.mul(&FixedInterval::integer(factor, bits)?)?;
        incident[index].sub(&normal)
    })?;
    if !sum_squares(&reflected_oriented, &zero)?.contains_integer(1)? {
        return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
            "interval reflected direction lost unit-vector containment",
        ));
    }
    let (reflectance, transmittance, transmitted_oriented) = match branch {
        IntervalUniformBranchV1::AllTir => (one.clone(), zero.clone(), None),
        IntervalUniformBranchV1::AllTransmit => {
            let eta_i_sq = checked_u512_product(&[U512::from(eta_i), U512::from(eta_i)])?;
            let eta_t_sq = checked_u512_product(&[U512::from(eta_t), U512::from(eta_t)])?;
            let sin_lower_num = checked_u512_product(&[eta_i_sq, bounds.tangent_min])?;
            let sin_lower_den = checked_u512_product(&[
                eta_t_sq,
                Option::<U512>::from(bounds.tangent_min.checked_add(&bounds.normal_max)).ok_or(
                    VisibleRadianceInterfaceError::ArithmeticDefect(
                        "interval lower denominator overflow",
                    ),
                )?,
            ])?;
            let sin_upper_num = checked_u512_product(&[eta_i_sq, bounds.tangent_max])?;
            let sin_upper_den = checked_u512_product(&[
                eta_t_sq,
                Option::<U512>::from(bounds.tangent_max.checked_add(&bounds.normal_min)).ok_or(
                    VisibleRadianceInterfaceError::ArithmeticDefect(
                        "interval upper denominator overflow",
                    ),
                )?,
            ])?;
            let lower = FixedInterval::unsigned_ratio(sin_lower_num, sin_lower_den, bits)?;
            let upper = FixedInterval::unsigned_ratio(sin_upper_num, sin_upper_den, bits)?;
            let sin_t_squared =
                FixedInterval::new(lower.lower, upper.upper, bits)?.intersect_unit()?;
            let cos_t = one.sub(&sin_t_squared)?.intersect_unit()?.sqrt()?;
            let q = FixedInterval::unsigned_ratio(U512::from(eta_t), U512::from(eta_i), bits)?;
            let q_cos_i = q.mul(&cos_i)?;
            let q_cos_t = q.mul(&cos_t)?;
            let parallel_denominator = q_cos_i.add(&cos_t)?;
            let perpendicular_denominator = cos_i.add(&q_cos_t)?;
            if contains_zero(&parallel_denominator) || contains_zero(&perpendicular_denominator) {
                return Ok(None);
            }
            let r_parallel = q_cos_i.sub(&cos_t)?.div(&parallel_denominator)?;
            let r_perpendicular = cos_i.sub(&q_cos_t)?.div(&perpendicular_denominator)?;
            let reflectance = r_parallel
                .square()?
                .add(&r_perpendicular.square()?)?
                .div(&two)?
                .intersect_unit()?;
            let transmittance = one.sub(&reflectance)?;
            let transmitted = map_components(|index| {
                if index == raw.axis {
                    Ok(cos_t.clone())
                } else {
                    incident[index].div(&q)
                }
            })?;
            if !reflectance.add(&transmittance)?.contains_integer(1)? {
                return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                    "interval power enclosure lost energy containment",
                ));
            }
            if !sum_squares(&transmitted, &zero)?.contains_integer(1)? {
                return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                    "interval transmitted direction lost unit-vector containment",
                ));
            }
            (reflectance, transmittance, Some(transmitted))
        }
    };
    let reflected = world_components(reflected_oriented, raw)?;
    let transmitted = transmitted_oriented
        .map(|values| world_components(values, raw))
        .transpose()?;
    let maximum_stored_endpoint_bits = reflected.iter().chain(transmitted.iter().flatten()).fold(
        reflectance.max_bits().max(transmittance.max_bits()),
        |maximum, value| maximum.max(value.max_bits()),
    );
    let public = BandInterfaceEventV1 {
        total_internal_reflection: branch == IntervalUniformBranchV1::AllTir,
        reflected_power: public_fixed(&reflectance, POWER_SCALE_BITS, FixedScaleV1::Q0_48)?,
        transmitted_power: public_fixed(&transmittance, POWER_SCALE_BITS, FixedScaleV1::Q0_48)?,
        reflected_direction_xyz: public_components(&reflected)?,
        transmitted_direction_xyz: transmitted
            .map(|values| public_components(&values))
            .transpose()?,
    };
    Ok(Some((public, maximum_stored_endpoint_bits)))
}

fn raw_components(raw: &RawBox) -> Result<[FixedInterval; 3], VisibleRadianceInterfaceError> {
    map_components(|index| {
        FixedInterval::new(
            Signed512::from_i128(i128::from(raw.lower[index]))
                .checked_shl(INTERVAL_FRACTIONAL_BITS - DIRECTION_SCALE_BITS)?,
            Signed512::from_i128(i128::from(raw.upper[index]))
                .checked_shl(INTERVAL_FRACTIONAL_BITS - DIRECTION_SCALE_BITS)?,
            INTERVAL_FRACTIONAL_BITS,
        )
    })
}

fn world_components(
    mut values: [FixedInterval; 3],
    raw: &RawBox,
) -> Result<[FixedInterval; 3], VisibleRadianceInterfaceError> {
    if raw.normal_sign < 0 {
        values[raw.axis] = negate(&values[raw.axis])?;
    }
    Ok(values)
}

fn negate(value: &FixedInterval) -> Result<FixedInterval, VisibleRadianceInterfaceError> {
    FixedInterval::new(
        value.upper.checked_neg(),
        value.lower.checked_neg(),
        value.bits,
    )
}

fn sum_squares(
    values: &[FixedInterval],
    zero: &FixedInterval,
) -> Result<FixedInterval, VisibleRadianceInterfaceError> {
    values
        .iter()
        .try_fold(zero.clone(), |sum, value| sum.add(&value.square()?))
}

fn map_components(
    mut function: impl FnMut(usize) -> Result<FixedInterval, VisibleRadianceInterfaceError>,
) -> Result<[FixedInterval; 3], VisibleRadianceInterfaceError> {
    Ok([function(0)?, function(1)?, function(2)?])
}

fn public_fixed(
    value: &FixedInterval,
    bits: u16,
    scale: FixedScaleV1,
) -> Result<DecimalIntervalV1, VisibleRadianceInterfaceError> {
    Ok(public_interval(&value.project(bits)?, scale))
}

fn public_components(
    values: &[FixedInterval; 3],
) -> Result<[DecimalIntervalV1; 3], VisibleRadianceInterfaceError> {
    let projected: Vec<DecimalIntervalV1> = values
        .iter()
        .map(|value| public_fixed(value, DIRECTION_SCALE_BITS, FixedScaleV1::Q1_62))
        .collect::<Result<_, _>>()?;
    projected
        .try_into()
        .map_err(|_| VisibleRadianceInterfaceError::ArithmeticDefect("interval projection count"))
}

fn validate_interval_outcome(
    outcome: &IntervalInterfaceOutcomeV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    let IntervalInterfaceOutcomeV1::Evaluated {
        bands_rgb,
        arithmetic_receipt,
    } = outcome
    else {
        return Ok(());
    };
    let evaluated = bands_rgb
        .iter()
        .filter(|band| !matches!(band, IntervalBandOutcomeV1::AmbiguousInterfaceBranch))
        .count() as u8;
    if arithmetic_receipt.exact_branch_classifications != 3
        || arithmetic_receipt.evaluated_band_count != evaluated
        || arithmetic_receipt.fractional_bits != INTERVAL_FRACTIONAL_BITS
        || arithmetic_receipt.fractional_bit_work_units
            != u16::from(evaluated) * INTERVAL_FRACTIONAL_BITS
        || arithmetic_receipt.storage_bits != 512
        || arithmetic_receipt.derived_maximum_live_bits != INTERVAL_DERIVED_MAXIMUM_LIVE_BITS
        || arithmetic_receipt.maximum_stored_endpoint_bits > 512
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid interval arithmetic receipt",
        ));
    }
    for band in bands_rgb {
        match band {
            IntervalBandOutcomeV1::BoundedEnclosure { branch, event } => {
                if event.total_internal_reflection != (*branch == IntervalUniformBranchV1::AllTir)
                    || event.total_internal_reflection != event.transmitted_direction_xyz.is_none()
                {
                    return Err(VisibleRadianceInterfaceError::Invalid(
                        "interval branch output mismatch",
                    ));
                }
                validate_public_band(event)?;
            }
            IntervalBandOutcomeV1::NonconvergentEnclosure { reason_code }
                if reason_code != "finite_enclosure_unavailable_at_fixed_160" =>
            {
                return Err(VisibleRadianceInterfaceError::Invalid(
                    "invalid interval nonconvergence reason",
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_public_band(event: &BandInterfaceEventV1) -> Result<(), VisibleRadianceInterfaceError> {
    validate_public_interval(&event.reflected_power, FixedScaleV1::Q0_48)?;
    validate_public_interval(&event.transmitted_power, FixedScaleV1::Q0_48)?;
    for value in &event.reflected_direction_xyz {
        validate_public_interval(value, FixedScaleV1::Q1_62)?;
    }
    if let Some(values) = &event.transmitted_direction_xyz {
        for value in values {
            validate_public_interval(value, FixedScaleV1::Q1_62)?;
        }
    }
    Ok(())
}

fn validate_public_interval(
    value: &DecimalIntervalV1,
    scale: FixedScaleV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    if value.scale != scale
        || parse_canonical_i128(&value.lower)? > parse_canonical_i128(&value.upper)?
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid interval public decimal",
        ));
    }
    Ok(())
}

fn parse_q62(value: &str) -> Result<i64, VisibleRadianceInterfaceError> {
    let parsed = parse_canonical_i128(value)?;
    let limit = 1_i128 << DIRECTION_SCALE_BITS;
    if parsed < -limit || parsed > limit {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interval Q1.62 endpoint outside unit range",
        ));
    }
    i64::try_from(parsed)
        .map_err(|_| VisibleRadianceInterfaceError::Invalid("interval Q1.62 endpoint outside i64"))
}

fn parse_canonical_i128(value: &str) -> Result<i128, VisibleRadianceInterfaceError> {
    let parsed = value.parse::<i128>().map_err(|_| {
        VisibleRadianceInterfaceError::Invalid("noncanonical interval signed decimal")
    })?;
    if parsed.to_string() != value {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "noncanonical interval signed decimal",
        ));
    }
    Ok(parsed)
}

fn square_min(lower: i64, upper: i64) -> u128 {
    if lower <= 0 && upper >= 0 {
        0
    } else {
        u128::from(lower.unsigned_abs())
            .pow(2)
            .min(u128::from(upper.unsigned_abs()).pow(2))
    }
}

fn square_max(lower: i64, upper: i64) -> u128 {
    u128::from(lower.unsigned_abs())
        .pow(2)
        .max(u128::from(upper.unsigned_abs()).pow(2))
}

fn shared_face_axis(a: CellIndex3V1, b: CellIndex3V1) -> Option<usize> {
    let a = [a.x, a.y, a.z];
    let b = [b.x, b.y, b.z];
    let differing: Vec<_> = (0..3).filter(|index| a[*index] != b[*index]).collect();
    (differing.len() == 1 && a[differing[0]].abs_diff(b[differing[0]]) == 1).then_some(differing[0])
}

fn coordinate(cell: CellIndex3V1, axis: usize) -> u32 {
    [cell.x, cell.y, cell.z][axis]
}

fn contains_zero(value: &FixedInterval) -> bool {
    value.lower <= Signed512::zero() && value.upper >= Signed512::zero()
}

fn encode_capped<T: Serialize>(
    value: &T,
    ceiling: usize,
    message: &'static str,
) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
    let bytes = crate::encode(value)?;
    if bytes.len() > ceiling {
        return Err(VisibleRadianceInterfaceError::Invalid(message));
    }
    Ok(bytes)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VisibleRadianceInterfaceError> {
    serde_json::from_slice(bytes)
        .map_err(|error| VisibleRadianceInterfaceError::Codec(error.to_string()))
}

fn require_canonical<T: Serialize>(
    value: &T,
    bytes: &[u8],
    message: &'static str,
) -> Result<(), VisibleRadianceInterfaceError> {
    if crate::encode(value)? != bytes {
        return Err(VisibleRadianceInterfaceError::Invalid(message));
    }
    Ok(())
}
