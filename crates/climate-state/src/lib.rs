//! Strict, capability-free top-of-atmosphere radiation-budget seam.
//! It does not derive temperature, weather, circulation, phase equilibrium,
//! habitability, scientific validity, or runtime behavior.

use hydrological_state::{HydrologicalContract, validate_hydrological};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
const PERMILLE: u64 = 1_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClimateInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub climate_source_id: [u8; 32],
    pub hydrological: HydrologicalContract,
    pub bond_albedo_permille: u16,
    pub outgoing_longwave_fraction_of_incident_permille: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClimateContent {
    pub schema_version: u16,
    pub input_id: String,
    pub hydrological_state_id: String,
    pub incident_shortwave_quarter_millionths_earth: u64,
    pub absorbed_shortwave_quarter_billionths_earth: u64,
    pub outgoing_longwave_quarter_billionths_earth: u64,
    pub net_radiation_quarter_billionths_earth: i64,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClimateState {
    pub state_id: String,
    pub content: ClimateContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClimateContract {
    pub input: ClimateInput,
    pub state: ClimateState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClimateError {
    Invalid(&'static str),
    Codec(String),
    Hydrological(hydrological_state::HydrologicalError),
}

impl ClimateInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ClimateError> {
        validate_input(self)?;
        encode(self)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ClimateError> {
        let value: Self = decode(bytes)?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(ClimateError::Invalid("noncanonical input bytes"));
        }
        Ok(value)
    }
}
impl ClimateState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ClimateError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(ClimateError::Invalid("state identity drift"));
        }
        encode(self)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ClimateError> {
        let value: Self = decode(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(ClimateError::Invalid("noncanonical or drifted state"));
        }
        Ok(value)
    }
}
impl ClimateContract {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ClimateError> {
        validate_climate(self)?;
        encode(self)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ClimateError> {
        let value: Self = decode(bytes)?;
        validate_climate(&value)?;
        if value.to_bytes()? != bytes {
            return Err(ClimateError::Invalid("noncanonical contract bytes"));
        }
        Ok(value)
    }
}

pub fn compile_climate(input: &ClimateInput) -> Result<ClimateContract, ClimateError> {
    let input_bytes = input.to_bytes()?;
    let incident = input
        .hydrological
        .input
        .geological_atmospheric
        .input
        .stellar_orbital
        .state
        .content
        .irradiance_mean_distance_millionths_earth;
    let absorbed = incident
        .checked_mul(PERMILLE - u64::from(input.bond_albedo_permille))
        .ok_or(ClimateError::Invalid("absorbed shortwave overflow"))?;
    let outgoing = incident
        .checked_mul(u64::from(
            input.outgoing_longwave_fraction_of_incident_permille,
        ))
        .ok_or(ClimateError::Invalid("outgoing longwave overflow"))?;
    let net = i64::try_from(i128::from(absorbed) - i128::from(outgoing))
        .map_err(|_| ClimateError::Invalid("net radiation overflow"))?;
    let content = ClimateContent {
        schema_version: CONTRACT_VERSION,
        input_id: hex(&hash(b"mindwarp.climate-state.input.v1\0", &input_bytes)),
        hydrological_state_id: input.hydrological.state.state_id.clone(),
        incident_shortwave_quarter_millionths_earth: incident,
        absorbed_shortwave_quarter_billionths_earth: absorbed,
        outgoing_longwave_quarter_billionths_earth: outgoing,
        net_radiation_quarter_billionths_earth: net,
        limitations: limitations(),
        authority_effect: "none_evidence_only".into(),
    };
    validate_content(&content)?;
    Ok(ClimateContract {
        input: input.clone(),
        state: ClimateState {
            state_id: state_id(&content)?,
            content,
        },
    })
}

pub fn validate_climate(contract: &ClimateContract) -> Result<(), ClimateError> {
    if compile_climate(&contract.input)?.state != contract.state {
        return Err(ClimateError::Invalid("climate state drift"));
    }
    Ok(())
}
fn validate_input(input: &ClimateInput) -> Result<(), ClimateError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(ClimateError::Invalid("unsupported contract version"));
    }
    if input.reconstruction_id == [0; 32] || input.climate_source_id == [0; 32] {
        return Err(ClimateError::Invalid("missing identity binding"));
    }
    validate_hydrological(&input.hydrological).map_err(ClimateError::Hydrological)?;
    if input.reconstruction_id != input.hydrological.input.reconstruction_id {
        return Err(ClimateError::Invalid(
            "hydrological reconstruction mismatch",
        ));
    }
    if input.bond_albedo_permille > 1_000
        || input.outgoing_longwave_fraction_of_incident_permille > 1_000
    {
        return Err(ClimateError::Invalid("radiation budget range"));
    }
    Ok(())
}
fn validate_content(c: &ClimateContent) -> Result<(), ClimateError> {
    if c.schema_version != CONTRACT_VERSION {
        return Err(ClimateError::Invalid("unsupported state schema"));
    }
    if !valid_id(&c.input_id) || !valid_id(&c.hydrological_state_id) {
        return Err(ClimateError::Invalid("malformed state identity"));
    }
    let net = i128::from(c.absorbed_shortwave_quarter_billionths_earth)
        - i128::from(c.outgoing_longwave_quarter_billionths_earth);
    if i128::from(c.net_radiation_quarter_billionths_earth) != net {
        return Err(ClimateError::Invalid("state radiation invariant"));
    }
    if c.limitations != limitations() || c.authority_effect != "none_evidence_only" {
        return Err(ClimateError::Invalid("state claim or authority drift"));
    }
    Ok(())
}
fn limitations() -> Vec<String> {
    [
    "bounded top-of-atmosphere radiation-budget reference; not climate equilibrium or scientific validation",
    "no temperature greenhouse weather circulation cloud phase material habitability biome or runtime claim",
].map(String::from).to_vec()
}
fn state_id(c: &ClimateContent) -> Result<String, ClimateError> {
    Ok(hex(&hash(
        b"mindwarp.climate-state.state.v1\0",
        &encode(c)?,
    )))
}
fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>, ClimateError> {
    serde_json::to_vec(v).map_err(|e| ClimateError::Codec(e.to_string()))
}
fn decode<'a, T: Deserialize<'a>>(b: &'a [u8]) -> Result<T, ClimateError> {
    serde_json::from_slice(b).map_err(|e| ClimateError::Codec(e.to_string()))
}
fn valid_id(v: &str) -> bool {
    v.len() == 64
        && v.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain);
    h.update(bytes);
    h.finalize().into()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use geological_atmospheric::{GeologicalAtmosphericInput, compile_geological_atmospheric};
    use hydrological_state::{HydrologicalInput, compile_hydrological};
    use stellar_orbital::{StellarOrbitalInput, compile_stellar_orbital};
    fn input() -> ClimateInput {
        let r = [1; 32];
        let s = compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id: r,
            stellar_source_id: [2; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap();
        let g = compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: 1,
            reconstruction_id: r,
            planetary_body_id: [3; 32],
            stellar_orbital: s,
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000; 3],
        })
        .unwrap();
        let h = compile_hydrological(&HydrologicalInput {
            schema_version: 1,
            reconstruction_id: r,
            hydrological_source_id: [4; 32],
            geological_atmospheric: g,
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap();
        ClimateInput {
            schema_version: 1,
            reconstruction_id: r,
            climate_source_id: [5; 32],
            hydrological: h,
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        }
    }
    #[test]
    fn balanced_reference_is_strict_and_replayable() {
        let c = compile_climate(&input()).unwrap();
        assert_eq!(
            c.state.content.absorbed_shortwave_quarter_billionths_earth,
            700_000_000
        );
        assert_eq!(c.state.content.net_radiation_quarter_billionths_earth, 0);
        assert_eq!(
            ClimateContract::from_bytes(&c.to_bytes().unwrap()).unwrap(),
            c
        )
    }
    #[test]
    fn drivers_have_separate_exact_causality() {
        let b = compile_climate(&input()).unwrap();
        let mut v = input();
        v.bond_albedo_permille = 100;
        assert!(
            compile_climate(&v)
                .unwrap()
                .state
                .content
                .absorbed_shortwave_quarter_billionths_earth
                > b.state.content.absorbed_shortwave_quarter_billionths_earth
        );
        let mut v = input();
        v.outgoing_longwave_fraction_of_incident_permille += 1;
        let c = compile_climate(&v).unwrap();
        assert_eq!(
            c.state.content.absorbed_shortwave_quarter_billionths_earth,
            b.state.content.absorbed_shortwave_quarter_billionths_earth
        );
        assert_eq!(
            c.state.content.net_radiation_quarter_billionths_earth,
            -1_000_000
        )
    }
    #[test]
    fn foreign_and_fabricated_hydrology_fail() {
        let mut v = input();
        v.reconstruction_id = [9; 32];
        assert!(compile_climate(&v).is_err());
        let mut v = input();
        v.hydrological.state.content.total_water_column_g_m2 += 1;
        assert!(matches!(
            compile_climate(&v),
            Err(ClimateError::Hydrological(_))
        ))
    }
    #[test]
    fn ranges_unknown_and_noncanonical_fail() {
        let mut v = input();
        v.bond_albedo_permille = 1001;
        assert!(compile_climate(&v).is_err());
        let mut v = input();
        v.outgoing_longwave_fraction_of_incident_permille = 1_001;
        assert!(compile_climate(&v).is_err());
        let bytes = input().to_bytes().unwrap();
        let mut j: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        j["temperature_k"] = serde_json::json!(288);
        assert!(ClimateInput::from_bytes(&serde_json::to_vec(&j).unwrap()).is_err());
        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(ClimateInput::from_bytes(&spaced).is_err())
    }
    #[test]
    fn fabricated_state_and_claim_drift_fail() {
        let mut c = compile_climate(&input()).unwrap();
        c.state.content.net_radiation_quarter_billionths_earth = 1;
        c.state.state_id = state_id(&c.state.content).unwrap();
        assert_eq!(
            validate_climate(&c),
            Err(ClimateError::Invalid("climate state drift"))
        );
        let mut s = compile_climate(&input()).unwrap().state;
        s.content.limitations.clear();
        assert!(s.to_bytes().is_err())
    }

    #[test]
    fn stellar_irradiance_scales_both_fluxes_without_changing_fractions() {
        let base = compile_climate(&input()).unwrap();
        let mut brighter = input();
        brighter
            .hydrological
            .input
            .geological_atmospheric
            .input
            .stellar_orbital
            .input
            .stellar_luminosity_millionths_solar = 2_000_000;
        let stellar_input = brighter
            .hydrological
            .input
            .geological_atmospheric
            .input
            .stellar_orbital
            .input
            .clone();
        brighter
            .hydrological
            .input
            .geological_atmospheric
            .input
            .stellar_orbital = compile_stellar_orbital(&stellar_input).unwrap();
        let geological_input = brighter
            .hydrological
            .input
            .geological_atmospheric
            .input
            .clone();
        brighter.hydrological.input.geological_atmospheric =
            compile_geological_atmospheric(&geological_input).unwrap();
        let hydrological_input = brighter.hydrological.input.clone();
        brighter.hydrological = compile_hydrological(&hydrological_input).unwrap();
        let brighter = compile_climate(&brighter).unwrap();
        assert!(
            brighter
                .state
                .content
                .absorbed_shortwave_quarter_billionths_earth
                > base
                    .state
                    .content
                    .absorbed_shortwave_quarter_billionths_earth
        );
        assert!(
            brighter
                .state
                .content
                .outgoing_longwave_quarter_billionths_earth
                > base
                    .state
                    .content
                    .outgoing_longwave_quarter_billionths_earth
        );
    }

    #[test]
    fn hydrology_changes_provenance_without_fabricating_radiation_change() {
        let base = compile_climate(&input()).unwrap();
        let mut wetter = input();
        wetter.hydrological.input.total_water_column_g_m2 += 1;
        wetter.hydrological = compile_hydrological(&wetter.hydrological.input).unwrap();
        let wetter = compile_climate(&wetter).unwrap();
        assert_ne!(wetter.state.state_id, base.state.state_id);
        assert_eq!(
            wetter
                .state
                .content
                .incident_shortwave_quarter_millionths_earth,
            base.state
                .content
                .incident_shortwave_quarter_millionths_earth
        );
        assert_eq!(
            wetter
                .state
                .content
                .absorbed_shortwave_quarter_billionths_earth,
            base.state
                .content
                .absorbed_shortwave_quarter_billionths_earth
        );
    }
}
