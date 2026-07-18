//! Capability-free local visible-radiance smooth-dielectric interface evidence.
//!
//! The reference reconstructs an exact physical path, admits at most one
//! explicit shared-face event, and returns bounded power and direction
//! enclosures. It never continues a refracted path or claims perception,
//! rendering, passage, biome, planet, terrain, runtime, approval, or promotion.

mod arithmetic;
mod interval;

pub use interval::*;

use arithmetic::{FixedInterval, PRECISIONS, ProjectedInterval, Signed512, checked_u512_product};
use crypto_bigint::U512;
use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, Id, PathIntersectionKindV1, PhysicalPathError,
    PhysicalPathQueryV1, PhysicalVolumeRecipeInputV1, build_physical_cell, compile_path_witness,
    compile_physical_volume, compile_physical_volume_recipe,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
pub const REFRACTIVE_INDEX_SCALE_BITS: u16 = 48;
pub const POWER_SCALE_BITS: u16 = 48;
pub const DIRECTION_SCALE_BITS: u16 = 62;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SmoothDielectricBandV1 {
    pub eta_a_q16_48: u64,
    pub eta_b_q16_48: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum InterfaceModelV1 {
    SmoothLosslessUnpolarizedDielectric {
        bands_rgb: [SmoothDielectricBandV1; 3],
    },
    Unsupported {
        model_source_id: Id,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FaceInteractionEvidenceV1 {
    pub interaction_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub interaction_revision: u32,
    pub cell_a: CellIndex3V1,
    pub cell_b: CellIndex3V1,
    pub medium_a: CellEvidenceV1,
    pub medium_b: CellEvidenceV1,
    pub model: InterfaceModelV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceInterfaceInputV1 {
    pub schema_version: u16,
    pub profile_source_id: Id,
    pub scope_id: Id,
    pub reconstruction_id: Id,
    pub profile_revision: u32,
    pub physical_volume_recipe_input: PhysicalVolumeRecipeInputV1,
    pub path_query: PhysicalPathQueryV1,
    pub face_interaction: FaceInteractionEvidenceV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FixedScaleV1 {
    Q0_48,
    Q1_62,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DecimalIntervalV1 {
    pub lower: String,
    pub upper: String,
    pub scale: FixedScaleV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BandInterfaceEventV1 {
    pub total_internal_reflection: bool,
    pub reflected_power: DecimalIntervalV1,
    pub transmitted_power: DecimalIntervalV1,
    pub reflected_direction_xyz: [DecimalIntervalV1; 3],
    pub transmitted_direction_xyz: Option<[DecimalIntervalV1; 3]>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArithmeticReceiptV1 {
    pub attempted_fractional_bits: Vec<u16>,
    pub fractional_bit_work_units: u16,
    pub maximum_stored_endpoint_bits: u16,
    pub storage_bits: u16,
    pub derived_maximum_live_bits: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum InterfaceEventOutcomeV1 {
    Known {
        bands_rgb: [BandInterfaceEventV1; 3],
        arithmetic_receipt: ArithmeticReceiptV1,
    },
    NonconvergentEnclosure {
        retained_bands_rgb: [BandInterfaceEventV1; 3],
        arithmetic_receipt: ArithmeticReceiptV1,
        reason_code: String,
    },
    NoInterfaceEvent,
    UnavailableEvidence,
    AmbiguousBoundaryLane,
    AmbiguousInterfaceGeometry,
    MissingInterfaceEvidence,
    UnsupportedInterfaceModel,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VisibleRadianceInterfaceEventV1 {
    pub schema_version: u16,
    pub interface_input_id: Id,
    pub event_id: Id,
    pub outcome: InterfaceEventOutcomeV1,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VisibleRadianceInterfaceError {
    Invalid(&'static str),
    ArithmeticDefect(&'static str),
    Codec(String),
    Physical(PhysicalPathError),
}

impl From<PhysicalPathError> for VisibleRadianceInterfaceError {
    fn from(value: PhysicalPathError) -> Self {
        Self::Physical(value)
    }
}

impl VisibleRadianceInterfaceInputV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
        validate_input(self)?;
        encode(self)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VisibleRadianceInterfaceError> {
        let value: Self = decode(bytes)?;
        validate_input(&value)?;
        require_canonical(&value, bytes, "noncanonical interface input bytes")?;
        Ok(value)
    }
}

impl VisibleRadianceInterfaceEventV1 {
    pub fn to_bytes(
        &self,
        input: &VisibleRadianceInterfaceInputV1,
    ) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
        validate_visible_radiance_interface_event(input, self)?;
        encode(self)
    }
    pub fn from_bytes(
        bytes: &[u8],
        input: &VisibleRadianceInterfaceInputV1,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        let value: Self = decode(bytes)?;
        validate_visible_radiance_interface_event(input, &value)?;
        require_canonical(&value, bytes, "noncanonical interface event bytes")?;
        Ok(value)
    }
}

pub fn compile_visible_radiance_interface_event(
    input: &VisibleRadianceInterfaceInputV1,
) -> Result<VisibleRadianceInterfaceEventV1, VisibleRadianceInterfaceError> {
    validate_input(input)?;
    let input_bytes = encode(input)?;
    let interface_input_id = hash(b"forge-visible-radiance-interface-input-v1", &input_bytes);
    let outcome = compile_outcome(input)?;
    let limitations = limitations();
    let authority_effect = "none".to_owned();
    let event_id = hash(
        b"forge-visible-radiance-interface-event-v1",
        &encode(&(
            interface_input_id,
            &outcome,
            &limitations,
            &authority_effect,
        ))?,
    );
    Ok(VisibleRadianceInterfaceEventV1 {
        schema_version: CONTRACT_VERSION,
        interface_input_id,
        event_id,
        outcome,
        limitations,
        authority_effect,
    })
}

pub fn validate_visible_radiance_interface_event(
    input: &VisibleRadianceInterfaceInputV1,
    event: &VisibleRadianceInterfaceEventV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    let expected = compile_visible_radiance_interface_event(input)?;
    if &expected != event {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interface event replay mismatch",
        ));
    }
    validate_outcome_codec(&event.outcome)
}

fn validate_input(
    input: &VisibleRadianceInterfaceInputV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    if input.schema_version != CONTRACT_VERSION
        || input.profile_source_id == [0; 32]
        || input.scope_id == [0; 32]
        || input.reconstruction_id == [0; 32]
        || input.profile_revision == 0
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid interface input provenance",
        ));
    }
    let face = &input.face_interaction;
    if face.interaction_source_id == [0; 32]
        || face.scope_id != input.scope_id
        || face.reconstruction_id != input.reconstruction_id
        || face.interaction_revision == 0
        || input.physical_volume_recipe_input.reconstruction_id != input.reconstruction_id
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid face interaction provenance",
        ));
    }
    if face.cell_a >= face.cell_b || shared_face_axis(face.cell_a, face.cell_b).is_none() {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "noncanonical shared-face key",
        ));
    }
    if face.medium_a == face.medium_b
        || matches!(face.medium_a, CellEvidenceV1::Unavailable)
        || matches!(face.medium_b, CellEvidenceV1::Unavailable)
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid declared interface media",
        ));
    }
    if let InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb } = &face.model {
        for band in bands_rgb {
            if band.eta_a_q16_48 < (1_u64 << 46)
                || band.eta_a_q16_48 > (16_u64 << 48)
                || band.eta_b_q16_48 < (1_u64 << 46)
                || band.eta_b_q16_48 > (16_u64 << 48)
            {
                return Err(VisibleRadianceInterfaceError::Invalid(
                    "refractive index outside admitted range",
                ));
            }
        }
    } else if let InterfaceModelV1::Unsupported { model_source_id } = &face.model
        && *model_source_id == [0; 32]
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "zero unsupported model identity",
        ));
    }
    let recipe = compile_physical_volume_recipe(&input.physical_volume_recipe_input)?;
    let volume = compile_physical_volume(&recipe)?;
    if input.path_query.schema_version != CONTRACT_VERSION
        || input.path_query.physical_volume_id != volume.physical_volume_id
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interface query volume mismatch",
        ));
    }
    let cell_a = build_physical_cell(&recipe, &volume, face.cell_a)?;
    let cell_b = build_physical_cell(&recipe, &volume, face.cell_b)?;
    if cell_a.evidence != face.medium_a || cell_b.evidence != face.medium_b {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "face interaction medium reconstruction mismatch",
        ));
    }
    Ok(())
}

fn compile_outcome(
    input: &VisibleRadianceInterfaceInputV1,
) -> Result<InterfaceEventOutcomeV1, VisibleRadianceInterfaceError> {
    let recipe = compile_physical_volume_recipe(&input.physical_volume_recipe_input)?;
    let volume = compile_physical_volume(&recipe)?;
    if input.path_query.physical_volume_id != volume.physical_volume_id {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "interface query volume mismatch",
        ));
    }
    let witness = compile_path_witness(&recipe, &volume, &input.path_query)?;
    let intervals = witness
        .records
        .iter()
        .filter(|record| record.intersection_kind == PathIntersectionKindV1::Interval)
        .collect::<Vec<_>>();
    if intervals.is_empty() {
        return Ok(InterfaceEventOutcomeV1::NoInterfaceEvent);
    }
    for pair in intervals.windows(2) {
        if pair[0].t_exit > pair[1].t_enter {
            return Ok(InterfaceEventOutcomeV1::AmbiguousBoundaryLane);
        }
    }
    let mut transitions = Vec::new();
    for pair in intervals.windows(2) {
        if pair[0].t_exit != pair[1].t_enter {
            continue;
        }
        let before = build_physical_cell(&recipe, &volume, pair[0].index)?;
        let after = build_physical_cell(&recipe, &volume, pair[1].index)?;
        if matches!(before.evidence, CellEvidenceV1::Unavailable)
            || matches!(after.evidence, CellEvidenceV1::Unavailable)
        {
            return Ok(InterfaceEventOutcomeV1::UnavailableEvidence);
        }
        if before.evidence != after.evidence {
            transitions.push((before, after));
        }
    }
    if transitions.is_empty() {
        return Ok(InterfaceEventOutcomeV1::NoInterfaceEvent);
    }
    if transitions.len() != 1 {
        return Ok(InterfaceEventOutcomeV1::AmbiguousInterfaceGeometry);
    }
    let (before, after) = transitions.pop().expect("one transition");
    let axis = match shared_face_axis(before.index, after.index) {
        Some(value) => value,
        None => return Ok(InterfaceEventOutcomeV1::AmbiguousInterfaceGeometry),
    };
    let face = &input.face_interaction;
    let (cell_a, medium_a, medium_b, forward) = if before.index < after.index {
        (before.index, &before.evidence, &after.evidence, true)
    } else {
        (after.index, &after.evidence, &before.evidence, false)
    };
    if face.cell_a != cell_a
        || face.cell_b != if forward { after.index } else { before.index }
        || &face.medium_a != medium_a
        || &face.medium_b != medium_b
    {
        return Ok(InterfaceEventOutcomeV1::MissingInterfaceEvidence);
    }
    let InterfaceModelV1::SmoothLosslessUnpolarizedDielectric { bands_rgb } = &face.model else {
        return Ok(InterfaceEventOutcomeV1::UnsupportedInterfaceModel);
    };
    let mut delta = [0_i128; 3];
    for index in 0..3 {
        delta[index] = i128::from(input.path_query.end_q32_32[index])
            - i128::from(input.path_query.start_q32_32[index]);
    }
    let normal_sign = if coordinate(after.index, axis) > coordinate(before.index, axis) {
        1_i128
    } else {
        -1
    };
    if delta[axis] * normal_sign <= 0 {
        return Ok(InterfaceEventOutcomeV1::AmbiguousInterfaceGeometry);
    }
    let oriented = OrientedCase {
        delta,
        axis,
        normal_sign,
    };
    let band_inputs = bands_rgb.clone().map(|band| {
        if forward {
            (band.eta_a_q16_48, band.eta_b_q16_48)
        } else {
            (band.eta_b_q16_48, band.eta_a_q16_48)
        }
    });
    adaptive_outcome(&oriented, &band_inputs)
}

#[derive(Clone, Copy)]
struct OrientedCase {
    delta: [i128; 3],
    axis: usize,
    normal_sign: i128,
}

#[derive(Clone, Debug)]
struct ProjectedBand {
    tir: bool,
    reflected_power: ProjectedInterval,
    transmitted_power: ProjectedInterval,
    reflected: [ProjectedInterval; 3],
    transmitted: Option<[ProjectedInterval; 3]>,
    max_stored_bits: u16,
}

impl ProjectedBand {
    fn intersect(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        if self.tir != other.tir || self.transmitted.is_some() != other.transmitted.is_some() {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "adaptive branch changed",
            ));
        }
        let reflected = [
            self.reflected[0].intersect(&other.reflected[0])?,
            self.reflected[1].intersect(&other.reflected[1])?,
            self.reflected[2].intersect(&other.reflected[2])?,
        ];
        let transmitted = match (&self.transmitted, &other.transmitted) {
            (Some(a), Some(b)) => Some([
                a[0].intersect(&b[0])?,
                a[1].intersect(&b[1])?,
                a[2].intersect(&b[2])?,
            ]),
            (None, None) => None,
            _ => {
                return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                    "adaptive transmitted branch changed",
                ));
            }
        };
        Ok(Self {
            tir: self.tir,
            reflected_power: self.reflected_power.intersect(&other.reflected_power)?,
            transmitted_power: self.transmitted_power.intersect(&other.transmitted_power)?,
            reflected,
            transmitted,
            max_stored_bits: self.max_stored_bits.max(other.max_stored_bits),
        })
    }
    fn certified(&self) -> Result<bool, VisibleRadianceInterfaceError> {
        let mut result = self.reflected_power.certified_one_unit()?
            && self.transmitted_power.certified_one_unit()?;
        for component in &self.reflected {
            result &= component.certified_one_unit()?;
        }
        if let Some(transmitted) = &self.transmitted {
            for component in transmitted {
                result &= component.certified_one_unit()?;
            }
        }
        Ok(result)
    }
    fn public(&self) -> BandInterfaceEventV1 {
        BandInterfaceEventV1 {
            total_internal_reflection: self.tir,
            reflected_power: public_interval(&self.reflected_power, FixedScaleV1::Q0_48),
            transmitted_power: public_interval(&self.transmitted_power, FixedScaleV1::Q0_48),
            reflected_direction_xyz: self
                .reflected
                .clone()
                .map(|v| public_interval(&v, FixedScaleV1::Q1_62)),
            transmitted_direction_xyz: self
                .transmitted
                .clone()
                .map(|values| values.map(|v| public_interval(&v, FixedScaleV1::Q1_62))),
        }
    }
}

fn adaptive_outcome(
    case: &OrientedCase,
    bands: &[(u64, u64); 3],
) -> Result<InterfaceEventOutcomeV1, VisibleRadianceInterfaceError> {
    if let Some(exact) = exact_fast_outcome(case, bands)? {
        let maximum_stored_endpoint_bits = exact
            .iter()
            .map(|band| band.max_stored_bits)
            .max()
            .unwrap_or(0);
        return Ok(InterfaceEventOutcomeV1::Known {
            bands_rgb: exact.map(|band| band.public()),
            arithmetic_receipt: receipt(&[], maximum_stored_endpoint_bits),
        });
    }
    adaptive_outcome_with_precisions(case, bands, &PRECISIONS)
}

fn adaptive_outcome_with_precisions(
    case: &OrientedCase,
    bands: &[(u64, u64); 3],
    precisions: &[u16],
) -> Result<InterfaceEventOutcomeV1, VisibleRadianceInterfaceError> {
    if precisions.is_empty() || !PRECISIONS.starts_with(precisions) {
        return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
            "invalid adaptive precision schedule",
        ));
    }
    let mut retained: Option<[ProjectedBand; 3]> = None;
    let mut attempted = Vec::new();
    let mut maximum_stored_endpoint_bits = 0;
    let wide_geometry_guard = squared_delta(case.delta)? > u128::from(u64::MAX);
    for &bits in precisions {
        attempted.push(bits);
        let current = [
            staged_band(case, bands[0].0, bands[0].1, bits)?,
            staged_band(case, bands[1].0, bands[1].1, bits)?,
            staged_band(case, bands[2].0, bands[2].1, bits)?,
        ];
        let next = match retained.take() {
            None => current,
            Some(previous) => [
                previous[0].intersect(&current[0])?,
                previous[1].intersect(&current[1])?,
                previous[2].intersect(&current[2])?,
            ],
        };
        maximum_stored_endpoint_bits = maximum_stored_endpoint_bits.max(
            next.iter()
                .map(|band| band.max_stored_bits)
                .max()
                .unwrap_or(0),
        );
        let certified = next.iter().try_fold(true, |value, band| {
            Ok::<_, VisibleRadianceInterfaceError>(value && band.certified()?)
        })?;
        retained = Some(next);
        if certified && (!wide_geometry_guard || bits == 160) {
            let receipt = receipt(&attempted, maximum_stored_endpoint_bits);
            let public = retained.expect("retained event").map(|band| band.public());
            return Ok(InterfaceEventOutcomeV1::Known {
                bands_rgb: public,
                arithmetic_receipt: receipt,
            });
        }
    }
    let receipt = receipt(&attempted, maximum_stored_endpoint_bits);
    let public = retained
        .expect("three attempted levels")
        .map(|band| band.public());
    Ok(InterfaceEventOutcomeV1::NonconvergentEnclosure {
        retained_bands_rgb: public,
        arithmetic_receipt: receipt,
        reason_code: "one_unit_certification_not_reached_at_declared_cap".to_owned(),
    })
}

fn exact_fast_outcome(
    case: &OrientedCase,
    bands: &[(u64, u64); 3],
) -> Result<Option<[ProjectedBand; 3]>, VisibleRadianceInterfaceError> {
    let squared = squared_delta(case.delta)?;
    let normal = (case.delta[case.axis] * case.normal_sign).unsigned_abs();
    let tangent_squared = squared
        .checked_sub(normal.checked_mul(normal).ok_or(
            VisibleRadianceInterfaceError::ArithmeticDefect("normal square overflow"),
        )?)
        .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
            "negative tangent square",
        ))?;
    let root = integer_sqrt(squared);
    let perfect_square = root.checked_mul(root) == Some(squared);
    if tangent_squared != 0 && !perfect_square {
        return Ok(None);
    }

    let mut kinds = [ExactBandKind::General; 3];
    for (index, (eta_i, eta_t)) in bands.iter().copied().enumerate() {
        kinds[index] = if tangent_squared == 0 {
            ExactBandKind::Normal
        } else if eta_i == eta_t {
            ExactBandKind::Matched
        } else if exact_tir(tangent_squared, squared, eta_i, eta_t)? {
            ExactBandKind::Tir
        } else {
            ExactBandKind::General
        };
    }
    if kinds.contains(&ExactBandKind::General) {
        return Ok(None);
    }
    let denominator = root;
    let incident: [Result<ProjectedInterval, VisibleRadianceInterfaceError>; 3] =
        std::array::from_fn(|index| {
            exact_signed_ratio(case.delta[index], denominator, DIRECTION_SCALE_BITS)
        });
    let incident = [
        incident[0].clone()?,
        incident[1].clone()?,
        incident[2].clone()?,
    ];
    let reflected: [Result<ProjectedInterval, VisibleRadianceInterfaceError>; 3] =
        std::array::from_fn(|index| {
            let numerator = if index == case.axis {
                -case.delta[index]
            } else {
                case.delta[index]
            };
            exact_signed_ratio(numerator, denominator, DIRECTION_SCALE_BITS)
        });
    let reflected = [
        reflected[0].clone()?,
        reflected[1].clone()?,
        reflected[2].clone()?,
    ];
    let result: [Result<ProjectedBand, VisibleRadianceInterfaceError>; 3] =
        std::array::from_fn(|index| {
            let (eta_i, eta_t) = bands[index];
            let (tir, reflected_power, transmitted_power, transmitted) = match kinds[index] {
                ExactBandKind::Normal => {
                    let difference = eta_i.abs_diff(eta_t);
                    let numerator = U512::from(difference).checked_mul(&U512::from(difference));
                    let sum = eta_i.checked_add(eta_t).ok_or(
                        VisibleRadianceInterfaceError::ArithmeticDefect(
                            "normal Fresnel sum overflow",
                        ),
                    )?;
                    let denominator_power = U512::from(sum).checked_mul(&U512::from(sum));
                    let numerator = Option::<U512>::from(numerator).ok_or(
                        VisibleRadianceInterfaceError::ArithmeticDefect(
                            "normal Fresnel numerator overflow",
                        ),
                    )?;
                    let denominator_power = Option::<U512>::from(denominator_power).ok_or(
                        VisibleRadianceInterfaceError::ArithmeticDefect(
                            "normal Fresnel denominator overflow",
                        ),
                    )?;
                    let reflected_power =
                        exact_unsigned_ratio(numerator, denominator_power, POWER_SCALE_BITS)?;
                    let transmitted_power = exact_unsigned_ratio(
                        denominator_power.wrapping_sub(&numerator),
                        denominator_power,
                        POWER_SCALE_BITS,
                    )?;
                    (
                        false,
                        reflected_power,
                        transmitted_power,
                        Some(incident.clone()),
                    )
                }
                ExactBandKind::Matched => (
                    false,
                    exact_integer(0, POWER_SCALE_BITS)?,
                    exact_integer(1, POWER_SCALE_BITS)?,
                    Some(incident.clone()),
                ),
                ExactBandKind::Tir => (
                    true,
                    exact_integer(1, POWER_SCALE_BITS)?,
                    exact_integer(0, POWER_SCALE_BITS)?,
                    None,
                ),
                ExactBandKind::General => unreachable!("general exact kind rejected"),
            };
            let max_stored_bits = reflected
                .iter()
                .chain(transmitted.iter().flatten())
                .flat_map(|value| [&value.lower, &value.upper])
                .map(Signed512::bits)
                .chain([
                    reflected_power.lower.bits(),
                    reflected_power.upper.bits(),
                    transmitted_power.lower.bits(),
                    transmitted_power.upper.bits(),
                ])
                .max()
                .unwrap_or(0);
            Ok(ProjectedBand {
                tir,
                reflected_power,
                transmitted_power,
                reflected: reflected.clone(),
                transmitted,
                max_stored_bits,
            })
        });
    Ok(Some([
        result[0].clone()?,
        result[1].clone()?,
        result[2].clone()?,
    ]))
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ExactBandKind {
    Normal,
    Matched,
    Tir,
    General,
}

fn exact_integer(
    value: i128,
    bits: u16,
) -> Result<ProjectedInterval, VisibleRadianceInterfaceError> {
    let raw = Signed512::from_i128(value).checked_shl(bits)?;
    ProjectedInterval::new(raw.clone(), raw, bits)
}

fn exact_signed_ratio(
    numerator: i128,
    denominator: u128,
    bits: u16,
) -> Result<ProjectedInterval, VisibleRadianceInterfaceError> {
    let shifted = Signed512::from_i128(numerator).checked_shl(bits)?;
    let denominator = Signed512::new(false, U512::from(denominator));
    ProjectedInterval::new(
        shifted.div_floor(&denominator)?,
        shifted.div_ceil(&denominator)?,
        bits,
    )
}

fn exact_unsigned_ratio(
    numerator: U512,
    denominator: U512,
    bits: u16,
) -> Result<ProjectedInterval, VisibleRadianceInterfaceError> {
    let shifted = Signed512::new(false, numerator).checked_shl(bits)?;
    let denominator = Signed512::new(false, denominator);
    ProjectedInterval::new(
        shifted.div_floor(&denominator)?,
        shifted.div_ceil(&denominator)?,
        bits,
    )
}

fn receipt(attempted: &[u16], max_stored: u16) -> ArithmeticReceiptV1 {
    ArithmeticReceiptV1 {
        attempted_fractional_bits: attempted.to_vec(),
        fractional_bit_work_units: attempted.iter().sum(),
        maximum_stored_endpoint_bits: max_stored,
        storage_bits: 512,
        derived_maximum_live_bits: 452,
    }
}

fn staged_band(
    case: &OrientedCase,
    eta_i: u64,
    eta_t: u64,
    bits: u16,
) -> Result<ProjectedBand, VisibleRadianceInterfaceError> {
    let zero = FixedInterval::integer(0, bits)?;
    let one = FixedInterval::integer(1, bits)?;
    let two = FixedInterval::integer(2, bits)?;
    let squared = squared_delta(case.delta)?;
    let normal = (case.delta[case.axis] * case.normal_sign).unsigned_abs();
    let normal_squared =
        normal
            .checked_mul(normal)
            .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
                "normal square overflow",
            ))?;
    let tangent_squared = squared.checked_sub(normal_squared).ok_or(
        VisibleRadianceInterfaceError::ArithmeticDefect("negative tangent square"),
    )?;
    let tir = exact_tir(tangent_squared, squared, eta_i, eta_t)?;
    let sqrt_squared =
        FixedInterval::unsigned_ratio(U512::from(squared), U512::ONE, bits)?.sqrt()?;
    let incident = try_components(|index| {
        FixedInterval::integer(case.delta[index], bits).and_then(|v| v.div(&sqrt_squared))
    })?;
    let cos_i = FixedInterval::integer(normal as i128, bits)?.div(&sqrt_squared)?;
    let reflected = try_components(|index| {
        let factor = if index == case.axis {
            2 * case.normal_sign
        } else {
            0
        };
        FixedInterval::integer(factor, bits)
            .and_then(|v| cos_i.mul(&v))
            .and_then(|v| incident[index].sub(&v))
    })?;
    if !sum_squares(&reflected, &zero)?.contains_integer(1)? {
        return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
            "reflected direction lost unit-vector containment",
        ));
    }
    let (reflectance, transmittance, transmitted) = if tir {
        (one.clone(), zero.clone(), None)
    } else {
        let numerator = checked_u512_product(&[
            U512::from(tangent_squared),
            U512::from(eta_i),
            U512::from(eta_i),
        ])?;
        let denominator =
            checked_u512_product(&[U512::from(squared), U512::from(eta_t), U512::from(eta_t)])?;
        let sin_t_squared =
            FixedInterval::unsigned_ratio(numerator, denominator, bits)?.intersect_unit()?;
        let cos_t = one.sub(&sin_t_squared)?.intersect_unit()?.sqrt()?;
        let q = FixedInterval::unsigned_ratio(U512::from(eta_t), U512::from(eta_i), bits)?;
        let q_cos_i = q.mul(&cos_i)?;
        let q_cos_t = q.mul(&cos_t)?;
        let r_parallel = q_cos_i.sub(&cos_t)?.div(&q_cos_i.add(&cos_t)?)?;
        let r_perpendicular = cos_i.sub(&q_cos_t)?.div(&cos_i.add(&q_cos_t)?)?;
        let reflectance = r_parallel
            .square()?
            .add(&r_perpendicular.square()?)?
            .div(&two)?
            .intersect_unit()?;
        let transmittance = one.sub(&reflectance)?;
        let tangent = try_components(|index| {
            let factor = if index == case.axis {
                case.normal_sign
            } else {
                0
            };
            FixedInterval::integer(factor, bits)
                .and_then(|v| cos_i.mul(&v))
                .and_then(|v| incident[index].sub(&v))
        })?;
        let transmitted = try_components(|index| {
            let factor = if index == case.axis {
                case.normal_sign
            } else {
                0
            };
            tangent[index].div(&q).and_then(|v| {
                FixedInterval::integer(factor, bits)
                    .and_then(|n| cos_t.mul(&n))
                    .and_then(|normal_part| v.add(&normal_part))
            })
        })?;
        if !reflectance.add(&transmittance)?.contains_integer(1)? {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "power enclosure lost energy containment",
            ));
        }
        if !sum_squares(&transmitted, &zero)?.contains_integer(1)? {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "transmitted direction lost unit-vector containment",
            ));
        }
        let mut tangent_sum = zero.clone();
        for (index, component) in transmitted.iter().enumerate() {
            if index != case.axis {
                tangent_sum = tangent_sum.add(&component.square()?)?;
            }
        }
        if !tangent_sum.overlaps(&sin_t_squared)? {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "transmitted direction lost Snell containment",
            ));
        }
        (reflectance, transmittance, Some(transmitted))
    };
    let max_stored_bits = reflected.iter().chain(transmitted.iter().flatten()).fold(
        reflectance.max_bits().max(transmittance.max_bits()),
        |maximum, value| maximum.max(value.max_bits()),
    );
    Ok(ProjectedBand {
        tir,
        reflected_power: reflectance.project(POWER_SCALE_BITS)?,
        transmitted_power: transmittance.project(POWER_SCALE_BITS)?,
        reflected: reflected
            .map(|value| value.project(DIRECTION_SCALE_BITS))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .expect("three components"),
        transmitted: transmitted
            .map(|values| {
                values
                    .map(|value| value.project(DIRECTION_SCALE_BITS))
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.try_into().expect("three components"))
            })
            .transpose()?,
        max_stored_bits,
    })
}

fn try_components(
    mut component: impl FnMut(usize) -> Result<FixedInterval, VisibleRadianceInterfaceError>,
) -> Result<[FixedInterval; 3], VisibleRadianceInterfaceError> {
    Ok([component(0)?, component(1)?, component(2)?])
}

fn sum_squares(
    values: &[FixedInterval],
    zero: &FixedInterval,
) -> Result<FixedInterval, VisibleRadianceInterfaceError> {
    values
        .iter()
        .try_fold(zero.clone(), |sum, value| sum.add(&value.square()?))
}

fn exact_tir(
    tangent_squared: u128,
    squared: u128,
    eta_i: u64,
    eta_t: u64,
) -> Result<bool, VisibleRadianceInterfaceError> {
    let left = checked_u512_product(&[
        U512::from(tangent_squared),
        U512::from(eta_i),
        U512::from(eta_i),
    ])?;
    let right = checked_u512_product(&[U512::from(squared), U512::from(eta_t), U512::from(eta_t)])?;
    Ok(left >= right)
}

fn squared_delta(delta: [i128; 3]) -> Result<u128, VisibleRadianceInterfaceError> {
    delta
        .into_iter()
        .try_fold(0_u128, |sum, component| {
            let magnitude = component.unsigned_abs();
            let square =
                magnitude
                    .checked_mul(magnitude)
                    .ok_or(VisibleRadianceInterfaceError::Invalid(
                        "interface squared-length arithmetic ceiling",
                    ))?;
            sum.checked_add(square)
                .ok_or(VisibleRadianceInterfaceError::Invalid(
                    "interface squared-length arithmetic ceiling",
                ))
        })
        .and_then(|value| {
            if value == 0 {
                Err(VisibleRadianceInterfaceError::Invalid(
                    "stationary interface query",
                ))
            } else {
                Ok(value)
            }
        })
}

fn integer_sqrt(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let mut estimate = 1_u128 << ((value.ilog2() + 2) / 2);
    loop {
        let next = (estimate + value / estimate) / 2;
        if next >= estimate {
            return estimate;
        }
        estimate = next;
    }
}

fn shared_face_axis(a: CellIndex3V1, b: CellIndex3V1) -> Option<usize> {
    let av = [a.x, a.y, a.z];
    let bv = [b.x, b.y, b.z];
    let changed = (0..3)
        .filter(|&index| av[index].abs_diff(bv[index]) != 0)
        .collect::<Vec<_>>();
    (changed.len() == 1 && av[changed[0]].abs_diff(bv[changed[0]]) == 1).then_some(changed[0])
}

fn coordinate(cell: CellIndex3V1, axis: usize) -> u32 {
    [cell.x, cell.y, cell.z][axis]
}

fn public_interval(value: &ProjectedInterval, scale: FixedScaleV1) -> DecimalIntervalV1 {
    DecimalIntervalV1 {
        lower: value.lower.canonical_decimal(),
        upper: value.upper.canonical_decimal(),
        scale,
    }
}

fn validate_outcome_codec(
    outcome: &InterfaceEventOutcomeV1,
) -> Result<(), VisibleRadianceInterfaceError> {
    let bands = match outcome {
        InterfaceEventOutcomeV1::Known {
            bands_rgb,
            arithmetic_receipt,
        } => {
            validate_receipt(arithmetic_receipt)?;
            Some((bands_rgb, true))
        }
        InterfaceEventOutcomeV1::NonconvergentEnclosure {
            retained_bands_rgb,
            arithmetic_receipt,
            reason_code,
        } => {
            validate_receipt(arithmetic_receipt)?;
            if reason_code != "one_unit_certification_not_reached_at_declared_cap" {
                return Err(VisibleRadianceInterfaceError::Invalid(
                    "invalid nonconvergence reason",
                ));
            }
            Some((retained_bands_rgb, false))
        }
        _ => None,
    };
    if let Some((bands, require_certified)) = bands {
        for band in bands {
            validate_decimal_interval(&band.reflected_power, POWER_SCALE_BITS, require_certified)?;
            validate_decimal_interval(
                &band.transmitted_power,
                POWER_SCALE_BITS,
                require_certified,
            )?;
            for value in &band.reflected_direction_xyz {
                validate_decimal_interval(value, DIRECTION_SCALE_BITS, require_certified)?;
            }
            if band.total_internal_reflection != band.transmitted_direction_xyz.is_none() {
                return Err(VisibleRadianceInterfaceError::Invalid(
                    "TIR direction shape mismatch",
                ));
            }
            if let Some(values) = &band.transmitted_direction_xyz {
                for value in values {
                    validate_decimal_interval(value, DIRECTION_SCALE_BITS, require_certified)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_receipt(receipt: &ArithmeticReceiptV1) -> Result<(), VisibleRadianceInterfaceError> {
    if !PRECISIONS.starts_with(&receipt.attempted_fractional_bits)
        || receipt.fractional_bit_work_units
            != receipt.attempted_fractional_bits.iter().sum::<u16>()
        || receipt.storage_bits != 512
        || receipt.derived_maximum_live_bits != 452
        || receipt.maximum_stored_endpoint_bits > 512
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "invalid arithmetic receipt",
        ));
    }
    Ok(())
}

fn validate_decimal_interval(
    value: &DecimalIntervalV1,
    expected_bits: u16,
    require_certified: bool,
) -> Result<(), VisibleRadianceInterfaceError> {
    let actual_bits = match value.scale {
        FixedScaleV1::Q0_48 => POWER_SCALE_BITS,
        FixedScaleV1::Q1_62 => DIRECTION_SCALE_BITS,
    };
    if actual_bits != expected_bits {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "fixed scale mismatch",
        ));
    }
    let lower = parse_decimal(&value.lower)?;
    let upper = parse_decimal(&value.upper)?;
    if lower > upper {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "reversed decimal interval",
        ));
    }
    if require_certified && upper.checked_sub(&lower)? > Signed512::one() {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "known interval wider than one unit",
        ));
    }
    Ok(())
}

fn parse_decimal(value: &str) -> Result<Signed512, VisibleRadianceInterfaceError> {
    if value.is_empty()
        || value.starts_with('+')
        || value == "-0"
        || value.starts_with("00")
        || value.starts_with("-0")
        || value
            .bytes()
            .any(|byte| byte != b'-' && !byte.is_ascii_digit())
    {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "noncanonical signed decimal",
        ));
    }
    let (negative, digits) = value
        .strip_prefix('-')
        .map_or((false, value), |digits| (true, digits));
    if digits.is_empty() {
        return Err(VisibleRadianceInterfaceError::Invalid(
            "noncanonical signed decimal",
        ));
    }
    let magnitude = U512::from_str_radix_vartime(digits, 10).map_err(|_| {
        VisibleRadianceInterfaceError::Invalid("signed decimal outside 512-bit range")
    })?;
    Ok(Signed512::new(negative, magnitude))
}

fn limitations() -> Vec<String> {
    vec![
        "local observer-independent smooth lossless unpolarized dielectric interface evidence only".into(),
        "no coefficient catalogue downstream path perception rendering passage biome planet terrain runtime approval or promotion claim".into(),
    ]
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(bytes);
    digest.finalize().into()
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, VisibleRadianceInterfaceError> {
    serde_json::to_vec(value)
        .map_err(|error| VisibleRadianceInterfaceError::Codec(error.to_string()))
}
fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VisibleRadianceInterfaceError> {
    serde_json::from_slice(bytes)
        .map_err(|error| VisibleRadianceInterfaceError::Codec(error.to_string()))
}
fn require_canonical<T: Serialize>(
    value: &T,
    bytes: &[u8],
    message: &'static str,
) -> Result<(), VisibleRadianceInterfaceError> {
    if encode(value)? != bytes {
        return Err(VisibleRadianceInterfaceError::Invalid(message));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use physical_path_substrate::{AdjacencyV1, BoundaryModeV1, ColumnRunV1, CoordinateFrameV1};
    const ONE: i64 = 1_i64 << 32;
    fn id(value: u32) -> Id {
        let mut id = [0; 32];
        id[..4].copy_from_slice(&value.to_le_bytes());
        id[31] = 1;
        id
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
            physical_volume_recipe_input: recipe_input,
        }
    }
    fn smooth(a: u64, b: u64) -> InterfaceModelV1 {
        InterfaceModelV1::SmoothLosslessUnpolarizedDielectric {
            bands_rgb: std::array::from_fn(|_| SmoothDielectricBandV1 {
                eta_a_q16_48: a,
                eta_b_q16_48: b,
            }),
        }
    }
    #[test]
    fn normal_incidence_is_known_and_strictly_replays() {
        let input = fixture(smooth(1 << 48, 3 << 47), [3 * ONE / 2, ONE / 2, ONE / 2]);
        let event = compile_visible_radiance_interface_event(&input).unwrap();
        assert!(matches!(
            event.outcome,
            InterfaceEventOutcomeV1::Known { .. }
        ));
        let bytes = event.to_bytes(&input).unwrap();
        assert_eq!(
            VisibleRadianceInterfaceEventV1::from_bytes(&bytes, &input).unwrap(),
            event
        );
    }
    #[test]
    fn index_match_and_reverse_direction_are_known() {
        let mut input = fixture(smooth(1 << 48, 1 << 48), [3 * ONE / 2, ONE, ONE / 2]);
        assert!(matches!(
            compile_visible_radiance_interface_event(&input)
                .unwrap()
                .outcome,
            InterfaceEventOutcomeV1::Known { .. }
        ));
        let start = input.path_query.start_q32_32;
        input.path_query.start_q32_32 = input.path_query.end_q32_32;
        input.path_query.end_q32_32 = start;
        assert!(matches!(
            compile_visible_radiance_interface_event(&input)
                .unwrap()
                .outcome,
            InterfaceEventOutcomeV1::Known { .. }
        ));
    }
    #[test]
    fn missing_unsupported_and_same_medium_subdivision_fail_closed() {
        let unsupported = fixture(
            InterfaceModelV1::Unsupported {
                model_source_id: id(99),
            },
            [3 * ONE / 2, ONE / 2, ONE / 2],
        );
        assert_eq!(
            compile_visible_radiance_interface_event(&unsupported)
                .unwrap()
                .outcome,
            InterfaceEventOutcomeV1::UnsupportedInterfaceModel
        );
        let mut missing = fixture(smooth(1 << 48, 3 << 47), [3 * ONE / 2, ONE / 2, ONE / 2]);
        missing.face_interaction.medium_b = CellEvidenceV1::Solid {
            substance_source_id: id(11),
        };
        assert!(compile_visible_radiance_interface_event(&missing).is_err());
    }
    #[test]
    fn edge_lane_and_codec_poison_do_not_fabricate_an_event() {
        let input = fixture(
            smooth(1 << 48, 3 << 47),
            [3 * ONE / 2, 3 * ONE / 2, ONE / 2],
        );
        assert!(!matches!(
            compile_visible_radiance_interface_event(&input)
                .unwrap()
                .outcome,
            InterfaceEventOutcomeV1::Known { .. }
        ));
        let valid = fixture(smooth(1 << 48, 3 << 47), [3 * ONE / 2, ONE / 2, ONE / 2]);
        let mut bytes = valid.to_bytes().unwrap();
        bytes.push(b' ');
        assert!(VisibleRadianceInterfaceInputV1::from_bytes(&bytes).is_err());
    }
    #[test]
    fn public_claims_and_resources_remain_bounded() {
        let input = fixture(smooth(1 << 48, 3 << 47), [3 * ONE / 2, ONE, ONE / 2]);
        let event = compile_visible_radiance_interface_event(&input).unwrap();
        let InterfaceEventOutcomeV1::Known {
            arithmetic_receipt, ..
        } = event.outcome
        else {
            panic!("expected known")
        };
        assert!(arithmetic_receipt.attempted_fractional_bits.len() <= 3);
        assert!(arithmetic_receipt.fractional_bit_work_units <= 384);
        assert!(limitations().join(" ").contains("planet"));
    }

    #[test]
    fn retained_critical_and_coprime_wide_kernel_cases_certify_without_hidden_work() {
        let q48 = 1_u64 << 48;
        let cases = [
            (
                OrientedCase {
                    delta: [3, 4, 0],
                    axis: 0,
                    normal_sign: 1,
                },
                5 * q48,
                4 * q48,
            ),
            (
                OrientedCase {
                    delta: [3, 4, 0],
                    axis: 0,
                    normal_sign: 1,
                },
                5 * q48,
                4 * q48 - 1,
            ),
            (
                OrientedCase {
                    delta: [3, 4, 0],
                    axis: 0,
                    normal_sign: 1,
                },
                5 * q48,
                4 * q48 + 1,
            ),
            (
                OrientedCase {
                    delta: [1, (1_i128 << 64) - 100, 0],
                    axis: 0,
                    normal_sign: 1,
                },
                (1_u64 << 52) - 1,
                (1_u64 << 52) - 3,
            ),
            (
                OrientedCase {
                    delta: [1, (1_i128 << 64) - 100, 0],
                    axis: 0,
                    normal_sign: 1,
                },
                (1_u64 << 52) - 3,
                (1_u64 << 52) - 1,
            ),
        ];
        let mut saw_160 = false;
        for (case, eta_i, eta_t) in cases {
            let outcome = adaptive_outcome(&case, &[(eta_i, eta_t); 3]).unwrap();
            let InterfaceEventOutcomeV1::Known {
                arithmetic_receipt, ..
            } = outcome
            else {
                panic!("retained kernel case did not certify")
            };
            assert!(arithmetic_receipt.fractional_bit_work_units <= 384);
            saw_160 |= arithmetic_receipt.attempted_fractional_bits.contains(&160);
        }
        assert!(
            saw_160,
            "hostile portfolio no longer exercises the 160-bit level"
        );
    }

    #[test]
    fn normal_incidence_matches_the_exact_analytic_power_fraction() {
        let case = OrientedCase {
            delta: [7, 0, 0],
            axis: 0,
            normal_sign: 1,
        };
        let q48 = 1_u64 << 48;
        let outcome = adaptive_outcome(&case, &[(q48, 3 * q48 / 2); 3]).unwrap();
        let InterfaceEventOutcomeV1::Known {
            bands_rgb,
            arithmetic_receipt,
        } = outcome
        else {
            panic!()
        };
        assert!(arithmetic_receipt.attempted_fractional_bits.is_empty());
        let interval = &bands_rgb[0].reflected_power;
        let lower = interval.lower.parse::<i128>().unwrap();
        let upper = interval.upper.parse::<i128>().unwrap();
        let exact_numerator = 1_i128 << POWER_SCALE_BITS;
        assert!(lower * 25 <= exact_numerator && exact_numerator <= upper * 25);
        assert_eq!(upper - lower, 1);
        let direction_one = (1_i128 << DIRECTION_SCALE_BITS).to_string();
        let direction_minus_one = (-(1_i128 << DIRECTION_SCALE_BITS)).to_string();
        assert_eq!(
            bands_rgb[0].transmitted_direction_xyz.as_ref().unwrap()[0].lower,
            direction_one
        );
        assert_eq!(
            bands_rgb[0].reflected_direction_xyz[0].lower,
            direction_minus_one
        );
    }

    #[test]
    fn perfect_square_fast_paths_and_forced_cap_remain_distinct() {
        let q48 = 1_u64 << 48;
        let matched = OrientedCase {
            delta: [4, 3, 0],
            axis: 0,
            normal_sign: 1,
        };
        let InterfaceEventOutcomeV1::Known {
            arithmetic_receipt, ..
        } = adaptive_outcome(&matched, &[(7 * q48 / 5, 7 * q48 / 5); 3]).unwrap()
        else {
            panic!()
        };
        assert!(arithmetic_receipt.attempted_fractional_bits.is_empty());

        let critical = OrientedCase {
            delta: [3, 4, 0],
            axis: 0,
            normal_sign: 1,
        };
        let InterfaceEventOutcomeV1::Known {
            bands_rgb,
            arithmetic_receipt,
        } = adaptive_outcome(&critical, &[(5 * q48, 4 * q48); 3]).unwrap()
        else {
            panic!()
        };
        assert!(arithmetic_receipt.attempted_fractional_bits.is_empty());
        assert!(bands_rgb.iter().all(|band| band.total_internal_reflection));

        let hostile = OrientedCase {
            delta: [1, (1_i128 << 64) - 100, 0],
            axis: 0,
            normal_sign: 1,
        };
        let outcome = adaptive_outcome_with_precisions(
            &hostile,
            &[((1_u64 << 52) - 3, (1_u64 << 52) - 1); 3],
            &[96, 128],
        )
        .unwrap();
        let InterfaceEventOutcomeV1::NonconvergentEnclosure {
            arithmetic_receipt, ..
        } = outcome
        else {
            panic!()
        };
        assert_eq!(arithmetic_receipt.attempted_fractional_bits, vec![96, 128]);
        assert_eq!(arithmetic_receipt.fractional_bit_work_units, 224);
    }

    #[test]
    fn deterministic_generated_kernel_portfolio_preserves_all_postconditions() {
        let mut state = 0x49_4e_54_45_52_46_41_43_u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let eta_span = (1_u64 << 52) - (1_u64 << 46);
        let mut stops = [0_usize; 4];
        for index in 0..1024 {
            let axis = index % 3;
            let mut delta = [0_i128; 3];
            for component in &mut delta {
                *component = i128::from((next() & ((1 << 21) - 1)) as i32 - (1 << 20));
            }
            delta[axis] = i128::from((next() % (1 << 20) + 1) as u32);
            let eta_i = (1_u64 << 46) + next() % eta_span;
            let eta_t = (1_u64 << 46) + next() % eta_span;
            let outcome = adaptive_outcome(
                &OrientedCase {
                    delta,
                    axis,
                    normal_sign: 1,
                },
                &[(eta_i, eta_t); 3],
            )
            .unwrap();
            validate_outcome_codec(&outcome).unwrap();
            let InterfaceEventOutcomeV1::Known {
                arithmetic_receipt, ..
            } = outcome
            else {
                panic!("generated admitted case failed to certify")
            };
            let bucket = match arithmetic_receipt.attempted_fractional_bits.last().copied() {
                None => 0,
                Some(96) => 1,
                Some(128) => 2,
                Some(160) => 3,
                _ => panic!("unexpected precision"),
            };
            stops[bucket] += 1;
        }
        assert_eq!(stops.iter().sum::<usize>(), 1024);
        assert!(
            stops[1] > 0,
            "generated portfolio lost the ordinary 96-bit path"
        );
    }

    #[test]
    fn independent_python_exact_fixed_portfolio_checksum_matches() {
        struct Case(String, [i128; 3], usize, u64, u64);
        let q48 = 1_u64 << 48;
        let case =
            |name: &str, delta, axis, eta_i, eta_t| Case(name.into(), delta, axis, eta_i, eta_t);
        let mut cases = vec![
            case("normal", [7, 0, 0], 0, q48, 3 * q48 / 2),
            case("matched", [4, 3, 0], 0, 7 * q48 / 5, 7 * q48 / 5),
            case("critical", [3, 4, 0], 0, 5 * q48, 4 * q48),
            case("above-critical", [3, 4, 0], 0, 5 * q48, 4 * q48 - 1),
            case("below-critical", [3, 4, 0], 0, 5 * q48, 4 * q48 + 1),
            case("grazing-transmit", [1, 1_i128 << 40, 3], 0, q48, 16 * q48),
            case("reverse-normal", [9, 0, 0], 0, 3 * q48 / 2, q48),
            case("dispersive-red", [13, 8, -5], 0, 3 * q48 / 2, 4 * q48 / 3),
            case("dispersive-green", [13, 8, -5], 0, 3 * q48 / 2, 7 * q48 / 5),
            case("dispersive-blue", [13, 8, -5], 0, 3 * q48 / 2, 8 * q48 / 5),
        ];
        for axis in 0..3 {
            let mut delta = [0; 3];
            delta[axis] = 3;
            delta[(axis + 1) % 3] = 4;
            cases.push(case(
                &format!("critical-axis-{axis}"),
                delta,
                axis,
                5 * q48,
                4 * q48,
            ));
            cases.push(case(
                &format!("critical-axis-{axis}-below"),
                delta,
                axis,
                5 * q48,
                4 * q48 + 1,
            ));
            cases.push(case(
                &format!("critical-axis-{axis}-above"),
                delta,
                axis,
                5 * q48,
                4 * q48 - 1,
            ));
        }
        let hostile = (1_i128 << 64) - 100;
        cases.extend([
            case(
                "coprime-wide-tir",
                [1, hostile, 0],
                0,
                (1 << 52) - 1,
                (1 << 52) - 3,
            ),
            case(
                "coprime-wide-transmit",
                [1, hostile, 0],
                0,
                (1 << 52) - 3,
                (1 << 52) - 1,
            ),
            case("normal-neighbor", [7, 1, 0], 0, q48, 3 * q48 / 2),
            case(
                "matched-neighbor",
                [4, 3, 0],
                0,
                7 * q48 / 5,
                7 * q48 / 5 + 1,
            ),
            case(
                "perfect-square-neighbor",
                [4, 3, 1],
                0,
                7 * q48 / 5,
                7 * q48 / 5,
            ),
            case("tir-perfect-neighbor", [4, 3, 0], 0, 5 * q48, 4 * q48 + 1),
        ]);
        let pair = |value: &DecimalIntervalV1| {
            serde_json::json!([
                value.lower.parse::<i128>().unwrap(),
                value.upper.parse::<i128>().unwrap()
            ])
        };
        let mut rows = Vec::new();
        for Case(name, delta, axis, eta_i, eta_t) in cases {
            let outcome = adaptive_outcome(
                &OrientedCase {
                    delta,
                    axis,
                    normal_sign: 1,
                },
                &[(eta_i, eta_t); 3],
            )
            .unwrap();
            let InterfaceEventOutcomeV1::Known { bands_rgb, .. } = outcome else {
                panic!()
            };
            let band = &bands_rgb[0];
            let mut row = vec![
                serde_json::json!(name),
                serde_json::json!(band.total_internal_reflection),
                pair(&band.reflected_power),
                pair(&band.transmitted_power),
            ];
            row.extend(band.reflected_direction_xyz.iter().map(&pair));
            if let Some(transmitted) = &band.transmitted_direction_xyz {
                row.extend(transmitted.iter().map(&pair));
            }
            rows.push(serde_json::Value::Array(row));
        }
        let digest = Sha256::digest(serde_json::to_vec(&rows).unwrap());
        assert_eq!(
            format!("{digest:x}"),
            "3e595f04af1d9cb560dfe0dc684ca7ac0eec6597b15aa449cf0bc984b3cf2593"
        );
    }
}
