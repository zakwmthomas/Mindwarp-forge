use optical_lineage_binding::*;
use physical_path_substrate::*;
use receiver_arrival_geometry_binding::*;
use visible_radiance_bulk_transfer::*;

const ONE_Q32: i64 = 1_i64 << 32;
const ONE_Q62: i64 = 1_i64 << 62;
const ZERO: &str = "0";
const QUARTER: &str = "365375409332725729550921208179070754913983135744";
const HALF: &str = "730750818665451459101842416358141509827966271488";
const THREE_QUARTER: &str = "1096126227998177188652763624537212264741949407232";
const SEVEN_EIGHTHS: &str = "1278813932664540053428224228626747642198940975104";
const ONE: &str = "1461501637330902918203684832716283019655932542976";

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
fn profile_extent(extent: [u32; 3]) -> VisibleRadianceBulkProfileV1 {
    let recipe = PhysicalVolumeRecipeInputV1 {
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
        default_evidence: CellEvidenceV1::Vacuum,
        column_runs: vec![],
    };
    compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: id(2),
        reconstruction_id: id(3),
        profile_revision: 1,
        physical_volume_recipe_input: recipe,
        substance_interactions: vec![],
    })
    .unwrap()
}
fn profile() -> VisibleRadianceBulkProfileV1 {
    profile_extent([1; 3])
}
fn input(
    point: [(&str, &str); 3],
    direction: [(i64, i64); 3],
    minimum: [&str; 3],
    maximum: [&str; 3],
) -> ReceiverArrivalGeometryInputV1 {
    let profile = profile();
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
        point_q160: point.map(|(a, b)| interval(160, a, b)),
        direction_q1_62: direction.map(|(a, b)| interval(62, a, b)),
    };
    let event = compile_conditional_interval_cell_step(&recipe, &volume, &cell_input).unwrap();
    let query = ConditionalIntervalBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        band: VisibleRadianceBandV1::Red,
        interval_cell_step_input: cell_input,
        interval_cell_step_event: event,
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
    let receiver = ReceiverAabbV1::compile(
        id(40),
        id(2),
        id(3),
        1,
        minimum.map(str::to_owned),
        maximum.map(str::to_owned),
    )
    .unwrap();
    ReceiverArrivalGeometryInputV1 {
        schema_version: 1,
        bundle,
        manifest,
        receiver,
    }
}
fn ray(minimum: [&str; 3], maximum: [&str; 3]) -> ReceiverArrivalGeometryInputV1 {
    input(
        [(HALF, HALF), (HALF, HALF), (HALF, HALF)],
        [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)],
        minimum,
        maximum,
    )
}

fn two_step_face_tie_input() -> ReceiverArrivalGeometryInputV1 {
    let profile = profile_extent([2, 1, 1]);
    let recipe =
        compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let make = |source, revision, cell, point: [SignedDecimalIntervalV1; 3]| {
        let cell_input = ConditionalIntervalCellStepInputV1 {
            schema_version: 1,
            state_source_id: source,
            scope_id: id(2),
            reconstruction_id: id(3),
            state_revision: revision,
            evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
            physical_volume_recipe_id: recipe.physical_volume_recipe_id,
            physical_volume_id: volume.physical_volume_id,
            current_cell: cell,
            point_q160: point,
            direction_q1_62: [
                interval(62, ONE_Q62, ONE_Q62),
                interval(62, 0, 0),
                interval(62, 0, 0),
            ],
        };
        let event = compile_conditional_interval_cell_step(&recipe, &volume, &cell_input).unwrap();
        ConditionalIntervalBulkQueryV1 {
            schema_version: 1,
            visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
            band: VisibleRadianceBandV1::Red,
            interval_cell_step_input: cell_input,
            interval_cell_step_event: event,
        }
    };
    let first = make(
        id(20),
        1,
        CellIndex3V1 { x: 0, y: 0, z: 0 },
        std::array::from_fn(|_| interval(160, HALF, HALF)),
    );
    let first_transfer = compile_conditional_interval_bulk_transfer(&profile, &first).unwrap();
    let lane_id = derive_optical_lane_id(
        id(3),
        profile.visible_radiance_bulk_profile_id,
        VisibleRadianceBandV1::Red,
        first_transfer.interval_cell_step_input_id,
        id(20),
    )
    .unwrap();
    let first_step_id = derive_optical_lineage_step_id(
        lane_id,
        0,
        None,
        first_transfer.interval_cell_step_input_id,
        first_transfer.interval_cell_step_event_id,
        first_transfer.conditional_interval_bulk_query_id,
        first_transfer.conditional_interval_bulk_transfer_id,
        None,
        OpticalLineageDispositionV1::ContinueSameMedium,
    )
    .unwrap();
    let certified = match &first.interval_cell_step_event.outcome {
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } => certified,
        _ => panic!("first face"),
    };
    let source = derive_optical_lineage_source_id(
        lane_id,
        1,
        Some(first_step_id),
        OpticalLineageDerivedSourceRoleV1::CellInput,
    )
    .unwrap();
    let second = make(
        source,
        2,
        certified.neighbor.unwrap(),
        certified.point_q160.clone(),
    );
    let second_transfer = compile_conditional_interval_bulk_transfer(&profile, &second).unwrap();
    let bundle = OpticalLineageBundleInputV1 {
        schema_version: 1,
        lane_source_id: id(20),
        profile,
        band: VisibleRadianceBandV1::Red,
        steps: vec![
            OpticalLineageStepEvidenceV1 {
                bulk_query: first,
                bulk_transfer: first_transfer,
                interface_input: None,
                interface_event: None,
            },
            OpticalLineageStepEvidenceV1 {
                bulk_query: second,
                bulk_transfer: second_transfer,
                interface_input: None,
                interface_event: None,
            },
        ],
    };
    let manifest = compile_optical_lane_manifest(&bundle).unwrap();
    let receiver = ReceiverAabbV1::compile(
        id(40),
        id(2),
        id(3),
        1,
        [ONE.to_owned(), QUARTER.to_owned(), QUARTER.to_owned()],
        [
            "1826877046663628647754606040895353774569915678720".to_owned(),
            THREE_QUARTER.to_owned(),
            THREE_QUARTER.to_owned(),
        ],
    )
    .unwrap();
    ReceiverArrivalGeometryInputV1 {
        schema_version: 1,
        bundle,
        manifest,
        receiver,
    }
}

#[test]
fn strict_entry_start_inside_no_arrival_and_contact_are_distinct() {
    let strict = ray(
        [THREE_QUARTER, QUARTER, QUARTER],
        [SEVEN_EIGHTHS, THREE_QUARTER, THREE_QUARTER],
    );
    let result = compile_receiver_arrival_geometry(&strict).unwrap();
    assert!(matches!(
        result.outcome,
        ReceiverArrivalOutcomeV1::CertifiedStrictInteriorArrival {
            step_ordinal: 0,
            ..
        }
    ));
    assert!(result.contacts.is_empty());
    let start = ray(
        [QUARTER, QUARTER, QUARTER],
        [THREE_QUARTER, THREE_QUARTER, THREE_QUARTER],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&start).unwrap().outcome,
        ReceiverArrivalOutcomeV1::ArrivalAtStart { step_ordinal: 0 }
    ));
    let behind = ray(
        [ZERO, QUARTER, QUARTER],
        [QUARTER, THREE_QUARTER, THREE_QUARTER],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&behind).unwrap().outcome,
        ReceiverArrivalOutcomeV1::NoArrivalBeforeLineageTerminal {
            terminal: OpticalLineageTerminalV1::OuterDomainExit
        }
    ));
    let tangent = ray(
        [THREE_QUARTER, HALF, QUARTER],
        [SEVEN_EIGHTHS, THREE_QUARTER, THREE_QUARTER],
    );
    let contact = compile_receiver_arrival_geometry(&tangent).unwrap();
    assert!(matches!(
        contact.outcome,
        ReceiverArrivalOutcomeV1::NoArrivalBeforeLineageTerminal { .. }
    ));
    assert_eq!(contact.contacts.len(), 1);
}

#[test]
fn conditional_evidence_is_typed_unsupported_and_not_sampled() {
    let conditional = input(
        [(HALF, HALF), (HALF, HALF), (HALF, HALF)],
        [(ONE_Q62 / 2, ONE_Q62), (0, 0), (0, 0)],
        [THREE_QUARTER, QUARTER, QUARTER],
        [SEVEN_EIGHTHS, THREE_QUARTER, THREE_QUARTER],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&conditional)
            .unwrap()
            .outcome,
        ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
            first_unsupported_ordinal: 0
        }
    ));

    let conditional_point = input(
        [(QUARTER, HALF), (HALF, HALF), (HALF, HALF)],
        [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)],
        [THREE_QUARTER, QUARTER, QUARTER],
        [SEVEN_EIGHTHS, THREE_QUARTER, THREE_QUARTER],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&conditional_point)
            .unwrap()
            .outcome,
        ReceiverArrivalOutcomeV1::UnsupportedConditionalEvidence {
            first_unsupported_ordinal: 0
        }
    ));
}

#[test]
fn reverse_direction_and_parallel_outside_are_ordered_exactly() {
    let reverse = input(
        [(HALF, HALF), (HALF, HALF), (HALF, HALF)],
        [(-ONE_Q62, -ONE_Q62), (0, 0), (0, 0)],
        [QUARTER, QUARTER, QUARTER],
        [
            "548063113999088594326381812268606132370974703616",
            THREE_QUARTER,
            THREE_QUARTER,
        ],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&reverse).unwrap().outcome,
        ReceiverArrivalOutcomeV1::CertifiedStrictInteriorArrival {
            step_ordinal: 0,
            ..
        }
    ));

    let parallel_outside = input(
        [(HALF, HALF), (HALF, HALF), (HALF, HALF)],
        [(ONE_Q62, ONE_Q62), (0, 0), (0, 0)],
        [THREE_QUARTER, ZERO, QUARTER],
        [SEVEN_EIGHTHS, QUARTER, THREE_QUARTER],
    );
    assert!(matches!(
        compile_receiver_arrival_geometry(&parallel_outside)
            .unwrap()
            .outcome,
        ReceiverArrivalOutcomeV1::NoArrivalBeforeLineageTerminal { .. }
    ));
}

#[test]
fn receiver_face_tie_is_contact_then_successor_owned_arrival() {
    let input = two_step_face_tie_input();
    let result = compile_receiver_arrival_geometry(&input).unwrap();
    assert_eq!(result.contacts.len(), 1);
    assert_eq!(result.contacts[0].step_ordinal, 0);
    assert!(matches!(
        result.outcome,
        ReceiverArrivalOutcomeV1::CertifiedStrictInteriorArrival {
            step_ordinal: 1,
            ..
        }
    ));
}

#[test]
fn receiver_identity_bounds_codecs_and_replay_fail_closed() {
    let value = ray(
        [THREE_QUARTER, QUARTER, QUARTER],
        [SEVEN_EIGHTHS, THREE_QUARTER, THREE_QUARTER],
    );
    let result = compile_receiver_arrival_geometry(&value).unwrap();
    let input_bytes = value.to_bytes().unwrap();
    assert_eq!(
        ReceiverArrivalGeometryInputV1::from_bytes(&input_bytes).unwrap(),
        value
    );
    let output_bytes = result.to_bytes(&value).unwrap();
    assert_eq!(
        ReceiverArrivalGeometryV1::from_bytes(&output_bytes, &value).unwrap(),
        result
    );
    let mut forged = value.clone();
    forged.receiver.receiver_id[0] ^= 1;
    assert!(compile_receiver_arrival_geometry(&forged).is_err());
    let point = ReceiverAabbV1::compile(
        id(40),
        id(2),
        id(3),
        1,
        [HALF.to_owned(), QUARTER.to_owned(), QUARTER.to_owned()],
        [
            HALF.to_owned(),
            THREE_QUARTER.to_owned(),
            THREE_QUARTER.to_owned(),
        ],
    )
    .unwrap();
    let mut point_input = value.clone();
    point_input.receiver = point;
    assert!(compile_receiver_arrival_geometry(&point_input).is_err());
    let outside = ReceiverAabbV1::compile(
        id(40),
        id(2),
        id(3),
        1,
        [
            THREE_QUARTER.to_owned(),
            QUARTER.to_owned(),
            QUARTER.to_owned(),
        ],
        [
            "1461501637330902918203684832716283019655932542977".to_owned(),
            THREE_QUARTER.to_owned(),
            THREE_QUARTER.to_owned(),
        ],
    )
    .unwrap();
    let mut outside_input = value.clone();
    outside_input.receiver = outside;
    assert!(compile_receiver_arrival_geometry(&outside_input).is_err());
    let mut trailing = input_bytes.clone();
    trailing.push(b' ');
    assert!(ReceiverArrivalGeometryInputV1::from_bytes(&trailing).is_err());
    let mut drift = result.clone();
    drift.authority_effect = "runtime".into();
    assert!(drift.to_bytes(&value).is_err());
    assert_eq!(ONE, "1461501637330902918203684832716283019655932542976");
}
