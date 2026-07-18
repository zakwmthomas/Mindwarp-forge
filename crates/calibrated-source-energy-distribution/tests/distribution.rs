use calibrated_source_energy_distribution::*;
use calibrated_spectral_time_basis::{
    CalibratedBandV1, CalibratedSpectralIntervalV1, CalibratedSpectralTimeBasisInputV1,
    CalibratedTimeCellV1, ExactUnsignedRationalV1, compile_calibrated_spectral_time_basis,
};
use optical_phase_space_cell_binding::{
    CorrelatedAffineOutputV1, OpticalPhaseSpaceRootInputV1, PhaseSpaceOutputRoleV1,
    PhaseSpaceParameterAxisV1, PhaseSpaceParameterizationV1, PositiveRationalV1,
    compile_optical_phase_space_root,
};
use sha2::{Digest, Sha256};

fn rational(numerator: &str, denominator: &str) -> ExactUnsignedRationalV1 {
    ExactUnsignedRationalV1 {
        denominator: denominator.into(),
        numerator: numerator.into(),
    }
}

fn energy(numerator: &str, denominator: &str) -> ExactRadiantEnergyV1 {
    ExactRadiantEnergyV1 {
        denominator: denominator.into(),
        numerator: numerator.into(),
    }
}

fn calibration() -> calibrated_spectral_time_basis::CalibratedSpectralTimeBasisV1 {
    compile_calibrated_spectral_time_basis(&CalibratedSpectralTimeBasisInputV1 {
        basis_version: 1,
        calibration_provenance_id: [17; 32],
        quantity_kind: "radiant_energy".into(),
        schema_version: 1,
        spectral_coordinate: "vacuum_wavelength_metre".into(),
        spectral_intervals: [
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Blue,
                lower: rational("1", "2500000"),
                upper: rational("1", "2000000"),
            },
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Green,
                lower: rational("1", "2000000"),
                upper: rational("3", "5000000"),
            },
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Red,
                lower: rational("3", "5000000"),
                upper: rational("7", "10000000"),
            },
        ],
        spectral_weighting: "unit_energy_integral".into(),
        time_cell: CalibratedTimeCellV1 {
            clock_origin_id: [34; 32],
            end_tick: 116,
            seconds_per_tick: rational("1", "1000"),
            start_tick: 100,
        },
        unit: "joule".into(),
    })
    .unwrap()
}

fn form(role: PhaseSpaceOutputRoleV1, coefficients: [&str; 4]) -> CorrelatedAffineOutputV1 {
    CorrelatedAffineOutputV1 {
        role,
        center_numerator: "0".into(),
        coefficient_numerators: coefficients.map(str::to_owned),
        remainder_lower_numerator: "0".into(),
        remainder_upper_numerator: "0".into(),
    }
}

fn root() -> optical_phase_space_cell_binding::OpticalPhaseSpaceCellV1 {
    compile_optical_phase_space_root(&OpticalPhaseSpaceRootInputV1 {
        schema_version: 1,
        source_id: [1; 32],
        scope_id: [2; 32],
        reconstruction_id: [3; 32],
        source_revision: 1,
        parameterization: PhaseSpaceParameterizationV1::TransverseAreaDirection4d,
        measure: PositiveRationalV1 {
            numerator: "1".into(),
            denominator: "1".into(),
        },
        form_denominator: "4".into(),
        forms: [
            form(PhaseSpaceOutputRoleV1::PointX, ["1", "0", "0", "0"]),
            form(PhaseSpaceOutputRoleV1::PointY, ["0", "1", "0", "0"]),
            form(PhaseSpaceOutputRoleV1::PointZ, ["0", "0", "0", "0"]),
            form(PhaseSpaceOutputRoleV1::DirectionX, ["0", "0", "1", "0"]),
            form(PhaseSpaceOutputRoleV1::DirectionY, ["0", "0", "0", "1"]),
            form(PhaseSpaceOutputRoleV1::DirectionZ, ["0", "0", "0", "0"]),
        ],
    })
    .unwrap()
}

fn query() -> CalibratedSourceEnergyDistributionQueryV1 {
    CalibratedSourceEnergyDistributionQueryV1 {
        schema_version: 1,
        calibrated_basis: calibration(),
        selected_band: CalibratedBandV1::Green,
        source_provenance_id: [99; 32],
        root_cell: root(),
        root_joules: energy("1", "1"),
        directives: Vec::new(),
    }
}

fn directive(
    parent_allocation_id: [u8; 32],
    axis: PhaseSpaceParameterAxisV1,
    lower: (&str, &str),
    upper: (&str, &str),
) -> SourceEnergyRefinementDirectiveV1 {
    SourceEnergyRefinementDirectiveV1 {
        parent_allocation_id,
        axis,
        lower_joules: energy(lower.0, lower.1),
        upper_joules: energy(upper.0, upper.1),
        lower_resolution: SourceEnergyResolutionV1::UnresolvedWithinCell,
        upper_resolution: SourceEnergyResolutionV1::UnresolvedWithinCell,
    }
}

#[test]
fn root_only_is_exact_strict_and_replayable() {
    let query = query();
    let result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    assert_eq!(
        result.frontier_allocations,
        vec![result.root_allocation.clone()]
    );
    assert_eq!(result.resource_receipt.frontier_allocation_count, 1);
    assert_eq!(result.authority_effect, AUTHORITY_EFFECT_NONE);
    assert!(result.limitations.contains("no transport"));
    let query_bytes = query.to_bytes().unwrap();
    assert_eq!(
        CalibratedSourceEnergyDistributionQueryV1::from_bytes(&query_bytes).unwrap(),
        query
    );
    let result_bytes = result.to_bytes().unwrap();
    assert_eq!(
        CalibratedSourceEnergyDistributionV1::from_bytes(&result_bytes).unwrap(),
        result
    );
    replay_calibrated_source_energy_distribution(query, &result).unwrap();
}

#[test]
fn upstream_axis_split_is_replayed_and_energy_is_conserved() {
    let mut query = query();
    let root = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    query.directives.push(directive(
        root.root_allocation.allocation_id,
        PhaseSpaceParameterAxisV1::U2,
        ("1", "3"),
        ("2", "3"),
    ));
    let result = compile_calibrated_source_energy_distribution(query).unwrap();
    assert_eq!(result.frontier_allocations.len(), 2);
    assert_eq!(result.split_receipts.len(), 1);
    assert_eq!(
        result.frontier_allocations[0].path[0].axis,
        PhaseSpaceParameterAxisV1::U2
    );
    assert_ne!(
        result.frontier_allocations[0].cell_id,
        result.frontier_allocations[1].cell_id
    );
    assert!(result.maximum_energy_arithmetic_live_bits > 0);
}

#[test]
fn mixed_depth_frontier_has_canonical_path_order() {
    let mut query = query();
    let root_result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    query.directives.push(directive(
        root_result.root_allocation.allocation_id,
        PhaseSpaceParameterAxisV1::U1,
        ("1", "2"),
        ("1", "2"),
    ));
    let once = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    query.directives.push(directive(
        once.frontier_allocations[0].allocation_id,
        PhaseSpaceParameterAxisV1::U3,
        ("1", "4"),
        ("1", "4"),
    ));
    let result = compile_calibrated_source_energy_distribution(query).unwrap();
    assert_eq!(result.frontier_allocations.len(), 3);
    assert_eq!(result.frontier_allocations[0].path.len(), 2);
    assert_eq!(result.frontier_allocations[1].path.len(), 2);
    assert_eq!(result.frontier_allocations[2].path.len(), 1);
}

#[test]
fn non_frontier_and_nonconserving_directives_fail_typed() {
    let mut missing = query();
    missing.directives.push(directive(
        [88; 32],
        PhaseSpaceParameterAxisV1::U0,
        ("1", "2"),
        ("1", "2"),
    ));
    assert_eq!(
        compile_calibrated_source_energy_distribution(missing),
        Err(SourceEnergyDistributionError::NonFrontierParent)
    );

    let mut mismatch = query();
    let root_result = compile_calibrated_source_energy_distribution(mismatch.clone()).unwrap();
    mismatch.directives.push(directive(
        root_result.root_allocation.allocation_id,
        PhaseSpaceParameterAxisV1::U0,
        ("1", "3"),
        ("1", "3"),
    ));
    assert_eq!(
        compile_calibrated_source_energy_distribution(mismatch),
        Err(SourceEnergyDistributionError::EnergyConservationMismatch)
    );
}

#[test]
fn resolved_leaf_cannot_be_refined_again() {
    let mut query = query();
    let root_result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    let mut first = directive(
        root_result.root_allocation.allocation_id,
        PhaseSpaceParameterAxisV1::U0,
        ("1", "2"),
        ("1", "2"),
    );
    first.lower_resolution = SourceEnergyResolutionV1::ResolvedLeaf;
    query.directives.push(first);
    let once = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    query.directives.push(directive(
        once.frontier_allocations[0].allocation_id,
        PhaseSpaceParameterAxisV1::U1,
        ("1", "4"),
        ("1", "4"),
    ));
    assert_eq!(
        compile_calibrated_source_energy_distribution(query),
        Err(SourceEnergyDistributionError::InvalidResolution)
    );
}

#[test]
fn provenance_and_canonical_decimal_shields_fail_closed() {
    let mut conflated = query();
    conflated.source_provenance_id = conflated.calibrated_basis.input.calibration_provenance_id;
    assert_eq!(
        compile_calibrated_source_energy_distribution(conflated),
        Err(SourceEnergyDistributionError::ProvenanceConflation)
    );

    let mut decimal = query();
    decimal.root_joules = energy("01", "1");
    assert_eq!(
        compile_calibrated_source_energy_distribution(decimal),
        Err(SourceEnergyDistributionError::NoncanonicalEnergy)
    );
}

#[test]
fn result_replay_rejects_identity_tamper_and_codecs_reject_trailing_bytes() {
    let query = query();
    let mut result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    result.distribution_id[0] ^= 1;
    assert_eq!(
        replay_calibrated_source_energy_distribution(query.clone(), &result),
        Err(SourceEnergyDistributionError::IdentityMismatch)
    );
    let mut bytes = query.to_bytes().unwrap();
    bytes.push(b' ');
    assert_eq!(
        CalibratedSourceEnergyDistributionQueryV1::from_bytes(&bytes),
        Err(SourceEnergyDistributionError::CodecDefect)
    );
}

#[test]
fn directive_count_is_bounded_before_replay() {
    let mut query = query();
    query.directives = (0..=MAX_REFINEMENT_DIRECTIVES)
        .map(|_| {
            directive(
                [1; 32],
                PhaseSpaceParameterAxisV1::U0,
                ("0", "1"),
                ("1", "1"),
            )
        })
        .collect();
    assert_eq!(
        compile_calibrated_source_energy_distribution(query),
        Err(SourceEnergyDistributionError::ResourceCeiling)
    );
}

fn hex(bytes: [u8; 32]) -> String {
    bytes.iter().map(|value| format!("{value:02x}")).collect()
}

fn sha(bytes: &[u8]) -> String {
    hex(Sha256::digest(bytes).into())
}

#[test]
fn identity_and_codec_fixture_lock() {
    let lock: serde_json::Value = serde_json::from_str(include_str!(
        "../fixtures/distribution_v1_identity_lock.json"
    ))
    .unwrap();
    let mut query = query();
    let root_result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    query.directives.push(directive(
        root_result.root_allocation.allocation_id,
        PhaseSpaceParameterAxisV1::U2,
        ("1", "3"),
        ("2", "3"),
    ));
    let result = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
    let query_bytes = query.to_bytes().unwrap();
    let result_bytes = result.to_bytes().unwrap();
    assert_eq!(
        query_bytes.len(),
        lock["query"]["bytes"].as_u64().unwrap() as usize
    );
    assert_eq!(sha(&query_bytes), lock["query"]["sha256"]);
    assert_eq!(
        result_bytes.len(),
        lock["result"]["bytes"].as_u64().unwrap() as usize
    );
    assert_eq!(sha(&result_bytes), lock["result"]["sha256"]);
    assert_eq!(hex(result.subject_id), lock["subject_id"]);
    assert_eq!(
        hex(result.root_allocation.allocation_id),
        lock["root_allocation_id"]
    );
    assert_eq!(
        hex(result.split_receipts[0].energy_split_id),
        lock["energy_split_id"]
    );
    assert_eq!(hex(result.distribution_id), lock["distribution_id"]);
}

#[test]
fn full_sixty_four_leaf_envelope_is_admitted() {
    let mut query = query();
    query.root_joules = energy("64", "1");
    for _ in 0..MAX_REFINEMENT_DIRECTIVES {
        let current = compile_calibrated_source_energy_distribution(query.clone()).unwrap();
        let parent = current
            .frontier_allocations
            .iter()
            .min_by_key(|allocation| allocation.path.len())
            .unwrap();
        let numerator: u128 = parent.joules.numerator.parse().unwrap();
        assert_eq!(numerator % 2, 0);
        let half = (numerator / 2).to_string();
        query.directives.push(directive(
            parent.allocation_id,
            PhaseSpaceParameterAxisV1::U0,
            (&half, &parent.joules.denominator),
            (&half, &parent.joules.denominator),
        ));
    }
    let result = compile_calibrated_source_energy_distribution(query).unwrap();
    assert_eq!(result.frontier_allocations.len(), MAX_FRONTIER_ALLOCATIONS);
    assert_eq!(result.split_receipts.len(), MAX_REFINEMENT_DIRECTIVES);
    assert!(
        result.resource_receipt.aggregate_live_canonical_bytes as usize
            <= MAX_AGGREGATE_LIVE_CANONICAL_BYTES
    );
}
