//! Capability-free visible-radiance bulk-transfer evidence reference.
//!
//! V1 reconstructs one exact physical volume and path witness, admits known
//! transfer only through vacuum or one exact substance, and returns directed
//! optical-depth/transmission bounds. It owns no interface optics, perception,
//! rendering, passage, biome, planet, runtime, approval, or promotion policy.

mod interval;
pub use interval::*;

use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, Id, PathIntersectionKindV1, PhysicalPathError,
    PhysicalPathQueryV1, PhysicalPathWitnessV1, PhysicalVolumeRecipeInputV1,
    PhysicalVolumeRecipeV1, PhysicalVolumeV1, compile_path_witness, compile_physical_volume,
    compile_physical_volume_recipe, validate_path_witness, validate_physical_volume,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_BULK_PROFILE_SUBSTANCES: usize = 65_536;
pub const TRANSMISSION_ONE_Q0_48: u64 = 1_u64 << 48;
pub const MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES: usize = 4 * 1024;
pub const BULK_OPTICAL_DEPTH_MAXIMUM_RAW_BITS: u16 = 118;
const Q0_64_ONE: u128 = 1_u128 << 64;
const Q0_64_HALF: u128 = 1_u128 << 63;
const MAX_EXP_TERMS: u32 = 192;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BulkBandInteractionV1 {
    Finite {
        extinction_q16_48_per_coordinate_unit: u64,
    },
    Opaque,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubstanceBulkInteractionV1 {
    pub substance_source_id: Id,
    pub bands_rgb: [BulkBandInteractionV1; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceBulkProfileInputV1 {
    pub schema_version: u16,
    pub profile_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub profile_revision: u32,
    pub physical_volume_recipe_input: PhysicalVolumeRecipeInputV1,
    pub substance_interactions: Vec<SubstanceBulkInteractionV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceBulkProfileV1 {
    pub schema_version: u16,
    pub visible_radiance_bulk_profile_id: Id,
    pub physical_volume_id: Id,
    pub input: VisibleRadianceBulkProfileInputV1,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceBulkQueryV1 {
    pub schema_version: u16,
    pub visible_radiance_bulk_profile_id: Id,
    pub path_query: PhysicalPathQueryV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FixedU128V1 {
    pub high_u64: u64,
    pub low_u64: u64,
}

impl FixedU128V1 {
    pub fn from_u128(value: u128) -> Self {
        Self {
            high_u64: (value >> 64) as u64,
            low_u64: value as u64,
        }
    }

    pub fn to_u128(self) -> u128 {
        (u128::from(self.high_u64) << 64) | u128::from(self.low_u64)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BulkOpticalDepthEvaluationInputV1 {
    pub schema_version: u16,
    pub optical_depth_lower_q64_64: FixedU128V1,
    pub optical_depth_upper_q64_64: FixedU128V1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BulkOpticalDepthEvaluationArithmeticReceiptV1 {
    pub optical_depth_fractional_bits: u16,
    pub transfer_fractional_bits: u16,
    pub maximum_raw_bits: u16,
    pub observed_maximum_raw_bits: u16,
    pub exponential_kernel_calls: u8,
    pub exponential_term_ceiling: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BulkOpticalDepthEvaluationV1 {
    pub schema_version: u16,
    pub bulk_optical_depth_evaluation_input_id: Id,
    pub optical_depth_lower_q64_64: FixedU128V1,
    pub optical_depth_upper_q64_64: FixedU128V1,
    pub transfer_lower_q0_48: u64,
    pub transfer_upper_q0_48: u64,
    pub arithmetic_receipt: BulkOpticalDepthEvaluationArithmeticReceiptV1,
    pub bulk_optical_depth_evaluation_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BandTransferV1 {
    Finite {
        optical_depth_lower_q64_64: FixedU128V1,
        optical_depth_upper_q64_64: FixedU128V1,
        transmission_lower_q0_48: u64,
        transmission_upper_q0_48: u64,
    },
    Opaque,
    VacuumIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BulkTransferOutcomeV1 {
    Known { bands_rgb: [BandTransferV1; 3] },
    UnavailableEvidence,
    AmbiguousBoundaryLane,
    InterfaceModelRequired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceBulkTransferV1 {
    pub schema_version: u16,
    pub visible_radiance_bulk_profile_id: Id,
    pub physical_volume_id: Id,
    pub path_query_id: Id,
    pub path_witness_id: Id,
    pub outcome: BulkTransferOutcomeV1,
    pub visible_radiance_bulk_transfer_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VisibleRadianceBulkError {
    Invalid(&'static str),
    Codec(String),
    Path(PhysicalPathError),
}

impl From<PhysicalPathError> for VisibleRadianceBulkError {
    fn from(value: PhysicalPathError) -> Self {
        Self::Path(value)
    }
}

impl VisibleRadianceBulkProfileInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_profile_input(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VisibleRadianceBulkError> {
        let value: Self = decode(bytes)?;
        validate_profile_input(&value)?;
        require_canonical(&value, bytes, "noncanonical bulk-profile input bytes")?;
        Ok(value)
    }
}

impl BulkOpticalDepthEvaluationInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_bulk_optical_depth_evaluation_input(self)?;
        let bytes = encode(self)?;
        if bytes.len() > MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "bulk optical-depth evaluation input byte ceiling",
            ));
        }
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VisibleRadianceBulkError> {
        if bytes.len() > MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "bulk optical-depth evaluation input byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes()? != bytes {
            return Err(VisibleRadianceBulkError::Invalid(
                "noncanonical bulk optical-depth evaluation input bytes",
            ));
        }
        Ok(value)
    }
}

impl BulkOpticalDepthEvaluationV1 {
    pub fn to_bytes(
        &self,
        input: &BulkOpticalDepthEvaluationInputV1,
    ) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_bulk_optical_depth_evaluation(input, self)?;
        let bytes = encode(self)?;
        if bytes.len() > MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "bulk optical-depth evaluation result byte ceiling",
            ));
        }
        Ok(bytes)
    }

    pub fn from_bytes(
        bytes: &[u8],
        input: &BulkOpticalDepthEvaluationInputV1,
    ) -> Result<Self, VisibleRadianceBulkError> {
        if bytes.len() > MAX_BULK_OPTICAL_DEPTH_EVALUATION_BYTES {
            return Err(VisibleRadianceBulkError::Invalid(
                "bulk optical-depth evaluation result byte ceiling",
            ));
        }
        let value: Self = decode(bytes)?;
        if value.to_bytes(input)? != bytes {
            return Err(VisibleRadianceBulkError::Invalid(
                "noncanonical bulk optical-depth evaluation result bytes",
            ));
        }
        Ok(value)
    }
}

impl VisibleRadianceBulkProfileV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_visible_radiance_bulk_profile(self)?;
        encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VisibleRadianceBulkError> {
        let value: Self = decode(bytes)?;
        validate_visible_radiance_bulk_profile(&value)?;
        require_canonical(&value, bytes, "noncanonical bulk-profile bytes")?;
        Ok(value)
    }
}

impl VisibleRadianceBulkQueryV1 {
    pub fn to_bytes(
        &self,
        profile: &VisibleRadianceBulkProfileV1,
    ) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_bulk_query(profile, self)?;
        encode(self)
    }

    pub fn from_bytes(
        bytes: &[u8],
        profile: &VisibleRadianceBulkProfileV1,
    ) -> Result<Self, VisibleRadianceBulkError> {
        let value: Self = decode(bytes)?;
        validate_bulk_query(profile, &value)?;
        require_canonical(&value, bytes, "noncanonical bulk-transfer query bytes")?;
        Ok(value)
    }
}

impl VisibleRadianceBulkTransferV1 {
    pub fn to_bytes(
        &self,
        profile: &VisibleRadianceBulkProfileV1,
        query: &VisibleRadianceBulkQueryV1,
    ) -> Result<Vec<u8>, VisibleRadianceBulkError> {
        validate_visible_radiance_bulk_transfer(profile, query, self)?;
        encode(self)
    }

    pub fn from_bytes(
        bytes: &[u8],
        profile: &VisibleRadianceBulkProfileV1,
        query: &VisibleRadianceBulkQueryV1,
    ) -> Result<Self, VisibleRadianceBulkError> {
        let value: Self = decode(bytes)?;
        validate_visible_radiance_bulk_transfer(profile, query, &value)?;
        require_canonical(&value, bytes, "noncanonical bulk-transfer bytes")?;
        Ok(value)
    }
}

pub fn compile_bulk_optical_depth_evaluation(
    input: &BulkOpticalDepthEvaluationInputV1,
) -> Result<BulkOpticalDepthEvaluationV1, VisibleRadianceBulkError> {
    let (lower, upper, observed_bits) = validate_bulk_optical_depth_evaluation_input(input)?;
    let input_bytes = input.to_bytes()?;
    let input_id = hash(
        b"mindwarp.visible-radiance.bulk-optical-depth-evaluation.input.v1",
        &input_bytes,
    );

    let (transfer_lower_q64, _) = exp_neg_q0_64_bounds(upper)?;
    let (_, transfer_upper_q64) = exp_neg_q0_64_bounds(lower)?;
    let transfer_lower_q0_48 = u64::try_from(transfer_lower_q64 >> 16).map_err(|_| {
        VisibleRadianceBulkError::Invalid("bulk optical-depth lower transfer overflow")
    })?;
    let transfer_upper_q0_48 =
        u64::try_from(ceil_div_pow2(transfer_upper_q64, 16)?).map_err(|_| {
            VisibleRadianceBulkError::Invalid("bulk optical-depth upper transfer overflow")
        })?;
    if transfer_lower_q0_48 > transfer_upper_q0_48 || transfer_upper_q0_48 > TRANSMISSION_ONE_Q0_48
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk optical-depth transfer enclosure invalid",
        ));
    }

    let arithmetic_receipt = BulkOpticalDepthEvaluationArithmeticReceiptV1 {
        optical_depth_fractional_bits: 64,
        transfer_fractional_bits: 48,
        maximum_raw_bits: BULK_OPTICAL_DEPTH_MAXIMUM_RAW_BITS,
        observed_maximum_raw_bits: observed_bits,
        exponential_kernel_calls: 2,
        exponential_term_ceiling: MAX_EXP_TERMS as u16,
    };
    let limitations = bulk_optical_depth_evaluation_limitations();
    let authority_effect = "none_evidence_only".to_owned();
    let identity_bytes = encode(&(
        input_id,
        input.optical_depth_lower_q64_64,
        input.optical_depth_upper_q64_64,
        transfer_lower_q0_48,
        transfer_upper_q0_48,
        arithmetic_receipt,
        &limitations,
        &authority_effect,
    ))?;
    Ok(BulkOpticalDepthEvaluationV1 {
        schema_version: CONTRACT_VERSION,
        bulk_optical_depth_evaluation_input_id: input_id,
        optical_depth_lower_q64_64: input.optical_depth_lower_q64_64,
        optical_depth_upper_q64_64: input.optical_depth_upper_q64_64,
        transfer_lower_q0_48,
        transfer_upper_q0_48,
        arithmetic_receipt,
        bulk_optical_depth_evaluation_id: hash(
            b"mindwarp.visible-radiance.bulk-optical-depth-evaluation.result.v1",
            &identity_bytes,
        ),
        limitations,
        authority_effect,
    })
}

pub fn validate_bulk_optical_depth_evaluation(
    input: &BulkOpticalDepthEvaluationInputV1,
    result: &BulkOpticalDepthEvaluationV1,
) -> Result<(), VisibleRadianceBulkError> {
    if &compile_bulk_optical_depth_evaluation(input)? != result {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk optical-depth evaluation drift",
        ));
    }
    Ok(())
}

fn validate_bulk_optical_depth_evaluation_input(
    input: &BulkOpticalDepthEvaluationInputV1,
) -> Result<(u128, u128, u16), VisibleRadianceBulkError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk optical-depth evaluation schema mismatch",
        ));
    }
    let lower = input.optical_depth_lower_q64_64.to_u128();
    let upper = input.optical_depth_upper_q64_64.to_u128();
    if lower > upper {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk optical-depth evaluation endpoints unordered",
        ));
    }
    let observed_bits = u16::try_from(u128::BITS - upper.leading_zeros())
        .map_err(|_| VisibleRadianceBulkError::Invalid("bulk optical-depth raw-bit conversion"))?;
    if observed_bits > BULK_OPTICAL_DEPTH_MAXIMUM_RAW_BITS {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk optical-depth evaluation raw-bit ceiling",
        ));
    }
    Ok((lower, upper, observed_bits))
}

fn bulk_optical_depth_evaluation_limitations() -> Vec<String> {
    vec![
        "dimensionless optical-depth evaluation only; no path, medium, band or time derivation"
            .into(),
        "transfer enclosure is evidence only; no source magnitude, detector, visibility or perception"
            .into(),
        "no runtime, rendering, gameplay, approval, promotion or C3 closure authority".into(),
    ]
}

pub fn compile_visible_radiance_bulk_profile(
    input: &VisibleRadianceBulkProfileInputV1,
) -> Result<VisibleRadianceBulkProfileV1, VisibleRadianceBulkError> {
    validate_profile_input(input)?;
    let (recipe, volume) = rebuild_volume(input)?;
    let bytes = input.to_bytes()?;
    let id = hash(
        b"mindwarp.visible-radiance.bulk-profile.v1",
        &encode(&(volume.physical_volume_id, bytes))?,
    );
    let _ = recipe;
    Ok(VisibleRadianceBulkProfileV1 {
        schema_version: CONTRACT_VERSION,
        visible_radiance_bulk_profile_id: id,
        physical_volume_id: volume.physical_volume_id,
        input: input.clone(),
        limitations: profile_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_visible_radiance_bulk_profile(
    profile: &VisibleRadianceBulkProfileV1,
) -> Result<(), VisibleRadianceBulkError> {
    if &compile_visible_radiance_bulk_profile(&profile.input)? != profile {
        return Err(VisibleRadianceBulkError::Invalid("bulk-profile drift"));
    }
    Ok(())
}

pub fn compile_visible_radiance_bulk_transfer(
    profile: &VisibleRadianceBulkProfileV1,
    query: &VisibleRadianceBulkQueryV1,
) -> Result<VisibleRadianceBulkTransferV1, VisibleRadianceBulkError> {
    validate_visible_radiance_bulk_profile(profile)?;
    validate_bulk_query(profile, query)?;
    let (recipe, volume) = rebuild_volume(&profile.input)?;
    let witness = compile_path_witness(&recipe, &volume, &query.path_query)?;
    let outcome = classify_and_transfer(profile, query, &recipe, &volume, &witness)?;
    let transfer_bytes = encode(&(
        profile.visible_radiance_bulk_profile_id,
        volume.physical_volume_id,
        witness.path_query_id,
        witness.path_witness_id,
        &outcome,
    ))?;
    Ok(VisibleRadianceBulkTransferV1 {
        schema_version: CONTRACT_VERSION,
        visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
        physical_volume_id: volume.physical_volume_id,
        path_query_id: witness.path_query_id,
        path_witness_id: witness.path_witness_id,
        outcome,
        visible_radiance_bulk_transfer_id: hash(
            b"mindwarp.visible-radiance.bulk-transfer.v1",
            &transfer_bytes,
        ),
        limitations: transfer_limitations(),
        authority_effect: "none_evidence_only".into(),
    })
}

pub fn validate_visible_radiance_bulk_transfer(
    profile: &VisibleRadianceBulkProfileV1,
    query: &VisibleRadianceBulkQueryV1,
    transfer: &VisibleRadianceBulkTransferV1,
) -> Result<(), VisibleRadianceBulkError> {
    if &compile_visible_radiance_bulk_transfer(profile, query)? != transfer {
        return Err(VisibleRadianceBulkError::Invalid("bulk-transfer drift"));
    }
    Ok(())
}

fn rebuild_volume(
    input: &VisibleRadianceBulkProfileInputV1,
) -> Result<(PhysicalVolumeRecipeV1, PhysicalVolumeV1), VisibleRadianceBulkError> {
    let recipe = compile_physical_volume_recipe(&input.physical_volume_recipe_input)?;
    let volume = compile_physical_volume(&recipe)?;
    validate_physical_volume(&recipe, &volume)?;
    Ok((recipe, volume))
}

fn validate_profile_input(
    input: &VisibleRadianceBulkProfileInputV1,
) -> Result<(), VisibleRadianceBulkError> {
    if input.schema_version != CONTRACT_VERSION {
        return Err(VisibleRadianceBulkError::Invalid(
            "unsupported bulk-profile schema",
        ));
    }
    if input.profile_source_id == [0; 32]
        || input.scope_id == [0; 32]
        || input.reconstruction_id == [0; 32]
        || input.profile_revision == 0
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk-profile identity or revision is zero",
        ));
    }
    if input.reconstruction_id != input.physical_volume_recipe_input.reconstruction_id {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk-profile reconstruction mismatch",
        ));
    }
    if input.substance_interactions.len() > MAX_BULK_PROFILE_SUBSTANCES {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk-profile substance ceiling exceeded",
        ));
    }
    let _ = rebuild_volume(input)?;
    let mut expected = BTreeSet::new();
    collect_substance(
        &input.physical_volume_recipe_input.default_evidence,
        &mut expected,
    )?;
    for run in &input.physical_volume_recipe_input.column_runs {
        collect_substance(&run.evidence, &mut expected)?;
    }
    let mut previous = None;
    let mut actual = Vec::with_capacity(input.substance_interactions.len());
    for entry in &input.substance_interactions {
        if entry.substance_source_id == [0; 32]
            || previous.is_some_and(|value| value >= entry.substance_source_id)
        {
            return Err(VisibleRadianceBulkError::Invalid(
                "noncanonical bulk-profile substance order",
            ));
        }
        previous = Some(entry.substance_source_id);
        actual.push(entry.substance_source_id);
    }
    if actual != expected.into_iter().collect::<Vec<_>>() {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk-profile substance coverage mismatch",
        ));
    }
    Ok(())
}

fn collect_substance(
    evidence: &CellEvidenceV1,
    output: &mut BTreeSet<Id>,
) -> Result<(), VisibleRadianceBulkError> {
    match evidence {
        CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } => {
            if *substance_source_id == [0; 32] {
                return Err(VisibleRadianceBulkError::Invalid(
                    "bulk-profile substance identity is zero",
                ));
            }
            output.insert(*substance_source_id);
        }
        CellEvidenceV1::Unavailable | CellEvidenceV1::Vacuum => {}
    }
    Ok(())
}

fn validate_bulk_query(
    profile: &VisibleRadianceBulkProfileV1,
    query: &VisibleRadianceBulkQueryV1,
) -> Result<(), VisibleRadianceBulkError> {
    validate_visible_radiance_bulk_profile(profile)?;
    if query.schema_version != CONTRACT_VERSION
        || query.visible_radiance_bulk_profile_id != profile.visible_radiance_bulk_profile_id
    {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk-transfer query profile mismatch",
        ));
    }
    let (recipe, volume) = rebuild_volume(&profile.input)?;
    let _ = query.path_query.to_bytes(&recipe, &volume)?;
    checked_squared_delta(&query.path_query)?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MediumKey {
    Vacuum,
    Substance(Id),
}

fn classify_and_transfer(
    profile: &VisibleRadianceBulkProfileV1,
    query: &VisibleRadianceBulkQueryV1,
    recipe: &PhysicalVolumeRecipeV1,
    volume: &PhysicalVolumeV1,
    witness: &PhysicalPathWitnessV1,
) -> Result<BulkTransferOutcomeV1, VisibleRadianceBulkError> {
    validate_path_witness(recipe, volume, &query.path_query, witness)?;
    let evidence = reconstruct_evidence(recipe)?;
    if query.path_query.start_q32_32 == query.path_query.end_q32_32 {
        let unavailable = witness.records.iter().any(|record| {
            matches!(
                evidence.get(&record.index),
                Some(CellEvidenceV1::Unavailable)
            )
        });
        return Ok(if unavailable {
            BulkTransferOutcomeV1::UnavailableEvidence
        } else {
            known_vacuum()
        });
    }

    let interval_records = witness
        .records
        .iter()
        .filter(|record| record.intersection_kind == PathIntersectionKindV1::Interval)
        .collect::<Vec<_>>();
    if interval_records.is_empty() {
        return Err(VisibleRadianceBulkError::Invalid(
            "nonstationary path has no interval evidence",
        ));
    }
    let mut breakpoints = interval_records
        .iter()
        .flat_map(|record| [record.t_enter, record.t_exit])
        .collect::<Vec<_>>();
    breakpoints.sort();
    breakpoints.dedup();
    let mut sequence = Vec::new();
    for pair in breakpoints.windows(2) {
        if pair[0] == pair[1] {
            continue;
        }
        let active = interval_records
            .iter()
            .filter(|record| record.t_enter <= pair[0] && record.t_exit >= pair[1])
            .collect::<Vec<_>>();
        if active.iter().any(|record| {
            matches!(
                evidence.get(&record.index),
                Some(CellEvidenceV1::Unavailable)
            )
        }) {
            return Ok(BulkTransferOutcomeV1::UnavailableEvidence);
        }
        if active.len() != 1 {
            return Ok(BulkTransferOutcomeV1::AmbiguousBoundaryLane);
        }
        let key = medium_key(evidence.get(&active[0].index).ok_or(
            VisibleRadianceBulkError::Invalid("witness cell evidence missing"),
        )?)?;
        if sequence.last() != Some(&key) {
            sequence.push(key);
        }
    }
    if sequence.len() != 1 {
        return Ok(BulkTransferOutcomeV1::InterfaceModelRequired);
    }
    match sequence[0] {
        MediumKey::Vacuum => Ok(known_vacuum()),
        MediumKey::Substance(id) => {
            let entry = profile
                .input
                .substance_interactions
                .binary_search_by_key(&id, |value| value.substance_source_id)
                .ok()
                .and_then(|index| profile.input.substance_interactions.get(index))
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk-profile interaction missing",
                ))?;
            let (length_lower, length_upper) = directed_length_bounds(&query.path_query)?;
            let bands_rgb = [
                compile_band_transfer(&entry.bands_rgb[0], length_lower, length_upper)?,
                compile_band_transfer(&entry.bands_rgb[1], length_lower, length_upper)?,
                compile_band_transfer(&entry.bands_rgb[2], length_lower, length_upper)?,
            ];
            Ok(BulkTransferOutcomeV1::Known { bands_rgb })
        }
    }
}

fn reconstruct_evidence(
    recipe: &PhysicalVolumeRecipeV1,
) -> Result<BTreeMap<CellIndex3V1, CellEvidenceV1>, VisibleRadianceBulkError> {
    let mut output = BTreeMap::new();
    for x in 0..recipe.input.extent[0] {
        for y in 0..recipe.input.extent[1] {
            for z in 0..recipe.input.extent[2] {
                let index = CellIndex3V1 { x, y, z };
                output.insert(index, recipe.input.default_evidence.clone());
            }
        }
    }
    for run in &recipe.input.column_runs {
        for z in run.z_start..run.z_start + run.length {
            output.insert(
                CellIndex3V1 {
                    x: run.x_index,
                    y: run.y_index,
                    z,
                },
                run.evidence.clone(),
            );
        }
    }
    Ok(output)
}

fn medium_key(evidence: &CellEvidenceV1) -> Result<MediumKey, VisibleRadianceBulkError> {
    match evidence {
        CellEvidenceV1::Vacuum => Ok(MediumKey::Vacuum),
        CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } => Ok(MediumKey::Substance(*substance_source_id)),
        CellEvidenceV1::Unavailable => Err(VisibleRadianceBulkError::Invalid(
            "unavailable medium reached classification",
        )),
    }
}

fn known_vacuum() -> BulkTransferOutcomeV1 {
    BulkTransferOutcomeV1::Known {
        bands_rgb: std::array::from_fn(|_| BandTransferV1::VacuumIdentity),
    }
}

fn compile_band_transfer(
    interaction: &BulkBandInteractionV1,
    length_lower_q32_32: u64,
    length_upper_q32_32: u64,
) -> Result<BandTransferV1, VisibleRadianceBulkError> {
    match interaction {
        BulkBandInteractionV1::Opaque => Ok(BandTransferV1::Opaque),
        BulkBandInteractionV1::Finite {
            extinction_q16_48_per_coordinate_unit,
        } if *extinction_q16_48_per_coordinate_unit == 0 => Ok(BandTransferV1::Finite {
            optical_depth_lower_q64_64: FixedU128V1::from_u128(0),
            optical_depth_upper_q64_64: FixedU128V1::from_u128(0),
            transmission_lower_q0_48: TRANSMISSION_ONE_Q0_48,
            transmission_upper_q0_48: TRANSMISSION_ONE_Q0_48,
        }),
        BulkBandInteractionV1::Finite {
            extinction_q16_48_per_coordinate_unit,
        } => {
            let lower_product = u128::from(length_lower_q32_32)
                .checked_mul(u128::from(*extinction_q16_48_per_coordinate_unit))
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk optical-depth multiplication overflow",
                ))?;
            let upper_product = u128::from(length_upper_q32_32)
                .checked_mul(u128::from(*extinction_q16_48_per_coordinate_unit))
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk optical-depth multiplication overflow",
                ))?;
            let optical_lower = lower_product >> 16;
            let optical_upper = ceil_div_pow2(upper_product, 16)?;
            let (transmission_lower_q64, _) = exp_neg_q0_64_bounds(optical_upper)?;
            let (_, transmission_upper_q64) = exp_neg_q0_64_bounds(optical_lower)?;
            let transmission_lower_q0_48 = (transmission_lower_q64 >> 16) as u64;
            let transmission_upper_q0_48 =
                u64::try_from(ceil_div_pow2(transmission_upper_q64, 16)?).map_err(|_| {
                    VisibleRadianceBulkError::Invalid("bulk transmission output overflow")
                })?;
            if transmission_upper_q0_48 < transmission_lower_q0_48
                || transmission_upper_q0_48 > TRANSMISSION_ONE_Q0_48
                || transmission_upper_q0_48 - transmission_lower_q0_48 > 1
            {
                return Err(VisibleRadianceBulkError::Invalid(
                    "bulk transmission enclosure width invalid",
                ));
            }
            Ok(BandTransferV1::Finite {
                optical_depth_lower_q64_64: FixedU128V1::from_u128(optical_lower),
                optical_depth_upper_q64_64: FixedU128V1::from_u128(optical_upper),
                transmission_lower_q0_48,
                transmission_upper_q0_48,
            })
        }
    }
}

fn directed_length_bounds(
    query: &PhysicalPathQueryV1,
) -> Result<(u64, u64), VisibleRadianceBulkError> {
    let squared = checked_squared_delta(query)?;
    let lower = integer_sqrt(squared);
    let upper = if lower * lower == squared {
        lower
    } else {
        lower + 1
    };
    Ok((
        u64::try_from(lower)
            .map_err(|_| VisibleRadianceBulkError::Invalid("bulk length lower ceiling exceeded"))?,
        u64::try_from(upper)
            .map_err(|_| VisibleRadianceBulkError::Invalid("bulk length upper ceiling exceeded"))?,
    ))
}

fn checked_squared_delta(query: &PhysicalPathQueryV1) -> Result<u128, VisibleRadianceBulkError> {
    query
        .start_q32_32
        .iter()
        .zip(query.end_q32_32.iter())
        .try_fold(0_u128, |sum, (start, end)| {
            let delta = (i128::from(*end) - i128::from(*start)).unsigned_abs();
            let square = delta
                .checked_mul(delta)
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk squared-length arithmetic ceiling",
                ))?;
            sum.checked_add(square)
                .ok_or(VisibleRadianceBulkError::Invalid(
                    "bulk squared-length arithmetic ceiling",
                ))
        })
}

fn integer_sqrt(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let mut current = 1_u128 << ((128 - value.leading_zeros() + 1) / 2);
    loop {
        let next = (current + value / current) / 2;
        if next >= current {
            return current;
        }
        current = next;
    }
}

fn exp_neg_q0_64_bounds(value_q64_64: u128) -> Result<(u128, u128), VisibleRadianceBulkError> {
    if value_q64_64 == 0 {
        return Ok((Q0_64_ONE, Q0_64_ONE));
    }
    let mut shifts = 0_u32;
    while ceil_div_pow2(value_q64_64, shifts)? > Q0_64_HALF {
        shifts = shifts
            .checked_add(1)
            .ok_or(VisibleRadianceBulkError::Invalid(
                "bulk exponential range reduction overflow",
            ))?;
        if shifts > 127 {
            return Err(VisibleRadianceBulkError::Invalid(
                "bulk exponential range reduction ceiling",
            ));
        }
    }
    let lower_argument = value_q64_64 >> shifts;
    let upper_argument = ceil_div_pow2(value_q64_64, shifts)?;
    let (mut lower, _) = exp_base_q0_64(upper_argument)?;
    let (_, mut upper) = exp_base_q0_64(lower_argument)?;
    for _ in 0..shifts {
        lower = square_q0_64_floor(lower)?;
        upper = square_q0_64_ceil(upper)?;
    }
    Ok((lower, upper))
}

fn exp_base_q0_64(argument: u128) -> Result<(u128, u128), VisibleRadianceBulkError> {
    if argument > Q0_64_HALF {
        return Err(VisibleRadianceBulkError::Invalid(
            "bulk exponential base range exceeded",
        ));
    }
    let mut term_lower = Q0_64_ONE;
    let mut term_upper = Q0_64_ONE;
    let mut sum_lower = Q0_64_ONE;
    let mut sum_upper = Q0_64_ONE;
    for term_index in 1..=MAX_EXP_TERMS {
        let denominator = Q0_64_ONE.checked_mul(u128::from(term_index)).ok_or(
            VisibleRadianceBulkError::Invalid("bulk exponential denominator overflow"),
        )?;
        term_lower = mul_div_floor(term_lower, argument, denominator)?;
        term_upper = mul_div_ceil(term_upper, argument, denominator)?;
        if term_index % 2 == 1 {
            sum_lower =
                sum_lower
                    .checked_sub(term_upper)
                    .ok_or(VisibleRadianceBulkError::Invalid(
                        "bulk exponential lower underflow",
                    ))?;
            sum_upper =
                sum_upper
                    .checked_sub(term_lower)
                    .ok_or(VisibleRadianceBulkError::Invalid(
                        "bulk exponential upper underflow",
                    ))?;
        } else {
            sum_lower =
                sum_lower
                    .checked_add(term_lower)
                    .ok_or(VisibleRadianceBulkError::Invalid(
                        "bulk exponential lower overflow",
                    ))?;
            sum_upper =
                sum_upper
                    .checked_add(term_upper)
                    .ok_or(VisibleRadianceBulkError::Invalid(
                        "bulk exponential upper overflow",
                    ))?;
        }
        let next_denominator = Q0_64_ONE.checked_mul(u128::from(term_index + 1)).ok_or(
            VisibleRadianceBulkError::Invalid("bulk exponential next denominator overflow"),
        )?;
        let next_upper = mul_div_ceil(term_upper, argument, next_denominator)?;
        if next_upper <= 1 {
            return if term_index % 2 == 1 {
                Ok((
                    sum_lower,
                    sum_upper
                        .checked_add(next_upper)
                        .ok_or(VisibleRadianceBulkError::Invalid(
                            "bulk exponential terminal overflow",
                        ))?,
                ))
            } else {
                Ok((
                    sum_lower
                        .checked_sub(next_upper)
                        .ok_or(VisibleRadianceBulkError::Invalid(
                            "bulk exponential terminal underflow",
                        ))?,
                    sum_upper,
                ))
            };
        }
    }
    Err(VisibleRadianceBulkError::Invalid(
        "bulk exponential did not converge",
    ))
}

fn mul_div_floor(
    left: u128,
    right: u128,
    denominator: u128,
) -> Result<u128, VisibleRadianceBulkError> {
    Ok(left
        .checked_mul(right)
        .ok_or(VisibleRadianceBulkError::Invalid(
            "bulk directed multiplication overflow",
        ))?
        / denominator)
}

fn mul_div_ceil(
    left: u128,
    right: u128,
    denominator: u128,
) -> Result<u128, VisibleRadianceBulkError> {
    let product = left
        .checked_mul(right)
        .ok_or(VisibleRadianceBulkError::Invalid(
            "bulk directed multiplication overflow",
        ))?;
    Ok(product / denominator + u128::from(product % denominator != 0))
}

fn square_q0_64_floor(value: u128) -> Result<u128, VisibleRadianceBulkError> {
    if value == Q0_64_ONE {
        return Ok(value);
    }
    mul_div_floor(value, value, Q0_64_ONE)
}

fn square_q0_64_ceil(value: u128) -> Result<u128, VisibleRadianceBulkError> {
    if value == Q0_64_ONE {
        return Ok(value);
    }
    mul_div_ceil(value, value, Q0_64_ONE)
}

fn ceil_div_pow2(value: u128, shift: u32) -> Result<u128, VisibleRadianceBulkError> {
    if shift == 0 {
        return Ok(value);
    }
    if shift >= 128 {
        return Ok(u128::from(value != 0));
    }
    let mask = (1_u128 << shift) - 1;
    Ok((value >> shift) + u128::from(value & mask != 0))
}

fn profile_limitations() -> Vec<String> {
    vec![
        "declared volume-bound three-band bulk extinction evidence only; no coefficient catalogue or SI claim".into(),
        "no interface reflection refraction scattering emission perception rendering passage biome planet runtime approval or promotion claim".into(),
    ]
}

fn transfer_limitations() -> Vec<String> {
    vec![
        "observer-independent single-medium direct-beam bulk-transfer bounds only".into(),
        "material transitions require an interface model; no perception rendering gameplay biome planet runtime approval or promotion claim".into(),
    ]
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, VisibleRadianceBulkError> {
    serde_json::to_vec(value).map_err(|error| VisibleRadianceBulkError::Codec(error.to_string()))
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VisibleRadianceBulkError> {
    serde_json::from_slice(bytes)
        .map_err(|error| VisibleRadianceBulkError::Codec(error.to_string()))
}

fn require_canonical<T: Serialize>(
    value: &T,
    bytes: &[u8],
    message: &'static str,
) -> Result<(), VisibleRadianceBulkError> {
    if encode(value)? != bytes {
        return Err(VisibleRadianceBulkError::Invalid(message));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use physical_path_substrate::{AdjacencyV1, BoundaryModeV1, ColumnRunV1, CoordinateFrameV1};
    use std::time::Instant;

    const ONE: i64 = 1_i64 << 32;

    fn id(value: u32) -> Id {
        let mut result = [0_u8; 32];
        result[..4].copy_from_slice(&value.to_le_bytes());
        result[31] = 1;
        result
    }

    fn recipe(
        default_evidence: CellEvidenceV1,
        extent: [u32; 3],
        runs: Vec<ColumnRunV1>,
    ) -> PhysicalVolumeRecipeInputV1 {
        PhysicalVolumeRecipeInputV1 {
            schema_version: 1,
            recipe_source_id: id(1),
            scope_id: id(2),
            reconstruction_id: id(3),
            recipe_revision: 1,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            origin_q32_32: [0; 3],
            cell_step_q32_32: ONE,
            extent,
            boundary_mode: BoundaryModeV1::BoundedAbsent,
            adjacency: AdjacencyV1::SharedFace6,
            default_evidence,
            column_runs: runs,
        }
    }

    fn finite(value: u64) -> BulkBandInteractionV1 {
        BulkBandInteractionV1::Finite {
            extinction_q16_48_per_coordinate_unit: value,
        }
    }

    fn profile_input(
        volume: PhysicalVolumeRecipeInputV1,
        ids: &[Id],
    ) -> VisibleRadianceBulkProfileInputV1 {
        VisibleRadianceBulkProfileInputV1 {
            schema_version: 1,
            profile_source_id: id(10),
            scope_id: id(11),
            reconstruction_id: volume.reconstruction_id,
            profile_revision: 1,
            physical_volume_recipe_input: volume,
            substance_interactions: ids
                .iter()
                .map(|substance_source_id| SubstanceBulkInteractionV1 {
                    substance_source_id: *substance_source_id,
                    bands_rgb: [
                        finite(1_u64 << 47),
                        finite(1_u64 << 48),
                        BulkBandInteractionV1::Opaque,
                    ],
                })
                .collect(),
        }
    }

    fn compile_query(
        input: VisibleRadianceBulkProfileInputV1,
        start: [i64; 3],
        end: [i64; 3],
    ) -> (VisibleRadianceBulkProfileV1, VisibleRadianceBulkQueryV1) {
        let profile = compile_visible_radiance_bulk_profile(&input).unwrap();
        let query = VisibleRadianceBulkQueryV1 {
            schema_version: 1,
            visible_radiance_bulk_profile_id: profile.visible_radiance_bulk_profile_id,
            path_query: PhysicalPathQueryV1 {
                schema_version: 1,
                physical_volume_id: profile.physical_volume_id,
                start_q32_32: start,
                end_q32_32: end,
            },
        };
        (profile, query)
    }

    #[test]
    fn strict_profile_query_and_transfer_replay() {
        let substance = id(20);
        let input = profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
                [2, 1, 1],
                vec![],
            ),
            &[substance],
        );
        let (profile, query) = compile_query(
            input.clone(),
            [0, ONE / 2, ONE / 2],
            [2 * ONE, ONE / 2, ONE / 2],
        );
        let transfer = compile_visible_radiance_bulk_transfer(&profile, &query).unwrap();
        assert_eq!(
            VisibleRadianceBulkProfileInputV1::from_bytes(&input.to_bytes().unwrap()).unwrap(),
            input
        );
        assert_eq!(
            VisibleRadianceBulkProfileV1::from_bytes(&profile.to_bytes().unwrap()).unwrap(),
            profile
        );
        assert_eq!(
            VisibleRadianceBulkQueryV1::from_bytes(&query.to_bytes(&profile).unwrap(), &profile)
                .unwrap(),
            query
        );
        assert_eq!(
            VisibleRadianceBulkTransferV1::from_bytes(
                &transfer.to_bytes(&profile, &query).unwrap(),
                &profile,
                &query
            )
            .unwrap(),
            transfer
        );
    }

    #[test]
    fn profile_coverage_order_identity_and_schema_fail_closed() {
        let a = id(20);
        let b = id(21);
        let volume = recipe(
            CellEvidenceV1::Gas {
                substance_source_id: a,
            },
            [1, 1, 2],
            vec![ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: 1,
                length: 1,
                evidence: CellEvidenceV1::Solid {
                    substance_source_id: b,
                },
            }],
        );
        let mut valid = profile_input(volume, &[a, b]);
        assert!(compile_visible_radiance_bulk_profile(&valid).is_ok());
        valid.substance_interactions.swap(0, 1);
        assert!(matches!(
            compile_visible_radiance_bulk_profile(&valid),
            Err(VisibleRadianceBulkError::Invalid(
                "noncanonical bulk-profile substance order"
            ))
        ));
        valid.substance_interactions.pop();
        valid
            .substance_interactions
            .sort_by_key(|entry| entry.substance_source_id);
        assert!(matches!(
            compile_visible_radiance_bulk_profile(&valid),
            Err(VisibleRadianceBulkError::Invalid(
                "bulk-profile substance coverage mismatch"
            ))
        ));
        valid.schema_version = 2;
        assert!(compile_visible_radiance_bulk_profile(&valid).is_err());
    }

    #[test]
    fn vacuum_zero_finite_positive_and_opaque_are_distinct() {
        let vacuum = profile_input(recipe(CellEvidenceV1::Vacuum, [1, 1, 1], vec![]), &[]);
        let (profile, query) =
            compile_query(vacuum, [0, ONE / 2, ONE / 2], [ONE, ONE / 2, ONE / 2]);
        let transfer = compile_visible_radiance_bulk_transfer(&profile, &query).unwrap();
        assert_eq!(transfer.outcome, known_vacuum());

        let substance = id(20);
        let mut input = profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
                [1, 1, 1],
                vec![],
            ),
            &[substance],
        );
        input.substance_interactions[0].bands_rgb[0] = finite(0);
        let (profile, query) = compile_query(input, [0, ONE / 2, ONE / 2], [ONE, ONE / 2, ONE / 2]);
        let transfer = compile_visible_radiance_bulk_transfer(&profile, &query).unwrap();
        let BulkTransferOutcomeV1::Known { bands_rgb } = transfer.outcome else {
            panic!()
        };
        assert!(matches!(
            bands_rgb[0],
            BandTransferV1::Finite {
                transmission_lower_q0_48: TRANSMISSION_ONE_Q0_48,
                transmission_upper_q0_48: TRANSMISSION_ONE_Q0_48,
                ..
            }
        ));
        assert!(
            matches!(bands_rgb[1], BandTransferV1::Finite { transmission_upper_q0_48, .. } if transmission_upper_q0_48 < TRANSMISSION_ONE_Q0_48)
        );
        assert_eq!(bands_rgb[2], BandTransferV1::Opaque);
    }

    #[test]
    fn same_substance_subdivision_and_reversal_are_invariant() {
        let substance = id(20);
        let input = profile_input(
            recipe(
                CellEvidenceV1::Liquid {
                    substance_source_id: substance,
                },
                [4, 1, 1],
                vec![],
            ),
            &[substance],
        );
        let (profile, forward) =
            compile_query(input, [0, ONE / 2, ONE / 2], [4 * ONE, ONE / 2, ONE / 2]);
        let reverse = VisibleRadianceBulkQueryV1 {
            path_query: PhysicalPathQueryV1 {
                start_q32_32: forward.path_query.end_q32_32,
                end_q32_32: forward.path_query.start_q32_32,
                ..forward.path_query.clone()
            },
            ..forward.clone()
        };
        let a = compile_visible_radiance_bulk_transfer(&profile, &forward).unwrap();
        let b = compile_visible_radiance_bulk_transfer(&profile, &reverse).unwrap();
        assert_eq!(a.outcome, b.outcome);
    }

    #[test]
    fn unavailable_ambiguous_and_interfaces_fail_as_typed_evidence() {
        let unavailable =
            profile_input(recipe(CellEvidenceV1::Unavailable, [1, 1, 1], vec![]), &[]);
        let (profile, query) =
            compile_query(unavailable, [0, ONE / 2, ONE / 2], [ONE, ONE / 2, ONE / 2]);
        assert_eq!(
            compile_visible_radiance_bulk_transfer(&profile, &query)
                .unwrap()
                .outcome,
            BulkTransferOutcomeV1::UnavailableEvidence
        );

        let substance = id(20);
        let face = profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
                [1, 2, 1],
                vec![],
            ),
            &[substance],
        );
        let (profile, query) = compile_query(face, [0, ONE, ONE / 2], [ONE, ONE, ONE / 2]);
        assert_eq!(
            compile_visible_radiance_bulk_transfer(&profile, &query)
                .unwrap()
                .outcome,
            BulkTransferOutcomeV1::AmbiguousBoundaryLane
        );

        let a = id(20);
        let b = id(21);
        let mixed = profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: a,
                },
                [2, 1, 1],
                vec![ColumnRunV1 {
                    x_index: 1,
                    y_index: 0,
                    z_start: 0,
                    length: 1,
                    evidence: CellEvidenceV1::Solid {
                        substance_source_id: b,
                    },
                }],
            ),
            &[a, b],
        );
        let (profile, query) =
            compile_query(mixed, [0, ONE / 2, ONE / 2], [2 * ONE, ONE / 2, ONE / 2]);
        assert_eq!(
            compile_visible_radiance_bulk_transfer(&profile, &query)
                .unwrap()
                .outcome,
            BulkTransferOutcomeV1::InterfaceModelRequired
        );
    }

    #[test]
    fn stationary_unavailable_and_point_contact_rules_hold() {
        let unavailable =
            profile_input(recipe(CellEvidenceV1::Unavailable, [1, 1, 1], vec![]), &[]);
        let (profile, query) = compile_query(unavailable, [ONE / 2; 3], [ONE / 2; 3]);
        assert_eq!(
            compile_visible_radiance_bulk_transfer(&profile, &query)
                .unwrap()
                .outcome,
            BulkTransferOutcomeV1::UnavailableEvidence
        );

        let substance = id(20);
        let tangent = profile_input(
            recipe(
                CellEvidenceV1::Vacuum,
                [2, 2, 1],
                vec![ColumnRunV1 {
                    x_index: 1,
                    y_index: 1,
                    z_start: 0,
                    length: 1,
                    evidence: CellEvidenceV1::Solid {
                        substance_source_id: substance,
                    },
                }],
            ),
            &[substance],
        );
        let (profile, query) = compile_query(tangent, [0, 0, ONE / 2], [ONE, ONE, ONE / 2]);
        assert_eq!(
            compile_visible_radiance_bulk_transfer(&profile, &query)
                .unwrap()
                .outcome,
            known_vacuum()
        );
    }

    #[test]
    fn perfect_square_and_nonsquare_lengths_are_directed() {
        let query = PhysicalPathQueryV1 {
            schema_version: 1,
            physical_volume_id: id(1),
            start_q32_32: [0; 3],
            end_q32_32: [3 * ONE, 4 * ONE, 0],
        };
        assert_eq!(
            directed_length_bounds(&query).unwrap(),
            (5 * ONE as u64, 5 * ONE as u64)
        );
        let diagonal = PhysicalPathQueryV1 {
            end_q32_32: [ONE, ONE, ONE],
            ..query
        };
        let (lower, upper) = directed_length_bounds(&diagonal).unwrap();
        assert_eq!(upper, lower + 1);
    }

    #[test]
    fn squared_length_ceiling_fails_before_transfer() {
        let query = PhysicalPathQueryV1 {
            schema_version: 1,
            physical_volume_id: id(1),
            start_q32_32: [i64::MIN; 3],
            end_q32_32: [i64::MAX; 3],
        };
        assert_eq!(
            checked_squared_delta(&query),
            Err(VisibleRadianceBulkError::Invalid(
                "bulk squared-length arithmetic ceiling"
            ))
        );
    }

    #[test]
    fn monotonicity_and_oracle_fixed_vectors_hold() {
        let interaction = finite(1_u64 << 48);
        let short = compile_band_transfer(&interaction, ONE as u64, ONE as u64).unwrap();
        let long = compile_band_transfer(&interaction, (2 * ONE) as u64, (2 * ONE) as u64).unwrap();
        let upper = |value: &BandTransferV1| match value {
            BandTransferV1::Finite {
                transmission_upper_q0_48,
                ..
            } => *transmission_upper_q0_48,
            _ => panic!(),
        };
        assert!(upper(&long) <= upper(&short));
        let oracle_vectors = [
            (0_u128, (281_474_976_710_656_u64, 281_474_976_710_656_u64)),
            (1, (281_474_976_710_655, 281_474_976_710_656)),
            (1_u128 << 60, (264_421_269_977_109, 264_421_269_977_110)),
            (1_u128 << 63, (170_723_203_316_912, 170_723_203_316_913)),
            (1_u128 << 64, (103_548_857_136_060, 103_548_857_136_061)),
            (2_u128 << 64, (38_093_495_697_155, 38_093_495_697_156)),
            (20_u128 << 64, (580_163, 580_164)),
        ];
        for (value, expected_q0_48) in oracle_vectors {
            let (lower, upper) = exp_neg_q0_64_bounds(value).unwrap();
            assert!(lower <= upper && upper <= Q0_64_ONE);
            assert!(ceil_div_pow2(upper, 16).unwrap() - (lower >> 16) <= 1);
            assert_eq!(
                (
                    (lower >> 16) as u64,
                    ceil_div_pow2(upper, 16).unwrap() as u64
                ),
                expected_q0_48,
            );
        }
    }

    #[test]
    fn forged_state_unknown_fields_and_noncanonical_bytes_fail() {
        let substance = id(20);
        let input = profile_input(
            recipe(
                CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
                [1, 1, 1],
                vec![],
            ),
            &[substance],
        );
        let (profile, query) = compile_query(input, [0, ONE / 2, ONE / 2], [ONE, ONE / 2, ONE / 2]);
        let mut transfer = compile_visible_radiance_bulk_transfer(&profile, &query).unwrap();
        transfer.authority_effect = "approved".into();
        assert!(transfer.to_bytes(&profile, &query).is_err());
        let mut bytes = profile.to_bytes().unwrap();
        bytes.push(b' ');
        assert!(VisibleRadianceBulkProfileV1::from_bytes(&bytes).is_err());
        let unknown = String::from_utf8(profile.to_bytes().unwrap())
            .unwrap()
            .replacen("{", "{\"unknown\":1,", 1);
        assert!(VisibleRadianceBulkProfileV1::from_bytes(unknown.as_bytes()).is_err());
    }

    #[test]
    fn maximum_profile_cost_receipt() {
        let started = Instant::now();
        let mut runs = Vec::with_capacity(MAX_BULK_PROFILE_SUBSTANCES);
        let mut ids = Vec::with_capacity(MAX_BULK_PROFILE_SUBSTANCES);
        for z in 0..MAX_BULK_PROFILE_SUBSTANCES as u32 {
            let substance = id(z + 100);
            ids.push(substance);
            runs.push(ColumnRunV1 {
                x_index: 0,
                y_index: 0,
                z_start: z,
                length: 1,
                evidence: CellEvidenceV1::Gas {
                    substance_source_id: substance,
                },
            });
        }
        ids.sort();
        let input = profile_input(
            recipe(
                CellEvidenceV1::Unavailable,
                [1, 1, MAX_BULK_PROFILE_SUBSTANCES as u32],
                runs,
            ),
            &ids,
        );
        let profile = compile_visible_radiance_bulk_profile(&input).unwrap();
        let bytes = profile.to_bytes().unwrap();
        eprintln!(
            "maximum profile: substances={} bytes={} elapsed_ms={}",
            MAX_BULK_PROFILE_SUBSTANCES,
            bytes.len(),
            started.elapsed().as_millis()
        );
        assert_eq!(
            profile.input.substance_interactions.len(),
            MAX_BULK_PROFILE_SUBSTANCES
        );
    }

    #[test]
    fn public_claims_remain_evidence_only() {
        let joined = profile_limitations()
            .into_iter()
            .chain(transfer_limitations())
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in [
            "coefficient catalogue",
            "interface",
            "perception",
            "rendering",
            "passage",
            "biome",
            "planet",
            "runtime",
            "approval",
            "promotion",
        ] {
            assert!(joined.contains(forbidden));
        }
    }
}
