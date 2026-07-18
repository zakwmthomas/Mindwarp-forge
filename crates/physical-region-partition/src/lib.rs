//! Capability-free bounded physical-region partition reference.
//!
//! V1 classifies exact regional physical evidence into versioned signatures,
//! then reconstructs shared-edge connected components. It is not a planet,
//! biome, habitat, terrain, runtime map, storage partition, or authority path.

use climate_state::{ClimateContract, ClimateError, validate_climate};
use regional_environment_state::{
    RegionalEnvironmentError, RegionalFieldBindingV1, compile_regional_environment_for_binding_cell,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spatial_domain::{
    CellIndex, Id, SpatialDomain, SpatialDomainError, build_spatial_cell, neighbour_indices,
    validate_spatial_domain,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_PARTITION_PROOF_CELLS: u64 = 65_536;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PartitionDimensionV1 {
    RegionalExposurePermille,
    RegionalMoisturePotentialPermille,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DimensionClassifierV1 {
    ExactValue,
    LowerBoundCuts { cuts: Vec<u16> },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionRuleV1 {
    pub dimension: PartitionDimensionV1,
    pub classifier: DimensionClassifierV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalPartitionRecipeInputV1 {
    pub schema_version: u16,
    pub recipe_source_id: Id,
    pub scope_id: Id,
    pub recipe_revision: u32,
    pub spatial_domain_contract_version: u16,
    pub regional_environment_contract_version: u16,
    pub climate_contract_version: u16,
    pub dimension_rules: Vec<DimensionRuleV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalPartitionRecipeV1 {
    pub schema_version: u16,
    pub physical_partition_recipe_id: Id,
    pub input: PhysicalPartitionRecipeInputV1,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalPartitionInputV1 {
    pub schema_version: u16,
    pub spatial_domain: SpatialDomain,
    pub regional_field_binding: RegionalFieldBindingV1,
    pub climate: ClimateContract,
    pub recipe: PhysicalPartitionRecipeV1,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SignatureValueV1 {
    Unavailable,
    Exact { value: u16 },
    Bin { index: u16 },
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionSignatureV1 {
    pub dimension: PartitionDimensionV1,
    pub value: SignatureValueV1,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalSignatureV1 {
    pub schema_version: u16,
    pub dimensions: Vec<DimensionSignatureV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalRegionComponentV1 {
    pub schema_version: u16,
    pub component_id: Id,
    pub signature: PhysicalSignatureV1,
    pub member_indices: Vec<CellIndex>,
    pub member_cell_ids: Vec<Id>,
    pub boundary_component_ids: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalRegionPartitionV1 {
    pub schema_version: u16,
    pub partition_input_id: Id,
    pub physical_partition_recipe_id: Id,
    pub spatial_domain_id: Id,
    pub components: Vec<PhysicalRegionComponentV1>,
    pub partition_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhysicalPartitionError {
    Invalid(&'static str),
    Codec(String),
    Spatial(SpatialDomainError),
    Regional(RegionalEnvironmentError),
    Climate(ClimateError),
}

impl PhysicalPartitionRecipeInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PhysicalPartitionError> {
        validate_recipe_input(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PhysicalPartitionError> {
        let value: Self = decode(bytes)?;
        validate_recipe_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical partition-recipe input bytes",
            ));
        }
        Ok(value)
    }
}

impl PhysicalPartitionRecipeV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PhysicalPartitionError> {
        validate_partition_recipe(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PhysicalPartitionError> {
        let value: Self = decode(bytes)?;
        validate_partition_recipe(&value)?;
        if value.to_bytes()? != bytes {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical partition-recipe bytes",
            ));
        }
        Ok(value)
    }
}

impl PhysicalPartitionInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, PhysicalPartitionError> {
        validate_partition_input(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PhysicalPartitionError> {
        let value: Self = decode(bytes)?;
        validate_partition_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical physical-partition input bytes",
            ));
        }
        Ok(value)
    }
}

impl PhysicalRegionPartitionV1 {
    pub fn to_bytes(
        &self,
        input: &PhysicalPartitionInputV1,
    ) -> Result<Vec<u8>, PhysicalPartitionError> {
        validate_physical_region_partition(input, self)?;
        encode(self)
    }

    pub fn from_bytes(
        input: &PhysicalPartitionInputV1,
        bytes: &[u8],
    ) -> Result<Self, PhysicalPartitionError> {
        let value: Self = decode(bytes)?;
        validate_physical_region_partition(input, &value)?;
        if value.to_bytes(input)? != bytes {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical physical-partition result bytes",
            ));
        }
        Ok(value)
    }
}

pub fn compile_partition_recipe(
    input: &PhysicalPartitionRecipeInputV1,
) -> Result<PhysicalPartitionRecipeV1, PhysicalPartitionError> {
    let bytes = input.to_bytes()?;
    Ok(PhysicalPartitionRecipeV1 {
        schema_version: CONTRACT_VERSION,
        physical_partition_recipe_id: hash(b"mindwarp.physical-partition.recipe.v1", &bytes),
        input: input.clone(),
        limitations: recipe_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_partition_recipe(
    recipe: &PhysicalPartitionRecipeV1,
) -> Result<(), PhysicalPartitionError> {
    if recipe.schema_version != CONTRACT_VERSION
        || recipe.limitations != recipe_limitations()
        || recipe.authority_effect != "none_evidence_only"
    {
        return Err(PhysicalPartitionError::Invalid(
            "partition-recipe claim or schema drift",
        ));
    }
    if compile_partition_recipe(&recipe.input)? != *recipe {
        return Err(PhysicalPartitionError::Invalid(
            "partition-recipe identity drift",
        ));
    }
    Ok(())
}

pub fn compile_physical_region_partition(
    input: &PhysicalPartitionInputV1,
) -> Result<PhysicalRegionPartitionV1, PhysicalPartitionError> {
    let input_bytes = input.to_bytes()?;
    let partition_input_id = hash(b"mindwarp.physical-partition.input.v1", &input_bytes);
    let exposure_available = input
        .climate
        .state
        .content
        .absorbed_shortwave_quarter_billionths_earth
        > 0;
    let moisture_available = input
        .climate
        .input
        .hydrological
        .state
        .content
        .has_surface_accessible_liquid;

    let mut cells = BTreeMap::new();
    let mut ordered_indices = Vec::with_capacity(cell_count_usize(input)?);
    for x in 0..input.spatial_domain.input.cell_count[0] {
        for y in 0..input.spatial_domain.input.cell_count[1] {
            let index = CellIndex::new(x, y);
            let cell = build_spatial_cell(&input.spatial_domain, index)
                .map_err(PhysicalPartitionError::Spatial)?;
            let regional = compile_regional_environment_for_binding_cell(
                &input.regional_field_binding,
                &input.spatial_domain,
                index,
            )
            .map_err(PhysicalPartitionError::Regional)?;
            let signature = build_signature(
                &input.recipe.input.dimension_rules,
                regional.state.content.exposure_permille,
                regional.state.content.moisture_potential_permille,
                exposure_available,
                moisture_available,
            )?;
            ordered_indices.push(index);
            cells.insert(index, (cell.cell_id, signature));
        }
    }

    let mut components = build_components(
        &input.spatial_domain,
        partition_input_id,
        &ordered_indices,
        &cells,
    )?;
    attach_boundaries(&input.spatial_domain, &mut components)?;
    let partition_id = partition_identity(partition_input_id, &components)?;
    let result = PhysicalRegionPartitionV1 {
        schema_version: CONTRACT_VERSION,
        partition_input_id,
        physical_partition_recipe_id: input.recipe.physical_partition_recipe_id,
        spatial_domain_id: input.spatial_domain.spatial_domain_id,
        components,
        partition_id,
        limitations: partition_limitations(),
        authority_effect: "none_evidence_only".into(),
    };
    validate_result_shape(input, &result)?;
    Ok(result)
}

pub fn validate_physical_region_partition(
    input: &PhysicalPartitionInputV1,
    result: &PhysicalRegionPartitionV1,
) -> Result<(), PhysicalPartitionError> {
    validate_result_shape(input, result)?;
    if compile_physical_region_partition(input)? != *result {
        return Err(PhysicalPartitionError::Invalid(
            "physical-partition result drift",
        ));
    }
    Ok(())
}

fn validate_recipe_input(
    input: &PhysicalPartitionRecipeInputV1,
) -> Result<(), PhysicalPartitionError> {
    if input.schema_version != CONTRACT_VERSION
        || input.spatial_domain_contract_version != spatial_domain::CONTRACT_VERSION
        || input.regional_environment_contract_version
            != regional_environment_state::CONTRACT_VERSION
        || input.climate_contract_version != climate_state::CONTRACT_VERSION
    {
        return Err(PhysicalPartitionError::Invalid(
            "unsupported partition-recipe applicability",
        ));
    }
    if input.recipe_source_id == [0; 32]
        || input.scope_id == [0; 32]
        || input.recipe_revision == 0
        || input.dimension_rules.is_empty()
    {
        return Err(PhysicalPartitionError::Invalid(
            "missing partition-recipe binding",
        ));
    }
    let mut dimensions = BTreeSet::new();
    let mut previous_dimension = None;
    for rule in &input.dimension_rules {
        if previous_dimension.is_some_and(|previous| previous >= rule.dimension) {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical partition dimension order",
            ));
        }
        previous_dimension = Some(rule.dimension);
        if !dimensions.insert(rule.dimension) {
            return Err(PhysicalPartitionError::Invalid(
                "duplicate partition dimension",
            ));
        }
        if let DimensionClassifierV1::LowerBoundCuts { cuts } = &rule.classifier {
            if cuts.is_empty()
                || cuts.iter().any(|cut| *cut == 0 || *cut > 1_000)
                || cuts.windows(2).any(|pair| pair[0] >= pair[1])
            {
                return Err(PhysicalPartitionError::Invalid(
                    "invalid partition lower-bound cuts",
                ));
            }
        }
    }
    Ok(())
}

fn validate_partition_input(
    input: &PhysicalPartitionInputV1,
) -> Result<(), PhysicalPartitionError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(PhysicalPartitionError::Invalid(
            "unsupported physical-partition input schema",
        ));
    }
    validate_spatial_domain(&input.spatial_domain).map_err(PhysicalPartitionError::Spatial)?;
    input
        .regional_field_binding
        .to_bytes()
        .map_err(PhysicalPartitionError::Regional)?;
    validate_climate(&input.climate).map_err(PhysicalPartitionError::Climate)?;
    validate_partition_recipe(&input.recipe)?;
    let reconstruction_id = input.spatial_domain.input.reconstruction_id;
    if input.regional_field_binding.reconstruction_id != reconstruction_id
        || input.climate.input.reconstruction_id != reconstruction_id
    {
        return Err(PhysicalPartitionError::Invalid(
            "partition reconstruction mismatch",
        ));
    }
    let cell_count = cell_count(input)?;
    if cell_count > MAX_PARTITION_PROOF_CELLS {
        return Err(PhysicalPartitionError::Invalid(
            "partition proof-cell ceiling exceeded",
        ));
    }
    let max_edges = cell_count
        .checked_mul(2)
        .ok_or(PhysicalPartitionError::Invalid(
            "partition edge accounting overflow",
        ))?;
    if max_edges > MAX_PARTITION_PROOF_CELLS * 2 {
        return Err(PhysicalPartitionError::Invalid(
            "partition edge accounting exceeds proof bound",
        ));
    }
    Ok(())
}

fn build_signature(
    rules: &[DimensionRuleV1],
    exposure: u16,
    moisture: u16,
    exposure_available: bool,
    moisture_available: bool,
) -> Result<PhysicalSignatureV1, PhysicalPartitionError> {
    let dimensions = rules
        .iter()
        .map(|rule| {
            let (available, value) = match rule.dimension {
                PartitionDimensionV1::RegionalExposurePermille => (exposure_available, exposure),
                PartitionDimensionV1::RegionalMoisturePotentialPermille => {
                    (moisture_available, moisture)
                }
            };
            let value = if available {
                classify(value, &rule.classifier)?
            } else {
                SignatureValueV1::Unavailable
            };
            Ok(DimensionSignatureV1 {
                dimension: rule.dimension,
                value,
            })
        })
        .collect::<Result<Vec<_>, PhysicalPartitionError>>()?;
    Ok(PhysicalSignatureV1 {
        schema_version: CONTRACT_VERSION,
        dimensions,
    })
}

fn classify(
    value: u16,
    classifier: &DimensionClassifierV1,
) -> Result<SignatureValueV1, PhysicalPartitionError> {
    if value > 1_000 {
        return Err(PhysicalPartitionError::Invalid(
            "regional partition value outside permille range",
        ));
    }
    match classifier {
        DimensionClassifierV1::ExactValue => Ok(SignatureValueV1::Exact { value }),
        DimensionClassifierV1::LowerBoundCuts { cuts } => {
            let count = cuts.iter().filter(|cut| value >= **cut).count();
            let index = u16::try_from(count)
                .map_err(|_| PhysicalPartitionError::Invalid("partition bin-index overflow"))?;
            Ok(SignatureValueV1::Bin { index })
        }
    }
}

fn build_components(
    domain: &SpatialDomain,
    partition_input_id: Id,
    ordered_indices: &[CellIndex],
    cells: &BTreeMap<CellIndex, (Id, PhysicalSignatureV1)>,
) -> Result<Vec<PhysicalRegionComponentV1>, PhysicalPartitionError> {
    let mut visited = BTreeSet::new();
    let mut components = Vec::new();
    for start in ordered_indices {
        if visited.contains(start) {
            continue;
        }
        let signature = cells
            .get(start)
            .ok_or(PhysicalPartitionError::Invalid(
                "missing reconstructed cell",
            ))?
            .1
            .clone();
        let mut queue = VecDeque::from([*start]);
        visited.insert(*start);
        let mut member_indices = Vec::new();
        while let Some(index) = queue.pop_front() {
            member_indices.push(index);
            for neighbour in
                neighbour_indices(domain, index).map_err(PhysicalPartitionError::Spatial)?
            {
                if visited.contains(&neighbour) {
                    continue;
                }
                let neighbour_signature = &cells
                    .get(&neighbour)
                    .ok_or(PhysicalPartitionError::Invalid(
                        "missing reconstructed neighbour",
                    ))?
                    .1;
                if *neighbour_signature == signature {
                    visited.insert(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }
        member_indices.sort();
        let mut member_cell_ids = member_indices
            .iter()
            .map(|index| {
                cells
                    .get(index)
                    .map(|cell| cell.0)
                    .ok_or(PhysicalPartitionError::Invalid(
                        "missing component member cell",
                    ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        member_cell_ids.sort();
        let component_id = component_identity(
            partition_input_id,
            &signature,
            &member_indices,
            &member_cell_ids,
        )?;
        components.push(PhysicalRegionComponentV1 {
            schema_version: CONTRACT_VERSION,
            component_id,
            signature,
            member_indices,
            member_cell_ids,
            boundary_component_ids: Vec::new(),
        });
    }
    components.sort_by_key(|component| component.member_indices[0]);
    Ok(components)
}

fn attach_boundaries(
    domain: &SpatialDomain,
    components: &mut [PhysicalRegionComponentV1],
) -> Result<(), PhysicalPartitionError> {
    let mut owner = BTreeMap::new();
    for (component_index, component) in components.iter().enumerate() {
        for member in &component.member_indices {
            if owner.insert(*member, component_index).is_some() {
                return Err(PhysicalPartitionError::Invalid(
                    "duplicate component membership",
                ));
            }
        }
    }
    let ids: Vec<_> = components
        .iter()
        .map(|component| component.component_id)
        .collect();
    for (component_index, component) in components.iter_mut().enumerate() {
        let mut boundaries = BTreeSet::new();
        for member in &component.member_indices {
            for neighbour in
                neighbour_indices(domain, *member).map_err(PhysicalPartitionError::Spatial)?
            {
                let neighbour_owner =
                    *owner
                        .get(&neighbour)
                        .ok_or(PhysicalPartitionError::Invalid(
                            "missing neighbour component",
                        ))?;
                if neighbour_owner != component_index {
                    boundaries.insert(ids[neighbour_owner]);
                }
            }
        }
        component.boundary_component_ids = boundaries.into_iter().collect();
    }
    Ok(())
}

fn validate_result_shape(
    input: &PhysicalPartitionInputV1,
    result: &PhysicalRegionPartitionV1,
) -> Result<(), PhysicalPartitionError> {
    validate_partition_input(input)?;
    if result.schema_version != CONTRACT_VERSION
        || result.physical_partition_recipe_id != input.recipe.physical_partition_recipe_id
        || result.spatial_domain_id != input.spatial_domain.spatial_domain_id
        || result.limitations != partition_limitations()
        || result.authority_effect != "none_evidence_only"
        || result.components.is_empty()
    {
        return Err(PhysicalPartitionError::Invalid(
            "physical-partition result claim or binding drift",
        ));
    }
    let expected_input_id = hash(b"mindwarp.physical-partition.input.v1", &input.to_bytes()?);
    if result.partition_input_id != expected_input_id
        || result.partition_id != partition_identity(expected_input_id, &result.components)?
    {
        return Err(PhysicalPartitionError::Invalid(
            "physical-partition result identity drift",
        ));
    }
    let mut all_members = BTreeSet::new();
    let mut previous_start = None;
    let component_ids: BTreeSet<_> = result
        .components
        .iter()
        .map(|component| component.component_id)
        .collect();
    if component_ids.len() != result.components.len() {
        return Err(PhysicalPartitionError::Invalid(
            "duplicate physical component identity",
        ));
    }
    for component in &result.components {
        if component.schema_version != CONTRACT_VERSION
            || component.member_indices.is_empty()
            || component.member_indices.len() != component.member_cell_ids.len()
            || !strictly_sorted(&component.member_indices)
            || !strictly_sorted(&component.member_cell_ids)
            || !strictly_sorted(&component.boundary_component_ids)
            || component
                .boundary_component_ids
                .iter()
                .any(|id| *id == component.component_id || !component_ids.contains(id))
        {
            return Err(PhysicalPartitionError::Invalid(
                "malformed physical component",
            ));
        }
        if previous_start.is_some_and(|previous| previous >= component.member_indices[0]) {
            return Err(PhysicalPartitionError::Invalid(
                "noncanonical physical component order",
            ));
        }
        previous_start = Some(component.member_indices[0]);
        let mut expected_cell_ids = Vec::with_capacity(component.member_indices.len());
        for index in &component.member_indices {
            let rebuilt = build_spatial_cell(&input.spatial_domain, *index)
                .map_err(PhysicalPartitionError::Spatial)?;
            expected_cell_ids.push(rebuilt.cell_id);
            if !all_members.insert(*index) {
                return Err(PhysicalPartitionError::Invalid(
                    "forged or duplicate physical component member",
                ));
            }
        }
        expected_cell_ids.sort();
        if expected_cell_ids != component.member_cell_ids {
            return Err(PhysicalPartitionError::Invalid(
                "forged physical component cell identity",
            ));
        }
        if component.component_id
            != component_identity(
                expected_input_id,
                &component.signature,
                &component.member_indices,
                &component.member_cell_ids,
            )?
        {
            return Err(PhysicalPartitionError::Invalid(
                "physical component identity drift",
            ));
        }
    }
    if u64::try_from(all_members.len())
        .map_err(|_| PhysicalPartitionError::Invalid("partition membership count overflow"))?
        != cell_count(input)?
    {
        return Err(PhysicalPartitionError::Invalid(
            "partition membership is not exhaustive",
        ));
    }
    Ok(())
}

fn component_identity(
    partition_input_id: Id,
    signature: &PhysicalSignatureV1,
    member_indices: &[CellIndex],
    member_cell_ids: &[Id],
) -> Result<Id, PhysicalPartitionError> {
    #[derive(Serialize)]
    struct Identity<'a> {
        partition_input_id: Id,
        signature: &'a PhysicalSignatureV1,
        member_indices: &'a [CellIndex],
        member_cell_ids: &'a [Id],
    }
    Ok(hash(
        b"mindwarp.physical-region.component.v1",
        &encode(&Identity {
            partition_input_id,
            signature,
            member_indices,
            member_cell_ids,
        })?,
    ))
}

fn partition_identity(
    partition_input_id: Id,
    components: &[PhysicalRegionComponentV1],
) -> Result<Id, PhysicalPartitionError> {
    #[derive(Serialize)]
    struct Identity<'a> {
        partition_input_id: Id,
        components: &'a [PhysicalRegionComponentV1],
    }
    Ok(hash(
        b"mindwarp.physical-region.partition.v1",
        &encode(&Identity {
            partition_input_id,
            components,
        })?,
    ))
}

fn cell_count(input: &PhysicalPartitionInputV1) -> Result<u64, PhysicalPartitionError> {
    u64::from(input.spatial_domain.input.cell_count[0])
        .checked_mul(u64::from(input.spatial_domain.input.cell_count[1]))
        .ok_or(PhysicalPartitionError::Invalid(
            "partition cell-count overflow",
        ))
}

fn cell_count_usize(input: &PhysicalPartitionInputV1) -> Result<usize, PhysicalPartitionError> {
    usize::try_from(cell_count(input)?)
        .map_err(|_| PhysicalPartitionError::Invalid("partition allocation overflow"))
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn recipe_limitations() -> Vec<String> {
    [
        "content-authored physical classification policy; not a universal natural threshold",
        "no local tolerance clustering weighted score executable expression or biome semantics",
        "recipe identity grants no approval promotion runtime or persistence authority",
    ]
    .map(String::from)
    .to_vec()
}

fn partition_limitations() -> Vec<String> {
    [
        "bounded observer-independent physical components only; not biome habitat or organism evidence",
        "rectified proof topology is not a sphere planet surface terrain watershed or runtime map",
        "proof-cell ceiling is not production world size storage streaming or geometry policy",
        "no visibility traversability approval promotion persistence engine or external capability",
    ]
    .map(String::from)
    .to_vec()
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, PhysicalPartitionError> {
    serde_json::to_vec(value).map_err(|error| PhysicalPartitionError::Codec(error.to_string()))
}

fn decode<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, PhysicalPartitionError> {
    serde_json::from_slice(bytes).map_err(|error| PhysicalPartitionError::Codec(error.to_string()))
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
    use climate_state::{ClimateInput, compile_climate};
    use field_basis::{FieldRecipe, ONE, Term};
    use geological_atmospheric::{GeologicalAtmosphericInput, compile_geological_atmospheric};
    use hydrological_state::{HydrologicalInput, compile_hydrological};
    use spatial_domain::{
        Adjacency, BoundaryMode, CoordinateFrame, SpatialDomainInput, compile_spatial_domain,
    };
    use stellar_orbital::{StellarOrbitalInput, compile_stellar_orbital};

    const RECONSTRUCTION: Id = [1; 32];

    fn climate(albedo: u16, accessible_liquid: u16) -> ClimateContract {
        let stellar = compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: stellar_orbital::CONTRACT_VERSION,
            reconstruction_id: RECONSTRUCTION,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap();
        let geological = compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: geological_atmospheric::CONTRACT_VERSION,
            reconstruction_id: RECONSTRUCTION,
            planetary_body_id: [4; 32],
            stellar_orbital: stellar,
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000; 3],
        })
        .unwrap();
        let hydrological = compile_hydrological(&HydrologicalInput {
            schema_version: hydrological_state::CONTRACT_VERSION,
            reconstruction_id: RECONSTRUCTION,
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological,
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: accessible_liquid,
        })
        .unwrap();
        compile_climate(&ClimateInput {
            schema_version: climate_state::CONTRACT_VERSION,
            reconstruction_id: RECONSTRUCTION,
            climate_source_id: [6; 32],
            hydrological,
            bond_albedo_permille: albedo,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }

    fn binding(exposure: i64, moisture: i64) -> RegionalFieldBindingV1 {
        RegionalFieldBindingV1 {
            schema_version: regional_environment_state::CONTRACT_VERSION,
            reconstruction_id: RECONSTRUCTION,
            regional_source_id: [8; 32],
            field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(exposure)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            moisture_source_id: [9; 32],
            moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(moisture)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
        }
    }

    fn domain(cell_count: [u32; 2]) -> SpatialDomain {
        compile_spatial_domain(&SpatialDomainInput {
            schema_version: spatial_domain::CONTRACT_VERSION,
            logical_world_id: [2; 32],
            reconstruction_id: RECONSTRUCTION,
            coordinate_frame: CoordinateFrame::FieldQ32_32Cartesian2dV1,
            cell_center_origin_q32_32: [0, 0],
            cell_step_q32_32: [1_u64 << 32, 1_u64 << 32],
            cell_count,
            adjacency: Adjacency::SharedEdge4,
            boundary_mode: BoundaryMode::BoundedAbsent,
        })
        .unwrap()
    }

    fn recipe_input() -> PhysicalPartitionRecipeInputV1 {
        PhysicalPartitionRecipeInputV1 {
            schema_version: CONTRACT_VERSION,
            recipe_source_id: [10; 32],
            scope_id: [11; 32],
            recipe_revision: 1,
            spatial_domain_contract_version: spatial_domain::CONTRACT_VERSION,
            regional_environment_contract_version: regional_environment_state::CONTRACT_VERSION,
            climate_contract_version: climate_state::CONTRACT_VERSION,
            dimension_rules: vec![
                DimensionRuleV1 {
                    dimension: PartitionDimensionV1::RegionalExposurePermille,
                    classifier: DimensionClassifierV1::LowerBoundCuts {
                        cuts: vec![250, 750],
                    },
                },
                DimensionRuleV1 {
                    dimension: PartitionDimensionV1::RegionalMoisturePotentialPermille,
                    classifier: DimensionClassifierV1::ExactValue,
                },
            ],
        }
    }

    fn input(cell_count: [u32; 2]) -> PhysicalPartitionInputV1 {
        PhysicalPartitionInputV1 {
            schema_version: CONTRACT_VERSION,
            spatial_domain: domain(cell_count),
            regional_field_binding: binding(0, 0),
            climate: climate(300, 700),
            recipe: compile_partition_recipe(&recipe_input()).unwrap(),
        }
    }

    #[test]
    fn deterministic_replay_strict_codec_and_uniform_connectivity() {
        let input = input([3, 2]);
        let first = compile_physical_region_partition(&input).unwrap();
        let second = compile_physical_region_partition(&input).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.components.len(), 1);
        assert_eq!(first.components[0].member_indices.len(), 6);
        let bytes = first.to_bytes(&input).unwrap();
        assert_eq!(
            PhysicalRegionPartitionV1::from_bytes(&input, &bytes).unwrap(),
            first
        );
        let mut padded = bytes;
        padded.push(b' ');
        assert!(PhysicalRegionPartitionV1::from_bytes(&input, &padded).is_err());
    }

    #[test]
    fn lower_bound_cuts_are_exact_at_boundaries() {
        let classifier = DimensionClassifierV1::LowerBoundCuts {
            cuts: vec![250, 750],
        };
        assert_eq!(
            classify(249, &classifier).unwrap(),
            SignatureValueV1::Bin { index: 0 }
        );
        assert_eq!(
            classify(250, &classifier).unwrap(),
            SignatureValueV1::Bin { index: 1 }
        );
        assert_eq!(
            classify(749, &classifier).unwrap(),
            SignatureValueV1::Bin { index: 1 }
        );
        assert_eq!(
            classify(750, &classifier).unwrap(),
            SignatureValueV1::Bin { index: 2 }
        );
    }

    #[test]
    fn availability_is_climate_derived_and_distinct_from_numeric_zero() {
        let mut unavailable = input([1, 1]);
        unavailable.climate = climate(1_000, 0);
        unavailable.regional_field_binding = binding(-ONE, -ONE);
        let result = compile_physical_region_partition(&unavailable).unwrap();
        assert!(
            result.components[0]
                .signature
                .dimensions
                .iter()
                .all(|dimension| dimension.value == SignatureValueV1::Unavailable)
        );

        let mut available = unavailable;
        available.climate = climate(300, 700);
        let result = compile_physical_region_partition(&available).unwrap();
        assert_eq!(
            result.components[0].signature.dimensions[0].value,
            SignatureValueV1::Bin { index: 0 }
        );
        assert_eq!(
            result.components[0].signature.dimensions[1].value,
            SignatureValueV1::Exact { value: 0 }
        );
    }

    #[test]
    fn recipe_rejects_duplicate_reordered_and_invalid_rules() {
        let mut duplicate = recipe_input();
        duplicate.dimension_rules[1].dimension = PartitionDimensionV1::RegionalExposurePermille;
        assert!(compile_partition_recipe(&duplicate).is_err());
        let mut reordered = recipe_input();
        reordered.dimension_rules.reverse();
        assert!(compile_partition_recipe(&reordered).is_err());
        let mut bad_cuts = recipe_input();
        bad_cuts.dimension_rules[0].classifier = DimensionClassifierV1::LowerBoundCuts {
            cuts: vec![750, 250],
        };
        assert!(compile_partition_recipe(&bad_cuts).is_err());
    }

    #[test]
    fn unknown_classifier_and_noncanonical_input_bytes_fail_closed() {
        let bytes = recipe_input().to_bytes().unwrap();
        let json = String::from_utf8(bytes).unwrap();
        let forged = json.replace("\"lower_bound_cuts\"", "\"local_tolerance\"");
        assert!(PhysicalPartitionRecipeInputV1::from_bytes(forged.as_bytes()).is_err());
        let mut padded = recipe_input().to_bytes().unwrap();
        padded.push(b'\n');
        assert!(PhysicalPartitionRecipeInputV1::from_bytes(&padded).is_err());
    }

    #[test]
    fn reconstruction_mismatch_and_proof_ceiling_fail_before_sampling() {
        let mut mismatch = input([1, 1]);
        mismatch.regional_field_binding.reconstruction_id = [12; 32];
        assert!(compile_physical_region_partition(&mismatch).is_err());
        let oversized = input([257, 256]);
        assert!(matches!(
            compile_physical_region_partition(&oversized),
            Err(PhysicalPartitionError::Invalid(
                "partition proof-cell ceiling exceeded"
            ))
        ));
    }

    #[test]
    fn forged_membership_identity_boundary_and_authority_fail_closed() {
        let input = input([2, 1]);
        let result = compile_physical_region_partition(&input).unwrap();
        let mut forged = result.clone();
        forged.components[0].member_indices.pop();
        forged.components[0].member_cell_ids.pop();
        assert!(validate_physical_region_partition(&input, &forged).is_err());
        let mut forged = result.clone();
        forged.components[0].component_id = [99; 32];
        assert!(validate_physical_region_partition(&input, &forged).is_err());
        let mut forged = result.clone();
        forged.authority_effect = "promote".into();
        assert!(validate_physical_region_partition(&input, &forged).is_err());
    }

    #[test]
    fn disconnected_equal_signatures_remain_distinct_without_edge_wrap() {
        let domain = domain([3, 1]);
        let a = PhysicalSignatureV1 {
            schema_version: CONTRACT_VERSION,
            dimensions: vec![DimensionSignatureV1 {
                dimension: PartitionDimensionV1::RegionalExposurePermille,
                value: SignatureValueV1::Exact { value: 1 },
            }],
        };
        let mut b = a.clone();
        b.dimensions[0].value = SignatureValueV1::Exact { value: 2 };
        let indices = vec![
            CellIndex::new(0, 0),
            CellIndex::new(1, 0),
            CellIndex::new(2, 0),
        ];
        let cells = indices
            .iter()
            .map(|index| {
                let cell = build_spatial_cell(&domain, *index).unwrap();
                let signature = if index.x == 1 { b.clone() } else { a.clone() };
                (*index, (cell.cell_id, signature))
            })
            .collect();
        let mut components = build_components(&domain, [7; 32], &indices, &cells).unwrap();
        attach_boundaries(&domain, &mut components).unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].signature, components[2].signature);
        assert_ne!(components[0].component_id, components[2].component_id);
        assert!(
            !components[0]
                .boundary_component_ids
                .contains(&components[2].component_id)
        );
        assert_eq!(components[1].boundary_component_ids.len(), 2);
    }

    #[test]
    fn component_construction_is_traversal_order_independent() {
        let input = input([2, 2]);
        let mut cells = BTreeMap::new();
        let signature =
            build_signature(&input.recipe.input.dimension_rules, 500, 500, true, true).unwrap();
        let mut indices = Vec::new();
        for x in 0..2 {
            for y in 0..2 {
                let index = CellIndex::new(x, y);
                let cell = build_spatial_cell(&input.spatial_domain, index).unwrap();
                indices.push(index);
                cells.insert(index, (cell.cell_id, signature.clone()));
            }
        }
        let forward = build_components(&input.spatial_domain, [7; 32], &indices, &cells).unwrap();
        indices.reverse();
        let reverse = build_components(&input.spatial_domain, [7; 32], &indices, &cells).unwrap();
        assert_eq!(forward, reverse);
    }

    #[test]
    fn exact_bindings_rekey_partition_identity() {
        let first_input = input([1, 1]);
        let first = compile_physical_region_partition(&first_input).unwrap();
        let mut second_input = first_input.clone();
        second_input.recipe.input.recipe_revision = 2;
        second_input.recipe = compile_partition_recipe(&second_input.recipe.input).unwrap();
        let second = compile_physical_region_partition(&second_input).unwrap();
        assert_ne!(first.partition_input_id, second.partition_input_id);
        assert_ne!(first.partition_id, second.partition_id);
    }
}
