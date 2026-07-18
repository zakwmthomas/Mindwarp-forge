//! Checked cumulative dimensionless transfer over one validated optical lane.
//!
//! Every factor is derived from replayed owner evidence. This crate makes no
//! source-emission, receiver-arrival, visibility, perception, or authority claim.

use fixed_interval_arithmetic::Signed512;
use optical_lineage_binding::{
    OpticalLaneManifestV1, OpticalLineageBundleInputV1, OpticalLineageDispositionV1,
    OpticalLineageTerminalV1, validate_optical_lane_manifest,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use visible_radiance_bulk_transfer::{
    BandTransferV1, ConditionalIntervalBulkOutcomeV1, VisibleRadianceBandV1,
};
use visible_radiance_interface_event::{
    FixedScaleV1, IntervalBandOutcomeV1, IntervalInterfaceOutcomeV1, IntervalUniformBranchV1,
};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_CUMULATIVE_INPUT_BYTES: usize = 18 * 1024 * 1024;
pub const MAX_CUMULATIVE_OUTPUT_BYTES: usize = 256 * 1024;
pub const MAX_VALIDATION_LIVE_CANONICAL_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_CUMULATIVE_FACTORS: usize = 128;
pub const ACCUMULATOR_FRACTIONAL_BITS: u16 = 160;
pub const FACTOR_FRACTIONAL_BITS: u16 = 48;
pub const MAXIMUM_LIVE_BITS: u16 = 209;
pub const ONE_Q0_48: u64 = 1_u64 << FACTOR_FRACTIONAL_BITS;
pub type Id = [u8; 32];

const FACTOR_DOMAIN: &[u8] = b"mindwarp.optical-lineage.cumulative-factor.v1";
const RESULT_DOMAIN: &[u8] = b"mindwarp.optical-lineage.cumulative-result.v1";
const TRANSCRIPT_DOMAIN: &[u8] = b"mindwarp.optical-lineage.cumulative-transcript.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CumulativeLaneTransferError {
    Invalid(&'static str),
    Codec(String),
    Arithmetic,
}

impl core::fmt::Display for CumulativeLaneTransferError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Codec(message) => write!(formatter, "codec: {message}"),
            Self::Arithmetic => formatter.write_str("checked cumulative arithmetic failed"),
        }
    }
}

impl std::error::Error for CumulativeLaneTransferError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CumulativeOpticalLaneTransferInputV1 {
    pub schema_version: u16,
    pub bundle: OpticalLineageBundleInputV1,
    pub manifest: OpticalLaneManifestV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CumulativeLaneFactorRoleV1 {
    BulkTransfer,
    TransmittedInterface,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CumulativeLaneFactorReceiptV1 {
    pub step_ordinal: u8,
    pub role: CumulativeLaneFactorRoleV1,
    pub band: VisibleRadianceBandV1,
    pub owner_object_id: Id,
    pub lower_q0_48: u64,
    pub upper_q0_48: u64,
    pub factor_id: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CumulativeLaneArithmeticReceiptV1 {
    pub accumulator_fractional_bits: u16,
    pub factor_fractional_bits: u16,
    pub storage_bits: u16,
    pub maximum_live_bits: u16,
    pub observed_maximum_live_bits: u16,
    pub factor_count: u16,
    pub lower_multiplications: u16,
    pub upper_multiplications: u16,
    pub floor_divisions: u16,
    pub ceiling_divisions: u16,
    pub final_projection_pairs: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CumulativeOpticalLaneTransferV1 {
    pub schema_version: u16,
    pub bundle_sha256: Id,
    pub lineage_transcript_id: Id,
    pub lane_id: Id,
    pub band: VisibleRadianceBandV1,
    pub factors: Vec<CumulativeLaneFactorReceiptV1>,
    pub cumulative_lower_q0_160: String,
    pub cumulative_upper_q0_160: String,
    pub cumulative_lower_q0_48: u64,
    pub cumulative_upper_q0_48: u64,
    pub final_terminal: OpticalLineageTerminalV1,
    pub arithmetic_receipt: CumulativeLaneArithmeticReceiptV1,
    pub result_id: Id,
    pub transcript_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl CumulativeOpticalLaneTransferInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CumulativeLaneTransferError> {
        let result = compile_cumulative_optical_lane_transfer(self)?;
        let bytes = encode_capped(
            self,
            MAX_CUMULATIVE_INPUT_BYTES,
            "cumulative input byte ceiling",
        )?;
        enforce_live_bytes(bytes.len(), result.to_bytes(self)?.len())?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CumulativeLaneTransferError> {
        if bytes.len() > MAX_CUMULATIVE_INPUT_BYTES {
            return Err(CumulativeLaneTransferError::Invalid(
                "cumulative input byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(CumulativeLaneTransferError::Invalid(
                "cumulative input canonical drift",
            ));
        }
        Ok(value)
    }
}

impl CumulativeOpticalLaneTransferV1 {
    pub fn to_bytes(
        &self,
        input: &CumulativeOpticalLaneTransferInputV1,
    ) -> Result<Vec<u8>, CumulativeLaneTransferError> {
        validate_cumulative_optical_lane_transfer(input, self)?;
        encode_capped(
            self,
            MAX_CUMULATIVE_OUTPUT_BYTES,
            "cumulative output byte ceiling",
        )
    }

    pub fn from_bytes(
        bytes: &[u8],
        input: &CumulativeOpticalLaneTransferInputV1,
    ) -> Result<Self, CumulativeLaneTransferError> {
        if bytes.len() > MAX_CUMULATIVE_OUTPUT_BYTES {
            return Err(CumulativeLaneTransferError::Invalid(
                "cumulative output byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(input)? != bytes {
            return Err(CumulativeLaneTransferError::Invalid(
                "cumulative output canonical drift",
            ));
        }
        Ok(value)
    }
}

pub fn compile_cumulative_optical_lane_transfer(
    input: &CumulativeOpticalLaneTransferInputV1,
) -> Result<CumulativeOpticalLaneTransferV1, CumulativeLaneTransferError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative input schema version",
        ));
    }
    validate_optical_lane_manifest(&input.bundle, &input.manifest)
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative lineage replay"))?;
    let bundle_bytes = input
        .bundle
        .to_bytes()
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative bundle replay"))?;
    let manifest_bytes = input
        .manifest
        .to_bytes(&input.bundle)
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative manifest replay"))?;
    enforce_live_bytes(bundle_bytes.len(), manifest_bytes.len())?;
    let bundle_sha256 = Sha256::digest(&bundle_bytes).into();
    let factors = extract_factors(input)?;
    let (lower, upper, lower_q48, upper_q48, arithmetic_receipt) = accumulate(&factors)?;
    let limitations = limitations();
    let authority_effect = "none_evidence_only".to_owned();
    let lower_decimal = lower.canonical_decimal();
    let upper_decimal = upper.canonical_decimal();
    let result_id = domain_hash(
        RESULT_DOMAIN,
        &encode(&(
            bundle_sha256,
            input.manifest.transcript_id,
            input.manifest.lane_id,
            input.manifest.band,
            &factors,
            &lower_decimal,
            &upper_decimal,
            lower_q48,
            upper_q48,
            input.manifest.final_terminal,
            arithmetic_receipt,
            &limitations,
            &authority_effect,
        ))?,
    );
    let transcript_id = domain_hash(
        TRANSCRIPT_DOMAIN,
        &encode(&(bundle_sha256, input.manifest.transcript_id, result_id))?,
    );
    let result = CumulativeOpticalLaneTransferV1 {
        schema_version: CONTRACT_VERSION,
        bundle_sha256,
        lineage_transcript_id: input.manifest.transcript_id,
        lane_id: input.manifest.lane_id,
        band: input.manifest.band,
        factors,
        cumulative_lower_q0_160: lower_decimal,
        cumulative_upper_q0_160: upper_decimal,
        cumulative_lower_q0_48: lower_q48,
        cumulative_upper_q0_48: upper_q48,
        final_terminal: input.manifest.final_terminal,
        arithmetic_receipt,
        result_id,
        transcript_id,
        limitations,
        authority_effect,
    };
    let output_bytes = encode_capped(
        &result,
        MAX_CUMULATIVE_OUTPUT_BYTES,
        "cumulative output byte ceiling",
    )?;
    enforce_live_bytes(
        bundle_bytes.len() + manifest_bytes.len(),
        output_bytes.len(),
    )?;
    Ok(result)
}

pub fn validate_cumulative_optical_lane_transfer(
    input: &CumulativeOpticalLaneTransferInputV1,
    result: &CumulativeOpticalLaneTransferV1,
) -> Result<(), CumulativeLaneTransferError> {
    if &compile_cumulative_optical_lane_transfer(input)? != result {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative result drift",
        ));
    }
    Ok(())
}

fn extract_factors(
    input: &CumulativeOpticalLaneTransferInputV1,
) -> Result<Vec<CumulativeLaneFactorReceiptV1>, CumulativeLaneTransferError> {
    let mut factors = Vec::new();
    for (ordinal, (evidence, step)) in input
        .bundle
        .steps
        .iter()
        .zip(&input.manifest.steps)
        .enumerate()
    {
        if let ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
            band_transfer, ..
        } = &evidence.bulk_transfer.outcome
        {
            let endpoints = match band_transfer {
                BandTransferV1::VacuumIdentity => (ONE_Q0_48, ONE_Q0_48),
                BandTransferV1::Opaque => (0, 0),
                BandTransferV1::Finite {
                    transmission_lower_q0_48,
                    transmission_upper_q0_48,
                    ..
                } => (*transmission_lower_q0_48, *transmission_upper_q0_48),
            };
            push_factor(
                &mut factors,
                ordinal,
                CumulativeLaneFactorRoleV1::BulkTransfer,
                input.manifest.band,
                evidence.bulk_transfer.conditional_interval_bulk_transfer_id,
                endpoints,
            )?;
        }
        if step.disposition == OpticalLineageDispositionV1::ContinueAfterInterface {
            let event =
                evidence
                    .interface_event
                    .as_ref()
                    .ok_or(CumulativeLaneTransferError::Invalid(
                        "cumulative interface evidence",
                    ))?;
            let bands = match &event.outcome {
                IntervalInterfaceOutcomeV1::Evaluated { bands_rgb, .. } => bands_rgb,
                _ => {
                    return Err(CumulativeLaneTransferError::Invalid(
                        "cumulative interface outcome",
                    ));
                }
            };
            let transmitted = match &bands[band_index(input.manifest.band)] {
                IntervalBandOutcomeV1::BoundedEnclosure {
                    branch: IntervalUniformBranchV1::AllTransmit,
                    event,
                } => &event.transmitted_power,
                _ => {
                    return Err(CumulativeLaneTransferError::Invalid(
                        "cumulative selected interface",
                    ));
                }
            };
            if transmitted.scale != FixedScaleV1::Q0_48 {
                return Err(CumulativeLaneTransferError::Invalid(
                    "cumulative interface scale",
                ));
            }
            push_factor(
                &mut factors,
                ordinal,
                CumulativeLaneFactorRoleV1::TransmittedInterface,
                input.manifest.band,
                event.event_id,
                (
                    parse_q48(&transmitted.lower)?,
                    parse_q48(&transmitted.upper)?,
                ),
            )?;
        }
    }
    if factors.len() > MAX_CUMULATIVE_FACTORS {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative factor ceiling",
        ));
    }
    Ok(factors)
}

fn push_factor(
    factors: &mut Vec<CumulativeLaneFactorReceiptV1>,
    ordinal: usize,
    role: CumulativeLaneFactorRoleV1,
    band: VisibleRadianceBandV1,
    owner_object_id: Id,
    endpoints: (u64, u64),
) -> Result<(), CumulativeLaneTransferError> {
    if endpoints.0 > endpoints.1 || endpoints.1 > ONE_Q0_48 {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative factor endpoints",
        ));
    }
    let step_ordinal = u8::try_from(ordinal)
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative factor ordinal"))?;
    let factor_id = domain_hash(
        FACTOR_DOMAIN,
        &encode(&(
            step_ordinal,
            role,
            band,
            owner_object_id,
            endpoints.0,
            endpoints.1,
        ))?,
    );
    factors.push(CumulativeLaneFactorReceiptV1 {
        step_ordinal,
        role,
        band,
        owner_object_id,
        lower_q0_48: endpoints.0,
        upper_q0_48: endpoints.1,
        factor_id,
    });
    Ok(())
}

fn accumulate(
    factors: &[CumulativeLaneFactorReceiptV1],
) -> Result<
    (
        Signed512,
        Signed512,
        u64,
        u64,
        CumulativeLaneArithmeticReceiptV1,
    ),
    CumulativeLaneTransferError,
> {
    if factors.len() > MAX_CUMULATIVE_FACTORS {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative factor ceiling",
        ));
    }
    if factors
        .iter()
        .any(|factor| factor.lower_q0_48 > factor.upper_q0_48 || factor.upper_q0_48 > ONE_Q0_48)
    {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative factor endpoints",
        ));
    }
    let scale = Signed512::one()
        .checked_shl(FACTOR_FRACTIONAL_BITS)
        .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
    let mut lower = Signed512::one()
        .checked_shl(ACCUMULATOR_FRACTIONAL_BITS)
        .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
    let mut upper = lower.clone();
    let mut observed = lower.maximum_magnitude_bits();
    for factor in factors {
        let lower_factor = Signed512::from_i128(i128::from(factor.lower_q0_48));
        let upper_factor = Signed512::from_i128(i128::from(factor.upper_q0_48));
        let lower_product = lower
            .checked_mul(&lower_factor)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
        let upper_product = upper
            .checked_mul(&upper_factor)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
        observed = observed
            .max(lower_product.maximum_magnitude_bits())
            .max(upper_product.maximum_magnitude_bits());
        if observed > MAXIMUM_LIVE_BITS {
            return Err(CumulativeLaneTransferError::Invalid(
                "cumulative live-bit shield",
            ));
        }
        lower = lower_product
            .div_floor(&scale)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
        upper = upper_product
            .div_ceil(&scale)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
    }
    let projection = Signed512::one()
        .checked_shl(ACCUMULATOR_FRACTIONAL_BITS - FACTOR_FRACTIONAL_BITS)
        .map_err(|_| CumulativeLaneTransferError::Arithmetic)?;
    let lower_q48 = decimal_to_u64(
        &lower
            .div_floor(&projection)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?
            .canonical_decimal(),
    )?;
    let upper_q48 = decimal_to_u64(
        &upper
            .div_ceil(&projection)
            .map_err(|_| CumulativeLaneTransferError::Arithmetic)?
            .canonical_decimal(),
    )?;
    let count = u16::try_from(factors.len())
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative factor ceiling"))?;
    Ok((
        lower,
        upper,
        lower_q48,
        upper_q48,
        CumulativeLaneArithmeticReceiptV1 {
            accumulator_fractional_bits: ACCUMULATOR_FRACTIONAL_BITS,
            factor_fractional_bits: FACTOR_FRACTIONAL_BITS,
            storage_bits: 512,
            maximum_live_bits: MAXIMUM_LIVE_BITS,
            observed_maximum_live_bits: observed,
            factor_count: count,
            lower_multiplications: count,
            upper_multiplications: count,
            floor_divisions: count,
            ceiling_divisions: count,
            final_projection_pairs: 1,
        },
    ))
}

fn parse_q48(value: &str) -> Result<u64, CumulativeLaneTransferError> {
    let parsed = decimal_to_u64(value)?;
    if parsed > ONE_Q0_48 {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative interface endpoint",
        ));
    }
    Ok(parsed)
}

fn decimal_to_u64(value: &str) -> Result<u64, CumulativeLaneTransferError> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| CumulativeLaneTransferError::Invalid("cumulative unsigned decimal"))?;
    if parsed.to_string() != value {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative canonical decimal",
        ));
    }
    Ok(parsed)
}

fn band_index(band: VisibleRadianceBandV1) -> usize {
    match band {
        VisibleRadianceBandV1::Red => 0,
        VisibleRadianceBandV1::Green => 1,
        VisibleRadianceBandV1::Blue => 2,
    }
}

fn limitations() -> Vec<String> {
    vec![
        "dimensionless cumulative followed-lane transfer enclosure only; no source emission receiver arrival inverse-square spreading or visibility claim".into(),
        "no detector perception rendering gameplay line-of-sight runtime persistence approval promotion or C3 closure claim".into(),
    ]
}

fn enforce_live_bytes(left: usize, right: usize) -> Result<(), CumulativeLaneTransferError> {
    if left
        .checked_add(right)
        .is_none_or(|sum| sum > MAX_VALIDATION_LIVE_CANONICAL_BYTES)
    {
        return Err(CumulativeLaneTransferError::Invalid(
            "cumulative live canonical byte ceiling",
        ));
    }
    Ok(())
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CumulativeLaneTransferError> {
    serde_json::to_vec(value).map_err(|error| CumulativeLaneTransferError::Codec(error.to_string()))
}

fn encode_capped<T: Serialize>(
    value: &T,
    maximum: usize,
    message: &'static str,
) -> Result<Vec<u8>, CumulativeLaneTransferError> {
    let bytes = encode(value)?;
    if bytes.len() > maximum {
        return Err(CumulativeLaneTransferError::Invalid(message));
    }
    Ok(bytes)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CumulativeLaneTransferError> {
    serde_json::from_slice(bytes)
        .map_err(|error| CumulativeLaneTransferError::Codec(error.to_string()))
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update([0]);
    hash.update(bytes);
    hash.finalize().into()
}

#[cfg(test)]
mod arithmetic_tests {
    use super::*;

    fn factor(lower: u64, upper: u64) -> CumulativeLaneFactorReceiptV1 {
        CumulativeLaneFactorReceiptV1 {
            step_ordinal: 0,
            role: CumulativeLaneFactorRoleV1::BulkTransfer,
            band: VisibleRadianceBandV1::Red,
            owner_object_id: [1; 32],
            lower_q0_48: lower,
            upper_q0_48: upper,
            factor_id: [2; 32],
        }
    }

    #[test]
    fn directed_q160_fold_preserves_sub_q48_products_until_final_projection() {
        let factors = vec![factor(1, 1), factor(1, 1)];
        let (lower, upper, lower_q48, upper_q48, receipt) = accumulate(&factors).unwrap();
        assert_eq!(lower.canonical_decimal(), "18446744073709551616");
        assert_eq!(upper.canonical_decimal(), "18446744073709551616");
        assert_eq!((lower_q48, upper_q48), (0, 1));
        assert_eq!(receipt.factor_count, 2);
        assert!(receipt.observed_maximum_live_bits <= MAXIMUM_LIVE_BITS);
    }

    #[test]
    fn one_hundred_twenty_eight_factors_are_bounded_and_costed_exactly() {
        let factors = vec![factor(ONE_Q0_48, ONE_Q0_48); MAX_CUMULATIVE_FACTORS];
        let (_, _, lower_q48, upper_q48, receipt) = accumulate(&factors).unwrap();
        assert_eq!((lower_q48, upper_q48), (ONE_Q0_48, ONE_Q0_48));
        assert_eq!(receipt.lower_multiplications, 128);
        assert_eq!(receipt.ceiling_divisions, 128);
        assert_eq!(receipt.observed_maximum_live_bits, MAXIMUM_LIVE_BITS);
        assert!(accumulate(&vec![factor(1, 1); MAX_CUMULATIVE_FACTORS + 1]).is_err());
        assert!(accumulate(&[factor(ONE_Q0_48, ONE_Q0_48 + 1)]).is_err());
    }
}
