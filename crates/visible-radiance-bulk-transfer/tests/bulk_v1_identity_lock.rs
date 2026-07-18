use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, ColumnRunV1, CoordinateFrameV1, Id,
    PhysicalPathQueryV1, PhysicalVolumeRecipeInputV1, compile_path_witness,
    compile_physical_volume, compile_physical_volume_recipe,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use visible_radiance_bulk_transfer::{
    BulkBandInteractionV1, SubstanceBulkInteractionV1, VisibleRadianceBulkProfileInputV1,
    VisibleRadianceBulkQueryV1, compile_visible_radiance_bulk_profile,
    compile_visible_radiance_bulk_transfer,
};

const ONE: i64 = 1_i64 << 32;

fn id(value: u32) -> Id {
    let mut result = [0_u8; 32];
    result[..4].copy_from_slice(&value.to_le_bytes());
    result[31] = 1;
    result
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn finite(value: u64) -> BulkBandInteractionV1 {
    BulkBandInteractionV1::Finite {
        extinction_q16_48_per_coordinate_unit: value,
    }
}

fn recipe(
    default_evidence: CellEvidenceV1,
    extent: [u32; 3],
    runs: Vec<ColumnRunV1>,
) -> PhysicalVolumeRecipeInputV1 {
    PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: ONE,
        extent,
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence,
        column_runs: runs,
    }
}

fn profile_input(
    volume: PhysicalVolumeRecipeInputV1,
    interactions: Vec<SubstanceBulkInteractionV1>,
) -> VisibleRadianceBulkProfileInputV1 {
    VisibleRadianceBulkProfileInputV1 {
        schema_version: 1,
        profile_source_id: id(10),
        scope_id: id(11),
        reconstruction_id: volume.reconstruction_id,
        profile_revision: 1,
        physical_volume_recipe_input: volume,
        substance_interactions: interactions,
    }
}

fn interaction(substance_source_id: Id, red: BulkBandInteractionV1) -> SubstanceBulkInteractionV1 {
    SubstanceBulkInteractionV1 {
        substance_source_id,
        bands_rgb: [red, finite(1_u64 << 48), BulkBandInteractionV1::Opaque],
    }
}

fn family(
    name: &str,
    input: VisibleRadianceBulkProfileInputV1,
    start: [i64; 3],
    end: [i64; 3],
) -> Value {
    let input_bytes = input.to_bytes().expect("profile input");
    let profile = compile_visible_radiance_bulk_profile(&input).expect("profile");
    let profile_bytes = profile.to_bytes().expect("profile bytes");
    let query = VisibleRadianceBulkQueryV1 {
        schema_version: 1,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        path_query: PhysicalPathQueryV1 {
            schema_version: 1,
            physical_volume_id: profile.physical_volume_id,
            start_q32_32: start,
            end_q32_32: end,
        },
    };
    let query_bytes = query.to_bytes(&profile).expect("query bytes");
    let recipe = compile_physical_volume_recipe(&profile.input.physical_volume_recipe_input)
        .expect("recipe");
    let volume = compile_physical_volume(&recipe).expect("volume");
    let witness = compile_path_witness(&recipe, &volume, &query.path_query).expect("witness");
    let transfer = compile_visible_radiance_bulk_transfer(&profile, &query).expect("transfer");
    let transfer_bytes = transfer.to_bytes(&profile, &query).expect("transfer bytes");
    let outcome = serde_json::to_value(&transfer.outcome).expect("outcome value");
    let outcome_bytes = serde_json::to_vec(&transfer.outcome).expect("outcome bytes");
    json!({
        "name": name,
        "profile_input_bytes": input_bytes.len(),
        "profile_input_sha256": sha256(&input_bytes),
        "profile_bytes": profile_bytes.len(),
        "profile_sha256": sha256(&profile_bytes),
        "profile_id": hex(&profile.visible_radiance_bulk_profile_id),
        "recipe_id": hex(&recipe.physical_volume_recipe_id),
        "volume_id": hex(&volume.physical_volume_id),
        "query_bytes": query_bytes.len(),
        "query_sha256": sha256(&query_bytes),
        "path_query_id": hex(&witness.path_query_id),
        "path_witness_id": hex(&witness.path_witness_id),
        "transfer_bytes": transfer_bytes.len(),
        "transfer_sha256": sha256(&transfer_bytes),
        "transfer_id": hex(&transfer.visible_radiance_bulk_transfer_id),
        "outcome_kind": outcome.get("kind").expect("tagged outcome"),
        "outcome_sha256": sha256(&outcome_bytes),
    })
}

fn actual_fixture() -> Value {
    let substance_a = id(20);
    let substance_b = id(21);
    let one_cell_path = ([0, ONE / 2, ONE / 2], [ONE, ONE / 2, ONE / 2]);
    let mut families = Vec::new();

    families.push(family(
        "vacuum_identity",
        profile_input(recipe(CellEvidenceV1::Vacuum, [1, 1, 1], vec![]), vec![]),
        one_cell_path.0,
        one_cell_path.1,
    ));
    families.push(family(
        "finite_zero_identity",
        profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance_a,
                },
                [1, 1, 1],
                vec![],
            ),
            vec![interaction(substance_a, finite(0))],
        ),
        one_cell_path.0,
        one_cell_path.1,
    ));
    families.push(family(
        "finite_positive_attenuation",
        profile_input(
            recipe(
                CellEvidenceV1::Liquid {
                    substance_source_id: substance_a,
                },
                [1, 1, 1],
                vec![],
            ),
            vec![interaction(substance_a, finite(1_u64 << 47))],
        ),
        one_cell_path.0,
        one_cell_path.1,
    ));
    families.push(family(
        "opaque_termination",
        profile_input(
            recipe(
                CellEvidenceV1::Solid {
                    substance_source_id: substance_a,
                },
                [1, 1, 1],
                vec![],
            ),
            vec![interaction(substance_a, BulkBandInteractionV1::Opaque)],
        ),
        one_cell_path.0,
        one_cell_path.1,
    ));
    families.push(family(
        "unavailable_evidence",
        profile_input(
            recipe(CellEvidenceV1::Unavailable, [1, 1, 1], vec![]),
            vec![],
        ),
        one_cell_path.0,
        one_cell_path.1,
    ));
    families.push(family(
        "ambiguous_boundary_lane",
        profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance_a,
                },
                [1, 2, 1],
                vec![],
            ),
            vec![interaction(substance_a, finite(1_u64 << 47))],
        ),
        [0, ONE, ONE / 2],
        [ONE, ONE, ONE / 2],
    ));
    families.push(family(
        "interface_model_required",
        profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance_a,
                },
                [2, 1, 1],
                vec![ColumnRunV1 {
                    x_index: 1,
                    y_index: 0,
                    z_start: 0,
                    length: 1,
                    evidence: CellEvidenceV1::Solid {
                        substance_source_id: substance_b,
                    },
                }],
            ),
            vec![
                interaction(substance_a, finite(1_u64 << 47)),
                interaction(substance_b, finite(1_u64 << 48)),
            ],
        ),
        [0, ONE / 2, ONE / 2],
        [2 * ONE, ONE / 2, ONE / 2],
    ));
    families.push(family(
        "stationary_point_behavior",
        profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance_a,
                },
                [1, 1, 1],
                vec![],
            ),
            vec![interaction(substance_a, finite(1_u64 << 47))],
        ),
        [ONE / 2; 3],
        [ONE / 2; 3],
    ));

    json!({"schema_version": 1, "families": families})
}

#[test]
fn bulk_v1_bytes_and_identities_remain_locked() {
    let actual = actual_fixture();
    let expected: Value =
        serde_json::from_str(include_str!("../fixtures/bulk_v1_identity_lock.json"))
            .expect("canonical fixture");
    assert_eq!(
        actual,
        expected,
        "actual fixture:\n{}",
        serde_json::to_string_pretty(&actual).unwrap()
    );
}
