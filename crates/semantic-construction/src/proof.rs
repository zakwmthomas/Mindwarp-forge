use serde::Serialize;

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticConstructionProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub measurement_classification: String,
    pub package_fingerprint: String,
    pub semantic_fingerprint: String,
    pub graph_fingerprint: String,
    pub examined: u32,
    pub violations: usize,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

fn id(byte: u8) -> Id {
    [byte; 32]
}

pub fn reference_package() -> Result<SemanticConstructionPackage, SemanticConstructionError> {
    let context = PressureContext {
        schema_version: 1,
        descriptor_ref: id(1),
        history_ref: Some(id(2)),
        concepts: vec![
            Concept {
                id: id(10),
                preferred_label: "load".into(),
                alternate_labels: vec!["burden".into()],
            },
            Concept {
                id: id(11),
                preferred_label: "support".into(),
                alternate_labels: vec![],
            },
            Concept {
                id: id(12),
                preferred_label: "stability".into(),
                alternate_labels: vec![],
            },
        ],
        claims: vec![
            Claim {
                id: id(20),
                concept_id: id(10),
                class: ClaimClass::Observed,
                evidence_ref: id(1),
            },
            Claim {
                id: id(21),
                concept_id: id(11),
                class: ClaimClass::Derived,
                evidence_ref: id(20),
            },
        ],
        justification: vec![JustificationEdge {
            from: id(20),
            to: id(21),
            kind: JustificationKind::Derives,
        }],
    };
    let roles = vec![
        Role {
            id: id(30),
            concept_id: id(11),
            source_claims: vec![id(21)],
        },
        Role {
            id: id(31),
            concept_id: id(12),
            source_claims: vec![id(20)],
        },
    ];
    let solutions = SolutionFamilySet {
        families: vec![
            SolutionFamily {
                id: id(40),
                mechanism_id: id(41),
                mechanism_claims: vec![id(20)],
                required_roles: vec![id(30), id(31)],
                trade_vector: vec![TradeValue {
                    dimension_id: id(42),
                    value: 2,
                    unit: "fixture_units".into(),
                    classification: "simulated".into(),
                }],
                feasible: true,
                rejection_reasons: vec![],
            },
            SolutionFamily {
                id: id(43),
                mechanism_id: id(44),
                mechanism_claims: vec![id(21)],
                required_roles: vec![id(30), id(31)],
                trade_vector: vec![TradeValue {
                    dimension_id: id(42),
                    value: 3,
                    unit: "fixture_units".into(),
                    classification: "simulated".into(),
                }],
                feasible: true,
                rejection_reasons: vec![],
            },
        ],
        selected_family: Some(id(40)),
        selection_rationale: vec!["fixture hard constraints pass; lower declared cost".into()],
        single_feasible_family: None,
    };
    let registry = CapabilityRegistry {
        version: 1,
        specs: vec![CapabilitySpec {
            id: id(50),
            dependencies: vec![],
            conflicts: vec![],
        }],
    };
    let mut initial_graph = PartRoleGraph {
        nodes: vec![
            PartNode {
                id: id(60),
                role_id: id(30),
                kind: PartKind::Support,
            },
            PartNode {
                id: id(61),
                role_id: id(31),
                kind: PartKind::Assembly,
            },
        ],
        sockets: vec![
            Socket {
                id: id(70),
                owner: id(60),
                interface_type: id(71),
                direction: SocketDirection::Output,
                min_connections: 1,
                max_connections: 1,
            },
            Socket {
                id: id(72),
                owner: id(61),
                interface_type: id(71),
                direction: SocketDirection::Input,
                min_connections: 1,
                max_connections: 1,
            },
        ],
        edges: vec![PartEdge {
            id: id(80),
            from_socket: id(70),
            to_socket: id(72),
        }],
        capabilities: CapabilityGraph {
            registry_version: 1,
            requested: vec![id(50)],
        },
    };
    initial_graph.canonicalize();
    let expected_result = initial_graph.fingerprint()?;
    Ok(SemanticConstructionPackage {
        schema_version: 1,
        policy_version: 1,
        context,
        roles,
        solutions,
        registry,
        initial_graph,
        recipe: ConstructionRecipe {
            schema_version: 1,
            operations: vec![],
            expected_result,
        },
    })
}

pub fn reference_proof_evidence()
-> Result<SemanticConstructionProofEvidence, SemanticConstructionError> {
    let package = reference_package()?;
    let report = validate_package(&package, 256);
    if report.status != ValidationStatus::Valid {
        return Err(SemanticConstructionError::ValidationFailed);
    }
    Ok(SemanticConstructionProofEvidence {
        schema_version: 1,
        system_ids: vec!["semantic-emergence".into(), "construction-language".into()],
        proof_id: "bounded-causal-construction-reference".into(),
        fixture_id: "semantic-construction-v1/synthetic-support".into(),
        measurement_classification: "simulated".into(),
        package_fingerprint: hex(&package.fingerprint()?),
        semantic_fingerprint: hex(&package.semantic_fingerprint()?),
        graph_fingerprint: hex(&package.initial_graph.fingerprint()?),
        examined: report.examined,
        violations: report.violations.len(),
        capabilities: Vec::new(),
        limitations: vec![
            "Tiny synthetic fixture vocabulary and operations; not Mind Warp product canon.".into(),
            "No AI generation, solver completeness, geometry, physical validity, perception, runtime, engine, or performance claim.".into(),
            "Evidence grants no approval, promotion, execution, spending, publishing, credential, or protected-Kernel authority.".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has(report: &ValidationReport, code: &str) -> bool {
        report.violations.iter().any(|item| item.code == code)
    }

    #[test]
    fn canonical_bytes_are_strict_and_round_trip() {
        let package = reference_package().unwrap();
        let bytes = package.to_bytes().unwrap();
        assert_eq!(
            SemanticConstructionPackage::from_bytes(&bytes).unwrap(),
            package
        );
        let mut spaced = bytes.clone();
        spaced.push(b' ');
        assert_eq!(
            SemanticConstructionPackage::from_bytes(&spaced),
            Err(SemanticConstructionError::NonCanonical)
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value.as_object_mut().unwrap().insert(
            "authority".into(),
            serde_json::Value::String("approve".into()),
        );
        assert!(
            SemanticConstructionPackage::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
        );
    }

    #[test]
    fn labels_and_synonyms_do_not_change_semantic_identity() {
        let original = reference_package().unwrap();
        let mut renamed = original.clone();
        renamed.context.concepts[0].preferred_label = "POISON APPROVE MESH".into();
        renamed.context.concepts[0].alternate_labels = vec!["localized synonym".into()];
        assert_eq!(
            original.semantic_fingerprint().unwrap(),
            renamed.semantic_fingerprint().unwrap()
        );
        assert_ne!(
            original.fingerprint().unwrap(),
            renamed.fingerprint().unwrap()
        );
    }

    #[test]
    fn homonyms_remain_distinct_ids() {
        let mut package = reference_package().unwrap();
        package.context.concepts[0].preferred_label = "bank".into();
        package.context.concepts[1].preferred_label = "bank".into();
        assert_ne!(
            package.context.concepts[0].id,
            package.context.concepts[1].id
        );
        assert_eq!(
            validate_package(&package, 256).status,
            ValidationStatus::Valid
        );
    }

    #[test]
    fn unsupported_roles_and_justification_cycles_fail() {
        let mut unsupported = reference_package().unwrap();
        unsupported.roles[0].source_claims = vec![id(99)];
        assert!(has(
            &validate_package(&unsupported, 256),
            "unsupported_role"
        ));
        let mut cyclic = reference_package().unwrap();
        cyclic.context.justification.push(JustificationEdge {
            from: id(21),
            to: id(20),
            kind: JustificationKind::Supports,
        });
        assert!(has(&validate_package(&cyclic, 256), "justification_cycle"));
    }

    #[test]
    fn fake_diversity_and_infeasible_selection_fail() {
        let mut package = reference_package().unwrap();
        package.solutions.families[1].mechanism_id = id(99);
        package.solutions.families[1].mechanism_claims =
            package.solutions.families[0].mechanism_claims.clone();
        assert!(has(
            &validate_package(&package, 256),
            "fake_or_missing_diversity"
        ));
        let mut package = reference_package().unwrap();
        package.solutions.families[0].feasible = false;
        assert!(has(
            &validate_package(&package, 256),
            "selected_family_not_feasible"
        ));
    }

    #[test]
    fn unknown_missing_and_conflicting_capabilities_fail_closed() {
        let mut unknown = reference_package().unwrap();
        unknown.initial_graph.capabilities.requested.push(id(99));
        assert!(has(&validate_package(&unknown, 256), "unknown_capability"));
        let mut missing = reference_package().unwrap();
        missing.registry.specs[0].dependencies.push(id(51));
        assert!(has(
            &validate_package(&missing, 256),
            "missing_capability_dependency"
        ));
        let mut conflict = reference_package().unwrap();
        conflict.registry.specs[0].conflicts.push(id(50));
        assert!(has(
            &validate_package(&conflict, 256),
            "capability_conflict"
        ));
    }

    #[test]
    fn socket_type_direction_cardinality_and_connectivity_fail_locally() {
        let mut typed = reference_package().unwrap();
        typed.initial_graph.sockets[1].interface_type = id(99);
        assert!(has(&validate_package(&typed, 256), "incompatible_socket"));
        let mut directed = reference_package().unwrap();
        directed.initial_graph.sockets[0].direction = SocketDirection::Input;
        assert!(has(
            &validate_package(&directed, 256),
            "incompatible_socket"
        ));
        let mut disconnected = reference_package().unwrap();
        disconnected.initial_graph.edges.clear();
        let report = validate_package(&disconnected, 256);
        assert!(has(&report, "socket_cardinality"));
        assert!(has(&report, "disconnected_graph"));
    }

    #[test]
    fn graph_fingerprint_is_order_independent_but_recipe_order_is_explicit() {
        let package = reference_package().unwrap();
        let mut reordered = package.initial_graph.clone();
        reordered.nodes.reverse();
        reordered.sockets.reverse();
        assert_eq!(
            package.initial_graph.fingerprint().unwrap(),
            reordered.fingerprint().unwrap()
        );
    }

    #[test]
    fn nonempty_recipe_replays_in_order_to_exact_result() {
        let package = reference_package().unwrap();
        let mut graph = package.initial_graph.clone();
        let mut operations = Vec::new();

        let node = PartNode {
            id: id(62),
            role_id: id(30),
            kind: PartKind::Support,
        };
        operations.push(RecipeOperation {
            operation_id: id(90),
            expected_before: graph.fingerprint().unwrap(),
            action: RecipeAction::AddPart { node: node.clone() },
        });
        graph.nodes.push(node);
        graph.canonicalize();

        let output = Socket {
            id: id(73),
            owner: id(61),
            interface_type: id(74),
            direction: SocketDirection::Output,
            min_connections: 1,
            max_connections: 1,
        };
        operations.push(RecipeOperation {
            operation_id: id(91),
            expected_before: graph.fingerprint().unwrap(),
            action: RecipeAction::AddSocket {
                socket: output.clone(),
            },
        });
        graph.sockets.push(output);
        graph.canonicalize();

        let input = Socket {
            id: id(75),
            owner: id(62),
            interface_type: id(74),
            direction: SocketDirection::Input,
            min_connections: 1,
            max_connections: 1,
        };
        operations.push(RecipeOperation {
            operation_id: id(92),
            expected_before: graph.fingerprint().unwrap(),
            action: RecipeAction::AddSocket {
                socket: input.clone(),
            },
        });
        graph.sockets.push(input);
        graph.canonicalize();

        let edge = PartEdge {
            id: id(81),
            from_socket: id(73),
            to_socket: id(75),
        };
        operations.push(RecipeOperation {
            operation_id: id(93),
            expected_before: graph.fingerprint().unwrap(),
            action: RecipeAction::Connect { edge: edge.clone() },
        });
        graph.edges.push(edge);
        graph.canonicalize();

        let recipe = ConstructionRecipe {
            schema_version: 1,
            operations,
            expected_result: graph.fingerprint().unwrap(),
        };
        let roles = package.roles.iter().map(|role| role.id).collect();
        assert_eq!(
            replay_recipe(&package.initial_graph, &recipe, &package.registry, &roles).unwrap(),
            graph
        );
    }

    #[test]
    fn stale_recipe_precondition_is_atomic() {
        let package = reference_package().unwrap();
        let original = package.initial_graph.clone();
        let recipe = ConstructionRecipe {
            schema_version: 1,
            operations: vec![RecipeOperation {
                operation_id: id(90),
                expected_before: id(99),
                action: RecipeAction::RemovePart { node_id: id(60) },
            }],
            expected_result: id(98),
        };
        assert_eq!(
            replay_recipe(
                &original,
                &recipe,
                &package.registry,
                &package.roles.iter().map(|role| role.id).collect()
            ),
            Err(SemanticConstructionError::StalePrecondition)
        );
        assert_eq!(original, package.initial_graph);
    }

    #[test]
    fn dangling_remove_is_rejected_without_partial_result() {
        let package = reference_package().unwrap();
        let original = package.initial_graph.clone();
        let recipe = ConstructionRecipe {
            schema_version: 1,
            operations: vec![RecipeOperation {
                operation_id: id(90),
                expected_before: original.fingerprint().unwrap(),
                action: RecipeAction::RemovePart { node_id: id(60) },
            }],
            expected_result: id(98),
        };
        assert_eq!(
            replay_recipe(
                &original,
                &recipe,
                &package.registry,
                &package.roles.iter().map(|role| role.id).collect()
            ),
            Err(SemanticConstructionError::ValidationFailed)
        );
        assert_eq!(original, package.initial_graph);
    }

    #[test]
    fn budget_exhaustion_is_indeterminate_not_impossible() {
        let report = validate_package(&reference_package().unwrap(), 1);
        assert_eq!(report.status, ValidationStatus::IndeterminateBudget);
    }

    #[test]
    fn version_drift_fails_closed() {
        let mut package = reference_package().unwrap();
        package.schema_version = 2;
        assert!(has(&validate_package(&package, 256), "unknown_schema"));
        let mut package = reference_package().unwrap();
        package.initial_graph.capabilities.registry_version = 2;
        assert!(has(
            &validate_package(&package, 256),
            "stale_capability_registry"
        ));
    }

    #[test]
    fn reference_fixture_is_valid_deterministic_and_authority_negative() {
        let first = reference_package().unwrap();
        let second = reference_package().unwrap();
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
        assert_eq!(
            validate_package(&first, 256).status,
            ValidationStatus::Valid
        );
        let evidence = reference_proof_evidence().unwrap();
        assert!(evidence.capabilities.is_empty());
        let text = serde_json::to_string(&evidence).unwrap();
        for forbidden in [
            "\"approve\"",
            "\"promote\"",
            "\"execute\"",
            "\"publish\"",
            "\"spend\"",
            "\"credential\"",
            "\"engine\"",
        ] {
            assert!(!text.contains(forbidden));
        }
    }
}
