use optical_lineage_binding::*;
use physical_path_substrate::*;
use visible_radiance_bulk_transfer::*;
use visible_radiance_interface_event::*;

const ONE_Q32: i64 = 1_i64 << 32;
const ONE_Q48: u64 = 1_u64 << 48;
const ONE_Q62: i64 = 1_i64 << 62;
const HALF_Q160: &str = "730750818665451459101842416358141509827966271488";
const THREE_QUARTER_Q160: &str = "1096126227998177188652763624537212264741949407232";

fn id(byte: u8) -> Id {
    [byte; 32]
}

fn interval(bits: u16, lower: impl ToString, upper: impl ToString) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: bits,
        lower: lower.to_string(),
        upper: upper.to_string(),
    }
}

fn recipe(
    default_evidence: CellEvidenceV1,
    extent: [u32; 3],
    column_runs: Vec<ColumnRunV1>,
) -> PhysicalVolumeRecipeInputV1 {
    PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: ONE_Q32,
        extent,
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence,
        column_runs,
    }
}

fn profile(input: PhysicalVolumeRecipeInputV1, substances: &[Id]) -> VisibleRadianceBulkProfileV1 {
    compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: input.scope_id,
        reconstruction_id: input.reconstruction_id,
        profile_revision: 1,
        physical_volume_recipe_input: input,
        substance_interactions: substances
            .iter()
            .map(|source| SubstanceBulkInteractionV1 {
                substance_source_id: *source,
                bands_rgb: std::array::from_fn(|_| BulkBandInteractionV1::Finite {
                    extinction_q16_48_per_coordinate_unit: ONE_Q48,
                }),
            })
            .collect(),
    })
    .unwrap()
}

fn make_query(
    profile: &VisibleRadianceBulkProfileV1,
    source: Id,
    revision: u32,
    cell: CellIndex3V1,
    point: [SignedDecimalIntervalV1; 3],
    direction: [SignedDecimalIntervalV1; 3],
) -> ConditionalIntervalBulkQueryV1 {
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let input = ConditionalIntervalCellStepInputV1 {
        schema_version: 1,
        state_source_id: source,
        scope_id: recipe.input.scope_id,
        reconstruction_id: recipe.input.reconstruction_id,
        state_revision: revision,
        evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: cell,
        point_q160: point,
        direction_q1_62: direction,
    };
    let event = compile_conditional_interval_cell_step(&recipe, &volume, &input).unwrap();
    ConditionalIntervalBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        band: VisibleRadianceBandV1::Red,
        interval_cell_step_input: input,
        interval_cell_step_event: event,
    }
}

fn initial_query(
    profile: &VisibleRadianceBulkProfileV1,
    direction: [(i64, i64); 3],
) -> ConditionalIntervalBulkQueryV1 {
    make_query(
        profile,
        id(20),
        1,
        CellIndex3V1 { x: 0, y: 0, z: 0 },
        std::array::from_fn(|_| interval(160, HALF_Q160, HALF_Q160)),
        direction.map(|(lower, upper)| interval(62, lower, upper)),
    )
}

fn evidence(
    profile: &VisibleRadianceBulkProfileV1,
    query: ConditionalIntervalBulkQueryV1,
) -> OpticalLineageStepEvidenceV1 {
    let bulk_transfer = compile_conditional_interval_bulk_transfer(profile, &query).unwrap();
    OpticalLineageStepEvidenceV1 {
        bulk_query: query,
        bulk_transfer,
        interface_input: None,
        interface_event: None,
    }
}

fn bundle(
    profile: VisibleRadianceBulkProfileV1,
    step: OpticalLineageStepEvidenceV1,
) -> OpticalLineageBundleInputV1 {
    OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: VisibleRadianceBandV1::Red,
        steps: vec![step],
    }
}

fn assert_terminal(bundle: &OpticalLineageBundleInputV1, expected: OpticalLineageTerminalV1) {
    let manifest = compile_optical_lane_manifest(bundle).unwrap();
    assert_eq!(manifest.final_terminal, expected);
    assert_eq!(manifest.steps.len(), bundle.steps.len());
    let bundle_bytes = bundle.to_bytes().unwrap();
    assert_eq!(
        OpticalLineageBundleInputV1::from_bytes(&bundle_bytes).unwrap(),
        *bundle
    );
    let manifest_bytes = manifest.to_bytes(bundle).unwrap();
    assert_eq!(
        OpticalLaneManifestV1::from_bytes(&manifest_bytes, bundle).unwrap(),
        manifest
    );
}

#[test]
fn bulk_terminal_families_are_replayed_and_strictly_encoded() {
    let outer_profile = profile(recipe(CellEvidenceV1::Vacuum, [1, 1, 1], vec![]), &[]);
    let outer = bundle(
        outer_profile.clone(),
        evidence(
            &outer_profile,
            initial_query(&outer_profile, [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)]),
        ),
    );
    assert_terminal(&outer, OpticalLineageTerminalV1::OuterDomainExit);

    let unavailable_profile = profile(recipe(CellEvidenceV1::Unavailable, [1, 1, 1], vec![]), &[]);
    let unavailable = bundle(
        unavailable_profile.clone(),
        evidence(
            &unavailable_profile,
            initial_query(&unavailable_profile, [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)]),
        ),
    );
    assert_terminal(&unavailable, OpticalLineageTerminalV1::UnavailableCurrent);

    let neighbor_substance = id(30);
    let neighbor_profile = profile(
        recipe(
            CellEvidenceV1::Unavailable,
            [2, 1, 1],
            vec![ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 0,
                length: 1,
                evidence: CellEvidenceV1::Gas {
                    substance_source_id: neighbor_substance,
                },
            }],
        ),
        &[neighbor_substance],
    );
    let neighbor = bundle(
        neighbor_profile.clone(),
        evidence(
            &neighbor_profile,
            initial_query(&neighbor_profile, [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)]),
        ),
    );
    assert_terminal(&neighbor, OpticalLineageTerminalV1::UnavailableNeighbor);

    let ambiguous_profile = profile(recipe(CellEvidenceV1::Vacuum, [2, 2, 1], vec![]), &[]);
    let ambiguous = bundle(
        ambiguous_profile.clone(),
        evidence(
            &ambiguous_profile,
            initial_query(
                &ambiguous_profile,
                [(ONE_Q62, ONE_Q62), (ONE_Q62, ONE_Q62), (0, 0)],
            ),
        ),
    );
    assert_terminal(&ambiguous, OpticalLineageTerminalV1::AmbiguousNextFace);
    let stationary = bundle(
        ambiguous_profile.clone(),
        evidence(
            &ambiguous_profile,
            initial_query(&ambiguous_profile, [(0, 0), (0, 0), (0, 0)]),
        ),
    );
    assert_terminal(&stationary, OpticalLineageTerminalV1::NoForwardProgress);

    let mut poisoned = outer.to_bytes().unwrap();
    poisoned.push(b' ');
    assert!(OpticalLineageBundleInputV1::from_bytes(&poisoned).is_err());
    let mut unknown: serde_json::Value =
        serde_json::from_slice(&outer.to_bytes().unwrap()).unwrap();
    unknown["unexpected"] = serde_json::Value::Bool(true);
    assert!(
        OpticalLineageBundleInputV1::from_bytes(&serde_json::to_vec(&unknown).unwrap()).is_err()
    );
}

fn interface_bundle(
    selected: VisibleRadianceBandV1,
    unsupported: bool,
) -> OpticalLineageBundleInputV1 {
    let gas = id(40);
    let liquid = id(41);
    let recipe_input = recipe(
        CellEvidenceV1::Gas {
            substance_source_id: gas,
        },
        [2, 1, 1],
        vec![ColumnRunV1 {
            x_index: 1,
            y_index: 0,
            z_start: 0,
            length: 1,
            evidence: CellEvidenceV1::Liquid {
                substance_source_id: liquid,
            },
        }],
    );
    let profile = profile(recipe_input, &[gas, liquid]);
    let mut query = make_query(
        &profile,
        id(20),
        1,
        CellIndex3V1 { x: 0, y: 0, z: 0 },
        [
            interval(160, THREE_QUARTER_Q160, THREE_QUARTER_Q160),
            interval(160, HALF_Q160, HALF_Q160),
            interval(160, HALF_Q160, HALF_Q160),
        ],
        [
            interval(62, ONE_Q62 / 2, ONE_Q62),
            interval(62, ONE_Q62 / 5 * 3, ONE_Q62 / 4 * 3),
            interval(62, 0, 0),
        ],
    );
    query.band = selected;
    let transfer = compile_conditional_interval_bulk_transfer(&profile, &query).unwrap();
    let lane_id = derive_optical_lane_id(
        profile.input.reconstruction_id,
        profile.visible_radiance_bulk_profile_id,
        selected,
        transfer.interval_cell_step_input_id,
        id(20),
    )
    .unwrap();
    let incident_source_id = derive_optical_lineage_source_id(
        lane_id,
        0,
        None,
        OpticalLineageDerivedSourceRoleV1::InterfaceInput,
    )
    .unwrap();
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let bands = [
        SmoothDielectricBandV1 {
            eta_a_q16_48: ONE_Q48,
            eta_b_q16_48: 3 * ONE_Q48 / 2,
        },
        SmoothDielectricBandV1 {
            eta_a_q16_48: 2 * ONE_Q48,
            eta_b_q16_48: ONE_Q48,
        },
        SmoothDielectricBandV1 {
            eta_a_q16_48: 5 * ONE_Q48 / 4,
            eta_b_q16_48: ONE_Q48,
        },
    ];
    let model = if unsupported {
        InterfaceModelV1::Unsupported {
            model_source_id: id(42),
        }
    } else {
        InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb: bands }
    };
    let interface_input = VisibleRadianceIntervalInterfaceInputV1 {
        schema_version: 1,
        incident_source_id,
        scope_id: recipe.input.scope_id,
        reconstruction_id: recipe.input.reconstruction_id,
        incident_revision: 1,
        evidence_kind: IntervalEvidenceKindV1::DeclaredConditionalDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        source_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        target_cell: CellIndex3V1 { x: 1, y: 0, z: 0 },
        face_interaction: FaceInteractionEvidenceV1 {
            interaction_source_id: id(43),
            scope_id: recipe.input.scope_id,
            reconstruction_id: recipe.input.reconstruction_id,
            interaction_revision: 1,
            cell_a: CellIndex3V1 { x: 0, y: 0, z: 0 },
            cell_b: CellIndex3V1 { x: 1, y: 0, z: 0 },
            medium_a: CellEvidenceV1::Gas {
                substance_source_id: gas,
            },
            medium_b: CellEvidenceV1::Liquid {
                substance_source_id: liquid,
            },
            model,
        },
        incident_direction_xyz: query
            .interval_cell_step_input
            .direction_q1_62
            .each_ref()
            .map(|value| DecimalIntervalV1 {
                lower: value.lower.clone(),
                upper: value.upper.clone(),
                scale: FixedScaleV1::Q1_62,
            }),
    };
    let interface_event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &interface_input)
            .unwrap();
    OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: selected,
        steps: vec![OpticalLineageStepEvidenceV1 {
            bulk_query: query,
            bulk_transfer: transfer,
            interface_input: Some(interface_input),
            interface_event: Some(interface_event),
        }],
    }
}

#[test]
fn real_interface_owner_outputs_map_to_frozen_terminals() {
    assert_terminal(
        &interface_bundle(VisibleRadianceBandV1::Green, false),
        OpticalLineageTerminalV1::AllTir,
    );
    assert_terminal(
        &interface_bundle(VisibleRadianceBandV1::Blue, false),
        OpticalLineageTerminalV1::AmbiguousInterfaceBranch,
    );
    assert_terminal(
        &interface_bundle(VisibleRadianceBandV1::Red, true),
        OpticalLineageTerminalV1::UnsupportedInterfaceModel,
    );
}

#[test]
fn independently_resealed_local_objects_do_not_break_adjacency() {
    let valid = interface_bundle(VisibleRadianceBandV1::Green, false);
    let mut forged = valid.clone();
    forged.steps[0]
        .interface_input
        .as_mut()
        .unwrap()
        .incident_source_id = id(90);
    let recipe =
        compile_physical_volume_recipe(&forged.profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    forged.steps[0].interface_event = Some(
        compile_visible_radiance_interval_interface_event(
            &recipe,
            &volume,
            forged.steps[0].interface_input.as_ref().unwrap(),
        )
        .unwrap(),
    );
    assert!(compile_optical_lane_manifest(&forged).is_err());

    let mut drift = valid;
    drift.band = VisibleRadianceBandV1::Blue;
    assert!(compile_optical_lane_manifest(&drift).is_err());
}

fn certified_continuation(
    query: &ConditionalIntervalBulkQueryV1,
) -> (CellIndex3V1, [SignedDecimalIntervalV1; 3]) {
    let ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } =
        &query.interval_cell_step_event.outcome
    else {
        panic!("expected certified continuation")
    };
    (certified.neighbor.unwrap(), certified.point_q160.clone())
}

#[test]
fn sixty_four_replayed_steps_end_only_in_typed_work_exhaustion() {
    let profile = profile(recipe(CellEvidenceV1::Vacuum, [65, 1, 1], vec![]), &[]);
    let first_query = initial_query(&profile, [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)]);
    let first_transfer =
        compile_conditional_interval_bulk_transfer(&profile, &first_query).unwrap();
    let lane_id = derive_optical_lane_id(
        profile.input.reconstruction_id,
        profile.visible_radiance_bulk_profile_id,
        VisibleRadianceBandV1::Red,
        first_transfer.interval_cell_step_input_id,
        id(20),
    )
    .unwrap();
    let mut steps = Vec::with_capacity(MAX_LINEAGE_STEPS);
    let mut query = first_query;
    let mut predecessor = None;
    for ordinal in 0..MAX_LINEAGE_STEPS {
        let transfer = compile_conditional_interval_bulk_transfer(&profile, &query).unwrap();
        let step_id = derive_optical_lineage_step_id(
            lane_id,
            ordinal as u8,
            predecessor,
            transfer.interval_cell_step_input_id,
            transfer.interval_cell_step_event_id,
            transfer.conditional_interval_bulk_query_id,
            transfer.conditional_interval_bulk_transfer_id,
            None,
            OpticalLineageDispositionV1::ContinueSameMedium,
        )
        .unwrap();
        let (neighbor, point) = certified_continuation(&query);
        steps.push(OpticalLineageStepEvidenceV1 {
            bulk_query: query.clone(),
            bulk_transfer: transfer,
            interface_input: None,
            interface_event: None,
        });
        if ordinal + 1 < MAX_LINEAGE_STEPS {
            let source = derive_optical_lineage_source_id(
                lane_id,
                (ordinal + 1) as u8,
                Some(step_id),
                OpticalLineageDerivedSourceRoleV1::CellInput,
            )
            .unwrap();
            query = make_query(
                &profile,
                source,
                ordinal as u32 + 2,
                neighbor,
                point,
                [
                    interval(62, ONE_Q62, ONE_Q62),
                    interval(62, 0, 0),
                    interval(62, 0, 0),
                ],
            );
        }
        predecessor = Some(step_id);
    }
    let bundle = OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: VisibleRadianceBandV1::Red,
        steps,
    };
    let manifest = compile_optical_lane_manifest(&bundle).unwrap();
    assert_eq!(
        manifest.final_terminal,
        OpticalLineageTerminalV1::WorkExhaustion
    );
    assert_eq!(manifest.steps.len(), MAX_LINEAGE_STEPS);
    assert!(matches!(
        manifest.steps.last().unwrap().disposition,
        OpticalLineageDispositionV1::Terminal {
            terminal: OpticalLineageTerminalV1::WorkExhaustion
        }
    ));

    let mut forged = bundle;
    forged.steps[1]
        .bulk_query
        .interval_cell_step_input
        .state_revision = 99;
    assert!(compile_optical_lane_manifest(&forged).is_err());
}
