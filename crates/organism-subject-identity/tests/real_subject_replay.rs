use body_plan_structure::{MAX_VALIDATION_EXAMINATIONS, reference_fixtures};
use derived_world_rules::{
    ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
    HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract, RegionalEnvironmentInput,
    SignalChannel, SignalPotential, StellarOrbitalContract, StellarOrbitalInput,
    WorldGenerationInput, compile_climate, compile_geological_atmospheric, compile_hydrological,
    compile_regional_environment, compile_stellar_orbital, compile_surface_material, compile_world,
};
use entity_lifecycle::{AgeCohort, LifecycleEvent, LifecycleState};
use entity_lifecycle_history_binding::{AmbientCohortBindingV1, demo_baseline_manifest, drive};
use field_basis::{FieldRecipe, ONE, Term};
use hierarchy_history::HistoryStream;
use macro_lineage_binding::build_macro_lineage_candidate;
use niche_graph_binding::build_environmental_opportunity_graph;
use organism_subject_identity::*;

fn regional(reconstruction_id: Id) -> RegionalEnvironmentContract {
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
fn stellar(r: Id) -> StellarOrbitalContract {
    compile_stellar_orbital(&StellarOrbitalInput {
        schema_version: 1,
        reconstruction_id: r,
        stellar_source_id: [3; 32],
        primary_mass_milli_solar: 1_000,
        stellar_luminosity_millionths_solar: 1_000_000,
        stellar_spectrum_rgb_permille: [400, 350, 250],
        semi_major_axis_milli_au: 1_000,
        eccentricity_millionths: 0,
    })
    .unwrap()
}
fn geological(r: Id) -> GeologicalAtmosphericContract {
    compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id: r,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar(r),
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
fn hydrological(r: Id) -> HydrologicalContract {
    compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id: r,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological(r),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}
fn climate(r: Id) -> ClimateContract {
    compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id: r,
        climate_source_id: [6; 32],
        hydrological: hydrological(r),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}
fn world() -> (WorldGenerationInput, derived_world_rules::CausalWorldPacket) {
    let r = [1; 32];
    let input = WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id: r,
        surface_material: compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id: r,
            material_source_id: [7; 32],
            climate: climate(r),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap(),
        regional_environment: regional(r),
        signal_potentials: vec![
            SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 900,
            },
            SignalPotential {
                channel: SignalChannel::ChemicalGradient,
                baseline_strength_permille: 800,
            },
        ],
    };
    let packet = compile_world(&input).unwrap();
    (input, packet)
}

#[test]
fn c6_08_individual_identity_is_stable_across_replayed_subject_bindings() {
    let fixtures = reference_fixtures().unwrap();
    let (input, packet) = world();
    let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
    let candidate = build_macro_lineage_candidate(
        &input,
        &packet,
        &graph,
        [21; 32],
        None,
        fixtures.humanoid.family.family_id,
        vec![graph.nodes[0].id],
    )
    .unwrap();
    let lineage = build_lineage_subject_ref(
        &input,
        &packet,
        &graph,
        &candidate,
        &fixtures.humanoid.family,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    assert!(
        build_lineage_subject_ref(
            &input,
            &packet,
            &graph,
            &candidate,
            &fixtures.humanoid.family,
            32,
            MAX_VALIDATION_EXAMINATIONS + 1
        )
        .is_err()
    );
    let form = build_form_template_identity(
        &lineage,
        &fixtures.humanoid.family,
        &fixtures.humanoid.expression,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    let species = build_species_candidate_identity(&lineage, [22; 32], 32).unwrap();
    let individual = build_individual_identity(&packet.packet_id, [23; 32], 32).unwrap();
    let subject = build_individual_subject_binding(&individual, &species, &form, 32).unwrap();
    let reclassified = build_species_candidate_identity(&lineage, [27; 32], 32).unwrap();
    let rebound = build_individual_subject_binding(&individual, &reclassified, &form, 32).unwrap();
    assert_eq!(subject.individual_id, rebound.individual_id);
    assert_ne!(subject.subject_binding_id, rebound.subject_binding_id);
    let assignment = [24; 32];
    let initial = LifecycleState::ambient(AgeCohort::Young);
    let cohort =
        AmbientCohortBindingV1::new(individual.individual_id, assignment, initial.cohort).unwrap();
    let baseline = demo_baseline_manifest(individual.individual_id, [25; 32]).unwrap();
    let mut stream = HistoryStream::new(baseline.clone()).unwrap();
    let final_state = drive(
        &mut stream,
        initial,
        &[
            LifecycleEvent::BeginTracking,
            LifecycleEvent::AdvanceMaturity {
                delta_permille: 125,
            },
            LifecycleEvent::SetAppearanceLock { locked: true },
        ],
    )
    .unwrap();
    let encoded: Vec<_> = stream
        .events()
        .iter()
        .map(|e| e.encode_canonical().unwrap())
        .collect();
    let lifecycle = bind_lifecycle_history_subject(
        &individual,
        &cohort,
        assignment,
        &baseline,
        &encoded,
        initial,
        final_state,
        32,
    )
    .unwrap();
    assert!(
        build_subject_bundle(
            &input,
            &packet,
            &graph,
            &candidate,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            lineage.clone(),
            form.clone(),
            species.clone(),
            individual.clone(),
            subject.clone(),
            lifecycle.clone(),
            &cohort,
            assignment,
            &baseline,
            &encoded,
            initial,
            final_state,
            11,
            MAX_VALIDATION_EXAMINATIONS
        )
        .is_err()
    );
    let bundle = build_subject_bundle(
        &input,
        &packet,
        &graph,
        &candidate,
        &fixtures.humanoid.family,
        &fixtures.humanoid.expression,
        lineage.clone(),
        form.clone(),
        species.clone(),
        individual.clone(),
        subject,
        lifecycle.clone(),
        &cohort,
        assignment,
        &baseline,
        &encoded,
        initial,
        final_state,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    assert_eq!(bundle.individual().individual_id, individual.individual_id);
    assert_eq!(bundle.lifecycle_binding(), &lifecycle);

    let radial_candidate = build_macro_lineage_candidate(
        &input,
        &packet,
        &graph,
        [31; 32],
        None,
        fixtures.radial.family.family_id,
        vec![graph.nodes[0].id],
    )
    .unwrap();
    let radial_lineage = build_lineage_subject_ref(
        &input,
        &packet,
        &graph,
        &radial_candidate,
        &fixtures.radial.family,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    let radial_five = build_form_template_identity(
        &radial_lineage,
        &fixtures.radial.family,
        &fixtures.radial.five,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    let radial_seven = build_form_template_identity(
        &radial_lineage,
        &fixtures.radial.family,
        &fixtures.radial.seven,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    assert!(
        build_form_template_identity(
            &radial_lineage,
            &fixtures.withheld.family,
            &fixtures.withheld.expression,
            32,
            MAX_VALIDATION_EXAMINATIONS
        )
        .is_err()
    );
    let radial_species = build_species_candidate_identity(&radial_lineage, [32; 32], 32).unwrap();
    let radial_individual = build_individual_identity(&packet.packet_id, [33; 32], 32).unwrap();
    let radial_subject =
        build_individual_subject_binding(&radial_individual, &radial_species, &radial_five, 32)
            .unwrap();
    let radial_assignment = [34; 32];
    let radial_initial = LifecycleState::ambient(AgeCohort::Juvenile);
    let radial_cohort = AmbientCohortBindingV1::new(
        radial_individual.individual_id,
        radial_assignment,
        radial_initial.cohort,
    )
    .unwrap();
    let radial_baseline =
        demo_baseline_manifest(radial_individual.individual_id, [35; 32]).unwrap();
    let mut radial_stream = HistoryStream::new(radial_baseline.clone()).unwrap();
    let radial_final = drive(
        &mut radial_stream,
        radial_initial,
        &[
            LifecycleEvent::BeginTracking,
            LifecycleEvent::AdvanceMaturity {
                delta_permille: 100,
            },
        ],
    )
    .unwrap();
    let radial_encoded: Vec<_> = radial_stream
        .events()
        .iter()
        .map(|e| e.encode_canonical().unwrap())
        .collect();
    let radial_lifecycle = bind_lifecycle_history_subject(
        &radial_individual,
        &radial_cohort,
        radial_assignment,
        &radial_baseline,
        &radial_encoded,
        radial_initial,
        radial_final,
        32,
    )
    .unwrap();
    let radial_bundle = build_subject_bundle(
        &input,
        &packet,
        &graph,
        &radial_candidate,
        &fixtures.radial.family,
        &fixtures.radial.five,
        radial_lineage,
        radial_five,
        radial_species,
        radial_individual,
        radial_subject,
        radial_lifecycle,
        &radial_cohort,
        radial_assignment,
        &radial_baseline,
        &radial_encoded,
        radial_initial,
        radial_final,
        32,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    let humanoid_population = build_population_identity(&packet.packet_id, [41; 32], 32).unwrap();
    let radial_population = build_population_identity(&packet.packet_id, [42; 32], 32).unwrap();
    let second_humanoid = build_individual_identity(&packet.packet_id, [45; 32], 32).unwrap();
    let second_humanoid_binding = build_individual_subject_binding(
        &second_humanoid,
        bundle.species_candidate(),
        bundle.form_template(),
        32,
    )
    .unwrap();
    assert_ne!(
        second_humanoid.individual_id,
        bundle.individual().individual_id
    );
    assert_eq!(
        second_humanoid_binding.form_template_id,
        bundle.subject_binding().form_template_id
    );
    let receipt = build_reference_receipt(
        [43; 32],
        &bundle,
        &radial_bundle,
        &second_humanoid,
        &second_humanoid_binding,
        &radial_seven,
        &fixtures.radial.family,
        &fixtures.radial.seven,
        &humanoid_population,
        &radial_population,
        [44; 32],
        MAX_IDENTITY_EXAMINATIONS,
        MAX_VALIDATION_EXAMINATIONS,
    )
    .unwrap();
    assert!(
        build_reference_receipt(
            [43; 32],
            &bundle,
            &radial_bundle,
            &second_humanoid,
            &second_humanoid_binding,
            &radial_seven,
            &fixtures.radial.family,
            &fixtures.radial.seven,
            &humanoid_population,
            &radial_population,
            [44; 32],
            receipt.identity_validation_examinations,
            receipt.body_plan_validation_examinations,
        )
        .is_ok()
    );
    assert!(matches!(
        build_reference_receipt(
            [43; 32],
            &bundle,
            &radial_bundle,
            &second_humanoid,
            &second_humanoid_binding,
            &radial_seven,
            &fixtures.radial.family,
            &fixtures.radial.seven,
            &humanoid_population,
            &radial_population,
            [44; 32],
            receipt.identity_validation_examinations - 1,
            receipt.body_plan_validation_examinations,
        ),
        Err(IdentityError::IndeterminateBudget)
    ));
    assert!(matches!(
        build_reference_receipt(
            [43; 32],
            &bundle,
            &radial_bundle,
            &second_humanoid,
            &second_humanoid_binding,
            &radial_seven,
            &fixtures.radial.family,
            &fixtures.radial.seven,
            &humanoid_population,
            &radial_population,
            [44; 32],
            receipt.identity_validation_examinations,
            receipt.body_plan_validation_examinations - 1,
        ),
        Err(IdentityError::IndeterminateBudget)
    ));
    assert_eq!(
        receipt.lifecycle_binding_id,
        bundle.lifecycle_binding().lifecycle_binding_id
    );
    assert_eq!(
        receipt.final_history_head,
        bundle.lifecycle_binding().final_history_head.unwrap()
    );
    assert!(
        build_reference_receipt(
            [43; 32],
            &bundle,
            &radial_bundle,
            &second_humanoid,
            &second_humanoid_binding,
            &bundle.form_template().clone(),
            &fixtures.radial.family,
            &fixtures.radial.seven,
            &humanoid_population,
            &radial_population,
            [44; 32],
            MAX_IDENTITY_EXAMINATIONS,
            MAX_VALIDATION_EXAMINATIONS
        )
        .is_err()
    );

    let mut corrupt = encoded.clone();
    corrupt.last_mut().unwrap().push(0);
    assert!(
        bind_lifecycle_history_subject(
            &individual,
            &cohort,
            assignment,
            &baseline,
            &corrupt,
            initial,
            final_state,
            32
        )
        .is_err()
    );
    let foreign = build_individual_identity(&packet.packet_id, [26; 32], 32).unwrap();
    assert!(
        build_subject_bundle(
            &input,
            &packet,
            &graph,
            &candidate,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            lineage,
            form,
            species,
            foreign,
            bundle.subject_binding().clone(),
            lifecycle,
            &cohort,
            assignment,
            &baseline,
            &encoded,
            initial,
            final_state,
            32,
            MAX_VALIDATION_EXAMINATIONS
        )
        .is_err()
    );
}
