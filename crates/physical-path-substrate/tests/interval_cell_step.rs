use physical_path_substrate::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const Q160_TWO: &str = "680564733841876926926749214863536422912";
const Q62_ONE: &str = "4611686018427387904";

fn id(byte: u8) -> Id {
    [byte; 32]
}
fn interval(bits: u16, lower: &str, upper: &str) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: bits,
        lower: lower.into(),
        upper: upper.into(),
    }
}
fn recipe_input(extent: [u32; 3], evidence: CellEvidenceV1) -> PhysicalVolumeRecipeInputV1 {
    PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: 4,
        extent,
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence: evidence,
        column_runs: Vec::new(),
    }
}
fn setup(extent: [u32; 3], evidence: CellEvidenceV1) -> (PhysicalVolumeRecipeV1, PhysicalVolumeV1) {
    let recipe = compile_physical_volume_recipe(&recipe_input(extent, evidence)).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    (recipe, volume)
}
fn input(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    direction: [(&str, &str); 3],
) -> ConditionalIntervalCellStepInputV1 {
    ConditionalIntervalCellStepInputV1 {
        schema_version: 1,
        state_source_id: id(9),
        scope_id: recipe.input.scope_id,
        reconstruction_id: recipe.input.reconstruction_id,
        state_revision: 1,
        evidence_kind: ConditionalIntervalEvidenceKindV1::DeclaredConditionalPointDirectionBox,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        physical_volume_id: volume.physical_volume_id,
        current_cell: CellIndex3V1 { x: 0, y: 0, z: 0 },
        point_q160: [
            interval(160, Q160_TWO, Q160_TWO),
            interval(160, Q160_TWO, Q160_TWO),
            interval(160, Q160_TWO, Q160_TWO),
        ],
        direction_q1_62: direction.map(|(lower, upper)| interval(62, lower, upper)),
    }
}

fn shifted_q160(value: i64) -> String {
    let negative = value.is_negative();
    let mut digits = value.unsigned_abs().to_string().into_bytes();
    for _ in 0..128 {
        let mut carry = 0_u8;
        for digit in digits.iter_mut().rev() {
            let doubled = (*digit - b'0') * 2 + carry;
            *digit = b'0' + doubled % 10;
            carry = doubled / 10;
        }
        if carry != 0 {
            digits.insert(0, b'0' + carry);
        }
    }
    let magnitude = String::from_utf8(digits).unwrap();
    if negative {
        format!("-{magnitude}")
    } else {
        magnitude
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct LockBlob {
    len: usize,
    sha256: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct IntervalIdentityFamily {
    name: String,
    input: LockBlob,
    event: LockBlob,
    physical_volume_recipe_id: String,
    physical_volume_id: String,
    interval_cell_step_input_id: String,
    interval_cell_step_event_id: String,
    outcome: String,
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 15) as usize] as char);
    }
    output
}

fn lock_blob(bytes: &[u8]) -> LockBlob {
    LockBlob {
        len: bytes.len(),
        sha256: hex(&Sha256::digest(bytes)),
    }
}

fn outcome_name(outcome: &ConditionalIntervalCellStepOutcomeV1) -> &'static str {
    match outcome {
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. } => "certified_next_face",
        ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace => "ambiguous_next_face",
        ConditionalIntervalCellStepOutcomeV1::NoForwardProgress => "no_forward_progress",
        ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. } => "outer_domain_exit",
        ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { .. } => "unavailable_neighbor",
    }
}

fn identity_family(
    name: &str,
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    input: ConditionalIntervalCellStepInputV1,
) -> IntervalIdentityFamily {
    let input_bytes = input.to_bytes(recipe, volume).unwrap();
    let event = compile_conditional_interval_cell_step(recipe, volume, &input).unwrap();
    let event_bytes = event.to_bytes(recipe, volume, &input).unwrap();
    IntervalIdentityFamily {
        name: name.into(),
        input: lock_blob(&input_bytes),
        event: lock_blob(&event_bytes),
        physical_volume_recipe_id: hex(&recipe.physical_volume_recipe_id),
        physical_volume_id: hex(&volume.physical_volume_id),
        interval_cell_step_input_id: hex(&event.interval_cell_step_input_id),
        interval_cell_step_event_id: hex(&event.interval_cell_step_event_id),
        outcome: outcome_name(&event.outcome).into(),
    }
}

fn current_interval_identity_families() -> Vec<IntervalIdentityFamily> {
    let zero = ("0", "0");
    let (recipe, volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let normal = input(&recipe, &volume, [(Q62_ONE, Q62_ONE), zero, zero]);
    let reverse = input(
        &recipe,
        &volume,
        [("-4611686018427387904", "-4611686018427387904"), zero, zero],
    );
    let ambiguous = input(
        &recipe,
        &volume,
        [(Q62_ONE, Q62_ONE), (Q62_ONE, Q62_ONE), zero],
    );
    let no_progress = input(&recipe, &volume, [zero, zero, zero]);
    let near_parallel = input(&recipe, &volume, [(Q62_ONE, Q62_ONE), ("1", "1"), zero]);
    let (unavailable_recipe, unavailable_volume) = setup([2, 1, 1], CellEvidenceV1::Unavailable);
    let unavailable = input(
        &unavailable_recipe,
        &unavailable_volume,
        [(Q62_ONE, Q62_ONE), zero, zero],
    );
    let mut high_source = recipe_input([2, 1, 1], CellEvidenceV1::Vacuum);
    high_source.origin_q32_32 = [i64::MAX - 8, -8, -4];
    let high_recipe = compile_physical_volume_recipe(&high_source).unwrap();
    let high_volume = compile_physical_volume(&high_recipe).unwrap();
    let mut high = input(
        &high_recipe,
        &high_volume,
        [(Q62_ONE, Q62_ONE), ("1", "1"), zero],
    );
    high.current_cell = CellIndex3V1 { x: 1, y: 0, z: 0 };
    high.point_q160 = [
        interval(
            160,
            &shifted_q160(i64::MAX - 2),
            &shifted_q160(i64::MAX - 2),
        ),
        interval(160, &shifted_q160(-6), &shifted_q160(-6)),
        interval(160, &shifted_q160(-2), &shifted_q160(-2)),
    ];
    vec![
        identity_family("normal_certified_face", &recipe, &volume, normal),
        identity_family("reverse_outer_face", &recipe, &volume, reverse),
        identity_family("exact_ambiguity", &recipe, &volume, ambiguous),
        identity_family("no_forward_progress", &recipe, &volume, no_progress),
        identity_family(
            "unavailable_neighbor",
            &unavailable_recipe,
            &unavailable_volume,
            unavailable,
        ),
        identity_family(
            "near_parallel_transfer_ready",
            &recipe,
            &volume,
            near_parallel,
        ),
        identity_family("negative_near_maximum", &high_recipe, &high_volume, high),
    ]
}

#[test]
fn interval_cell_step_bytes_and_ids_remain_locked() {
    let expected: Vec<IntervalIdentityFamily> = serde_json::from_str(include_str!(
        "../fixtures/interval_cell_step_identity_lock.json"
    ))
    .unwrap();
    assert_eq!(current_interval_identity_families(), expected);
}

#[test]
fn normal_reverse_ambiguous_zero_outer_and_unavailable_are_typed() {
    let (recipe, volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let zero = ("0", "0");
    let forward = input(&recipe, &volume, [(Q62_ONE, Q62_ONE), zero, zero]);
    let event = compile_conditional_interval_cell_step(&recipe, &volume, &forward).unwrap();
    assert!(matches!(
        event.outcome,
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
    ));
    assert_eq!(event.arithmetic_receipt.fractional_bits, 160);
    assert!(event.arithmetic_receipt.observed_maximum_live_bits <= 414);
    let bytes = event.to_bytes(&recipe, &volume, &forward).unwrap();
    assert_eq!(
        ConditionalIntervalCellStepEventV1::from_bytes(&recipe, &volume, &forward, &bytes).unwrap(),
        event
    );

    let reverse = input(
        &recipe,
        &volume,
        [("-4611686018427387904", "-4611686018427387904"), zero, zero],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &reverse)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. }
    ));
    let tie = input(
        &recipe,
        &volume,
        [(Q62_ONE, Q62_ONE), (Q62_ONE, Q62_ONE), zero],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &tie)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace
    ));
    let stationary = input(&recipe, &volume, [zero, zero, zero]);
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &stationary)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::NoForwardProgress
    ));

    let (outer_recipe, outer_volume) = setup([1, 1, 1], CellEvidenceV1::Vacuum);
    let outer = input(
        &outer_recipe,
        &outer_volume,
        [(Q62_ONE, Q62_ONE), zero, zero],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&outer_recipe, &outer_volume, &outer)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. }
    ));
    let (unavailable_recipe, unavailable_volume) = setup([2, 1, 1], CellEvidenceV1::Unavailable);
    let unavailable = input(
        &unavailable_recipe,
        &unavailable_volume,
        [(Q62_ONE, Q62_ONE), zero, zero],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(
            &unavailable_recipe,
            &unavailable_volume,
            &unavailable
        )
        .unwrap()
        .outcome,
        ConditionalIntervalCellStepOutcomeV1::UnavailableNeighbor { .. }
    ));
}

#[test]
fn strict_decimals_scales_provenance_caps_and_replay_fail_closed() {
    let (recipe, volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let mut value = input(
        &recipe,
        &volume,
        [(Q62_ONE, Q62_ONE), ("0", "0"), ("0", "0")],
    );
    let bytes = value.to_bytes(&recipe, &volume).unwrap();
    assert_eq!(
        ConditionalIntervalCellStepInputV1::from_bytes(&recipe, &volume, &bytes).unwrap(),
        value
    );
    let mut poisoned = bytes.clone();
    poisoned.push(b' ');
    assert!(ConditionalIntervalCellStepInputV1::from_bytes(&recipe, &volume, &poisoned).is_err());
    let oversized = vec![b' '; MAX_INTERVAL_CELL_STEP_INPUT_BYTES + 1];
    assert!(ConditionalIntervalCellStepInputV1::from_bytes(&recipe, &volume, &oversized).is_err());
    value.direction_q1_62[0].lower = "+1".into();
    assert!(value.to_bytes(&recipe, &volume).is_err());
    value.direction_q1_62[0].lower = "01".into();
    assert!(value.to_bytes(&recipe, &volume).is_err());
    value.direction_q1_62[0].lower = "-0".into();
    assert!(value.to_bytes(&recipe, &volume).is_err());
    value.direction_q1_62[0].lower = "0".into();
    value.direction_q1_62[0].fractional_bits = 160;
    assert!(value.to_bytes(&recipe, &volume).is_err());
    value.direction_q1_62[0].fractional_bits = 62;
    value.physical_volume_id = id(44);
    assert!(value.to_bytes(&recipe, &volume).is_err());
}

#[test]
fn minimum_q62_direction_and_one_unit_reversal_remain_deterministic() {
    let (recipe, volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let tiny = input(
        &recipe,
        &volume,
        [(Q62_ONE, Q62_ONE), ("1", "1"), ("0", "0")],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &tiny)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
    ));
    let one_less = "3458764513820540928";
    let one_more = "3458764513820540929";
    let x_wins = input(
        &recipe,
        &volume,
        [(one_more, one_more), (one_less, one_less), ("0", "0")],
    );
    let y_wins = input(
        &recipe,
        &volume,
        [(one_less, one_less), (one_more, one_more), ("0", "0")],
    );
    let x = compile_conditional_interval_cell_step(&recipe, &volume, &x_wins).unwrap();
    let y = compile_conditional_interval_cell_step(&recipe, &volume, &y_wins).unwrap();
    assert!(matches!(
        x.outcome,
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
    ));
    assert!(matches!(
        y.outcome,
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
    ));
    assert_ne!(x.interval_cell_step_event_id, y.interval_cell_step_event_id);
}

#[test]
fn six_outer_exits_prior_face_zero_and_zero_straddling_competitor_are_conservative() {
    let (recipe, volume) = setup([1, 1, 1], CellEvidenceV1::Vacuum);
    let zero = ("0", "0");
    for direction in [
        [(Q62_ONE, Q62_ONE), zero, zero],
        [("-4611686018427387904", "-4611686018427387904"), zero, zero],
        [zero, (Q62_ONE, Q62_ONE), zero],
        [zero, ("-4611686018427387904", "-4611686018427387904"), zero],
        [zero, zero, (Q62_ONE, Q62_ONE)],
        [zero, zero, ("-4611686018427387904", "-4611686018427387904")],
    ] {
        assert!(matches!(
            compile_conditional_interval_cell_step(
                &recipe,
                &volume,
                &input(&recipe, &volume, direction)
            )
            .unwrap()
            .outcome,
            ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. }
        ));
    }
    let mut prior_face = input(&recipe, &volume, [(Q62_ONE, Q62_ONE), zero, zero]);
    prior_face.point_q160[0] = interval(
        160,
        "1361129467683753853853498429727072845824",
        "1361129467683753853853498429727072845824",
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &prior_face)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::NoForwardProgress
    ));

    let (wide_recipe, wide_volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let straddling = input(
        &wide_recipe,
        &wide_volume,
        [(Q62_ONE, Q62_ONE), ("-1", "1"), zero],
    );
    assert!(matches!(
        compile_conditional_interval_cell_step(&wide_recipe, &wide_volume, &straddling)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { .. }
    ));
}

#[test]
fn correlation_erasure_invalid_boxes_and_near_maximum_coordinates_fail_safely() {
    let (recipe, volume) = setup([2, 2, 2], CellEvidenceV1::Vacuum);
    let mut erased = input(
        &recipe,
        &volume,
        [
            ("2305843009213693952", Q62_ONE),
            ("2305843009213693952", Q62_ONE),
            ("0", "0"),
        ],
    );
    erased.point_q160[0] = interval(160, "0", Q160_TWO);
    erased.point_q160[1] = interval(160, "0", Q160_TWO);
    assert!(matches!(
        compile_conditional_interval_cell_step(&recipe, &volume, &erased)
            .unwrap()
            .outcome,
        ConditionalIntervalCellStepOutcomeV1::AmbiguousNextFace
    ));
    erased.point_q160[0] = interval(160, Q160_TWO, "0");
    assert!(erased.to_bytes(&recipe, &volume).is_err());
    erased.point_q160[0] = interval(160, "-1", "0");
    assert!(erased.to_bytes(&recipe, &volume).is_err());

    let mut high_source = recipe_input([2, 1, 1], CellEvidenceV1::Vacuum);
    high_source.origin_q32_32 = [i64::MAX - 8, -8, -4];
    let high_recipe = compile_physical_volume_recipe(&high_source).unwrap();
    let high_volume = compile_physical_volume(&high_recipe).unwrap();
    let mut high = input(
        &high_recipe,
        &high_volume,
        [(Q62_ONE, Q62_ONE), ("1", "1"), ("0", "0")],
    );
    high.current_cell = CellIndex3V1 { x: 1, y: 0, z: 0 };
    high.point_q160 = [
        interval(
            160,
            &shifted_q160(i64::MAX - 2),
            &shifted_q160(i64::MAX - 2),
        ),
        interval(160, &shifted_q160(-6), &shifted_q160(-6)),
        interval(160, &shifted_q160(-2), &shifted_q160(-2)),
    ];
    let event = compile_conditional_interval_cell_step(&high_recipe, &high_volume, &high).unwrap();
    assert!(matches!(
        event.outcome,
        ConditionalIntervalCellStepOutcomeV1::OuterDomainExit { .. }
    ));
    assert!(event.arithmetic_receipt.observed_maximum_live_bits <= 414);
    assert!(
        high.to_bytes(&high_recipe, &high_volume).unwrap().len()
            <= MAX_INTERVAL_CELL_STEP_INPUT_BYTES
    );
    assert!(
        event
            .to_bytes(&high_recipe, &high_volume, &high)
            .unwrap()
            .len()
            <= MAX_INTERVAL_CELL_STEP_EVENT_BYTES
    );
}
