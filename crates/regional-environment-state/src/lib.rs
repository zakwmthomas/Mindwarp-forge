//! Exact deterministic regional exposure evidence over `field-basis`.
//! This is not terrain, weather, radiative transfer, visibility, or simulation.

use field_basis::{FieldError, FieldRecipe, ONE, recipe_fingerprint, sample};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spatial_domain::{CellIndex, SpatialDomain, SpatialDomainError, build_spatial_cell};

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegionalEnvironmentError {
    Invalid(&'static str),
    Codec(String),
    Field(FieldError),
    Spatial(SpatialDomainError),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalEnvironmentInput {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub regional_source_id: [u8; 32],
    pub field_recipe_bytes: Vec<u8>,
    pub moisture_source_id: [u8; 32],
    pub moisture_field_recipe_bytes: Vec<u8>,
    pub coordinate_q32_32: [i64; 2],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalFieldBindingV1 {
    pub schema_version: u16,
    pub reconstruction_id: [u8; 32],
    pub regional_source_id: [u8; 32],
    pub field_recipe_bytes: Vec<u8>,
    pub moisture_source_id: [u8; 32],
    pub moisture_field_recipe_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalEnvironmentContent {
    pub schema_version: u16,
    pub input_id: String,
    pub field_recipe_id: String,
    pub moisture_field_recipe_id: String,
    pub coordinate_q32_32: [i64; 2],
    pub exposure_permille: u16,
    pub moisture_potential_permille: u16,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalEnvironmentState {
    pub state_id: String,
    pub content: RegionalEnvironmentContent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalEnvironmentContract {
    pub input: RegionalEnvironmentInput,
    pub state: RegionalEnvironmentState,
}

impl RegionalEnvironmentInput {
    pub fn from_field_binding(
        binding: &RegionalFieldBindingV1,
        coordinate_q32_32: [i64; 2],
    ) -> Result<Self, RegionalEnvironmentError> {
        validate_field_binding(binding)?;
        Ok(Self {
            schema_version: CONTRACT_VERSION,
            reconstruction_id: binding.reconstruction_id,
            regional_source_id: binding.regional_source_id,
            field_recipe_bytes: binding.field_recipe_bytes.clone(),
            moisture_source_id: binding.moisture_source_id,
            moisture_field_recipe_bytes: binding.moisture_field_recipe_bytes.clone(),
            coordinate_q32_32,
        })
    }

    pub fn field_binding(&self) -> Result<RegionalFieldBindingV1, RegionalEnvironmentError> {
        validate_input(self)?;
        Ok(RegionalFieldBindingV1 {
            schema_version: CONTRACT_VERSION,
            reconstruction_id: self.reconstruction_id,
            regional_source_id: self.regional_source_id,
            field_recipe_bytes: self.field_recipe_bytes.clone(),
            moisture_source_id: self.moisture_source_id,
            moisture_field_recipe_bytes: self.moisture_field_recipe_bytes.clone(),
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, RegionalEnvironmentError> {
        validate_input(self)?;
        serde_json::to_vec(self).map_err(|e| RegionalEnvironmentError::Codec(e.to_string()))
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegionalEnvironmentError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|e| RegionalEnvironmentError::Codec(e.to_string()))?;
        validate_input(&value)?;
        if value.to_bytes()? != bytes {
            return Err(RegionalEnvironmentError::Invalid(
                "noncanonical input bytes",
            ));
        }
        Ok(value)
    }
}

impl RegionalFieldBindingV1 {
    pub fn to_bytes(&self) -> Result<Vec<u8>, RegionalEnvironmentError> {
        validate_field_binding(self)?;
        serde_json::to_vec(self).map_err(|error| RegionalEnvironmentError::Codec(error.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegionalEnvironmentError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| RegionalEnvironmentError::Codec(error.to_string()))?;
        validate_field_binding(&value)?;
        if value.to_bytes()? != bytes {
            return Err(RegionalEnvironmentError::Invalid(
                "noncanonical regional field-binding bytes",
            ));
        }
        Ok(value)
    }
}

impl RegionalEnvironmentState {
    pub fn to_bytes(&self) -> Result<Vec<u8>, RegionalEnvironmentError> {
        validate_content(&self.content)?;
        if self.state_id != state_id(&self.content)? {
            return Err(RegionalEnvironmentError::Invalid("state identity drift"));
        }
        serde_json::to_vec(self).map_err(|e| RegionalEnvironmentError::Codec(e.to_string()))
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegionalEnvironmentError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|e| RegionalEnvironmentError::Codec(e.to_string()))?;
        validate_content(&value.content)?;
        if value.to_bytes()? != bytes {
            return Err(RegionalEnvironmentError::Invalid(
                "noncanonical state bytes",
            ));
        }
        Ok(value)
    }
}

pub fn compile_regional_environment(
    input: &RegionalEnvironmentInput,
) -> Result<RegionalEnvironmentContract, RegionalEnvironmentError> {
    let input_bytes = input.to_bytes()?;
    let recipe = FieldRecipe::decode_strict(&input.field_recipe_bytes)
        .map_err(RegionalEnvironmentError::Field)?;
    let moisture_recipe = FieldRecipe::decode_strict(&input.moisture_field_recipe_bytes)
        .map_err(RegionalEnvironmentError::Field)?;
    let raw = sample(
        &recipe,
        input.reconstruction_id,
        input.coordinate_q32_32[0],
        input.coordinate_q32_32[1],
    )
    .map_err(RegionalEnvironmentError::Field)?;
    if !(-ONE..=ONE).contains(&raw) {
        return Err(RegionalEnvironmentError::Invalid(
            "regional exposure sample outside normalized range",
        ));
    }
    let moisture_raw = sample(
        &moisture_recipe,
        moisture_stream_key(input),
        input.coordinate_q32_32[0],
        input.coordinate_q32_32[1],
    )
    .map_err(RegionalEnvironmentError::Field)?;
    if !(-ONE..=ONE).contains(&moisture_raw) {
        return Err(RegionalEnvironmentError::Invalid(
            "regional moisture sample outside normalized range",
        ));
    }
    let denominator = 2 * i128::from(ONE);
    let exposure = ((i128::from(raw) + i128::from(ONE)) * 1_000 + denominator / 2) / denominator;
    let moisture =
        ((i128::from(moisture_raw) + i128::from(ONE)) * 1_000 + denominator / 2) / denominator;
    let content = RegionalEnvironmentContent {
        schema_version: CONTRACT_VERSION,
        input_id: hex(&hash(
            b"mindwarp.regional-environment.input.v1\0",
            &input_bytes,
        )),
        field_recipe_id: hex(&recipe_fingerprint(&recipe).map_err(RegionalEnvironmentError::Field)?),
        moisture_field_recipe_id: hex(
            &recipe_fingerprint(&moisture_recipe).map_err(RegionalEnvironmentError::Field)?
        ),
        coordinate_q32_32: input.coordinate_q32_32,
        exposure_permille: u16::try_from(exposure)
            .map_err(|_| RegionalEnvironmentError::Invalid("regional exposure overflow"))?,
        moisture_potential_permille: u16::try_from(moisture)
            .map_err(|_| RegionalEnvironmentError::Invalid("regional moisture overflow"))?,
        limitations: limitations(),
        authority_effect: "none_evidence_only".into(),
    };
    let state = RegionalEnvironmentState {
        state_id: state_id(&content)?,
        content,
    };
    Ok(RegionalEnvironmentContract {
        input: input.clone(),
        state,
    })
}

pub fn compile_regional_environment_for_cell(
    input: &RegionalEnvironmentInput,
    domain: &SpatialDomain,
    index: CellIndex,
) -> Result<RegionalEnvironmentContract, RegionalEnvironmentError> {
    if input.reconstruction_id != domain.input.reconstruction_id {
        return Err(RegionalEnvironmentError::Invalid(
            "regional and spatial reconstruction mismatch",
        ));
    }
    let cell = build_spatial_cell(domain, index).map_err(RegionalEnvironmentError::Spatial)?;
    let mut bound_input = input.clone();
    bound_input.coordinate_q32_32 = cell.sample_coordinate_q32_32;
    compile_regional_environment(&bound_input)
}

pub fn compile_regional_environment_for_binding_cell(
    binding: &RegionalFieldBindingV1,
    domain: &SpatialDomain,
    index: CellIndex,
) -> Result<RegionalEnvironmentContract, RegionalEnvironmentError> {
    validate_field_binding(binding)?;
    if binding.reconstruction_id != domain.input.reconstruction_id {
        return Err(RegionalEnvironmentError::Invalid(
            "regional binding and spatial reconstruction mismatch",
        ));
    }
    let cell = build_spatial_cell(domain, index).map_err(RegionalEnvironmentError::Spatial)?;
    let input =
        RegionalEnvironmentInput::from_field_binding(binding, cell.sample_coordinate_q32_32)?;
    compile_regional_environment(&input)
}

pub fn validate_regional_environment(
    contract: &RegionalEnvironmentContract,
) -> Result<(), RegionalEnvironmentError> {
    if &compile_regional_environment(&contract.input)? != contract {
        return Err(RegionalEnvironmentError::Invalid(
            "regional environment contract drift",
        ));
    }
    Ok(())
}

fn validate_input(input: &RegionalEnvironmentInput) -> Result<(), RegionalEnvironmentError> {
    let binding = RegionalFieldBindingV1 {
        schema_version: input.schema_version,
        reconstruction_id: input.reconstruction_id,
        regional_source_id: input.regional_source_id,
        field_recipe_bytes: input.field_recipe_bytes.clone(),
        moisture_source_id: input.moisture_source_id,
        moisture_field_recipe_bytes: input.moisture_field_recipe_bytes.clone(),
    };
    validate_field_binding(&binding)
}

fn validate_field_binding(
    binding: &RegionalFieldBindingV1,
) -> Result<(), RegionalEnvironmentError> {
    if binding.schema_version != CONTRACT_VERSION {
        return Err(RegionalEnvironmentError::Invalid(
            "unsupported input schema",
        ));
    }
    if binding.reconstruction_id == [0; 32]
        || binding.regional_source_id == [0; 32]
        || binding.moisture_source_id == [0; 32]
    {
        return Err(RegionalEnvironmentError::Invalid(
            "missing identity binding",
        ));
    }
    let recipe = FieldRecipe::decode_strict(&binding.field_recipe_bytes)
        .map_err(RegionalEnvironmentError::Field)?;
    if recipe
        .encode_canonical()
        .map_err(RegionalEnvironmentError::Field)?
        != binding.field_recipe_bytes
    {
        return Err(RegionalEnvironmentError::Invalid(
            "noncanonical field recipe",
        ));
    }
    let moisture_recipe = FieldRecipe::decode_strict(&binding.moisture_field_recipe_bytes)
        .map_err(RegionalEnvironmentError::Field)?;
    if moisture_recipe
        .encode_canonical()
        .map_err(RegionalEnvironmentError::Field)?
        != binding.moisture_field_recipe_bytes
    {
        return Err(RegionalEnvironmentError::Invalid(
            "noncanonical moisture field recipe",
        ));
    }
    Ok(())
}

fn validate_content(content: &RegionalEnvironmentContent) -> Result<(), RegionalEnvironmentError> {
    if content.schema_version != CONTRACT_VERSION {
        return Err(RegionalEnvironmentError::Invalid(
            "unsupported state schema",
        ));
    }
    validate_hex(&content.input_id, "malformed input identity")?;
    validate_hex(&content.field_recipe_id, "malformed recipe identity")?;
    validate_hex(
        &content.moisture_field_recipe_id,
        "malformed moisture recipe identity",
    )?;
    if content.exposure_permille > 1_000 || content.moisture_potential_permille > 1_000 {
        return Err(RegionalEnvironmentError::Invalid("exposure range"));
    }
    if content.limitations != limitations() || content.authority_effect != "none_evidence_only" {
        return Err(RegionalEnvironmentError::Invalid(
            "claim or authority drift",
        ));
    }
    Ok(())
}

fn limitations() -> Vec<String> {
    [
        "normalized procedural exposure only; not measured terrain slope aspect shadow or clouds",
        "normalized regional moisture potential only; not rainfall humidity soil groundwater or surface coverage",
        "no weather radiative-transfer visibility-distance biome traversability habitability or runtime claim",
    ]
    .map(String::from)
    .to_vec()
}

fn moisture_stream_key(input: &RegionalEnvironmentInput) -> [u8; 32] {
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&input.reconstruction_id);
    bytes.extend_from_slice(&input.moisture_source_id);
    hash(b"mindwarp.regional-moisture.stream.v1\0", &bytes)
}

fn state_id(content: &RegionalEnvironmentContent) -> Result<String, RegionalEnvironmentError> {
    let bytes =
        serde_json::to_vec(content).map_err(|e| RegionalEnvironmentError::Codec(e.to_string()))?;
    Ok(hex(&hash(
        b"mindwarp.regional-environment.state.v1\0",
        &bytes,
    )))
}

fn validate_hex(value: &str, error: &'static str) -> Result<(), RegionalEnvironmentError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(RegionalEnvironmentError::Invalid(error));
    }
    Ok(())
}

fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use field_basis::Term;

    fn recipe(term: Term) -> Vec<u8> {
        FieldRecipe::new(vec![term], 0)
            .unwrap()
            .encode_canonical()
            .unwrap()
    }
    fn input(bytes: Vec<u8>) -> RegionalEnvironmentInput {
        RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            regional_source_id: [2; 32],
            field_recipe_bytes: bytes,
            moisture_source_id: [3; 32],
            moisture_field_recipe_bytes: recipe(Term::Constant(0)),
            coordinate_q32_32: [0, 0],
        }
    }

    #[test]
    fn deterministic_strict_replay_and_midpoint_normalization() {
        let input = input(recipe(Term::Constant(0)));
        let contract = compile_regional_environment(&input).unwrap();
        assert_eq!(contract.state.content.exposure_permille, 500);
        assert_eq!(contract.state.content.moisture_potential_permille, 500);
        assert_eq!(compile_regional_environment(&input).unwrap(), contract);
        assert_eq!(
            RegionalEnvironmentInput::from_bytes(&input.to_bytes().unwrap()).unwrap(),
            input
        );
        assert_eq!(
            RegionalEnvironmentState::from_bytes(&contract.state.to_bytes().unwrap()).unwrap(),
            contract.state
        );
        assert!(validate_regional_environment(&contract).is_ok());
    }

    #[test]
    fn normalized_endpoints_map_exactly() {
        assert_eq!(
            compile_regional_environment(&input(recipe(Term::Constant(-ONE))))
                .unwrap()
                .state
                .content
                .exposure_permille,
            0
        );
        assert_eq!(
            compile_regional_environment(&input(recipe(Term::Constant(ONE))))
                .unwrap()
                .state
                .content
                .exposure_permille,
            1_000
        );
    }

    #[test]
    fn coordinates_cause_deterministic_regional_variation() {
        let bytes = recipe(Term::ValueLattice2 {
            frequency: 1,
            amplitude: ONE,
            component: 7,
        });
        let first = input(bytes.clone());
        let mut second = input(bytes);
        second.coordinate_q32_32 = [1_i64 << field_basis::COORD_FRAC, 0];
        let a = compile_regional_environment(&first).unwrap();
        let b = compile_regional_environment(&second).unwrap();
        assert_ne!(
            a.state.content.exposure_permille,
            b.state.content.exposure_permille
        );
        assert_ne!(a.state.state_id, b.state.state_id);
    }

    #[test]
    fn spatial_cell_reconstructs_exact_regional_coordinate() {
        let base = input(recipe(Term::Constant(0)));
        let spatial = spatial_domain::compile_spatial_domain(&spatial_domain::SpatialDomainInput {
            schema_version: 1,
            logical_world_id: [9; 32],
            reconstruction_id: base.reconstruction_id,
            coordinate_frame: spatial_domain::CoordinateFrame::FieldQ32_32Cartesian2dV1,
            cell_center_origin_q32_32: [-(1_i64 << field_basis::COORD_FRAC), 0],
            cell_step_q32_32: [1_u64 << field_basis::COORD_FRAC; 2],
            cell_count: [3, 2],
            adjacency: spatial_domain::Adjacency::SharedEdge4,
            boundary_mode: spatial_domain::BoundaryMode::BoundedAbsent,
        })
        .unwrap();
        let contract = compile_regional_environment_for_cell(
            &base,
            &spatial,
            spatial_domain::CellIndex::new(2, 1),
        )
        .unwrap();
        assert_eq!(
            contract.input.coordinate_q32_32,
            [
                1_i64 << field_basis::COORD_FRAC,
                1_i64 << field_basis::COORD_FRAC
            ]
        );
        assert!(validate_regional_environment(&contract).is_ok());

        let mut foreign = base;
        foreign.reconstruction_id = [8; 32];
        assert!(matches!(
            compile_regional_environment_for_cell(
                &foreign,
                &spatial,
                spatial_domain::CellIndex::new(0, 0)
            ),
            Err(RegionalEnvironmentError::Invalid(
                "regional and spatial reconstruction mismatch"
            ))
        ));
    }

    #[test]
    fn coordinate_free_binding_is_strict_and_preserves_point_semantics() {
        let base = input(recipe(Term::Constant(0)));
        let binding = base.field_binding().unwrap();
        assert_eq!(
            RegionalFieldBindingV1::from_bytes(&binding.to_bytes().unwrap()).unwrap(),
            binding
        );
        let rebuilt =
            RegionalEnvironmentInput::from_field_binding(&binding, base.coordinate_q32_32).unwrap();
        assert_eq!(rebuilt, base);
        assert_eq!(
            compile_regional_environment(&rebuilt).unwrap(),
            compile_regional_environment(&base).unwrap()
        );

        let spatial = spatial_domain::compile_spatial_domain(&spatial_domain::SpatialDomainInput {
            schema_version: 1,
            logical_world_id: [9; 32],
            reconstruction_id: binding.reconstruction_id,
            coordinate_frame: spatial_domain::CoordinateFrame::FieldQ32_32Cartesian2dV1,
            cell_center_origin_q32_32: [0, 0],
            cell_step_q32_32: [1_u64 << field_basis::COORD_FRAC; 2],
            cell_count: [2, 1],
            adjacency: spatial_domain::Adjacency::SharedEdge4,
            boundary_mode: spatial_domain::BoundaryMode::BoundedAbsent,
        })
        .unwrap();
        let cell = compile_regional_environment_for_binding_cell(
            &binding,
            &spatial,
            spatial_domain::CellIndex::new(1, 0),
        )
        .unwrap();
        assert_eq!(
            cell.input.coordinate_q32_32,
            [1_i64 << field_basis::COORD_FRAC, 0]
        );

        let mut unknown = binding.to_bytes().unwrap();
        unknown.pop();
        unknown.extend_from_slice(br#",\"extra\":1}"#);
        assert!(RegionalFieldBindingV1::from_bytes(&unknown).is_err());
    }

    #[test]
    fn moisture_source_changes_only_moisture_potential() {
        let mut first = input(recipe(Term::Constant(0)));
        first.moisture_field_recipe_bytes = recipe(Term::ValueLattice2 {
            frequency: 1,
            amplitude: ONE,
            component: 19,
        });
        let mut second = first.clone();
        second.moisture_source_id = [4; 32];
        let a = compile_regional_environment(&first).unwrap();
        let b = compile_regional_environment(&second).unwrap();
        assert_eq!(
            a.state.content.exposure_permille,
            b.state.content.exposure_permille
        );
        assert_ne!(
            a.state.content.moisture_potential_permille,
            b.state.content.moisture_potential_permille
        );
    }

    #[test]
    fn out_of_normalized_range_is_rejected() {
        assert!(matches!(
            compile_regional_environment(&input(recipe(Term::Constant(ONE + 1)))),
            Err(RegionalEnvironmentError::Invalid(
                "regional exposure sample outside normalized range"
            ))
        ));
    }

    #[test]
    fn missing_id_and_invalid_recipe_fail_closed() {
        let mut missing = input(recipe(Term::Constant(0)));
        missing.reconstruction_id = [0; 32];
        assert!(compile_regional_environment(&missing).is_err());
        assert!(matches!(
            compile_regional_environment(&input(vec![0xff])),
            Err(RegionalEnvironmentError::Field(_))
        ));
    }

    #[test]
    fn unknown_and_noncanonical_bytes_are_rejected() {
        let input = input(recipe(Term::Constant(0)));
        let mut value: serde_json::Value =
            serde_json::from_slice(&input.to_bytes().unwrap()).unwrap();
        value["unknown"] = serde_json::json!(true);
        assert!(
            RegionalEnvironmentInput::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
        );
        let mut spaced = input.to_bytes().unwrap();
        spaced.push(b' ');
        assert!(matches!(
            RegionalEnvironmentInput::from_bytes(&spaced),
            Err(RegionalEnvironmentError::Invalid(
                "noncanonical input bytes"
            ))
        ));
    }

    #[test]
    fn fabricated_state_and_claim_drift_are_rejected() {
        let input = input(recipe(Term::Constant(0)));
        let mut contract = compile_regional_environment(&input).unwrap();
        contract.state.content.exposure_permille += 1;
        assert!(validate_regional_environment(&contract).is_err());
        let mut claim_drift = compile_regional_environment(&input).unwrap();
        claim_drift.state.content.limitations.clear();
        assert!(claim_drift.state.to_bytes().is_err());
    }
}
