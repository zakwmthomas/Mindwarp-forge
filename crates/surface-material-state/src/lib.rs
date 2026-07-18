//! Strict capability-free surface-reflectance provenance seam.
//! This is one coarse optical descriptor, not composition, chemistry, BRDF,
//! weathering, mechanics, crafting, visibility, or material simulation.

use climate_state::{ClimateContract, validate_climate};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceMaterialInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub material_source_id: [u8; 32],
    pub climate: ClimateContract,
    pub dominant_surface_reflectance_rgb_permille: [u16; 3],
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceMaterialContent {
    pub schema_version: u16,
    pub input_id: String,
    pub climate_state_id: String,
    pub dominant_surface_reflectance_rgb_permille: [u16; 3],
    pub limitations: Vec<String>,
    pub authority_effect: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceMaterialState {
    pub state_id: String,
    pub content: SurfaceMaterialContent,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceMaterialContract {
    pub input: SurfaceMaterialInput,
    pub state: SurfaceMaterialState,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceMaterialError {
    Invalid(&'static str),
    Codec(String),
    Climate(climate_state::ClimateError),
}

impl SurfaceMaterialInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SurfaceMaterialError> {
        validate_input(self)?;
        encode(self)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self, SurfaceMaterialError> {
        let v: Self = decode(b)?;
        validate_input(&v)?;
        if v.to_bytes()? != b {
            return Err(SurfaceMaterialError::Invalid("noncanonical input bytes"));
        }
        Ok(v)
    }
}
impl SurfaceMaterialState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SurfaceMaterialError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(SurfaceMaterialError::Invalid("state identity drift"));
        }
        encode(self)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self, SurfaceMaterialError> {
        let v: Self = decode(b)?;
        if v.to_bytes()? != b {
            return Err(SurfaceMaterialError::Invalid(
                "noncanonical or drifted state",
            ));
        }
        Ok(v)
    }
}
impl SurfaceMaterialContract {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SurfaceMaterialError> {
        validate_surface_material(self)?;
        encode(self)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self, SurfaceMaterialError> {
        let v: Self = decode(b)?;
        validate_surface_material(&v)?;
        if v.to_bytes()? != b {
            return Err(SurfaceMaterialError::Invalid("noncanonical contract bytes"));
        }
        Ok(v)
    }
}
pub fn compile_surface_material(
    i: &SurfaceMaterialInput,
) -> Result<SurfaceMaterialContract, SurfaceMaterialError> {
    let bytes = i.to_bytes()?;
    let content = SurfaceMaterialContent {
        schema_version: CONTRACT_VERSION,
        input_id: hex(&hash(b"mindwarp.surface-material.input.v1\0", &bytes)),
        climate_state_id: i.climate.state.state_id.clone(),
        dominant_surface_reflectance_rgb_permille: i.dominant_surface_reflectance_rgb_permille,
        limitations: limitations(),
        authority_effect: "none_evidence_only".into(),
    };
    validate_content(&content)?;
    Ok(SurfaceMaterialContract {
        input: i.clone(),
        state: SurfaceMaterialState {
            state_id: state_id(&content)?,
            content,
        },
    })
}
pub fn validate_surface_material(c: &SurfaceMaterialContract) -> Result<(), SurfaceMaterialError> {
    if compile_surface_material(&c.input)?.state != c.state {
        return Err(SurfaceMaterialError::Invalid(
            "surface material state drift",
        ));
    }
    Ok(())
}
fn validate_input(i: &SurfaceMaterialInput) -> Result<(), SurfaceMaterialError> {
    if i.schema_version != CONTRACT_VERSION {
        return Err(SurfaceMaterialError::Invalid(
            "unsupported contract version",
        ));
    }
    if i.reconstruction_id == [0; 32] || i.material_source_id == [0; 32] {
        return Err(SurfaceMaterialError::Invalid("missing identity binding"));
    }
    validate_climate(&i.climate).map_err(SurfaceMaterialError::Climate)?;
    if i.reconstruction_id != i.climate.input.reconstruction_id {
        return Err(SurfaceMaterialError::Invalid(
            "climate reconstruction mismatch",
        ));
    }
    if i.dominant_surface_reflectance_rgb_permille
        .iter()
        .any(|v| *v > 1_000)
    {
        return Err(SurfaceMaterialError::Invalid("reflectance range"));
    }
    Ok(())
}
fn validate_content(c: &SurfaceMaterialContent) -> Result<(), SurfaceMaterialError> {
    if c.schema_version != CONTRACT_VERSION {
        return Err(SurfaceMaterialError::Invalid("unsupported state schema"));
    }
    if !valid_id(&c.input_id) || !valid_id(&c.climate_state_id) {
        return Err(SurfaceMaterialError::Invalid("malformed state identity"));
    }
    if c.dominant_surface_reflectance_rgb_permille
        .iter()
        .any(|v| *v > 1_000)
    {
        return Err(SurfaceMaterialError::Invalid("state reflectance range"));
    }
    if c.limitations != limitations() || c.authority_effect != "none_evidence_only" {
        return Err(SurfaceMaterialError::Invalid(
            "state claim or authority drift",
        ));
    }
    Ok(())
}
fn limitations() -> Vec<String> {
    ["coarse declared three-band surface reflectance reference; not composition BRDF or scientific validation","no chemistry roughness weathering thermal mechanical crafting biome visibility traversability habitability or runtime claim"].map(String::from).to_vec()
}
fn state_id(c: &SurfaceMaterialContent) -> Result<String, SurfaceMaterialError> {
    Ok(hex(&hash(
        b"mindwarp.surface-material.state.v1\0",
        &encode(c)?,
    )))
}
fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>, SurfaceMaterialError> {
    serde_json::to_vec(v).map_err(|e| SurfaceMaterialError::Codec(e.to_string()))
}
fn decode<'a, T: Deserialize<'a>>(b: &'a [u8]) -> Result<T, SurfaceMaterialError> {
    serde_json::from_slice(b).map_err(|e| SurfaceMaterialError::Codec(e.to_string()))
}
fn valid_id(v: &str) -> bool {
    v.len() == 64
        && v.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn hash(d: &[u8], b: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(d);
    h.update(b);
    h.finalize().into()
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use climate_state::{ClimateInput, compile_climate};
    use geological_atmospheric::{GeologicalAtmosphericInput, compile_geological_atmospheric};
    use hydrological_state::{HydrologicalInput, compile_hydrological};
    use stellar_orbital::{StellarOrbitalInput, compile_stellar_orbital};
    fn input() -> SurfaceMaterialInput {
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
        let c = compile_climate(&ClimateInput {
            schema_version: 1,
            reconstruction_id: r,
            climate_source_id: [5; 32],
            hydrological: h,
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap();
        SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id: r,
            material_source_id: [6; 32],
            climate: c,
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        }
    }
    #[test]
    fn deterministic_strict_replay() {
        let c = compile_surface_material(&input()).unwrap();
        assert_eq!(c, compile_surface_material(&input()).unwrap());
        assert_eq!(
            SurfaceMaterialContract::from_bytes(&c.to_bytes().unwrap()).unwrap(),
            c
        )
    }
    #[test]
    fn reflectance_has_exact_causality() {
        let a = compile_surface_material(&input()).unwrap();
        let mut v = input();
        v.dominant_surface_reflectance_rgb_permille = [100, 800, 200];
        let b = compile_surface_material(&v).unwrap();
        assert_ne!(
            a.state.content.dominant_surface_reflectance_rgb_permille,
            b.state.content.dominant_surface_reflectance_rgb_permille
        );
        assert_eq!(
            a.state.content.climate_state_id,
            b.state.content.climate_state_id
        )
    }
    #[test]
    fn climate_moves_provenance_not_reflectance() {
        let a = compile_surface_material(&input()).unwrap();
        let mut v = input();
        v.climate.input.bond_albedo_permille = 400;
        v.climate = compile_climate(&v.climate.input).unwrap();
        let b = compile_surface_material(&v).unwrap();
        assert_ne!(
            a.state.content.climate_state_id,
            b.state.content.climate_state_id
        );
        assert_eq!(
            a.state.content.dominant_surface_reflectance_rgb_permille,
            b.state.content.dominant_surface_reflectance_rgb_permille
        )
    }
    #[test]
    fn foreign_and_fabricated_climate_fail() {
        let mut v = input();
        v.reconstruction_id = [9; 32];
        assert!(compile_surface_material(&v).is_err());
        let mut v = input();
        v.climate
            .state
            .content
            .net_radiation_quarter_billionths_earth += 1;
        assert!(matches!(
            compile_surface_material(&v),
            Err(SurfaceMaterialError::Climate(_))
        ))
    }
    #[test]
    fn ranges_fail() {
        let mut v = input();
        v.dominant_surface_reflectance_rgb_permille[0] = 1_001;
        assert_eq!(
            compile_surface_material(&v),
            Err(SurfaceMaterialError::Invalid("reflectance range"))
        )
    }
    #[test]
    fn unknown_and_noncanonical_fail() {
        let bytes = input().to_bytes().unwrap();
        let mut j: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        j["roughness"] = serde_json::json!(500);
        assert!(SurfaceMaterialInput::from_bytes(&serde_json::to_vec(&j).unwrap()).is_err());
        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(SurfaceMaterialInput::from_bytes(&spaced).is_err())
    }
    #[test]
    fn fabricated_state_and_claim_drift_fail() {
        let mut c = compile_surface_material(&input()).unwrap();
        c.state.content.dominant_surface_reflectance_rgb_permille[0] += 1;
        c.state.state_id = state_id(&c.state.content).unwrap();
        assert_eq!(
            validate_surface_material(&c),
            Err(SurfaceMaterialError::Invalid(
                "surface material state drift"
            ))
        );
        let mut s = compile_surface_material(&input()).unwrap().state;
        s.content.limitations.clear();
        assert!(s.to_bytes().is_err())
    }
}
