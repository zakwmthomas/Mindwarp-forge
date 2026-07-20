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
use organism_subject_identity::{
    Id, bind_lifecycle_history_subject, build_form_template_identity, build_individual_identity,
    build_individual_subject_binding, build_lineage_subject_ref, build_species_candidate_identity,
    build_subject_bundle,
};
use person_form_eligibility::{
    BoundSubjectError, COMPARISON_DIMENSIONS, CapacityGrounding, PersonFormCapacity,
    capacity_concept_id, evaluate_identity_bound_person_form_prerequisites,
    evaluate_person_form_prerequisites,
};
use semantic_construction::{Claim, ClaimClass};
use sha2::{Digest, Sha256};

fn id(label: &str) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.person-form.identity-consumer.v1\0");
    hasher.update(label.as_bytes());
    hasher.finalize().into()
}

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

fn stellar(reconstruction_id: Id) -> StellarOrbitalContract {
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

fn geological(reconstruction_id: Id) -> GeologicalAtmosphericContract {
    compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar(reconstruction_id),
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

fn hydrological(reconstruction_id: Id) -> HydrologicalContract {
    compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological(reconstruction_id),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}

fn climate(reconstruction_id: Id) -> ClimateContract {
    compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id,
        climate_source_id: [6; 32],
        hydrological: hydrological(reconstruction_id),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}

fn world() -> (WorldGenerationInput, derived_world_rules::CausalWorldPacket) {
    let reconstruction_id = [1; 32];
    let input = WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id,
        surface_material: compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id,
            material_source_id: [7; 32],
            climate: climate(reconstruction_id),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap(),
        regional_environment: regional(reconstruction_id),
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

fn grounding(capacity: PersonFormCapacity, lineage_id: Id) -> CapacityGrounding {
    CapacityGrounding {
        capacity,
        lineage_id,
        claim: Claim {
            id: id(&format!("{capacity:?}-{lineage_id:?}-claim")),
            concept_id: capacity_concept_id(capacity),
            class: ClaimClass::Hypothesis,
            evidence_ref: id(&format!("{capacity:?}-{lineage_id:?}-evidence")),
        },
    }
}

#[test]
fn c6_12_identity_bound_person_form_consumer_delegates_unchanged_and_rejects_mismatch() {
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
        .map(|event| event.encode_canonical().unwrap())
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
    let bundle = build_subject_bundle(
        &input,
        &packet,
        &graph,
        &candidate,
        &fixtures.humanoid.family,
        &fixtures.humanoid.expression,
        lineage,
        form,
        species,
        individual,
        subject,
        lifecycle,
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

    let lineage_id = bundle.lineage_subject().lineage_id;
    let family_id = fixtures.humanoid.family.family_id;
    let groundings: Vec<_> = COMPARISON_DIMENSIONS
        .into_iter()
        .map(|capacity| grounding(capacity, lineage_id))
        .collect();
    let expected = evaluate_person_form_prerequisites(lineage_id, Some(family_id), &groundings);
    let actual = evaluate_identity_bound_person_form_prerequisites(
        &bundle,
        &fixtures.humanoid.family,
        &fixtures.humanoid.expression,
        MAX_VALIDATION_EXAMINATIONS,
        lineage_id,
        Some(family_id),
        &groundings,
    )
    .unwrap();
    assert_eq!(actual, expected);

    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            MAX_VALIDATION_EXAMINATIONS,
            id("foreign-lineage"),
            Some(family_id),
            &groundings,
        ),
        Err(BoundSubjectError::AssessedLineageMismatch)
    ));
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.radial.family,
            &fixtures.humanoid.expression,
            MAX_VALIDATION_EXAMINATIONS,
            lineage_id,
            Some(family_id),
            &groundings,
        ),
        Err(BoundSubjectError::BodyPlanMismatch)
    ));
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &fixtures.radial.five,
            MAX_VALIDATION_EXAMINATIONS,
            lineage_id,
            Some(family_id),
            &groundings,
        ),
        Err(BoundSubjectError::BodyPlanMismatch)
    ));
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            MAX_VALIDATION_EXAMINATIONS,
            lineage_id,
            Some(id("foreign-family")),
            &groundings,
        ),
        Err(BoundSubjectError::BodyPlanMismatch)
    ));
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            0,
            lineage_id,
            Some(family_id),
            &groundings,
        ),
        Err(BoundSubjectError::IndeterminateBudget)
    ));
    let mut invalid_expression = fixtures.humanoid.expression.clone();
    invalid_expression.occurrences.clear();
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &invalid_expression,
            MAX_VALIDATION_EXAMINATIONS,
            lineage_id,
            Some(family_id),
            &groundings,
        ),
        Err(BoundSubjectError::InvalidBodyPlanEvidence)
    ));
    let mut excessive = groundings.clone();
    excessive.push(grounding(PersonFormCapacity::Manipulation, lineage_id));
    assert!(matches!(
        evaluate_identity_bound_person_form_prerequisites(
            &bundle,
            &fixtures.humanoid.family,
            &fixtures.humanoid.expression,
            MAX_VALIDATION_EXAMINATIONS,
            lineage_id,
            Some(family_id),
            &excessive,
        ),
        Err(BoundSubjectError::GroundingLimit)
    ));
}
