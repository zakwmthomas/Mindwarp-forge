//! Capability-free synthetic causal world contract.
//!
//! V1 proves ordering and failure boundaries only. It is not a complete planet
//! simulation, scientific-accuracy claim, visual generator, or runtime API.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub use climate_state::{ClimateContract, ClimateInput, compile_climate, validate_climate};
pub use geological_atmospheric::{
    GeologicalAtmosphericContract, GeologicalAtmosphericInput, compile_geological_atmospheric,
    validate_geological_atmospheric,
};
pub use hydrological_state::{
    HydrologicalContract, HydrologicalInput, compile_hydrological, validate_hydrological,
};
pub use regional_environment_state::{
    RegionalEnvironmentContract, RegionalEnvironmentInput, compile_regional_environment,
    validate_regional_environment,
};
pub use stellar_orbital::{
    StellarOrbitalContract, StellarOrbitalInput, compile_stellar_orbital, validate_stellar_orbital,
};
pub use surface_material_state::{
    SurfaceMaterialContract, SurfaceMaterialInput, compile_surface_material,
    validate_surface_material,
};

pub const CONTRACT_VERSION: u16 = 1;
const PERMILLE: u64 = 1_000;
const MAX_SIGNALS: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorldError {
    Invalid(&'static str),
    Codec(String),
    Stellar(stellar_orbital::StellarOrbitalError),
    Geological(geological_atmospheric::GeologicalAtmosphericError),
    Hydrological(hydrological_state::HydrologicalError),
    Climate(climate_state::ClimateError),
    SurfaceMaterial(surface_material_state::SurfaceMaterialError),
    RegionalEnvironment(regional_environment_state::RegionalEnvironmentError),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalChannel {
    VisibleRadiance,
    InfraredRadiance,
    ChemicalGradient,
    PressureWave,
    SubstrateVibration,
    ElectricField,
    MagneticField,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignalPotential {
    pub channel: SignalChannel,
    pub baseline_strength_permille: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldGenerationInput {
    pub schema_version: u16,
    pub field_contract_version: u16,
    pub reconstruction_id: [u8; 32],
    pub surface_material: SurfaceMaterialContract,
    pub regional_environment: RegionalEnvironmentContract,
    pub signal_potentials: Vec<SignalPotential>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignalAvailability {
    pub channel: SignalChannel,
    pub effective_strength_permille: u16,
    pub available: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CausalWorldContent {
    pub schema_version: u16,
    pub input_id: String,
    pub physical_palette_rgb_permille: [u16; 3],
    pub regional_exposure_permille: u16,
    pub signals: Vec<SignalAvailability>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CausalWorldPacket {
    pub packet_id: String,
    pub content: CausalWorldContent,
}

impl WorldGenerationInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, WorldError> {
        validate_input(self)?;
        let mut canonical = self.clone();
        canonical
            .signal_potentials
            .sort_by_key(|potential| potential.channel);
        serde_json::to_vec(&canonical).map_err(|error| WorldError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WorldError> {
        let value: Self =
            serde_json::from_slice(bytes).map_err(|error| WorldError::Codec(error.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(WorldError::Invalid("noncanonical input bytes"));
        }
        Ok(value)
    }
}

impl CausalWorldPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>, WorldError> {
        validate_content(&self.content)?;
        if self.packet_id != packet_id(&self.content)? {
            return Err(WorldError::Invalid("packet identity drift"));
        }
        serde_json::to_vec(self).map_err(|error| WorldError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WorldError> {
        let value: Self =
            serde_json::from_slice(bytes).map_err(|error| WorldError::Codec(error.to_string()))?;
        validate_content(&value.content)?;
        if value.to_bytes()? != bytes || value.packet_id != packet_id(&value.content)? {
            return Err(WorldError::Invalid("noncanonical or drifted packet"));
        }
        Ok(value)
    }
}

pub fn compile_world(input: &WorldGenerationInput) -> Result<CausalWorldPacket, WorldError> {
    let input_bytes = input.to_bytes()?;
    let input_id = hex(&domain_hash(
        b"mindwarp.derived-world.input.v1\0",
        &input_bytes,
    ));
    let mut palette = [0_u16; 3];
    for (index, output) in palette.iter_mut().enumerate() {
        let product = u64::from(
            input
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .input
                .stellar_orbital
                .state
                .content
                .bounded_stellar_irradiance_rgb_permille[index],
        ) * u64::from(
            input
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .state
                .content
                .atmosphere_transmission_rgb_permille[index],
        ) * u64::from(
            input
                .surface_material
                .state
                .content
                .dominant_surface_reflectance_rgb_permille[index],
        ) * u64::from(input.regional_environment.state.content.exposure_permille);
        *output = u16::try_from((product + 500_000_000) / 1_000_000_000)
            .map_err(|_| WorldError::Invalid("palette overflow"))?;
    }

    let mut signal_potentials: Vec<_> = input.signal_potentials.iter().collect();
    signal_potentials.sort_by_key(|potential| potential.channel);
    let signals = signal_potentials
        .into_iter()
        .map(|potential| {
            let regional_exposure = if potential.channel == SignalChannel::VisibleRadiance {
                u64::from(input.regional_environment.state.content.exposure_permille)
            } else {
                PERMILLE
            };
            let effective = (u64::from(potential.baseline_strength_permille) * regional_exposure
                + PERMILLE / 2)
                / PERMILLE;
            let effective_strength_permille =
                u16::try_from(effective).map_err(|_| WorldError::Invalid("signal overflow"))?;
            Ok(SignalAvailability {
                channel: potential.channel,
                effective_strength_permille,
                available: effective_strength_permille > 0,
            })
        })
        .collect::<Result<Vec<_>, WorldError>>()?;

    let content = CausalWorldContent {
        schema_version: CONTRACT_VERSION,
        input_id,
        physical_palette_rgb_permille: palette,
        regional_exposure_permille: input.regional_environment.state.content.exposure_permille,
        signals,
        limitations: vec![
            "synthetic causal reference; not a complete scientific planet model".into(),
            "no organism biome aesthetic shader asset runtime or performance claim".into(),
            "signal strengths are normalized baseline potentials; no propagation distance attenuation or biological detectability claim".into(),
        ],
        authority_effect: "none_evidence_only".into(),
    };
    Ok(CausalWorldPacket {
        packet_id: packet_id(&content)?,
        content,
    })
}

pub fn validate_world_packet(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<(), WorldError> {
    if &compile_world(input)? != packet {
        return Err(WorldError::Invalid("causal output drift"));
    }
    Ok(())
}

fn validate_content(content: &CausalWorldContent) -> Result<(), WorldError> {
    if content.schema_version != CONTRACT_VERSION {
        return Err(WorldError::Invalid("unsupported packet schema"));
    }
    if content.input_id.len() != 64
        || !content
            .input_id
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(WorldError::Invalid("malformed input identity"));
    }
    if content
        .physical_palette_rgb_permille
        .iter()
        .any(|value| *value > 1_000)
    {
        return Err(WorldError::Invalid("packet palette range"));
    }
    if content.regional_exposure_permille > 1_000 {
        return Err(WorldError::Invalid("packet regional exposure range"));
    }
    let mut previous = None;
    for signal in &content.signals {
        if signal.effective_strength_permille > 1_000
            || signal.available != (signal.effective_strength_permille > 0)
        {
            return Err(WorldError::Invalid("packet signal invariant"));
        }
        if previous.is_some_and(|channel| channel >= signal.channel) {
            return Err(WorldError::Invalid("packet signal order or duplicate"));
        }
        previous = Some(signal.channel);
    }
    if content.limitations
        != [
            "synthetic causal reference; not a complete scientific planet model",
            "no organism biome aesthetic shader asset runtime or performance claim",
            "signal strengths are normalized baseline potentials; no propagation distance attenuation or biological detectability claim",
        ]
        .map(String::from)
        .to_vec()
        || content.authority_effect != "none_evidence_only"
    {
        return Err(WorldError::Invalid("packet claim or authority drift"));
    }
    Ok(())
}

fn validate_input(input: &WorldGenerationInput) -> Result<(), WorldError> {
    if input.schema_version != CONTRACT_VERSION
        || input.field_contract_version != field_basis::CONTRACT_VERSION
    {
        return Err(WorldError::Invalid("unsupported contract version"));
    }
    if input.reconstruction_id == [0; 32] {
        return Err(WorldError::Invalid("missing identity binding"));
    }
    validate_surface_material(&input.surface_material).map_err(WorldError::SurfaceMaterial)?;
    if input.reconstruction_id != input.surface_material.input.reconstruction_id {
        return Err(WorldError::Invalid(
            "surface material reconstruction mismatch",
        ));
    }
    validate_regional_environment(&input.regional_environment)
        .map_err(WorldError::RegionalEnvironment)?;
    if input.reconstruction_id != input.regional_environment.input.reconstruction_id {
        return Err(WorldError::Invalid(
            "regional environment reconstruction mismatch",
        ));
    }
    if input.signal_potentials.len() > MAX_SIGNALS {
        return Err(WorldError::Invalid("signal count"));
    }
    let mut channels = BTreeSet::new();
    for potential in &input.signal_potentials {
        if potential.baseline_strength_permille > 1_000 || !channels.insert(potential.channel) {
            return Err(WorldError::Invalid("signal range or duplicate"));
        }
        if potential.baseline_strength_permille == 0 {
            continue;
        }
        match potential.channel {
            SignalChannel::PressureWave
                if input
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric
                    .state
                    .content
                    .surface_pressure_pa
                    == 0 =>
            {
                return Err(WorldError::Invalid("pressure signal without atmosphere"));
            }
            SignalChannel::ChemicalGradient
                if input
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric
                    .state
                    .content
                    .surface_pressure_pa
                    == 0
                    && !input
                        .surface_material
                        .input
                        .climate
                        .input
                        .hydrological
                        .state
                        .content
                        .has_surface_accessible_liquid =>
            {
                return Err(WorldError::Invalid("chemical signal without medium"));
            }
            SignalChannel::SubstrateVibration
                if input
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric
                    .state
                    .content
                    .solid_surface_fraction_permille
                    == 0 =>
            {
                return Err(WorldError::Invalid("vibration without substrate"));
            }
            _ => {}
        }
    }
    Ok(())
}

fn packet_id(content: &CausalWorldContent) -> Result<String, WorldError> {
    let bytes =
        serde_json::to_vec(content).map_err(|error| WorldError::Codec(error.to_string()))?;
    Ok(hex(&domain_hash(
        b"mindwarp.derived-world.packet.v1\0",
        &bytes,
    )))
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
    use field_basis::{FieldRecipe, ONE, Term};

    fn stellar_contract(reconstruction_id: [u8; 32], spectrum: [u16; 3]) -> StellarOrbitalContract {
        compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: stellar_orbital::CONTRACT_VERSION,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: spectrum,
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn geological_contract(
        reconstruction_id: [u8; 32],
        spectrum: [u16; 3],
    ) -> GeologicalAtmosphericContract {
        compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: geological_atmospheric::CONTRACT_VERSION,
            reconstruction_id,
            planetary_body_id: [4; 32],
            stellar_orbital: stellar_contract(reconstruction_id, spectrum),
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

    fn hydrological_contract(
        reconstruction_id: [u8; 32],
        spectrum: [u16; 3],
    ) -> HydrologicalContract {
        compile_hydrological(&HydrologicalInput {
            schema_version: hydrological_state::CONTRACT_VERSION,
            reconstruction_id,
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract(reconstruction_id, spectrum),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap()
    }

    fn climate_contract(reconstruction_id: [u8; 32], spectrum: [u16; 3]) -> ClimateContract {
        compile_climate(&ClimateInput {
            schema_version: climate_state::CONTRACT_VERSION,
            reconstruction_id,
            climate_source_id: [6; 32],
            hydrological: hydrological_contract(reconstruction_id, spectrum),
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }

    fn surface_material_contract(
        reconstruction_id: [u8; 32],
        spectrum: [u16; 3],
    ) -> SurfaceMaterialContract {
        compile_surface_material(&SurfaceMaterialInput {
            schema_version: surface_material_state::CONTRACT_VERSION,
            reconstruction_id,
            material_source_id: [7; 32],
            climate: climate_contract(reconstruction_id, spectrum),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap()
    }

    fn regional_contract(
        reconstruction_id: [u8; 32],
        term: Term,
        coordinate_q32_32: [i64; 2],
    ) -> RegionalEnvironmentContract {
        compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: regional_environment_state::CONTRACT_VERSION,
            reconstruction_id,
            regional_source_id: [8; 32],
            field_recipe_bytes: FieldRecipe::new(vec![term], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            moisture_source_id: [9; 32],
            moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            coordinate_q32_32,
        })
        .unwrap()
    }

    fn recompile_nested_state(value: &mut WorldGenerationInput) {
        value
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric = compile_geological_atmospheric(
            &value
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .input,
        )
        .unwrap();
        value.surface_material.input.climate.input.hydrological = compile_hydrological(
            &value
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input,
        )
        .unwrap();
        value.surface_material.input.climate =
            compile_climate(&value.surface_material.input.climate.input).unwrap();
        value.surface_material = compile_surface_material(&value.surface_material.input).unwrap();
    }

    fn input() -> WorldGenerationInput {
        WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            surface_material: surface_material_contract([1; 32], [400, 350, 250]),
            regional_environment: regional_contract([1; 32], Term::Constant(ONE), [0, 0]),
            signal_potentials: vec![
                SignalPotential {
                    channel: SignalChannel::VisibleRadiance,
                    baseline_strength_permille: 900,
                },
                SignalPotential {
                    channel: SignalChannel::PressureWave,
                    baseline_strength_permille: 600,
                },
            ],
        }
    }

    #[test]
    fn compile_is_deterministic_strict_and_replayable() {
        let input = input();
        let first = compile_world(&input).unwrap();
        assert_eq!(first, compile_world(&input).unwrap());
        validate_world_packet(&input, &first).unwrap();
        assert_eq!(
            CausalWorldPacket::from_bytes(&first.to_bytes().unwrap()).unwrap(),
            first
        );
        assert_eq!(
            WorldGenerationInput::from_bytes(&input.to_bytes().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn palette_is_caused_by_star_atmosphere_and_material() {
        let base = input();
        let first = compile_world(&base).unwrap();
        let mut changed = base.clone();
        changed.surface_material = surface_material_contract([1; 32], [250, 350, 400]);
        let stellar_changed = compile_world(&changed).unwrap();
        assert_ne!(
            first.content.physical_palette_rgb_permille,
            stellar_changed.content.physical_palette_rgb_permille
        );
        changed = base.clone();
        changed
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric
            .input
            .gas_transmission_rgb_permille = [200, 900, 950];
        recompile_nested_state(&mut changed);
        let second = compile_world(&changed).unwrap();
        assert_ne!(
            first.content.physical_palette_rgb_permille,
            second.content.physical_palette_rgb_permille
        );
        changed = base.clone();
        changed
            .surface_material
            .input
            .dominant_surface_reflectance_rgb_permille = [100, 800, 200];
        changed.surface_material =
            compile_surface_material(&changed.surface_material.input).unwrap();
        let material_changed = compile_world(&changed).unwrap();
        assert_ne!(
            first.content.physical_palette_rgb_permille,
            material_changed.content.physical_palette_rgb_permille
        );
        assert!(
            first
                .content
                .limitations
                .iter()
                .any(|limit| limit.contains("no organism biome aesthetic"))
        );
    }

    #[test]
    fn regional_coordinates_cause_palette_and_visible_signal_variation() {
        let term = Term::ValueLattice2 {
            frequency: 1,
            amplitude: ONE,
            component: 7,
        };
        let mut first = input();
        first.regional_environment = regional_contract([1; 32], term.clone(), [0, 0]);
        let mut second = first.clone();
        second.regional_environment =
            regional_contract([1; 32], term, [1_i64 << field_basis::COORD_FRAC, 0]);
        let a = compile_world(&first).unwrap();
        let b = compile_world(&second).unwrap();
        assert_ne!(
            a.content.regional_exposure_permille,
            b.content.regional_exposure_permille
        );
        assert_ne!(
            a.content.physical_palette_rgb_permille,
            b.content.physical_palette_rgb_permille
        );
        assert_ne!(a.content.signals[0], b.content.signals[0]);
        assert_eq!(a.content.signals[1], b.content.signals[1]);
    }

    #[test]
    fn medium_dependent_signals_fail_closed() {
        let cases = [
            (SignalChannel::PressureWave, 0, 500, 500),
            (SignalChannel::ChemicalGradient, 0, 0, 500),
            (SignalChannel::SubstrateVibration, 10, 500, 0),
        ];
        for (channel, pressure, liquid, substrate) in cases {
            let mut value = input();
            if liquid == 0 {
                value
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .phase_partition_permille = [900, 0, 100];
                value
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .surface_accessible_liquid_fraction_permille = 0;
            }
            value
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .input
                .atmospheric_column_mass_g_m2 = if pressure == 0 { 0 } else { 10_332_000 };
            if pressure == 0 {
                value
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric
                    .input
                    .gas_transmission_rgb_permille = [1_000; 3];
            }
            value
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .input
                .solid_surface_fraction_permille = substrate;
            recompile_nested_state(&mut value);
            value.signal_potentials = vec![SignalPotential {
                channel,
                baseline_strength_permille: 500,
            }];
            assert!(compile_world(&value).is_err());
        }
    }

    #[test]
    fn a_valid_signal_ecology_need_not_have_visible_light_or_eyes() {
        let mut value = input();
        value
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric
            .input
            .gas_transmission_rgb_permille = [0; 3];
        recompile_nested_state(&mut value);
        value.signal_potentials = vec![
            SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 0,
            },
            SignalPotential {
                channel: SignalChannel::MagneticField,
                baseline_strength_permille: 800,
            },
        ];
        let packet = compile_world(&value).unwrap();
        assert!(!packet.content.signals[0].available);
        assert!(packet.content.signals[1].available);
        assert_eq!(packet.content.physical_palette_rgb_permille, [0; 3]);
    }

    #[test]
    fn ranges_duplicates_unknown_fields_and_noncanonical_bytes_fail() {
        let mut invalid = input();
        invalid
            .signal_potentials
            .push(invalid.signal_potentials[0].clone());
        assert!(compile_world(&invalid).is_err());

        let bytes = input().to_bytes().unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["solver"] = serde_json::json!("universal_lightning");
        assert!(WorldGenerationInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());

        let mut legacy: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        legacy["signal_potentials"][0]["transmission_permille"] = serde_json::json!(900);
        assert!(WorldGenerationInput::from_bytes(&serde_json::to_vec(&legacy).unwrap()).is_err());

        let mut spaced = bytes;
        spaced.push(b' ');
        assert!(WorldGenerationInput::from_bytes(&spaced).is_err());
    }

    #[test]
    fn baseline_potentials_do_not_claim_unimplemented_propagation() {
        let packet = compile_world(&input()).unwrap();
        assert_eq!(packet.content.signals[0].effective_strength_permille, 900);
        assert_eq!(packet.content.signals[1].effective_strength_permille, 600);
        assert!(packet.content.limitations.iter().any(|limit| {
            limit.contains("no propagation distance attenuation or biological detectability")
        }));
    }

    #[test]
    fn identity_changes_provenance_without_fabricating_physical_difference() {
        let first = input();
        let mut second = first.clone();
        second.reconstruction_id = [9; 32];
        second.surface_material = surface_material_contract([9; 32], [400, 350, 250]);
        second.regional_environment = regional_contract([9; 32], Term::Constant(ONE), [0, 0]);
        let a = compile_world(&first).unwrap();
        let b = compile_world(&second).unwrap();
        assert_ne!(a.content.input_id, b.content.input_id);
        assert_eq!(
            a.content.physical_palette_rgb_permille,
            b.content.physical_palette_rgb_permille
        );
    }

    #[test]
    fn signal_set_order_is_canonical() {
        let first = input();
        let mut permuted = first.clone();
        permuted.signal_potentials.reverse();
        assert_eq!(first.to_bytes().unwrap(), permuted.to_bytes().unwrap());
        assert_eq!(
            compile_world(&first).unwrap(),
            compile_world(&permuted).unwrap()
        );
    }

    #[test]
    fn fabricated_packet_content_fails_before_serialization() {
        let mut packet = compile_world(&input()).unwrap();
        packet.content.physical_palette_rgb_permille[0] = 9_999;
        assert_eq!(
            packet.to_bytes(),
            Err(WorldError::Invalid("packet palette range"))
        );

        let mut packet = compile_world(&input()).unwrap();
        packet.content.signals[0].available = false;
        assert_eq!(
            packet.to_bytes(),
            Err(WorldError::Invalid("packet signal invariant"))
        );
    }

    #[test]
    fn foreign_or_fabricated_stellar_orbital_state_fails_before_world_compile() {
        let mut foreign = input();
        foreign.surface_material = surface_material_contract([9; 32], [400, 350, 250]);
        assert_eq!(
            compile_world(&foreign),
            Err(WorldError::Invalid(
                "surface material reconstruction mismatch"
            ))
        );

        let mut fabricated = input();
        fabricated
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric
            .input
            .stellar_orbital
            .state
            .content
            .irradiance_mean_distance_millionths_earth += 1;
        assert!(matches!(
            compile_world(&fabricated),
            Err(WorldError::SurfaceMaterial(_))
        ));
    }
}
