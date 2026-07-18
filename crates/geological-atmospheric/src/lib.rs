//! Strict, capability-free geological/atmospheric causal contract.
//!
//! This crate proves only a bounded reference seam: declared planetary bulk
//! properties and an atmospheric column deterministically produce surface
//! gravity, column pressure, and three-band direct transmission evidence. It
//! does not model composition, vertical structure, weather, climate,
//! hydrology, materials, tectonics, habitability, or runtime simulation.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub use stellar_orbital::{
    StellarOrbitalContract, StellarOrbitalInput, compile_stellar_orbital, validate_stellar_orbital,
};

pub const CONTRACT_VERSION: u16 = 1;
const EARTH_STANDARD_GRAVITY_MM_S2: u128 = 9_807;
const PERMILLE: u128 = 1_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeologicalAtmosphericInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub planetary_body_id: [u8; 32],
    pub stellar_orbital: StellarOrbitalContract,
    pub planet_mass_milli_earth: u32,
    pub planet_radius_milli_earth: u32,
    pub internal_heat_flux_milli_w_m2: u32,
    pub solid_surface_fraction_permille: u16,
    pub atmospheric_column_mass_g_m2: u64,
    pub gas_transmission_rgb_permille: [u16; 3],
    pub aerosol_transmission_rgb_permille: [u16; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeologicalAtmosphericContent {
    pub schema_version: u16,
    pub input_id: String,
    pub stellar_orbital_state_id: String,
    pub surface_gravity_mm_s2: u64,
    pub surface_pressure_pa: u64,
    pub internal_heat_flux_milli_w_m2: u32,
    pub solid_surface_fraction_permille: u16,
    pub atmosphere_transmission_rgb_permille: [u16; 3],
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeologicalAtmosphericState {
    pub state_id: String,
    pub content: GeologicalAtmosphericContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeologicalAtmosphericContract {
    pub input: GeologicalAtmosphericInput,
    pub state: GeologicalAtmosphericState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeologicalAtmosphericError {
    Invalid(&'static str),
    Codec(String),
    Stellar(stellar_orbital::StellarOrbitalError),
}

impl GeologicalAtmosphericInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GeologicalAtmosphericError> {
        validate_input(self)?;
        serde_json::to_vec(self)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GeologicalAtmosphericError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(GeologicalAtmosphericError::Invalid(
                "noncanonical input bytes",
            ));
        }
        Ok(value)
    }
}

impl GeologicalAtmosphericState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GeologicalAtmosphericError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(GeologicalAtmosphericError::Invalid("state identity drift"));
        }
        serde_json::to_vec(self)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GeologicalAtmosphericError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(GeologicalAtmosphericError::Invalid(
                "noncanonical or drifted state",
            ));
        }
        Ok(value)
    }
}

impl GeologicalAtmosphericContract {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GeologicalAtmosphericError> {
        validate_geological_atmospheric(self)?;
        serde_json::to_vec(self)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, GeologicalAtmosphericError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))?;
        validate_geological_atmospheric(&value)?;
        if value.to_bytes()? != bytes {
            return Err(GeologicalAtmosphericError::Invalid(
                "noncanonical contract bytes",
            ));
        }
        Ok(value)
    }
}

pub fn compile_geological_atmospheric(
    input: &GeologicalAtmosphericInput,
) -> Result<GeologicalAtmosphericContract, GeologicalAtmosphericError> {
    let input_bytes = input.to_bytes()?;
    let input_id = hex(&domain_hash(
        b"mindwarp.geological-atmospheric.input.v1\0",
        &input_bytes,
    ));
    let radius = u128::from(input.planet_radius_milli_earth);
    let gravity = rounded_div(
        EARTH_STANDARD_GRAVITY_MM_S2 * u128::from(input.planet_mass_milli_earth) * PERMILLE,
        radius * radius,
    );
    let surface_gravity_mm_s2 = u64::try_from(gravity)
        .map_err(|_| GeologicalAtmosphericError::Invalid("gravity overflow"))?;
    if surface_gravity_mm_s2 == 0 {
        return Err(GeologicalAtmosphericError::Invalid(
            "gravity below contract resolution",
        ));
    }
    let pressure = rounded_div(
        u128::from(input.atmospheric_column_mass_g_m2) * gravity,
        1_000_000,
    );
    let surface_pressure_pa = u64::try_from(pressure)
        .map_err(|_| GeologicalAtmosphericError::Invalid("pressure overflow"))?;

    let mut transmission = [0_u16; 3];
    for (index, output) in transmission.iter_mut().enumerate() {
        *output = u16::try_from(rounded_div(
            u128::from(input.gas_transmission_rgb_permille[index])
                * u128::from(input.aerosol_transmission_rgb_permille[index]),
            PERMILLE,
        ))
        .map_err(|_| GeologicalAtmosphericError::Invalid("transmission overflow"))?;
    }

    let content = GeologicalAtmosphericContent {
        schema_version: CONTRACT_VERSION,
        input_id,
        stellar_orbital_state_id: input.stellar_orbital.state.state_id.clone(),
        surface_gravity_mm_s2,
        surface_pressure_pa,
        internal_heat_flux_milli_w_m2: input.internal_heat_flux_milli_w_m2,
        solid_surface_fraction_permille: input.solid_surface_fraction_permille,
        atmosphere_transmission_rgb_permille: transmission,
        limitations: vec![
            "bounded spherical gravity column-pressure and direct-transmission reference; not scientific validation".into(),
            "no composition vertical structure weather climate hydrology material tectonic habitability biome or runtime claim".into(),
        ],
        authority_effect: "none_evidence_only".into(),
    };
    validate_content(&content)?;
    Ok(GeologicalAtmosphericContract {
        input: input.clone(),
        state: GeologicalAtmosphericState {
            state_id: state_id(&content)?,
            content,
        },
    })
}

pub fn validate_geological_atmospheric(
    contract: &GeologicalAtmosphericContract,
) -> Result<(), GeologicalAtmosphericError> {
    let expected = compile_geological_atmospheric(&contract.input)?;
    if expected.state != contract.state {
        return Err(GeologicalAtmosphericError::Invalid(
            "geological/atmospheric state drift",
        ));
    }
    Ok(())
}

fn validate_input(input: &GeologicalAtmosphericInput) -> Result<(), GeologicalAtmosphericError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(GeologicalAtmosphericError::Invalid(
            "unsupported contract version",
        ));
    }
    if input.reconstruction_id == [0; 32] || input.planetary_body_id == [0; 32] {
        return Err(GeologicalAtmosphericError::Invalid(
            "missing identity binding",
        ));
    }
    validate_stellar_orbital(&input.stellar_orbital)
        .map_err(GeologicalAtmosphericError::Stellar)?;
    if input.reconstruction_id != input.stellar_orbital.input.reconstruction_id {
        return Err(GeologicalAtmosphericError::Invalid(
            "stellar/orbital reconstruction mismatch",
        ));
    }
    if !(1..=1_000_000).contains(&input.planet_mass_milli_earth)
        || !(1..=1_000_000).contains(&input.planet_radius_milli_earth)
        || input.internal_heat_flux_milli_w_m2 > 1_000_000
        || input.solid_surface_fraction_permille > 1_000
        || input.atmospheric_column_mass_g_m2 > 1_000_000_000
    {
        return Err(GeologicalAtmosphericError::Invalid(
            "planetary or column range",
        ));
    }
    if input
        .gas_transmission_rgb_permille
        .iter()
        .chain(input.aerosol_transmission_rgb_permille.iter())
        .any(|value| *value > 1_000)
    {
        return Err(GeologicalAtmosphericError::Invalid("transmission range"));
    }
    if input.atmospheric_column_mass_g_m2 == 0
        && (input.gas_transmission_rgb_permille != [1_000; 3]
            || input.aerosol_transmission_rgb_permille != [1_000; 3])
    {
        return Err(GeologicalAtmosphericError::Invalid(
            "attenuation without atmospheric column",
        ));
    }
    Ok(())
}

fn validate_content(
    content: &GeologicalAtmosphericContent,
) -> Result<(), GeologicalAtmosphericError> {
    if content.schema_version != CONTRACT_VERSION {
        return Err(GeologicalAtmosphericError::Invalid(
            "unsupported state schema",
        ));
    }
    if !valid_hex_id(&content.input_id) || !valid_hex_id(&content.stellar_orbital_state_id) {
        return Err(GeologicalAtmosphericError::Invalid(
            "malformed state identity",
        ));
    }
    if content.surface_gravity_mm_s2 == 0
        || content.solid_surface_fraction_permille > 1_000
        || content
            .atmosphere_transmission_rgb_permille
            .iter()
            .any(|value| *value > 1_000)
    {
        return Err(GeologicalAtmosphericError::Invalid(
            "state physical invariant",
        ));
    }
    if content.limitations
        != [
            "bounded spherical gravity column-pressure and direct-transmission reference; not scientific validation",
            "no composition vertical structure weather climate hydrology material tectonic habitability biome or runtime claim",
        ]
        .map(String::from)
        .to_vec()
        || content.authority_effect != "none_evidence_only"
    {
        return Err(GeologicalAtmosphericError::Invalid(
            "state claim or authority drift",
        ));
    }
    Ok(())
}

fn rounded_div(numerator: u128, denominator: u128) -> u128 {
    (numerator + denominator / 2) / denominator
}

fn state_id(content: &GeologicalAtmosphericContent) -> Result<String, GeologicalAtmosphericError> {
    let bytes = serde_json::to_vec(content)
        .map_err(|error| GeologicalAtmosphericError::Codec(error.to_string()))?;
    Ok(hex(&domain_hash(
        b"mindwarp.geological-atmospheric.state.v1\0",
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

    fn stellar_contract(reconstruction_id: [u8; 32]) -> StellarOrbitalContract {
        compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: stellar_orbital::CONTRACT_VERSION,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn input() -> GeologicalAtmosphericInput {
        GeologicalAtmosphericInput {
            schema_version: CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            planetary_body_id: [2; 32],
            stellar_orbital: stellar_contract([1; 32]),
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000, 1_000, 1_000],
        }
    }

    #[test]
    fn earth_normalized_contract_is_deterministic_strict_and_replayable() {
        let contract = compile_geological_atmospheric(&input()).unwrap();
        assert_eq!(contract, compile_geological_atmospheric(&input()).unwrap());
        assert_eq!(contract.state.content.surface_gravity_mm_s2, 9_807);
        assert_eq!(contract.state.content.surface_pressure_pa, 101_326);
        assert_eq!(
            contract.state.content.atmosphere_transmission_rgb_permille,
            [800, 900, 950]
        );
        assert_eq!(
            GeologicalAtmosphericContract::from_bytes(&contract.to_bytes().unwrap()).unwrap(),
            contract
        );
    }

    #[test]
    fn mass_and_radius_change_gravity_and_column_pressure_in_causal_order() {
        let base = compile_geological_atmospheric(&input()).unwrap();
        let mut heavier = input();
        heavier.planet_mass_milli_earth = 2_000;
        let heavier = compile_geological_atmospheric(&heavier).unwrap();
        assert_eq!(
            heavier.state.content.surface_gravity_mm_s2,
            2 * base.state.content.surface_gravity_mm_s2
        );
        assert_eq!(
            heavier.state.content.surface_pressure_pa,
            2 * base.state.content.surface_pressure_pa
        );

        let mut larger = input();
        larger.planet_radius_milli_earth = 2_000;
        let larger = compile_geological_atmospheric(&larger).unwrap();
        assert!(
            larger.state.content.surface_gravity_mm_s2 < base.state.content.surface_gravity_mm_s2
        );
        assert!(larger.state.content.surface_pressure_pa < base.state.content.surface_pressure_pa);
    }

    #[test]
    fn gas_and_aerosol_transmission_compose_per_band() {
        let mut value = input();
        value.gas_transmission_rgb_permille = [500, 800, 1_000];
        value.aerosol_transmission_rgb_permille = [400, 750, 900];
        assert_eq!(
            compile_geological_atmospheric(&value)
                .unwrap()
                .state
                .content
                .atmosphere_transmission_rgb_permille,
            [200, 600, 900]
        );
    }

    #[test]
    fn column_mass_changes_pressure_without_fabricating_gravity() {
        let first = compile_geological_atmospheric(&input()).unwrap();
        let mut value = input();
        value.atmospheric_column_mass_g_m2 *= 2;
        let second = compile_geological_atmospheric(&value).unwrap();
        assert_eq!(
            first.state.content.surface_gravity_mm_s2,
            second.state.content.surface_gravity_mm_s2
        );
        assert_eq!(
            second.state.content.surface_pressure_pa,
            2 * first.state.content.surface_pressure_pa
        );
    }

    #[test]
    fn foreign_or_fabricated_stellar_state_fails_before_compile() {
        let mut foreign = input();
        foreign.stellar_orbital = stellar_contract([9; 32]);
        assert_eq!(
            compile_geological_atmospheric(&foreign),
            Err(GeologicalAtmosphericError::Invalid(
                "stellar/orbital reconstruction mismatch"
            ))
        );

        let mut fabricated = input();
        fabricated
            .stellar_orbital
            .state
            .content
            .irradiance_mean_distance_millionths_earth += 1;
        assert!(matches!(
            compile_geological_atmospheric(&fabricated),
            Err(GeologicalAtmosphericError::Stellar(_))
        ));
    }

    #[test]
    fn ranges_unknown_fields_and_noncanonical_bytes_fail() {
        let mut invalid = input();
        invalid.solid_surface_fraction_permille = 1_001;
        assert!(compile_geological_atmospheric(&invalid).is_err());

        invalid = input();
        invalid.gas_transmission_rgb_permille[0] = 1_001;
        assert!(compile_geological_atmospheric(&invalid).is_err());

        invalid = input();
        invalid.atmospheric_column_mass_g_m2 = 0;
        assert_eq!(
            compile_geological_atmospheric(&invalid),
            Err(GeologicalAtmosphericError::Invalid(
                "attenuation without atmospheric column"
            ))
        );
        invalid.gas_transmission_rgb_permille = [1_000; 3];
        assert!(compile_geological_atmospheric(&invalid).is_ok());

        let bytes = input().to_bytes().unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["climate_solver"] = serde_json::json!("implicit");
        assert!(
            GeologicalAtmosphericInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
        );

        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(GeologicalAtmosphericInput::from_bytes(&spaced).is_err());
    }

    #[test]
    fn plausible_fabricated_public_state_and_claim_drift_fail() {
        let mut contract = compile_geological_atmospheric(&input()).unwrap();
        contract.state.content.surface_pressure_pa += 1;
        contract.state.state_id = state_id(&contract.state.content).unwrap();
        assert_eq!(
            validate_geological_atmospheric(&contract),
            Err(GeologicalAtmosphericError::Invalid(
                "geological/atmospheric state drift"
            ))
        );

        let mut state = compile_geological_atmospheric(&input()).unwrap().state;
        state.content.limitations.clear();
        assert_eq!(
            state.to_bytes(),
            Err(GeologicalAtmosphericError::Invalid(
                "state claim or authority drift"
            ))
        );
    }
}
