//! Capability-free finite sampling-domain reference over Forge Q32.32 fields.
//!
//! This crate does not model a planet, sphere, projection, terrain, physical
//! partition, biome, runtime map, or storage layout. V1 is deliberately a
//! bounded, non-wrapping rectified grid whose cells, coordinates, identities
//! and shared-edge neighbours are always reconstructed from one strict input.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
pub const COORDINATE_FRACTION_BITS: u32 = field_basis::COORD_FRAC;
pub const MAX_PROOF_CELLS: u64 = 1_000_000;
pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateFrame {
    FieldQ32_32Cartesian2dV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Adjacency {
    SharedEdge4,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryMode {
    BoundedAbsent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialDomainInput {
    pub schema_version: u16,
    pub logical_world_id: Id,
    pub reconstruction_id: Id,
    pub coordinate_frame: CoordinateFrame,
    pub cell_center_origin_q32_32: [i64; 2],
    pub cell_step_q32_32: [u64; 2],
    pub cell_count: [u32; 2],
    pub adjacency: Adjacency,
    pub boundary_mode: BoundaryMode,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialDomain {
    pub schema_version: u16,
    pub spatial_domain_id: Id,
    pub input: SpatialDomainInput,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellIndex {
    pub schema_version: u16,
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialCell {
    pub schema_version: u16,
    pub spatial_domain_id: Id,
    pub index: CellIndex,
    pub cell_id: Id,
    pub sample_coordinate_q32_32: [i64; 2],
    pub neighbour_indices: Vec<CellIndex>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialDomainError {
    Invalid(&'static str),
    Codec(String),
}

impl CellIndex {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            schema_version: CONTRACT_VERSION,
            x,
            y,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpatialDomainError> {
        validate_index_schema(self)?;
        serde_json::to_vec(self).map_err(|error| SpatialDomainError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SpatialDomainError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| SpatialDomainError::Codec(error.to_string()))?;
        validate_index_schema(&value)?;
        if value.to_bytes()? != bytes {
            return Err(SpatialDomainError::Invalid("noncanonical cell-index bytes"));
        }
        Ok(value)
    }
}

impl SpatialDomainInput {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SpatialDomainError> {
        validate_input(self)?;
        serde_json::to_vec(self).map_err(|error| SpatialDomainError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SpatialDomainError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| SpatialDomainError::Codec(error.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(SpatialDomainError::Invalid(
                "noncanonical spatial-domain input bytes",
            ));
        }
        Ok(value)
    }
}

impl SpatialDomain {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SpatialDomainError> {
        validate_spatial_domain(self)?;
        serde_json::to_vec(self).map_err(|error| SpatialDomainError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SpatialDomainError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| SpatialDomainError::Codec(error.to_string()))?;
        validate_spatial_domain(&value)?;
        if value.to_bytes()? != bytes {
            return Err(SpatialDomainError::Invalid(
                "noncanonical spatial-domain bytes",
            ));
        }
        Ok(value)
    }
}

impl SpatialCell {
    pub fn to_bytes(&self, domain: &SpatialDomain) -> Result<Vec<u8>, SpatialDomainError> {
        validate_spatial_cell(domain, self)?;
        serde_json::to_vec(self).map_err(|error| SpatialDomainError::Codec(error.to_string()))
    }

    pub fn from_bytes(domain: &SpatialDomain, bytes: &[u8]) -> Result<Self, SpatialDomainError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| SpatialDomainError::Codec(error.to_string()))?;
        validate_spatial_cell(domain, &value)?;
        if value.to_bytes(domain)? != bytes {
            return Err(SpatialDomainError::Invalid("noncanonical cell bytes"));
        }
        Ok(value)
    }
}

pub fn compile_spatial_domain(
    input: &SpatialDomainInput,
) -> Result<SpatialDomain, SpatialDomainError> {
    let input_bytes = input.to_bytes()?;
    Ok(SpatialDomain {
        schema_version: CONTRACT_VERSION,
        spatial_domain_id: hash(b"mindwarp.spatial-domain.v1", &input_bytes),
        input: input.clone(),
        limitations: domain_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_spatial_domain(domain: &SpatialDomain) -> Result<(), SpatialDomainError> {
    if domain.schema_version != CONTRACT_VERSION {
        return Err(SpatialDomainError::Invalid("unsupported domain schema"));
    }
    if domain.limitations != domain_limitations() || domain.authority_effect != "none_evidence_only"
    {
        return Err(SpatialDomainError::Invalid(
            "domain claim or authority drift",
        ));
    }
    if compile_spatial_domain(&domain.input)? != *domain {
        return Err(SpatialDomainError::Invalid("spatial-domain identity drift"));
    }
    Ok(())
}

pub fn sample_coordinate(
    domain: &SpatialDomain,
    index: CellIndex,
) -> Result<[i64; 2], SpatialDomainError> {
    validate_spatial_domain(domain)?;
    validate_index(&domain.input, index)?;
    coordinate_for(&domain.input, index)
}

pub fn neighbour_indices(
    domain: &SpatialDomain,
    index: CellIndex,
) -> Result<Vec<CellIndex>, SpatialDomainError> {
    validate_spatial_domain(domain)?;
    validate_index(&domain.input, index)?;
    let mut neighbours = Vec::with_capacity(4);
    if index.x > 0 {
        neighbours.push(CellIndex::new(index.x - 1, index.y));
    }
    if index.x + 1 < domain.input.cell_count[0] {
        neighbours.push(CellIndex::new(index.x + 1, index.y));
    }
    if index.y > 0 {
        neighbours.push(CellIndex::new(index.x, index.y - 1));
    }
    if index.y + 1 < domain.input.cell_count[1] {
        neighbours.push(CellIndex::new(index.x, index.y + 1));
    }
    Ok(neighbours)
}

pub fn build_spatial_cell(
    domain: &SpatialDomain,
    index: CellIndex,
) -> Result<SpatialCell, SpatialDomainError> {
    validate_spatial_domain(domain)?;
    let index_bytes = index.to_bytes()?;
    validate_index(&domain.input, index)?;
    let mut identity_bytes = Vec::with_capacity(32 + index_bytes.len());
    identity_bytes.extend_from_slice(&domain.spatial_domain_id);
    identity_bytes.extend_from_slice(&index_bytes);
    Ok(SpatialCell {
        schema_version: CONTRACT_VERSION,
        spatial_domain_id: domain.spatial_domain_id,
        index,
        cell_id: hash(b"mindwarp.spatial-cell.v1", &identity_bytes),
        sample_coordinate_q32_32: coordinate_for(&domain.input, index)?,
        neighbour_indices: neighbour_indices(domain, index)?,
        limitations: cell_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_spatial_cell(
    domain: &SpatialDomain,
    cell: &SpatialCell,
) -> Result<(), SpatialDomainError> {
    validate_cell_shape(cell)?;
    if build_spatial_cell(domain, cell.index)? != *cell {
        return Err(SpatialDomainError::Invalid(
            "spatial-cell reconstruction drift",
        ));
    }
    Ok(())
}

fn validate_input(input: &SpatialDomainInput) -> Result<(), SpatialDomainError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(SpatialDomainError::Invalid("unsupported input schema"));
    }
    if input.logical_world_id == [0; 32] || input.reconstruction_id == [0; 32] {
        return Err(SpatialDomainError::Invalid(
            "missing world identity binding",
        ));
    }
    if input.coordinate_frame != CoordinateFrame::FieldQ32_32Cartesian2dV1
        || input.adjacency != Adjacency::SharedEdge4
        || input.boundary_mode != BoundaryMode::BoundedAbsent
    {
        return Err(SpatialDomainError::Invalid("unsupported spatial semantics"));
    }
    if input.cell_step_q32_32.contains(&0)
        || input
            .cell_step_q32_32
            .iter()
            .any(|step| *step > i64::MAX as u64)
    {
        return Err(SpatialDomainError::Invalid("invalid cell step"));
    }
    if input.cell_count.contains(&0) {
        return Err(SpatialDomainError::Invalid("empty spatial domain"));
    }
    let proof_cells = u64::from(input.cell_count[0])
        .checked_mul(u64::from(input.cell_count[1]))
        .ok_or(SpatialDomainError::Invalid("cell-count overflow"))?;
    if proof_cells > MAX_PROOF_CELLS {
        return Err(SpatialDomainError::Invalid("proof cell ceiling exceeded"));
    }
    let furthest = CellIndex::new(input.cell_count[0] - 1, input.cell_count[1] - 1);
    coordinate_for(input, furthest)?;
    Ok(())
}

fn validate_index_schema(index: &CellIndex) -> Result<(), SpatialDomainError> {
    if index.schema_version != CONTRACT_VERSION {
        return Err(SpatialDomainError::Invalid("unsupported cell-index schema"));
    }
    Ok(())
}

fn validate_index(input: &SpatialDomainInput, index: CellIndex) -> Result<(), SpatialDomainError> {
    validate_index_schema(&index)?;
    if index.x >= input.cell_count[0] || index.y >= input.cell_count[1] {
        return Err(SpatialDomainError::Invalid("cell index out of bounds"));
    }
    Ok(())
}

fn coordinate_for(
    input: &SpatialDomainInput,
    index: CellIndex,
) -> Result<[i64; 2], SpatialDomainError> {
    let indices = [index.x, index.y];
    let mut coordinate = [0_i64; 2];
    for axis in 0..2 {
        let value = i128::from(input.cell_center_origin_q32_32[axis])
            .checked_add(
                i128::from(indices[axis])
                    .checked_mul(i128::from(input.cell_step_q32_32[axis]))
                    .ok_or(SpatialDomainError::Invalid("sample-coordinate overflow"))?,
            )
            .ok_or(SpatialDomainError::Invalid("sample-coordinate overflow"))?;
        coordinate[axis] = i64::try_from(value)
            .map_err(|_| SpatialDomainError::Invalid("sample-coordinate overflow"))?;
    }
    Ok(coordinate)
}

fn validate_cell_shape(cell: &SpatialCell) -> Result<(), SpatialDomainError> {
    if cell.schema_version != CONTRACT_VERSION {
        return Err(SpatialDomainError::Invalid("unsupported cell schema"));
    }
    validate_index_schema(&cell.index)?;
    for neighbour in &cell.neighbour_indices {
        validate_index_schema(neighbour)?;
    }
    if cell.limitations != cell_limitations() || cell.authority_effect != "none_evidence_only" {
        return Err(SpatialDomainError::Invalid("cell claim or authority drift"));
    }
    Ok(())
}

fn domain_limitations() -> Vec<String> {
    [
        "bounded rectified sampling domain only; not a planet sphere projection terrain or runtime map",
        "bounded-absent edges never wrap and shared-edge adjacency excludes diagonal contact",
        "proof cell ceiling is a validation-cost guard not a canonical production world size",
        "no physical partition biome ecology visibility traversability approval promotion or runtime authority",
    ]
    .map(String::from)
    .to_vec()
}

fn cell_limitations() -> Vec<String> {
    [
        "cell coordinate and neighbours are reconstructed from the exact bounded domain",
        "cell identity does not imply terrain region biome habitability or world-shape geometry",
        "no approval promotion persistence runtime or external capability authority",
    ]
    .map(String::from)
    .to_vec()
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update([0]);
    hasher.update(bytes);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> SpatialDomainInput {
        SpatialDomainInput {
            schema_version: CONTRACT_VERSION,
            logical_world_id: [1; 32],
            reconstruction_id: [2; 32],
            coordinate_frame: CoordinateFrame::FieldQ32_32Cartesian2dV1,
            cell_center_origin_q32_32: [
                -(2_i64 << COORDINATE_FRACTION_BITS),
                3_i64 << COORDINATE_FRACTION_BITS,
            ],
            cell_step_q32_32: [
                1_u64 << COORDINATE_FRACTION_BITS,
                2_u64 << COORDINATE_FRACTION_BITS,
            ],
            cell_count: [3, 2],
            adjacency: Adjacency::SharedEdge4,
            boundary_mode: BoundaryMode::BoundedAbsent,
        }
    }

    #[test]
    fn descriptor_and_cells_replay_strictly() {
        let input = input();
        let domain = compile_spatial_domain(&input).unwrap();
        assert_eq!(
            SpatialDomainInput::from_bytes(&input.to_bytes().unwrap()).unwrap(),
            input
        );
        assert_eq!(
            SpatialDomain::from_bytes(&domain.to_bytes().unwrap()).unwrap(),
            domain
        );
        let cell = build_spatial_cell(&domain, CellIndex::new(1, 1)).unwrap();
        assert_eq!(
            SpatialCell::from_bytes(&domain, &cell.to_bytes(&domain).unwrap()).unwrap(),
            cell
        );
        assert!(validate_spatial_cell(&domain, &cell).is_ok());
    }

    #[test]
    fn coordinates_are_checked_exact_and_signed() {
        let domain = compile_spatial_domain(&input()).unwrap();
        assert_eq!(
            sample_coordinate(&domain, CellIndex::new(0, 0)).unwrap(),
            [
                -(2_i64 << COORDINATE_FRACTION_BITS),
                3_i64 << COORDINATE_FRACTION_BITS
            ]
        );
        assert_eq!(
            sample_coordinate(&domain, CellIndex::new(2, 1)).unwrap(),
            [0, 5_i64 << COORDINATE_FRACTION_BITS]
        );
    }

    #[test]
    fn neighbours_are_ordered_edge_only_and_never_wrap() {
        let domain = compile_spatial_domain(&input()).unwrap();
        assert_eq!(
            neighbour_indices(&domain, CellIndex::new(1, 0)).unwrap(),
            vec![
                CellIndex::new(0, 0),
                CellIndex::new(2, 0),
                CellIndex::new(1, 1)
            ]
        );
        assert_eq!(
            neighbour_indices(&domain, CellIndex::new(0, 0)).unwrap(),
            vec![CellIndex::new(1, 0), CellIndex::new(0, 1)]
        );
        assert!(
            !neighbour_indices(&domain, CellIndex::new(0, 0))
                .unwrap()
                .contains(&CellIndex::new(2, 0))
        );
    }

    #[test]
    fn one_cell_domain_has_no_neighbours() {
        let mut one = input();
        one.cell_count = [1, 1];
        let domain = compile_spatial_domain(&one).unwrap();
        assert!(
            neighbour_indices(&domain, CellIndex::new(0, 0))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn descriptor_changes_rekey_domain_and_cells() {
        let first = compile_spatial_domain(&input()).unwrap();
        let mut changed = input();
        changed.cell_step_q32_32[0] += 1;
        let second = compile_spatial_domain(&changed).unwrap();
        assert_ne!(first.spatial_domain_id, second.spatial_domain_id);
        assert_ne!(
            build_spatial_cell(&first, CellIndex::new(0, 0))
                .unwrap()
                .cell_id,
            build_spatial_cell(&second, CellIndex::new(0, 0))
                .unwrap()
                .cell_id
        );
    }

    #[test]
    fn invalid_bounds_steps_identity_and_resource_limits_fail_closed() {
        let mut missing = input();
        missing.logical_world_id = [0; 32];
        assert!(compile_spatial_domain(&missing).is_err());
        let mut empty = input();
        empty.cell_count = [0, 1];
        assert!(compile_spatial_domain(&empty).is_err());
        let mut zero_step = input();
        zero_step.cell_step_q32_32[0] = 0;
        assert!(compile_spatial_domain(&zero_step).is_err());
        let mut huge = input();
        huge.cell_count = [1_001, 1_000];
        assert!(matches!(
            compile_spatial_domain(&huge),
            Err(SpatialDomainError::Invalid("proof cell ceiling exceeded"))
        ));
    }

    #[test]
    fn coordinate_overflow_and_out_of_bounds_fail_before_output() {
        let mut overflow = input();
        overflow.cell_center_origin_q32_32 = [i64::MAX - 1, 0];
        overflow.cell_step_q32_32 = [2, 1];
        overflow.cell_count = [2, 1];
        assert!(matches!(
            compile_spatial_domain(&overflow),
            Err(SpatialDomainError::Invalid("sample-coordinate overflow"))
        ));
        let domain = compile_spatial_domain(&input()).unwrap();
        assert!(matches!(
            build_spatial_cell(&domain, CellIndex::new(3, 0)),
            Err(SpatialDomainError::Invalid("cell index out of bounds"))
        ));
    }

    #[test]
    fn forged_coordinate_neighbours_identity_and_claims_are_rejected() {
        let domain = compile_spatial_domain(&input()).unwrap();
        let cell = build_spatial_cell(&domain, CellIndex::new(1, 0)).unwrap();
        let mut forged_coordinate = cell.clone();
        forged_coordinate.sample_coordinate_q32_32[0] += 1;
        assert!(validate_spatial_cell(&domain, &forged_coordinate).is_err());
        let mut forged_wrap = cell.clone();
        forged_wrap.neighbour_indices.push(CellIndex::new(1, 1));
        assert!(validate_spatial_cell(&domain, &forged_wrap).is_err());
        let mut forged_identity = cell.clone();
        forged_identity.cell_id = [9; 32];
        assert!(validate_spatial_cell(&domain, &forged_identity).is_err());
        let mut authority = cell;
        authority.authority_effect = "promote".into();
        assert!(validate_spatial_cell(&domain, &authority).is_err());
        assert!(authority.to_bytes(&domain).is_err());
    }

    #[test]
    fn unknown_fields_whitespace_and_schema_drift_are_rejected() {
        let input = input();
        let mut value: serde_json::Value =
            serde_json::from_slice(&input.to_bytes().unwrap()).unwrap();
        value["wrap"] = serde_json::json!(true);
        assert!(SpatialDomainInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
        let mut spaced = input.to_bytes().unwrap();
        spaced.push(b' ');
        assert!(matches!(
            SpatialDomainInput::from_bytes(&spaced),
            Err(SpatialDomainError::Invalid(
                "noncanonical spatial-domain input bytes"
            ))
        ));
        let mut stale = input;
        stale.schema_version = 2;
        assert!(compile_spatial_domain(&stale).is_err());

        let index = CellIndex::new(1, 1);
        let mut spaced_index = index.to_bytes().unwrap();
        spaced_index.push(b' ');
        assert!(matches!(
            CellIndex::from_bytes(&spaced_index),
            Err(SpatialDomainError::Invalid("noncanonical cell-index bytes"))
        ));
    }
}
