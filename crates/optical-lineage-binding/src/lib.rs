//! Capability-free ordered binding of already-validated local optical evidence.
//!
//! This crate owns lineage identity and adjacency only. It owns no physical
//! traversal, interface equation, attenuation kernel, cumulative power,
//! receiver arrival, visibility, runtime behavior, approval, or promotion.

use physical_path_substrate::{
    ConditionalIntervalCellStepOutcomeV1, Id, PhysicalVolumeRecipeV1, PhysicalVolumeV1,
    SignedDecimalIntervalV1, build_physical_cell, compile_physical_volume,
    compile_physical_volume_recipe,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use visible_radiance_bulk_transfer::{
    ConditionalIntervalBulkOutcomeV1, ConditionalIntervalBulkQueryV1,
    ConditionalIntervalBulkTransferV1, IntervalBulkTerminalV1, VisibleRadianceBandV1,
    VisibleRadianceBulkProfileV1, validate_conditional_interval_bulk_transfer,
    validate_visible_radiance_bulk_profile,
};
use visible_radiance_interface_event::{
    DecimalIntervalV1, FixedScaleV1, IntervalBandOutcomeV1, IntervalInterfaceOutcomeV1,
    IntervalUniformBranchV1, VisibleRadianceIntervalInterfaceEventV1,
    VisibleRadianceIntervalInterfaceInputV1, validate_visible_radiance_interval_interface_event,
};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_LINEAGE_STEPS: usize = 64;
pub const MAX_LINEAGE_MANIFEST_BYTES: usize = 1024 * 1024;
pub const MAX_LINEAGE_BUNDLE_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_LINEAGE_OBJECTS: usize = 384;

const LANE_DOMAIN: &[u8] = b"mindwarp.optical-lineage.lane.v1";
const DERIVED_SOURCE_DOMAIN: &[u8] = b"mindwarp.optical-lineage.derived-source.v1";
const STEP_DOMAIN: &[u8] = b"mindwarp.optical-lineage.step.v1";
const BUNDLE_DOMAIN: &[u8] = b"mindwarp.optical-lineage.bundle-receipt.v1";
const TRANSCRIPT_DOMAIN: &[u8] = b"mindwarp.optical-lineage.transcript.v1";

#[derive(Debug, Eq, PartialEq)]
pub enum OpticalLineageError {
    Invalid(&'static str),
    Codec(String),
}

impl fmt::Display for OpticalLineageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Codec(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for OpticalLineageError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLineageStepEvidenceV1 {
    pub bulk_query: ConditionalIntervalBulkQueryV1,
    pub bulk_transfer: ConditionalIntervalBulkTransferV1,
    pub interface_input: Option<VisibleRadianceIntervalInterfaceInputV1>,
    pub interface_event: Option<VisibleRadianceIntervalInterfaceEventV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLineageBundleInputV1 {
    pub schema_version: u16,
    pub lane_source_id: Id,
    pub profile: VisibleRadianceBulkProfileV1,
    pub band: VisibleRadianceBandV1,
    pub steps: Vec<OpticalLineageStepEvidenceV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLineageObjectReceiptV1 {
    pub object_id: Id,
    pub canonical_sha256: Id,
    pub canonical_bytes: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLineageBundleReceiptV1 {
    pub object_count: u16,
    pub canonical_bytes: u32,
    pub entries_sha256: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpticalLineageTerminalV1 {
    OuterDomainExit,
    UnavailableNeighbor,
    UnavailableCurrent,
    AmbiguousNextFace,
    NoForwardProgress,
    AllTir,
    AmbiguousInterfaceBranch,
    NonconvergentInterface,
    UnsupportedInterfaceModel,
    WorkExhaustion,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum OpticalLineageDispositionV1 {
    ContinueSameMedium,
    ContinueAfterInterface,
    Terminal { terminal: OpticalLineageTerminalV1 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpticalLineageDerivedSourceRoleV1 {
    CellInput,
    InterfaceInput,
}

/// Derives the stable identity of a lane from its first already-compiled cell input.
/// This exposes the frozen identity rule without granting any traversal authority.
pub fn derive_optical_lane_id(
    reconstruction_id: Id,
    visible_radiance_bulk_profile_id: Id,
    band: VisibleRadianceBandV1,
    initial_interval_cell_step_input_id: Id,
    lane_source_id: Id,
) -> Result<Id, OpticalLineageError> {
    if reconstruction_id == [0; 32]
        || visible_radiance_bulk_profile_id == [0; 32]
        || initial_interval_cell_step_input_id == [0; 32]
        || lane_source_id == [0; 32]
    {
        return Err(OpticalLineageError::Invalid("lineage identity provenance"));
    }
    Ok(domain_hash(
        LANE_DOMAIN,
        &encode(&(
            reconstruction_id,
            visible_radiance_bulk_profile_id,
            band,
            initial_interval_cell_step_input_id,
            lane_source_id,
        ))?,
    ))
}

/// Derives an adjacency-bound source identity for a later owner input.
pub fn derive_optical_lineage_source_id(
    lane_id: Id,
    ordinal: u8,
    predecessor_step_id: Option<Id>,
    role: OpticalLineageDerivedSourceRoleV1,
) -> Result<Id, OpticalLineageError> {
    if lane_id == [0; 32] {
        return Err(OpticalLineageError::Invalid("lineage identity provenance"));
    }
    let role = match role {
        OpticalLineageDerivedSourceRoleV1::CellInput => b"cell_input".as_slice(),
        OpticalLineageDerivedSourceRoleV1::InterfaceInput => b"interface_input".as_slice(),
    };
    derived_source(lane_id, ordinal, predecessor_step_id, role)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLaneStepV1 {
    pub lane_id: Id,
    pub ordinal: u8,
    pub predecessor_step_id: Option<Id>,
    pub interval_cell_step_input_id: Id,
    pub interval_cell_step_event_id: Id,
    pub conditional_interval_bulk_query_id: Id,
    pub conditional_interval_bulk_transfer_id: Id,
    pub interval_interface_input_id: Option<Id>,
    pub interval_interface_event_id: Option<Id>,
    pub disposition: OpticalLineageDispositionV1,
    pub step_id: Id,
}

#[allow(clippy::too_many_arguments)]
pub fn derive_optical_lineage_step_id(
    lane_id: Id,
    ordinal: u8,
    predecessor_step_id: Option<Id>,
    interval_cell_step_input_id: Id,
    interval_cell_step_event_id: Id,
    conditional_interval_bulk_query_id: Id,
    conditional_interval_bulk_transfer_id: Id,
    interface_ids: Option<(Id, Id)>,
    disposition: OpticalLineageDispositionV1,
) -> Result<Id, OpticalLineageError> {
    if lane_id == [0; 32]
        || interval_cell_step_input_id == [0; 32]
        || interval_cell_step_event_id == [0; 32]
        || conditional_interval_bulk_query_id == [0; 32]
        || conditional_interval_bulk_transfer_id == [0; 32]
    {
        return Err(OpticalLineageError::Invalid("lineage identity provenance"));
    }
    Ok(domain_hash(
        STEP_DOMAIN,
        &encode(&(
            lane_id,
            ordinal,
            predecessor_step_id,
            interval_cell_step_input_id,
            interval_cell_step_event_id,
            conditional_interval_bulk_query_id,
            conditional_interval_bulk_transfer_id,
            interface_ids,
            disposition,
        ))?,
    ))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalLaneManifestV1 {
    pub schema_version: u16,
    pub reconstruction_id: Id,
    pub visible_radiance_bulk_profile_id: Id,
    pub band: VisibleRadianceBandV1,
    pub lane_source_id: Id,
    pub lane_id: Id,
    pub steps: Vec<OpticalLaneStepV1>,
    pub bundle_receipt: OpticalLineageBundleReceiptV1,
    pub final_terminal: OpticalLineageTerminalV1,
    pub transcript_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl OpticalLineageBundleInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, OpticalLineageError> {
        compile_optical_lane_manifest(self)?;
        encode_capped(
            self,
            MAX_LINEAGE_BUNDLE_BYTES,
            "lineage bundle byte ceiling",
        )
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OpticalLineageError> {
        if bytes.len() > MAX_LINEAGE_BUNDLE_BYTES {
            return Err(OpticalLineageError::Invalid("lineage bundle byte ceiling"));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(OpticalLineageError::Invalid(
                "noncanonical lineage bundle bytes",
            ));
        }
        Ok(value)
    }
}

impl OpticalLaneManifestV1 {
    pub fn to_bytes(
        &self,
        bundle: &OpticalLineageBundleInputV1,
    ) -> Result<Vec<u8>, OpticalLineageError> {
        validate_optical_lane_manifest(bundle, self)?;
        encode_capped(
            self,
            MAX_LINEAGE_MANIFEST_BYTES,
            "lineage manifest byte ceiling",
        )
    }

    pub fn from_bytes(
        bytes: &[u8],
        bundle: &OpticalLineageBundleInputV1,
    ) -> Result<Self, OpticalLineageError> {
        if bytes.len() > MAX_LINEAGE_MANIFEST_BYTES {
            return Err(OpticalLineageError::Invalid(
                "lineage manifest byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(bundle)? != bytes {
            return Err(OpticalLineageError::Invalid(
                "noncanonical lineage manifest bytes",
            ));
        }
        Ok(value)
    }
}

pub fn compile_optical_lane_manifest(
    bundle: &OpticalLineageBundleInputV1,
) -> Result<OpticalLaneManifestV1, OpticalLineageError> {
    let raw_bundle = encode_capped(
        bundle,
        MAX_LINEAGE_BUNDLE_BYTES,
        "lineage bundle byte ceiling",
    )?;
    if bundle.schema_version != CONTRACT_VERSION || bundle.lane_source_id == [0; 32] {
        return Err(OpticalLineageError::Invalid("lineage bundle provenance"));
    }
    if bundle.steps.is_empty() || bundle.steps.len() > MAX_LINEAGE_STEPS {
        return Err(OpticalLineageError::Invalid("lineage step ceiling"));
    }
    validate_visible_radiance_bulk_profile(&bundle.profile)
        .map_err(|_| OpticalLineageError::Invalid("lineage profile replay"))?;
    let recipe = compile_physical_volume_recipe(&bundle.profile.input.physical_volume_recipe_input)
        .map_err(|_| OpticalLineageError::Invalid("lineage recipe replay"))?;
    let volume = compile_physical_volume(&recipe)
        .map_err(|_| OpticalLineageError::Invalid("lineage volume replay"))?;
    let initial = &bundle.steps[0].bulk_query.interval_cell_step_input;
    if initial.state_source_id != bundle.lane_source_id || initial.state_revision != 1 {
        return Err(OpticalLineageError::Invalid(
            "lineage initial source or revision",
        ));
    }
    let lane_id = derive_optical_lane_id(
        bundle.profile.input.reconstruction_id,
        bundle.profile.visible_radiance_bulk_profile_id,
        bundle.band,
        bundle.steps[0].bulk_transfer.interval_cell_step_input_id,
        bundle.lane_source_id,
    )?;
    let bundle_receipt = compile_bundle_receipt(bundle, &recipe, &volume)?;
    let mut steps = Vec::with_capacity(bundle.steps.len());
    let mut predecessor = None;
    let mut prior: Option<Prior> = None;
    let mut final_terminal = None;
    for (ordinal, evidence) in bundle.steps.iter().enumerate() {
        validate_conditional_interval_bulk_transfer(
            &bundle.profile,
            &evidence.bulk_query,
            &evidence.bulk_transfer,
        )
        .map_err(|_| OpticalLineageError::Invalid("lineage bulk replay"))?;
        if evidence.bulk_query.band != bundle.band || evidence.bulk_transfer.band != bundle.band {
            return Err(OpticalLineageError::Invalid("lineage band drift"));
        }
        let input = &evidence.bulk_query.interval_cell_step_input;
        if ordinal > 0 {
            let expected = derived_source(lane_id, ordinal as u8, predecessor, b"cell_input")?;
            if input.state_source_id != expected || input.state_revision != ordinal as u32 + 1 {
                return Err(OpticalLineageError::Invalid("lineage derived cell source"));
            }
            let prior = prior
                .as_ref()
                .ok_or(OpticalLineageError::Invalid("lineage predecessor missing"))?;
            if input.current_cell != prior.neighbor
                || input.point_q160 != prior.hit_point
                || input.direction_q1_62 != prior.next_direction
            {
                return Err(OpticalLineageError::Invalid(
                    "lineage successor propagation",
                ));
            }
        }
        let mut disposition = classify_step(
            bundle,
            evidence,
            ordinal,
            lane_id,
            predecessor,
            &recipe,
            &volume,
        )?;
        let continuation = continuation(evidence, &disposition)?;
        if ordinal + 1 < bundle.steps.len() && continuation.is_none() {
            return Err(OpticalLineageError::Invalid(
                "lineage evidence after terminal",
            ));
        }
        if ordinal + 1 == bundle.steps.len() {
            match disposition {
                OpticalLineageDispositionV1::Terminal { terminal } => {
                    final_terminal = Some(terminal)
                }
                _ if bundle.steps.len() == MAX_LINEAGE_STEPS => {
                    disposition = OpticalLineageDispositionV1::Terminal {
                        terminal: OpticalLineageTerminalV1::WorkExhaustion,
                    };
                    final_terminal = Some(OpticalLineageTerminalV1::WorkExhaustion);
                }
                _ => {
                    return Err(OpticalLineageError::Invalid(
                        "lineage truncated continuation",
                    ));
                }
            }
        }
        let interface_ids = evidence
            .interface_event
            .as_ref()
            .map(|value| (value.interval_interface_input_id, value.event_id));
        let step_id = derive_optical_lineage_step_id(
            lane_id,
            ordinal as u8,
            predecessor,
            evidence.bulk_transfer.interval_cell_step_input_id,
            evidence.bulk_transfer.interval_cell_step_event_id,
            evidence.bulk_transfer.conditional_interval_bulk_query_id,
            evidence.bulk_transfer.conditional_interval_bulk_transfer_id,
            interface_ids,
            disposition,
        )?;
        steps.push(OpticalLaneStepV1 {
            lane_id,
            ordinal: ordinal as u8,
            predecessor_step_id: predecessor,
            interval_cell_step_input_id: evidence.bulk_transfer.interval_cell_step_input_id,
            interval_cell_step_event_id: evidence.bulk_transfer.interval_cell_step_event_id,
            conditional_interval_bulk_query_id: evidence
                .bulk_transfer
                .conditional_interval_bulk_query_id,
            conditional_interval_bulk_transfer_id: evidence
                .bulk_transfer
                .conditional_interval_bulk_transfer_id,
            interval_interface_input_id: interface_ids.map(|value| value.0),
            interval_interface_event_id: interface_ids.map(|value| value.1),
            disposition,
            step_id,
        });
        predecessor = Some(step_id);
        prior = continuation.map(|value| Prior {
            neighbor: value.neighbor,
            hit_point: value.hit_point,
            next_direction: value.next_direction,
        });
    }
    let final_terminal =
        final_terminal.ok_or(OpticalLineageError::Invalid("lineage final terminal"))?;
    let limitations = lineage_limitations();
    let authority_effect = "none_evidence_only".to_owned();
    let transcript_id = domain_hash(
        TRANSCRIPT_DOMAIN,
        &encode(&(
            lane_id,
            &steps,
            &bundle_receipt,
            final_terminal,
            &limitations,
            &authority_effect,
        ))?,
    );
    let manifest = OpticalLaneManifestV1 {
        schema_version: CONTRACT_VERSION,
        reconstruction_id: bundle.profile.input.reconstruction_id,
        visible_radiance_bulk_profile_id: bundle.profile.visible_radiance_bulk_profile_id,
        band: bundle.band,
        lane_source_id: bundle.lane_source_id,
        lane_id,
        steps,
        bundle_receipt,
        final_terminal,
        transcript_id,
        limitations,
        authority_effect,
    };
    let bytes = encode_capped(
        &manifest,
        MAX_LINEAGE_MANIFEST_BYTES,
        "lineage manifest byte ceiling",
    )?;
    let _ = raw_bundle;
    let _ = bytes;
    Ok(manifest)
}

pub fn validate_optical_lane_manifest(
    bundle: &OpticalLineageBundleInputV1,
    manifest: &OpticalLaneManifestV1,
) -> Result<(), OpticalLineageError> {
    if &compile_optical_lane_manifest(bundle)? != manifest {
        return Err(OpticalLineageError::Invalid("lineage manifest drift"));
    }
    Ok(())
}

struct Continuation {
    neighbor: physical_path_substrate::CellIndex3V1,
    hit_point: [SignedDecimalIntervalV1; 3],
    next_direction: [SignedDecimalIntervalV1; 3],
}

struct Prior {
    neighbor: physical_path_substrate::CellIndex3V1,
    hit_point: [SignedDecimalIntervalV1; 3],
    next_direction: [SignedDecimalIntervalV1; 3],
}

fn continuation(
    evidence: &OpticalLineageStepEvidenceV1,
    disposition: &OpticalLineageDispositionV1,
) -> Result<Option<Continuation>, OpticalLineageError> {
    let certified = match &evidence.bulk_query.interval_cell_step_event.outcome {
        ConditionalIntervalCellStepOutcomeV1::CertifiedNextFace { certified, .. } => certified,
        _ => return Ok(None),
    };
    let neighbor = certified
        .neighbor
        .ok_or(OpticalLineageError::Invalid("lineage certified neighbor"))?;
    let direction = match disposition {
        OpticalLineageDispositionV1::ContinueSameMedium => evidence
            .bulk_query
            .interval_cell_step_input
            .direction_q1_62
            .clone(),
        OpticalLineageDispositionV1::ContinueAfterInterface => {
            selected_transmitted_direction(evidence)?
        }
        OpticalLineageDispositionV1::Terminal { .. } => return Ok(None),
    };
    Ok(Some(Continuation {
        neighbor,
        hit_point: certified.point_q160.clone(),
        next_direction: direction,
    }))
}

fn classify_step(
    bundle: &OpticalLineageBundleInputV1,
    evidence: &OpticalLineageStepEvidenceV1,
    ordinal: usize,
    lane_id: Id,
    predecessor: Option<Id>,
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
) -> Result<OpticalLineageDispositionV1, OpticalLineageError> {
    match &evidence.bulk_transfer.outcome {
        ConditionalIntervalBulkOutcomeV1::UnavailableCurrentCell => {
            require_no_interface(evidence)?;
            return Ok(terminal(OpticalLineageTerminalV1::UnavailableCurrent));
        }
        ConditionalIntervalBulkOutcomeV1::UpstreamAmbiguousNextFace => {
            require_no_interface(evidence)?;
            return Ok(terminal(OpticalLineageTerminalV1::AmbiguousNextFace));
        }
        ConditionalIntervalBulkOutcomeV1::UpstreamNoForwardProgress => {
            require_no_interface(evidence)?;
            return Ok(terminal(OpticalLineageTerminalV1::NoForwardProgress));
        }
        ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
            terminal: bulk_terminal,
            ..
        } => match bulk_terminal {
            IntervalBulkTerminalV1::OuterDomainExit => {
                require_no_interface(evidence)?;
                return Ok(terminal(OpticalLineageTerminalV1::OuterDomainExit));
            }
            IntervalBulkTerminalV1::UnavailableNeighbor { .. } => {
                require_no_interface(evidence)?;
                return Ok(terminal(OpticalLineageTerminalV1::UnavailableNeighbor));
            }
            IntervalBulkTerminalV1::KnownNeighbor { neighbor } => {
                let input = &evidence.bulk_query.interval_cell_step_input;
                let current = build_physical_cell(recipe, volume, input.current_cell)
                    .map_err(|_| OpticalLineageError::Invalid("lineage current cell replay"))?;
                let next = build_physical_cell(recipe, volume, *neighbor)
                    .map_err(|_| OpticalLineageError::Invalid("lineage neighbor replay"))?;
                if current.evidence == next.evidence {
                    require_no_interface(evidence)?;
                    return Ok(OpticalLineageDispositionV1::ContinueSameMedium);
                }
            }
        },
    }
    let interface_input = evidence
        .interface_input
        .as_ref()
        .ok_or(OpticalLineageError::Invalid(
            "lineage missing interface input",
        ))?;
    let interface_event = evidence
        .interface_event
        .as_ref()
        .ok_or(OpticalLineageError::Invalid(
            "lineage missing interface event",
        ))?;
    validate_visible_radiance_interval_interface_event(
        recipe,
        volume,
        interface_input,
        interface_event,
    )
    .map_err(|_| OpticalLineageError::Invalid("lineage interface replay"))?;
    let cell_input = &evidence.bulk_query.interval_cell_step_input;
    let expected_source = derived_source(lane_id, ordinal as u8, predecessor, b"interface_input")?;
    if interface_input.incident_source_id != expected_source
        || interface_input.incident_revision != ordinal as u32 + 1
        || interface_input.source_cell != cell_input.current_cell
        || !directions_equal(
            &interface_input.incident_direction_xyz,
            &cell_input.direction_q1_62,
        )
    {
        return Err(OpticalLineageError::Invalid("lineage interface adjacency"));
    }
    let neighbor = match evidence.bulk_transfer.outcome {
        ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
            terminal: IntervalBulkTerminalV1::KnownNeighbor { neighbor },
            ..
        } => neighbor,
        _ => {
            return Err(OpticalLineageError::Invalid(
                "lineage interface without known neighbor",
            ));
        }
    };
    if interface_input.target_cell != neighbor {
        return Err(OpticalLineageError::Invalid("lineage interface target"));
    }
    match &interface_event.outcome {
        IntervalInterfaceOutcomeV1::UnsupportedInterfaceModel => Ok(terminal(
            OpticalLineageTerminalV1::UnsupportedInterfaceModel,
        )),
        IntervalInterfaceOutcomeV1::Evaluated { bands_rgb, .. } => {
            match &bands_rgb[band_index(bundle.band)] {
                IntervalBandOutcomeV1::AmbiguousInterfaceBranch => {
                    Ok(terminal(OpticalLineageTerminalV1::AmbiguousInterfaceBranch))
                }
                IntervalBandOutcomeV1::NonconvergentEnclosure { .. } => {
                    Ok(terminal(OpticalLineageTerminalV1::NonconvergentInterface))
                }
                IntervalBandOutcomeV1::BoundedEnclosure {
                    branch: IntervalUniformBranchV1::AllTir,
                    ..
                } => Ok(terminal(OpticalLineageTerminalV1::AllTir)),
                IntervalBandOutcomeV1::BoundedEnclosure {
                    branch: IntervalUniformBranchV1::AllTransmit,
                    event,
                } => {
                    if event.transmitted_direction_xyz.is_none() {
                        return Err(OpticalLineageError::Invalid(
                            "lineage transmitted direction missing",
                        ));
                    }
                    Ok(OpticalLineageDispositionV1::ContinueAfterInterface)
                }
            }
        }
    }
}

fn selected_transmitted_direction(
    evidence: &OpticalLineageStepEvidenceV1,
) -> Result<[SignedDecimalIntervalV1; 3], OpticalLineageError> {
    let event = evidence
        .interface_event
        .as_ref()
        .ok_or(OpticalLineageError::Invalid(
            "lineage interface event missing",
        ))?;
    let band = evidence.bulk_query.band;
    let band_event = match &event.outcome {
        IntervalInterfaceOutcomeV1::Evaluated { bands_rgb, .. } => {
            match &bands_rgb[band_index(band)] {
                IntervalBandOutcomeV1::BoundedEnclosure {
                    branch: IntervalUniformBranchV1::AllTransmit,
                    event,
                } => event,
                _ => {
                    return Err(OpticalLineageError::Invalid(
                        "lineage selected band does not transmit",
                    ));
                }
            }
        }
        _ => {
            return Err(OpticalLineageError::Invalid(
                "lineage interface is not evaluated",
            ));
        }
    };
    let values =
        band_event
            .transmitted_direction_xyz
            .as_ref()
            .ok_or(OpticalLineageError::Invalid(
                "lineage transmitted direction missing",
            ))?;
    Ok(values.each_ref().map(decimal_to_physical))
}

fn decimal_to_physical(value: &DecimalIntervalV1) -> SignedDecimalIntervalV1 {
    SignedDecimalIntervalV1 {
        fractional_bits: 62,
        lower: value.lower.clone(),
        upper: value.upper.clone(),
    }
}

fn directions_equal(
    interface: &[DecimalIntervalV1; 3],
    physical: &[SignedDecimalIntervalV1; 3],
) -> bool {
    interface.iter().zip(physical).all(|(left, right)| {
        left.scale == FixedScaleV1::Q1_62
            && right.fractional_bits == 62
            && left.lower == right.lower
            && left.upper == right.upper
    })
}

fn terminal(value: OpticalLineageTerminalV1) -> OpticalLineageDispositionV1 {
    OpticalLineageDispositionV1::Terminal { terminal: value }
}

fn require_no_interface(
    evidence: &OpticalLineageStepEvidenceV1,
) -> Result<(), OpticalLineageError> {
    if evidence.interface_input.is_some() || evidence.interface_event.is_some() {
        Err(OpticalLineageError::Invalid("lineage unexpected interface"))
    } else {
        Ok(())
    }
}

fn compile_bundle_receipt(
    bundle: &OpticalLineageBundleInputV1,
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
) -> Result<OpticalLineageBundleReceiptV1, OpticalLineageError> {
    let mut entries = Vec::new();
    for evidence in &bundle.steps {
        let input_bytes = evidence
            .bulk_query
            .interval_cell_step_input
            .to_bytes(recipe, volume)
            .map_err(|_| OpticalLineageError::Invalid("lineage cell input replay"))?;
        let event_bytes = evidence
            .bulk_query
            .interval_cell_step_event
            .to_bytes(
                recipe,
                volume,
                &evidence.bulk_query.interval_cell_step_input,
            )
            .map_err(|_| OpticalLineageError::Invalid("lineage cell event replay"))?;
        let query_bytes = evidence
            .bulk_query
            .to_bytes(&bundle.profile)
            .map_err(|_| OpticalLineageError::Invalid("lineage bulk query replay"))?;
        let transfer_bytes = evidence
            .bulk_transfer
            .to_bytes(&bundle.profile, &evidence.bulk_query)
            .map_err(|_| OpticalLineageError::Invalid("lineage bulk transfer replay"))?;
        entries.push(object_receipt(
            evidence.bulk_transfer.interval_cell_step_input_id,
            &input_bytes,
        )?);
        entries.push(object_receipt(
            evidence.bulk_transfer.interval_cell_step_event_id,
            &event_bytes,
        )?);
        entries.push(object_receipt(
            evidence.bulk_transfer.conditional_interval_bulk_query_id,
            &query_bytes,
        )?);
        entries.push(object_receipt(
            evidence.bulk_transfer.conditional_interval_bulk_transfer_id,
            &transfer_bytes,
        )?);
        match (&evidence.interface_input, &evidence.interface_event) {
            (Some(input), Some(event)) => {
                let input_bytes = input
                    .to_bytes(recipe, volume)
                    .map_err(|_| OpticalLineageError::Invalid("lineage interface input replay"))?;
                let event_bytes = event
                    .to_bytes(recipe, volume, input)
                    .map_err(|_| OpticalLineageError::Invalid("lineage interface event replay"))?;
                entries.push(object_receipt(
                    event.interval_interface_input_id,
                    &input_bytes,
                )?);
                entries.push(object_receipt(event.event_id, &event_bytes)?);
            }
            (None, None) => {}
            _ => {
                return Err(OpticalLineageError::Invalid(
                    "lineage partial interface pair",
                ));
            }
        }
    }
    if entries.len() > MAX_LINEAGE_OBJECTS {
        return Err(OpticalLineageError::Invalid("lineage object ceiling"));
    }
    entries.sort();
    let ids: BTreeSet<_> = entries.iter().map(|entry| entry.object_id).collect();
    if ids.len() != entries.len() {
        return Err(OpticalLineageError::Invalid("lineage duplicate object"));
    }
    let canonical_bytes = entries
        .iter()
        .try_fold(0_u32, |sum, entry| sum.checked_add(entry.canonical_bytes))
        .ok_or(OpticalLineageError::Invalid(
            "lineage receipt byte overflow",
        ))?;
    Ok(OpticalLineageBundleReceiptV1 {
        object_count: entries.len() as u16,
        canonical_bytes,
        entries_sha256: domain_hash(BUNDLE_DOMAIN, &encode(&entries)?),
    })
}

fn object_receipt(
    object_id: Id,
    bytes: &[u8],
) -> Result<OpticalLineageObjectReceiptV1, OpticalLineageError> {
    Ok(OpticalLineageObjectReceiptV1 {
        object_id,
        canonical_sha256: Sha256::digest(bytes).into(),
        canonical_bytes: u32::try_from(bytes.len())
            .map_err(|_| OpticalLineageError::Invalid("lineage object byte overflow"))?,
    })
}

fn derived_source(
    lane_id: Id,
    ordinal: u8,
    predecessor: Option<Id>,
    role: &[u8],
) -> Result<Id, OpticalLineageError> {
    Ok(domain_hash(
        DERIVED_SOURCE_DOMAIN,
        &encode(&(lane_id, ordinal, predecessor, role))?,
    ))
}

fn band_index(band: VisibleRadianceBandV1) -> usize {
    match band {
        VisibleRadianceBandV1::Red => 0,
        VisibleRadianceBandV1::Green => 1,
        VisibleRadianceBandV1::Blue => 2,
    }
}

fn lineage_limitations() -> Vec<String> {
    vec![
        "ordered local optical-opportunity lineage only; no cumulative power receiver arrival or visibility claim".into(),
        "no numerical kernel coefficient catalogue persistence runtime perception rendering passage biome planet terrain approval or promotion claim".into(),
    ]
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, OpticalLineageError> {
    serde_json::to_vec(value).map_err(|error| OpticalLineageError::Codec(error.to_string()))
}

fn encode_capped<T: Serialize>(
    value: &T,
    maximum: usize,
    message: &'static str,
) -> Result<Vec<u8>, OpticalLineageError> {
    let bytes = encode(value)?;
    if bytes.len() > maximum {
        return Err(OpticalLineageError::Invalid(message));
    }
    Ok(bytes)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, OpticalLineageError> {
    serde_json::from_slice(bytes).map_err(|error| OpticalLineageError::Codec(error.to_string()))
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update([0]);
    hash.update(bytes);
    hash.finalize().into()
}
