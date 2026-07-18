//! Strict, capability-free stellar/orbital causal contract.
//!
//! This crate proves only a bounded two-body reference seam: a declared
//! stellar source and elliptical orbit deterministically produce distance,
//! irradiation, and normalized-period-squared bounds. It does not model
//! multi-star dynamics, perturbations, orbital phase, stellar evolution,
//! geology, atmosphere, climate, habitability, or runtime ephemerides.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
const RATIO_SCALE: u128 = 1_000_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StellarOrbitalInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub stellar_source_id: [u8; 32],
    pub primary_mass_milli_solar: u32,
    pub stellar_luminosity_millionths_solar: u32,
    pub stellar_spectrum_rgb_permille: [u16; 3],
    pub semi_major_axis_milli_au: u32,
    pub eccentricity_millionths: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StellarOrbitalContent {
    pub schema_version: u16,
    pub input_id: String,
    pub periapsis_milli_au: u32,
    pub apoapsis_milli_au: u32,
    pub irradiance_min_millionths_earth: u64,
    pub irradiance_mean_distance_millionths_earth: u64,
    pub irradiance_max_millionths_earth: u64,
    pub orbital_period_squared_millionths_earth_year_squared: u64,
    pub stellar_spectrum_rgb_permille: [u16; 3],
    pub bounded_stellar_irradiance_rgb_permille: [u16; 3],
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StellarOrbitalState {
    pub state_id: String,
    pub content: StellarOrbitalContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StellarOrbitalContract {
    pub input: StellarOrbitalInput,
    pub state: StellarOrbitalState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StellarOrbitalError {
    Invalid(&'static str),
    Codec(String),
}

impl StellarOrbitalInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, StellarOrbitalError> {
        validate_input(self)?;
        serde_json::to_vec(self).map_err(|error| StellarOrbitalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StellarOrbitalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| StellarOrbitalError::Codec(error.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(StellarOrbitalError::Invalid("noncanonical input bytes"));
        }
        Ok(value)
    }
}

impl StellarOrbitalState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, StellarOrbitalError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(StellarOrbitalError::Invalid("state identity drift"));
        }
        serde_json::to_vec(self).map_err(|error| StellarOrbitalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StellarOrbitalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| StellarOrbitalError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(StellarOrbitalError::Invalid(
                "noncanonical or drifted state",
            ));
        }
        Ok(value)
    }
}

impl StellarOrbitalContract {
    pub fn to_bytes(&self) -> Result<Vec<u8>, StellarOrbitalError> {
        validate_stellar_orbital(self)?;
        serde_json::to_vec(self).map_err(|error| StellarOrbitalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StellarOrbitalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| StellarOrbitalError::Codec(error.to_string()))?;
        validate_stellar_orbital(&value)?;
        if value.to_bytes()? != bytes {
            return Err(StellarOrbitalError::Invalid("noncanonical contract bytes"));
        }
        Ok(value)
    }
}

pub fn compile_stellar_orbital(
    input: &StellarOrbitalInput,
) -> Result<StellarOrbitalContract, StellarOrbitalError> {
    let input_bytes = input.to_bytes()?;
    let input_id = hex(&domain_hash(
        b"mindwarp.stellar-orbital.input.v1\0",
        &input_bytes,
    ));
    let eccentricity = u128::from(input.eccentricity_millionths);
    let axis = u128::from(input.semi_major_axis_milli_au);
    let periapsis = rounded_div(axis * (RATIO_SCALE - eccentricity), RATIO_SCALE);
    let apoapsis = rounded_div(axis * (RATIO_SCALE + eccentricity), RATIO_SCALE);
    let periapsis_milli_au =
        u32::try_from(periapsis).map_err(|_| StellarOrbitalError::Invalid("periapsis overflow"))?;
    let apoapsis_milli_au =
        u32::try_from(apoapsis).map_err(|_| StellarOrbitalError::Invalid("apoapsis overflow"))?;
    if periapsis_milli_au == 0 {
        return Err(StellarOrbitalError::Invalid(
            "periapsis below contract resolution",
        ));
    }

    let content = StellarOrbitalContent {
        schema_version: CONTRACT_VERSION,
        input_id,
        periapsis_milli_au,
        apoapsis_milli_au,
        irradiance_min_millionths_earth: irradiance_at(
            input.stellar_luminosity_millionths_solar,
            apoapsis_milli_au,
        )?,
        irradiance_mean_distance_millionths_earth: irradiance_at(
            input.stellar_luminosity_millionths_solar,
            input.semi_major_axis_milli_au,
        )?,
        irradiance_max_millionths_earth: irradiance_at(
            input.stellar_luminosity_millionths_solar,
            periapsis_milli_au,
        )?,
        orbital_period_squared_millionths_earth_year_squared: u64::try_from(rounded_div(
            axis * axis * axis,
            u128::from(input.primary_mass_milli_solar),
        ))
        .map_err(|_| StellarOrbitalError::Invalid("period ratio overflow"))?,
        stellar_spectrum_rgb_permille: input.stellar_spectrum_rgb_permille,
        bounded_stellar_irradiance_rgb_permille: bounded_rgb_irradiance(
            input.stellar_spectrum_rgb_permille,
            irradiance_at(
                input.stellar_luminosity_millionths_solar,
                input.semi_major_axis_milli_au,
            )?,
        )?,
        limitations: vec![
            "bounded two-body reference; not a production ephemeris or scientific validation"
                .into(),
            "no multi-star perturbation phase geology atmosphere climate habitability or runtime claim"
                .into(),
        ],
        authority_effect: "none_evidence_only".into(),
    };
    validate_content(&content)?;
    Ok(StellarOrbitalContract {
        input: input.clone(),
        state: StellarOrbitalState {
            state_id: state_id(&content)?,
            content,
        },
    })
}

pub fn validate_stellar_orbital(
    contract: &StellarOrbitalContract,
) -> Result<(), StellarOrbitalError> {
    let expected = compile_stellar_orbital(&contract.input)?;
    if expected.state != contract.state {
        return Err(StellarOrbitalError::Invalid("stellar/orbital state drift"));
    }
    Ok(())
}

fn validate_input(input: &StellarOrbitalInput) -> Result<(), StellarOrbitalError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(StellarOrbitalError::Invalid("unsupported contract version"));
    }
    if input.reconstruction_id == [0; 32] || input.stellar_source_id == [0; 32] {
        return Err(StellarOrbitalError::Invalid("missing identity binding"));
    }
    if !(1..=1_000_000).contains(&input.primary_mass_milli_solar)
        || input.stellar_luminosity_millionths_solar > 1_000_000_000
    {
        return Err(StellarOrbitalError::Invalid("stellar range"));
    }
    if input
        .stellar_spectrum_rgb_permille
        .iter()
        .any(|value| *value > 1_000)
        || input
            .stellar_spectrum_rgb_permille
            .iter()
            .map(|value| u32::from(*value))
            .sum::<u32>()
            != 1_000
    {
        return Err(StellarOrbitalError::Invalid(
            "stellar spectrum range or sum",
        ));
    }
    if !(1..=1_000_000).contains(&input.semi_major_axis_milli_au)
        || input.eccentricity_millionths >= 1_000_000
    {
        return Err(StellarOrbitalError::Invalid("orbital range"));
    }
    Ok(())
}

fn validate_content(content: &StellarOrbitalContent) -> Result<(), StellarOrbitalError> {
    if content.schema_version != CONTRACT_VERSION {
        return Err(StellarOrbitalError::Invalid("unsupported state schema"));
    }
    if !valid_hex_id(&content.input_id) {
        return Err(StellarOrbitalError::Invalid("malformed input identity"));
    }
    if content.periapsis_milli_au == 0
        || content.periapsis_milli_au > content.apoapsis_milli_au
        || content.irradiance_min_millionths_earth
            > content.irradiance_mean_distance_millionths_earth
        || content.irradiance_mean_distance_millionths_earth
            > content.irradiance_max_millionths_earth
        || content.orbital_period_squared_millionths_earth_year_squared == 0
    {
        return Err(StellarOrbitalError::Invalid("state physical invariant"));
    }
    if content
        .stellar_spectrum_rgb_permille
        .iter()
        .map(|value| u32::from(*value))
        .sum::<u32>()
        != 1_000
    {
        return Err(StellarOrbitalError::Invalid("state spectrum invariant"));
    }
    if content
        .bounded_stellar_irradiance_rgb_permille
        .iter()
        .any(|value| *value > 1_000)
    {
        return Err(StellarOrbitalError::Invalid(
            "bounded stellar irradiance invariant",
        ));
    }
    if content.limitations
        != [
            "bounded two-body reference; not a production ephemeris or scientific validation",
            "no multi-star perturbation phase geology atmosphere climate habitability or runtime claim",
        ]
        .map(String::from)
        .to_vec()
        || content.authority_effect != "none_evidence_only"
    {
        return Err(StellarOrbitalError::Invalid(
            "state claim or authority drift",
        ));
    }
    Ok(())
}

fn irradiance_at(
    luminosity_millionths_solar: u32,
    distance_milli_au: u32,
) -> Result<u64, StellarOrbitalError> {
    if distance_milli_au == 0 {
        return Err(StellarOrbitalError::Invalid("zero irradiation distance"));
    }
    let numerator = u128::from(luminosity_millionths_solar) * RATIO_SCALE;
    let distance = u128::from(distance_milli_au);
    u64::try_from(rounded_div(numerator, distance * distance))
        .map_err(|_| StellarOrbitalError::Invalid("irradiance overflow"))
}

fn bounded_rgb_irradiance(
    spectrum_rgb_permille: [u16; 3],
    mean_irradiance_millionths_earth: u64,
) -> Result<[u16; 3], StellarOrbitalError> {
    let mut result = [0_u16; 3];
    for (index, spectrum) in spectrum_rgb_permille.into_iter().enumerate() {
        let scaled = rounded_div(
            u128::from(spectrum) * u128::from(mean_irradiance_millionths_earth),
            RATIO_SCALE,
        )
        .min(1_000);
        result[index] = u16::try_from(scaled)
            .map_err(|_| StellarOrbitalError::Invalid("bounded irradiance overflow"))?;
    }
    Ok(result)
}

fn rounded_div(numerator: u128, denominator: u128) -> u128 {
    (numerator + denominator / 2) / denominator
}

fn state_id(content: &StellarOrbitalContent) -> Result<String, StellarOrbitalError> {
    let bytes = serde_json::to_vec(content)
        .map_err(|error| StellarOrbitalError::Codec(error.to_string()))?;
    Ok(hex(&domain_hash(
        b"mindwarp.stellar-orbital.state.v1\0",
        &bytes,
    )))
}

fn valid_hex_id(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
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

    fn input() -> StellarOrbitalInput {
        StellarOrbitalInput {
            schema_version: CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            stellar_source_id: [2; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        }
    }

    #[test]
    fn earth_normalized_contract_is_deterministic_strict_and_replayable() {
        let contract = compile_stellar_orbital(&input()).unwrap();
        assert_eq!(contract, compile_stellar_orbital(&input()).unwrap());
        assert_eq!(contract.state.content.periapsis_milli_au, 1_000);
        assert_eq!(contract.state.content.apoapsis_milli_au, 1_000);
        assert_eq!(
            contract
                .state
                .content
                .irradiance_mean_distance_millionths_earth,
            1_000_000
        );
        assert_eq!(
            contract
                .state
                .content
                .bounded_stellar_irradiance_rgb_permille,
            [400, 350, 250]
        );
        assert_eq!(
            contract
                .state
                .content
                .orbital_period_squared_millionths_earth_year_squared,
            1_000_000
        );
        assert_eq!(
            StellarOrbitalContract::from_bytes(&contract.to_bytes().unwrap()).unwrap(),
            contract
        );
    }

    #[test]
    fn eccentric_orbit_has_ordered_distance_and_inverse_square_flux_bounds() {
        let mut value = input();
        value.eccentricity_millionths = 200_000;
        let state = compile_stellar_orbital(&value).unwrap().state.content;
        assert_eq!(state.periapsis_milli_au, 800);
        assert_eq!(state.apoapsis_milli_au, 1_200);
        assert!(state.irradiance_min_millionths_earth < 1_000_000);
        assert!(state.irradiance_max_millionths_earth > 1_000_000);
    }

    #[test]
    fn luminosity_changes_flux_without_fabricating_orbit_change() {
        let first = compile_stellar_orbital(&input()).unwrap();
        let mut brighter = input();
        brighter.stellar_luminosity_millionths_solar = 2_000_000;
        let second = compile_stellar_orbital(&brighter).unwrap();
        assert_eq!(
            first.state.content.periapsis_milli_au,
            second.state.content.periapsis_milli_au
        );
        assert_eq!(
            second
                .state
                .content
                .irradiance_mean_distance_millionths_earth,
            2 * first
                .state
                .content
                .irradiance_mean_distance_millionths_earth
        );
    }

    #[test]
    fn greater_orbital_distance_reduces_flux_and_increases_period_ratio() {
        let first = compile_stellar_orbital(&input()).unwrap();
        let mut farther = input();
        farther.semi_major_axis_milli_au = 2_000;
        let second = compile_stellar_orbital(&farther).unwrap();
        assert!(
            second
                .state
                .content
                .irradiance_mean_distance_millionths_earth
                < first
                    .state
                    .content
                    .irradiance_mean_distance_millionths_earth
        );
        assert!(
            second
                .state
                .content
                .orbital_period_squared_millionths_earth_year_squared
                > first
                    .state
                    .content
                    .orbital_period_squared_millionths_earth_year_squared
        );
    }

    #[test]
    fn ranges_unknown_fields_and_noncanonical_bytes_fail_closed() {
        let mut invalid = input();
        invalid.eccentricity_millionths = 1_000_000;
        assert!(compile_stellar_orbital(&invalid).is_err());

        invalid = input();
        invalid.stellar_spectrum_rgb_permille = [500, 500, 500];
        assert!(compile_stellar_orbital(&invalid).is_err());

        let bytes = input().to_bytes().unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["orbital_solver"] = serde_json::json!("universal_n_body");
        assert!(StellarOrbitalInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());

        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(StellarOrbitalInput::from_bytes(&spaced).is_err());
    }

    #[test]
    fn public_state_claim_and_identity_drift_fail_before_serialization() {
        let mut contract = compile_stellar_orbital(&input()).unwrap();
        contract.state.content.authority_effect = "promote".into();
        assert_eq!(
            contract.state.to_bytes(),
            Err(StellarOrbitalError::Invalid(
                "state claim or authority drift"
            ))
        );

        let mut contract = compile_stellar_orbital(&input()).unwrap();
        contract.state.state_id = "0".repeat(64);
        assert_eq!(
            contract.state.to_bytes(),
            Err(StellarOrbitalError::Invalid("state identity drift"))
        );
    }

    #[test]
    fn plausible_fabricated_state_is_rejected_by_exact_input_replay() {
        let mut contract = compile_stellar_orbital(&input()).unwrap();
        contract.state.content.irradiance_min_millionths_earth += 1;
        contract
            .state
            .content
            .irradiance_mean_distance_millionths_earth += 1;
        contract.state.content.irradiance_max_millionths_earth += 1;
        contract.state.state_id = state_id(&contract.state.content).unwrap();
        assert!(contract.state.to_bytes().is_ok());
        assert_eq!(
            validate_stellar_orbital(&contract),
            Err(StellarOrbitalError::Invalid("stellar/orbital state drift"))
        );
    }
}
