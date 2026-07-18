//! Disposable-simulation-tier evidence for G1-C3: bounded causal-range and
//! regional-variation portfolio plus compile cost for `derived-world-rules`.
//!
//! No natural-method (diffusion/SDF/Voronoi/branching) candidate is
//! exercised here: the v1 contract deliberately selects none, so no P16
//! candidate-versus-baseline comparison applies to this module. This binary
//! is evidence only; it makes no scientific, visual, or production claim.

use derived_world_rules::{
    ClimateInput, GeologicalAtmosphericInput, HydrologicalInput, RegionalEnvironmentInput,
    SignalChannel, SignalPotential, StellarOrbitalInput, SurfaceMaterialInput,
    WorldGenerationInput, compile_climate, compile_geological_atmospheric, compile_hydrological,
    compile_regional_environment, compile_stellar_orbital, compile_surface_material, compile_world,
};
use field_basis::{FieldRecipe, ONE, Term};
use serde::Serialize;
use std::collections::HashSet;
use std::time::Instant;

fn base_input() -> WorldGenerationInput {
    WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id: [1; 32],
        surface_material: compile_surface_material(&SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            material_source_id: [7; 32],
            climate: compile_climate(&ClimateInput {
                schema_version: 1,
                reconstruction_id: [1; 32],
                climate_source_id: [6; 32],
                hydrological: compile_hydrological(&HydrologicalInput {
                    schema_version: 1,
                    reconstruction_id: [1; 32],
                    hydrological_source_id: [5; 32],
                    geological_atmospheric: compile_geological_atmospheric(
                        &GeologicalAtmosphericInput {
                            schema_version: 1,
                            reconstruction_id: [1; 32],
                            planetary_body_id: [4; 32],
                            stellar_orbital: compile_stellar_orbital(&StellarOrbitalInput {
                                schema_version: 1,
                                reconstruction_id: [1; 32],
                                stellar_source_id: [3; 32],
                                primary_mass_milli_solar: 1_000,
                                stellar_luminosity_millionths_solar: 1_000_000,
                                stellar_spectrum_rgb_permille: [400, 350, 250],
                                semi_major_axis_milli_au: 1_000,
                                eccentricity_millionths: 0,
                            })
                            .unwrap(),
                            planet_mass_milli_earth: 1_000,
                            planet_radius_milli_earth: 1_000,
                            internal_heat_flux_milli_w_m2: 87,
                            solid_surface_fraction_permille: 600,
                            atmospheric_column_mass_g_m2: 10_332_000,
                            gas_transmission_rgb_permille: [800, 900, 950],
                            aerosol_transmission_rgb_permille: [1_000; 3],
                        },
                    )
                    .unwrap(),
                    total_water_column_g_m2: 2_000_000,
                    phase_partition_permille: [100, 850, 50],
                    surface_accessible_liquid_fraction_permille: 700,
                })
                .unwrap(),
                bond_albedo_permille: 300,
                outgoing_longwave_fraction_of_incident_permille: 700,
            })
            .unwrap(),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap(),
        regional_environment: compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
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
        .unwrap(),
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

fn rebuild_stellar(input: &mut WorldGenerationInput) {
    let mut material_input = input.surface_material.input.clone();
    let mut climate_input = material_input.climate.input.clone();
    let mut hydrological_input = climate_input.hydrological.input.clone();
    let mut geological_input = hydrological_input.geological_atmospheric.input.clone();
    let mut stellar_input = geological_input.stellar_orbital.input.clone();
    stellar_input.reconstruction_id = input.reconstruction_id;
    geological_input.reconstruction_id = input.reconstruction_id;
    geological_input.stellar_orbital = compile_stellar_orbital(&stellar_input).unwrap();
    hydrological_input.geological_atmospheric =
        compile_geological_atmospheric(&geological_input).unwrap();
    hydrological_input.reconstruction_id = input.reconstruction_id;
    climate_input.hydrological = compile_hydrological(&hydrological_input).unwrap();
    climate_input.reconstruction_id = input.reconstruction_id;
    material_input.climate = compile_climate(&climate_input).unwrap();
    material_input.reconstruction_id = input.reconstruction_id;
    input.surface_material = compile_surface_material(&material_input).unwrap();
    let mut regional_input = input.regional_environment.input.clone();
    regional_input.reconstruction_id = input.reconstruction_id;
    input.regional_environment = compile_regional_environment(&regional_input).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_rebuild_updates_every_reconstruction_binding() {
        let mut input = base_input();
        input.reconstruction_id = [9; 32];
        rebuild_stellar(&mut input);
        assert_eq!(input.surface_material.input.reconstruction_id, [9; 32]);
        assert_eq!(input.regional_environment.input.reconstruction_id, [9; 32]);
        assert_eq!(
            input.surface_material.input.climate.input.reconstruction_id,
            [9; 32]
        );
        assert_eq!(
            input
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .reconstruction_id,
            [9; 32]
        );
        compile_world(&input).unwrap();
    }
}

#[derive(Serialize)]
struct RangeSummary {
    total_cases: u32,
    ok_cases: u32,
    unexpected_errors: Vec<String>,
    max_palette_channel_permille: u16,
    min_palette_channel_permille: u16,
}

fn range_portfolio() -> RangeSummary {
    let levels = [0u16, 1, 500, 999, 1000];
    let mut total = 0u32;
    let mut ok = 0u32;
    let mut errors = Vec::new();
    let mut max_channel = 0u16;
    let mut min_channel = u16::MAX;
    for &irr in &levels {
        for &trans in &levels {
            for &refl in &levels {
                total += 1;
                let mut input = base_input();
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
                    .input
                    .stellar_luminosity_millionths_solar = u32::from(irr) * 1_000;
                rebuild_stellar(&mut input);
                input
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric
                    .input
                    .gas_transmission_rgb_permille = [trans, trans, trans];
                input
                    .surface_material
                    .input
                    .climate
                    .input
                    .hydrological
                    .input
                    .geological_atmospheric = compile_geological_atmospheric(
                    &input
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
                input.surface_material.input.climate.input.hydrological = compile_hydrological(
                    &input
                        .surface_material
                        .input
                        .climate
                        .input
                        .hydrological
                        .input,
                )
                .unwrap();
                input.surface_material.input.climate =
                    compile_climate(&input.surface_material.input.climate.input).unwrap();
                input
                    .surface_material
                    .input
                    .dominant_surface_reflectance_rgb_permille = [refl, refl, refl];
                input.surface_material =
                    compile_surface_material(&input.surface_material.input).unwrap();
                match compile_world(&input) {
                    Ok(packet) => {
                        ok += 1;
                        for channel in packet.content.physical_palette_rgb_permille {
                            max_channel = max_channel.max(channel);
                            min_channel = min_channel.min(channel);
                            if channel > 1000 {
                                errors.push(format!(
                                    "palette channel exceeded 1000 permille at irr={irr} trans={trans} refl={refl}: {channel}"
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!(
                            "unexpected compile failure at irr={irr} trans={trans} refl={refl}: {e:?}"
                        ));
                    }
                }
            }
        }
    }
    RangeSummary {
        total_cases: total,
        ok_cases: ok,
        unexpected_errors: errors,
        max_palette_channel_permille: max_channel,
        min_palette_channel_permille: min_channel,
    }
}

#[derive(Serialize)]
struct RegionalVariationSummary {
    reconstruction_ids_tried: u32,
    distinct_input_ids: usize,
    provenance_separation_holds: bool,
    coordinates_tried: u32,
    distinct_exposures: usize,
    distinct_physical_palettes: usize,
    coordinate_variation_holds: bool,
}

fn regional_variation_portfolio() -> RegionalVariationSummary {
    let mut ids = HashSet::new();
    let mut invariant_palettes = HashSet::new();
    let tried = 32u32;
    for seed in 0..tried {
        let mut input = base_input();
        let byte = (seed % 255) as u8 + 1;
        input.reconstruction_id = [byte; 32];
        rebuild_stellar(&mut input);
        let packet = compile_world(&input).expect("fixed physical drivers must compile");
        ids.insert(packet.content.input_id.clone());
        invariant_palettes.insert(packet.content.physical_palette_rgb_permille);
    }
    let coordinates_tried = 32u32;
    let recipe_bytes = FieldRecipe::new(
        vec![Term::ValueLattice2 {
            frequency: 1,
            amplitude: ONE,
            component: 7,
        }],
        0,
    )
    .unwrap()
    .encode_canonical()
    .unwrap();
    let mut exposures = HashSet::new();
    let mut regional_palettes = HashSet::new();
    for x in 0..coordinates_tried {
        let mut input = base_input();
        input.regional_environment = compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id: input.reconstruction_id,
            regional_source_id: [8; 32],
            field_recipe_bytes: recipe_bytes.clone(),
            moisture_source_id: [9; 32],
            moisture_field_recipe_bytes: recipe_bytes.clone(),
            coordinate_q32_32: [i64::from(x) << field_basis::COORD_FRAC, 0],
        })
        .unwrap();
        let packet = compile_world(&input).expect("regional coordinate must compile");
        exposures.insert(packet.content.regional_exposure_permille);
        regional_palettes.insert(packet.content.physical_palette_rgb_permille);
    }
    RegionalVariationSummary {
        reconstruction_ids_tried: tried,
        distinct_input_ids: ids.len(),
        provenance_separation_holds: ids.len() as u32 == tried && invariant_palettes.len() == 1,
        coordinates_tried,
        distinct_exposures: exposures.len(),
        distinct_physical_palettes: regional_palettes.len(),
        coordinate_variation_holds: exposures.len() > 1 && regional_palettes.len() > 1,
    }
}

#[derive(Serialize)]
struct CostSummary {
    samples: u64,
    elapsed_seconds: f64,
    compiles_per_second: f64,
}

fn cost_portfolio() -> CostSummary {
    let samples = 50_000u64;
    let start = Instant::now();
    for i in 0..samples {
        let mut input = base_input();
        let byte = (i % 251) as u8 + 1;
        input.reconstruction_id = [byte; 32];
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
            .input
            .stellar_luminosity_millionths_solar = ((i % 1001) * 1_000) as u32;
        rebuild_stellar(&mut input);
        let _ = compile_world(&input).expect("cost loop input must be valid");
    }
    let elapsed = start.elapsed().as_secs_f64();
    CostSummary {
        samples,
        elapsed_seconds: elapsed,
        compiles_per_second: if elapsed > 0.0 {
            samples as f64 / elapsed
        } else {
            f64::INFINITY
        },
    }
}

#[derive(Serialize)]
struct Report {
    range_portfolio: RangeSummary,
    regional_variation: RegionalVariationSummary,
    cost: CostSummary,
    p16_candidate_note: &'static str,
}

fn main() {
    let report = Report {
        range_portfolio: range_portfolio(),
        regional_variation: regional_variation_portfolio(),
        cost: cost_portfolio(),
        p16_candidate_note: "v1 selects no diffusion/SDF/Voronoi/branching mechanism; contract text already states this explicitly, so no P16 candidate-versus-baseline comparison is applicable to this module",
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
