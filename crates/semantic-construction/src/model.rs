use serde::{Deserialize, Serialize};

use crate::{CONTRACT_VERSION, SemanticConstructionError, canonical_json, hash};

const PACKAGE_DOMAIN: &[u8] = b"mindwarp.semantic-construction.package.v1";
const SEMANTIC_DOMAIN: &[u8] = b"mindwarp.semantic-construction.semantic.v1";
const GRAPH_DOMAIN: &[u8] = b"mindwarp.semantic-construction.graph.v1";

pub type Id = [u8; 32];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClass {
    Observed,
    Derived,
    Declared,
    Hypothesis,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Concept {
    pub id: Id,
    pub preferred_label: String,
    pub alternate_labels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub id: Id,
    pub concept_id: Id,
    pub class: ClaimClass,
    pub evidence_ref: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JustificationKind {
    Supports,
    Derives,
    Requires,
    Conflicts,
    Rejects,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JustificationEdge {
    pub from: Id,
    pub to: Id,
    pub kind: JustificationKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PressureContext {
    pub schema_version: u16,
    pub descriptor_ref: Id,
    pub history_ref: Option<Id>,
    pub concepts: Vec<Concept>,
    pub claims: Vec<Claim>,
    pub justification: Vec<JustificationEdge>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Role {
    pub id: Id,
    pub concept_id: Id,
    pub source_claims: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TradeValue {
    pub dimension_id: Id,
    pub value: i32,
    pub unit: String,
    pub classification: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SolutionFamily {
    pub id: Id,
    pub mechanism_id: Id,
    pub mechanism_claims: Vec<Id>,
    pub required_roles: Vec<Id>,
    pub trade_vector: Vec<TradeValue>,
    pub feasible: bool,
    pub rejection_reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SolutionFamilySet {
    pub families: Vec<SolutionFamily>,
    pub selected_family: Option<Id>,
    pub selection_rationale: Vec<String>,
    pub single_feasible_family: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilitySpec {
    pub id: Id,
    pub dependencies: Vec<Id>,
    pub conflicts: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRegistry {
    pub version: u16,
    pub specs: Vec<CapabilitySpec>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGraph {
    pub registry_version: u16,
    pub requested: Vec<Id>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PartKind {
    Assembly,
    Support,
    Interface,
    ActuatorRole,
    MaterialRegionRole,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartNode {
    pub id: Id,
    pub role_id: Id,
    pub kind: PartKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocketDirection {
    Input,
    Output,
    Bidirectional,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Socket {
    pub id: Id,
    pub owner: Id,
    pub interface_type: Id,
    pub direction: SocketDirection,
    pub min_connections: u16,
    pub max_connections: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartEdge {
    pub id: Id,
    pub from_socket: Id,
    pub to_socket: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartRoleGraph {
    pub nodes: Vec<PartNode>,
    pub sockets: Vec<Socket>,
    pub edges: Vec<PartEdge>,
    pub capabilities: CapabilityGraph,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecipeAction {
    AddPart { node: PartNode },
    AddSocket { socket: Socket },
    Connect { edge: PartEdge },
    RemovePart { node_id: Id },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeOperation {
    pub operation_id: Id,
    pub expected_before: Id,
    pub action: RecipeAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionRecipe {
    pub schema_version: u16,
    pub operations: Vec<RecipeOperation>,
    pub expected_result: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticConstructionPackage {
    pub schema_version: u16,
    pub policy_version: u16,
    pub context: PressureContext,
    pub roles: Vec<Role>,
    pub solutions: SolutionFamilySet,
    pub registry: CapabilityRegistry,
    pub initial_graph: PartRoleGraph,
    pub recipe: ConstructionRecipe,
}

#[derive(Serialize)]
struct SemanticKernel<'a> {
    schema_version: u16,
    descriptor_ref: Id,
    history_ref: Option<Id>,
    concept_ids: Vec<Id>,
    claims: &'a [Claim],
    justification: &'a [JustificationEdge],
    roles: &'a [Role],
    solutions: &'a SolutionFamilySet,
}

impl SemanticConstructionPackage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SemanticConstructionError> {
        canonical_json(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SemanticConstructionError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| SemanticConstructionError::Codec(error.to_string()))?;
        if value.to_bytes()? != bytes {
            return Err(SemanticConstructionError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<Id, SemanticConstructionError> {
        Ok(hash(PACKAGE_DOMAIN, &self.to_bytes()?))
    }

    pub fn semantic_fingerprint(&self) -> Result<Id, SemanticConstructionError> {
        let mut concept_ids: Vec<Id> = self.context.concepts.iter().map(|item| item.id).collect();
        concept_ids.sort();
        let kernel = SemanticKernel {
            schema_version: self.schema_version,
            descriptor_ref: self.context.descriptor_ref,
            history_ref: self.context.history_ref,
            concept_ids,
            claims: &self.context.claims,
            justification: &self.context.justification,
            roles: &self.roles,
            solutions: &self.solutions,
        };
        Ok(hash(SEMANTIC_DOMAIN, &canonical_json(&kernel)?))
    }
}

impl PartRoleGraph {
    pub fn canonicalize(&mut self) {
        self.nodes.sort_by_key(|item| item.id);
        self.sockets.sort_by_key(|item| item.id);
        self.edges.sort_by_key(|item| item.id);
        self.capabilities.requested.sort();
    }

    pub fn fingerprint(&self) -> Result<Id, SemanticConstructionError> {
        let mut canonical = self.clone();
        canonical.canonicalize();
        Ok(hash(GRAPH_DOMAIN, &canonical_json(&canonical)?))
    }
}

pub fn ensure_contract_version(version: u16) -> Result<(), SemanticConstructionError> {
    if version != CONTRACT_VERSION {
        return Err(SemanticConstructionError::Invalid(
            "unsupported contract version",
        ));
    }
    Ok(())
}
