use fixed_interval_arithmetic::Signed512;
use optical_phase_space_cell_binding::{
    CorrelatedAffineOutputV1, OpticalPhaseSpaceRootInputV1, PhaseSpaceOutputRoleV1,
    PhaseSpaceParameterizationV1, PositiveRationalV1, compile_optical_phase_space_root,
};
use optical_phase_space_receiver_coupling::{
    MAXIMUM_LIVE_BITS, WholeCellReceiverCouplingError, WholeCellReceiverCouplingInputV1,
    WholeCellReceiverCouplingOutcomeV1, WholeCellReceiverCouplingV1,
    compile_whole_cell_receiver_coupling, validate_whole_cell_receiver_coupling,
};
use optical_phase_space_transport_certificate::{
    OriginAnchoredTransportInputV1, compile_origin_anchored_transport,
};
use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, CellIndex3V1, CoordinateFrameV1,
    PhysicalVolumeRecipeInputV1, compile_physical_volume, compile_physical_volume_recipe,
};
use receiver_arrival_geometry_binding::ReceiverAabbV1;

fn form(role: PhaseSpaceOutputRoleV1, center: u128, small: u128) -> CorrelatedAffineOutputV1 {
    CorrelatedAffineOutputV1 {
        role,
        center_numerator: center.to_string(),
        coefficient_numerators: [small, small - 1, small - 2, small - 3]
            .map(|value| value.to_string()),
        remainder_lower_numerator: "0".into(),
        remainder_upper_numerator: "0".into(),
    }
}

fn transport() -> OriginAnchoredTransportInputV1 {
    let denominator = u64::MAX as u128;
    let small = denominator / 16_384;
    let scope = [2; 32];
    let reconstruction = [3; 32];
    let cell = compile_optical_phase_space_root(&OpticalPhaseSpaceRootInputV1 {
        schema_version: 1,
        source_id: [1; 32],
        scope_id: scope,
        reconstruction_id: reconstruction,
        source_revision: 1,
        parameterization: PhaseSpaceParameterizationV1::TransverseAreaDirection4d,
        measure: PositiveRationalV1 {
            numerator: "1".into(),
            denominator: "1".into(),
        },
        form_denominator: denominator.to_string(),
        forms: [
            form(PhaseSpaceOutputRoleV1::PointX, denominator / 5, small),
            form(PhaseSpaceOutputRoleV1::PointY, denominator / 5, small),
            form(PhaseSpaceOutputRoleV1::PointZ, denominator / 5, small),
            form(
                PhaseSpaceOutputRoleV1::DirectionX,
                denominator * 9 / 10,
                small / 8,
            ),
            form(
                PhaseSpaceOutputRoleV1::DirectionY,
                denominator * 3 / 5,
                small / 8,
            ),
            form(
                PhaseSpaceOutputRoleV1::DirectionZ,
                denominator / 3,
                small / 8,
            ),
        ],
    })
    .expect("cell");
    let recipe = compile_physical_volume_recipe(&PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: [4; 32],
        scope_id: scope,
        reconstruction_id: reconstruction,
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: 1_i64 << 32,
        extent: [4, 4, 4],
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence: CellEvidenceV1::Vacuum,
        column_runs: Vec::new(),
    })
    .expect("recipe");
    let volume = compile_physical_volume(&recipe).expect("volume");
    OriginAnchoredTransportInputV1 {
        schema_version: 1,
        cell,
        physical_volume_recipe: recipe,
        physical_volume: volume,
        current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        band_time_id: [5; 32],
        maximum_steps: 3,
    }
}

fn q160_from_q32(value: i64) -> String {
    Signed512::from_i64(value)
        .checked_shl(128)
        .expect("q160")
        .canonical_decimal()
}

fn receiver(
    input: &OriginAnchoredTransportInputV1,
    min: [i64; 3],
    max: [i64; 3],
) -> ReceiverAabbV1 {
    ReceiverAabbV1::compile(
        [7; 32],
        input.cell.scope_id,
        input.cell.reconstruction_id,
        1,
        min.map(q160_from_q32),
        max.map(q160_from_q32),
    )
    .expect("receiver")
}

fn compile_with(
    min: [i64; 3],
    max: [i64; 3],
    selected_step_index: u8,
) -> (
    WholeCellReceiverCouplingInputV1,
    WholeCellReceiverCouplingV1,
) {
    let transport_input = transport();
    let transport_certificate =
        compile_origin_anchored_transport(&transport_input).expect("transport");
    let receiver = receiver(&transport_input, min, max);
    let input = WholeCellReceiverCouplingInputV1 {
        schema_version: 1,
        transport_input,
        transport_certificate,
        selected_step_index,
        receiver,
    };
    let result = compile_whole_cell_receiver_coupling(&input).expect("coupling");
    (input, result)
}

#[test]
fn full_zero_unresolved_and_codec_are_conservative() {
    let half = 1_i64 << 31;
    let one = 1_i64 << 32;
    let two = 2_i64 << 32;
    let three = 3_i64 << 32;
    let (input, full) = compile_with([half, 0, 0], [one - 1, one, one], 0);
    assert!(matches!(
        full.outcome,
        WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace { .. }
    ));
    assert_eq!(full.accepted_measure, input.transport_input.cell.measure);
    assert_eq!(full.zero_measure.numerator, "0");
    assert_eq!(full.unresolved_measure.numerator, "0");
    assert!(full.arithmetic_receipt.observed_maximum_live_bits <= MAXIMUM_LIVE_BITS);
    assert_eq!(
        full.input_id,
        [
            0x39, 0x91, 0xd3, 0x81, 0xa1, 0x9a, 0xb5, 0x7d, 0x30, 0xc5, 0x68, 0xee, 0x7c, 0x21,
            0xa0, 0x5c, 0xd3, 0xb4, 0xab, 0x03, 0xc7, 0x86, 0x4a, 0xb9, 0xfa, 0x00, 0x72, 0xc4,
            0x8d, 0x15, 0x5d, 0xee,
        ]
    );
    assert_eq!(
        full.result_id,
        [
            0xdd, 0x35, 0x57, 0x16, 0x25, 0x08, 0x91, 0xa4, 0xe4, 0x91, 0x70, 0x94, 0xa1, 0x2a,
            0x2f, 0x72, 0xb9, 0x17, 0xfb, 0x20, 0x3d, 0x02, 0xcb, 0x1e, 0xc6, 0x99, 0x67, 0x35,
            0x2b, 0x64, 0x97, 0xcf,
        ]
    );
    assert_eq!(full.arithmetic_receipt.observed_maximum_live_bits, 320);
    let input_bytes = input.to_bytes().expect("input bytes");
    assert_eq!(
        WholeCellReceiverCouplingInputV1::from_bytes(&input_bytes).expect("input decode"),
        input
    );
    let bytes = full.to_bytes(&input).expect("bytes");
    assert_eq!(
        WholeCellReceiverCouplingV1::from_bytes(&input, &bytes).expect("decode"),
        full
    );

    let (_, zero) = compile_with([two, 0, 0], [three, one, one], 0);
    assert!(matches!(
        zero.outcome,
        WholeCellReceiverCouplingOutcomeV1::CertifiedZeroBeforeFace { .. }
    ));
    assert_eq!(zero.zero_measure.numerator, "1");

    let (_, unresolved) = compile_with([one, 0, 0], [two, one, one], 0);
    assert!(matches!(
        unresolved.outcome,
        WholeCellReceiverCouplingOutcomeV1::UnresolvedReceiverCoupling { .. }
    ));
    assert_eq!(unresolved.unresolved_measure.numerator, "1");
}

#[test]
fn later_segment_nested_forgery_and_codec_fail_closed() {
    let one = 1_i64 << 32;
    let two = 2_i64 << 32;
    let (_, later) = compile_with([0, one * 3 / 4, 0], [two, one - 1, one], 1);
    assert!(matches!(
        later.outcome,
        WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace { .. }
    ));

    let (input, result) = compile_with([one / 2, 0, 0], [one - 1, one, one], 0);
    let mut forged = result.clone();
    forged.authority_effect = "runtime_authorized".into();
    assert_eq!(
        validate_whole_cell_receiver_coupling(&input, &forged),
        Err(WholeCellReceiverCouplingError::IdentityMismatch)
    );
    let mut forged_input = input.clone();
    forged_input.transport_certificate.certificate_id = [9; 32];
    assert_eq!(
        compile_whole_cell_receiver_coupling(&forged_input),
        Err(WholeCellReceiverCouplingError::Dependency(
            "transport certificate replay failed"
        ))
    );
    let mut trailing = input.to_bytes().expect("input bytes");
    trailing.push(b' ');
    assert_eq!(
        WholeCellReceiverCouplingInputV1::from_bytes(&trailing),
        Err(WholeCellReceiverCouplingError::CodecDefect)
    );
}
