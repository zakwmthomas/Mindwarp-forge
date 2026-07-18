//! Capability-free bounded 3D occupancy and exact path-witness reference.
//!
//! This crate proves one finite Cartesian evidence volume and exhaustive exact
//! segment-versus-cell traversal. It is not a planet, terrain engine, runtime
//! voxel map, biome model, visibility result, propagation model, navigation
//! mesh, movement policy, storage layout, approval, or promotion path.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

mod interval;
pub use interval::*;

pub type Id = [u8; 32];

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_PHYSICAL_VOLUME_PROOF_CELLS: u64 = 65_536;
pub const MAX_PHYSICAL_VOLUME_RUNS: usize = 65_536;
pub const MAX_PATH_WITNESS_RECORDS: usize = 65_536;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateFrameV1 {
    CartesianQ32_32Volume3dV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryModeV1 {
    BoundedAbsent,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjacencyV1 {
    SharedFace6,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CellEvidenceV1 {
    Unavailable,
    Vacuum,
    Gas { substance_source_id: Id },
    Liquid { substance_source_id: Id },
    Solid { substance_source_id: Id },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnRunV1 {
    pub x_index: u32,
    pub y_index: u32,
    pub z_start: u32,
    pub length: u32,
    pub evidence: CellEvidenceV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalVolumeRecipeInputV1 {
    pub schema_version: u16,
    pub recipe_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub recipe_revision: u32,
    pub coordinate_frame: CoordinateFrameV1,
    pub origin_q32_32: [i64; 3],
    pub cell_step_q32_32: i64,
    pub extent: [u32; 3],
    pub boundary_mode: BoundaryModeV1,
    pub adjacency: AdjacencyV1,
    pub default_evidence: CellEvidenceV1,
    pub column_runs: Vec<ColumnRunV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalVolumeRecipeV1 {
    pub schema_version: u16,
    pub physical_volume_recipe_id: Id,
    pub input: PhysicalVolumeRecipeInputV1,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalVolumeV1 {
    pub schema_version: u16,
    pub physical_volume_recipe_id: Id,
    pub reconstruction_id: Id,
    pub cell_count: u64,
    pub occupancy_fingerprint: Id,
    pub physical_volume_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellIndex3V1 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalCellV1 {
    pub schema_version: u16,
    pub physical_volume_id: Id,
    pub index: CellIndex3V1,
    pub min_q32_32: [i64; 3],
    pub max_q32_32: [i64; 3],
    pub evidence: CellEvidenceV1,
    pub physical_cell_id: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnitRationalV1 {
    pub numerator: u64,
    pub denominator: u64,
}

impl Ord for UnitRationalV1 {
    fn cmp(&self, other: &Self) -> Ordering {
        (u128::from(self.numerator) * u128::from(other.denominator))
            .cmp(&(u128::from(other.numerator) * u128::from(self.denominator)))
    }
}

impl PartialOrd for UnitRationalV1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UnitRationalV1 {
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, PhysicalPathError> {
        if denominator == 0 || numerator > denominator {
            return Err(PhysicalPathError::Invalid("invalid unit rational range"));
        }
        let divisor = gcd(numerator, denominator);
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    pub fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    pub fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }

    pub fn one_minus(self) -> Self {
        Self::new(self.denominator - self.numerator, self.denominator)
            .expect("a valid unit rational has a valid complement")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalPathQueryV1 {
    pub schema_version: u16,
    pub physical_volume_id: Id,
    pub start_q32_32: [i64; 3],
    pub end_q32_32: [i64; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PathIntersectionKindV1 {
    Interval,
    Point,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellPathRecordV1 {
    pub physical_cell_id: Id,
    pub index: CellIndex3V1,
    pub t_enter: UnitRationalV1,
    pub t_exit: UnitRationalV1,
    pub intersection_kind: PathIntersectionKindV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalPathWitnessV1 {
    pub schema_version: u16,
    pub physical_volume_id: Id,
    pub path_query_id: Id,
    pub records: Vec<CellPathRecordV1>,
    pub path_witness_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhysicalPathError {
    Invalid(&'static str),
    Codec(String),
}

impl PhysicalVolumeRecipeInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PhysicalPathError> {
        validate_recipe_input(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PhysicalPathError> {
        let value: Self = decode(bytes)?;
        validate_recipe_input(&value)?;
        require_canonical(
            &value,
            bytes,
            "noncanonical physical-volume recipe input bytes",
        )?;
        Ok(value)
    }
}

impl PhysicalVolumeRecipeV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PhysicalPathError> {
        validate_physical_volume_recipe(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PhysicalPathError> {
        let value: Self = decode(bytes)?;
        validate_physical_volume_recipe(&value)?;
        require_canonical(&value, bytes, "noncanonical physical-volume recipe bytes")?;
        Ok(value)
    }
}

impl PhysicalVolumeV1 {
    pub fn to_bytes(&self, recipe: &PhysicalVolumeRecipeV1) -> Result<Vec<u8>, PhysicalPathError> {
        validate_physical_volume(recipe, self)?;
        encode(self)
    }

    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPathError> {
        let value: Self = decode(bytes)?;
        validate_physical_volume(recipe, &value)?;
        require_canonical(&value, bytes, "noncanonical physical-volume bytes")?;
        Ok(value)
    }
}

impl PhysicalPathQueryV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
    ) -> Result<Vec<u8>, PhysicalPathError> {
        validate_path_query(recipe, volume, self)?;
        encode(self)
    }

    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPathError> {
        let value: Self = decode(bytes)?;
        validate_path_query(recipe, volume, &value)?;
        require_canonical(&value, bytes, "noncanonical physical-path query bytes")?;
        Ok(value)
    }
}

impl PhysicalPathWitnessV1 {
    pub fn to_bytes(
        &self,
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        query: &PhysicalPathQueryV1,
    ) -> Result<Vec<u8>, PhysicalPathError> {
        validate_path_witness(recipe, volume, query, self)?;
        encode(self)
    }

    pub fn from_bytes(
        recipe: &PhysicalVolumeRecipeV1,
        volume: &PhysicalVolumeV1,
        query: &PhysicalPathQueryV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPathError> {
        let value: Self = decode(bytes)?;
        validate_path_witness(recipe, volume, query, &value)?;
        require_canonical(&value, bytes, "noncanonical physical-path witness bytes")?;
        Ok(value)
    }
}

pub fn compile_physical_volume_recipe(
    input: &PhysicalVolumeRecipeInputV1,
) -> Result<PhysicalVolumeRecipeV1, PhysicalPathError> {
    let bytes = input.to_bytes()?;
    Ok(PhysicalVolumeRecipeV1 {
        schema_version: CONTRACT_VERSION,
        physical_volume_recipe_id: hash(b"mindwarp.physical-path.volume-recipe.v1", &bytes),
        input: input.clone(),
        limitations: recipe_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_physical_volume_recipe(
    recipe: &PhysicalVolumeRecipeV1,
) -> Result<(), PhysicalPathError> {
    let expected = compile_physical_volume_recipe(&recipe.input)?;
    if recipe != &expected {
        return Err(PhysicalPathError::Invalid("physical-volume recipe drift"));
    }
    Ok(())
}

pub fn compile_physical_volume(
    recipe: &PhysicalVolumeRecipeV1,
) -> Result<PhysicalVolumeV1, PhysicalPathError> {
    validate_physical_volume_recipe(recipe)?;
    let cell_count = checked_cell_count(&recipe.input)?;
    let semantic_bytes = encode(&SemanticOccupancyV1::from(&recipe.input))?;
    let occupancy_fingerprint = hash(b"mindwarp.physical-path.occupancy.v1", &semantic_bytes);
    let volume_bytes = encode(&(
        recipe.physical_volume_recipe_id,
        occupancy_fingerprint,
        cell_count,
    ))?;
    Ok(PhysicalVolumeV1 {
        schema_version: CONTRACT_VERSION,
        physical_volume_recipe_id: recipe.physical_volume_recipe_id,
        reconstruction_id: recipe.input.reconstruction_id,
        cell_count,
        occupancy_fingerprint,
        physical_volume_id: hash(b"mindwarp.physical-path.volume.v1", &volume_bytes),
        limitations: volume_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_physical_volume(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
) -> Result<(), PhysicalPathError> {
    let expected = compile_physical_volume(recipe)?;
    if volume != &expected {
        return Err(PhysicalPathError::Invalid("physical-volume result drift"));
    }
    Ok(())
}

pub fn build_physical_cell(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    index: CellIndex3V1,
) -> Result<PhysicalCellV1, PhysicalPathError> {
    validate_physical_volume(recipe, volume)?;
    validate_index(&recipe.input, index)?;
    let evidence = reconstruct_evidence(&recipe.input)?[flat_index(&recipe.input, index)?].clone();
    build_cell_with_evidence(recipe, volume, index, evidence)
}

pub fn compile_path_witness(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    query: &PhysicalPathQueryV1,
) -> Result<PhysicalPathWitnessV1, PhysicalPathError> {
    validate_path_query(recipe, volume, query)?;
    let evidence = reconstruct_evidence(&recipe.input)?;
    let stationary = query.start_q32_32 == query.end_q32_32;
    let mut records = Vec::new();
    for x in 0..recipe.input.extent[0] {
        for y in 0..recipe.input.extent[1] {
            for z in 0..recipe.input.extent[2] {
                let index = CellIndex3V1 { x, y, z };
                let cell = build_cell_with_evidence(
                    recipe,
                    volume,
                    index,
                    evidence[flat_index(&recipe.input, index)?].clone(),
                )?;
                if let Some((t_enter, t_exit)) = intersect_closed_cell(query, &cell)? {
                    let intersection_kind = if stationary || t_enter == t_exit {
                        PathIntersectionKindV1::Point
                    } else {
                        PathIntersectionKindV1::Interval
                    };
                    records.push(CellPathRecordV1 {
                        physical_cell_id: cell.physical_cell_id,
                        index,
                        t_enter,
                        t_exit,
                        intersection_kind,
                    });
                }
            }
        }
    }
    records.sort_by(|left, right| {
        left.t_enter
            .cmp(&right.t_enter)
            .then_with(|| left.t_exit.cmp(&right.t_exit))
            .then_with(|| left.index.cmp(&right.index))
    });
    if records.len() > MAX_PATH_WITNESS_RECORDS {
        return Err(PhysicalPathError::Invalid(
            "path witness record ceiling exceeded",
        ));
    }
    let query_bytes = query.to_bytes(recipe, volume)?;
    let path_query_id = hash(b"mindwarp.physical-path.query.v1", &query_bytes);
    let witness_bytes = encode(&(volume.physical_volume_id, path_query_id, &records))?;
    Ok(PhysicalPathWitnessV1 {
        schema_version: CONTRACT_VERSION,
        physical_volume_id: volume.physical_volume_id,
        path_query_id,
        records,
        path_witness_id: hash(b"mindwarp.physical-path.witness.v1", &witness_bytes),
        limitations: witness_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_path_witness(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    query: &PhysicalPathQueryV1,
    witness: &PhysicalPathWitnessV1,
) -> Result<(), PhysicalPathError> {
    let expected = compile_path_witness(recipe, volume, query)?;
    if witness != &expected {
        return Err(PhysicalPathError::Invalid("physical-path witness drift"));
    }
    Ok(())
}

fn validate_recipe_input(input: &PhysicalVolumeRecipeInputV1) -> Result<(), PhysicalPathError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(PhysicalPathError::Invalid(
            "unsupported physical-volume recipe schema",
        ));
    }
    if input.recipe_source_id == [0; 32]
        || input.scope_id == [0; 32]
        || input.reconstruction_id == [0; 32]
    {
        return Err(PhysicalPathError::Invalid(
            "physical-volume identity is zero",
        ));
    }
    if input.recipe_revision == 0 || input.cell_step_q32_32 <= 0 || input.extent.contains(&0) {
        return Err(PhysicalPathError::Invalid(
            "physical-volume revision step or extent is zero",
        ));
    }
    validate_evidence(&input.default_evidence)?;
    checked_cell_count(input)?;
    outer_max(input)?;
    if input.column_runs.len() > MAX_PHYSICAL_VOLUME_RUNS {
        return Err(PhysicalPathError::Invalid(
            "physical-volume run ceiling exceeded",
        ));
    }
    let mut previous: Option<&ColumnRunV1> = None;
    for run in &input.column_runs {
        validate_evidence(&run.evidence)?;
        if run.length == 0
            || run.x_index >= input.extent[0]
            || run.y_index >= input.extent[1]
            || run
                .z_start
                .checked_add(run.length)
                .filter(|end| *end <= input.extent[2])
                .is_none()
            || run.evidence == input.default_evidence
        {
            return Err(PhysicalPathError::Invalid(
                "invalid physical-volume column run",
            ));
        }
        if let Some(prior) = previous {
            let prior_key = (prior.x_index, prior.y_index, prior.z_start);
            let key = (run.x_index, run.y_index, run.z_start);
            if key <= prior_key {
                return Err(PhysicalPathError::Invalid(
                    "column runs are not strictly sorted",
                ));
            }
            if prior.x_index == run.x_index && prior.y_index == run.y_index {
                let prior_end = prior
                    .z_start
                    .checked_add(prior.length)
                    .ok_or(PhysicalPathError::Invalid("column run end overflow"))?;
                if run.z_start < prior_end
                    || (run.z_start == prior_end && run.evidence == prior.evidence)
                {
                    return Err(PhysicalPathError::Invalid(
                        "column runs overlap or are mergeable",
                    ));
                }
            }
        }
        previous = Some(run);
    }
    Ok(())
}

fn validate_evidence(evidence: &CellEvidenceV1) -> Result<(), PhysicalPathError> {
    let source = match evidence {
        CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } => Some(substance_source_id),
        CellEvidenceV1::Unavailable | CellEvidenceV1::Vacuum => None,
    };
    if source == Some(&[0; 32]) {
        return Err(PhysicalPathError::Invalid(
            "non-vacuum substance identity is zero",
        ));
    }
    Ok(())
}

fn validate_path_query(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    query: &PhysicalPathQueryV1,
) -> Result<(), PhysicalPathError> {
    validate_physical_volume(recipe, volume)?;
    if query.schema_version != CONTRACT_VERSION
        || query.physical_volume_id != volume.physical_volume_id
    {
        return Err(PhysicalPathError::Invalid(
            "physical-path query binding drift",
        ));
    }
    let max = outer_max(&recipe.input)?;
    for point in [query.start_q32_32, query.end_q32_32] {
        for axis in 0..3 {
            if point[axis] < recipe.input.origin_q32_32[axis] || point[axis] > max[axis] {
                return Err(PhysicalPathError::Invalid(
                    "physical-path endpoint outside closed volume",
                ));
            }
        }
    }
    Ok(())
}

fn checked_cell_count(input: &PhysicalVolumeRecipeInputV1) -> Result<u64, PhysicalPathError> {
    let count = input
        .extent
        .into_iter()
        .try_fold(1_u64, |total, value| total.checked_mul(u64::from(value)))
        .ok_or(PhysicalPathError::Invalid(
            "physical-volume cell-count overflow",
        ))?;
    if count == 0 || count > MAX_PHYSICAL_VOLUME_PROOF_CELLS {
        return Err(PhysicalPathError::Invalid(
            "physical-volume proof-cell ceiling exceeded",
        ));
    }
    Ok(count)
}

fn outer_max(input: &PhysicalVolumeRecipeInputV1) -> Result<[i64; 3], PhysicalPathError> {
    let mut result = [0_i64; 3];
    for (axis, slot) in result.iter_mut().enumerate() {
        let value = i128::from(input.origin_q32_32[axis])
            .checked_add(
                i128::from(input.cell_step_q32_32)
                    .checked_mul(i128::from(input.extent[axis]))
                    .ok_or(PhysicalPathError::Invalid(
                        "physical-volume coordinate multiplication overflow",
                    ))?,
            )
            .ok_or(PhysicalPathError::Invalid(
                "physical-volume coordinate overflow",
            ))?;
        *slot = i64::try_from(value)
            .map_err(|_| PhysicalPathError::Invalid("physical-volume coordinate outside Q32.32"))?;
    }
    Ok(result)
}

fn reconstruct_evidence(
    input: &PhysicalVolumeRecipeInputV1,
) -> Result<Vec<CellEvidenceV1>, PhysicalPathError> {
    validate_recipe_input(input)?;
    let count = usize::try_from(checked_cell_count(input)?).map_err(|_| {
        PhysicalPathError::Invalid("physical-volume allocation conversion overflow")
    })?;
    let mut evidence = vec![input.default_evidence.clone(); count];
    for run in &input.column_runs {
        for z in run.z_start..run.z_start + run.length {
            let index = CellIndex3V1 {
                x: run.x_index,
                y: run.y_index,
                z,
            };
            evidence[flat_index(input, index)?] = run.evidence.clone();
        }
    }
    Ok(evidence)
}

fn validate_index(
    input: &PhysicalVolumeRecipeInputV1,
    index: CellIndex3V1,
) -> Result<(), PhysicalPathError> {
    if index.x >= input.extent[0] || index.y >= input.extent[1] || index.z >= input.extent[2] {
        return Err(PhysicalPathError::Invalid(
            "physical-volume cell index out of range",
        ));
    }
    Ok(())
}

fn flat_index(
    input: &PhysicalVolumeRecipeInputV1,
    index: CellIndex3V1,
) -> Result<usize, PhysicalPathError> {
    validate_index(input, index)?;
    let value = u64::from(index.x)
        .checked_mul(u64::from(input.extent[1]))
        .and_then(|v| v.checked_add(u64::from(index.y)))
        .and_then(|v| v.checked_mul(u64::from(input.extent[2])))
        .and_then(|v| v.checked_add(u64::from(index.z)))
        .ok_or(PhysicalPathError::Invalid(
            "physical-volume flat-index overflow",
        ))?;
    usize::try_from(value)
        .map_err(|_| PhysicalPathError::Invalid("physical-volume flat-index conversion overflow"))
}

fn build_cell_with_evidence(
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    index: CellIndex3V1,
    evidence: CellEvidenceV1,
) -> Result<PhysicalCellV1, PhysicalPathError> {
    let mut min = [0_i64; 3];
    let mut max = [0_i64; 3];
    let indices = [index.x, index.y, index.z];
    for axis in 0..3 {
        let low = i128::from(recipe.input.origin_q32_32[axis])
            + i128::from(recipe.input.cell_step_q32_32) * i128::from(indices[axis]);
        let high = low + i128::from(recipe.input.cell_step_q32_32);
        min[axis] = i64::try_from(low)
            .map_err(|_| PhysicalPathError::Invalid("physical-cell minimum overflow"))?;
        max[axis] = i64::try_from(high)
            .map_err(|_| PhysicalPathError::Invalid("physical-cell maximum overflow"))?;
    }
    let identity_bytes = encode(&(volume.physical_volume_id, index, &evidence))?;
    Ok(PhysicalCellV1 {
        schema_version: CONTRACT_VERSION,
        physical_volume_id: volume.physical_volume_id,
        index,
        min_q32_32: min,
        max_q32_32: max,
        evidence,
        physical_cell_id: hash(b"mindwarp.physical-path.cell.v1", &identity_bytes),
    })
}

fn intersect_closed_cell(
    query: &PhysicalPathQueryV1,
    cell: &PhysicalCellV1,
) -> Result<Option<(UnitRationalV1, UnitRationalV1)>, PhysicalPathError> {
    let mut enter = RawUnitFraction::zero();
    let mut exit = RawUnitFraction::one();
    for axis in 0..3 {
        let start = i128::from(query.start_q32_32[axis]);
        let end = i128::from(query.end_q32_32[axis]);
        let direction = end - start;
        let lower = i128::from(cell.min_q32_32[axis]);
        let upper = i128::from(cell.max_q32_32[axis]);
        if direction == 0 {
            if start < lower || start > upper {
                return Ok(None);
            }
            continue;
        }
        let denominator = direction.unsigned_abs();
        let (raw_enter, raw_exit) = if direction > 0 {
            (lower - start, upper - start)
        } else {
            (start - upper, start - lower)
        };
        if raw_exit < 0
            || raw_enter
                > i128::try_from(denominator).map_err(|_| {
                    PhysicalPathError::Invalid("path denominator conversion overflow")
                })?
        {
            return Ok(None);
        }
        let axis_enter = RawUnitFraction::clamped(raw_enter, denominator)?;
        let axis_exit = RawUnitFraction::clamped(raw_exit, denominator)?;
        if axis_enter > enter {
            enter = axis_enter;
        }
        if axis_exit < exit {
            exit = axis_exit;
        }
        if enter > exit {
            return Ok(None);
        }
    }
    Ok(Some((enter.reduced()?, exit.reduced()?)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RawUnitFraction {
    numerator: u64,
    denominator: u64,
}

impl RawUnitFraction {
    fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
    fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
    fn clamped(numerator: i128, denominator: u128) -> Result<Self, PhysicalPathError> {
        let denominator = u64::try_from(denominator)
            .map_err(|_| PhysicalPathError::Invalid("path denominator exceeds u64"))?;
        let numerator = if numerator <= 0 {
            0
        } else {
            u64::try_from(numerator)
                .map_err(|_| PhysicalPathError::Invalid("path numerator exceeds u64"))?
                .min(denominator)
        };
        Ok(Self {
            numerator,
            denominator,
        })
    }
    fn reduced(self) -> Result<UnitRationalV1, PhysicalPathError> {
        UnitRationalV1::new(self.numerator, self.denominator)
    }
}

impl Ord for RawUnitFraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (u128::from(self.numerator) * u128::from(other.denominator))
            .cmp(&(u128::from(other.numerator) * u128::from(self.denominator)))
    }
}
impl PartialOrd for RawUnitFraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize)]
struct SemanticOccupancyV1<'a> {
    coordinate_frame: CoordinateFrameV1,
    origin_q32_32: [i64; 3],
    cell_step_q32_32: i64,
    extent: [u32; 3],
    boundary_mode: BoundaryModeV1,
    adjacency: AdjacencyV1,
    default_evidence: &'a CellEvidenceV1,
    column_runs: &'a [ColumnRunV1],
}
impl<'a> From<&'a PhysicalVolumeRecipeInputV1> for SemanticOccupancyV1<'a> {
    fn from(value: &'a PhysicalVolumeRecipeInputV1) -> Self {
        Self {
            coordinate_frame: value.coordinate_frame,
            origin_q32_32: value.origin_q32_32,
            cell_step_q32_32: value.cell_step_q32_32,
            extent: value.extent,
            boundary_mode: value.boundary_mode,
            adjacency: value.adjacency,
            default_evidence: &value.default_evidence,
            column_runs: &value.column_runs,
        }
    }
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn encode<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, PhysicalPathError> {
    serde_json::to_vec(value).map_err(|error| PhysicalPathError::Codec(error.to_string()))
}
fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, PhysicalPathError> {
    serde_json::from_slice(bytes).map_err(|error| PhysicalPathError::Codec(error.to_string()))
}
fn require_canonical<T: Serialize + ?Sized>(
    value: &T,
    bytes: &[u8],
    message: &'static str,
) -> Result<(), PhysicalPathError> {
    if encode(value)? != bytes {
        return Err(PhysicalPathError::Invalid(message));
    }
    Ok(())
}
fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
fn recipe_limitations() -> Vec<String> {
    vec![
        "finite cubic Q32.32 Cartesian proof volume only".into(),
        "sparse columns are canonical source encoding not runtime storage".into(),
        "no planet terrain biome runtime or authority claim".into(),
    ]
}
fn volume_limitations() -> Vec<String> {
    vec![
        "dominant cell phase is bounded evidence not complete material physics".into(),
        "unavailable is distinct from vacuum".into(),
        "no opacity transfer passability biome planet runtime or authority claim".into(),
    ]
}
fn witness_limitations() -> Vec<String> {
    vec![
        "exhaustive exact closed-cell intersection reference".into(),
        "point and interval records are evidence not consumer blocking policy".into(),
        "no visibility propagation navigation biome planet runtime or authority claim".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn id(byte: u8) -> Id {
        [byte; 32]
    }
    fn base_input(extent: [u32; 3]) -> PhysicalVolumeRecipeInputV1 {
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
            default_evidence: CellEvidenceV1::Vacuum,
            column_runs: Vec::new(),
        }
    }
    fn setup(extent: [u32; 3]) -> (PhysicalVolumeRecipeV1, PhysicalVolumeV1) {
        let recipe = compile_physical_volume_recipe(&base_input(extent)).unwrap();
        let volume = compile_physical_volume(&recipe).unwrap();
        (recipe, volume)
    }
    fn query(volume: &PhysicalVolumeV1, start: [i64; 3], end: [i64; 3]) -> PhysicalPathQueryV1 {
        PhysicalPathQueryV1 {
            schema_version: 1,
            physical_volume_id: volume.physical_volume_id,
            start_q32_32: start,
            end_q32_32: end,
        }
    }

    #[test]
    fn strict_recipe_volume_query_and_witness_replay() {
        let (recipe, volume) = setup([3, 3, 3]);
        let q = query(&volume, [1, 1, 1], [11, 1, 1]);
        let witness = compile_path_witness(&recipe, &volume, &q).unwrap();
        assert_eq!(
            PhysicalVolumeRecipeV1::from_bytes(&recipe.to_bytes().unwrap()).unwrap(),
            recipe
        );
        assert_eq!(
            PhysicalVolumeV1::from_bytes(&recipe, &volume.to_bytes(&recipe).unwrap()).unwrap(),
            volume
        );
        assert_eq!(
            PhysicalPathQueryV1::from_bytes(
                &recipe,
                &volume,
                &q.to_bytes(&recipe, &volume).unwrap()
            )
            .unwrap(),
            q
        );
        assert_eq!(
            PhysicalPathWitnessV1::from_bytes(
                &recipe,
                &volume,
                &q,
                &witness.to_bytes(&recipe, &volume, &q).unwrap()
            )
            .unwrap(),
            witness
        );
    }

    #[test]
    fn unavailable_vacuum_and_substance_evidence_are_distinct() {
        let mut input = base_input([1, 1, 4]);
        input.default_evidence = CellEvidenceV1::Unavailable;
        input.column_runs = vec![
            ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 1,
                length: 1,
                evidence: CellEvidenceV1::Vacuum,
            },
            ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 2,
                length: 1,
                evidence: CellEvidenceV1::Gas {
                    substance_source_id: id(8),
                },
            },
            ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 3,
                length: 1,
                evidence: CellEvidenceV1::Solid {
                    substance_source_id: id(9),
                },
            },
        ];
        let recipe = compile_physical_volume_recipe(&input).unwrap();
        let volume = compile_physical_volume(&recipe).unwrap();
        let values: Vec<_> = (0..4)
            .map(|z| {
                build_physical_cell(&recipe, &volume, CellIndex3V1 { x: 0, y: 0, z })
                    .unwrap()
                    .evidence
            })
            .collect();
        assert_eq!(
            values,
            vec![
                CellEvidenceV1::Unavailable,
                CellEvidenceV1::Vacuum,
                CellEvidenceV1::Gas {
                    substance_source_id: id(8)
                },
                CellEvidenceV1::Solid {
                    substance_source_id: id(9)
                }
            ]
        );
        input.column_runs[1].evidence = CellEvidenceV1::Liquid {
            substance_source_id: [0; 32],
        };
        assert!(compile_physical_volume_recipe(&input).is_err());
    }

    #[test]
    fn column_run_canonicality_and_ceiling_fail_closed() {
        let mut input = base_input([1, 1, 4]);
        input.column_runs = vec![
            ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 0,
                length: 1,
                evidence: CellEvidenceV1::Solid {
                    substance_source_id: id(4),
                },
            },
            ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 1,
                length: 1,
                evidence: CellEvidenceV1::Solid {
                    substance_source_id: id(4),
                },
            },
        ];
        assert!(compile_physical_volume_recipe(&input).is_err());
        input.column_runs[1].evidence = CellEvidenceV1::Gas {
            substance_source_id: id(5),
        };
        assert!(compile_physical_volume_recipe(&input).is_ok());
        input.column_runs.swap(0, 1);
        assert!(compile_physical_volume_recipe(&input).is_err());
        assert!(compile_physical_volume_recipe(&base_input([257, 256, 1])).is_err());
    }

    #[test]
    fn coordinate_overflow_and_outside_endpoints_fail() {
        let mut input = base_input([2, 1, 1]);
        input.origin_q32_32 = [i64::MAX - 3, 0, 0];
        assert!(compile_physical_volume_recipe(&input).is_err());
        let (recipe, volume) = setup([1, 1, 1]);
        assert!(
            compile_path_witness(&recipe, &volume, &query(&volume, [-1, 0, 0], [1, 1, 1])).is_err()
        );
        assert!(
            compile_path_witness(&recipe, &volume, &query(&volume, [0, 0, 0], [4, 4, 4])).is_ok()
        );
    }

    #[test]
    fn face_edge_vertex_and_endpoint_contacts_are_conservative() {
        let (recipe, volume) = setup([3, 3, 3]);
        let cases = [
            ("face", [1, 4, 2], [11, 4, 2], 6, 0),
            ("edge", [1, 4, 4], [11, 4, 4], 12, 0),
            ("vertex", [1, 1, 1], [7, 7, 7], 8, 6),
            ("endpoint", [4, 4, 4], [7, 7, 7], 8, 7),
        ];
        for (name, start, end, total, points) in cases {
            let w = compile_path_witness(&recipe, &volume, &query(&volume, start, end)).unwrap();
            assert_eq!(w.records.len(), total, "{name}");
            assert_eq!(
                w.records
                    .iter()
                    .filter(|r| r.intersection_kind == PathIntersectionKindV1::Point)
                    .count(),
                points,
                "{name}"
            );
        }
    }

    #[test]
    fn zero_length_vertex_is_eight_points_with_full_parameter_preimage() {
        let (recipe, volume) = setup([3, 3, 3]);
        let q = query(&volume, [4, 4, 4], [4, 4, 4]);
        let w = compile_path_witness(&recipe, &volume, &q).unwrap();
        assert_eq!(w.records.len(), 8);
        assert!(
            w.records
                .iter()
                .all(|r| r.intersection_kind == PathIntersectionKindV1::Point
                    && r.t_enter == UnitRationalV1::zero()
                    && r.t_exit == UnitRationalV1::one())
        );
    }

    #[test]
    fn reversal_maps_exact_intervals_and_preserves_cells() {
        let (recipe, volume) = setup([3, 3, 3]);
        let forward =
            compile_path_witness(&recipe, &volume, &query(&volume, [1, 1, 1], [11, 9, 7])).unwrap();
        let reverse =
            compile_path_witness(&recipe, &volume, &query(&volume, [11, 9, 7], [1, 1, 1])).unwrap();
        for record in &forward.records {
            let counterpart = reverse
                .records
                .iter()
                .find(|other| other.index == record.index)
                .unwrap();
            assert_eq!(counterpart.intersection_kind, record.intersection_kind);
            assert_eq!(counterpart.t_enter, record.t_exit.one_minus());
            assert_eq!(counterpart.t_exit, record.t_enter.one_minus());
        }
    }

    #[test]
    fn thin_barrier_crossing_differs_from_point_contact() {
        let mut input = base_input([3, 3, 3]);
        input.column_runs = vec![ColumnRunV1 {
            x_index: 1,
            y_index: 0,
            z_start: 0,
            length: 1,
            evidence: CellEvidenceV1::Solid {
                substance_source_id: id(7),
            },
        }];
        let recipe = compile_physical_volume_recipe(&input).unwrap();
        let volume = compile_physical_volume(&recipe).unwrap();
        let cross =
            compile_path_witness(&recipe, &volume, &query(&volume, [1, 2, 2], [7, 2, 2])).unwrap();
        let vertex =
            compile_path_witness(&recipe, &volume, &query(&volume, [1, 1, 1], [7, 7, 7])).unwrap();
        let barrier = CellIndex3V1 { x: 1, y: 0, z: 0 };
        assert_eq!(
            cross
                .records
                .iter()
                .find(|r| r.index == barrier)
                .unwrap()
                .intersection_kind,
            PathIntersectionKindV1::Interval
        );
        assert_eq!(
            vertex
                .records
                .iter()
                .find(|r| r.index == barrier)
                .unwrap()
                .intersection_kind,
            PathIntersectionKindV1::Point
        );
    }

    #[test]
    fn outer_boundary_has_no_phantom_cells_and_records_are_unique() {
        let (recipe, volume) = setup([3, 3, 3]);
        let w =
            compile_path_witness(&recipe, &volume, &query(&volume, [0, 0, 0], [2, 2, 2])).unwrap();
        assert_eq!(w.records.len(), 1);
        assert_eq!(w.records[0].index, CellIndex3V1 { x: 0, y: 0, z: 0 });
        let mut ids = w
            .records
            .iter()
            .map(|r| r.physical_cell_id)
            .collect::<Vec<_>>();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), w.records.len());
    }

    #[test]
    fn provenance_rekeys_volume_but_not_semantic_occupancy() {
        let first = compile_physical_volume_recipe(&base_input([2, 2, 2])).unwrap();
        let first_volume = compile_physical_volume(&first).unwrap();
        let mut changed = base_input([2, 2, 2]);
        changed.recipe_source_id = id(99);
        changed.recipe_revision = 2;
        let second = compile_physical_volume_recipe(&changed).unwrap();
        let second_volume = compile_physical_volume(&second).unwrap();
        assert_ne!(
            first_volume.physical_volume_id,
            second_volume.physical_volume_id
        );
        assert_eq!(
            first_volume.occupancy_fingerprint,
            second_volume.occupancy_fingerprint
        );
    }

    #[test]
    fn forged_results_and_noncanonical_bytes_fail() {
        let (recipe, volume) = setup([2, 2, 2]);
        let q = query(&volume, [0, 0, 0], [8, 8, 8]);
        let mut witness = compile_path_witness(&recipe, &volume, &q).unwrap();
        witness.records[0].intersection_kind = PathIntersectionKindV1::Point;
        assert!(validate_path_witness(&recipe, &volume, &q, &witness).is_err());
        let mut bytes = recipe.input.to_bytes().unwrap();
        bytes.push(b' ');
        assert!(PhysicalVolumeRecipeInputV1::from_bytes(&bytes).is_err());
        let unreduced = UnitRationalV1 {
            numerator: 2,
            denominator: 4,
        };
        assert_ne!(unreduced, UnitRationalV1::new(2, 4).unwrap());
    }

    #[test]
    fn maximum_ceiling_cost_receipt() {
        let started = Instant::now();
        let (recipe, volume) = setup([256, 256, 1]);
        let q = query(&volume, [0, 0, 0], [1024, 1024, 4]);
        let witness = compile_path_witness(&recipe, &volume, &q).unwrap();
        let elapsed = started.elapsed();
        let recipe_bytes = recipe.to_bytes().unwrap().len();
        let volume_bytes = volume.to_bytes(&recipe).unwrap().len();
        let witness_bytes = witness.to_bytes(&recipe, &volume, &q).unwrap().len();
        assert_eq!(volume.cell_count, 65_536);
        assert!(witness.records.len() <= MAX_PATH_WITNESS_RECORDS);
        assert!(elapsed.as_secs() < 30);
        eprintln!(
            "physical_path_ceiling_cost cells={} witness_records={} recipe_bytes={} volume_bytes={} witness_bytes={} elapsed_ms={}",
            volume.cell_count,
            witness.records.len(),
            recipe_bytes,
            volume_bytes,
            witness_bytes,
            elapsed.as_millis()
        );
    }

    #[test]
    fn public_claims_remain_evidence_only() {
        let (recipe, volume) = setup([1, 1, 1]);
        let q = query(&volume, [0, 0, 0], [4, 4, 4]);
        let witness = compile_path_witness(&recipe, &volume, &q).unwrap();
        let text = String::from_utf8(witness.to_bytes(&recipe, &volume, &q).unwrap()).unwrap();
        assert!(text.contains("none_evidence_only"));
        for forbidden in [
            "planet_surface",
            "biome_weights",
            "visibility_result",
            "path_cost",
            "runtime_map",
            "promotion_authority",
        ] {
            assert!(!text.contains(forbidden));
        }
    }
}
