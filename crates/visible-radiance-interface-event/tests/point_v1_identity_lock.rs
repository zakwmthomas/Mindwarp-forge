use physical_path_substrate::{
    AdjacencyV1, BoundaryModeV1, CellEvidenceV1, CellIndex3V1, ColumnRunV1, CoordinateFrameV1, Id,
    PhysicalPathQueryV1, PhysicalVolumeRecipeInputV1, compile_physical_volume,
    compile_physical_volume_recipe,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use visible_radiance_interface_event::{
    FaceInteractionEvidenceV1, InterfaceModelV1, SmoothDielectricBandV1,
    VisibleRadianceInterfaceInputV1, compile_visible_radiance_interface_event,
};

const ONE: i64 = 1_i64 << 32;

fn id(value: u32) -> Id {
    let mut id = [0; 32];
    id[..4].copy_from_slice(&value.to_le_bytes());
    id[31] = 1;
    id
}

fn smooth(a: u64, b: u64) -> InterfaceModelV1 {
    InterfaceModelV1::SmoothLosslessUnpolarizedDielectric {
        bands_rgb: std::array::from_fn(|_| SmoothDielectricBandV1 {
            eta_a_q16_48: a,
            eta_b_q16_48: b,
        }),
    }
}

fn fixture(model: InterfaceModelV1, end: [i64; 3]) -> VisibleRadianceInterfaceInputV1 {
    let a = id(10);
    let b = id(11);
    let recipe_input = PhysicalVolumeRecipeInputV1 {
        schema_version: 1,
        recipe_source_id: id(1),
        scope_id: id(2),
        reconstruction_id: id(3),
        recipe_revision: 1,
        coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
        origin_q32_32: [0; 3],
        cell_step_q32_32: ONE,
        extent: [2, 2, 1],
        boundary_mode: BoundaryModeV1::BoundedAbsent,
        adjacency: AdjacencyV1::SharedFace6,
        default_evidence: CellEvidenceV1::Gas {
            substance_source_id: a,
        },
        column_runs: vec![ColumnRunV1 {
            x_index: 1,
            y_index: 0,
            z_start: 0,
            length: 1,
            evidence: CellEvidenceV1::Liquid {
                substance_source_id: b,
            },
        }],
    };
    let recipe = compile_physical_volume_recipe(&recipe_input).unwrap();
    let volume = compile_physical_volume(&recipe).unwrap();
    VisibleRadianceInterfaceInputV1 {
        schema_version: 1,
        profile_source_id: id(4),
        scope_id: id(2),
        reconstruction_id: id(3),
        profile_revision: 1,
        physical_volume_recipe_input: recipe_input,
        path_query: PhysicalPathQueryV1 {
            schema_version: 1,
            physical_volume_id: volume.physical_volume_id,
            start_q32_32: [ONE / 2, ONE / 2, ONE / 2],
            end_q32_32: end,
        },
        face_interaction: FaceInteractionEvidenceV1 {
            interaction_source_id: id(5),
            scope_id: id(2),
            reconstruction_id: id(3),
            interaction_revision: 1,
            cell_a: CellIndex3V1 { x: 0, y: 0, z: 0 },
            cell_b: CellIndex3V1 { x: 1, y: 0, z: 0 },
            medium_a: CellEvidenceV1::Gas {
                substance_source_id: a,
            },
            medium_b: CellEvidenceV1::Liquid {
                substance_source_id: b,
            },
            model,
        },
    }
}

#[derive(Serialize)]
struct LockedVector {
    name: &'static str,
    input_bytes: usize,
    input_bytes_sha256: String,
    event_bytes: usize,
    event_bytes_sha256: String,
    interface_input_id_hex: String,
    event_id_hex: String,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|value| format!("{value:02x}")).collect()
}

fn lock(name: &'static str, input: VisibleRadianceInterfaceInputV1) -> LockedVector {
    let input_bytes = input.to_bytes().unwrap();
    let event = compile_visible_radiance_interface_event(&input).unwrap();
    let event_bytes = event.to_bytes(&input).unwrap();
    LockedVector {
        name,
        input_bytes: input_bytes.len(),
        input_bytes_sha256: hex(&Sha256::digest(&input_bytes)),
        event_bytes: event_bytes.len(),
        event_bytes_sha256: hex(&Sha256::digest(&event_bytes)),
        interface_input_id_hex: hex(&event.interface_input_id),
        event_id_hex: hex(&event.event_id),
    }
}

#[test]
fn point_v1_canonical_bytes_and_ids_are_locked_before_interval_work() {
    let q48 = 1_u64 << 48;
    let normal = fixture(smooth(q48, 3 * q48 / 2), [3 * ONE / 2, ONE / 2, ONE / 2]);
    let index_match = fixture(smooth(q48, q48), [3 * ONE / 2, ONE, ONE / 2]);
    let mut reverse = index_match.clone();
    std::mem::swap(
        &mut reverse.path_query.start_q32_32,
        &mut reverse.path_query.end_q32_32,
    );
    let mut critical = fixture(
        smooth(5 * q48, 4 * q48),
        [5 * ONE / 4, 3 * ONE / 2, ONE / 2],
    );
    critical
        .physical_volume_recipe_input
        .column_runs
        .push(ColumnRunV1 {
            x_index: 1,
            y_index: 1,
            z_start: 0,
            length: 1,
            evidence: CellEvidenceV1::Liquid {
                substance_source_id: id(11),
            },
        });
    let critical_recipe =
        compile_physical_volume_recipe(&critical.physical_volume_recipe_input).unwrap();
    critical.path_query.physical_volume_id = compile_physical_volume(&critical_recipe)
        .unwrap()
        .physical_volume_id;
    critical.face_interaction.cell_a.y = 1;
    critical.face_interaction.cell_b.y = 1;
    let unsupported = fixture(
        InterfaceModelV1::Unsupported {
            model_source_id: id(99),
        },
        [3 * ONE / 2, ONE / 2, ONE / 2],
    );
    let vectors = [
        lock("normal_incidence", normal),
        lock("index_match", index_match),
        lock("reverse_direction", reverse),
        lock("critical_tir", critical),
        lock("unsupported_model", unsupported),
    ];
    let actual = serde_json::to_string_pretty(&vectors).unwrap() + "\n";
    let expected = include_str!("../fixtures/point_v1_identity_lock.json");
    assert_eq!(
        actual, expected,
        "replace the fixture only before interval source exists\n{actual}"
    );
}
