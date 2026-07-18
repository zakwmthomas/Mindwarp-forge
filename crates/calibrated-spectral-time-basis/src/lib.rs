#![deny(warnings)]
//! Capability-free physical spectral/time calibration evidence.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, fmt};

pub const CONTRACT_VERSION: u32 = 1;
pub const MAX_DECIMAL_DIGITS: usize = 39;
pub const MAX_INPUT_BYTES: usize = 16 * 1024;
pub const MAX_RESULT_BYTES: usize = 32 * 1024;
pub const MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 64 * 1024;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str = "no source allocation transport applicability spatial calibration detector visibility runtime promotion or C3 closure";

const BASIS_DOMAIN: &[u8] = b"mindwarp.calibrated-spectral-time-basis.basis.v1";
const LEGACY_TIME_DOMAIN: &[u8] =
    b"mindwarp.calibrated-spectral-time-basis.legacy-time-commitment.v1";
const LEGACY_BAND_TIME_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.band-time.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalibratedSpectralTimeBasisError {
    InvalidSchema,
    InvalidInput(&'static str),
    ByteCeiling,
    AggregateByteCeiling,
    IdentityMismatch,
    CodecDefect,
}

impl fmt::Display for CalibratedSpectralTimeBasisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for CalibratedSpectralTimeBasisError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CalibratedBandV1 {
    Blue,
    Green,
    Red,
}

impl CalibratedBandV1 {
    const ORDERED: [Self; 3] = [Self::Blue, Self::Green, Self::Red];

    fn name(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Red => "red",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactUnsignedRationalV1 {
    pub denominator: String,
    pub numerator: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSpectralIntervalV1 {
    pub band: CalibratedBandV1,
    pub lower: ExactUnsignedRationalV1,
    pub upper: ExactUnsignedRationalV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedTimeCellV1 {
    pub clock_origin_id: [u8; 32],
    pub end_tick: u64,
    pub seconds_per_tick: ExactUnsignedRationalV1,
    pub start_tick: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSpectralTimeBasisInputV1 {
    pub basis_version: u32,
    pub calibration_provenance_id: [u8; 32],
    pub quantity_kind: String,
    pub schema_version: u32,
    pub spectral_coordinate: String,
    pub spectral_intervals: [CalibratedSpectralIntervalV1; 3],
    pub spectral_weighting: String,
    pub time_cell: CalibratedTimeCellV1,
    pub unit: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivedLegacyBandTimeIdsV1 {
    pub blue: [u8; 32],
    pub green: [u8; 32],
    pub red: [u8; 32],
}

impl DerivedLegacyBandTimeIdsV1 {
    pub fn get(self, band: CalibratedBandV1) -> [u8; 32] {
        match band {
            CalibratedBandV1::Blue => self.blue,
            CalibratedBandV1::Green => self.green,
            CalibratedBandV1::Red => self.red,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSpectralTimeBasisV1 {
    pub authority_effect: String,
    pub calibrated_basis_id: [u8; 32],
    pub derived_legacy_band_time_ids: DerivedLegacyBandTimeIdsV1,
    pub derived_legacy_time_basis_id: [u8; 32],
    pub input: CalibratedSpectralTimeBasisInputV1,
    pub limitations: String,
    pub schema_version: u32,
}

impl ExactUnsignedRationalV1 {
    fn parsed(&self, positive: bool) -> Result<(u128, u128), CalibratedSpectralTimeBasisError> {
        let numerator = parse_decimal(&self.numerator, positive)?;
        let denominator = parse_decimal(&self.denominator, true)?;
        if gcd(numerator, denominator) != 1 {
            return Err(CalibratedSpectralTimeBasisError::InvalidInput(
                "non-reduced rational",
            ));
        }
        Ok((numerator, denominator))
    }
}

impl CalibratedSpectralTimeBasisInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CalibratedSpectralTimeBasisError> {
        validate_input(self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(CalibratedSpectralTimeBasisError::ByteCeiling);
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CalibratedSpectralTimeBasisError> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(CalibratedSpectralTimeBasisError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? == bytes {
            Ok(value)
        } else {
            Err(CalibratedSpectralTimeBasisError::CodecDefect)
        }
    }
}

impl CalibratedSpectralTimeBasisV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CalibratedSpectralTimeBasisError> {
        validate_calibrated_spectral_time_basis(self)?;
        let input_bytes = self.input.to_bytes()?;
        let bytes = json(self)?;
        check_result_bytes(input_bytes.len(), bytes.len())?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CalibratedSpectralTimeBasisError> {
        if bytes.len() > MAX_RESULT_BYTES {
            return Err(CalibratedSpectralTimeBasisError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? == bytes {
            Ok(value)
        } else {
            Err(CalibratedSpectralTimeBasisError::CodecDefect)
        }
    }
}

pub fn compile_calibrated_spectral_time_basis(
    input: &CalibratedSpectralTimeBasisInputV1,
) -> Result<CalibratedSpectralTimeBasisV1, CalibratedSpectralTimeBasisError> {
    let input_bytes = input.to_bytes()?;
    let calibrated_basis_id = domain_hash(BASIS_DOMAIN, &input_bytes);
    let legacy_time_bytes = json(&calibrated_basis_id)?;
    let derived_legacy_time_basis_id = domain_hash(LEGACY_TIME_DOMAIN, &legacy_time_bytes);
    let band_id = |band: CalibratedBandV1| -> Result<[u8; 32], CalibratedSpectralTimeBasisError> {
        let bytes = json(&(band.name(), derived_legacy_time_basis_id))?;
        Ok(domain_hash(LEGACY_BAND_TIME_DOMAIN, &bytes))
    };
    let result = CalibratedSpectralTimeBasisV1 {
        authority_effect: AUTHORITY_EFFECT_NONE.to_owned(),
        calibrated_basis_id,
        derived_legacy_band_time_ids: DerivedLegacyBandTimeIdsV1 {
            blue: band_id(CalibratedBandV1::Blue)?,
            green: band_id(CalibratedBandV1::Green)?,
            red: band_id(CalibratedBandV1::Red)?,
        },
        derived_legacy_time_basis_id,
        input: input.clone(),
        limitations: LIMITATIONS_V1.to_owned(),
        schema_version: CONTRACT_VERSION,
    };
    let result_bytes = json(&result)?;
    check_result_bytes(input_bytes.len(), result_bytes.len())?;
    Ok(result)
}

pub fn validate_calibrated_spectral_time_basis(
    value: &CalibratedSpectralTimeBasisV1,
) -> Result<(), CalibratedSpectralTimeBasisError> {
    if value.schema_version != CONTRACT_VERSION {
        return Err(CalibratedSpectralTimeBasisError::InvalidSchema);
    }
    if value.authority_effect != AUTHORITY_EFFECT_NONE || value.limitations != LIMITATIONS_V1 {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "authority or limitations drift",
        ));
    }
    let expected = compile_calibrated_spectral_time_basis(&value.input)?;
    if &expected != value {
        return Err(CalibratedSpectralTimeBasisError::IdentityMismatch);
    }
    Ok(())
}

fn validate_input(
    input: &CalibratedSpectralTimeBasisInputV1,
) -> Result<(), CalibratedSpectralTimeBasisError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(CalibratedSpectralTimeBasisError::InvalidSchema);
    }
    if input.basis_version == 0 {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "zero basis version",
        ));
    }
    nonzero_id(
        input.calibration_provenance_id,
        "zero calibration provenance",
    )?;
    if input.quantity_kind != "radiant_energy" || input.unit != "joule" {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "quantity basis",
        ));
    }
    if input.spectral_coordinate != "vacuum_wavelength_metre" {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "spectral coordinate",
        ));
    }
    if input.spectral_weighting != "unit_energy_integral" {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "spectral weighting",
        ));
    }
    let mut previous_upper = None;
    for (expected_band, interval) in CalibratedBandV1::ORDERED
        .into_iter()
        .zip(&input.spectral_intervals)
    {
        if interval.band != expected_band {
            return Err(CalibratedSpectralTimeBasisError::InvalidInput("band order"));
        }
        let lower = interval.lower.parsed(false)?;
        let upper = interval.upper.parsed(true)?;
        if compare_rationals(lower, upper) != Ordering::Less {
            return Err(CalibratedSpectralTimeBasisError::InvalidInput(
                "empty or reversed interval",
            ));
        }
        if previous_upper.is_some_and(|value| compare_rationals(value, lower) != Ordering::Equal) {
            return Err(CalibratedSpectralTimeBasisError::InvalidInput(
                "spectral gap or overlap",
            ));
        }
        previous_upper = Some(upper);
    }
    nonzero_id(input.time_cell.clock_origin_id, "zero clock origin")?;
    if input.time_cell.start_tick >= input.time_cell.end_tick {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "empty time cell",
        ));
    }
    input.time_cell.seconds_per_tick.parsed(true)?;
    Ok(())
}

fn parse_decimal(text: &str, positive: bool) -> Result<u128, CalibratedSpectralTimeBasisError> {
    if text.is_empty()
        || text.len() > MAX_DECIMAL_DIGITS
        || !text.bytes().all(|byte| byte.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "noncanonical decimal",
        ));
    }
    let value = text
        .parse::<u128>()
        .map_err(|_| CalibratedSpectralTimeBasisError::InvalidInput("u128 overflow"))?;
    if positive && value == 0 {
        return Err(CalibratedSpectralTimeBasisError::InvalidInput(
            "positive value required",
        ));
    }
    Ok(value)
}

fn compare_rationals(mut left: (u128, u128), mut right: (u128, u128)) -> Ordering {
    let mut reversed = false;
    loop {
        let left_quotient = left.0 / left.1;
        let right_quotient = right.0 / right.1;
        if left_quotient != right_quotient {
            return if reversed {
                right_quotient.cmp(&left_quotient)
            } else {
                left_quotient.cmp(&right_quotient)
            };
        }
        let left_remainder = left.0 % left.1;
        let right_remainder = right.0 % right.1;
        match (left_remainder == 0, right_remainder == 0) {
            (true, true) => return Ordering::Equal,
            (true, false) => {
                return if reversed {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
            (false, true) => {
                return if reversed {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
            (false, false) => {
                left = (left.1, left_remainder);
                right = (right.1, right_remainder);
                reversed = !reversed;
            }
        }
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn nonzero_id(
    value: [u8; 32],
    reason: &'static str,
) -> Result<(), CalibratedSpectralTimeBasisError> {
    if value == [0; 32] {
        Err(CalibratedSpectralTimeBasisError::InvalidInput(reason))
    } else {
        Ok(())
    }
}

fn check_result_bytes(
    input_bytes: usize,
    result_bytes: usize,
) -> Result<(), CalibratedSpectralTimeBasisError> {
    if result_bytes > MAX_RESULT_BYTES {
        return Err(CalibratedSpectralTimeBasisError::ByteCeiling);
    }
    let aggregate = input_bytes
        .checked_mul(2)
        .and_then(|value| value.checked_add(result_bytes))
        .ok_or(CalibratedSpectralTimeBasisError::AggregateByteCeiling)?;
    if aggregate > MAX_AGGREGATE_LIVE_CANONICAL_BYTES {
        return Err(CalibratedSpectralTimeBasisError::AggregateByteCeiling);
    }
    Ok(())
}

fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, CalibratedSpectralTimeBasisError> {
    serde_json::to_vec(value).map_err(|_| CalibratedSpectralTimeBasisError::CodecDefect)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CalibratedSpectralTimeBasisError> {
    serde_json::from_slice(bytes).map_err(|_| CalibratedSpectralTimeBasisError::CodecDefect)
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
