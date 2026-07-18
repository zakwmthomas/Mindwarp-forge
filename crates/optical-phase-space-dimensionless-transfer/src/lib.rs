#![deny(warnings)]
//! Capability-free whole-cell dimensionless-transfer evidence.

use optical_phase_space_receiver_coupling::{
    WholeCellFullProofV1, WholeCellReceiverCouplingInputV1, WholeCellReceiverCouplingOutcomeV1,
    WholeCellReceiverCouplingV1, validate_whole_cell_receiver_coupling,
};
use optical_phase_space_transport_certificate::validate_origin_anchored_transport_certificate;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::fmt;
use visible_radiance_bulk_transfer::{
    BandTransferV1, BulkOpticalDepthEvaluationInputV1, BulkOpticalDepthEvaluationV1,
    ConditionalIntervalBulkOutcomeV1, ConditionalIntervalBulkQueryV1,
    ConditionalIntervalBulkTransferV1, FixedU128V1, VisibleRadianceBandV1,
    VisibleRadianceBulkProfileV1, compile_bulk_optical_depth_evaluation,
    compile_conditional_interval_bulk_transfer, validate_visible_radiance_bulk_profile,
};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAXIMUM_STEPS: usize = 64;
pub const MAXIMUM_ENDPOINT_ADDITIONS: u16 = 128;
pub const MAXIMUM_RAW_OPTICAL_DEPTH_BITS: u16 = 118;
pub const MAX_BAND_TIME_BINDING_BYTES: usize = 4 * 1024;
pub const MAX_INPUT_BYTES: usize = 128 * 1024 * 1024;
pub const MAX_RESULT_BYTES: usize = 256 * 1024;
pub const MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 192 * 1024 * 1024;
pub const AUTHORITY_EFFECT_NONE: &str = "none_evidence_only";
pub const LIMITATIONS_V1: &str = "whole_cell_dimensionless_transfer_evidence_only_no_source_magnitude_detector_visibility_perception_runtime_promotion_or_c3_closure";

const BAND_TIME_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.band-time.v1";
const INPUT_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.dimensionless-transfer.input.v1";
const RESULT_DOMAIN: &[u8] = b"mindwarp.optical-phase-space.dimensionless-transfer.result.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WholeCellDimensionlessTransferError {
    InvalidSchema,
    InvalidInput(&'static str),
    Dependency(&'static str),
    ByteCeiling,
    ResourceCeiling,
    ArithmeticShieldExceeded,
    IdentityMismatch,
    CodecDefect,
}

impl fmt::Display for WholeCellDimensionlessTransferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for WholeCellDimensionlessTransferError {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpticalBandTimeBindingV1 {
    pub band: VisibleRadianceBandV1,
    pub time_basis_id: [u8; 32],
    pub band_time_id: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactMeasureV1 {
    pub numerator: String,
    pub denominator: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptedUnresolvedTransferReasonV1 {
    SelectedPartialOpaque,
    StartInsidePriorOpaque,
    UncertainBulkEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WholeCellDimensionlessTransferOutcomeV1 {
    CertifiedAcceptedFiniteTransfer {
        bulk_evaluation: BulkOpticalDepthEvaluationV1,
    },
    CertifiedAcceptedOpaqueTransfer {
        mandatory_prefix_step_id: [u8; 32],
    },
    CertifiedAcceptedUnresolvedTransfer {
        reason: AcceptedUnresolvedTransferReasonV1,
        transfer_upper_q0_48: u64,
        bulk_evaluation: BulkOpticalDepthEvaluationV1,
    },
    CertifiedZeroCoupling,
    UnresolvedCoupling,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellDimensionlessTransferArithmeticReceiptV1 {
    pub maximum_steps: u8,
    pub conditional_bulk_calls: u8,
    pub maximum_endpoint_additions: u16,
    pub endpoint_additions: u16,
    pub maximum_raw_optical_depth_bits: u16,
    pub observed_raw_optical_depth_bits: u16,
    pub bulk_evaluation_calls: u8,
    pub bulk_exponential_kernel_calls: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellDimensionlessTransferInputV1 {
    pub schema_version: u16,
    pub bulk_profile: VisibleRadianceBulkProfileV1,
    pub receiver_coupling_input: WholeCellReceiverCouplingInputV1,
    pub receiver_coupling: WholeCellReceiverCouplingV1,
    pub band_time_binding: OpticalBandTimeBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WholeCellDimensionlessTransferV1 {
    pub schema_version: u16,
    pub input_id: [u8; 32],
    pub cell_id: [u8; 32],
    pub transport_certificate_id: [u8; 32],
    pub receiver_coupling_result_id: [u8; 32],
    pub visible_radiance_bulk_profile_id: [u8; 32],
    pub band_time_id: [u8; 32],
    pub selected_step_id: [u8; 32],
    pub accepted_measure: ExactMeasureV1,
    pub zero_measure: ExactMeasureV1,
    pub unresolved_measure: ExactMeasureV1,
    pub conditional_bulk_transfers: Vec<ConditionalIntervalBulkTransferV1>,
    pub outcome: WholeCellDimensionlessTransferOutcomeV1,
    pub arithmetic_receipt: WholeCellDimensionlessTransferArithmeticReceiptV1,
    pub result_id: [u8; 32],
    pub limitations: String,
    pub authority_effect: String,
}

#[derive(Default)]
struct Composition {
    lower: u128,
    upper: u128,
    additions: u16,
    observed_bits: u16,
    transfers: Vec<ConditionalIntervalBulkTransferV1>,
}

impl Composition {
    fn add_lower(&mut self, value: u128) -> Result<(), WholeCellDimensionlessTransferError> {
        self.lower = self
            .lower
            .checked_add(value)
            .ok_or(WholeCellDimensionlessTransferError::ArithmeticShieldExceeded)?;
        self.additions = self
            .additions
            .checked_add(1)
            .ok_or(WholeCellDimensionlessTransferError::ResourceCeiling)?;
        self.check()
    }

    fn add_upper(&mut self, value: u128) -> Result<(), WholeCellDimensionlessTransferError> {
        self.upper = self
            .upper
            .checked_add(value)
            .ok_or(WholeCellDimensionlessTransferError::ArithmeticShieldExceeded)?;
        self.additions = self
            .additions
            .checked_add(1)
            .ok_or(WholeCellDimensionlessTransferError::ResourceCeiling)?;
        self.check()
    }

    fn check(&mut self) -> Result<(), WholeCellDimensionlessTransferError> {
        if self.additions > MAXIMUM_ENDPOINT_ADDITIONS {
            return Err(WholeCellDimensionlessTransferError::ResourceCeiling);
        }
        self.observed_bits = self
            .observed_bits
            .max(raw_bits(self.lower))
            .max(raw_bits(self.upper));
        if self.observed_bits > MAXIMUM_RAW_OPTICAL_DEPTH_BITS {
            return Err(WholeCellDimensionlessTransferError::ArithmeticShieldExceeded);
        }
        Ok(())
    }
}

pub fn compile_optical_band_time_binding(
    band: VisibleRadianceBandV1,
    time_basis_id: [u8; 32],
) -> Result<OpticalBandTimeBindingV1, WholeCellDimensionlessTransferError> {
    if time_basis_id == [0; 32] {
        return Err(WholeCellDimensionlessTransferError::InvalidInput(
            "zero time-basis identity",
        ));
    }
    let identity_bytes = json(&(band, time_basis_id))?;
    Ok(OpticalBandTimeBindingV1 {
        band,
        time_basis_id,
        band_time_id: domain_hash(BAND_TIME_DOMAIN, &identity_bytes),
    })
}

impl OpticalBandTimeBindingV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, WholeCellDimensionlessTransferError> {
        validate_band_time_binding(self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_BAND_TIME_BINDING_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WholeCellDimensionlessTransferError> {
        if bytes.len() > MAX_BAND_TIME_BINDING_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? == bytes {
            Ok(value)
        } else {
            Err(WholeCellDimensionlessTransferError::CodecDefect)
        }
    }
}

impl WholeCellDimensionlessTransferInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, WholeCellDimensionlessTransferError> {
        validate_input(self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WholeCellDimensionlessTransferError> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? == bytes {
            Ok(value)
        } else {
            Err(WholeCellDimensionlessTransferError::CodecDefect)
        }
    }
}

impl WholeCellDimensionlessTransferV1 {
    pub fn to_bytes(
        &self,
        input: &WholeCellDimensionlessTransferInputV1,
    ) -> Result<Vec<u8>, WholeCellDimensionlessTransferError> {
        validate_whole_cell_dimensionless_transfer(input, self)?;
        let bytes = json(self)?;
        if bytes.len() > MAX_RESULT_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        Ok(bytes)
    }

    pub fn from_bytes(
        input: &WholeCellDimensionlessTransferInputV1,
        bytes: &[u8],
    ) -> Result<Self, WholeCellDimensionlessTransferError> {
        if bytes.len() > MAX_RESULT_BYTES {
            return Err(WholeCellDimensionlessTransferError::ByteCeiling);
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(input)? == bytes {
            Ok(value)
        } else {
            Err(WholeCellDimensionlessTransferError::CodecDefect)
        }
    }
}

pub fn compile_whole_cell_dimensionless_transfer(
    input: &WholeCellDimensionlessTransferInputV1,
) -> Result<WholeCellDimensionlessTransferV1, WholeCellDimensionlessTransferError> {
    validate_input(input)?;
    let input_bytes = input.to_bytes()?;
    let input_id = domain_hash(INPUT_DOMAIN, &input_bytes);
    let coupling = &input.receiver_coupling;
    let selected_index = usize::from(input.receiver_coupling_input.selected_step_index);
    let selected_step = input
        .receiver_coupling_input
        .transport_certificate
        .steps
        .get(selected_index)
        .ok_or(WholeCellDimensionlessTransferError::InvalidInput(
            "selected transport step missing",
        ))?;
    let mut composition = Composition::default();

    let outcome = match coupling.outcome {
        WholeCellReceiverCouplingOutcomeV1::CertifiedZeroBeforeFace { .. } => {
            WholeCellDimensionlessTransferOutcomeV1::CertifiedZeroCoupling
        }
        WholeCellReceiverCouplingOutcomeV1::UnresolvedReceiverCoupling { .. } => {
            WholeCellDimensionlessTransferOutcomeV1::UnresolvedCoupling
        }
        WholeCellReceiverCouplingOutcomeV1::CertifiedFullBeforeFace { proof, .. } => {
            compile_accepted(input, proof, selected_index, &mut composition)?
        }
    };

    let bulk_evaluation_calls = u8::from(matches!(
        outcome,
        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedFiniteTransfer { .. }
            | WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedUnresolvedTransfer { .. }
    ));
    let arithmetic_receipt = WholeCellDimensionlessTransferArithmeticReceiptV1 {
        maximum_steps: MAXIMUM_STEPS as u8,
        conditional_bulk_calls: u8::try_from(composition.transfers.len())
            .map_err(|_| WholeCellDimensionlessTransferError::ResourceCeiling)?,
        maximum_endpoint_additions: MAXIMUM_ENDPOINT_ADDITIONS,
        endpoint_additions: composition.additions,
        maximum_raw_optical_depth_bits: MAXIMUM_RAW_OPTICAL_DEPTH_BITS,
        observed_raw_optical_depth_bits: composition.observed_bits,
        bulk_evaluation_calls,
        bulk_exponential_kernel_calls: bulk_evaluation_calls.saturating_mul(2),
    };
    let mut result = WholeCellDimensionlessTransferV1 {
        schema_version: CONTRACT_VERSION,
        input_id,
        cell_id: coupling.cell_id,
        transport_certificate_id: coupling.transport_certificate_id,
        receiver_coupling_result_id: coupling.result_id,
        visible_radiance_bulk_profile_id: input.bulk_profile.visible_radiance_bulk_profile_id,
        band_time_id: input.band_time_binding.band_time_id,
        selected_step_id: selected_step.step_id,
        accepted_measure: ExactMeasureV1 {
            numerator: coupling.accepted_measure.numerator.clone(),
            denominator: coupling.accepted_measure.denominator.clone(),
        },
        zero_measure: ExactMeasureV1 {
            numerator: coupling.zero_measure.numerator.clone(),
            denominator: coupling.zero_measure.denominator.clone(),
        },
        unresolved_measure: ExactMeasureV1 {
            numerator: coupling.unresolved_measure.numerator.clone(),
            denominator: coupling.unresolved_measure.denominator.clone(),
        },
        conditional_bulk_transfers: composition.transfers,
        outcome,
        arithmetic_receipt,
        result_id: [0; 32],
        limitations: LIMITATIONS_V1.into(),
        authority_effect: AUTHORITY_EFFECT_NONE.into(),
    };
    result.result_id = domain_hash(RESULT_DOMAIN, &json(&result)?);
    let result_bytes = json(&result)?;
    if result_bytes.len() > MAX_RESULT_BYTES {
        return Err(WholeCellDimensionlessTransferError::ByteCeiling);
    }
    if input_bytes.len().saturating_add(result_bytes.len()) > MAX_AGGREGATE_LIVE_CANONICAL_BYTES {
        return Err(WholeCellDimensionlessTransferError::ResourceCeiling);
    }
    Ok(result)
}

pub fn validate_whole_cell_dimensionless_transfer(
    input: &WholeCellDimensionlessTransferInputV1,
    result: &WholeCellDimensionlessTransferV1,
) -> Result<(), WholeCellDimensionlessTransferError> {
    if &compile_whole_cell_dimensionless_transfer(input)? == result {
        Ok(())
    } else {
        Err(WholeCellDimensionlessTransferError::IdentityMismatch)
    }
}

fn compile_accepted(
    input: &WholeCellDimensionlessTransferInputV1,
    proof: WholeCellFullProofV1,
    selected_index: usize,
    composition: &mut Composition,
) -> Result<WholeCellDimensionlessTransferOutcomeV1, WholeCellDimensionlessTransferError> {
    let step_count = match proof {
        WholeCellFullProofV1::ReceiverFace => selected_index.saturating_add(1),
        WholeCellFullProofV1::StartInside => selected_index,
    };
    let mut unresolved_reason = None;
    for index in 0..step_count {
        let step = input
            .receiver_coupling_input
            .transport_certificate
            .steps
            .get(index)
            .ok_or(WholeCellDimensionlessTransferError::InvalidInput(
                "transport prefix step missing",
            ))?;
        let query = ConditionalIntervalBulkQueryV1 {
            schema_version: CONTRACT_VERSION,
            visible_radiance_bulk_profile_id: input.bulk_profile.visible_radiance_bulk_profile_id,
            band: input.band_time_binding.band,
            interval_cell_step_input: step.physical_input.clone(),
            interval_cell_step_event: step.physical_event.clone(),
        };
        let transfer = compile_conditional_interval_bulk_transfer(&input.bulk_profile, &query)
            .map_err(|_| {
                WholeCellDimensionlessTransferError::Dependency(
                    "conditional bulk transfer replay failed",
                )
            })?;
        let selected_partial =
            proof == WholeCellFullProofV1::ReceiverFace && index == selected_index;
        match &transfer.outcome {
            ConditionalIntervalBulkOutcomeV1::KnownCurrentCellTransfer {
                band_transfer, ..
            } => match band_transfer {
                BandTransferV1::VacuumIdentity => {}
                BandTransferV1::Finite {
                    optical_depth_lower_q64_64,
                    optical_depth_upper_q64_64,
                    ..
                } => {
                    if proof == WholeCellFullProofV1::ReceiverFace && !selected_partial {
                        composition.add_lower(optical_depth_lower_q64_64.to_u128())?;
                    }
                    composition.add_upper(optical_depth_upper_q64_64.to_u128())?;
                }
                BandTransferV1::Opaque if selected_partial => {
                    unresolved_reason =
                        Some(AcceptedUnresolvedTransferReasonV1::SelectedPartialOpaque);
                }
                BandTransferV1::Opaque if proof == WholeCellFullProofV1::StartInside => {
                    composition.lower = 0;
                    composition.upper = 0;
                    unresolved_reason =
                        Some(AcceptedUnresolvedTransferReasonV1::StartInsidePriorOpaque);
                }
                BandTransferV1::Opaque => {
                    let mandatory_prefix_step_id = step.step_id;
                    composition.transfers.push(transfer);
                    return Ok(
                        WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedOpaqueTransfer {
                            mandatory_prefix_step_id,
                        },
                    );
                }
            },
            _ => {
                composition.lower = 0;
                unresolved_reason = Some(AcceptedUnresolvedTransferReasonV1::UncertainBulkEvidence);
            }
        }
        composition.transfers.push(transfer);
    }

    let evaluation_input = BulkOpticalDepthEvaluationInputV1 {
        schema_version: CONTRACT_VERSION,
        optical_depth_lower_q64_64: FixedU128V1::from_u128(composition.lower),
        optical_depth_upper_q64_64: FixedU128V1::from_u128(composition.upper),
    };
    let bulk_evaluation =
        compile_bulk_optical_depth_evaluation(&evaluation_input).map_err(|_| {
            WholeCellDimensionlessTransferError::Dependency("bulk optical-depth evaluation failed")
        })?;
    match unresolved_reason {
        Some(reason) => Ok(
            WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedUnresolvedTransfer {
                reason,
                transfer_upper_q0_48: bulk_evaluation.transfer_upper_q0_48,
                bulk_evaluation,
            },
        ),
        None => Ok(
            WholeCellDimensionlessTransferOutcomeV1::CertifiedAcceptedFiniteTransfer {
                bulk_evaluation,
            },
        ),
    }
}

fn validate_input(
    input: &WholeCellDimensionlessTransferInputV1,
) -> Result<(), WholeCellDimensionlessTransferError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(WholeCellDimensionlessTransferError::InvalidSchema);
    }
    validate_visible_radiance_bulk_profile(&input.bulk_profile).map_err(|_| {
        WholeCellDimensionlessTransferError::Dependency("bulk profile replay failed")
    })?;
    validate_origin_anchored_transport_certificate(
        &input.receiver_coupling_input.transport_input,
        &input.receiver_coupling_input.transport_certificate,
    )
    .map_err(|_| {
        WholeCellDimensionlessTransferError::Dependency("transport certificate replay failed")
    })?;
    validate_whole_cell_receiver_coupling(&input.receiver_coupling_input, &input.receiver_coupling)
        .map_err(|_| {
            WholeCellDimensionlessTransferError::Dependency("receiver coupling replay failed")
        })?;
    validate_band_time_binding(&input.band_time_binding)?;
    let transport = &input.receiver_coupling_input.transport_input;
    let certificate = &input.receiver_coupling_input.transport_certificate;
    if input.band_time_binding.band_time_id != transport.band_time_id
        || input.band_time_binding.band_time_id != certificate.band_time_id
    {
        return Err(WholeCellDimensionlessTransferError::IdentityMismatch);
    }
    if input.bulk_profile.physical_volume_id != transport.physical_volume.physical_volume_id {
        return Err(WholeCellDimensionlessTransferError::IdentityMismatch);
    }
    if certificate.steps.len() > MAXIMUM_STEPS {
        return Err(WholeCellDimensionlessTransferError::ResourceCeiling);
    }
    Ok(())
}

fn validate_band_time_binding(
    binding: &OpticalBandTimeBindingV1,
) -> Result<(), WholeCellDimensionlessTransferError> {
    if &compile_optical_band_time_binding(binding.band, binding.time_basis_id)? == binding {
        Ok(())
    } else {
        Err(WholeCellDimensionlessTransferError::IdentityMismatch)
    }
}

fn raw_bits(value: u128) -> u16 {
    (u128::BITS - value.leading_zeros()) as u16
}

fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, WholeCellDimensionlessTransferError> {
    serde_json::to_vec(value).map_err(|_| WholeCellDimensionlessTransferError::CodecDefect)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WholeCellDimensionlessTransferError> {
    serde_json::from_slice(bytes).map_err(|_| WholeCellDimensionlessTransferError::CodecDefect)
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update([0]);
    hash.update(bytes);
    hash.finalize().into()
}
