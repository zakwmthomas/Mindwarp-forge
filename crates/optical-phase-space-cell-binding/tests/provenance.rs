use optical_phase_space_cell_binding::*;

fn id(value: u8) -> [u8; 32] {
    [value; 32]
}

fn form(
    role: PhaseSpaceOutputRoleV1,
    center: &str,
    coefficients: [&str; 4],
) -> CorrelatedAffineOutputV1 {
    CorrelatedAffineOutputV1 {
        role,
        center_numerator: center.into(),
        coefficient_numerators: coefficients.map(str::to_owned),
        remainder_lower_numerator: "0".into(),
        remainder_upper_numerator: "0".into(),
    }
}

fn input() -> OpticalPhaseSpaceRootInputV1 {
    OpticalPhaseSpaceRootInputV1 {
        schema_version: 1,
        source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        source_revision: 1,
        parameterization: PhaseSpaceParameterizationV1::TransverseAreaDirection4d,
        measure: PositiveRationalV1 {
            numerator: "1".into(),
            denominator: "1".into(),
        },
        form_denominator: "4".into(),
        forms: [
            form(PhaseSpaceOutputRoleV1::PointX, "0", ["1", "0", "0", "0"]),
            form(PhaseSpaceOutputRoleV1::PointY, "0", ["0", "1", "0", "0"]),
            form(PhaseSpaceOutputRoleV1::PointZ, "0", ["0", "0", "0", "0"]),
            form(
                PhaseSpaceOutputRoleV1::DirectionX,
                "0",
                ["0", "0", "1", "0"],
            ),
            form(
                PhaseSpaceOutputRoleV1::DirectionY,
                "0",
                ["0", "0", "0", "1"],
            ),
            form(
                PhaseSpaceOutputRoleV1::DirectionZ,
                "0",
                ["0", "0", "0", "0"],
            ),
        ],
    }
}

#[test]
fn root_split_and_projection_are_exact_and_strict() {
    let root = compile_optical_phase_space_root(&input()).unwrap();
    assert_eq!(root.cell_id, root.root_id);
    assert_eq!(
        OpticalPhaseSpaceCellV1::from_bytes(&root.to_bytes().unwrap()).unwrap(),
        root
    );
    let split = split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
        schema_version: 1,
        cell: root.clone(),
        axis: PhaseSpaceParameterAxisV1::U0,
    })
    .unwrap();
    assert_eq!(
        split.children[0].measure,
        PositiveRationalV1 {
            numerator: "1".into(),
            denominator: "2".into()
        }
    );
    assert_eq!(split.children[1].measure, split.children[0].measure);
    assert_eq!(split.children[0].forms[0].center_numerator, "-1");
    assert_eq!(split.children[1].forms[0].center_numerator, "1");
    assert_eq!(split.children[0].form_denominator, "8");
    let projection = project_optical_phase_space_cell(&OpticalPhaseSpaceProjectionQueryV1 {
        schema_version: 1,
        cell: split.children[0].clone(),
        target: OpticalProjectionTargetV1::ExistingOpticalIntervalSeamV1,
    })
    .unwrap();
    assert_eq!(projection.position_intervals[0].fractional_bits, 160);
    assert_eq!(projection.direction_intervals[0].fractional_bits, 62);
    assert_eq!(projection.authority_effect, AUTHORITY_EFFECT_NONE);
    let mut bytes = root.to_bytes().unwrap();
    bytes.push(b' ');
    assert_eq!(
        OpticalPhaseSpaceCellV1::from_bytes(&bytes),
        Err(OpticalPhaseSpaceCellError::CodecDefect)
    );
}

#[test]
fn correlation_depth_caps_and_hostile_forms_fail_typed() {
    let mut correlated = input();
    correlated.forms[1].coefficient_numerators = ["1".into(), "0".into(), "0".into(), "0".into()];
    let root = compile_optical_phase_space_root(&correlated).unwrap();
    assert_eq!(
        correlated_difference_interval(&root, 0, 1).unwrap(),
        ("0".into(), "0".into())
    );
    let mut cell = root;
    for _ in 0..MAX_DEPTH {
        cell = split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
            schema_version: 1,
            cell,
            axis: PhaseSpaceParameterAxisV1::U0,
        })
        .unwrap()
        .children[0]
            .clone();
    }
    let retained = cell.measure.clone();
    assert_eq!(
        split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
            schema_version: 1,
            cell,
            axis: PhaseSpaceParameterAxisV1::U0
        }),
        Err(OpticalPhaseSpaceCellError::DepthLimit {
            retained_measure: retained
        })
    );
    let mut noncanonical = input();
    noncanonical.measure.numerator = "01".into();
    assert_eq!(
        compile_optical_phase_space_root(&noncanonical),
        Err(OpticalPhaseSpaceCellError::NoncanonicalRational)
    );
    let mut aliased = input();
    aliased.form_denominator = "8".into();
    for form in &mut aliased.forms {
        for coefficient in &mut form.coefficient_numerators {
            *coefficient = (coefficient.parse::<i32>().unwrap() * 2).to_string();
        }
    }
    assert_eq!(
        compile_optical_phase_space_root(&aliased),
        Err(OpticalPhaseSpaceCellError::NoncanonicalForm)
    );
}

#[test]
fn direction_projection_rejects_values_outside_q1_62() {
    let mut hostile = input();
    hostile.forms[3].center_numerator = "5".into();
    assert_eq!(
        project_optical_phase_space_cell(&OpticalPhaseSpaceProjectionQueryV1 {
            schema_version: 1,
            cell: compile_optical_phase_space_root(&hostile).unwrap(),
            target: OpticalProjectionTargetV1::ExistingOpticalIntervalSeamV1
        }),
        Err(OpticalPhaseSpaceCellError::ProjectionOutOfRange)
    );
}

fn hex(value: &[u8; 32]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn sixty_four_leaf_conservation_bit_edges_and_identity_fixtures() {
    let root = compile_optical_phase_space_root(&input()).unwrap();
    let first = split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
        schema_version: 1,
        cell: root.clone(),
        axis: PhaseSpaceParameterAxisV1::U0,
    })
    .unwrap();
    let projection = project_optical_phase_space_cell(&OpticalPhaseSpaceProjectionQueryV1 {
        schema_version: 1,
        cell: first.children[0].clone(),
        target: OpticalProjectionTargetV1::ExistingOpticalIntervalSeamV1,
    })
    .unwrap();
    assert_eq!(
        hex(&root.root_id),
        "8724e0219d44bc40dbcb7315369dabe3153710617def82854d1ad490a802141f"
    );
    assert_eq!(
        hex(&first.split_id),
        "c379a3a24786c2ab7187e54e2db97abc801f8c6022f64d29d75414b85fb5f320"
    );
    assert_eq!(
        hex(&projection.projection_id),
        "2f7a9ddde5fc877add6b05eff7cbbac8e3ca84de35e224455a11d0bd074fe3ee"
    );

    let mut leaves = vec![root];
    for depth in 0..6 {
        let axis = [
            PhaseSpaceParameterAxisV1::U0,
            PhaseSpaceParameterAxisV1::U1,
            PhaseSpaceParameterAxisV1::U2,
            PhaseSpaceParameterAxisV1::U3,
        ][depth % 4];
        leaves = leaves
            .into_iter()
            .flat_map(|cell| {
                split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
                    schema_version: 1,
                    cell,
                    axis,
                })
                .unwrap()
                .children
            })
            .collect();
    }
    assert_eq!(leaves.len(), 64);
    assert!(leaves.iter().all(|cell| cell.measure
        == PositiveRationalV1 {
            numerator: "1".into(),
            denominator: "64".into()
        }));
    assert_eq!(
        OpticalPhaseSpaceSplitReceiptV1::from_bytes(&first.to_bytes().unwrap()).unwrap(),
        first
    );
    assert_eq!(
        OpticalPhaseSpaceProjectionReceiptV1::from_bytes(&projection.to_bytes().unwrap()).unwrap(),
        projection
    );

    let mut edge = input();
    edge.forms[0].center_numerator =
        "3138550867693340381917894711603833208051177722232017256448".into();
    assert!(compile_optical_phase_space_root(&edge).is_ok());
    edge.forms[0].center_numerator =
        "6277101735386680763835789423207666416102355444464034512896".into();
    assert_eq!(
        compile_optical_phase_space_root(&edge),
        Err(OpticalPhaseSpaceCellError::ResourceCeiling)
    );
}
