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
    project(&reference_scene())
}

pub fn negative_control_snapshots() -> Result<Vec<ControlledSnapshot>, ViewportError> {
    let mut broken = reference_scene();
    broken.scene_id = "control-broken-connection".into();
    broken
        .edges
        .retain(|edge| !(edge.from == "base_right" && edge.to == "spine_low"));

    let mut collapsed = reference_scene();
    collapsed.scene_id = "control-silhouette-collapse".into();
    for frame in &mut collapsed.frames {
        for vertex in &mut frame.vertices {
            if vertex.id == "arm_left" {
                vertex.x = -10;
            } else if vertex.id == "arm_right" {
                vertex.x = 10;
            }
        }
    }
    for vertex in &mut collapsed.vertices {
        if vertex.id == "arm_left" {
            vertex.x = -10;
        } else if vertex.id == "arm_right" {
            vertex.x = 10;
        }
    }

    let mut drifted = reference_scene();
    drifted.scene_id = "control-articulation-drift".into();
    for vertex in &mut drifted.frames[1].vertices {
        if vertex.id == "arm_left" {
            vertex.x = -360;
            vertex.y = -40;
        } else if vertex.id == "arm_right" {
            vertex.x = 360;
            vertex.y = -40;
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
        vertex("base_left", -180, -120, 0),
        vertex("base_right", 180, -120, 0),
        vertex("spine_low", 0, -80, 0),
        vertex("spine_high", 0, 120, 0),
        vertex("arm_left", -150, 40, 0),
        vertex("arm_right", 150, 40, 0),
        vertex("head", 0, 210, 0),
    ];
    let raised = vec![
        vertex("base_left", -180, -120, 0),
        vertex("base_right", 180, -120, 0),
        vertex("spine_low", 0, -80, 0),
        vertex("spine_high", 0, 120, 0),
        vertex("arm_left", -125, 145, 30),
        vertex("arm_right", 125, 145, -30),
        vertex("head", 0, 210, 0),
    ];
    ReferenceScene {
        schema_version: SCHEMA_VERSION,
        scene_id: "neutral-articulation-fixture".into(),
        artifact_id: "artifact-reference-viewport-001".into(),
        vertices: base.clone(),
        edges: vec![
            edge("base_left", "spine_low", "support"),
            edge("base_right", "spine_low", "support"),
            edge("spine_low", "spine_high", "structure"),
            edge("spine_high", "arm_left", "articulation"),
            edge("spine_high", "arm_right", "articulation"),
            edge("spine_high", "head", "structure"),
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
        assert_eq!((front.x, front.y), (-180, -120));
        assert_eq!((side.x, side.y), (0, -120));
        assert_eq!((top.x, top.y), (-180, 0));
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
}
