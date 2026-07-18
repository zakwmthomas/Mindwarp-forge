#![deny(warnings)]
//! Immutable, capability-free calibrated source-energy allocation evidence.

use calibrated_spectral_time_basis::{
    CalibratedBandV1, CalibratedSpectralTimeBasisV1, validate_calibrated_spectral_time_basis,
};
use fixed_interval_arithmetic::Signed512;
use optical_phase_space_cell_binding::{
    OpticalPhaseSpaceCellV1, OpticalPhaseSpaceSplitQueryV1, PhaseSpaceParameterAxisV1,
    PhaseSpaceSplitSideV1, PhaseSpaceSplitStepV1, PositiveRationalV1,
    split_optical_phase_space_cell,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_FRONTIER_ALLOCATIONS: usize = 64;
pub const MAX_REFINEMENT_DIRECTIVES: usize = 63;
pub const MAX_QUERY_BYTES: usize = 128 * 1024;
pub const MAX_RESULT_BYTES: usize = 256 * 1024;
pub const MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_ENERGY_LIVE_BITS: u16 = 385;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str = "no transport applicability spatial calibration detector visibility runtime promotion supersession or C3 closure";

const SUBJECT_DOMAIN: &[u8] = b"mindwarp.calibrated-source-energy-distribution.subject.v1";
const ALLOCATION_DOMAIN: &[u8] = b"mindwarp.calibrated-source-energy-distribution.allocation.v1";
const SPLIT_DOMAIN: &[u8] = b"mindwarp.calibrated-source-energy-distribution.split.v1";
const DISTRIBUTION_DOMAIN: &[u8] =
    b"mindwarp.calibrated-source-energy-distribution.distribution.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceEnergyDistributionError {
    InvalidSchema,
    InvalidProvenance,
    ProvenanceConflation,
    InvalidRoot,
    NoncanonicalEnergy,
    NonFrontierParent,
    InvalidResolution,
    EnergyConservationMismatch,
    ArithmeticShieldExceeded,
    ArithmeticDefect,
    ResourceCeiling,
    UpstreamReplayDefect,
    IdentityMismatch,
    CodecDefect,
}

impl fmt::Display for SourceEnergyDistributionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for SourceEnergyDistributionError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRadiantEnergyV1 {
    pub denominator: String,
    pub numerator: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceEnergyResolutionV1 {
    ResolvedLeaf,
    UnresolvedWithinCell,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceEnergyRefinementDirectiveV1 {
    pub parent_allocation_id: [u8; 32],
    pub axis: PhaseSpaceParameterAxisV1,
    pub lower_joules: ExactRadiantEnergyV1,
    pub upper_joules: ExactRadiantEnergyV1,
    pub lower_resolution: SourceEnergyResolutionV1,
    pub upper_resolution: SourceEnergyResolutionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSourceEnergyDistributionQueryV1 {
    pub schema_version: u16,
    pub calibrated_basis: CalibratedSpectralTimeBasisV1,
    pub selected_band: CalibratedBandV1,
    pub source_provenance_id: [u8; 32],
    pub root_cell: OpticalPhaseSpaceCellV1,
    pub root_joules: ExactRadiantEnergyV1,
    pub directives: Vec<SourceEnergyRefinementDirectiveV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSourceEnergyAllocationV1 {
    pub subject_id: [u8; 32],
    pub cell_id: [u8; 32],
    pub path: Vec<PhaseSpaceSplitStepV1>,
    pub measure: PositiveRationalV1,
    pub joules: ExactRadiantEnergyV1,
    pub resolution: SourceEnergyResolutionV1,
    pub allocation_id: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSourceEnergySplitReceiptV1 {
    pub subject_id: [u8; 32],
    pub parent_allocation_id: [u8; 32],
    pub phase_split_id: [u8; 32],
    pub child_allocation_ids: [[u8; 32]; 2],
    pub energy_split_id: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceEnergyResourceReceiptV1 {
    pub directive_count: u16,
    pub split_receipt_count: u16,
    pub frontier_allocation_count: u16,
    pub query_bytes: u32,
    pub result_bytes_before_resource_receipt: u32,
    pub aggregate_live_canonical_bytes: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CalibratedSourceEnergyDistributionV1 {
    pub schema_version: u16,
    pub subject_id: [u8; 32],
    pub root_allocation: CalibratedSourceEnergyAllocationV1,
    pub split_receipts: Vec<CalibratedSourceEnergySplitReceiptV1>,
    pub frontier_allocations: Vec<CalibratedSourceEnergyAllocationV1>,
    pub distribution_id: [u8; 32],
    pub maximum_energy_arithmetic_live_bits: u16,
    pub resource_receipt: SourceEnergyResourceReceiptV1,
    pub authority_effect: String,
    pub limitations: String,
}

#[derive(Serialize)]
struct SubjectIdentity<'a> {
    source_id: [u8; 32],
    scope_id: [u8; 32],
    source_provenance_id: [u8; 32],
    source_revision: u32,
    calibrated_basis_id: [u8; 32],
    selected_band: CalibratedBandV1,
    band_time_id: [u8; 32],
    reconstruction_id: [u8; 32],
    root_id: [u8; 32],
    marker: &'a str,
}

impl ExactRadiantEnergyV1 {
    fn parsed(&self) -> Result<(u128, u128), SourceEnergyDistributionError> {
        let numerator = canonical_u128(&self.numerator, false)?;
        let denominator = canonical_u128(&self.denominator, true)?;
        if gcd(numerator, denominator) != 1 {
            return Err(SourceEnergyDistributionError::NoncanonicalEnergy);
        }
        Ok((numerator, denominator))
    }
}

impl CalibratedSourceEnergyDistributionQueryV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SourceEnergyDistributionError> {
        let bytes = json(self)?;
        if bytes.len() > MAX_QUERY_BYTES {
            return Err(SourceEnergyDistributionError::ResourceCeiling);
        }
        validate_query_envelope(self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SourceEnergyDistributionError> {
        if bytes.len() > MAX_QUERY_BYTES {
            return Err(SourceEnergyDistributionError::ResourceCeiling);
        }
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|_| SourceEnergyDistributionError::CodecDefect)?;
        if value.to_bytes()? != bytes {
            return Err(SourceEnergyDistributionError::CodecDefect);
        }
        Ok(value)
    }
}

impl CalibratedSourceEnergyDistributionV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SourceEnergyDistributionError> {
        let bytes = json(self)?;
        if bytes.len() > MAX_RESULT_BYTES {
            Err(SourceEnergyDistributionError::ResourceCeiling)
        } else {
            Ok(bytes)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SourceEnergyDistributionError> {
        if bytes.len() > MAX_RESULT_BYTES {
            return Err(SourceEnergyDistributionError::ResourceCeiling);
        }
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|_| SourceEnergyDistributionError::CodecDefect)?;
        if value.schema_version != CONTRACT_VERSION
            || value.authority_effect != AUTHORITY_EFFECT_NONE
            || value.limitations != LIMITATIONS_V1
            || value.split_receipts.len() > MAX_REFINEMENT_DIRECTIVES
            || value.frontier_allocations.len() > MAX_FRONTIER_ALLOCATIONS
            || usize::from(value.resource_receipt.split_receipt_count) != value.split_receipts.len()
            || usize::from(value.resource_receipt.frontier_allocation_count)
                != value.frontier_allocations.len()
            || value.maximum_energy_arithmetic_live_bits > MAX_ENERGY_LIVE_BITS
            || json(&value)? != bytes
        {
            return Err(SourceEnergyDistributionError::CodecDefect);
        }
        Ok(value)
    }
}

pub fn compile_calibrated_source_energy_distribution(
    query: CalibratedSourceEnergyDistributionQueryV1,
) -> Result<CalibratedSourceEnergyDistributionV1, SourceEnergyDistributionError> {
    let query_bytes = query.to_bytes()?.len();
    let subject_id = derive_subject(&query)?;
    let root_allocation = allocation(
        subject_id,
        &query.root_cell,
        query.root_joules.clone(),
        SourceEnergyResolutionV1::UnresolvedWithinCell,
    )?;
    let mut frontier = BTreeMap::new();
    frontier.insert(
        root_allocation.allocation_id,
        (query.root_cell.clone(), root_allocation.clone()),
    );
    let mut receipts = Vec::with_capacity(query.directives.len());
    let mut maximum_bits = 0;

    for directive in &query.directives {
        let (parent_cell, parent) = frontier
            .get(&directive.parent_allocation_id)
            .cloned()
            .ok_or(SourceEnergyDistributionError::NonFrontierParent)?;
        if parent.resolution == SourceEnergyResolutionV1::ResolvedLeaf {
            return Err(SourceEnergyDistributionError::InvalidResolution);
        }
        maximum_bits = maximum_bits.max(prove_conservation(
            &parent.joules,
            &directive.lower_joules,
            &directive.upper_joules,
        )?);
        let phase = split_optical_phase_space_cell(&OpticalPhaseSpaceSplitQueryV1 {
            schema_version: 1,
            cell: parent_cell,
            axis: directive.axis,
        })
        .map_err(|_| SourceEnergyDistributionError::UpstreamReplayDefect)?;
        let lower = allocation(
            subject_id,
            &phase.children[0],
            directive.lower_joules.clone(),
            directive.lower_resolution,
        )?;
        let upper = allocation(
            subject_id,
            &phase.children[1],
            directive.upper_joules.clone(),
            directive.upper_resolution,
        )?;
        let child_ids = [lower.allocation_id, upper.allocation_id];
        let split_id = hash(
            SPLIT_DOMAIN,
            &json(&(subject_id, parent.allocation_id, phase.split_id, child_ids))?,
        );
        frontier.remove(&directive.parent_allocation_id);
        frontier.insert(lower.allocation_id, (phase.children[0].clone(), lower));
        frontier.insert(upper.allocation_id, (phase.children[1].clone(), upper));
        if frontier.len() > MAX_FRONTIER_ALLOCATIONS {
            return Err(SourceEnergyDistributionError::ResourceCeiling);
        }
        receipts.push(CalibratedSourceEnergySplitReceiptV1 {
            subject_id,
            parent_allocation_id: parent.allocation_id,
            phase_split_id: phase.split_id,
            child_allocation_ids: child_ids,
            energy_split_id: split_id,
        });
    }

    let mut frontier_allocations: Vec<_> = frontier.into_values().map(|(_, value)| value).collect();
    frontier_allocations.sort_by(|left, right| path_key(&left.path).cmp(&path_key(&right.path)));
    let split_ids: Vec<_> = receipts.iter().map(|value| value.energy_split_id).collect();
    let frontier_ids: Vec<_> = frontier_allocations
        .iter()
        .map(|value| value.allocation_id)
        .collect();
    let distribution_id = hash(
        DISTRIBUTION_DOMAIN,
        &json(&(
            subject_id,
            root_allocation.allocation_id,
            split_ids,
            frontier_ids,
        ))?,
    );
    let mut result = CalibratedSourceEnergyDistributionV1 {
        schema_version: CONTRACT_VERSION,
        subject_id,
        root_allocation,
        split_receipts: receipts,
        frontier_allocations,
        distribution_id,
        maximum_energy_arithmetic_live_bits: maximum_bits,
        resource_receipt: SourceEnergyResourceReceiptV1 {
            directive_count: query.directives.len() as u16,
            split_receipt_count: query.directives.len() as u16,
            frontier_allocation_count: (query.directives.len() + 1) as u16,
            query_bytes: query_bytes as u32,
            result_bytes_before_resource_receipt: 0,
            aggregate_live_canonical_bytes: 0,
        },
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
        limitations: LIMITATIONS_V1.into(),
    };
    let provisional_bytes = json(&result)?.len();
    let live_bytes = query_bytes
        .checked_add(provisional_bytes)
        .and_then(|value| value.checked_add(64 * 1024))
        .and_then(|value| value.checked_add(result.frontier_allocations.len() * 32 * 1024))
        .ok_or(SourceEnergyDistributionError::ResourceCeiling)?;
    if provisional_bytes > MAX_RESULT_BYTES || live_bytes > MAX_AGGREGATE_LIVE_CANONICAL_BYTES {
        return Err(SourceEnergyDistributionError::ResourceCeiling);
    }
    result.resource_receipt.result_bytes_before_resource_receipt = provisional_bytes as u32;
    result.resource_receipt.aggregate_live_canonical_bytes = live_bytes as u32;
    if result.to_bytes()?.len() > MAX_RESULT_BYTES {
        return Err(SourceEnergyDistributionError::ResourceCeiling);
    }
    Ok(result)
}

pub fn replay_calibrated_source_energy_distribution(
    query: CalibratedSourceEnergyDistributionQueryV1,
    claimed: &CalibratedSourceEnergyDistributionV1,
) -> Result<(), SourceEnergyDistributionError> {
    let expected = compile_calibrated_source_energy_distribution(query)?;
    if &expected == claimed {
        Ok(())
    } else {
        Err(SourceEnergyDistributionError::IdentityMismatch)
    }
}

fn validate_query_envelope(
    query: &CalibratedSourceEnergyDistributionQueryV1,
) -> Result<(), SourceEnergyDistributionError> {
    if query.schema_version != CONTRACT_VERSION {
        return Err(SourceEnergyDistributionError::InvalidSchema);
    }
    if query.directives.len() > MAX_REFINEMENT_DIRECTIVES {
        return Err(SourceEnergyDistributionError::ResourceCeiling);
    }
    validate_calibrated_spectral_time_basis(&query.calibrated_basis)
        .map_err(|_| SourceEnergyDistributionError::UpstreamReplayDefect)?;
    query
        .root_cell
        .to_bytes()
        .map_err(|_| SourceEnergyDistributionError::UpstreamReplayDefect)?;
    if query.root_cell.depth != 0
        || !query.root_cell.path.is_empty()
        || query.root_cell.parent_id.is_some()
        || query.root_cell.cell_id != query.root_cell.root_id
    {
        return Err(SourceEnergyDistributionError::InvalidRoot);
    }
    if query.source_provenance_id == [0; 32] {
        return Err(SourceEnergyDistributionError::InvalidProvenance);
    }
    if query.source_provenance_id == query.calibrated_basis.input.calibration_provenance_id {
        return Err(SourceEnergyDistributionError::ProvenanceConflation);
    }
    query.root_joules.parsed()?;
    for directive in &query.directives {
        directive.lower_joules.parsed()?;
        directive.upper_joules.parsed()?;
    }
    Ok(())
}

fn derive_subject(
    query: &CalibratedSourceEnergyDistributionQueryV1,
) -> Result<[u8; 32], SourceEnergyDistributionError> {
    let payload = SubjectIdentity {
        source_id: query.root_cell.source_id,
        scope_id: query.root_cell.scope_id,
        source_provenance_id: query.source_provenance_id,
        source_revision: query.root_cell.source_revision,
        calibrated_basis_id: query.calibrated_basis.calibrated_basis_id,
        selected_band: query.selected_band,
        band_time_id: query
            .calibrated_basis
            .derived_legacy_band_time_ids
            .get(query.selected_band),
        reconstruction_id: query.root_cell.reconstruction_id,
        root_id: query.root_cell.root_id,
        marker: "calibrated_source_energy_distribution_v1",
    };
    Ok(hash(SUBJECT_DOMAIN, &json(&payload)?))
}

fn allocation(
    subject_id: [u8; 32],
    cell: &OpticalPhaseSpaceCellV1,
    joules: ExactRadiantEnergyV1,
    resolution: SourceEnergyResolutionV1,
) -> Result<CalibratedSourceEnergyAllocationV1, SourceEnergyDistributionError> {
    joules.parsed()?;
    let allocation_id = hash(
        ALLOCATION_DOMAIN,
        &json(&(subject_id, cell.cell_id, &joules, resolution))?,
    );
    Ok(CalibratedSourceEnergyAllocationV1 {
        subject_id,
        cell_id: cell.cell_id,
        path: cell.path.clone(),
        measure: cell.measure.clone(),
        joules,
        resolution,
        allocation_id,
    })
}

fn prove_conservation(
    parent: &ExactRadiantEnergyV1,
    lower: &ExactRadiantEnergyV1,
    upper: &ExactRadiantEnergyV1,
) -> Result<u16, SourceEnergyDistributionError> {
    let (a, b) = parent.parsed()?;
    let (c, d) = lower.parsed()?;
    let (e, f) = upper.parsed()?;
    let values = [a, b, c, d, e, f].map(|value| {
        Signed512::from_canonical_decimal(&value.to_string())
            .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)
    });
    let [a, b, c, d, e, f] = values
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let cd = c
        .checked_mul(&f)
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let ed = e
        .checked_mul(&d)
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let sum = cd
        .checked_add(&ed)
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let left = a
        .checked_mul(&d)
        .and_then(|value| value.checked_mul(&f))
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let right = b
        .checked_mul(&sum)
        .map_err(|_| SourceEnergyDistributionError::ArithmeticDefect)?;
    let maximum = [a, b, c, d, e, f, cd, ed, sum, left.clone(), right.clone()]
        .iter()
        .map(Signed512::maximum_magnitude_bits)
        .max()
        .unwrap_or(0);
    if maximum > MAX_ENERGY_LIVE_BITS {
        return Err(SourceEnergyDistributionError::ArithmeticShieldExceeded);
    }
    if left != right {
        return Err(SourceEnergyDistributionError::EnergyConservationMismatch);
    }
    Ok(maximum)
}

fn canonical_u128(value: &str, positive: bool) -> Result<u128, SourceEnergyDistributionError> {
    if value.is_empty()
        || value.len() > 39
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(SourceEnergyDistributionError::NoncanonicalEnergy);
    }
    let parsed = value
        .parse::<u128>()
        .map_err(|_| SourceEnergyDistributionError::NoncanonicalEnergy)?;
    if positive && parsed == 0 {
        return Err(SourceEnergyDistributionError::NoncanonicalEnergy);
    }
    Ok(parsed)
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left
}

fn path_key(path: &[PhaseSpaceSplitStepV1]) -> Vec<u8> {
    path.iter()
        .flat_map(|step| {
            let axis = match step.axis {
                PhaseSpaceParameterAxisV1::U0 => 0,
                PhaseSpaceParameterAxisV1::U1 => 1,
                PhaseSpaceParameterAxisV1::U2 => 2,
                PhaseSpaceParameterAxisV1::U3 => 3,
            };
            let side = match step.side {
                PhaseSpaceSplitSideV1::Lower => 0,
                PhaseSpaceSplitSideV1::Upper => 1,
            };
            [axis, side]
        })
        .collect()
}

fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, SourceEnergyDistributionError> {
    serde_json::to_vec(value).map_err(|_| SourceEnergyDistributionError::CodecDefect)
}

fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
