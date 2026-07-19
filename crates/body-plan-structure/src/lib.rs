//! Capability-free, coordinate-free body-plan family and expression contract.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub type Id = [u8; 32];
pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_PART_TEMPLATES: usize = 64;
pub const MAX_RELATION_RULES: usize = 128;
pub const MAX_HOMOLOGY_GROUPS: usize = 32;
pub const MAX_SYMMETRY_DECLARATIONS: usize = 32;
pub const MAX_ACTIVE_PREDICATES: usize = 64;
pub const MAX_OCCURRENCES: usize = 256;
pub const MAX_RELATION_INSTANCES: usize = 512;
pub const MAX_SYMMETRY_POSITIONS: usize = 256;
pub const MAX_LIMITATIONS: usize = 16;
pub const MAX_LIMITATION_BYTES: usize = 160;
pub const MAX_CANONICAL_BYTES: usize = 262_144;
pub const MAX_VALIDATION_EXAMINATIONS: u32 = 4_096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BodyPlanError {
    Codec(String),
    NonCanonical,
    Invalid(&'static str),
    ResourceLimit(&'static str),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cardinality {
    pub minimum: u16,
    pub maximum: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum PresenceRule {
    Unconditional,
    Conditional { predicate_id: Id },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartTemplate {
    pub template_id: Id,
    pub role_id: Id,
    pub cardinality: Cardinality,
    pub presence: PresenceRule,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    Containment,
    StructuralConnection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationRule {
    pub rule_id: Id,
    pub kind: RelationKind,
    pub from_template_id: Id,
    pub to_template_id: Id,
    pub from_degree: Cardinality,
    pub to_degree: Cardinality,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HomologyGroup {
    pub group_id: Id,
    pub member_template_ids: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum SymmetryPattern {
    NoDeclaredSymmetry,
    Bilateral,
    Radial { minimum: u16, maximum: u16 },
    Serial { minimum: u16, maximum: u16 },
    OtherDeclared { pattern_ref: Id, positions: u16 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymmetryDeclaration {
    pub declaration_id: Id,
    pub pattern: SymmetryPattern,
    pub member_template_ids: Vec<Id>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BodyPlanFamilyDefinition {
    pub part_templates: Vec<PartTemplate>,
    pub relation_rules: Vec<RelationRule>,
    pub homology_groups: Vec<HomologyGroup>,
    pub symmetry_declarations: Vec<SymmetryDeclaration>,
    pub limitations: Vec<String>,
}

impl BodyPlanFamilyDefinition {
    pub fn empty() -> Self {
        Self {
            part_templates: vec![],
            relation_rules: vec![],
            homology_groups: vec![],
            symmetry_declarations: vec![],
            limitations: vec![],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BodyPlanFamily {
    pub schema_version: u16,
    pub family_id: Id,
    pub part_templates: Vec<PartTemplate>,
    pub relation_rules: Vec<RelationRule>,
    pub homology_groups: Vec<HomologyGroup>,
    pub symmetry_declarations: Vec<SymmetryDeclaration>,
    pub limitations: Vec<String>,
}

impl From<&BodyPlanFamily> for BodyPlanFamilyDefinition {
    fn from(v: &BodyPlanFamily) -> Self {
        Self {
            part_templates: v.part_templates.clone(),
            relation_rules: v.relation_rules.clone(),
            homology_groups: v.homology_groups.clone(),
            symmetry_declarations: v.symmetry_declarations.clone(),
            limitations: v.limitations.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartOccurrence {
    pub occurrence_id: Id,
    pub template_id: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationInstance {
    pub rule_id: Id,
    pub from_occurrence_id: Id,
    pub to_occurrence_id: Id,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymmetryPosition {
    pub declaration_id: Id,
    pub occurrence_id: Id,
    pub position: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralExpressionDefinition {
    pub family_id: Id,
    pub active_predicate_ids: Vec<Id>,
    pub occurrences: Vec<PartOccurrence>,
    pub relation_instances: Vec<RelationInstance>,
    pub symmetry_positions: Vec<SymmetryPosition>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralExpression {
    pub schema_version: u16,
    pub expression_id: Id,
    pub family_id: Id,
    pub active_predicate_ids: Vec<Id>,
    pub occurrences: Vec<PartOccurrence>,
    pub relation_instances: Vec<RelationInstance>,
    pub symmetry_positions: Vec<SymmetryPosition>,
    pub limitations: Vec<String>,
}

impl From<&StructuralExpression> for StructuralExpressionDefinition {
    fn from(v: &StructuralExpression) -> Self {
        Self {
            family_id: v.family_id,
            active_predicate_ids: v.active_predicate_ids.clone(),
            occurrences: v.occurrences.clone(),
            relation_instances: v.relation_instances.clone(),
            symmetry_positions: v.symmetry_positions.clone(),
            limitations: v.limitations.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    IndeterminateBudget,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

struct Validator {
    budget: u32,
    examined: u32,
    violations: Vec<Violation>,
    exhausted: bool,
}
impl Validator {
    fn new(budget: u32) -> Self {
        Self {
            budget,
            examined: 0,
            violations: vec![],
            exhausted: false,
        }
    }
    fn examine(&mut self) -> bool {
        if self.examined >= self.budget {
            self.exhausted = true;
            false
        } else {
            self.examined += 1;
            true
        }
    }
    fn reject(&mut self, code: &str, location: &str) {
        if self.examine() {
            self.violations.push(Violation {
                code: code.into(),
                location: location.into(),
            });
        }
    }
    fn check(&mut self, ok: bool, code: &str, location: &str) {
        if !self.examine() {
            return;
        }
        if !ok {
            self.violations.push(Violation {
                code: code.into(),
                location: location.into(),
            });
        }
    }
    fn finish(self) -> ValidationReport {
        if self.exhausted {
            ValidationReport {
                status: ValidationStatus::IndeterminateBudget,
                examined: self.examined,
                violations: vec![],
            }
        } else {
            let status = if self.violations.is_empty() {
                ValidationStatus::Valid
            } else {
                ValidationStatus::Invalid
            };
            ValidationReport {
                status,
                examined: self.examined,
                violations: self.violations,
            }
        }
    }
}

fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut h = Sha256::new();
    h.update(domain);
    h.update([0]);
    h.update(bytes);
    h.finalize().into()
}
fn nonzero(id: &Id) -> bool {
    *id != [0; 32]
}
pub fn id_from_u16(v: u16) -> Id {
    let mut id = [0; 32];
    id[30..].copy_from_slice(&v.to_be_bytes());
    id
}
pub fn hex(id: Id) -> String {
    id.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn optional_template(v: u16) -> PartTemplate {
    PartTemplate {
        template_id: id_from_u16(v),
        role_id: id_from_u16(v.wrapping_add(1000)),
        cardinality: Cardinality {
            minimum: 0,
            maximum: 1,
        },
        presence: PresenceRule::Unconditional,
    }
}

fn canonical_family(
    mut d: BodyPlanFamilyDefinition,
) -> Result<BodyPlanFamilyDefinition, BodyPlanError> {
    if d.part_templates.len() > MAX_PART_TEMPLATES {
        return Err(BodyPlanError::ResourceLimit("part templates"));
    }
    if d.relation_rules.len() > MAX_RELATION_RULES {
        return Err(BodyPlanError::ResourceLimit("relation rules"));
    }
    if d.homology_groups.len() > MAX_HOMOLOGY_GROUPS {
        return Err(BodyPlanError::ResourceLimit("homology groups"));
    }
    if d.symmetry_declarations.len() > MAX_SYMMETRY_DECLARATIONS {
        return Err(BodyPlanError::ResourceLimit("symmetry declarations"));
    }
    check_limitations(&d.limitations)?;
    if d.homology_groups
        .iter()
        .any(|group| group.member_template_ids.len() > MAX_PART_TEMPLATES)
    {
        return Err(BodyPlanError::ResourceLimit("homology members"));
    }
    if d.symmetry_declarations
        .iter()
        .any(|declaration| declaration.member_template_ids.len() > MAX_PART_TEMPLATES)
    {
        return Err(BodyPlanError::ResourceLimit("symmetry members"));
    }
    d.part_templates.sort_by_key(|x| x.template_id);
    d.relation_rules.sort_by_key(|x| x.rule_id);
    d.homology_groups.sort_by_key(|x| x.group_id);
    d.symmetry_declarations.sort_by_key(|x| x.declaration_id);
    for h in &mut d.homology_groups {
        h.member_template_ids.sort();
    }
    for s in &mut d.symmetry_declarations {
        s.member_template_ids.sort();
    }
    d.limitations.sort();
    unique(
        d.part_templates.iter().map(|x| x.template_id),
        "duplicate template",
    )?;
    unique(
        d.relation_rules.iter().map(|x| x.rule_id),
        "duplicate relation rule",
    )?;
    unique(
        d.homology_groups.iter().map(|x| x.group_id),
        "duplicate homology group",
    )?;
    unique(
        d.symmetry_declarations.iter().map(|x| x.declaration_id),
        "duplicate symmetry declaration",
    )?;
    for h in &d.homology_groups {
        unique(
            h.member_template_ids.iter().copied(),
            "duplicate homology member",
        )?;
    }
    for s in &d.symmetry_declarations {
        unique(
            s.member_template_ids.iter().copied(),
            "duplicate symmetry member",
        )?;
    }
    unique(d.limitations.iter(), "duplicate limitation")?;
    Ok(d)
}

fn canonical_expression(
    mut d: StructuralExpressionDefinition,
) -> Result<StructuralExpressionDefinition, BodyPlanError> {
    if d.active_predicate_ids.len() > MAX_ACTIVE_PREDICATES {
        return Err(BodyPlanError::ResourceLimit("active predicates"));
    }
    if d.occurrences.len() > MAX_OCCURRENCES {
        return Err(BodyPlanError::ResourceLimit("occurrences"));
    }
    if d.relation_instances.len() > MAX_RELATION_INSTANCES {
        return Err(BodyPlanError::ResourceLimit("relation instances"));
    }
    if d.symmetry_positions.len() > MAX_SYMMETRY_POSITIONS {
        return Err(BodyPlanError::ResourceLimit("symmetry positions"));
    }
    check_limitations(&d.limitations)?;
    d.active_predicate_ids.sort();
    d.occurrences.sort_by_key(|x| x.occurrence_id);
    d.relation_instances
        .sort_by_key(|x| (x.rule_id, x.from_occurrence_id, x.to_occurrence_id));
    d.symmetry_positions
        .sort_by_key(|x| (x.declaration_id, x.position, x.occurrence_id));
    d.limitations.sort();
    unique(
        d.active_predicate_ids.iter().copied(),
        "duplicate active predicate",
    )?;
    unique(
        d.occurrences.iter().map(|x| x.occurrence_id),
        "duplicate occurrence",
    )?;
    unique(
        d.relation_instances
            .iter()
            .map(|x| (x.rule_id, x.from_occurrence_id, x.to_occurrence_id)),
        "duplicate relation instance",
    )?;
    unique(
        d.symmetry_positions
            .iter()
            .map(|x| (x.declaration_id, x.occurrence_id)),
        "duplicate symmetry occurrence",
    )?;
    unique(
        d.symmetry_positions
            .iter()
            .map(|x| (x.declaration_id, x.position)),
        "duplicate symmetry position",
    )?;
    unique(d.limitations.iter(), "duplicate limitation")?;
    Ok(d)
}

fn unique<T: Ord>(
    items: impl Iterator<Item = T>,
    error: &'static str,
) -> Result<(), BodyPlanError> {
    let mut s = BTreeSet::new();
    for x in items {
        if !s.insert(x) {
            return Err(BodyPlanError::Invalid(error));
        }
    }
    Ok(())
}
fn check_limitations(v: &[String]) -> Result<(), BodyPlanError> {
    if v.len() > MAX_LIMITATIONS {
        return Err(BodyPlanError::ResourceLimit("limitations"));
    }
    if v.iter().any(|x| x.len() > MAX_LIMITATION_BYTES) {
        return Err(BodyPlanError::ResourceLimit("limitation bytes"));
    }
    Ok(())
}

fn invalid_preflight(code: &str, location: &str) -> ValidationReport {
    ValidationReport {
        status: ValidationStatus::Invalid,
        examined: 0,
        violations: vec![Violation {
            code: code.into(),
            location: location.into(),
        }],
    }
}

fn family_preflight(f: &BodyPlanFamily) -> Result<(), (&'static str, &'static str)> {
    if f.part_templates.len() > MAX_PART_TEMPLATES {
        return Err(("resource", "part_templates"));
    }
    if f.relation_rules.len() > MAX_RELATION_RULES {
        return Err(("resource", "relation_rules"));
    }
    if f.homology_groups.len() > MAX_HOMOLOGY_GROUPS {
        return Err(("resource", "homology_groups"));
    }
    if f.symmetry_declarations.len() > MAX_SYMMETRY_DECLARATIONS {
        return Err(("resource", "symmetry_declarations"));
    }
    if f.limitations.len() > MAX_LIMITATIONS
        || f.limitations.iter().any(|x| x.len() > MAX_LIMITATION_BYTES)
    {
        return Err(("resource", "limitations"));
    }
    if f.homology_groups
        .iter()
        .any(|group| group.member_template_ids.len() > MAX_PART_TEMPLATES)
    {
        return Err(("resource", "homology_members"));
    }
    if f.symmetry_declarations
        .iter()
        .any(|declaration| declaration.member_template_ids.len() > MAX_PART_TEMPLATES)
    {
        return Err(("resource", "symmetry_members"));
    }
    if serde_json::to_vec(f).map_or(true, |bytes| bytes.len() > MAX_CANONICAL_BYTES) {
        return Err(("resource", "canonical_family_bytes"));
    }
    Ok(())
}

fn expression_preflight(e: &StructuralExpression) -> Result<(), (&'static str, &'static str)> {
    if e.active_predicate_ids.len() > MAX_ACTIVE_PREDICATES {
        return Err(("resource", "active_predicates"));
    }
    if e.occurrences.len() > MAX_OCCURRENCES {
        return Err(("resource", "occurrences"));
    }
    if e.relation_instances.len() > MAX_RELATION_INSTANCES {
        return Err(("resource", "relation_instances"));
    }
    if e.symmetry_positions.len() > MAX_SYMMETRY_POSITIONS {
        return Err(("resource", "symmetry_positions"));
    }
    if e.limitations.len() > MAX_LIMITATIONS
        || e.limitations.iter().any(|x| x.len() > MAX_LIMITATION_BYTES)
    {
        return Err(("resource", "limitations"));
    }
    if serde_json::to_vec(e).map_or(true, |bytes| bytes.len() > MAX_CANONICAL_BYTES) {
        return Err(("resource", "canonical_expression_bytes"));
    }
    Ok(())
}

fn family_semantic_bytes(f: &BodyPlanFamily) -> Result<Vec<u8>, BodyPlanError> {
    if f.schema_version != CONTRACT_VERSION {
        return Err(BodyPlanError::Invalid("unsupported schema version"));
    }
    let canonical = canonical_family(BodyPlanFamilyDefinition::from(f))?;
    serde_json::to_vec(&(
        f.schema_version,
        &canonical.part_templates,
        &canonical.relation_rules,
        &canonical.homology_groups,
        &canonical.symmetry_declarations,
    ))
    .map_err(|e| BodyPlanError::Codec(e.to_string()))
}
fn expression_semantic_bytes(e: &StructuralExpression) -> Result<Vec<u8>, BodyPlanError> {
    if e.schema_version != CONTRACT_VERSION {
        return Err(BodyPlanError::Invalid("unsupported schema version"));
    }
    let canonical = canonical_expression(StructuralExpressionDefinition::from(e))?;
    serde_json::to_vec(&(
        e.schema_version,
        canonical.family_id,
        &canonical.active_predicate_ids,
        &canonical.occurrences,
        &canonical.relation_instances,
        &canonical.symmetry_positions,
    ))
    .map_err(|e| BodyPlanError::Codec(e.to_string()))
}

pub fn build_family(d: BodyPlanFamilyDefinition) -> Result<BodyPlanFamily, BodyPlanError> {
    let d = canonical_family(d)?;
    let mut f = BodyPlanFamily {
        schema_version: CONTRACT_VERSION,
        family_id: [0; 32],
        part_templates: d.part_templates,
        relation_rules: d.relation_rules,
        homology_groups: d.homology_groups,
        symmetry_declarations: d.symmetry_declarations,
        limitations: d.limitations,
    };
    f.family_id = hash(b"mindwarp.body-plan-family.v1", &family_semantic_bytes(&f)?);
    if validate_family(&f, MAX_VALIDATION_EXAMINATIONS).status != ValidationStatus::Valid {
        return Err(BodyPlanError::Invalid("invalid family"));
    }
    let record_bytes =
        serde_json::to_vec(&f).map_err(|error| BodyPlanError::Codec(error.to_string()))?;
    if record_bytes.len() > MAX_CANONICAL_BYTES {
        return Err(BodyPlanError::ResourceLimit("canonical family bytes"));
    }
    Ok(f)
}

pub fn build_expression(
    family: &BodyPlanFamily,
    d: StructuralExpressionDefinition,
) -> Result<StructuralExpression, BodyPlanError> {
    let d = canonical_expression(d)?;
    let mut e = StructuralExpression {
        schema_version: CONTRACT_VERSION,
        expression_id: [0; 32],
        family_id: d.family_id,
        active_predicate_ids: d.active_predicate_ids,
        occurrences: d.occurrences,
        relation_instances: d.relation_instances,
        symmetry_positions: d.symmetry_positions,
        limitations: d.limitations,
    };
    e.expression_id = hash(
        b"mindwarp.body-plan-expression.v1",
        &expression_semantic_bytes(&e)?,
    );
    if validate_expression(family, &e, MAX_VALIDATION_EXAMINATIONS).status
        != ValidationStatus::Valid
    {
        return Err(BodyPlanError::Invalid("invalid expression"));
    }
    if e.to_bytes()?.len() > MAX_CANONICAL_BYTES {
        return Err(BodyPlanError::ResourceLimit("canonical expression bytes"));
    }
    Ok(e)
}

impl BodyPlanFamily {
    pub fn fingerprint(&self) -> Result<Id, BodyPlanError> {
        Ok(hash(
            b"mindwarp.body-plan-family.v1",
            &family_semantic_bytes(self)?,
        ))
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, BodyPlanError> {
        if self.schema_version != CONTRACT_VERSION || self.fingerprint()? != self.family_id {
            return Err(BodyPlanError::Invalid("family identity mismatch"));
        }
        let d = canonical_family(BodyPlanFamilyDefinition::from(self))?;
        let mut c = BodyPlanFamily {
            schema_version: CONTRACT_VERSION,
            family_id: [0; 32],
            part_templates: d.part_templates,
            relation_rules: d.relation_rules,
            homology_groups: d.homology_groups,
            symmetry_declarations: d.symmetry_declarations,
            limitations: d.limitations,
        };
        c.family_id = self.family_id;
        let b = serde_json::to_vec(&c).map_err(|e| BodyPlanError::Codec(e.to_string()))?;
        if b.len() > MAX_CANONICAL_BYTES {
            Err(BodyPlanError::ResourceLimit("canonical family bytes"))
        } else {
            Ok(b)
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BodyPlanError> {
        if bytes.len() > MAX_CANONICAL_BYTES {
            return Err(BodyPlanError::ResourceLimit("canonical family bytes"));
        }
        let v: Self =
            serde_json::from_slice(bytes).map_err(|e| BodyPlanError::Codec(e.to_string()))?;
        if v.schema_version != CONTRACT_VERSION
            || validate_family(&v, MAX_VALIDATION_EXAMINATIONS).status != ValidationStatus::Valid
            || v.to_bytes()?.as_slice() != bytes
        {
            return Err(BodyPlanError::NonCanonical);
        }
        Ok(v)
    }
}

impl StructuralExpression {
    pub fn fingerprint(&self) -> Result<Id, BodyPlanError> {
        Ok(hash(
            b"mindwarp.body-plan-expression.v1",
            &expression_semantic_bytes(self)?,
        ))
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, BodyPlanError> {
        if self.schema_version != CONTRACT_VERSION || self.fingerprint()? != self.expression_id {
            return Err(BodyPlanError::Invalid("expression identity mismatch"));
        }
        let d = canonical_expression(StructuralExpressionDefinition::from(self))?;
        let mut c = StructuralExpression {
            schema_version: CONTRACT_VERSION,
            expression_id: [0; 32],
            family_id: d.family_id,
            active_predicate_ids: d.active_predicate_ids,
            occurrences: d.occurrences,
            relation_instances: d.relation_instances,
            symmetry_positions: d.symmetry_positions,
            limitations: d.limitations,
        };
        c.expression_id = self.expression_id;
        let b = serde_json::to_vec(&c).map_err(|e| BodyPlanError::Codec(e.to_string()))?;
        if b.len() > MAX_CANONICAL_BYTES {
            Err(BodyPlanError::ResourceLimit("canonical expression bytes"))
        } else {
            Ok(b)
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BodyPlanError> {
        if bytes.len() > MAX_CANONICAL_BYTES {
            return Err(BodyPlanError::ResourceLimit("canonical expression bytes"));
        }
        let v: Self =
            serde_json::from_slice(bytes).map_err(|e| BodyPlanError::Codec(e.to_string()))?;
        if v.schema_version != CONTRACT_VERSION
            || v.fingerprint()? != v.expression_id
            || v.to_bytes()?.as_slice() != bytes
        {
            return Err(BodyPlanError::NonCanonical);
        }
        Ok(v)
    }
}

pub fn validate_family(f: &BodyPlanFamily, budget: u32) -> ValidationReport {
    if let Err((code, location)) = family_preflight(f) {
        return invalid_preflight(code, location);
    }
    let mut v = Validator::new(budget);
    v.check(f.schema_version == CONTRACT_VERSION, "schema", "family");
    v.check(
        f.fingerprint().ok() == Some(f.family_id),
        "identity",
        "family",
    );
    v.check(
        !f.part_templates.is_empty() && f.part_templates.len() <= MAX_PART_TEMPLATES,
        "resource",
        "part_templates",
    );
    let templates: BTreeSet<_> = f.part_templates.iter().map(|x| x.template_id).collect();
    v.check(
        templates.len() == f.part_templates.len(),
        "duplicate",
        "part_templates",
    );
    for (i, t) in f.part_templates.iter().enumerate() {
        v.check(
            nonzero(&t.template_id) && nonzero(&t.role_id),
            "zero_id",
            &format!("part_templates[{i}]"),
        );
        v.check(
            t.cardinality.minimum <= t.cardinality.maximum,
            "cardinality",
            &format!("part_templates[{i}]"),
        );
        if let PresenceRule::Conditional { predicate_id } = t.presence {
            v.check(
                nonzero(&predicate_id),
                "zero_id",
                &format!("part_templates[{i}].predicate"),
            );
        }
    }
    let rules: BTreeSet<_> = f.relation_rules.iter().map(|x| x.rule_id).collect();
    v.check(
        rules.len() == f.relation_rules.len() && f.relation_rules.len() <= MAX_RELATION_RULES,
        "duplicate_or_resource",
        "relation_rules",
    );
    for (i, r) in f.relation_rules.iter().enumerate() {
        v.check(
            nonzero(&r.rule_id)
                && templates.contains(&r.from_template_id)
                && templates.contains(&r.to_template_id),
            "unresolved",
            &format!("relation_rules[{i}]"),
        );
        v.check(
            r.from_degree.minimum <= r.from_degree.maximum
                && r.to_degree.minimum <= r.to_degree.maximum,
            "cardinality",
            &format!("relation_rules[{i}]"),
        );
    }
    let homology_ids: BTreeSet<_> = f.homology_groups.iter().map(|x| x.group_id).collect();
    v.check(
        homology_ids.len() == f.homology_groups.len(),
        "duplicate",
        "homology_groups",
    );
    for (i, h) in f.homology_groups.iter().enumerate() {
        let members: BTreeSet<_> = h.member_template_ids.iter().collect();
        v.check(
            nonzero(&h.group_id)
                && members.len() == h.member_template_ids.len()
                && members.len() >= 2
                && members.iter().all(|x| templates.contains(*x)),
            "homology",
            &format!("homology_groups[{i}]"),
        );
    }
    let symmetry_ids: BTreeSet<_> = f
        .symmetry_declarations
        .iter()
        .map(|x| x.declaration_id)
        .collect();
    v.check(
        symmetry_ids.len() == f.symmetry_declarations.len(),
        "duplicate",
        "symmetry_declarations",
    );
    for (i, s) in f.symmetry_declarations.iter().enumerate() {
        let members: BTreeSet<_> = s.member_template_ids.iter().collect();
        v.check(
            nonzero(&s.declaration_id)
                && members.len() == s.member_template_ids.len()
                && members.iter().all(|x| templates.contains(*x)),
            "symmetry_members",
            &format!("symmetry_declarations[{i}]"),
        );
        let ok = match s.pattern {
            SymmetryPattern::NoDeclaredSymmetry => members.is_empty(),
            SymmetryPattern::Bilateral => !members.is_empty(),
            SymmetryPattern::Radial { minimum, maximum }
            | SymmetryPattern::Serial { minimum, maximum } => minimum > 0 && minimum <= maximum,
            SymmetryPattern::OtherDeclared {
                pattern_ref,
                positions,
            } => nonzero(&pattern_ref) && positions > 0,
        };
        v.check(
            ok,
            "symmetry_pattern",
            &format!("symmetry_declarations[{i}]"),
        );
    }
    v.finish()
}

pub fn validate_expression(
    f: &BodyPlanFamily,
    e: &StructuralExpression,
    budget: u32,
) -> ValidationReport {
    if let Err((code, location)) = expression_preflight(e) {
        return invalid_preflight(code, location);
    }
    let family_report = validate_family(f, budget);
    if family_report.status != ValidationStatus::Valid {
        return family_report;
    }
    let family_examined = family_report.examined;
    let mut v = Validator::new(budget.saturating_sub(family_examined));
    v.check(
        e.schema_version == CONTRACT_VERSION
            && e.family_id == f.family_id
            && e.fingerprint().ok() == Some(e.expression_id),
        "identity",
        "expression",
    );
    let templates: BTreeMap<_, _> = f
        .part_templates
        .iter()
        .map(|x| (x.template_id, x))
        .collect();
    let occurrences: BTreeMap<_, _> = e.occurrences.iter().map(|x| (x.occurrence_id, x)).collect();
    v.check(
        !e.occurrences.is_empty() && occurrences.len() == e.occurrences.len(),
        "duplicate_or_resource",
        "occurrences",
    );
    for (i, o) in e.occurrences.iter().enumerate() {
        v.check(
            nonzero(&o.occurrence_id) && templates.contains_key(&o.template_id),
            "unresolved",
            &format!("occurrences[{i}]"),
        );
    }
    let active: BTreeSet<_> = e.active_predicate_ids.iter().copied().collect();
    let declared_predicates: BTreeSet<_> = f
        .part_templates
        .iter()
        .filter_map(|template| match template.presence {
            PresenceRule::Conditional { predicate_id } => Some(predicate_id),
            PresenceRule::Unconditional => None,
        })
        .collect();
    v.check(
        active.len() == e.active_predicate_ids.len()
            && active.iter().all(nonzero)
            && active.is_subset(&declared_predicates),
        "duplicate_or_unresolved",
        "active_predicates",
    );
    for t in &f.part_templates {
        let count = e
            .occurrences
            .iter()
            .filter(|o| o.template_id == t.template_id)
            .count() as u16;
        let enabled = match t.presence {
            PresenceRule::Unconditional => true,
            PresenceRule::Conditional { predicate_id } => active.contains(&predicate_id),
        };
        let ok = if enabled {
            count >= t.cardinality.minimum && count <= t.cardinality.maximum
        } else {
            count == 0
        };
        v.check(ok, "presence_cardinality", &hex(t.template_id));
    }
    let rules: BTreeMap<_, _> = f.relation_rules.iter().map(|x| (x.rule_id, x)).collect();
    let relation_keys: BTreeSet<_> = e
        .relation_instances
        .iter()
        .map(|x| (x.rule_id, x.from_occurrence_id, x.to_occurrence_id))
        .collect();
    v.check(
        relation_keys.len() == e.relation_instances.len(),
        "duplicate",
        "relation_instances",
    );
    let mut edges = Vec::new();
    let mut incoming_containment: BTreeMap<Id, usize> = BTreeMap::new();
    let mut outgoing_degree: BTreeMap<(Id, Id), u16> = BTreeMap::new();
    let mut incoming_degree: BTreeMap<(Id, Id), u16> = BTreeMap::new();
    for (i, ri) in e.relation_instances.iter().enumerate() {
        let resolved = rules.get(&ri.rule_id).and_then(|r| {
            Some((
                r,
                occurrences.get(&ri.from_occurrence_id)?,
                occurrences.get(&ri.to_occurrence_id)?,
            ))
        });
        if let Some((r, from, to)) = resolved {
            v.check(
                from.template_id == r.from_template_id
                    && to.template_id == r.to_template_id
                    && from.occurrence_id != to.occurrence_id,
                "relation_endpoint",
                &format!("relation_instances[{i}]"),
            );
            edges.push((from.occurrence_id, to.occurrence_id, r.kind));
            *outgoing_degree
                .entry((r.rule_id, from.occurrence_id))
                .or_default() += 1;
            *incoming_degree
                .entry((r.rule_id, to.occurrence_id))
                .or_default() += 1;
            if r.kind == RelationKind::Containment {
                *incoming_containment.entry(to.occurrence_id).or_default() += 1;
            }
        } else {
            v.reject("unresolved_relation", &format!("relation_instances[{i}]"));
        }
    }
    for (id, count) in &incoming_containment {
        v.check(*count <= 1, "multiple_containers", &hex(*id));
    }
    for r in &f.relation_rules {
        let from_occurrences = e
            .occurrences
            .iter()
            .filter(|o| o.template_id == r.from_template_id)
            .count();
        let to_occurrences = e
            .occurrences
            .iter()
            .filter(|o| o.template_id == r.to_template_id)
            .count();
        let from_nonzero = outgoing_degree
            .keys()
            .filter(|(rule_id, _)| *rule_id == r.rule_id)
            .count();
        let to_nonzero = incoming_degree
            .keys()
            .filter(|(rule_id, _)| *rule_id == r.rule_id)
            .count();
        v.check(
            r.from_degree.minimum == 0 || from_nonzero == from_occurrences,
            "from_degree_minimum",
            &hex(r.rule_id),
        );
        v.check(
            r.to_degree.minimum == 0 || to_nonzero == to_occurrences,
            "to_degree_minimum",
            &hex(r.rule_id),
        );
    }
    for ((rule_id, occurrence_id), degree) in &outgoing_degree {
        if let Some(rule) = rules.get(rule_id) {
            v.check(
                *degree >= rule.from_degree.minimum && *degree <= rule.from_degree.maximum,
                "from_degree",
                &hex(*occurrence_id),
            );
        }
    }
    for ((rule_id, occurrence_id), degree) in &incoming_degree {
        if let Some(rule) = rules.get(rule_id) {
            v.check(
                *degree >= rule.to_degree.minimum && *degree <= rule.to_degree.maximum,
                "to_degree",
                &hex(*occurrence_id),
            );
        }
    }
    v.check(
        containment_acyclic(occurrences.keys().copied(), &edges),
        "containment_cycle",
        "relations",
    );
    v.check(
        connected(occurrences.keys().copied(), &edges),
        "disconnected",
        "relations",
    );
    for s in &f.symmetry_declarations {
        let member_occ: BTreeSet<_> = e
            .occurrences
            .iter()
            .filter(|o| s.member_template_ids.contains(&o.template_id))
            .map(|o| o.occurrence_id)
            .collect();
        let positions: Vec<_> = e
            .symmetry_positions
            .iter()
            .filter(|p| p.declaration_id == s.declaration_id)
            .collect();
        let occurrence_set: BTreeSet<_> = positions.iter().map(|p| p.occurrence_id).collect();
        let number_set: BTreeSet<_> = positions.iter().map(|p| p.position).collect();
        let n = positions.len() as u16;
        let contiguous = number_set.iter().copied().eq(0..n);
        let count_ok = match s.pattern {
            SymmetryPattern::NoDeclaredSymmetry => n == 0,
            SymmetryPattern::Bilateral => n == 2,
            SymmetryPattern::Radial { minimum, maximum }
            | SymmetryPattern::Serial { minimum, maximum } => n >= minimum && n <= maximum,
            SymmetryPattern::OtherDeclared { positions, .. } => n == positions,
        };
        v.check(
            occurrence_set == member_occ
                && occurrence_set.len() == positions.len()
                && contiguous
                && count_ok,
            "symmetry_positions",
            &hex(s.declaration_id),
        );
    }
    for p in &e.symmetry_positions {
        v.check(
            f.symmetry_declarations
                .iter()
                .any(|s| s.declaration_id == p.declaration_id)
                && occurrences.contains_key(&p.occurrence_id),
            "unresolved_symmetry",
            "symmetry_positions",
        );
    }
    let mut report = v.finish();
    report.examined = report.examined.saturating_add(family_examined);
    report
}

fn containment_acyclic(nodes: impl Iterator<Item = Id>, edges: &[(Id, Id, RelationKind)]) -> bool {
    let mut indegree: BTreeMap<Id, usize> = nodes.map(|x| (x, 0)).collect();
    for (_, b, k) in edges {
        if *k == RelationKind::Containment {
            *indegree.entry(*b).or_default() += 1;
        }
    }
    let mut q: VecDeque<_> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(x, _)| *x)
        .collect();
    let mut seen = 0;
    while let Some(a) = q.pop_front() {
        seen += 1;
        for (_, b, _k) in edges
            .iter()
            .filter(|(x, _, k)| *x == a && *k == RelationKind::Containment)
        {
            let d = indegree.get_mut(b).unwrap();
            *d -= 1;
            if *d == 0 {
                q.push_back(*b)
            }
        }
    }
    seen == indegree.len()
}
fn connected(nodes: impl Iterator<Item = Id>, edges: &[(Id, Id, RelationKind)]) -> bool {
    let all: BTreeSet<_> = nodes.collect();
    if all.len() <= 1 {
        return true;
    }
    let Some(start) = all.iter().next().copied() else {
        return true;
    };
    let mut seen = BTreeSet::from([start]);
    let mut q = VecDeque::from([start]);
    while let Some(a) = q.pop_front() {
        for (x, y, _) in edges {
            let b = if *x == a {
                Some(*y)
            } else if *y == a {
                Some(*x)
            } else {
                None
            };
            if let Some(b) = b
                && seen.insert(b)
            {
                q.push_back(b)
            }
        }
    }
    seen == all
}

pub fn validate_body_plan_ref(
    expected: Id,
    family: &BodyPlanFamily,
    budget: u32,
) -> ValidationReport {
    let mut report = validate_family(family, budget);
    if report.status != ValidationStatus::Valid {
        return report;
    }
    if report.examined >= budget {
        report.status = ValidationStatus::IndeterminateBudget;
        report.violations.clear();
        return report;
    }
    report.examined += 1;
    if !nonzero(&expected)
        || family.fingerprint().ok() != Some(family.family_id)
        || expected != family.family_id
    {
        report.status = ValidationStatus::Invalid;
        report.violations.push(Violation {
            code: "body_plan_ref".into(),
            location: "family_id".into(),
        });
    }
    report
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixturePair {
    pub family: BodyPlanFamily,
    pub expression: StructuralExpression,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadialFixture {
    pub family: BodyPlanFamily,
    pub five: StructuralExpression,
    pub seven: StructuralExpression,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceFixtures {
    pub humanoid: FixturePair,
    pub radial: RadialFixture,
    pub withheld: FixturePair,
}

fn occurrence(id: u16, t: u16) -> PartOccurrence {
    PartOccurrence {
        occurrence_id: id_from_u16(id),
        template_id: id_from_u16(t),
    }
}
fn relation(rule: u16, from: u16, to: u16) -> RelationInstance {
    RelationInstance {
        rule_id: id_from_u16(rule),
        from_occurrence_id: id_from_u16(from),
        to_occurrence_id: id_from_u16(to),
    }
}
fn position(decl: u16, occ: u16, pos: u16) -> SymmetryPosition {
    SymmetryPosition {
        declaration_id: id_from_u16(decl),
        occurrence_id: id_from_u16(occ),
        position: pos,
    }
}
fn required(t: u16, role: u16) -> PartTemplate {
    PartTemplate {
        template_id: id_from_u16(t),
        role_id: id_from_u16(role),
        cardinality: Cardinality {
            minimum: 1,
            maximum: 1,
        },
        presence: PresenceRule::Unconditional,
    }
}
fn rule(
    id: u16,
    from: u16,
    to: u16,
    kind: RelationKind,
    from_degree: Cardinality,
    to_degree: Cardinality,
) -> RelationRule {
    RelationRule {
        rule_id: id_from_u16(id),
        kind,
        from_template_id: id_from_u16(from),
        to_template_id: id_from_u16(to),
        from_degree,
        to_degree,
    }
}

pub fn reference_fixtures() -> Result<ReferenceFixtures, BodyPlanError> {
    let humanoid_family = build_family(BodyPlanFamilyDefinition {
        part_templates: vec![
            required(1, 101),
            required(2, 102),
            required(3, 103),
            optional_template(4),
            PartTemplate {
                template_id: id_from_u16(5),
                role_id: id_from_u16(105),
                cardinality: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                presence: PresenceRule::Conditional {
                    predicate_id: id_from_u16(501),
                },
            },
        ],
        relation_rules: vec![
            rule(
                11,
                1,
                2,
                RelationKind::Containment,
                Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            ),
            rule(
                12,
                1,
                3,
                RelationKind::Containment,
                Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            ),
            rule(
                13,
                1,
                4,
                RelationKind::Containment,
                Cardinality {
                    minimum: 0,
                    maximum: 1,
                },
                Cardinality {
                    minimum: 0,
                    maximum: 1,
                },
            ),
            rule(
                14,
                1,
                5,
                RelationKind::Containment,
                Cardinality {
                    minimum: 0,
                    maximum: 1,
                },
                Cardinality {
                    minimum: 0,
                    maximum: 1,
                },
            ),
        ],
        homology_groups: vec![HomologyGroup {
            group_id: id_from_u16(21),
            member_template_ids: vec![id_from_u16(2), id_from_u16(3)],
        }],
        symmetry_declarations: vec![SymmetryDeclaration {
            declaration_id: id_from_u16(31),
            pattern: SymmetryPattern::Bilateral,
            member_template_ids: vec![id_from_u16(2), id_from_u16(3)],
        }],
        limitations: vec!["coordinate-free synthetic structural control".into()],
    })?;
    let humanoid_expression = build_expression(
        &humanoid_family,
        StructuralExpressionDefinition {
            family_id: humanoid_family.family_id,
            active_predicate_ids: vec![id_from_u16(501)],
            occurrences: vec![
                occurrence(101, 1),
                occurrence(102, 2),
                occurrence(103, 3),
                occurrence(105, 5),
            ],
            relation_instances: vec![
                relation(11, 101, 102),
                relation(12, 101, 103),
                relation(14, 101, 105),
            ],
            symmetry_positions: vec![position(31, 102, 0), position(31, 103, 1)],
            limitations: vec![],
        },
    )?;
    let radial_family = build_family(BodyPlanFamilyDefinition {
        part_templates: vec![
            required(10, 110),
            PartTemplate {
                template_id: id_from_u16(11),
                role_id: id_from_u16(111),
                cardinality: Cardinality {
                    minimum: 5,
                    maximum: 7,
                },
                presence: PresenceRule::Conditional {
                    predicate_id: id_from_u16(511),
                },
            },
        ],
        relation_rules: vec![rule(
            41,
            10,
            11,
            RelationKind::StructuralConnection,
            Cardinality {
                minimum: 5,
                maximum: 7,
            },
            Cardinality {
                minimum: 1,
                maximum: 1,
            },
        )],
        homology_groups: vec![],
        symmetry_declarations: vec![SymmetryDeclaration {
            declaration_id: id_from_u16(51),
            pattern: SymmetryPattern::Radial {
                minimum: 5,
                maximum: 7,
            },
            member_template_ids: vec![id_from_u16(11)],
        }],
        limitations: vec![],
    })?;
    let radial_expression = |count: u16| {
        let mut occurrences = vec![occurrence(200, 10)];
        let mut relations = vec![];
        let mut positions = vec![];
        for n in 0..count {
            occurrences.push(occurrence(201 + n, 11));
            relations.push(relation(41, 200, 201 + n));
            positions.push(position(51, 201 + n, n));
        }
        build_expression(
            &radial_family,
            StructuralExpressionDefinition {
                family_id: radial_family.family_id,
                active_predicate_ids: vec![id_from_u16(511)],
                occurrences,
                relation_instances: relations,
                symmetry_positions: positions,
                limitations: vec![],
            },
        )
    };
    let five = radial_expression(5)?;
    let seven = radial_expression(7)?;
    let withheld_family = build_family(BodyPlanFamilyDefinition {
        part_templates: vec![PartTemplate {
            template_id: id_from_u16(20),
            role_id: id_from_u16(120),
            cardinality: Cardinality {
                minimum: 3,
                maximum: 3,
            },
            presence: PresenceRule::Unconditional,
        }],
        relation_rules: vec![rule(
            61,
            20,
            20,
            RelationKind::StructuralConnection,
            Cardinality {
                minimum: 0,
                maximum: 1,
            },
            Cardinality {
                minimum: 0,
                maximum: 1,
            },
        )],
        homology_groups: vec![],
        symmetry_declarations: vec![SymmetryDeclaration {
            declaration_id: id_from_u16(71),
            pattern: SymmetryPattern::Serial {
                minimum: 3,
                maximum: 3,
            },
            member_template_ids: vec![id_from_u16(20)],
        }],
        limitations: vec!["withheld negative-transfer control".into()],
    })?;
    let withheld_expression = build_expression(
        &withheld_family,
        StructuralExpressionDefinition {
            family_id: withheld_family.family_id,
            active_predicate_ids: vec![],
            occurrences: vec![
                occurrence(301, 20),
                occurrence(302, 20),
                occurrence(303, 20),
            ],
            relation_instances: vec![relation(61, 301, 302), relation(61, 302, 303)],
            symmetry_positions: vec![
                position(71, 301, 0),
                position(71, 302, 1),
                position(71, 303, 2),
            ],
            limitations: vec![],
        },
    )?;
    Ok(ReferenceFixtures {
        humanoid: FixturePair {
            family: humanoid_family,
            expression: humanoid_expression,
        },
        radial: RadialFixture {
            family: radial_family,
            five,
            seven,
        },
        withheld: FixturePair {
            family: withheld_family,
            expression: withheld_expression,
        },
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BodyPlanReferenceReceipt {
    pub schema_version: u16,
    pub fixture_suite_id: Id,
    pub humanoid_family_id: Id,
    pub radial_family_id: Id,
    pub radial_expression_ids: Vec<Id>,
    pub withheld_family_id: Id,
    pub hostile_registry_digest: Id,
    pub validation_examinations: u32,
    pub capabilities: Vec<String>,
    pub approved: bool,
    pub promoted: bool,
    pub runtime_authority: bool,
    pub geometry_truth: bool,
    pub biology_truth: bool,
}
impl BodyPlanReferenceReceipt {
    pub fn to_bytes(&self) -> Result<Vec<u8>, BodyPlanError> {
        serde_json::to_vec(self).map_err(|error| BodyPlanError::Codec(error.to_string()))
    }
    pub fn fingerprint(&self) -> Result<Id, BodyPlanError> {
        Ok(hash(
            b"mindwarp.body-plan-reference-receipt.v1",
            &self.to_bytes()?,
        ))
    }
}
pub fn reference_proof_receipt() -> Result<BodyPlanReferenceReceipt, BodyPlanError> {
    let f = reference_fixtures()?;
    let mut expressions = vec![f.radial.five.expression_id, f.radial.seven.expression_id];
    expressions.sort();
    let suite = serde_json::to_vec(&(
        f.humanoid.family.family_id,
        f.radial.family.family_id,
        &expressions,
        f.withheld.family.family_id,
    ))
    .map_err(|e| BodyPlanError::Codec(e.to_string()))?;
    let validation_examinations = [
        validate_family(&f.humanoid.family, MAX_VALIDATION_EXAMINATIONS),
        validate_expression(
            &f.humanoid.family,
            &f.humanoid.expression,
            MAX_VALIDATION_EXAMINATIONS,
        ),
        validate_family(&f.radial.family, MAX_VALIDATION_EXAMINATIONS),
        validate_expression(
            &f.radial.family,
            &f.radial.five,
            MAX_VALIDATION_EXAMINATIONS,
        ),
        validate_expression(
            &f.radial.family,
            &f.radial.seven,
            MAX_VALIDATION_EXAMINATIONS,
        ),
        validate_family(&f.withheld.family, MAX_VALIDATION_EXAMINATIONS),
        validate_expression(
            &f.withheld.family,
            &f.withheld.expression,
            MAX_VALIDATION_EXAMINATIONS,
        ),
    ]
    .iter()
    .map(|report| report.examined)
    .sum();
    let hostile_registry = [
        "C6-H200", "C6-H201", "C6-H202", "C6-H203", "C6-H204", "C6-H205",
    ];
    Ok(BodyPlanReferenceReceipt {
        schema_version: CONTRACT_VERSION,
        fixture_suite_id: hash(b"mindwarp.body-plan-fixture-suite.v1", &suite),
        humanoid_family_id: f.humanoid.family.family_id,
        radial_family_id: f.radial.family.family_id,
        radial_expression_ids: expressions,
        withheld_family_id: f.withheld.family.family_id,
        hostile_registry_digest: hash(
            b"mindwarp.body-plan-hostile-registry.v1",
            &serde_json::to_vec(&hostile_registry)
                .map_err(|error| BodyPlanError::Codec(error.to_string()))?,
        ),
        validation_examinations,
        capabilities: vec![],
        approved: false,
        promoted: false,
        runtime_authority: false,
        geometry_truth: false,
        biology_truth: false,
    })
}
