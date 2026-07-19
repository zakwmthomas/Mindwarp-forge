//! Capability-free G1-C6 macro-lineage identity and occupancy seam.
//!
//! This is the first bounded layer after environmental opportunities. It
//! binds one hypothetical native macro-lineage to:
//!
//! - one exact world packet;
//! - one exact environmental-opportunity graph;
//! - an explicit subset of opportunities the candidate may occupy;
//! - an explicit opaque body-plan reference;
//! - a stable lineage seed and optional parent lineage.
//!
//! The module does not invent anatomy. In particular it contains no head,
//! torso, limb, organ, sex, species, ecomorph, person-form, asset, or visual
//! fields. `body_plan_ref` remains an opaque schema-compatible identity seam;
//! the additive validator can now bind it to an exact validated body-plan
//! family without copying anatomy or an expression into this record.
//!
//! Occupancy is a hypothesis, not biological viability. Environmental
//! opportunity is necessary context only; it does not prove that a lineage
//! evolved or can exploit that opportunity.

use body_plan_structure::{BodyPlanFamily, ValidationReport, validate_body_plan_ref};
use derived_world_rules::{CausalWorldPacket, WorldGenerationInput};
use niche_graph_binding::{
    EnvironmentalOpportunityGraph, validate_environmental_opportunity_graph,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const CONTRACT_VERSION: u16 = 1;
pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateClass {
    Hypothesis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyRegionModelStatus {
    Deferred,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MacroLineageCandidate {
    pub schema_version: u16,
    pub lineage_id: Id,
    pub world_packet_id: String,
    pub opportunity_graph_ref: Id,
    pub physical_regime_ref: Id,
    pub lineage_seed: Id,
    pub parent_lineage_id: Option<Id>,
    pub body_plan_ref: Id,
    pub body_region_model_status: BodyRegionModelStatus,
    pub occupied_opportunity_ids: Vec<Id>,
    pub candidate_class: CandidateClass,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineageError {
    Invalid(&'static str),
    Codec(String),
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update([0]);
    hasher.update(bytes);
    hasher.finalize().into()
}

fn is_zero(id: Id) -> bool {
    id == [0; 32]
}

fn lineage_id(
    packet_id: &str,
    opportunity_graph_ref: Id,
    lineage_seed: Id,
    parent_lineage_id: Option<Id>,
    body_plan_ref: Id,
    occupied_opportunity_ids: &[Id],
) -> Id {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(packet_id.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&opportunity_graph_ref);
    bytes.extend_from_slice(&lineage_seed);
    bytes.extend_from_slice(&parent_lineage_id.unwrap_or([0; 32]));
    bytes.extend_from_slice(&body_plan_ref);
    for opportunity_id in occupied_opportunity_ids {
        bytes.extend_from_slice(opportunity_id);
    }
    hash(b"mindwarp.macro-lineage-candidate.v1", &bytes)
}

impl MacroLineageCandidate {
    pub fn to_bytes(&self) -> Result<Vec<u8>, LineageError> {
        serde_json::to_vec(self).map_err(|error| LineageError::Codec(error.to_string()))
    }

    pub fn fingerprint(&self) -> Result<Id, LineageError> {
        Ok(hash(
            b"mindwarp.macro-lineage-candidate-record.v1",
            &self.to_bytes()?,
        ))
    }
}

pub fn build_macro_lineage_candidate(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    opportunity_graph: &EnvironmentalOpportunityGraph,
    lineage_seed: Id,
    parent_lineage_id: Option<Id>,
    body_plan_ref: Id,
    occupied_opportunity_ids: Vec<Id>,
) -> Result<MacroLineageCandidate, LineageError> {
    validate_environmental_opportunity_graph(input, packet, opportunity_graph)
        .map_err(|_| LineageError::Invalid("invalid opportunity graph"))?;
    if opportunity_graph.world_packet_id != packet.packet_id {
        return Err(LineageError::Invalid(
            "world and opportunity graph mismatch",
        ));
    }
    if is_zero(lineage_seed) {
        return Err(LineageError::Invalid("zero lineage seed"));
    }
    if is_zero(body_plan_ref) {
        return Err(LineageError::Invalid("zero body plan reference"));
    }
    if parent_lineage_id.is_some_and(is_zero) {
        return Err(LineageError::Invalid("zero parent lineage identity"));
    }

    let available: BTreeSet<_> = opportunity_graph.nodes.iter().map(|node| node.id).collect();
    let occupied: BTreeSet<_> = occupied_opportunity_ids.into_iter().collect();
    if occupied.is_empty() {
        return Err(LineageError::Invalid("lineage occupies no opportunity"));
    }
    if !occupied.is_subset(&available) {
        return Err(LineageError::Invalid("unknown occupied opportunity"));
    }
    let occupied_opportunity_ids: Vec<_> = occupied.into_iter().collect();
    let opportunity_graph_ref = opportunity_graph
        .fingerprint()
        .map_err(|_| LineageError::Invalid("opportunity graph fingerprint failed"))?;
    let lineage_id = lineage_id(
        &packet.packet_id,
        opportunity_graph_ref,
        lineage_seed,
        parent_lineage_id,
        body_plan_ref,
        &occupied_opportunity_ids,
    );
    if parent_lineage_id == Some(lineage_id) {
        return Err(LineageError::Invalid("lineage cannot parent itself"));
    }

    Ok(MacroLineageCandidate {
        schema_version: CONTRACT_VERSION,
        lineage_id,
        world_packet_id: packet.packet_id.clone(),
        opportunity_graph_ref,
        physical_regime_ref: opportunity_graph.physical_regime_id,
        lineage_seed,
        parent_lineage_id,
        body_plan_ref,
        body_region_model_status: BodyRegionModelStatus::Deferred,
        occupied_opportunity_ids,
        candidate_class: CandidateClass::Hypothesis,
        limitations: vec![
            "opportunity occupancy is a hypothesis, not biological viability".into(),
            "body-plan content and body-region placement are intentionally deferred".into(),
            "no species, ecomorph, sensory organ, person-form or visual claim".into(),
        ],
    })
}

pub fn validate_macro_lineage_candidate(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    opportunity_graph: &EnvironmentalOpportunityGraph,
    candidate: &MacroLineageCandidate,
) -> Result<(), LineageError> {
    if candidate.schema_version != CONTRACT_VERSION {
        return Err(LineageError::Invalid("unsupported schema version"));
    }
    let rebuilt = build_macro_lineage_candidate(
        input,
        packet,
        opportunity_graph,
        candidate.lineage_seed,
        candidate.parent_lineage_id,
        candidate.body_plan_ref,
        candidate.occupied_opportunity_ids.clone(),
    )?;
    if &rebuilt != candidate {
        return Err(LineageError::Invalid(
            "candidate does not match causal inputs",
        ));
    }
    Ok(())
}

/// Validates the existing opaque reference against one complete body-plan
/// family without changing or enriching the lineage record.
pub fn validate_body_plan_binding(
    candidate: &MacroLineageCandidate,
    family: &BodyPlanFamily,
    validation_budget: u32,
) -> ValidationReport {
    validate_body_plan_ref(candidate.body_plan_ref, family, validation_budget)
}

#[cfg(test)]
mod tests {
    use super::*;
    use derived_world_rules::{
        ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
        HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract,
        RegionalEnvironmentInput, SignalChannel, SignalPotential, StellarOrbitalContract,
        StellarOrbitalInput, WorldGenerationInput, compile_climate, compile_geological_atmospheric,
        compile_hydrological, compile_regional_environment, compile_stellar_orbital, compile_world,
    };
    use field_basis::{FieldRecipe, ONE, Term};
    use niche_graph_binding::build_environmental_opportunity_graph;

    fn regional_contract(reconstruction_id: [u8; 32]) -> RegionalEnvironmentContract {
        compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id,
            regional_source_id: [8; 32],
            field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(ONE)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            moisture_source_id: [9; 32],
            moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            coordinate_q32_32: [0, 0],
        })
        .unwrap()
    }

    fn stellar_contract(reconstruction_id: [u8; 32]) -> StellarOrbitalContract {
        compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn geological_contract(reconstruction_id: [u8; 32]) -> GeologicalAtmosphericContract {
        compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: 1,
            reconstruction_id,
            planetary_body_id: [4; 32],
            stellar_orbital: stellar_contract(reconstruction_id),
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000; 3],
        })
        .unwrap()
    }

    fn hydrological_contract(reconstruction_id: [u8; 32]) -> HydrologicalContract {
        compile_hydrological(&HydrologicalInput {
            schema_version: 1,
            reconstruction_id,
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract(reconstruction_id),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap()
    }

    fn climate_contract(reconstruction_id: [u8; 32]) -> ClimateContract {
        compile_climate(&ClimateInput {
            schema_version: 1,
            reconstruction_id,
            climate_source_id: [6; 32],
            hydrological: hydrological_contract(reconstruction_id),
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }
    fn surface_material_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::SurfaceMaterialContract {
        derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id,
            material_source_id: [7; 32],
            climate: climate_contract(reconstruction_id),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap()
    }

    fn packet(reconstruction_byte: u8) -> (WorldGenerationInput, CausalWorldPacket) {
        let input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [reconstruction_byte; 32],
            surface_material: surface_material_contract([reconstruction_byte; 32]),
            regional_environment: regional_contract([reconstruction_byte; 32]),
            signal_potentials: vec![
                SignalPotential {
                    channel: SignalChannel::VisibleRadiance,
                    baseline_strength_permille: 900,
                },
                SignalPotential {
                    channel: SignalChannel::ChemicalGradient,
                    baseline_strength_permille: 800,
                },
            ],
        };
        let packet = compile_world(&input).unwrap();
        (input, packet)
    }

    fn candidate() -> (
        WorldGenerationInput,
        CausalWorldPacket,
        EnvironmentalOpportunityGraph,
        MacroLineageCandidate,
    ) {
        let (input, packet) = packet(1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let candidate = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            [4; 32],
            vec![graph.nodes[0].id],
        )
        .unwrap();
        (input, packet, graph, candidate)
    }

    #[test]
    fn candidate_binds_exact_world_graph_body_plan_and_occupancy() {
        let (input, packet, graph, candidate) = candidate();
        assert_eq!(candidate.world_packet_id, packet.packet_id);
        assert_eq!(
            candidate.opportunity_graph_ref,
            graph.fingerprint().unwrap()
        );
        assert_eq!(candidate.physical_regime_ref, graph.physical_regime_id);
        assert_eq!(
            candidate.body_region_model_status,
            BodyRegionModelStatus::Deferred
        );
        assert!(validate_macro_lineage_candidate(&input, &packet, &graph, &candidate).is_ok());
    }

    #[test]
    fn physical_regime_identity_ignores_provenance_but_not_physical_change() {
        let (first_input, first_packet) = packet(1);
        let (second_input, second_packet) = packet(9);
        let first_graph =
            build_environmental_opportunity_graph(&first_input, &first_packet).unwrap();
        let second_graph =
            build_environmental_opportunity_graph(&second_input, &second_packet).unwrap();
        assert_ne!(
            first_graph.fingerprint().unwrap(),
            second_graph.fingerprint().unwrap()
        );
        assert_eq!(
            first_graph.physical_regime_id,
            second_graph.physical_regime_id
        );

        let mut changed_input = first_input.clone();
        let mut regional_input = changed_input.regional_environment.input.clone();
        regional_input.moisture_field_recipe_bytes = FieldRecipe::new(vec![Term::Constant(ONE)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap();
        changed_input.regional_environment = compile_regional_environment(&regional_input).unwrap();
        let changed_packet = compile_world(&changed_input).unwrap();
        let changed_graph =
            build_environmental_opportunity_graph(&changed_input, &changed_packet).unwrap();
        assert_ne!(
            first_graph.physical_regime_id,
            changed_graph.physical_regime_id
        );
    }

    #[test]
    fn foreign_world_graph_is_rejected() {
        let (local_input, local_packet) = packet(1);
        let (foreign_input, foreign_packet) = packet(9);
        let foreign_graph =
            build_environmental_opportunity_graph(&foreign_input, &foreign_packet).unwrap();
        assert_eq!(
            build_macro_lineage_candidate(
                &local_input,
                &local_packet,
                &foreign_graph,
                [3; 32],
                None,
                [4; 32],
                vec![foreign_graph.nodes[0].id],
            ),
            Err(LineageError::Invalid("invalid opportunity graph"))
        );
    }

    #[test]
    fn unknown_or_empty_occupancy_fails_closed() {
        let (input, packet) = packet(1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        assert!(
            build_macro_lineage_candidate(&input, &packet, &graph, [3; 32], None, [4; 32], vec![],)
                .is_err()
        );
        assert!(
            build_macro_lineage_candidate(
                &input,
                &packet,
                &graph,
                [3; 32],
                None,
                [4; 32],
                vec![[99; 32]],
            )
            .is_err()
        );
    }

    #[test]
    fn body_plan_reference_changes_identity_without_inventing_regions() {
        let (input, packet) = packet(1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let first = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            [4; 32],
            vec![graph.nodes[0].id],
        )
        .unwrap();
        let second = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            [5; 32],
            vec![graph.nodes[0].id],
        )
        .unwrap();
        assert_ne!(first.lineage_id, second.lineage_id);
        let encoded = String::from_utf8(first.to_bytes().unwrap()).unwrap();
        assert!(!encoded.contains("head"));
        assert!(!encoded.contains("torso"));
    }

    #[test]
    fn occupancy_order_and_duplicates_canonicalize() {
        let (input, packet) = packet(1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let first = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            [4; 32],
            vec![graph.nodes[1].id, graph.nodes[0].id, graph.nodes[1].id],
        )
        .unwrap();
        let second = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            [4; 32],
            vec![graph.nodes[0].id, graph.nodes[1].id],
        )
        .unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn candidate_accepts_only_exact_validated_family_fingerprint() {
        let fixtures = body_plan_structure::reference_fixtures().unwrap();
        let (input, packet) = packet(1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let candidate = build_macro_lineage_candidate(
            &input,
            &packet,
            &graph,
            [3; 32],
            None,
            fixtures.humanoid.family.family_id,
            vec![graph.nodes[0].id],
        )
        .unwrap();
        let before = candidate.to_bytes().unwrap();
        assert_eq!(
            validate_body_plan_binding(
                &candidate,
                &fixtures.humanoid.family,
                body_plan_structure::MAX_VALIDATION_EXAMINATIONS,
            )
            .status,
            body_plan_structure::ValidationStatus::Valid
        );
        assert_eq!(candidate.to_bytes().unwrap(), before);

        let mut expression_ref = candidate.clone();
        expression_ref.body_plan_ref = fixtures.humanoid.expression.expression_id;
        assert_eq!(
            validate_body_plan_binding(
                &expression_ref,
                &fixtures.humanoid.family,
                body_plan_structure::MAX_VALIDATION_EXAMINATIONS,
            )
            .status,
            body_plan_structure::ValidationStatus::Invalid
        );

        let mut forged_family = fixtures.humanoid.family.clone();
        forged_family.family_id = [9; 32];
        assert_eq!(
            validate_body_plan_binding(
                &candidate,
                &forged_family,
                body_plan_structure::MAX_VALIDATION_EXAMINATIONS,
            )
            .status,
            body_plan_structure::ValidationStatus::Invalid
        );
    }
}
