use fixed_interval_arithmetic::{FixedArithmeticError, FixedInterval, Signed512};
use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, ConditionalIntervalCellStepEventV1,
    ConditionalIntervalCellStepInputV1, ConditionalIntervalCellStepOutcomeV1, Id,
    SignedDecimalIntervalV1, build_physical_cell, validate_conditional_interval_cell_step_event,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::{
    BandTransferV1, BulkBandInteractionV1, FixedU128V1, TRANSMISSION_ONE_Q0_48,
    VisibleRadianceBulkError, VisibleRadianceBulkProfileV1, encode, exp_neg_q0_64_bounds, hash,
    rebuild_volume, validate_visible_radiance_bulk_profile,
};

pub const MAX_INTERVAL_BULK_QUERY_BYTES: usize = 64 * 1024;
pub const MAX_INTERVAL_BULK_TRANSFER_BYTES: usize = 16 * 1024;
pub const INTERVAL_BULK_FRACTIONAL_BITS: u16 = 160;
pub const INTERVAL_BULK_STORAGE_BITS: u16 = 512;
pub const INTERVAL_BULK_DERIVED_MAXIMUM_MAGNITUDE_BITS: u16 = 414;
pub const INTERVAL_BULK_FINAL_LENGTH_MAXIMUM_MAGNITUDE_BITS: u16 = 192;

const QUERY_DOMAIN: &[u8] = b"mindwarp.visible-radiance.interval-bulk-query.v1";
const TRANSFER_DOMAIN: &[u8] = b"mindwarp.visible-radiance.interval-bulk-transfer.v1";
const DIRECTION_FRACTIONAL_BITS: u16 = 62;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleRadianceBandV1 {
    Red,
    Green,
    Blue,
}

impl VisibleRadianceBandV1 {
    fn index(self) -> usize {
        match self {
            Self::Red => 0,
            Self::Green => 1,
            Self::Blue => 2,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalIntervalBulkQueryV1 {
    pub schema_version: u16,
    pub visible_radiance_bulk_profile_id: Id,
    pub band: VisibleRadianceBandV1,
    pub interval_cell_step_input: ConditionalIntervalCellStepInputV1,
    pub interval_cell_step_event: ConditionalIntervalCellStepEventV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalBulkLengthCertificateV1 {
    pub speed_time_q160: SignedDecimalIntervalV1,
    pub displacement_q160: SignedDecimalIntervalV1,
    pub intersection_q160: SignedDecimalIntervalV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum IntervalBulkTerminalV1 {
    KnownNeighbor { neighbor: CellIndex3V1 },
    UnavailableNeighbor { neighbor: CellIndex3V1 },
    OuterDomainExit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalBulkArithmeticReceiptV1 {
    pub fractional_bits: u16,
    pub storage_bits: u16,
    pub derived_maximum_magnitude_bits: u16,
    pub observed_maximum_magnitude_bits: u16,
    pub endpoint_and_coefficient_shift_ceiling: u8,
    pub interval_multiplication_ceiling: u8,
    pub interval_addition_ceiling: u8,
    pub interval_subtraction_ceiling: u8,
    pub directed_square_root_ceiling: u8,
    pub intersection_ceiling: u8,
    pub projection_ceiling: u8,
    pub exponential_term_ceiling: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConditionalIntervalBulkOutcomeV1 {
    KnownCurrentCellTransfer {
        length_certificate: IntervalBulkLengthCertificateV1,
        band_transfer: BandTransferV1,
        terminal: IntervalBulkTerminalV1,
    },
    UnavailableCurrentCell,
    UpstreamAmbiguousNextFace,
    UpstreamNoForwardProgress,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalIntervalBulkTransferV1 {
    pub schema_version: u16,
    pub visible_radiance_bulk_profile_id: Id,
    pub physical_volume_id: Id,
    pub conditional_interval_bulk_query_id: Id,
    pub interval_cell_step_input_id: Id,
    pub interval_cell_step_event_id: Id,
    pub band: VisibleRadianceBandV1,
    pub current_cell: CellIndex3V1,
    pub outcome: ConditionalIntervalBulkOutcomeV1,
    pub arithmetic_receipt: IntervalBulkArithmeticReceiptV1,
    pub conditional_interval_bulk_transfer_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl ConditionalIntervalBulkQueryV1 {
    pub fn to_bytes(
        &self,
        profile: &VisibleRadianceBulkProfileV1,
    ) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_interval_bulk_query(profile, self)?;
        encode_capped(
            self,
            MAX_INTERVAL_BULK_QUERY_BYTES,
            "interval bulk query byte ceiling",
        )
    }

    pub fn from_bytes(
        bytes: &[u8],
        profile: &VisibleRadianceBulkProfileV1,
    ) -> Result<Self, VisibleRadianceBulkError> {
        if bytes.len() > MAX_INTERVAL_BULK_QUERY_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "interval bulk query byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(profile)? != bytes {
            return Err(VisibleRadianceBulkError::Invalid(
                "noncanonical interval bulk query bytes",
            ));
        }
        Ok(value)
    }
}

impl ConditionalIntervalBulkTransferV1 {
    pub fn to_bytes(
        &self,
        profile: &VisibleRadianceBulkProfileV1,
        query: &ConditionalIntervalBulkQueryV1,
    ) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_conditional_interval_bulk_transfer(profile, query, self)?;
        encode_capped(
            self,
            MAX_INTERVAL_BULK_TRANSFER_BYTES,
            "interval bulk transfer byte ceiling",
        )
    }

    pub fn from_bytes(
        bytes: &[u8],
        profile: &VisibleRadianceBulkProfileV1,
        query: &ConditionalIntervalBulkQueryV1,
    ) -> Result<Self, VisibleRadianceBulkError> {
        if bytes.len() > MAX_INTERVAL_BULK_TRANSFER_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "interval bulk transfer byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(profile, query)? != bytes {
            return Err(VisibleRadianceBulkError::Invalid(
                "noncanonical interval bulk transfer bytes",
            ));
        }
        Ok(value)
    }
}

pub fn compile_conditional_interval_bulk_transfer(
    profile: &VisibleRadianceBulkProfileV1,
    query: &ConditionalIntervalBulkQueryV1,
) -> Result<ConditionalIntervalBulkTransferV1, VisibleRadianceBulkError> {
    let (recipe, volume) = validate_interval_bulk_query(profile, query)?;
    let query_bytes = query.to_bytes(profile)?;
    let query_id = hash(QUERY_DOMAIN, &query_bytes);
    let input_id = query.interval_cell_step_event.interval_cell_step_input_id;
    let event_id = query.interval_cell_step_event.interval_cell_step_event_id;
    let (outcome, observed_bits) = compile_interval_outcome(profile, query, &recipe, &volume)?;
    let arithmetic_receipt = IntervalBulkArithmeticReceiptV1 {
        fractional_bits: INTERVAL_BULK_FRACTIONAL_BITS,
        storage_bits: INTERVAL_BULK_STORAGE_BITS,
        derived_maximum_magnitude_bits: INTERVAL_BULK_DERIVED_MAXIMUM_MAGNITUDE_BITS,
        observed_maximum_magnitude_bits: observed_bits,
        endpoint_and_coefficient_shift_ceiling: 7,
        interval_multiplication_ceiling: 8,
        interval_addition_ceiling: 4,
        interval_subtraction_ceiling: 3,
        directed_square_root_ceiling: 2,
        intersection_ceiling: 1,
        projection_ceiling: 1,
        exponential_term_ceiling: 192,
    };
    if observed_bits > INTERVAL_BULK_DERIVED_MAXIMUM_MAGNITUDE_BITS {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk arithmetic ceiling exceeded",
        ));
    }
    let identity_bytes = encode(&(
        profile.visible_radiance_bulk_profile_id,
        volume.physical_volume_id,
        query_id,
        event_id,
        query.band,
        query.interval_cell_step_input.current_cell,
        &outcome,
        arithmetic_receipt,
    ))?;
    Ok(ConditionalIntervalBulkTransferV1 {
        schema_version: super::CONTRACT_VERSION,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        physical_volume_id: volume.physical_volume_id,
        conditional_interval_bulk_query_id: query_id,
        interval_cell_step_input_id: input_id,
        interval_cell_step_event_id: event_id,
        band: query.band,
        current_cell: query.interval_cell_step_input.current_cell,
        outcome,
        arithmetic_receipt,
        conditional_interval_bulk_transfer_id: hash(TRANSFER_DOMAIN, &identity_bytes),
        limitations: interval_bulk_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_conditional_interval_bulk_transfer(
    profile: &VisibleRadianceBulkProfileV1,
    query: &ConditionalIntervalBulkQueryV1,
    transfer: &ConditionalIntervalBulkTransferV1,
) -> Result<(), VisibleRadianceBulkError> {
    if &compile_conditional_interval_bulk_transfer(profile, query)? != transfer {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk transfer drift",
        ));
    }
    Ok(())
}

fn validate_interval_bulk_query(
    profile: &VisibleRadianceBulkProfileV1,
    query: &ConditionalIntervalBulkQueryV1,
) -> Result<
    (
        physical_path_substrate::PhysicalVolumeRecipeV1,
        physical_path_substrate::PhysicalVolumeV1,
    ),
    VisibleRadianceBulkError,
> {
    validate_visible_radiance_bulk_profile(profile)?;
    if query.schema_version != super::CONTRACT_VERSION
        || query.visible_radiance_bulk_profile_id != profile.visible_radiance_bulk_profile_id
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk query profile or schema mismatch",
        ));
    }
    let (recipe, volume) = rebuild_volume(&profile.input)?;
    validate_conditional_interval_cell_step_event(
        &recipe,
        &volume,
        &query.interval_cell_step_input,
        &query.interval_cell_step_event,
    )?;
    if query.interval_cell_step_input.physical_volume_recipe_id != recipe.physical_volume_recipe_id
        || query.interval_cell_step_input.physical_volume_id != volume.physical_volume_id
        || query.interval_cell_step_event.physical_volume_recipe_id
            != recipe.physical_volume_recipe_id
        || query.interval_cell_step_event.physical_volume_id != volume.physical_volume_id
        || query.interval_cell_step_event.current_cell
            != query.interval_cell_step_input.current_cell
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk physical binding mismatch",
        ));
    }
    Ok((recipe, volume))
}

fn compile_interval_outcome(
    profile: &VisibleRadianceBulkProfileV1,
    query: &ConditionalIntervalBulkQueryV1,
    recipe: &physical_path_substrate::PhysicalVolumeRecipeV1,
    volume: &physical_path_substrate::PhysicalVolumeV1,
) -> Result<(ConditionalIntervalBulkOutcomeV1, u16), VisibleRadianceBulkError> {
    let (certified, terminal) = match &query.interval_cell_step_event.outcome {
        ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace => {
            return Ok((
                ConditionalIntervalBulkOutcomeV1::UpstreamAmbiguousNextFace,
                0,
            ));
        }
        ConditionalIntervalCellStepOutcomeV1::NoForwardProgress => {
            return Ok((
                ConditionalIntervalBulkOutcomeV1::UpstreamNoForwardProgress,
                0,
            ));
        }
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } => {
            let neighbor = certified.neighbor.ok_or(VisibleRadianceBulkError::Invalid(
                "certified interval face lacks neighbour",
            ))?;
            (
                certified,
                IntervalBulkTerminalV1::KnownNeighbor { neighbor },
            )
        }
        ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { certified } => {
            let neighbor = certified.neighbor.ok_or(VisibleRadianceBulkError::Invalid(
                "unavailable interval face lacks neighbour",
            ))?;
            (
                certified,
                IntervalBulkTerminalV1::UnavailableNeighbor { neighbor },
            )
        }
        ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { certified } => {
            (certified, IntervalBulkTerminalV1::OuterDomainExit)
        }
    };
    let current = build_physical_cell(recipe, volume, query.interval_cell_step_input.current_cell)?;
    if current.evidence == CellEvidenceV1::Unavailable {
        return Ok((ConditionalIntervalBulkOutcomeV1::UnavailableCurrentCell, 0));
    }

    let (length_certificate, length, observed_bits) =
        length_certificate(&query.interval_cell_step_input, certified)?;
    let band_transfer = match &current.evidence {
        CellEvidenceV1::Vacuum => BandTransferV1::VacuumIdentity,
        CellEvidenceV1::Unavailable => unreachable!("handled above"),
        CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } => {
            let interaction = profile
                .input
                .substance_interactions
                .binary_search_by_key(substance_source_id, |entry| entry.substance_source_id)
                .ok()
                .and_then(|index| profile.input.substance_interactions.get(index))
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk-profile interaction missing",
                ))?;
            compile_interval_band_transfer(&interaction.bands_rgb[query.band.index()], &length)?
        }
    };
    Ok((
        ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
            length_certificate,
            band_transfer,
            terminal,
        },
        observed_bits,
    ))
}

fn length_certificate(
    input: &ConditionalIntervalCellStepInputV1,
    certified: &physical_path_substrate::CertifiedIntervalCellFaceV1,
) -> Result<(IntervalBulkLengthCertificateV1, FixedInterval, u16), VisibleRadianceBulkError> {
    let directions = input
        .direction_q1_62
        .each_ref()
        .map(|value| parse_and_lift(value, DIRECTION_FRACTIONAL_BITS, 98))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let points = input
        .point_q160
        .each_ref()
        .map(|value| parse_fixed(value, INTERVAL_BULK_FRACTIONAL_BITS))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let hits = certified
        .point_q160
        .each_ref()
        .map(|value| parse_fixed(value, INTERVAL_BULK_FRACTIONAL_BITS))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let time = parse_fixed(&certified.time_q160, INTERVAL_BULK_FRACTIONAL_BITS)?;
    let speed = interval_norm(&directions)?;
    let speed_time = arithmetic(speed.checked_mul(&time))?;
    let displacements = points
        .iter()
        .zip(hits.iter())
        .map(|(start, hit)| arithmetic(hit.checked_sub(start)))
        .collect::<Result<Vec<_>, _>>()?;
    let displacement = interval_norm(&displacements)?;
    let intersection = arithmetic(speed_time.intersect(&displacement))?;
    if intersection.lower().is_negative()
        || intersection.maximum_magnitude_bits() > INTERVAL_BULK_FINAL_LENGTH_MAXIMUM_MAGNITUDE_BITS
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk final length ceiling exceeded",
        ));
    }
    let observed = directions
        .iter()
        .chain(points.iter())
        .chain(hits.iter())
        .chain(displacements.iter())
        .chain([&time, &speed_time, &displacement, &intersection])
        .map(FixedInterval::maximum_magnitude_bits)
        .max()
        .unwrap_or(0);
    Ok((
        IntervalBulkLengthCertificateV1 {
            speed_time_q160: public_interval(&speed_time),
            displacement_q160: public_interval(&displacement),
            intersection_q160: public_interval(&intersection),
        },
        intersection,
        observed,
    ))
}

fn interval_norm(values: &[FixedInterval]) -> Result<FixedInterval, VisibleRadianceBulkError> {
    let zero = FixedInterval::new(
        Signed512::zero(),
        Signed512::zero(),
        INTERVAL_BULK_FRACTIONAL_BITS,
    )
    .map_err(map_arithmetic)?;
    let sum = values.iter().try_fold(zero, |sum, value| {
        let square = interval_square(value)?;
        arithmetic(sum.checked_add(&square))
    })?;
    arithmetic(sum.sqrt())
}

fn interval_square(value: &FixedInterval) -> Result<FixedInterval, VisibleRadianceBulkError> {
    if value.lower() <= &Signed512::zero() && value.upper() >= &Signed512::zero() {
        let lower_square = arithmetic(value.lower().checked_mul(value.lower()))?;
        let upper_square = arithmetic(value.upper().checked_mul(value.upper()))?;
        let maximum = lower_square.max(upper_square);
        let scale = arithmetic(Signed512::one().checked_shl(value.fractional_bits()))?;
        FixedInterval::new(
            Signed512::zero(),
            arithmetic(maximum.div_ceil(&scale))?,
            value.fractional_bits(),
        )
        .map_err(map_arithmetic)
    } else {
        arithmetic(value.checked_mul(value))
    }
}

fn compile_interval_band_transfer(
    interaction: &BulkBandInteractionV1,
    length: &FixedInterval,
) -> Result<BandTransferV1, VisibleRadianceBulkError> {
    match interaction {
        BulkBandInteractionV1::Opaque => Ok(BandTransferV1::Opaque),
        BulkBandInteractionV1::Finite {
            extinction_q16_48_per_coordinate_unit,
        } => {
            let coefficient = arithmetic(
                Signed512::from_i128(i128::from(*extinction_q16_48_per_coordinate_unit))
                    .checked_shl(INTERVAL_BULK_FRACTIONAL_BITS - 48),
            )?;
            let coefficient = FixedInterval::new(
                coefficient.clone(),
                coefficient,
                INTERVAL_BULK_FRACTIONAL_BITS,
            )
            .map_err(map_arithmetic)?;
            let optical_q160 = arithmetic(length.checked_mul(&coefficient))?;
            let optical_q64 = arithmetic(optical_q160.project(64))?;
            let lower = nonnegative_u128(optical_q64.lower())?;
            let upper = nonnegative_u128(optical_q64.upper())?;
            let (transmission_lower_q64, _) = exp_neg_q0_64_bounds(upper)?;
            let (_, transmission_upper_q64) = exp_neg_q0_64_bounds(lower)?;
            let transmission_lower_q0_48 = (transmission_lower_q64 >> 16) as u64;
            let transmission_upper_q0_48 =
                u64::try_from(ceil_div_pow2(transmission_upper_q64, 16)?).map_err(|_| {
                    VisibleRadianceBulkError::Invalid("interval bulk transmission output overflow")
                })?;
            if transmission_upper_q0_48 < transmission_lower_q0_48
                || transmission_upper_q0_48 > TRANSMISSION_ONE_Q0_48
            {
                return Err(VisibleRadianceBulkError::Invalid(
                    "interval bulk transmission enclosure invalid",
                ));
            }
            Ok(BandTransferV1::Finite {
                optical_depth_lower_q64_64: FixedU128V1::from_u128(lower),
                optical_depth_upper_q64_64: FixedU128V1::from_u128(upper),
                transmission_lower_q0_48,
                transmission_upper_q0_48,
            })
        }
    }
}

fn parse_fixed(
    value: &SignedDecimalIntervalV1,
    required_bits: u16,
) -> Result<FixedInterval, VisibleRadianceBulkError> {
    if value.fractional_bits != required_bits {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk input scale mismatch",
        ));
    }
    FixedInterval::new(
        Signed512::from_canonical_decimal(&value.lower).map_err(map_arithmetic)?,
        Signed512::from_canonical_decimal(&value.upper).map_err(map_arithmetic)?,
        required_bits,
    )
    .map_err(map_arithmetic)
}

fn parse_and_lift(
    value: &SignedDecimalIntervalV1,
    required_bits: u16,
    shift: u16,
) -> Result<FixedInterval, VisibleRadianceBulkError> {
    let parsed = parse_fixed(value, required_bits)?;
    FixedInterval::new(
        arithmetic(parsed.lower().checked_shl(shift))?,
        arithmetic(parsed.upper().checked_shl(shift))?,
        required_bits + shift,
    )
    .map_err(map_arithmetic)
}

fn public_interval(value: &FixedInterval) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: value.fractional_bits(),
        lower: value.lower().canonical_decimal(),
        upper: value.upper().canonical_decimal(),
    }
}

fn nonnegative_u128(value: &Signed512) -> Result<u128, VisibleRadianceBulkError> {
    if value.is_negative() {
        return Err(VisibleRadianceBulkError::Invalid(
            "interval bulk negative optical depth",
        ));
    }
    value.canonical_decimal().parse::<u128>().map_err(|_| {
        VisibleRadianceBulkError::Invalid("interval bulk optical depth ceiling exceeded")
    })
}

fn arithmetic<T>(result: Result<T, FixedArithmeticError>) -> Result<T, VisibleRadianceBulkError> {
    result.map_err(map_arithmetic)
}

fn map_arithmetic(_: FixedArithmeticError) -> VisibleRadianceBulkError {
    VisibleRadianceBulkError::Invalid("interval bulk arithmetic defect")
}

fn ceil_div_pow2(value: u128, shift: u32) -> Result<u128, VisibleRadianceBulkError> {
    if shift == 0 {
        return Ok(value);
    }
    if shift >= 128 {
        return Ok(u128::from(value != 0));
    }
    let mask = (1_u128 << shift) - 1;
    Ok((value >> shift) + u128::from(value & mask != 0))
}

fn encode_capped<T: Serialize>(
    value: &T,
    maximum: usize,
    message: &'static str,
) -> Result<Vec<u8>, VisibleRadianceBulkError> {
    let bytes = encode(value)?;
    if bytes.len() > maximum {
        return Err(VisibleRadianceBulkError::Invalid(message));
    }
    Ok(bytes)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VisibleRadianceBulkError> {
    serde_json::from_slice(bytes)
        .map_err(|error| VisibleRadianceBulkError::Codec(error.to_string()))
}

fn interval_bulk_limitations() -> Vec<String> {
    vec![
        "one-band one-cell conditional bulk-transfer evidence only; no ordered lineage composition endpoint or visibility claim".into(),
        "no coefficient catalogue SI mapping perception rendering gameplay biome planet terrain runtime approval or promotion claim".into(),
    ]
}
