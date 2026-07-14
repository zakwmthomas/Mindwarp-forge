use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    IndeterminateBudget,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Violation {
    pub code: String,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationReport {
    pub status: ValidationStatus,
    pub examined: u32,
    pub violations: Vec<Violation>,
}

fn violation(code: &str, location: impl Into<String>) -> Violation {
    Violation {
        code: code.into(),
        location: location.into(),
    }
}

pub fn validate_package(package: &SemanticConstructionPackage, budget: u32) -> ValidationReport {
    let mut report = ValidationReport {
        status: ValidationStatus::Valid,
        examined: 0,
        violations: Vec::new(),
    };
    macro_rules! examine {
        () => {{
            if report.examined >= budget {
                report.status = ValidationStatus::IndeterminateBudget;
                report.violations.sort();
                return report;
            }
            report.examined += 1;
        }};
    }

    examine!();
    if ensure_contract_version(package.schema_version).is_err()
        || ensure_contract_version(package.context.schema_version).is_err()
        || ensure_contract_version(package.recipe.schema_version).is_err()
    {
        report
            .violations
            .push(violation("unknown_schema", "package"));
    }

    let concepts: BTreeSet<Id> = package
        .context
        .concepts
        .iter()
        .map(|item| item.id)
        .collect();
    if concepts.len() != package.context.concepts.len() {
        report
            .violations
            .push(violation("duplicate_concept", "context.concepts"));
    }
    let claims: BTreeSet<Id> = package.context.claims.iter().map(|item| item.id).collect();
    if claims.len() != package.context.claims.len() {
        report
            .violations
            .push(violation("duplicate_claim", "context.claims"));
    }
    for claim in &package.context.claims {
        examine!();
        if !concepts.contains(&claim.concept_id) {
            report.violations.push(violation(
                "unknown_concept",
                format!("claim:{:?}", claim.id),
            ));
        }
    }

    let mut indegree: BTreeMap<Id, usize> = claims.iter().map(|id| (*id, 0)).collect();
    let mut outgoing: BTreeMap<Id, Vec<Id>> = BTreeMap::new();
    for edge in &package.context.justification {
        examine!();
        if !claims.contains(&edge.from) || !claims.contains(&edge.to) {
            report.violations.push(violation(
                "unknown_justification_claim",
                "context.justification",
            ));
            continue;
        }
        *indegree.get_mut(&edge.to).unwrap() += 1;
        outgoing.entry(edge.from).or_default().push(edge.to);
    }
    let mut queue: VecDeque<Id> = indegree
        .iter()
        .filter(|(_, n)| **n == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut visited = 0;
    while let Some(id) = queue.pop_front() {
        visited += 1;
        for child in outgoing.get(&id).into_iter().flatten() {
            let degree = indegree.get_mut(child).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push_back(*child);
            }
        }
    }
    if visited != claims.len() {
        report
            .violations
            .push(violation("justification_cycle", "context.justification"));
    }

    let claim_rows: BTreeMap<Id, &Claim> = package
        .context
        .claims
        .iter()
        .map(|item| (item.id, item))
        .collect();
    let mut grounded: BTreeSet<Id> = package
        .context
        .claims
        .iter()
        .filter(|item| matches!(item.class, ClaimClass::Observed | ClaimClass::Declared))
        .map(|item| item.id)
        .collect();
    loop {
        let before = grounded.len();
        for edge in &package.context.justification {
            if grounded.contains(&edge.from)
                && matches!(
                    edge.kind,
                    JustificationKind::Supports
                        | JustificationKind::Derives
                        | JustificationKind::Requires
                )
                && claim_rows.contains_key(&edge.to)
            {
                grounded.insert(edge.to);
            }
        }
        if grounded.len() == before {
            break;
        }
    }

    let role_ids: BTreeSet<Id> = package.roles.iter().map(|item| item.id).collect();
    for role in &package.roles {
        examine!();
        if !concepts.contains(&role.concept_id)
            || role.source_claims.is_empty()
            || role
                .source_claims
                .iter()
                .any(|claim| !grounded.contains(claim))
        {
            report
                .violations
                .push(violation("unsupported_role", format!("role:{:?}", role.id)));
        }
    }

    let feasible: Vec<&SolutionFamily> = package
        .solutions
        .families
        .iter()
        .filter(|item| item.feasible)
        .collect();
    let mechanisms: BTreeSet<Vec<Id>> = feasible
        .iter()
        .map(|item| {
            let mut claims = item.mechanism_claims.clone();
            claims.sort();
            claims
        })
        .collect();
    if feasible.is_empty() {
        report
            .violations
            .push(violation("no_feasible_family", "solutions"));
    } else if mechanisms.len() < 2 && package.solutions.single_feasible_family.is_none() {
        report
            .violations
            .push(violation("fake_or_missing_diversity", "solutions"));
    }
    for family in &package.solutions.families {
        examine!();
        if family
            .required_roles
            .iter()
            .any(|role| !role_ids.contains(role))
        {
            report.violations.push(violation(
                "unknown_family_role",
                format!("family:{:?}", family.id),
            ));
        }
        if family.mechanism_claims.is_empty()
            || family
                .mechanism_claims
                .iter()
                .any(|claim| !grounded.contains(claim))
        {
            report.violations.push(violation(
                "ungrounded_mechanism",
                format!("family:{:?}", family.id),
            ));
        }
        let dimensions: BTreeSet<Id> = family
            .trade_vector
            .iter()
            .map(|item| item.dimension_id)
            .collect();
        if dimensions.len() != family.trade_vector.len() {
            report.violations.push(violation(
                "duplicate_trade_dimension",
                format!("family:{:?}", family.id),
            ));
        }
    }
    if let Some(selected) = package.solutions.selected_family {
        if !feasible.iter().any(|item| item.id == selected) {
            report.violations.push(violation(
                "selected_family_not_feasible",
                "solutions.selected_family",
            ));
        }
        if package.solutions.selection_rationale.is_empty() {
            report.violations.push(violation(
                "missing_selection_rationale",
                "solutions.selection_rationale",
            ));
        }
    }

    validate_capabilities(
        &package.registry,
        &package.initial_graph.capabilities,
        &mut report,
        budget,
    );
    if report.status == ValidationStatus::IndeterminateBudget {
        return report;
    }
    validate_graph(&package.initial_graph, &role_ids, &mut report, budget);
    if report.status == ValidationStatus::IndeterminateBudget {
        return report;
    }
    if replay_recipe(
        &package.initial_graph,
        &package.recipe,
        &package.registry,
        &role_ids,
    )
    .is_err()
    {
        report
            .violations
            .push(violation("recipe_replay_failed", "recipe"));
    }
    report.violations.sort();
    if !report.violations.is_empty() {
        report.status = ValidationStatus::Invalid;
    }
    report
}

fn spend(report: &mut ValidationReport, budget: u32) -> bool {
    if report.examined >= budget {
        report.status = ValidationStatus::IndeterminateBudget;
        false
    } else {
        report.examined += 1;
        true
    }
}

fn validate_capabilities(
    registry: &CapabilityRegistry,
    graph: &CapabilityGraph,
    report: &mut ValidationReport,
    budget: u32,
) {
    if registry.version != graph.registry_version {
        report
            .violations
            .push(violation("stale_capability_registry", "capabilities"));
    }
    let specs: BTreeMap<Id, &CapabilitySpec> =
        registry.specs.iter().map(|item| (item.id, item)).collect();
    let requested: BTreeSet<Id> = graph.requested.iter().copied().collect();
    if requested.len() != graph.requested.len() {
        report
            .violations
            .push(violation("duplicate_capability", "capabilities.requested"));
    }
    for id in &requested {
        if !spend(report, budget) {
            return;
        }
        let Some(spec) = specs.get(id) else {
            report.violations.push(violation(
                "unknown_capability",
                format!("capability:{id:?}"),
            ));
            continue;
        };
        for dependency in &spec.dependencies {
            if !requested.contains(dependency) {
                report.violations.push(violation(
                    "missing_capability_dependency",
                    format!("capability:{id:?}"),
                ));
            }
        }
        for conflict in &spec.conflicts {
            if requested.contains(conflict) {
                report.violations.push(violation(
                    "capability_conflict",
                    format!("capability:{id:?}"),
                ));
            }
        }
    }
}

fn validate_graph(
    graph: &PartRoleGraph,
    roles: &BTreeSet<Id>,
    report: &mut ValidationReport,
    budget: u32,
) {
    let nodes: BTreeMap<Id, &PartNode> = graph.nodes.iter().map(|item| (item.id, item)).collect();
    let sockets: BTreeMap<Id, &Socket> = graph.sockets.iter().map(|item| (item.id, item)).collect();
    let edge_ids: BTreeSet<Id> = graph.edges.iter().map(|item| item.id).collect();
    if nodes.len() != graph.nodes.len()
        || sockets.len() != graph.sockets.len()
        || edge_ids.len() != graph.edges.len()
    {
        report
            .violations
            .push(violation("duplicate_graph_id", "graph"));
    }
    for node in graph.nodes.iter() {
        if !spend(report, budget) {
            return;
        }
        if !roles.contains(&node.role_id) {
            report.violations.push(violation(
                "unknown_part_role",
                format!("node:{:?}", node.id),
            ));
        }
    }
    let mut counts: BTreeMap<Id, u16> = BTreeMap::new();
    let mut adjacent: BTreeMap<Id, BTreeSet<Id>> = BTreeMap::new();
    for socket in &graph.sockets {
        if !nodes.contains_key(&socket.owner) || socket.min_connections > socket.max_connections {
            report.violations.push(violation(
                "invalid_socket",
                format!("socket:{:?}", socket.id),
            ));
        }
    }
    for edge in &graph.edges {
        if !spend(report, budget) {
            return;
        }
        let (Some(from), Some(to)) = (sockets.get(&edge.from_socket), sockets.get(&edge.to_socket))
        else {
            report
                .violations
                .push(violation("dangling_edge", format!("edge:{:?}", edge.id)));
            continue;
        };
        *counts.entry(from.id).or_default() += 1;
        *counts.entry(to.id).or_default() += 1;
        adjacent.entry(from.owner).or_default().insert(to.owner);
        adjacent.entry(to.owner).or_default().insert(from.owner);
        if from.interface_type != to.interface_type
            || from.direction == SocketDirection::Input
            || to.direction == SocketDirection::Output
        {
            report.violations.push(violation(
                "incompatible_socket",
                format!("edge:{:?}", edge.id),
            ));
        }
    }
    for socket in &graph.sockets {
        let count = counts.get(&socket.id).copied().unwrap_or(0);
        if count < socket.min_connections || count > socket.max_connections {
            report.violations.push(violation(
                "socket_cardinality",
                format!("socket:{:?}", socket.id),
            ));
        }
    }
    if let Some(start) = nodes.keys().next().copied() {
        let mut seen = BTreeSet::from([start]);
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            for next in adjacent.get(&node).into_iter().flatten() {
                if seen.insert(*next) {
                    queue.push_back(*next);
                }
            }
        }
        if seen.len() != nodes.len() {
            report
                .violations
                .push(violation("disconnected_graph", "graph.nodes"));
        }
    }
}

pub fn replay_recipe(
    initial: &PartRoleGraph,
    recipe: &ConstructionRecipe,
    registry: &CapabilityRegistry,
    roles: &BTreeSet<Id>,
) -> Result<PartRoleGraph, SemanticConstructionError> {
    ensure_contract_version(recipe.schema_version)?;
    let mut graph = initial.clone();
    let mut operation_ids = BTreeSet::new();
    for operation in &recipe.operations {
        if !operation_ids.insert(operation.operation_id) {
            return Err(SemanticConstructionError::Invalid("duplicate operation"));
        }
        if graph.fingerprint()? != operation.expected_before {
            return Err(SemanticConstructionError::StalePrecondition);
        }
        match &operation.action {
            RecipeAction::AddPart { node } => {
                if graph.nodes.iter().any(|item| item.id == node.id) {
                    return Err(SemanticConstructionError::ValidationFailed);
                }
                graph.nodes.push(node.clone());
            }
            RecipeAction::AddSocket { socket } => {
                if graph.sockets.iter().any(|item| item.id == socket.id)
                    || !graph.nodes.iter().any(|item| item.id == socket.owner)
                {
                    return Err(SemanticConstructionError::ValidationFailed);
                }
                graph.sockets.push(socket.clone());
            }
            RecipeAction::Connect { edge } => {
                if graph.edges.iter().any(|item| item.id == edge.id) {
                    return Err(SemanticConstructionError::ValidationFailed);
                }
                graph.edges.push(edge.clone());
            }
            RecipeAction::RemovePart { node_id } => {
                let attached: BTreeSet<Id> = graph
                    .sockets
                    .iter()
                    .filter(|item| item.owner == *node_id)
                    .map(|item| item.id)
                    .collect();
                if graph.edges.iter().any(|edge| {
                    attached.contains(&edge.from_socket) || attached.contains(&edge.to_socket)
                }) {
                    return Err(SemanticConstructionError::ValidationFailed);
                }
                graph.sockets.retain(|item| item.owner != *node_id);
                let before = graph.nodes.len();
                graph.nodes.retain(|item| item.id != *node_id);
                if before == graph.nodes.len() {
                    return Err(SemanticConstructionError::ValidationFailed);
                }
            }
        }
        graph.canonicalize();
    }
    if graph.fingerprint()? != recipe.expected_result {
        return Err(SemanticConstructionError::ValidationFailed);
    }
    let mut report = ValidationReport {
        status: ValidationStatus::Valid,
        examined: 0,
        violations: Vec::new(),
    };
    validate_capabilities(registry, &graph.capabilities, &mut report, u32::MAX);
    validate_graph(&graph, roles, &mut report, u32::MAX);
    if !report.violations.is_empty() {
        return Err(SemanticConstructionError::ValidationFailed);
    }
    Ok(graph)
}
