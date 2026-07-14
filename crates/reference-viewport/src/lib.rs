//! Deterministic, data-only projection for the built-in Forge reference viewport.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fmt};

pub const SCHEMA_VERSION: u16 = 1;
const MAX_COORDINATE: i32 = 4096;
const MAX_VERTICES: usize = 256;
const MAX_EDGES: usize = 512;
const MAX_FRAMES: usize = 32;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Vertex {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PoseFrame {
    pub index: u16,
    pub vertices: Vec<Vertex>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceScene {
    pub schema_version: u16,
    pub scene_id: String,
    pub artifact_id: String,
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub frames: Vec<PoseFrame>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewKind {
    Front,
    Side,
    Top,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProjectedPoint {
    pub id: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProjectedView {
    pub view: ViewKind,
    pub points: Vec<ProjectedPoint>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProjectedFrame {
    pub index: u16,
    pub views: Vec<ProjectedView>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ViewportSnapshot {
    pub schema_version: u16,
    pub mode: &'static str,
    pub renderer_profile: &'static str,
    pub scene_fingerprint: String,
    pub read_only: bool,
    pub edges: Vec<Edge>,
    pub frames: Vec<ProjectedFrame>,
    pub limitations: Vec<&'static str>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NegativeControlKind {
    BrokenConnection,
    SilhouetteCollapse,
    ArticulationDrift,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ControlledSnapshot {
    pub control: NegativeControlKind,
    pub snapshot: ViewportSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewportError {
    Invalid(&'static str),
    Codec(String),
}

impl fmt::Display for ViewportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ViewportError {}

pub fn decode_scene(bytes: &[u8]) -> Result<ReferenceScene, ViewportError> {
    let scene: ReferenceScene =
        serde_json::from_slice(bytes).map_err(|error| ViewportError::Codec(error.to_string()))?;
    validate_scene(&scene)?;
    Ok(scene)
}

pub fn validate_scene(scene: &ReferenceScene) -> Result<(), ViewportError> {
    if scene.schema_version != SCHEMA_VERSION {
        return Err(ViewportError::Invalid("unsupported schema version"));
    }
    validate_token(&scene.scene_id)?;
    validate_token(&scene.artifact_id)?;
    if !(3..=MAX_VERTICES).contains(&scene.vertices.len()) {
        return Err(ViewportError::Invalid("vertex count outside bounds"));
    }
    if !(1..=MAX_EDGES).contains(&scene.edges.len()) {
        return Err(ViewportError::Invalid("edge count outside bounds"));
    }
    if scene.frames.is_empty() || scene.frames.len() > MAX_FRAMES {
        return Err(ViewportError::Invalid("frame count outside bounds"));
    }

    let base_ids = validate_vertices(&scene.vertices)?;
    let mut edge_keys = BTreeSet::new();
    for edge in &scene.edges {
        validate_token(&edge.role)?;
        if edge.from == edge.to || !base_ids.contains(&edge.from) || !base_ids.contains(&edge.to) {
            return Err(ViewportError::Invalid("edge endpoint is invalid"));
        }
        let key = if edge.from < edge.to {
            (edge.from.as_str(), edge.to.as_str())
        } else {
            (edge.to.as_str(), edge.from.as_str())
        };
        if !edge_keys.insert(key) {
            return Err(ViewportError::Invalid("duplicate edge"));
        }
    }

    for (expected_index, frame) in scene.frames.iter().enumerate() {
        if usize::from(frame.index) != expected_index {
            return Err(ViewportError::Invalid("frame indexes must be contiguous"));
        }
        if validate_vertices(&frame.vertices)? != base_ids {
            return Err(ViewportError::Invalid("frame vertex identity drift"));
        }
    }
    Ok(())
}

pub fn project(scene: &ReferenceScene) -> Result<ViewportSnapshot, ViewportError> {
    validate_scene(scene)?;
    let canonical =
        serde_json::to_vec(scene).map_err(|error| ViewportError::Codec(error.to_string()))?;
    let fingerprint = Sha256::digest(&canonical)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let frames = scene
        .frames
        .iter()
        .map(|frame| ProjectedFrame {
            index: frame.index,
            views: [ViewKind::Front, ViewKind::Side, ViewKind::Top]
                .into_iter()
                .map(|view| ProjectedView {
                    view,
                    points: frame
                        .vertices
                        .iter()
                        .map(|vertex| {
                            let (x, y) = match view {
                                ViewKind::Front => (vertex.x, vertex.y),
                                ViewKind::Side => (vertex.z, vertex.y),
                                ViewKind::Top => (vertex.x, vertex.z),
                            };
                            ProjectedPoint {
                                id: vertex.id.clone(),
                                x,
                                y,
                            }
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect();
    Ok(ViewportSnapshot {
        schema_version: SCHEMA_VERSION,
        mode: "built-in-data-only",
        renderer_profile: "forge-wireframe-orthographic-v1",
        scene_fingerprint: fingerprint,
        read_only: true,
        edges: scene.edges.clone(),
        frames,
        limitations: vec![
            "Reference wireframe only; no material, lighting, physics, or production-runtime claim.",
            "Accepts typed Forge scene data only; no files, scripts, plugins, URLs, or executable payloads.",
            "Human review may describe this exact fixture but cannot establish general player preference.",
        ],
    })
}

pub fn reference_snapshot() -> Result<ViewportSnapshot, ViewportError> {
    let scene = reference_scene();
    validate_reference_fixture_semantics(&scene)?;
    project(&scene)
}

pub fn negative_control_snapshots() -> Result<Vec<ControlledSnapshot>, ViewportError> {
    let reference = reference_scene();
    validate_reference_fixture_semantics(&reference)?;

    let mut broken = reference.clone();
    broken.scene_id = "control-broken-connection".into();
    broken
        .edges
        .retain(|edge| !(edge.from == "knee_right" && edge.to == "ankle_right"));

    let mut collapsed = reference.clone();
    collapsed.scene_id = "control-silhouette-collapse".into();
    for frame in &mut collapsed.frames {
        for vertex in &mut frame.vertices {
            match vertex.id.as_str() {
                "elbow_left" => vertex.x = -12,
                "hand_left" => vertex.x = -18,
                "elbow_right" => vertex.x = 12,
                "hand_right" => vertex.x = 18,
                _ => {}
            }
        }
    }
    for vertex in &mut collapsed.vertices {
        match vertex.id.as_str() {
            "elbow_left" => vertex.x = -12,
            "hand_left" => vertex.x = -18,
            "elbow_right" => vertex.x = 12,
            "hand_right" => vertex.x = 18,
            _ => {}
        }
    }

    let mut drifted = reference.clone();
    drifted.scene_id = "control-articulation-drift".into();
    for vertex in &mut drifted.frames[1].vertices {
        if vertex.id == "hand_left" {
            vertex.x = 180;
            vertex.y = 210;
        } else if vertex.id == "hand_right" {
            vertex.x = -180;
            vertex.y = 210;
        }
    }

    Ok(vec![
        ControlledSnapshot {
            control: NegativeControlKind::BrokenConnection,
            snapshot: project(&broken)?,
        },
        ControlledSnapshot {
            control: NegativeControlKind::SilhouetteCollapse,
            snapshot: project(&collapsed)?,
        },
        ControlledSnapshot {
            control: NegativeControlKind::ArticulationDrift,
            snapshot: project(&drifted)?,
        },
    ])
}

pub fn reference_scene() -> ReferenceScene {
    let base = vec![
        vertex("pelvis", 0, -70, 0),
        vertex("chest", 0, 90, 0),
        vertex("shoulder_left", 60, 90, 0),
        vertex("elbow_left", 180, 90, 0),
        vertex("hand_left", 300, 90, 0),
        vertex("shoulder_right", -60, 90, 0),
        vertex("elbow_right", -180, 90, 0),
        vertex("hand_right", -300, 90, 0),
        vertex("hip_left", 45, -70, 0),
        vertex("knee_left", 45, -190, 0),
        vertex("ankle_left", 45, -310, 0),
        vertex("toe_left", 45, -310, 60),
        vertex("hip_right", -45, -70, 0),
        vertex("knee_right", -45, -190, 0),
        vertex("ankle_right", -45, -310, 0),
        vertex("toe_right", -45, -310, 60),
        vertex("head", 0, 190, 0),
    ];
    let raised = vec![
        vertex("pelvis", 0, -70, 0),
        vertex("chest", 0, 90, 0),
        vertex("shoulder_left", 60, 90, 0),
        vertex("elbow_left", 180, 90, 0),
        vertex("hand_left", 180, -30, 0),
        vertex("shoulder_right", -60, 90, 0),
        vertex("elbow_right", -180, 90, 0),
        vertex("hand_right", -180, -30, 0),
        vertex("hip_left", 45, -70, 0),
        vertex("knee_left", 45, -190, 0),
        vertex("ankle_left", 45, -310, 0),
        vertex("toe_left", 45, -310, 60),
        vertex("hip_right", -45, -70, 0),
        vertex("knee_right", -45, -190, 0),
        vertex("ankle_right", -45, -310, 0),
        vertex("toe_right", -45, -310, 60),
        vertex("head", 0, 190, 0),
    ];
    ReferenceScene {
        schema_version: SCHEMA_VERSION,
        scene_id: "neutral-t-pose-articulation-fixture-v3".into(),
        artifact_id: "artifact-reference-viewport-003".into(),
        vertices: base.clone(),
        edges: vec![
            edge("pelvis", "chest", "structure"),
            edge("chest", "head", "structure"),
            edge("chest", "shoulder_left", "structure"),
            edge("shoulder_left", "elbow_left", "articulation"),
            edge("elbow_left", "hand_left", "articulation"),
            edge("chest", "shoulder_right", "structure"),
            edge("shoulder_right", "elbow_right", "articulation"),
            edge("elbow_right", "hand_right", "articulation"),
            edge("pelvis", "hip_left", "structure"),
            edge("hip_left", "knee_left", "support"),
            edge("knee_left", "ankle_left", "support"),
            edge("ankle_left", "toe_left", "orientation"),
            edge("pelvis", "hip_right", "structure"),
            edge("hip_right", "knee_right", "support"),
            edge("knee_right", "ankle_right", "support"),
            edge("ankle_right", "toe_right", "orientation"),
        ],
        frames: vec![
            PoseFrame {
                index: 0,
                vertices: base,
            },
            PoseFrame {
                index: 1,
                vertices: raised,
            },
        ],
    }
}

pub fn validate_reference_fixture_semantics(scene: &ReferenceScene) -> Result<(), ViewportError> {
    validate_scene(scene)?;
    if scene.scene_id != "neutral-t-pose-articulation-fixture-v3"
        || scene.artifact_id != "artifact-reference-viewport-003"
        || scene.frames.len() != 2
    {
        return Err(ViewportError::Invalid(
            "unsupported reference fixture profile",
        ));
    }

    let segments = [
        ("shoulder_left", "elbow_left"),
        ("elbow_left", "hand_left"),
        ("shoulder_right", "elbow_right"),
        ("elbow_right", "hand_right"),
        ("hip_left", "knee_left"),
        ("knee_left", "ankle_left"),
        ("hip_right", "knee_right"),
        ("knee_right", "ankle_right"),
    ];
    for (from, to) in segments {
        let base_length = squared_distance(&scene.vertices, from, to)?;
        if base_length != 14_400 {
            return Err(ViewportError::Invalid(
                "reference limb segment is not the declared v3 length",
            ));
        }
        for frame in &scene.frames {
            if squared_distance(&frame.vertices, from, to)? != base_length {
                return Err(ViewportError::Invalid(
                    "reference limb length drifts across pose frames",
                ));
            }
        }
    }
    validate_t_pose_rest_geometry(scene)?;
    validate_joint_hierarchy(scene)?;
    Ok(())
}

fn validate_t_pose_rest_geometry(scene: &ReferenceScene) -> Result<(), ViewportError> {
    if scene.vertices != scene.frames[0].vertices {
        return Err(ViewportError::Invalid(
            "frame zero must equal the declared rest bind pose",
        ));
    }
    let rest = &scene.vertices;
    let point = |id: &str| {
        rest.iter()
            .find(|vertex| vertex.id == id)
            .ok_or(ViewportError::Invalid("required fixture joint is missing"))
    };
    let shoulder_left = point("shoulder_left")?;
    let elbow_left = point("elbow_left")?;
    let hand_left = point("hand_left")?;
    let shoulder_right = point("shoulder_right")?;
    let elbow_right = point("elbow_right")?;
    let hand_right = point("hand_right")?;
    if ![
        shoulder_left,
        elbow_left,
        hand_left,
        shoulder_right,
        elbow_right,
        hand_right,
    ]
    .iter()
    .all(|joint| joint.y == 90 && joint.z == 0)
    {
        return Err(ViewportError::Invalid(
            "rest arms must be collinear on the x axis",
        ));
    }
    if !(shoulder_left.x > 0
        && elbow_left.x > shoulder_left.x
        && hand_left.x > elbow_left.x
        && shoulder_right.x < 0
        && elbow_right.x < shoulder_right.x
        && hand_right.x < elbow_right.x)
    {
        return Err(ViewportError::Invalid(
            "anatomical left must be positive x in the rest pose",
        ));
    }
    for side in ["left", "right"] {
        let hip = point(&format!("hip_{side}"))?;
        let knee = point(&format!("knee_{side}"))?;
        let ankle = point(&format!("ankle_{side}"))?;
        let toe = point(&format!("toe_{side}"))?;
        if hip.x != knee.x
            || knee.x != ankle.x
            || hip.z != 0
            || knee.z != 0
            || ankle.z != 0
            || !(hip.y > knee.y && knee.y > ankle.y)
        {
            return Err(ViewportError::Invalid(
                "rest legs must be straight vertical and parallel",
            ));
        }
        if toe.x != ankle.x || toe.y != ankle.y || toe.z <= ankle.z {
            return Err(ViewportError::Invalid(
                "rest toes must point forward along positive z",
            ));
        }
    }
    for role in ["shoulder", "elbow", "hand", "hip", "knee", "ankle", "toe"] {
        let left = point(&format!("{role}_left"))?;
        let right = point(&format!("{role}_right"))?;
        if left.x != -right.x || left.y != right.y || left.z != right.z {
            return Err(ViewportError::Invalid(
                "rest pose must be bilaterally symmetric",
            ));
        }
    }
    Ok(())
}

fn validate_joint_hierarchy(scene: &ReferenceScene) -> Result<(), ViewportError> {
    let required = [
        ("pelvis", "chest"),
        ("chest", "head"),
        ("chest", "shoulder_left"),
        ("shoulder_left", "elbow_left"),
        ("elbow_left", "hand_left"),
        ("chest", "shoulder_right"),
        ("shoulder_right", "elbow_right"),
        ("elbow_right", "hand_right"),
        ("pelvis", "hip_left"),
        ("hip_left", "knee_left"),
        ("knee_left", "ankle_left"),
        ("ankle_left", "toe_left"),
        ("pelvis", "hip_right"),
        ("hip_right", "knee_right"),
        ("knee_right", "ankle_right"),
        ("ankle_right", "toe_right"),
    ];
    if scene.edges.len() != required.len()
        || required.iter().any(|(from, to)| {
            !scene
                .edges
                .iter()
                .any(|edge| edge.from == *from && edge.to == *to)
        })
    {
        return Err(ViewportError::Invalid(
            "fixture joints must retain the directed pelvis-rooted hierarchy",
        ));
    }
    Ok(())
}

fn squared_distance(vertices: &[Vertex], from: &str, to: &str) -> Result<i64, ViewportError> {
    let from = vertices
        .iter()
        .find(|vertex| vertex.id == from)
        .ok_or(ViewportError::Invalid("required fixture joint is missing"))?;
    let to = vertices
        .iter()
        .find(|vertex| vertex.id == to)
        .ok_or(ViewportError::Invalid("required fixture joint is missing"))?;
    let dx = i64::from(to.x - from.x);
    let dy = i64::from(to.y - from.y);
    let dz = i64::from(to.z - from.z);
    Ok(dx * dx + dy * dy + dz * dz)
}

fn validate_vertices(vertices: &[Vertex]) -> Result<BTreeSet<String>, ViewportError> {
    let mut ids = BTreeSet::new();
    for vertex in vertices {
        validate_token(&vertex.id)?;
        if [vertex.x, vertex.y, vertex.z]
            .into_iter()
            .any(|coordinate| coordinate.abs() > MAX_COORDINATE)
        {
            return Err(ViewportError::Invalid("coordinate outside bounds"));
        }
        if !ids.insert(vertex.id.clone()) {
            return Err(ViewportError::Invalid("duplicate vertex id"));
        }
    }
    Ok(ids)
}

fn validate_token(value: &str) -> Result<(), ViewportError> {
    if value.is_empty()
        || value.len() > 80
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(ViewportError::Invalid(
            "identifier is not a bounded inert token",
        ));
    }
    Ok(())
}

fn vertex(id: &str, x: i32, y: i32, z: i32) -> Vertex {
    Vertex {
        id: id.into(),
        x,
        y,
        z,
    }
}

fn edge(from: &str, to: &str, role: &str) -> Edge {
    Edge {
        from: from.into(),
        to: to.into(),
        role: role.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_projection_is_deterministic_and_read_only() {
        let first = reference_snapshot().unwrap();
        let second = reference_snapshot().unwrap();
        assert_eq!(first, second);
        assert!(first.read_only);
        assert_eq!(first.frames.len(), 2);
        assert_eq!(first.frames[0].views.len(), 3);
    }

    #[test]
    fn unknown_fields_and_executable_shaped_identifiers_fail_closed() {
        let mut value = serde_json::to_value(reference_scene()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("script".into(), "run-me".into());
        assert!(decode_scene(&serde_json::to_vec(&value).unwrap()).is_err());
        let mut scene = reference_scene();
        scene.artifact_id = "file:///run.exe".into();
        assert_eq!(
            validate_scene(&scene),
            Err(ViewportError::Invalid(
                "identifier is not a bounded inert token"
            ))
        );
    }

    #[test]
    fn path_network_and_markup_tokens_are_rejected() {
        for hostile in ["../escape", "C:\\tool.exe", "https://host", "<script>"] {
            let mut scene = reference_scene();
            scene.scene_id = hostile.into();
            assert!(validate_scene(&scene).is_err());
        }
    }

    #[test]
    fn coordinate_and_cardinality_budgets_fail_closed() {
        let mut scene = reference_scene();
        scene.vertices[0].x = MAX_COORDINATE + 1;
        assert!(validate_scene(&scene).is_err());
        let mut scene = reference_scene();
        scene.frames.clear();
        assert!(validate_scene(&scene).is_err());
    }

    #[test]
    fn duplicate_and_dangling_edges_fail_closed() {
        let mut scene = reference_scene();
        scene.edges.push(scene.edges[0].clone());
        assert!(validate_scene(&scene).is_err());
        let mut scene = reference_scene();
        scene.edges[0].to = "missing".into();
        assert!(validate_scene(&scene).is_err());
    }

    #[test]
    fn frame_identity_drift_and_gaps_fail_closed() {
        let mut scene = reference_scene();
        scene.frames[1].vertices[0].id = "replacement".into();
        assert!(validate_scene(&scene).is_err());
        let mut scene = reference_scene();
        scene.frames[1].index = 3;
        assert!(validate_scene(&scene).is_err());
    }

    #[test]
    fn orthographic_axes_are_explicit() {
        let snapshot = reference_snapshot().unwrap();
        let front = &snapshot.frames[0].views[0].points[0];
        let side = &snapshot.frames[0].views[1].points[0];
        let top = &snapshot.frames[0].views[2].points[0];
        assert_eq!((front.x, front.y), (0, -70));
        assert_eq!((side.x, side.y), (0, -70));
        assert_eq!((top.x, top.y), (0, 0));
    }

    #[test]
    fn negative_controls_are_valid_distinct_and_repeatable() {
        let first = negative_control_snapshots().unwrap();
        let second = negative_control_snapshots().unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), 3);
        let fingerprints: BTreeSet<_> = first
            .iter()
            .map(|control| control.snapshot.scene_fingerprint.as_str())
            .collect();
        assert_eq!(fingerprints.len(), 3);
        assert!(!fingerprints.contains(reference_snapshot().unwrap().scene_fingerprint.as_str()));
    }

    #[test]
    fn v3_fixture_has_legible_segmented_limbs_with_stable_lengths() {
        let scene = reference_scene();
        validate_reference_fixture_semantics(&scene).unwrap();
        assert_eq!(scene.scene_id, "neutral-t-pose-articulation-fixture-v3");
        assert_eq!(scene.artifact_id, "artifact-reference-viewport-003");
        for frame in &scene.frames {
            for (from, to) in [
                ("shoulder_left", "elbow_left"),
                ("elbow_left", "hand_left"),
                ("hip_left", "knee_left"),
                ("knee_left", "ankle_left"),
            ] {
                assert_eq!(squared_distance(&frame.vertices, from, to).unwrap(), 14_400);
            }
        }
    }

    #[test]
    fn short_reference_limbs_fail_the_v3_semantic_gate() {
        let mut scene = reference_scene();
        for vertices in std::iter::once(&mut scene.vertices)
            .chain(scene.frames.iter_mut().map(|frame| &mut frame.vertices))
        {
            vertices
                .iter_mut()
                .find(|vertex| vertex.id == "hand_left")
                .unwrap()
                .x += 48;
        }
        assert_eq!(
            validate_reference_fixture_semantics(&scene),
            Err(ViewportError::Invalid(
                "reference limb segment is not the declared v3 length"
            ))
        );
    }

    #[test]
    fn pose_length_drift_fails_the_v3_semantic_gate() {
        let mut scene = reference_scene();
        scene.frames[1]
            .vertices
            .iter_mut()
            .find(|vertex| vertex.id == "hand_right")
            .unwrap()
            .x += 120;
        assert_eq!(
            validate_reference_fixture_semantics(&scene),
            Err(ViewportError::Invalid(
                "reference limb length drifts across pose frames"
            ))
        );
    }

    #[test]
    fn articulation_control_changes_pose_without_changing_segment_length() {
        let reference = reference_scene();
        let reference_frame = &reference.frames[1].vertices;
        let control = negative_control_snapshots()
            .unwrap()
            .into_iter()
            .find(|item| item.control == NegativeControlKind::ArticulationDrift)
            .unwrap();
        let control_frame = &control.snapshot.frames[1].views[0].points;
        for side in ["left", "right"] {
            let elbow = format!("elbow_{side}");
            let hand = format!("hand_{side}");
            let reference_length = squared_distance(reference_frame, &elbow, &hand).unwrap();
            let elbow_point = control_frame
                .iter()
                .find(|point| point.id == elbow)
                .unwrap();
            let hand_point = control_frame.iter().find(|point| point.id == hand).unwrap();
            let dx = i64::from(hand_point.x - elbow_point.x);
            let dy = i64::from(hand_point.y - elbow_point.y);
            assert_eq!(dx * dx + dy * dy, reference_length);
            let reference_hand = reference_frame
                .iter()
                .find(|point| point.id == hand)
                .unwrap();
            assert_ne!(
                (hand_point.x, hand_point.y),
                (reference_hand.x, reference_hand.y)
            );
        }
    }
}
