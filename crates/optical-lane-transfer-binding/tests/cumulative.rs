use optical_lane_transfer_binding::*;
use optical_lineage_binding::*;
use physical_path_substrate::*;
use visible_radiance_bulk_transfer::*;
use visible_radiance_interface_event::*;

const ONE_Q32: i64 = 1_i64 << 32;
const ONE_Q48: u64 = 1_u64 << 48;
const ONE_Q62: i64 = 1_i64 << 62;
const HALF_Q160: &str = "730750818665451459101842416358141509827966271488";
const THREE_QUARTER_Q160: &str = "1096126227998177188652763624537212264741949407232";

fn id(byte: u8) -> physical_path_substrate::Id {
    [byte; 32]
}

fn interval(bits: u16, value: impl ToString) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: bits,
        lower: value.to_string(),
        upper: value.to_string(),
    }
}

fn profile(
    evidence: CellEvidenceV1,
    interaction: Option<BulkBandInteractionV1>,
) -> VisibleRadianceBulkProfileV1 {
    let substance = match evidence {
        CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } => Some(substance_source_id),
        _ => None,
    };
    compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: id(2),
        reconstruction_id: id(3),
        profile_revision: 1,
        physical_volume_recipe_input: PhysicalVolumeRecipeInputV1 {
            schema_version: 1,
            recipe_source_id: id(1),
            scope_id: id(2),
            reconstruction_id: id(3),
            recipe_revision: 1,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            origin_q32_32: [0; 3],
            cell_step_q32_32: ONE_Q32,
            extent: [1, 1, 1],
            boundary_mode: BoundaryModeV1::BoundedAbsent,
            adjacency: AdjacencyV1::SharedFace6,
            default_evidence: evidence,
            column_runs: vec![],
        },
        substance_interactions: substance
            .into_iter()
            .map(|substance_source_id| SubstanceBulkInteractionV1 {
                substance_source_id,
                bands_rgb: std::array::from_fn(|_| interaction.clone().unwrap()),
            })
            .collect(),
    })
    .unwrap()
}

fn input(
    evidence: CellEvidenceV1,
    interaction: Option<BulkBandInteractionV1>,
) -> CumulativeOpticalLaneTransferInputV1 {
    let profile = profile(evidence, interaction);
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let cell_input = ConditionalIntervalCellStepInputV1 {
        schema_version: 1,
        state_source_id: id(20),
        scope_id: id(2),
        reconstruction_id: id(3),
        state_revision: 1,
        evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        point_q160: std::array::from_fn(|_| interval(160, HALF_Q160)),
        direction_q1_62: [interval(62, ONE_Q62), interval(62, 0), interval(62, 0)],
    };
    let cell_event = compile_conditional_interval_cell_step(&recipe, &volume, &cell_input).unwrap();
    let query = ConditionalIntervalBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        band: VisibleRadianceBandV1::Red,
        interval_cell_step_input: cell_input,
        interval_cell_step_event: cell_event,
    };
    let transfer = compile_conditional_interval_bulk_transfer(&profile, &query).unwrap();
    let bundle = OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: VisibleRadianceBandV1::Red,
        steps: vec![OpticalLineageStepEvidenceV1 {
            bulk_query: query,
            bulk_transfer: transfer,
            interface_input: None,
            interface_event: None,
        }],
    };
    let manifest = compile_optical_lane_manifest(&bundle).unwrap();
    CumulativeOpticalLaneTransferInputV1 {
        schema_version: 1,
        bundle,
        manifest,
    }
}

#[test]
fn derives_vacuum_finite_opaque_and_unavailable_without_factor_injection() {
    let vacuum = input(CellEvidenceV1::Vacuum, None);
    let vacuum_result = compile_cumulative_optical_lane_transfer(&vacuum).unwrap();
    assert_eq!(vacuum_result.factors.len(), 1);
    assert_eq!(vacuum_result.cumulative_lower_q0_48, ONE_Q48);
    assert_eq!(vacuum_result.cumulative_upper_q0_48, ONE_Q48);

    let substance = id(30);
    let finite = input(
        CellEvidenceV1::Gas {
            substance_source_id: substance,
        },
        Some(BulkBandInteractionV1::Finite {
            extinction_q16_48_per_coordinate_unit: ONE_Q48,
        }),
    );
    let finite_result = compile_cumulative_optical_lane_transfer(&finite).unwrap();
    assert_eq!(finite_result.factors.len(), 1);
    assert!(finite_result.cumulative_upper_q0_48 < ONE_Q48);
    assert!(finite_result.cumulative_lower_q0_48 <= finite_result.cumulative_upper_q0_48);

    let opaque = input(
        CellEvidenceV1::Solid {
            substance_source_id: id(31),
        },
        Some(BulkBandInteractionV1::Opaque),
    );
    let opaque_result = compile_cumulative_optical_lane_transfer(&opaque).unwrap();
    assert_eq!(
        (
            opaque_result.cumulative_lower_q0_48,
            opaque_result.cumulative_upper_q0_48
        ),
        (0, 0)
    );

    let unavailable = input(CellEvidenceV1::Unavailable, None);
    let unavailable_result = compile_cumulative_optical_lane_transfer(&unavailable).unwrap();
    assert!(unavailable_result.factors.is_empty());
    assert_eq!(
        unavailable_result.final_terminal,
        OpticalLineageTerminalV1::UnavailableCurrent
    );
    assert_eq!(
        (
            unavailable_result.cumulative_lower_q0_48,
            unavailable_result.cumulative_upper_q0_48
        ),
        (ONE_Q48, ONE_Q48)
    );
}

#[test]
fn strict_codecs_replay_and_reject_output_or_authority_forgery() {
    let input = input(CellEvidenceV1::Vacuum, None);
    let result = compile_cumulative_optical_lane_transfer(&input).unwrap();
    let input_bytes = input.to_bytes().unwrap();
    assert_eq!(
        CumulativeOpticalLaneTransferInputV1::from_bytes(&input_bytes).unwrap(),
        input
    );
    let output_bytes = result.to_bytes(&input).unwrap();
    assert_eq!(
        CumulativeOpticalLaneTransferV1::from_bytes(&output_bytes, &input).unwrap(),
        result
    );

    let mut trailing = input_bytes.clone();
    trailing.push(b' ');
    assert!(CumulativeOpticalLaneTransferInputV1::from_bytes(&trailing).is_err());
    let mut unknown: serde_json::Value = serde_json::from_slice(&input_bytes).unwrap();
    unknown["unexpected"] = serde_json::Value::Bool(true);
    assert!(
        CumulativeOpticalLaneTransferInputV1::from_bytes(&serde_json::to_vec(&unknown).unwrap())
            .is_err()
    );

    let mut forged = result.clone();
    forged.authority_effect = "promote".into();
    assert!(forged.to_bytes(&input).is_err());
    let mut reordered = result;
    reordered.factors[0].role = CumulativeLaneFactorRoleV1::TransmittedInterface;
    assert!(reordered.to_bytes(&input).is_err());

    let canonical = compile_cumulative_optical_lane_transfer(&input).unwrap();
    let mutations: Vec<Box<dyn Fn(&mut CumulativeOpticalLaneTransferV1)>> = vec![
        Box::new(|value| value.factors.clear()),
        Box::new(|value| value.factors.push(value.factors[0].clone())),
        Box::new(|value| value.factors[0].band = VisibleRadianceBandV1::Blue),
        Box::new(|value| value.factors[0].owner_object_id = id(99)),
        Box::new(|value| value.factors[0].lower_q0_48 = 0),
        Box::new(|value| value.factors[0].factor_id = id(98)),
        Box::new(|value| value.final_terminal = OpticalLineageTerminalV1::AllTir),
        Box::new(|value| value.limitations.clear()),
        Box::new(|value| value.result_id = id(97)),
        Box::new(|value| value.transcript_id = id(96)),
    ];
    for mutation in mutations {
        let mut forged = canonical.clone();
        mutation(&mut forged);
        assert!(forged.to_bytes(&input).is_err());
    }
    let mut stale = input.clone();
    stale.bundle.steps[0]
        .bulk_query
        .interval_cell_step_input
        .state_revision = 9;
    assert!(compile_cumulative_optical_lane_transfer(&stale).is_err());
    let mut unknown_output: serde_json::Value =
        serde_json::from_slice(&canonical.to_bytes(&input).unwrap()).unwrap();
    unknown_output["unexpected"] = serde_json::Value::Bool(true);
    assert!(
        CumulativeOpticalLaneTransferV1::from_bytes(
            &serde_json::to_vec(&unknown_output).unwrap(),
            &input,
        )
        .is_err()
    );
}

fn interface_bundle(
    selected: VisibleRadianceBandV1,
    unsupported: bool,
    extend_transmit: bool,
) -> OpticalLineageBundleInputV1 {
    let gas = id(40);
    let liquid = id(41);
    let recipe_input = PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: ONE_Q32,
        extent: [2, 1, 1],
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence: CellEvidenceV1::Gas {
            substance_source_id: gas,
        },
        column_runs: vec![ColumnRunV1 {
            x_index: 1,
            y_index: 0,
            z_start: 0,
            length: 1,
            evidence: CellEvidenceV1::Liquid {
                substance_source_id: liquid,
            },
        }],
    };
    let profile = compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: id(2),
        reconstruction_id: id(3),
        profile_revision: 1,
        physical_volume_recipe_input: recipe_input,
        substance_interactions: [gas, liquid]
            .into_iter()
            .map(|substance_source_id| SubstanceBulkInteractionV1 {
                substance_source_id,
                bands_rgb: std::array::from_fn(|_| BulkBandInteractionV1::Finite {
                    extinction_q16_48_per_coordinate_unit: ONE_Q48,
                }),
            })
            .collect(),
    })
    .unwrap();
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let first_input = ConditionalIntervalCellStepInputV1 {
        schema_version: 1,
        state_source_id: id(20),
        scope_id: id(2),
        reconstruction_id: id(3),
        state_revision: 1,
        evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        point_q160: [
            interval(160, THREE_QUARTER_Q160),
            interval(160, HALF_Q160),
            interval(160, HALF_Q160),
        ],
        direction_q1_62: [
            SignedDecimalIntervalV1 {
                fractional_bits: 62,
                lower: (ONE_Q62 / 2).to_string(),
                upper: ONE_Q62.to_string(),
            },
            SignedDecimalIntervalV1 {
                fractional_bits: 62,
                lower: (ONE_Q62 / 5 * 3).to_string(),
                upper: (ONE_Q62 / 4 * 3).to_string(),
            },
            interval(62, 0),
        ],
    };
    let first_event =
        compile_conditional_interval_cell_step(&recipe, &volume, &first_input).unwrap();
    let mut first_query = ConditionalIntervalBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        band: selected,
        interval_cell_step_input: first_input,
        interval_cell_step_event: first_event,
    };
    first_query.band = selected;
    let first_transfer =
        compile_conditional_interval_bulk_transfer(&profile, &first_query).unwrap();
    let lane_id = derive_optical_lane_id(
        profile.input.reconstruction_id,
        profile.visible_radiance_bulk_profile_id,
        selected,
        first_transfer.interval_cell_step_input_id,
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
    let model = if unsupported {
        InterfaceModelV1::Unsupported {
            model_source_id: id(42),
        }
    } else {
        InterfaceModelV1::SmoothLosslessUnpolarizedDielectric {
            bands_rgb: [
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
            ],
        }
    };
    let interface_input = VisibleRadianceIntervalInterfaceInputV1 {
        schema_version: 1,
        incident_source_id,
        scope_id: id(2),
        reconstruction_id: id(3),
        incident_revision: 1,
        evidence_kind: IntervalEvidenceKindV1::DeclaredConditionalDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        source_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        target_cell: CellIndex3V1 { x: 1, y: 0, z: 0 },
        face_interaction: FaceInteractionEvidenceV1 {
            interaction_source_id: id(43),
            scope_id: id(2),
            reconstruction_id: id(3),
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
        incident_direction_xyz: first_query
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
    let mut steps = vec![OpticalLineageStepEvidenceV1 {
        bulk_query: first_query.clone(),
        bulk_transfer: first_transfer.clone(),
        interface_input: Some(interface_input),
        interface_event: Some(interface_event.clone()),
    }];
    if extend_transmit {
        let bands = match &interface_event.outcome {
            IntervalInterfaceOutcomeV1::Evaluated { bands_rgb, .. } => bands_rgb,
            _ => panic!("expected evaluated interface"),
        };
        let transmitted = match &bands[0] {
            IntervalBandOutcomeV1::BoundedEnclosure {
                branch: IntervalUniformBranchV1::AllTransmit,
                event,
            } => event.transmitted_direction_xyz.as_ref().unwrap(),
            _ => panic!("expected red all-transmit"),
        };
        let first_step_id = derive_optical_lineage_step_id(
            lane_id,
            0,
            None,
            first_transfer.interval_cell_step_input_id,
            first_transfer.interval_cell_step_event_id,
            first_transfer.conditional_interval_bulk_query_id,
            first_transfer.conditional_interval_bulk_transfer_id,
            Some((
                interface_event.interval_interface_input_id,
                interface_event.event_id,
            )),
            OpticalLineageDispositionV1::ContinueAfterInterface,
        )
        .unwrap();
        let certified = match &first_query.interval_cell_step_event.outcome {
            ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } => certified,
            _ => panic!("expected certified face"),
        };
        let second_input = ConditionalIntervalCellStepInputV1 {
            schema_version: 1,
            state_source_id: derive_optical_lineage_source_id(
                lane_id,
                1,
                Some(first_step_id),
                OpticalLineageDerivedSourceRoleV1::CellInput,
            )
            .unwrap(),
            scope_id: id(2),
            reconstruction_id: id(3),
            state_revision: 2,
            evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
            physical_volume_recipe_id: recipe.physical_volume_recipe_id,
            physical_volume_id: volume.physical_volume_id,
            current_cell: certified.neighbor.unwrap(),
            point_q160: certified.point_q160.clone(),
            direction_q1_62: transmitted.each_ref().map(|value| SignedDecimalIntervalV1 {
                fractional_bits: 62,
                lower: value.lower.clone(),
                upper: value.upper.clone(),
            }),
        };
        let second_event =
            compile_conditional_interval_cell_step(&recipe, &volume, &second_input).unwrap();
        let second_query = ConditionalIntervalBulkQueryV1 {
            schema_version: 1,
            visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
            band: selected,
            interval_cell_step_input: second_input,
            interval_cell_step_event: second_event,
        };
        let second_transfer =
            compile_conditional_interval_bulk_transfer(&profile, &second_query).unwrap();
        steps.push(OpticalLineageStepEvidenceV1 {
            bulk_query: second_query,
            bulk_transfer: second_transfer,
            interface_input: None,
            interface_event: None,
        });
    }
    OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: selected,
        steps,
    }
}

fn cumulative_from_bundle(bundle: OpticalLineageBundleInputV1) -> CumulativeOpticalLaneTransferV1 {
    let manifest = compile_optical_lane_manifest(&bundle).unwrap();
    compile_cumulative_optical_lane_transfer(&CumulativeOpticalLaneTransferInputV1 {
        schema_version: 1,
        bundle,
        manifest,
    })
    .unwrap()
}

#[test]
fn followed_interface_factor_is_selected_but_terminal_interfaces_inject_none() {
    let followed =
        cumulative_from_bundle(interface_bundle(VisibleRadianceBandV1::Red, false, true));
    assert_eq!(followed.factors.len(), 2);
    assert_eq!(
        followed.factors[1].role,
        CumulativeLaneFactorRoleV1::TransmittedInterface
    );
    assert_eq!(followed.factors[1].band, VisibleRadianceBandV1::Red);

    for (band, unsupported, terminal) in [
        (
            VisibleRadianceBandV1::Green,
            false,
            OpticalLineageTerminalV1::AllTir,
        ),
        (
            VisibleRadianceBandV1::Blue,
            false,
            OpticalLineageTerminalV1::AmbiguousInterfaceBranch,
        ),
        (
            VisibleRadianceBandV1::Red,
            true,
            OpticalLineageTerminalV1::UnsupportedInterfaceModel,
        ),
    ] {
        let result = cumulative_from_bundle(interface_bundle(band, unsupported, false));
        assert_eq!(result.final_terminal, terminal);
        assert_eq!(result.factors.len(), 1);
        assert_eq!(
            result.factors[0].role,
            CumulativeLaneFactorRoleV1::BulkTransfer
        );
    }
}
