//! Capability-free G1-C6 integration proof: the first named organism-ecology
//! prerequisite ("environmental support for sensory-channel candidates")
//! grounded in a real
//! `derived_world_rules::CausalWorldPacket`, reusing `semantic-construction`'s
//! already-proven `PressureContext -> RoleSet -> SolutionFamilySet` pipeline
//! and its unmodified validator.
//!
//! This crate answers one bounded question: given real physical causal
//! conditions (bounded local signal potential and required-medium availability),
//! which of a tiny disposable sensory-mechanism fixture vocabulary clear an
//! environmental-support gate for "distance sensing", and does the required
//! mechanism-diversity/single-feasible-family rule already prove out with
//! real grounding rather than hand-invented labels? Passing this gate is
//! necessary but not sufficient for biological feasibility: emitters,
//! receivers, physiology, lineage, body plan, and evolutionary history are
//! outside the current world packet.
//!
//! Explicit non-claims: this is not the Mind Warp content grammar, not a
//! biological/physiological correctness claim, not person-form eligibility,
//! and not applicable dimorphism (which needs a within-species male/female or
//! caste comparison, not a cross-mechanism sensory comparison). Those remain
//! open, separate increments.

use derived_world_rules::{
    CausalWorldPacket, SignalChannel, WorldError, WorldGenerationInput, validate_world_packet,
};
use semantic_construction::{
    CapabilityGraph, CapabilityRegistry, Claim, ClaimClass, Concept, ConstructionRecipe, Id,
    PartRoleGraph, PressureContext, Role, SemanticConstructionError, SemanticConstructionPackage,
    SolutionFamily, SolutionFamilySet, TradeValue,
};
use sha2::{Digest, Sha256};

/// Below this effective strength (permille), a signal channel is treated as
/// too weak to ground a sensory mechanism. This is a disposable fixture
/// threshold, not an approved product/biological constant.
pub const SUFFICIENT_SIGNAL_PERMILLE: u16 = 300;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NicheBindingError {
    InvalidWorld(WorldError),
    Semantic(SemanticConstructionError),
}

impl From<SemanticConstructionError> for NicheBindingError {
    fn from(value: SemanticConstructionError) -> Self {
        Self::Semantic(value)
    }
}

fn fixture_id(label: &str) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.organism-niche-binding.fixture.v1\0");
    hasher.update(label.as_bytes());
    hasher.finalize().into()
}

fn packet_evidence_id(packet: &CausalWorldPacket) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.organism-niche-binding.packet-evidence.v1\0");
    hasher.update(packet.packet_id.as_bytes());
    hasher.finalize().into()
}

fn signal_strength(packet: &CausalWorldPacket, channel: SignalChannel) -> u16 {
    packet
        .content
        .signals
        .iter()
        .find(|item| item.channel == channel)
        .map(|item| item.effective_strength_permille)
        .unwrap_or(0)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensoryMechanism {
    PhotopicVision,
    Echolocation,
    Chemoreception,
}

impl SensoryMechanism {
    fn label(self) -> &'static str {
        match self {
            Self::PhotopicVision => "photopic-vision-mechanism",
            Self::Echolocation => "echolocation-mechanism",
            Self::Chemoreception => "chemoreception-mechanism",
        }
    }

    fn grounding_channel(self) -> SignalChannel {
        match self {
            Self::PhotopicVision => SignalChannel::VisibleRadiance,
            Self::Echolocation => SignalChannel::PressureWave,
            Self::Chemoreception => SignalChannel::ChemicalGradient,
        }
    }
}

const MECHANISMS: [SensoryMechanism; 3] = [
    SensoryMechanism::PhotopicVision,
    SensoryMechanism::Echolocation,
    SensoryMechanism::Chemoreception,
];

/// Builds a strict `SemanticConstructionPackage` whose candidate families'
/// `feasible` field means only "passes this disposable environmental-support
/// gate". It never means that an organism, organ, emitter, receiver, or
/// complete biological mechanism is feasible.
pub fn build_distance_sensing_niche_package(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
) -> Result<SemanticConstructionPackage, NicheBindingError> {
    validate_world_packet(input, packet).map_err(NicheBindingError::InvalidWorld)?;
    let evidence_ref = packet_evidence_id(packet);
    let niche_concept = Concept {
        id: fixture_id("distance-sensing-niche"),
        preferred_label: "distance-sensing-niche".into(),
        alternate_labels: vec![],
    };

    let mut concepts = vec![niche_concept.clone()];
    let mut claims = Vec::new();
    let mut families = Vec::new();

    for mechanism in MECHANISMS {
        let strength = signal_strength(packet, mechanism.grounding_channel());
        let sufficient = strength >= SUFFICIENT_SIGNAL_PERMILLE;
        let mechanism_concept = Concept {
            id: fixture_id(mechanism.label()),
            preferred_label: mechanism.label().into(),
            alternate_labels: vec![],
        };
        let signal_concept = Concept {
            id: fixture_id(&format!(
                "{}-environmental-signal-measurement",
                mechanism.label()
            )),
            preferred_label: format!(
                "{} environmental signal measurement",
                mechanism.grounding_channel_label()
            ),
            alternate_labels: vec![],
        };
        concepts.push(mechanism_concept.clone());
        concepts.push(signal_concept.clone());

        // The packet measurement is observed even when it is zero or below
        // threshold. `feasible` below records only whether that observed
        // measurement clears this disposable environmental-support gate.
        let claim = Claim {
            id: fixture_id(&format!("{}-signal-observed", mechanism.label())),
            concept_id: signal_concept.id,
            class: ClaimClass::Observed,
            evidence_ref,
        };
        let claim_id = claim.id;
        claims.push(claim);

        families.push((
            mechanism,
            SolutionFamily {
                id: fixture_id(&format!("{}-family", mechanism.label())),
                mechanism_id: mechanism_concept.id,
                mechanism_claims: vec![claim_id],
                required_roles: vec![],
                trade_vector: vec![TradeValue {
                    dimension_id: fixture_id("signal-effective-strength"),
                    value: i32::from(strength),
                    unit: "permille".into(),
                    classification: "simulated".into(),
                }],
                feasible: sufficient,
                rejection_reasons: if sufficient {
                    vec![]
                } else {
                    vec![format!(
                        "{} below sufficient threshold ({strength}/{SUFFICIENT_SIGNAL_PERMILLE} permille)",
                        mechanism.grounding_channel_label()
                    )]
                },
            },
        ));
    }

    let role = Role {
        id: fixture_id("distance-sensing-role"),
        concept_id: niche_concept.id,
        source_claims: claims.iter().map(|claim| claim.id).collect(),
    };
    for family in &mut families {
        family.1.required_roles = vec![role.id];
    }

    let feasible_count = families
        .iter()
        .filter(|(_, family)| family.feasible)
        .count();
    let single_feasible_family = if feasible_count == 1 {
        let (mechanism, _) = families.iter().find(|(_, family)| family.feasible).unwrap();
        Some(format!(
            "only {} clears the disposable environmental-support threshold under these causal conditions",
            mechanism.label()
        ))
    } else {
        None
    };
    let selected_family = families
        .iter()
        .find(|(_, family)| family.feasible)
        .map(|(_, family)| family.id);

    let context = PressureContext {
        schema_version: 1,
        descriptor_ref: evidence_ref,
        history_ref: None,
        concepts,
        claims: claims.clone(),
        // Every claim here records an observed packet measurement. It is not
        // an observation that an organism possesses the named mechanism.
        justification: Vec::new(),
    };

    // Stop before body-plan construction. Environmental support cannot
    // justify fabricating an organ node.
    let mut initial_graph = PartRoleGraph {
        nodes: vec![],
        sockets: vec![],
        edges: vec![],
        capabilities: CapabilityGraph {
            registry_version: 1,
            requested: vec![],
        },
    };
    initial_graph.canonicalize();
    let expected_result = initial_graph.fingerprint()?;

    Ok(SemanticConstructionPackage {
        schema_version: 1,
        policy_version: 1,
        context,
        roles: vec![role],
        solutions: SolutionFamilySet {
            families: families.into_iter().map(|(_, family)| family).collect(),
            selected_family,
            selection_rationale: vec![
                "disposable environmental-support threshold checked before any trade comparison; biological feasibility remains unproven".into(),
            ],
            single_feasible_family,
        },
        registry: CapabilityRegistry {
            version: 1,
            specs: vec![],
        },
        initial_graph,
        recipe: ConstructionRecipe {
            schema_version: 1,
            operations: vec![],
            expected_result,
        },
    })
}

impl SensoryMechanism {
    fn grounding_channel_label(self) -> &'static str {
        match self.grounding_channel() {
            SignalChannel::VisibleRadiance => "visible radiance",
            SignalChannel::PressureWave => "pressure wave",
            SignalChannel::ChemicalGradient => "chemical gradient",
            _ => "unmapped channel",
        }
    }
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
    use semantic_construction::{ValidationStatus, validate_package};

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

    fn base_input() -> WorldGenerationInput {
        WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            surface_material: surface_material_contract(),
            regional_environment: regional_contract(),
            signal_potentials: vec![],
        }
    }

    fn packet_with_signals(sources: Vec<SignalPotential>) -> CausalWorldPacket {
        let mut input = base_input();
        input.signal_potentials = sources;
        compile_world(&input).unwrap()
    }

    #[test]
    fn light_rich_world_makes_vision_the_single_environmentally_supported_family() {
        let packet = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }]);
        let input = base_input_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }]);
        let package = build_distance_sensing_niche_package(&input, &packet).unwrap();
        assert_eq!(
            validate_package(&package, 512).status,
            ValidationStatus::Valid
        );
        assert!(package.solutions.single_feasible_family.is_some());
        let feasible: Vec<_> = package
            .solutions
            .families
            .iter()
            .filter(|item| item.feasible)
            .collect();
        assert_eq!(feasible.len(), 1);
        assert_eq!(
            feasible[0].mechanism_id,
            fixture_id("photopic-vision-mechanism")
        );
    }

    #[test]
    fn dark_world_with_pressure_and_chemical_signal_supports_two_candidates() {
        let packet = packet_with_signals(vec![
            SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 50,
            },
            SignalPotential {
                channel: SignalChannel::PressureWave,
                baseline_strength_permille: 800,
            },
            SignalPotential {
                channel: SignalChannel::ChemicalGradient,
                baseline_strength_permille: 700,
            },
        ]);
        let input = base_input_with_signals(vec![
            SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 50,
            },
            SignalPotential {
                channel: SignalChannel::PressureWave,
                baseline_strength_permille: 800,
            },
            SignalPotential {
                channel: SignalChannel::ChemicalGradient,
                baseline_strength_permille: 700,
            },
        ]);
        let package = build_distance_sensing_niche_package(&input, &packet).unwrap();
        assert_eq!(
            validate_package(&package, 512).status,
            ValidationStatus::Valid
        );
        assert!(package.solutions.single_feasible_family.is_none());
        let feasible_count = package
            .solutions
            .families
            .iter()
            .filter(|item| item.feasible)
            .count();
        assert_eq!(feasible_count, 2);
    }

    #[test]
    fn no_grounded_signal_yields_no_feasible_family_and_fails_validation() {
        let packet = packet_with_signals(vec![]);
        let input = base_input_with_signals(vec![]);
        let package = build_distance_sensing_niche_package(&input, &packet).unwrap();
        let report = validate_package(&package, 512);
        assert_eq!(report.status, ValidationStatus::Invalid);
        assert!(
            report
                .violations
                .iter()
                .any(|item| item.code == "no_feasible_family")
        );
    }

    #[test]
    fn changing_only_the_real_causal_packet_changes_feasibility_deterministically() {
        let light = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }]);
        let dark = packet_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 50,
        }]);
        let light_input = base_input_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }]);
        let dark_input = base_input_with_signals(vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 50,
        }]);
        let light_package = build_distance_sensing_niche_package(&light_input, &light).unwrap();
        let dark_package = build_distance_sensing_niche_package(&dark_input, &dark).unwrap();
        assert_ne!(
            light_package.fingerprint().unwrap(),
            dark_package.fingerprint().unwrap()
        );

        let replay = build_distance_sensing_niche_package(&light_input, &light).unwrap();
        assert_eq!(
            light_package.fingerprint().unwrap(),
            replay.fingerprint().unwrap()
        );
    }

    fn base_input_with_signals(sources: Vec<SignalPotential>) -> WorldGenerationInput {
        let mut input = base_input();
        input.signal_potentials = sources;
        input
    }

    #[test]
    fn packet_from_another_input_cannot_drive_support() {
        let expected = base_input_with_signals(vec![]);
        let actual = base_input_with_signals(vec![SignalPotential {
            channel: SignalChannel::MagneticField,
            baseline_strength_permille: 900,
        }]);
        let packet = compile_world(&actual).unwrap();
        assert!(matches!(
            build_distance_sensing_niche_package(&expected, &packet),
            Err(NicheBindingError::InvalidWorld(_))
        ));
    }
}
