//! Strict, capability-free hydrological inventory contract.
//!
//! This crate proves only a bounded inventory seam: a declared total water
//! column, declared solid/liquid/vapor partition, and declared surface access
//! deterministically produce exact scaled reservoir evidence. It does not
//! infer phase equilibrium, temperature, precipitation, flow, climate,
//! terrain, salinity, habitability, or runtime simulation.

use geological_atmospheric::{GeologicalAtmosphericContract, validate_geological_atmospheric};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
const PERMILLE: u64 = 1_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologicalInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub hydrological_source_id: [u8; 32],
    pub geological_atmospheric: GeologicalAtmosphericContract,
    pub total_water_column_g_m2: u64,
    pub phase_partition_permille: [u16; 3],
    pub surface_accessible_liquid_fraction_permille: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologicalContent {
    pub schema_version: u16,
    pub input_id: String,
    pub geological_atmospheric_state_id: String,
    pub total_water_column_g_m2: u64,
    pub phase_column_thousandths_g_m2: [u64; 3],
    pub surface_accessible_liquid_column_millionths_g_m2: u64,
    pub has_surface_accessible_liquid: bool,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologicalState {
    pub state_id: String,
    pub content: HydrologicalContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologicalContract {
    pub input: HydrologicalInput,
    pub state: HydrologicalState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HydrologicalError {
    Invalid(&'static str),
    Codec(String),
    Geological(geological_atmospheric::GeologicalAtmosphericError),
}

impl HydrologicalInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, HydrologicalError> {
        validate_input(self)?;
        serde_json::to_vec(self).map_err(|error| HydrologicalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HydrologicalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| HydrologicalError::Codec(error.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(HydrologicalError::Invalid("noncanonical input bytes"));
        }
        Ok(value)
    }
}

impl HydrologicalState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, HydrologicalError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(HydrologicalError::Invalid("state identity drift"));
        }
        serde_json::to_vec(self).map_err(|error| HydrologicalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HydrologicalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| HydrologicalError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(HydrologicalError::Invalid("noncanonical or drifted state"));
        }
        Ok(value)
    }
}

impl HydrologicalContract {
    pub fn to_bytes(&self) -> Result<Vec<u8>, HydrologicalError> {
        validate_hydrological(self)?;
        serde_json::to_vec(self).map_err(|error| HydrologicalError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HydrologicalError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| HydrologicalError::Codec(error.to_string()))?;
        validate_hydrological(&value)?;
        if value.to_bytes()? != bytes {
            return Err(HydrologicalError::Invalid("noncanonical contract bytes"));
        }
        Ok(value)
    }
}

pub fn compile_hydrological(
    input: &HydrologicalInput,
) -> Result<HydrologicalContract, HydrologicalError> {
    let input_bytes = input.to_bytes()?;
    let input_id = hex(&domain_hash(
        b"mindwarp.hydrological-state.input.v1\0",
        &input_bytes,
    ));
    let mut phase_columns = [0_u64; 3];
    for (index, output) in phase_columns.iter_mut().enumerate() {
        *output = input
            .total_water_column_g_m2
            .checked_mul(u64::from(input.phase_partition_permille[index]))
            .ok_or(HydrologicalError::Invalid("phase column overflow"))?;
    }
    let surface_accessible_liquid = phase_columns[1]
        .checked_mul(u64::from(input.surface_accessible_liquid_fraction_permille))
        .ok_or(HydrologicalError::Invalid("surface liquid column overflow"))?;

    let content = HydrologicalContent {
        schema_version: CONTRACT_VERSION,
        input_id,
        geological_atmospheric_state_id: input.geological_atmospheric.state.state_id.clone(),
        total_water_column_g_m2: input.total_water_column_g_m2,
        phase_column_thousandths_g_m2: phase_columns,
        surface_accessible_liquid_column_millionths_g_m2: surface_accessible_liquid,
        has_surface_accessible_liquid: surface_accessible_liquid > 0,
        limitations: vec![
            "declared water inventory and reservoir partition reference; not phase equilibrium or scientific validation".into(),
            "no temperature precipitation transport terrain flow salinity climate material habitability biome or runtime claim".into(),
        ],
        authority_effect: "none_evidence_only".into(),
    };
    validate_content(&content)?;
    Ok(HydrologicalContract {
        input: input.clone(),
        state: HydrologicalState {
            state_id: state_id(&content)?,
            content,
        },
    })
}

pub fn validate_hydrological(contract: &HydrologicalContract) -> Result<(), HydrologicalError> {
    let expected = compile_hydrological(&contract.input)?;
    if expected.state != contract.state {
        return Err(HydrologicalError::Invalid("hydrological state drift"));
    }
    Ok(())
}

fn validate_input(input: &HydrologicalInput) -> Result<(), HydrologicalError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(HydrologicalError::Invalid("unsupported contract version"));
    }
    if input.reconstruction_id == [0; 32] || input.hydrological_source_id == [0; 32] {
        return Err(HydrologicalError::Invalid("missing identity binding"));
    }
    validate_geological_atmospheric(&input.geological_atmospheric)
        .map_err(HydrologicalError::Geological)?;
    if input.reconstruction_id != input.geological_atmospheric.input.reconstruction_id {
        return Err(HydrologicalError::Invalid(
            "geological/atmospheric reconstruction mismatch",
        ));
    }
    if input.total_water_column_g_m2 > 1_000_000_000_000
        || input.surface_accessible_liquid_fraction_permille > 1_000
        || input
            .phase_partition_permille
            .iter()
            .any(|value| *value > 1_000)
    {
        return Err(HydrologicalError::Invalid("inventory or partition range"));
    }
    let phase_sum = input
        .phase_partition_permille
        .iter()
        .map(|value| u32::from(*value))
        .sum::<u32>();
    if (input.total_water_column_g_m2 == 0
        && (phase_sum != 0 || input.surface_accessible_liquid_fraction_permille != 0))
        || (input.total_water_column_g_m2 > 0 && phase_sum != 1_000)
    {
        return Err(HydrologicalError::Invalid(
            "inventory and phase partition contradiction",
        ));
    }
    if input.phase_partition_permille[1] == 0
        && input.surface_accessible_liquid_fraction_permille != 0
    {
        return Err(HydrologicalError::Invalid(
            "surface access without liquid reservoir",
        ));
    }
    Ok(())
}

fn validate_content(content: &HydrologicalContent) -> Result<(), HydrologicalError> {
    if content.schema_version != CONTRACT_VERSION {
        return Err(HydrologicalError::Invalid("unsupported state schema"));
    }
    if !valid_hex_id(&content.input_id) || !valid_hex_id(&content.geological_atmospheric_state_id) {
        return Err(HydrologicalError::Invalid("malformed state identity"));
    }
    let expected_phase_sum = content
        .total_water_column_g_m2
        .checked_mul(PERMILLE)
        .ok_or(HydrologicalError::Invalid("state inventory overflow"))?;
    if content
        .phase_column_thousandths_g_m2
        .iter()
        .try_fold(0_u64, |sum, value| sum.checked_add(*value))
        != Some(expected_phase_sum)
        || content.has_surface_accessible_liquid
            != (content.surface_accessible_liquid_column_millionths_g_m2 > 0)
    {
        return Err(HydrologicalError::Invalid("state reservoir invariant"));
    }
    if content.limitations
        != [
            "declared water inventory and reservoir partition reference; not phase equilibrium or scientific validation",
            "no temperature precipitation transport terrain flow salinity climate material habitability biome or runtime claim",
        ]
        .map(String::from)
        .to_vec()
        || content.authority_effect != "none_evidence_only"
    {
        return Err(HydrologicalError::Invalid(
            "state claim or authority drift",
        ));
    }
    Ok(())
}

fn state_id(content: &HydrologicalContent) -> Result<String, HydrologicalError> {
    let bytes =
        serde_json::to_vec(content).map_err(|error| HydrologicalError::Codec(error.to_string()))?;
    Ok(hex(&domain_hash(
        b"mindwarp.hydrological-state.state.v1\0",
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
    use geological_atmospheric::{
        GeologicalAtmosphericInput, StellarOrbitalInput, compile_geological_atmospheric,
        compile_stellar_orbital,
    };

    fn geological_contract(reconstruction_id: [u8; 32]) -> GeologicalAtmosphericContract {
        let stellar = compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap();
        compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: 1,
            reconstruction_id,
            planetary_body_id: [4; 32],
            stellar_orbital: stellar,
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000; 3],
        })
        .unwrap()
    }

    fn input() -> HydrologicalInput {
        HydrologicalInput {
            schema_version: CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract([1; 32]),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        }
    }

    #[test]
    fn declared_inventory_is_deterministic_strict_and_replayable() {
        let contract = compile_hydrological(&input()).unwrap();
        assert_eq!(contract, compile_hydrological(&input()).unwrap());
        assert_eq!(
            contract.state.content.phase_column_thousandths_g_m2,
            [200_000_000, 1_700_000_000, 100_000_000]
        );
        assert_eq!(
            contract
                .state
                .content
                .surface_accessible_liquid_column_millionths_g_m2,
            1_190_000_000_000
        );
        assert!(contract.state.content.has_surface_accessible_liquid);
        assert_eq!(
            HydrologicalContract::from_bytes(&contract.to_bytes().unwrap()).unwrap(),
            contract
        );
    }

    #[test]
    fn partition_and_surface_access_change_only_their_exact_evidence() {
        let base = compile_hydrological(&input()).unwrap();
        let mut frozen = input();
        frozen.phase_partition_permille = [900, 100, 0];
        let frozen = compile_hydrological(&frozen).unwrap();
        assert_eq!(
            base.state.content.total_water_column_g_m2,
            frozen.state.content.total_water_column_g_m2
        );
        assert!(
            frozen.state.content.phase_column_thousandths_g_m2[0]
                > base.state.content.phase_column_thousandths_g_m2[0]
        );

        let mut inaccessible = input();
        inaccessible.surface_accessible_liquid_fraction_permille = 0;
        let inaccessible = compile_hydrological(&inaccessible).unwrap();
        assert!(!inaccessible.state.content.has_surface_accessible_liquid);
    }

    #[test]
    fn dry_inventory_requires_zero_partition_and_zero_surface_access() {
        let mut dry = input();
        dry.total_water_column_g_m2 = 0;
        assert!(compile_hydrological(&dry).is_err());
        dry.phase_partition_permille = [0; 3];
        dry.surface_accessible_liquid_fraction_permille = 0;
        let state = compile_hydrological(&dry).unwrap().state.content;
        assert_eq!(state.phase_column_thousandths_g_m2, [0; 3]);
        assert!(!state.has_surface_accessible_liquid);
    }

    #[test]
    fn surface_access_without_liquid_reservoir_fails_closed() {
        let mut value = input();
        value.phase_partition_permille = [900, 0, 100];
        assert_eq!(
            compile_hydrological(&value),
            Err(HydrologicalError::Invalid(
                "surface access without liquid reservoir"
            ))
        );
    }

    #[test]
    fn foreign_or_fabricated_planet_state_fails_before_compile() {
        let mut foreign = input();
        foreign.geological_atmospheric = geological_contract([9; 32]);
        assert_eq!(
            compile_hydrological(&foreign),
            Err(HydrologicalError::Invalid(
                "geological/atmospheric reconstruction mismatch"
            ))
        );

        let mut fabricated = input();
        fabricated
            .geological_atmospheric
            .state
            .content
            .surface_pressure_pa += 1;
        assert!(matches!(
            compile_hydrological(&fabricated),
            Err(HydrologicalError::Geological(_))
        ));
    }

    #[test]
    fn ranges_unknown_fields_and_noncanonical_bytes_fail() {
        let mut invalid = input();
        invalid.phase_partition_permille = [1_001, 0, 0];
        assert!(compile_hydrological(&invalid).is_err());
        invalid = input();
        invalid.phase_partition_permille = [100, 800, 50];
        assert!(compile_hydrological(&invalid).is_err());

        let bytes = input().to_bytes().unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["temperature_solver"] = serde_json::json!("implicit");
        assert!(HydrologicalInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());

        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(HydrologicalInput::from_bytes(&spaced).is_err());
    }

    #[test]
    fn plausible_fabricated_state_and_claim_drift_fail() {
        let mut contract = compile_hydrological(&input()).unwrap();
        contract
            .state
            .content
            .surface_accessible_liquid_column_millionths_g_m2 += 1;
        contract.state.state_id = state_id(&contract.state.content).unwrap();
        assert_eq!(
            validate_hydrological(&contract),
            Err(HydrologicalError::Invalid("hydrological state drift"))
        );

        let mut state = compile_hydrological(&input()).unwrap().state;
        state.content.limitations.clear();
        assert_eq!(
            state.to_bytes(),
            Err(HydrologicalError::Invalid("state claim or authority drift"))
        );
    }
}
