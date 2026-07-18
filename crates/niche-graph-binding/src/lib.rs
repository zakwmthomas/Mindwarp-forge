//! Capability-free G1-C3 physical-opportunity proof for ecological precursors.
//!
//! This module deliberately does **not** construct organs or a body plan.
//! A niche graph is ecological context that future macro-lineages may fill,
//! not a graph of body parts. The bounded proof here converts supported
//! environmental signal channels from one `CausalWorldPacket` into an
//! environmental-opportunity graph:
//!
//! - physical nodes expose only already validated energy, liquid, atmosphere,
//!   substrate, or signal evidence;
//! - each edge means only that two opportunities are co-available in
//!   the same world packet;
//! - no node or edge claims that an organism evolved a sense, emitter,
//!   receiver, organ, body region, communication system, or lineage.
//!
//! This is a strict causal precursor, not a complete ecological niche graph,
//! biome model, or the Mind Warp content grammar. Habitat suitability,
//! hazards, trophic roles, competition, macro-lineages, and body plans remain
//! open.

use derived_world_rules::{
    CausalWorldPacket, SignalChannel, WorldError, WorldGenerationInput, validate_world_packet,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const CONTRACT_VERSION: u16 = 1;

/// Disposable fixture threshold. It proves thresholded graph construction,
/// not a universal ecological or biological constant.
pub const SUPPORTED_SIGNAL_PERMILLE: u16 = 300;

pub type Id = [u8; 32];

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EnvironmentalOpportunity {
    RadiantEnergy {
        regional_exposure_permille: u16,
    },
    SurfaceAccessibleLiquid,
    SurfaceMoisturePotential {
        regional_potential_permille: u16,
    },
    Atmosphere,
    SolidSubstrate {
        global_fraction_permille: u16,
    },
    Signal {
        channel: SignalChannel,
        effective_strength_permille: u16,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentalOpportunityNode {
    pub id: Id,
    pub opportunity: EnvironmentalOpportunity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoavailabilityEdge {
    pub id: Id,
    pub first: Id,
    pub second: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentalOpportunityGraph {
    pub schema_version: u16,
    pub world_packet_id: String,
    pub physical_regime_id: Id,
    pub nodes: Vec<EnvironmentalOpportunityNode>,
    pub coavailability_edges: Vec<CoavailabilityEdge>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    Invalid(&'static str),
    InvalidWorld(WorldError),
    Codec(String),
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update([0]);
    hasher.update(bytes);
    hasher.finalize().into()
}

fn node_id(packet_id: &str, opportunity: &EnvironmentalOpportunity) -> Result<Id, GraphError> {
    let opportunity_bytes =
        serde_json::to_vec(opportunity).map_err(|error| GraphError::Codec(error.to_string()))?;
    let mut bytes = Vec::with_capacity(packet_id.len() + opportunity_bytes.len() + 1);
    bytes.extend_from_slice(packet_id.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&opportunity_bytes);
    Ok(hash(b"mindwarp.environmental-opportunity-node.v1", &bytes))
}

fn edge_id(first: Id, second: Id) -> Id {
    let (low, high) = if first <= second {
        (first, second)
    } else {
        (second, first)
    };
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&low);
    bytes.extend_from_slice(&high);
    hash(b"mindwarp.environmental-coavailability-edge.v1", &bytes)
}

impl EnvironmentalOpportunityGraph {
    pub fn to_bytes(&self) -> Result<Vec<u8>, GraphError> {
        serde_json::to_vec(self).map_err(|error| GraphError::Codec(error.to_string()))
    }

    pub fn fingerprint(&self) -> Result<Id, GraphError> {
        Ok(hash(
            b"mindwarp.environmental-opportunity-graph.v1",
            &self.to_bytes()?,
        ))
    }
}

/// Builds the smallest honest graph available from the current world
/// contract. Edges encode co-availability only; they are not physical joints
/// and do not imply that any one organism uses both channels.
pub fn build_environmental_opportunity_graph(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<EnvironmentalOpportunityGraph, GraphError> {
    validate_world_packet(input, packet).map_err(GraphError::InvalidWorld)?;
    let climate = &input.surface_material.input.climate;
    let hydrological = &climate.input.hydrological;
    let geological = &hydrological.input.geological_atmospheric;

    let mut opportunities = Vec::new();
    if climate
        .state
        .content
        .absorbed_shortwave_quarter_billionths_earth
        > 0
        && packet.content.regional_exposure_permille > 0
    {
        opportunities.push(EnvironmentalOpportunity::RadiantEnergy {
            regional_exposure_permille: packet.content.regional_exposure_permille,
        });
    }
    if hydrological.state.content.has_surface_accessible_liquid {
        opportunities.push(EnvironmentalOpportunity::SurfaceAccessibleLiquid);
        opportunities.push(EnvironmentalOpportunity::SurfaceMoisturePotential {
            regional_potential_permille: input
                .regional_environment
                .state
                .content
                .moisture_potential_permille,
        });
    }
    if geological.state.content.surface_pressure_pa > 0 {
        opportunities.push(EnvironmentalOpportunity::Atmosphere);
    }
    let solid_fraction = geological.state.content.solid_surface_fraction_permille;
    if solid_fraction > 0 {
        opportunities.push(EnvironmentalOpportunity::SolidSubstrate {
            global_fraction_permille: solid_fraction,
        });
    }
    opportunities.extend(
        packet
            .content
            .signals
            .iter()
            .filter(|signal| signal.effective_strength_permille >= SUPPORTED_SIGNAL_PERMILLE)
            .map(|signal| EnvironmentalOpportunity::Signal {
                channel: signal.channel,
                effective_strength_permille: signal.effective_strength_permille,
            }),
    );
    opportunities.sort();
    let physical_regime_bytes =
        serde_json::to_vec(&opportunities).map_err(|error| GraphError::Codec(error.to_string()))?;
    let physical_regime_id = hash(
        b"mindwarp.environmental-physical-regime.v1",
        &physical_regime_bytes,
    );

    let nodes: Vec<_> = opportunities
        .into_iter()
        .map(|opportunity| -> Result<_, GraphError> {
            Ok(EnvironmentalOpportunityNode {
                id: node_id(&packet.packet_id, &opportunity)?,
                opportunity,
            })
        })
        .collect::<Result<_, _>>()?;

    let mut coavailability_edges = Vec::new();
    for first_index in 0..nodes.len() {
        for second_index in (first_index + 1)..nodes.len() {
            let first = nodes[first_index].id;
            let second = nodes[second_index].id;
            coavailability_edges.push(CoavailabilityEdge {
                id: edge_id(first, second),
                first,
                second,
            });
        }
    }

    Ok(EnvironmentalOpportunityGraph {
        schema_version: CONTRACT_VERSION,
        world_packet_id: packet.packet_id.clone(),
        physical_regime_id,
        nodes,
        coavailability_edges,
        limitations: vec![
            "physical co-availability is only an environmental opportunity, not habitat suitability or organism capability"
                .into(),
            "hazards, trophic roles, competition and lineage occupancy are not represented"
                .into(),
            "signal threshold is a disposable fixture value, not a biological constant".into(),
            "regional exposure and global substrate fraction are bounded procedural evidence, not local terrain or scientific validation".into(),
        ],
    })
}

pub fn validate_environmental_opportunity_graph(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    graph: &EnvironmentalOpportunityGraph,
) -> Result<(), GraphError> {
    validate_world_packet(input, packet).map_err(GraphError::InvalidWorld)?;
    if graph.schema_version != CONTRACT_VERSION {
        return Err(GraphError::Invalid("unsupported schema version"));
    }
    if graph.world_packet_id.is_empty() {
        return Err(GraphError::Invalid("missing world packet identity"));
    }
    if graph.world_packet_id != packet.packet_id {
        return Err(GraphError::Invalid("world packet mismatch"));
    }

    let node_ids: BTreeSet<_> = graph.nodes.iter().map(|node| node.id).collect();
    let opportunities: BTreeSet<_> = graph
        .nodes
        .iter()
        .map(|node| node.opportunity.clone())
        .collect();
    if node_ids.len() != graph.nodes.len() || opportunities.len() != graph.nodes.len() {
        return Err(GraphError::Invalid("duplicate opportunity node"));
    }
    if graph.nodes.iter().any(|node| {
        matches!(
            node.opportunity,
            EnvironmentalOpportunity::Signal { effective_strength_permille, .. }
                if effective_strength_permille < SUPPORTED_SIGNAL_PERMILLE
        )
    }) {
        return Err(GraphError::Invalid("unsupported channel included"));
    }

    let mut observed_pairs = BTreeSet::new();
    for edge in &graph.coavailability_edges {
        if edge.first == edge.second
            || !node_ids.contains(&edge.first)
            || !node_ids.contains(&edge.second)
        {
            return Err(GraphError::Invalid("invalid coavailability edge"));
        }
        let pair = if edge.first < edge.second {
            (edge.first, edge.second)
        } else {
            (edge.second, edge.first)
        };
        if edge.id != edge_id(pair.0, pair.1) || !observed_pairs.insert(pair) {
            return Err(GraphError::Invalid("duplicate or misidentified edge"));
        }
    }

    let expected_edge_count = graph
        .nodes
        .len()
        .saturating_mul(graph.nodes.len().saturating_sub(1))
        / 2;
    if graph.coavailability_edges.len() != expected_edge_count {
        return Err(GraphError::Invalid("incomplete coavailability relation"));
    }
    let expected = build_environmental_opportunity_graph(input, packet)?;
    if graph != &expected {
        return Err(GraphError::Invalid("graph does not match causal packet"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use derived_world_rules::{
        ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
        HydrologicalContract, HydrologicalInput, RegionalEnvironmentInput, SignalPotential,
        StellarOrbitalContract, StellarOrbitalInput, WorldGenerationInput, compile_climate,
        compile_geological_atmospheric, compile_hydrological, compile_regional_environment,
        compile_stellar_orbital, compile_world,
    };
    use field_basis::{FieldRecipe, ONE, Term};

    fn regional_contract() -> derived_world_rules::RegionalEnvironmentContract {
        compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
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

    fn stellar_contract() -> StellarOrbitalContract {
        compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn geological_contract() -> GeologicalAtmosphericContract {
        compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            planetary_body_id: [4; 32],
            stellar_orbital: stellar_contract(),
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

    fn hydrological_contract() -> HydrologicalContract {
        compile_hydrological(&HydrologicalInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract(),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap()
    }

    fn climate_contract() -> ClimateContract {
        compile_climate(&ClimateInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            climate_source_id: [6; 32],
            hydrological: hydrological_contract(),
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }
    fn surface_material_contract() -> derived_world_rules::SurfaceMaterialContract {
        derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id: [1; 32],
            material_source_id: [7; 32],
            climate: climate_contract(),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap()
    }

    fn packet_with_signals(
        sources: Vec<SignalPotential>,
    ) -> (WorldGenerationInput, CausalWorldPacket) {
        let input = WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            surface_material: surface_material_contract(),
            regional_environment: regional_contract(),
            signal_potentials: sources,
        };
        let packet = compile_world(&input).unwrap();
        (input, packet)
    }

    #[test]
    fn supported_channels_become_coavailable_opportunities_not_organs() {
        let (input, packet) = packet_with_signals(vec![
            SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 900,
            },
            SignalPotential {
                channel: SignalChannel::ChemicalGradient,
                baseline_strength_permille: 800,
            },
        ]);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        assert_eq!(graph.nodes.len(), 7);
        assert_eq!(graph.coavailability_edges.len(), 21);
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::RadiantEnergy { .. }
        )));
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::SurfaceAccessibleLiquid
        )));
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::SurfaceMoisturePotential {
                regional_potential_permille: 500
            }
        )));
        assert!(
            graph
                .nodes
                .iter()
                .any(|node| matches!(node.opportunity, EnvironmentalOpportunity::Atmosphere))
        );
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::SolidSubstrate { .. }
        )));
        assert!(validate_environmental_opportunity_graph(&input, &packet, &graph).is_ok());
    }

    #[test]
    fn weak_channels_are_retained_in_world_evidence_but_excluded_from_opportunities() {
        let (input, packet) = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 100,
        }]);
        assert_eq!(packet.content.signals.len(), 1);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        assert_eq!(graph.nodes.len(), 5);
        assert!(
            !graph
                .nodes
                .iter()
                .any(|node| matches!(node.opportunity, EnvironmentalOpportunity::Signal { .. }))
        );
        assert!(validate_environmental_opportunity_graph(&input, &packet, &graph).is_ok());
    }

    #[test]
    fn absent_liquid_atmosphere_and_substrate_do_not_become_opportunities() {
        let (mut input, _) = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::MagneticField,
            baseline_strength_permille: 800,
        }]);

        let mut geological_input = input
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric
            .input
            .clone();
        geological_input.solid_surface_fraction_permille = 0;
        geological_input.atmospheric_column_mass_g_m2 = 0;
        geological_input.gas_transmission_rgb_permille = [1_000; 3];
        geological_input.aerosol_transmission_rgb_permille = [1_000; 3];
        let geological = compile_geological_atmospheric(&geological_input).unwrap();

        let mut hydrological_input = input
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .clone();
        hydrological_input.geological_atmospheric = geological;
        hydrological_input.total_water_column_g_m2 = 0;
        hydrological_input.phase_partition_permille = [0; 3];
        hydrological_input.surface_accessible_liquid_fraction_permille = 0;
        let hydrological = compile_hydrological(&hydrological_input).unwrap();

        let mut climate_input = input.surface_material.input.climate.input.clone();
        climate_input.hydrological = hydrological;
        let climate = compile_climate(&climate_input).unwrap();
        let mut material_input = input.surface_material.input.clone();
        material_input.climate = climate;
        input.surface_material =
            derived_world_rules::compile_surface_material(&material_input).unwrap();
        let packet = compile_world(&input).unwrap();

        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        assert!(!graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::SurfaceAccessibleLiquid
                | EnvironmentalOpportunity::SurfaceMoisturePotential { .. }
                | EnvironmentalOpportunity::Atmosphere
                | EnvironmentalOpportunity::SolidSubstrate { .. }
        )));
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::RadiantEnergy { .. }
        )));
        assert!(graph.nodes.iter().any(|node| matches!(
            node.opportunity,
            EnvironmentalOpportunity::Signal {
                channel: SignalChannel::MagneticField,
                ..
            }
        )));
        assert!(validate_environmental_opportunity_graph(&input, &packet, &graph).is_ok());
    }

    #[test]
    fn graph_replay_is_deterministic_and_node_order_is_canonical() {
        let sources = vec![
            SignalPotential {
                channel: SignalChannel::PressureWave,
                baseline_strength_permille: 800,
            },
            SignalPotential {
                channel: SignalChannel::MagneticField,
                baseline_strength_permille: 700,
            },
        ];
        let (input, packet) = packet_with_signals(sources.clone());
        let mut permuted_sources = sources;
        permuted_sources.reverse();
        let (permuted_input, permuted_packet) = packet_with_signals(permuted_sources);
        let first = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let replay =
            build_environmental_opportunity_graph(&permuted_input, &permuted_packet).unwrap();
        assert_eq!(packet, permuted_packet);
        assert_eq!(first.fingerprint().unwrap(), replay.fingerprint().unwrap());
        let channels: Vec<_> = first
            .nodes
            .iter()
            .filter_map(|node| match node.opportunity {
                EnvironmentalOpportunity::Signal { channel, .. } => Some(channel),
                _ => None,
            })
            .collect();
        assert_eq!(
            channels,
            vec![SignalChannel::PressureWave, SignalChannel::MagneticField]
        );
    }

    #[test]
    fn dangling_or_incomplete_edges_fail_closed() {
        let (input, packet) = packet_with_signals(vec![
            SignalPotential {
                channel: SignalChannel::ElectricField,
                baseline_strength_permille: 900,
            },
            SignalPotential {
                channel: SignalChannel::MagneticField,
                baseline_strength_permille: 900,
            },
        ]);
        let mut graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        graph.coavailability_edges.clear();
        assert_eq!(
            validate_environmental_opportunity_graph(&input, &packet, &graph),
            Err(GraphError::Invalid("incomplete coavailability relation"))
        );
    }

    #[test]
    fn serialized_graph_round_trips_exactly() {
        let (input, packet) = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::SubstrateVibration,
            baseline_strength_permille: 800,
        }]);
        let graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        let bytes = graph.to_bytes().unwrap();
        let decoded: EnvironmentalOpportunityGraph = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded, graph);
    }

    #[test]
    fn fabricated_node_is_rejected_against_the_packet() {
        let (input, packet) = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }]);
        let mut graph = build_environmental_opportunity_graph(&input, &packet).unwrap();
        graph.nodes[0].opportunity = EnvironmentalOpportunity::Signal {
            channel: SignalChannel::MagneticField,
            effective_strength_permille: 900,
        };
        assert_eq!(
            validate_environmental_opportunity_graph(&input, &packet, &graph),
            Err(GraphError::Invalid("graph does not match causal packet"))
        );
    }
}
