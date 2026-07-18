use fixed_interval_arithmetic::Signed512;
use optical_phase_space_cell_binding::{
    CorrelatedAffineOutputV1, OpticalPhaseSpaceRootInputV1, PhaseSpaceOutputRoleV1,
    PhaseSpaceParameterizationV1, PositiveRationalV1, compile_optical_phase_space_root,
};
use optical_phase_space_dimensionless_transfer::{
    OpticalBandTimeBindingV1, WholeCellDimensionlessTransferInputV1,
    WholeCellDimensionlessTransferOutcomeV1, WholeCellDimensionlessTransferV1,
    compile_optical_band_time_binding, compile_whole_cell_dimensionless_transfer,
    validate_whole_cell_dimensionless_transfer,
};
use optical_phase_space_receiver_coupling::{
    WholeCellReceiverCouplingInputV1, compile_whole_cell_receiver_coupling,
};
use optical_phase_space_transport_certificate::{
    OriginAnchoredTransportInputV1, compile_origin_anchored_transport,
};
use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, CellIndex3V1, CoordinateFrameV1,
    PhysicalVolumeRecipeInputV1, compile_physical_volume, compile_physical_volume_recipe,
};
use receiver_arrival_geometry_binding::ReceiverAabbV1;
use visible_radiance_bulk_transfer::{
    BulkBandInteractionV1, SubstanceBulkInteractionV1, VisibleRadianceBandV1,
    VisibleRadianceBulkProfileInputV1, VisibleRadianceBulkProfileV1,
    compile_visible_radiance_bulk_profile,
};

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

fn transport(band_time_id: [u8; 32], evidence: CellEvidenceV1) -> OriginAnchoredTransportInputV1 {
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
        default_evidence: evidence,
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
        band_time_id,
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

fn profile(
    transport: &OriginAnchoredTransportInputV1,
    substance: Option<[u8; 32]>,
) -> VisibleRadianceBulkProfileV1 {
    let substance_interactions = substance
        .map(|substance_source_id| {
            vec![SubstanceBulkInteractionV1 {
                substance_source_id,
                bands_rgb: [
                    BulkBandInteractionV1::Finite {
                        extinction_q16_48_per_coordinate_unit: 1_u64 << 48,
                    },
                    BulkBandInteractionV1::Finite {
                        extinction_q16_48_per_coordinate_unit: 2_u64 << 48,
                    },
                    BulkBandInteractionV1::Opaque,
                ],
            }]
        })
        .unwrap_or_default();
    compile_visible_radiance_bulk_profile(&VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: [8; 32],
        scope_id: [9; 32],
        reconstruction_id: transport.physical_volume_recipe.input.reconstruction_id,
        profile_revision: 1,
        physical_volume_recipe_input: transport.physical_volume_recipe.input.clone(),
        substance_interactions,
    })
    .expect("profile")
}

fn fixture(
    band: VisibleRadianceBandV1,
    evidence: CellEvidenceV1,
    substance: Option<[u8; 32]>,
    min: [i64; 3],
    max: [i64; 3],
    selected_step_index: u8,
) -> WholeCellDimensionlessTransferInputV1 {
    let binding = compile_optical_band_time_binding(band, [5; 32]).expect("binding");
    let transport_input = transport(binding.band_time_id, evidence);
    let transport_certificate =
        compile_origin_anchored_transport(&transport_input).expect("transport");
    let receiver = receiver(&transport_input, min, max);
    let coupling_input = WholeCellReceiverCouplingInputV1 {
        schema_version: 1,
        transport_input,
        transport_certificate,
        selected_step_index,
        receiver,
    };
    let coupling = compile_whole_cell_receiver_coupling(&coupling_input).expect("coupling");
    let bulk_profile = profile(&coupling_input.transport_input, substance);
    WholeCellDimensionlessTransferInputV1 {
        schema_version: 1,
        bulk_profile,
        receiver_coupling_input: coupling_input,
        receiver_coupling: coupling,
        band_time_binding: binding,
    }
}

#[test]
fn band_time_binding_is_deterministic_and_fail_closed() {
    let binding =
        compile_optical_band_time_binding(VisibleRadianceBandV1::Red, [5; 32]).expect("binding");
    assert_eq!(
        compile_optical_band_time_binding(VisibleRadianceBandV1::Red, [5; 32]).expect("binding"),
        binding
    );
    assert!(compile_optical_band_time_binding(VisibleRadianceBandV1::Red, [0; 32]).is_err());
    let bytes = binding.to_bytes().expect("bytes");
    assert_eq!(
        OpticalBandTimeBindingV1::from_bytes(&bytes).expect("decode"),
        binding
    );
    let mut forged = binding;
    forged.band_time_id = [9; 32];
    assert!(forged.to_bytes().is_err());
}

#[test]
fn finite_prefix_selected_partial_and_codecs_replay() {
    let one = 1_i64 << 32;
    let two = 2_i64 << 32;
    let substance = [30; 32];
    let input = fixture(
        VisibleRadianceBandV1::Red,
        CellEvidenceV1::Gas {
            substance_source_id: substance,
        },
        Some(substance),
        [0, one * 3 / 4, 0],
        [two, one - 1, one],
        1,
    );
    let result = compile_whole_cell_dimensionless_transfer(&input).expect("transfer");
    match &result.outcome {
        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedFiniteTransfer {
            bulk_evaluation,
        } => {
            assert!(bulk_evaluation.optical_depth_upper_q64_64.to_u128() > 0);
            assert!(bulk_evaluation.transfer_lower_q0_48 <= bulk_evaluation.transfer_upper_q0_48);
        }
        other => panic!("expected finite accepted transfer, got {other:?}"),
    }
    assert_eq!(result.accepted_measure.numerator, "1");
    assert_eq!(result.conditional_bulk_transfers.len(), 2);
    validate_whole_cell_dimensionless_transfer(&input, &result).expect("validates");
    let input_bytes = input.to_bytes().expect("input bytes");
    assert_eq!(
        WholeCellDimensionlessTransferInputV1::from_bytes(&input_bytes).expect("input decode"),
        input
    );
    let bytes = result.to_bytes(&input).expect("result bytes");
    assert_eq!(
        WholeCellDimensionlessTransferV1::from_bytes(&input, &bytes).expect("result decode"),
        result
    );
}

#[test]
fn selected_and_prefix_opacity_remain_distinct() {
    let one = 1_i64 << 32;
    let two = 2_i64 << 32;
    let substance = [30; 32];
    let selected = fixture(
        VisibleRadianceBandV1::Blue,
        CellEvidenceV1::Gas {
            substance_source_id: substance,
        },
        Some(substance),
        [one / 2, 0, 0],
        [one - 1, one, one],
        0,
    );
    assert!(matches!(
        compile_whole_cell_dimensionless_transfer(&selected)
            .expect("selected opacity")
            .outcome,
        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedUnresolvedTransfer { .. }
    ));

    let prefix = fixture(
        VisibleRadianceBandV1::Blue,
        CellEvidenceV1::Gas {
            substance_source_id: substance,
        },
        Some(substance),
        [0, one * 3 / 4, 0],
        [two, one - 1, one],
        1,
    );
    assert!(matches!(
        compile_whole_cell_dimensionless_transfer(&prefix)
            .expect("prefix opacity")
            .outcome,
        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedOpaqueTransfer { .. }
    ));
}

#[test]
fn zero_unresolved_and_forgery_preserve_authority_boundary() {
    let one = 1_i64 << 32;
    let two = 2_i64 << 32;
    let three = 3_i64 << 32;
    let zero_input = fixture(
        VisibleRadianceBandV1::Red,
        CellEvidenceV1::Vacuum,
        None,
        [two, 0, 0],
        [three, one, one],
        0,
    );
    let zero = compile_whole_cell_dimensionless_transfer(&zero_input).expect("zero");
    assert!(matches!(
        zero.outcome,
        WholeCellDimensionlessTransferOutcomeV1::CertifiedZeroCoupling
    ));
    assert!(zero.conditional_bulk_transfers.is_empty());
    assert_eq!(zero.zero_measure.numerator, "1");

    let unresolved_input = fixture(
        VisibleRadianceBandV1::Red,
        CellEvidenceV1::Vacuum,
        None,
        [one, 0, 0],
        [two, one, one],
        0,
    );
    let unresolved =
        compile_whole_cell_dimensionless_transfer(&unresolved_input).expect("unresolved");
    assert!(matches!(
        unresolved.outcome,
        WholeCellDimensionlessTransferOutcomeV1::UnresolvedCoupling
    ));
    assert_eq!(unresolved.unresolved_measure.numerator, "1");

    let mut forged = zero.clone();
    forged.authority_effect = "runtime_authorized".into();
    assert!(validate_whole_cell_dimensionless_transfer(&zero_input, &forged).is_err());
    let mut trailing = zero_input.to_bytes().expect("input bytes");
    trailing.push(b' ');
    assert!(WholeCellDimensionlessTransferInputV1::from_bytes(&trailing).is_err());
}

#[test]
fn start_inside_identity_and_cross_owner_mutations_fail_closed() {
    let half = 1_i64 << 31;
    let mut input = fixture(
        VisibleRadianceBandV1::Red,
        CellEvidenceV1::Vacuum,
        None,
        [0, 0, 0],
        [half, half, half],
        0,
    );
    let result = compile_whole_cell_dimensionless_transfer(&input).expect("start inside");
    match result.outcome {
        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedFiniteTransfer {
            bulk_evaluation,
        } => {
            assert_eq!(bulk_evaluation.optical_depth_lower_q64_64.to_u128(), 0);
            assert_eq!(bulk_evaluation.optical_depth_upper_q64_64.to_u128(), 0);
        }
        other => panic!("expected start-inside identity, got {other:?}"),
    }
    assert!(result.conditional_bulk_transfers.is_empty());

    input.band_time_binding =
        compile_optical_band_time_binding(VisibleRadianceBandV1::Green, [5; 32])
            .expect("foreign band binding");
    assert!(compile_whole_cell_dimensionless_transfer(&input).is_err());

    let mut nested = fixture(
        VisibleRadianceBandV1::Red,
        CellEvidenceV1::Vacuum,
        None,
        [0, 0, 0],
        [half, half, half],
        0,
    );
    nested.receiver_coupling_input.transport_certificate.steps[0].step_id = [9; 32];
    assert!(compile_whole_cell_dimensionless_transfer(&nested).is_err());
}
