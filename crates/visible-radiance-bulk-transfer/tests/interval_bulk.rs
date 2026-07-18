use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, CellIndex3V1, ConditionalIntervalCellStepInputV1,
    ConditionalIntervalEvidenceKindV1, CoordinateFrameV1, Id, PhysicalVolumeRecipeInputV1,
    SignedDecimalIntervalV1, compile_conditional_interval_cell_step, compile_physical_volume,
    compile_physical_volume_recipe,
};
use visible_radiance_bulk_transfer::*;

const ONE_Q32: i64 = 1_i64 << 32;
const ONE_Q62: &str = "4611686018427387904";
const HALF_Q160: &str = "730750818665451459101842416358141509827966271488";

fn id(byte: u8) -> Id {
    [byte; 32]
}

fn decimal_interval(bits: u16, lower: &str, upper: &str) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: bits,
        lower: lower.into(),
        upper: upper.into(),
    }
}

fn recipe(
    default_evidence: CellEvidenceV1,
    extent: [u32; 3],
    column_runs: Vec<physical_path_substrate::ColumnRunV1>,
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

fn finite(value: u64) -> BulkBandInteractionV1 {
    BulkBandInteractionV1::Finite {
        extinction_q16_48_per_coordinate_unit: value,
    }
}

fn profile(
    volume: PhysicalVolumeRecipeInputV1,
    substance: Option<Id>,
) -> VisibleRadianceBulkProfileV1 {
    let substance_interactions = substance
        .map(|substance_source_id| {
            vec![SubstanceBulkInteractionV1 {
                substance_source_id,
                bands_rgb: [
                    finite(1_u64 << 47),
                    finite(1_u64 << 48),
                    BulkBandInteractionV1::Opaque,
                ],
            }]
        })
        .unwrap_or_default();
    compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: id(11),
        reconstruction_id: volume.reconstruction_id,
        profile_revision: 1,
        physical_volume_recipe_input: volume,
        substance_interactions,
    })
    .unwrap()
}

fn query(
    profile: &VisibleRadianceBulkProfileV1,
    band: VisibleRadianceBandV1,
    direction: [(&str, &str); 3],
) -> ConditionalIntervalBulkQueryV1 {
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let input = ConditionalIntervalCellStepInputV1 {
        schema_version: 1,
        state_source_id: id(20),
        scope_id: recipe.input.scope_id,
        reconstruction_id: recipe.input.reconstruction_id,
        state_revision: 1,
        evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        point_q160: [
            decimal_interval(160, HALF_Q160, HALF_Q160),
            decimal_interval(160, HALF_Q160, HALF_Q160),
            decimal_interval(160, HALF_Q160, HALF_Q160),
        ],
        direction_q1_62: direction.map(|(lower, upper)| decimal_interval(62, lower, upper)),
    };
    let event = compile_conditional_interval_cell_step(&recipe, &volume, &input).unwrap();
    ConditionalIntervalBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        band,
        interval_cell_step_input: input,
        interval_cell_step_event: event,
    }
}

fn exact_forward_query(
    profile: &VisibleRadianceBulkProfileV1,
    band: VisibleRadianceBandV1,
) -> ConditionalIntervalBulkQueryV1 {
    query(profile, band, [(ONE_Q62, ONE_Q62), ("0", "0"), ("0", "0")])
}

fn known(
    transfer: &ConditionalIntervalBulkTransferV1,
) -> (
    &IntervalBulkLengthCertificateV1,
    &BandTransferV1,
    IntervalBulkTerminalV1,
) {
    match &transfer.outcome {
        ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
            length_certificate,
            band_transfer,
            terminal,
        } => (length_certificate, band_transfer, *terminal),
        other => panic!("expected known transfer, got {other:?}"),
    }
}

#[test]
fn one_band_exact_length_three_interactions_and_replay_are_directed() {
    let substance = id(30);
    let profile = profile(
        recipe(
            CellEvidenceV1::Gas {
                substance_source_id: substance,
            },
            [2, 1, 1],
            vec![],
        ),
        Some(substance),
    );
    let mut optical_depths = Vec::new();
    for band in [
        VisibleRadianceBandV1::Red,
        VisibleRadianceBandV1::Green,
        VisibleRadianceBandV1::Blue,
    ] {
        let query = exact_forward_query(&profile, band);
        let transfer = compile_conditional_interval_bulk_transfer(&profile, &query).unwrap();
        let (length, band_transfer, terminal) = known(&transfer);
        assert_eq!(length.speed_time_q160.lower, HALF_Q160);
        assert_eq!(length.speed_time_q160.upper, HALF_Q160);
        assert_eq!(length.displacement_q160, length.speed_time_q160);
        assert_eq!(length.intersection_q160, length.speed_time_q160);
        assert_eq!(
            terminal,
            IntervalBulkTerminalV1::KnownNeighbor {
                neighbor: CellIndex3V1 { x: 1, y: 0, z: 0 }
            }
        );
        match band_transfer {
            BandTransferV1::Finite {
                optical_depth_lower_q64_64,
                optical_depth_upper_q64_64,
                transmission_lower_q0_48,
                transmission_upper_q0_48,
            } => {
                assert_eq!(optical_depth_lower_q64_64, optical_depth_upper_q64_64);
                assert!(transmission_lower_q0_48 <= transmission_upper_q0_48);
                optical_depths.push(optical_depth_lower_q64_64.to_u128());
            }
            BandTransferV1::Opaque => assert_eq!(band, VisibleRadianceBandV1::Blue),
            BandTransferV1::VacuumIdentity => panic!("gas cannot compile as vacuum"),
        }
        let query_bytes = query.to_bytes(&profile).unwrap();
        assert_eq!(
            ConditionalIntervalBulkQueryV1::from_bytes(&query_bytes, &profile).unwrap(),
            query
        );
        let transfer_bytes = transfer.to_bytes(&profile, &query).unwrap();
        assert_eq!(
            ConditionalIntervalBulkTransferV1::from_bytes(&transfer_bytes, &profile, &query)
                .unwrap(),
            transfer
        );
    }
    assert_eq!(optical_depths, vec![1_u128 << 62, 1_u128 << 63]);
}

#[test]
fn current_cell_transfer_precedes_outer_and_unavailable_neighbor_terminal() {
    let vacuum = profile(recipe(CellEvidenceV1::Vacuum, [1, 1, 1], vec![]), None);
    let outer_query = exact_forward_query(&vacuum, VisibleRadianceBandV1::Red);
    let outer = compile_conditional_interval_bulk_transfer(&vacuum, &outer_query).unwrap();
    let (_, transfer, terminal) = known(&outer);
    assert_eq!(transfer, &BandTransferV1::VacuumIdentity);
    assert_eq!(terminal, IntervalBulkTerminalV1::OuterDomainExit);

    let substance = id(31);
    let unavailable_neighbor = profile(
        recipe(
            CellEvidenceV1::Unavailable,
            [2, 1, 1],
            vec![physical_path_substrate::ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 0,
                length: 1,
                evidence: CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
            }],
        ),
        Some(substance),
    );
    let unavailable_query =
        exact_forward_query(&unavailable_neighbor, VisibleRadianceBandV1::Green);
    let unavailable =
        compile_conditional_interval_bulk_transfer(&unavailable_neighbor, &unavailable_query)
            .unwrap();
    let (_, transfer, terminal) = known(&unavailable);
    assert!(matches!(transfer, BandTransferV1::Finite { .. }));
    assert_eq!(
        terminal,
        IntervalBulkTerminalV1::UnavailableNeighbor {
            neighbor: CellIndex3V1 { x: 1, y: 0, z: 0 }
        }
    );
}

#[test]
fn unavailable_current_ambiguity_no_progress_and_wide_direction_stay_typed() {
    let unavailable = profile(recipe(CellEvidenceV1::Unavailable, [2, 1, 1], vec![]), None);
    let unavailable_query = exact_forward_query(&unavailable, VisibleRadianceBandV1::Red);
    assert!(matches!(
        compile_conditional_interval_bulk_transfer(&unavailable, &unavailable_query)
            .unwrap()
            .outcome,
        ConditionalIntervalBulkOutcomeV1::UnavailableCurrentCell
    ));

    let vacuum = profile(recipe(CellEvidenceV1::Vacuum, [2, 2, 1], vec![]), None);
    let tie = query(
        &vacuum,
        VisibleRadianceBandV1::Red,
        [(ONE_Q62, ONE_Q62), (ONE_Q62, ONE_Q62), ("0", "0")],
    );
    assert!(matches!(
        compile_conditional_interval_bulk_transfer(&vacuum, &tie)
            .unwrap()
            .outcome,
        ConditionalIntervalBulkOutcomeV1::UpstreamAmbiguousNextFace
    ));
    let stationary = query(
        &vacuum,
        VisibleRadianceBandV1::Red,
        [("0", "0"), ("0", "0"), ("0", "0")],
    );
    assert!(matches!(
        compile_conditional_interval_bulk_transfer(&vacuum, &stationary)
            .unwrap()
            .outcome,
        ConditionalIntervalBulkOutcomeV1::UpstreamNoForwardProgress
    ));

    let wide = query(
        &vacuum,
        VisibleRadianceBandV1::Red,
        [(ONE_Q62, ONE_Q62), ("-1", "1"), ("0", "0")],
    );
    let wide_transfer = compile_conditional_interval_bulk_transfer(&vacuum, &wide).unwrap();
    let (length, _, _) = known(&wide_transfer);
    assert!(length.intersection_q160.lower <= length.intersection_q160.upper);
    assert!(
        wide_transfer
            .arithmetic_receipt
            .observed_maximum_magnitude_bits
            <= INTERVAL_BULK_DERIVED_MAXIMUM_MAGNITUDE_BITS
    );
}

#[test]
fn hostile_codecs_caps_and_forged_nested_evidence_fail_closed() {
    let vacuum = profile(recipe(CellEvidenceV1::Vacuum, [2, 1, 1], vec![]), None);
    let query = exact_forward_query(&vacuum, VisibleRadianceBandV1::Red);
    let transfer = compile_conditional_interval_bulk_transfer(&vacuum, &query).unwrap();

    let mut query_bytes = query.to_bytes(&vacuum).unwrap();
    query_bytes.push(b' ');
    assert!(ConditionalIntervalBulkQueryV1::from_bytes(&query_bytes, &vacuum).is_err());
    assert!(
        ConditionalIntervalBulkQueryV1::from_bytes(
            &vec![b' '; MAX_INTERVAL_BULK_QUERY_BYTES + 1],
            &vacuum
        )
        .is_err()
    );
    let mut unknown: serde_json::Value =
        serde_json::from_slice(&query.to_bytes(&vacuum).unwrap()).unwrap();
    unknown["unexpected"] = serde_json::Value::Bool(true);
    assert!(
        ConditionalIntervalBulkQueryV1::from_bytes(&serde_json::to_vec(&unknown).unwrap(), &vacuum)
            .is_err()
    );

    let mut forged_query = query.clone();
    forged_query
        .interval_cell_step_event
        .interval_cell_step_event_id = id(99);
    assert!(compile_conditional_interval_bulk_transfer(&vacuum, &forged_query).is_err());
    let mut forged_transfer = transfer.clone();
    forged_transfer.arithmetic_receipt.projection_ceiling = 2;
    assert!(forged_transfer.to_bytes(&vacuum, &query).is_err());
    assert!(
        ConditionalIntervalBulkTransferV1::from_bytes(
            &vec![b' '; MAX_INTERVAL_BULK_TRANSFER_BYTES + 1],
            &vacuum,
            &query
        )
        .is_err()
    );
}
