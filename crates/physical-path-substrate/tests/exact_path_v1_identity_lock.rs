use physical_path_substrate::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Blob {
    len: usize,
    sha256: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Family {
    name: String,
    recipe_input: Blob,
    recipe: Blob,
    physical_volume_recipe_id: String,
    volume: Blob,
    occupancy_fingerprint: String,
    physical_volume_id: String,
    query: Blob,
    witness: Blob,
    path_query_id: String,
    path_witness_id: String,
}

fn id(byte: u8) -> Id {
    [byte; 32]
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        value.push(DIGITS[(byte >> 4) as usize] as char);
        value.push(DIGITS[(byte & 15) as usize] as char);
    }
    value
}

fn blob(bytes: Vec<u8>) -> Blob {
    Blob {
        len: bytes.len(),
        sha256: hex(&Sha256::digest(&bytes)),
    }
}

fn input(origin: [i64; 3], step: i64, extent: [u32; 3]) -> PhysicalVolumeRecipeInputV1 {
    PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: origin,
        cell_step_q32_32: step,
        extent,
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence: CellEvidenceV1::Vacuum,
        column_runs: Vec::new(),
    }
}

fn family(
    name: &'static str,
    recipe_input: PhysicalVolumeRecipeInputV1,
    start: [i64; 3],
    end: [i64; 3],
) -> Family {
    let recipe_input_bytes = recipe_input.to_bytes().unwrap();
    let recipe = compile_physical_volume_recipe(&recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    let query = PhysicalPathQueryV1 {
        schema_version: 1,
        physical_volume_id: volume.physical_volume_id,
        start_q32_32: start,
        end_q32_32: end,
    };
    let witness = compile_path_witness(&recipe, &volume, &query).unwrap();
    Family {
        name: name.to_owned(),
        recipe_input: blob(recipe_input_bytes),
        recipe: blob(recipe.to_bytes().unwrap()),
        physical_volume_recipe_id: hex(&recipe.physical_volume_recipe_id),
        volume: blob(volume.to_bytes(&recipe).unwrap()),
        occupancy_fingerprint: hex(&volume.occupancy_fingerprint),
        physical_volume_id: hex(&volume.physical_volume_id),
        query: blob(query.to_bytes(&recipe, &volume).unwrap()),
        witness: blob(witness.to_bytes(&recipe, &volume, &query).unwrap()),
        path_query_id: hex(&witness.path_query_id),
        path_witness_id: hex(&witness.path_witness_id),
    }
}

fn current_families() -> Vec<Family> {
    let max_origin = i64::MAX - 16;
    vec![
        family(
            "straight_face",
            input([0; 3], 4, [3, 1, 1]),
            [1, 1, 1],
            [11, 1, 1],
        ),
        family(
            "exact_reverse",
            input([0; 3], 4, [3, 1, 1]),
            [11, 1, 1],
            [1, 1, 1],
        ),
        family(
            "simultaneous_vertex",
            input([0; 3], 4, [3, 3, 3]),
            [1, 1, 1],
            [11, 11, 11],
        ),
        family(
            "stationary_point",
            input([0; 3], 4, [2, 2, 2]),
            [4, 4, 4],
            [4, 4, 4],
        ),
        family(
            "negative_near_maximum",
            input([max_origin, -12, -8], 4, [3, 2, 2]),
            [max_origin + 1, -11, -7],
            [i64::MAX - 5, -5, -1],
        ),
    ]
}

#[test]
fn exact_path_v1_bytes_and_ids_remain_locked() {
    let expected: Vec<Family> =
        serde_json::from_str(include_str!("../fixtures/exact_path_v1_identity_lock.json")).unwrap();
    assert_eq!(current_families(), expected);
}
