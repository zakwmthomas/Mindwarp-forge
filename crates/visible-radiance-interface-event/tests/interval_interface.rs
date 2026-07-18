use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, CellIndex3V1, ColumnRunV1, CoordinateFrameV1, Id,
    PhysicalVolumeRecipeInputV1, compile_physical_volume, compile_physical_volume_recipe,
};
use visible_radiance_interface_event::{
    DIRECTION_SCALE_BITS, DecimalIntervalV1, FaceInteractionEvidenceV1, FixedScaleV1,
    InterfaceModelV1, IntervalBandOutcomeV1, IntervalEvidenceKindV1, IntervalInterfaceOutcomeV1,
    IntervalUniformBranchV1, MAX_INTERVAL_EVENT_BYTES, MAX_INTERVAL_INPUT_BYTES,
    SmoothDielectricBandV1, VisibleRadianceIntervalInterfaceEventV1,
    VisibleRadianceIntervalInterfaceInputV1, compile_visible_radiance_interval_interface_event,
};

const ONE_Q32: i64 = 1_i64 << 32;
const ONE_Q62: i64 = 1_i64 << DIRECTION_SCALE_BITS;
const ONE_Q48: u64 = 1_u64 << 48;

fn id(value: u32) -> Id {
    let mut id = [0; 32];
    id[..4].copy_from_slice(&value.to_le_bytes());
    id[31] = 1;
    id
}

fn direction(lower: i64, upper: i64) -> DecimalIntervalV1 {
    DecimalIntervalV1 {
        lower: lower.to_string(),
        upper: upper.to_string(),
        scale: FixedScaleV1::Q1_62,
    }
}

fn setup(
    bands_rgb: [SmoothDielectricBandV1; 3],
    incident_direction_xyz: [DecimalIntervalV1; 3],
) -> (
    physical_path_substrate::PhysicalVolumeRecipeV1,
    physical_path_substrate::PhysicalVolumeV1,
    VisibleRadianceIntervalInterfaceInputV1,
) {
    let gas = CellEvidenceV1::Gas {
        substance_source_id: id(10),
    };
    let liquid = CellEvidenceV1::Liquid {
        substance_source_id: id(11),
    };
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
        default_evidence: gas.clone(),
        column_runs: vec![ColumnRunV1 {
            x_index: 1,
            y_index: 0,
            z_start: 0,
            length: 1,
            evidence: liquid.clone(),
        }],
    };
    let recipe = compile_physical_volume_recipe(&recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let input = VisibleRadianceIntervalInterfaceInputV1 {
        schema_version: 1,
        incident_source_id: id(4),
        scope_id: id(2),
        reconstruction_id: id(3),
        incident_revision: 1,
        evidence_kind: IntervalEvidenceKindV1::DeclaredConditionalDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        source_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        target_cell: CellIndex3V1 { x: 1, y: 0, z: 0 },
        face_interaction: FaceInteractionEvidenceV1 {
            interaction_source_id: id(5),
            scope_id: id(2),
            reconstruction_id: id(3),
            interaction_revision: 1,
            cell_a: CellIndex3V1 { x: 0, y: 0, z: 0 },
            cell_b: CellIndex3V1 { x: 1, y: 0, z: 0 },
            medium_a: gas,
            medium_b: liquid,
            model: InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb },
        },
        incident_direction_xyz,
    };
    (recipe, volume, input)
}

fn equal_bands(a: u64, b: u64) -> [SmoothDielectricBandV1; 3] {
    std::array::from_fn(|_| SmoothDielectricBandV1 {
        eta_a_q16_48: a,
        eta_b_q16_48: b,
    })
}

#[test]
fn point_box_is_bounded_strict_and_replayable() {
    let (recipe, volume, input) = setup(
        equal_bands(ONE_Q48, 3 * ONE_Q48 / 2),
        [
            direction(ONE_Q62, ONE_Q62),
            direction(0, 0),
            direction(0, 0),
        ],
    );
    let event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).unwrap();
    let IntervalInterfaceOutcomeV1::Evaluated {
        bands_rgb,
        arithmetic_receipt,
    } = &event.outcome
    else {
        panic!("expected evaluated")
    };
    assert_eq!(arithmetic_receipt.evaluated_band_count, 3);
    assert_eq!(arithmetic_receipt.fractional_bit_work_units, 480);
    assert!(bands_rgb.iter().all(|band| matches!(
        band,
        IntervalBandOutcomeV1::BoundedEnclosure {
            branch: IntervalUniformBranchV1::AllTransmit,
            ..
        }
    )));
    let input_bytes = input.to_bytes(&recipe, &volume).unwrap();
    let event_bytes = event.to_bytes(&recipe, &volume, &input).unwrap();
    assert!(input_bytes.len() < MAX_INTERVAL_INPUT_BYTES);
    assert!(event_bytes.len() < MAX_INTERVAL_EVENT_BYTES);
    eprintln!(
        "interval_interface_cost input_bytes={} event_bytes={} input_struct_bytes={} event_struct_bytes={}",
        input_bytes.len(),
        event_bytes.len(),
        std::mem::size_of_val(&input),
        std::mem::size_of_val(&event)
    );
    assert_eq!(
        VisibleRadianceIntervalInterfaceInputV1::from_bytes(&recipe, &volume, &input_bytes)
            .unwrap(),
        input
    );
    assert_eq!(
        VisibleRadianceIntervalInterfaceEventV1::from_bytes(&recipe, &volume, &input, &event_bytes)
            .unwrap(),
        event
    );
}

#[test]
fn rgb_branches_are_independent_and_aggregate_masking_is_impossible() {
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
    let (recipe, volume, input) = setup(
        bands,
        [
            direction(ONE_Q62 / 2, ONE_Q62),
            direction(ONE_Q62 / 5 * 3, ONE_Q62 / 4 * 3),
            direction(0, 0),
        ],
    );
    let event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).unwrap();
    let IntervalInterfaceOutcomeV1::Evaluated {
        bands_rgb,
        arithmetic_receipt,
    } = event.outcome
    else {
        panic!()
    };
    assert!(matches!(
        bands_rgb[0],
        IntervalBandOutcomeV1::BoundedEnclosure {
            branch: IntervalUniformBranchV1::AllTransmit,
            ..
        }
    ));
    assert!(matches!(
        bands_rgb[1],
        IntervalBandOutcomeV1::BoundedEnclosure {
            branch: IntervalUniformBranchV1::AllTir,
            ..
        }
    ));
    assert_eq!(
        bands_rgb[2],
        IntervalBandOutcomeV1::AmbiguousInterfaceBranch
    );
    assert_eq!(arithmetic_receipt.evaluated_band_count, 2);
    assert_eq!(arithmetic_receipt.fractional_bit_work_units, 320);
}

#[test]
fn invalid_boxes_fail_before_arithmetic() {
    let valid_bands = equal_bands(ONE_Q48, 3 * ONE_Q48 / 2);
    let invalid = [
        [direction(1, 0), direction(0, 0), direction(0, 0)],
        [direction(0, 1), direction(0, 1), direction(0, 1)],
        [
            direction(ONE_Q62 / 8, ONE_Q62 / 4),
            direction(0, 0),
            direction(0, 0),
        ],
        [
            direction(-ONE_Q62, -ONE_Q62 / 2),
            direction(0, 0),
            direction(0, 0),
        ],
    ];
    for direction_box in invalid {
        let (recipe, volume, input) = setup(valid_bands.clone(), direction_box);
        assert!(
            compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).is_err()
        );
    }
}

#[test]
fn byte_caps_and_noncanonical_replay_fail_closed() {
    let (recipe, volume, input) = setup(
        equal_bands(ONE_Q48, ONE_Q48),
        [
            direction(ONE_Q62, ONE_Q62),
            direction(0, 0),
            direction(0, 0),
        ],
    );
    let event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).unwrap();
    let mut input_bytes = input.to_bytes(&recipe, &volume).unwrap();
    input_bytes.push(b' ');
    assert!(
        VisibleRadianceIntervalInterfaceInputV1::from_bytes(&recipe, &volume, &input_bytes)
            .is_err()
    );
    assert!(
        VisibleRadianceIntervalInterfaceInputV1::from_bytes(
            &recipe,
            &volume,
            &vec![b' '; MAX_INTERVAL_INPUT_BYTES + 1]
        )
        .is_err()
    );
    assert!(
        VisibleRadianceIntervalInterfaceEventV1::from_bytes(
            &recipe,
            &volume,
            &input,
            &vec![b' '; MAX_INTERVAL_EVENT_BYTES + 1]
        )
        .is_err()
    );
    let mut forged = event;
    forged.authority_effect = "owner".into();
    assert!(forged.to_bytes(&recipe, &volume, &input).is_err());
}

#[test]
fn reverse_orientation_is_world_signed_and_identity_distinct() {
    let (recipe, volume, forward) = setup(
        equal_bands(ONE_Q48, ONE_Q48),
        [
            direction(ONE_Q62, ONE_Q62),
            direction(0, 0),
            direction(0, 0),
        ],
    );
    let forward_event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &forward).unwrap();
    let mut reverse = forward.clone();
    std::mem::swap(&mut reverse.source_cell, &mut reverse.target_cell);
    reverse.incident_direction_xyz[0] = direction(-ONE_Q62, -ONE_Q62);
    let reverse_event =
        compile_visible_radiance_interval_interface_event(&recipe, &volume, &reverse).unwrap();
    assert_ne!(
        forward_event.interval_interface_input_id,
        reverse_event.interval_interface_input_id
    );
    let IntervalInterfaceOutcomeV1::Evaluated { bands_rgb, .. } = reverse_event.outcome else {
        panic!()
    };
    let IntervalBandOutcomeV1::BoundedEnclosure { event, .. } = &bands_rgb[0] else {
        panic!()
    };
    let transmitted = event.transmitted_direction_xyz.as_ref().unwrap();
    assert_eq!(transmitted[0].lower, (-ONE_Q62).to_string());
    assert_eq!(transmitted[0].upper, (-ONE_Q62).to_string());
}

#[test]
fn deterministic_hostile_box_portfolio_is_finite_and_replayable() {
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
    let mut state = 0x49_4e_54_45_52_56_41_4c_u64;
    let mut checksum = [0_u8; 32];
    for index in 0..128_u64 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let normal_lower = 1 + (state % (ONE_Q62 as u64 - 1)) as i64;
        let tangent = (state.rotate_left(17) % (ONE_Q62 as u64 + 1)) as i64;
        let (recipe, volume, input) = setup(
            bands.clone(),
            [
                direction(normal_lower, ONE_Q62),
                direction(-tangent, tangent),
                direction(0, 0),
            ],
        );
        let event =
            compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).unwrap();
        let bytes = event.to_bytes(&recipe, &volume, &input).unwrap();
        for (offset, value) in event.event_id.iter().enumerate() {
            checksum[(offset + index as usize) % 32] ^= value;
        }
        assert!(bytes.len() < MAX_INTERVAL_EVENT_BYTES);
    }
    assert_ne!(checksum, [0; 32]);
}

#[test]
fn scale_identity_and_volume_poison_fail_closed() {
    let (recipe, volume, mut input) = setup(
        equal_bands(ONE_Q48, ONE_Q48),
        [
            direction(ONE_Q62, ONE_Q62),
            direction(0, 0),
            direction(0, 0),
        ],
    );
    input.incident_direction_xyz[0].scale = FixedScaleV1::Q0_48;
    assert!(compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).is_err());
    input.incident_direction_xyz[0].scale = FixedScaleV1::Q1_62;
    input.incident_direction_xyz[0].lower = "+4611686018427387904".into();
    assert!(compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).is_err());
    input.incident_direction_xyz[0].lower = ONE_Q62.to_string();
    input.physical_volume_id = id(999);
    assert!(compile_visible_radiance_interval_interface_event(&recipe, &volume, &input).is_err());
}
