use derived_world_rules::{
    ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
    HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract, RegionalEnvironmentInput,
    SignalChannel, SignalPotential, StellarOrbitalContract, StellarOrbitalInput,
    WorldGenerationInput, compile_climate, compile_geological_atmospheric, compile_hydrological,
    compile_regional_environment, compile_stellar_orbital,
};
use field_basis::{FieldRecipe, ONE, Term};

pub fn world_input(reconstruction_id: [u8; 32]) -> WorldGenerationInput {
    WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id,
        surface_material: surface_contract(reconstruction_id),
        regional_environment: regional_contract(reconstruction_id),
        signal_potentials: vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }],
    }
}

fn regional_contract(reconstruction_id: [u8; 32]) -> RegionalEnvironmentContract {
    compile_regional_environment(&RegionalEnvironmentInput {
        schema_version: 1,
        reconstruction_id,
        regional_source_id: [8; 32],
        field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(ONE)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        moisture_source_id: [9; 32],
        moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        coordinate_q32_32: [0, 0],
    })
    .unwrap()
}

fn stellar_contract(reconstruction_id: [u8; 32]) -> StellarOrbitalContract {
    compile_stellar_orbital(&StellarOrbitalInput {
        schema_version: 1,
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

fn geological_contract(reconstruction_id: [u8; 32]) -> GeologicalAtmosphericContract {
    compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar_contract(reconstruction_id),
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

fn hydrological_contract(reconstruction_id: [u8; 32]) -> HydrologicalContract {
    compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological_contract(reconstruction_id),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}

fn climate_contract(reconstruction_id: [u8; 32]) -> ClimateContract {
    compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id,
        climate_source_id: [6; 32],
        hydrological: hydrological_contract(reconstruction_id),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}

fn surface_contract(reconstruction_id: [u8; 32]) -> derived_world_rules::SurfaceMaterialContract {
    derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
        schema_version: 1,
        reconstruction_id,
        material_source_id: [7; 32],
        climate: climate_contract(reconstruction_id),
        dominant_surface_reflectance_rgb_permille: [500, 400, 300],
    })
    .unwrap()
}
