//! Capability-free typed subject identities between C6 structure and later biology.
//!
//! These records distinguish lineage, reusable form, unresolved species candidate,
//! stable individual, mutable subject binding, population identity, and an exact C4
//! lifecycle/history target. They grant no membership, ancestry, viability, runtime,
//! representation, sex, dimorphism, reproduction, or development authority.

use body_plan_structure::{
    BodyPlanError, BodyPlanFamily, StructuralExpression, ValidationStatus, validate_expression,
};
use derived_world_rules::{CausalWorldPacket, WorldGenerationInput};
use entity_lifecycle::{AgeCohort, LifecycleMode, LifecycleState, validate_state};
use entity_lifecycle_history_binding::{
    AmbientCohortBindingV1, BindingError, reconstruct_from_reference_state,
};
use hierarchy_history::{BaselineManifest, HierarchyHistoryError, recover_known_good_prefix};
use macro_lineage_binding::{
    LineageError, MacroLineageCandidate, validate_body_plan_binding,
    validate_macro_lineage_candidate,
};
use niche_graph_binding::EnvironmentalOpportunityGraph;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

pub type Id = [u8; 32];
pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_RECORD_BYTES: usize = 4_096;
pub const MAX_RECEIPT_BYTES: usize = 32_768;
pub const MAX_IDENTITY_EXAMINATIONS: u32 = 2_048;
pub const MAX_PERSON_FORM_GROUNDINGS: usize = 5;
pub const MAX_C4_RECOVERY_RECORDS: u32 = 1_024;

const LINEAGE_DOMAIN: &str = "mindwarp/c6-lineage-subject-ref/v1";
const FORM_DOMAIN: &str = "mindwarp/c6-organism-form-template/v1";
const SPECIES_DOMAIN: &str = "mindwarp/c6-species-candidate/v1";
const INDIVIDUAL_DOMAIN: &str = "mindwarp/c6-individual-identity/v1";
const SUBJECT_BINDING_DOMAIN: &str = "mindwarp/c6-individual-subject-binding/v1";
const POPULATION_DOMAIN: &str = "mindwarp/c6-population-identity/v1";
const LIFECYCLE_DOMAIN: &str = "mindwarp/c6-lifecycle-history-subject-binding/v1";
const RECEIPT_DOMAIN: &str = "mindwarp/c6-organism-subject-reference-receipt/v1";

#[derive(Debug)]
pub enum IdentityError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
    ResourceLimit(&'static str),
    IndeterminateBudget,
    Lineage(LineageError),
    BodyPlan(BodyPlanError),
    Lifecycle(entity_lifecycle::LifecycleError),
    LifecycleBinding(BindingError),
    History(HierarchyHistoryError),
}

impl From<LineageError> for IdentityError {
    fn from(value: LineageError) -> Self {
        Self::Lineage(value)
    }
}
impl From<BodyPlanError> for IdentityError {
    fn from(value: BodyPlanError) -> Self {
        Self::BodyPlan(value)
    }
}
impl From<BindingError> for IdentityError {
    fn from(value: BindingError) -> Self {
        Self::LifecycleBinding(value)
    }
}
impl From<HierarchyHistoryError> for IdentityError {
    fn from(value: HierarchyHistoryError) -> Self {
        Self::History(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Unresolved,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalLifecycleMode {
    Ambient,
    Tracked,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalAgeCohort {
    Young,
    Juvenile,
    Adult,
    Elderly,
}

impl From<LifecycleMode> for CanonicalLifecycleMode {
    fn from(value: LifecycleMode) -> Self {
        match value {
            LifecycleMode::Ambient => Self::Ambient,
            LifecycleMode::Tracked => Self::Tracked,
        }
    }
}
impl From<AgeCohort> for CanonicalAgeCohort {
    fn from(value: AgeCohort) -> Self {
        match value {
            AgeCohort::Young => Self::Young,
            AgeCohort::Juvenile => Self::Juvenile,
            AgeCohort::Adult => Self::Adult,
            AgeCohort::Elderly => Self::Elderly,
        }
    }
}
impl From<CanonicalLifecycleMode> for LifecycleMode {
    fn from(value: CanonicalLifecycleMode) -> Self {
        match value {
            CanonicalLifecycleMode::Ambient => Self::Ambient,
            CanonicalLifecycleMode::Tracked => Self::Tracked,
        }
    }
}
impl From<CanonicalAgeCohort> for AgeCohort {
    fn from(value: CanonicalAgeCohort) -> Self {
        match value {
            CanonicalAgeCohort::Young => Self::Young,
            CanonicalAgeCohort::Juvenile => Self::Juvenile,
            CanonicalAgeCohort::Adult => Self::Adult,
            CanonicalAgeCohort::Elderly => Self::Elderly,
        }
    }
}

mod id_hex {
    use super::Id;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(id: &Id, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&id.iter().map(|b| format!("{b:02x}")).collect::<String>())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Id, D::Error> {
        let text = String::deserialize(deserializer)?;
        if text.len() != 64
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(serde::de::Error::custom(
                "expected 64 lowercase hexadecimal characters",
            ));
        }
        let mut id = [0; 32];
        for (index, byte) in id.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16)
                .map_err(serde::de::Error::custom)?;
        }
        Ok(id)
    }
}

mod optional_id_hex {
    use super::{Id, id_hex};
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(id: &Option<Id>, serializer: S) -> Result<S::Ok, S::Error> {
        match id {
            Some(value) => id_hex::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Id>, D::Error> {
        let value = Option::<String>::deserialize(deserializer)?;
        value
            .map(|text| {
                if text.len() != 64
                    || !text
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                {
                    return Err(serde::de::Error::custom("expected lowercase optional id"));
                }
                let mut id = [0; 32];
                for (index, byte) in id.iter_mut().enumerate() {
                    *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16)
                        .map_err(serde::de::Error::custom)?;
                }
                Ok(id)
            })
            .transpose()
    }
}

mod id_array2_hex {
    use super::{Id, hex};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(ids: &[Id; 2], serializer: S) -> Result<S::Ok, S::Error> {
        [hex(ids[0]), hex(ids[1])].serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[Id; 2], D::Error> {
        let values = <[String; 2]>::deserialize(deserializer)?;
        Ok([parse(&values[0])?, parse(&values[1])?])
    }
    fn parse<E: serde::de::Error>(text: &str) -> Result<Id, E> {
        if text.len() != 64
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(E::custom("lowercase id"));
        }
        let mut id = [0; 32];
        for (i, b) in id.iter_mut().enumerate() {
            *b = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).map_err(E::custom)?;
        }
        Ok(id)
    }
}
mod id_array3_hex {
    use super::{Id, hex};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(ids: &[Id; 3], serializer: S) -> Result<S::Ok, S::Error> {
        [hex(ids[0]), hex(ids[1]), hex(ids[2])].serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[Id; 3], D::Error> {
        let v = <[String; 3]>::deserialize(deserializer)?;
        Ok([parse(&v[0])?, parse(&v[1])?, parse(&v[2])?])
    }
    fn parse<E: serde::de::Error>(text: &str) -> Result<Id, E> {
        if text.len() != 64
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(E::custom("lowercase id"));
        }
        let mut id = [0; 32];
        for (i, b) in id.iter_mut().enumerate() {
            *b = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).map_err(E::custom)?;
        }
        Ok(id)
    }
}

fn hex(id: Id) -> String {
    id.iter().map(|b| format!("{b:02x}")).collect()
}
fn nonzero(id: Id) -> Result<(), IdentityError> {
    if id == [0; 32] {
        Err(IdentityError::Invalid("zero identity"))
    } else {
        Ok(())
    }
}
fn world(world_packet_id: &str) -> Result<(), IdentityError> {
    if world_packet_id.is_empty() || world_packet_id.len() > 256 {
        Err(IdentityError::Invalid("world packet id"))
    } else {
        Ok(())
    }
}
fn budget(examinations: u32) -> Result<(), IdentityError> {
    if examinations == 0 || examinations > MAX_IDENTITY_EXAMINATIONS {
        Err(IdentityError::IndeterminateBudget)
    } else {
        Ok(())
    }
}
fn validate_body_budget(examinations: u32) -> Result<(), IdentityError> {
    if examinations == 0 || examinations > body_plan_structure::MAX_VALIDATION_EXAMINATIONS {
        Err(IdentityError::IndeterminateBudget)
    } else {
        Ok(())
    }
}
fn hash_json(domain: &str, value: serde_json::Value) -> Result<Id, IdentityError> {
    let bytes = serde_json::to_vec(&value).map_err(|e| IdentityError::Codec(e.to_string()))?;
    let mut hash = Sha256::new();
    hash.update(domain.as_bytes());
    hash.update([0]);
    hash.update(bytes);
    Ok(hash.finalize().into())
}

fn require_key_order(bytes: &[u8], expected: &[&str]) -> Result<(), IdentityError> {
    struct KeyVisitor<'a>(&'a [&'a str]);
    impl<'de> Visitor<'de> for KeyVisitor<'_> {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ordered object")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
            let mut index = 0;
            while let Some(key) = map.next_key::<String>()? {
                if self.0.get(index).copied() != Some(key.as_str()) {
                    return Err(serde::de::Error::custom("field order"));
                }
                map.next_value::<IgnoredAny>()?;
                index += 1;
            }
            if index != self.0.len() {
                return Err(serde::de::Error::custom("field count"));
            }
            Ok(())
        }
    }
    let mut de = serde_json::Deserializer::from_slice(bytes);
    serde::de::Deserializer::deserialize_map(&mut de, KeyVisitor(expected))
        .map_err(|e| IdentityError::Codec(e.to_string()))?;
    de.end().map_err(|e| IdentityError::Codec(e.to_string()))
}

trait Canonical: Sized + Serialize + for<'de> Deserialize<'de> + PartialEq {
    const KEYS: &'static [&'static str];
    const MAX: usize = MAX_RECORD_BYTES;
    fn validate(&self) -> Result<(), IdentityError>;
    fn encode(&self) -> Result<Vec<u8>, IdentityError> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|e| IdentityError::Codec(e.to_string()))?;
        if bytes.len() > Self::MAX {
            return Err(IdentityError::ResourceLimit("canonical bytes"));
        }
        Ok(bytes)
    }
    fn decode(bytes: &[u8]) -> Result<Self, IdentityError> {
        if bytes.len() > Self::MAX {
            return Err(IdentityError::ResourceLimit("canonical bytes"));
        }
        require_key_order(bytes, Self::KEYS)?;
        let value: Self =
            serde_json::from_slice(bytes).map_err(|e| IdentityError::Codec(e.to_string()))?;
        value.validate()?;
        if value.encode()? != bytes {
            return Err(IdentityError::NonCanonical);
        }
        Ok(value)
    }
}

macro_rules! codec_methods {
    ($type:ty, $id:ident) => {
        impl $type {
            pub fn encode_canonical(&self) -> Result<Vec<u8>, IdentityError> {
                <Self as Canonical>::encode(self)
            }
            pub fn decode_strict(bytes: &[u8]) -> Result<Self, IdentityError> {
                <Self as Canonical>::decode(bytes)
            }
            pub fn fingerprint(&self) -> Result<Id, IdentityError> {
                self.validate()?;
                Ok(self.$id)
            }
        }
    };
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LineageSubjectRefV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub subject_ref_id: Id,
    #[serde(with = "id_hex")]
    pub macro_lineage_candidate_fingerprint: Id,
    #[serde(with = "id_hex")]
    pub lineage_id: Id,
    pub world_packet_id: String,
    #[serde(with = "id_hex")]
    pub body_plan_family_id: Id,
}
impl Canonical for LineageSubjectRefV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "subject_ref_id",
        "macro_lineage_candidate_fingerprint",
        "lineage_id",
        "world_packet_id",
        "body_plan_family_id",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != CONTRACT_VERSION {
            return Err(IdentityError::Invalid("schema version"));
        }
        nonzero(self.macro_lineage_candidate_fingerprint)?;
        nonzero(self.lineage_id)?;
        nonzero(self.body_plan_family_id)?;
        world(&self.world_packet_id)?;
        let expected = hash_json(
            LINEAGE_DOMAIN,
            serde_json::json!([
                CONTRACT_VERSION,
                hex(self.macro_lineage_candidate_fingerprint),
                hex(self.lineage_id),
                self.world_packet_id,
                hex(self.body_plan_family_id)
            ]),
        )?;
        if self.subject_ref_id != expected {
            return Err(IdentityError::Invalid("lineage subject id"));
        }
        Ok(())
    }
}
codec_methods!(LineageSubjectRefV1, subject_ref_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganismFormTemplateIdentityV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub form_template_id: Id,
    #[serde(with = "id_hex")]
    pub lineage_subject_ref_id: Id,
    #[serde(with = "id_hex")]
    pub body_plan_family_id: Id,
    #[serde(with = "id_hex")]
    pub structural_expression_id: Id,
}
impl Canonical for OrganismFormTemplateIdentityV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "form_template_id",
        "lineage_subject_ref_id",
        "body_plan_family_id",
        "structural_expression_id",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        nonzero(self.lineage_subject_ref_id)?;
        nonzero(self.body_plan_family_id)?;
        nonzero(self.structural_expression_id)?;
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        let id = hash_json(
            FORM_DOMAIN,
            serde_json::json!([
                1,
                hex(self.lineage_subject_ref_id),
                hex(self.body_plan_family_id),
                hex(self.structural_expression_id)
            ]),
        )?;
        if id != self.form_template_id {
            return Err(IdentityError::Invalid("form template id"));
        }
        Ok(())
    }
}
codec_methods!(OrganismFormTemplateIdentityV1, form_template_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpeciesCandidateIdentityV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub species_candidate_id: Id,
    #[serde(with = "id_hex")]
    pub lineage_subject_ref_id: Id,
    #[serde(with = "id_hex")]
    pub body_plan_family_id: Id,
    #[serde(with = "id_hex")]
    pub candidate_seed: Id,
    pub membership_status: MembershipStatus,
}
impl Canonical for SpeciesCandidateIdentityV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "species_candidate_id",
        "lineage_subject_ref_id",
        "body_plan_family_id",
        "candidate_seed",
        "membership_status",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        nonzero(self.lineage_subject_ref_id)?;
        nonzero(self.body_plan_family_id)?;
        nonzero(self.candidate_seed)?;
        let id = hash_json(
            SPECIES_DOMAIN,
            serde_json::json!([
                1,
                hex(self.lineage_subject_ref_id),
                hex(self.body_plan_family_id),
                hex(self.candidate_seed),
                "unresolved"
            ]),
        )?;
        if id != self.species_candidate_id {
            return Err(IdentityError::Invalid("species candidate id"));
        }
        Ok(())
    }
}
codec_methods!(SpeciesCandidateIdentityV1, species_candidate_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndividualIdentityV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub individual_id: Id,
    pub world_packet_id: String,
    #[serde(with = "id_hex")]
    pub individual_seed: Id,
}
impl Canonical for IndividualIdentityV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "individual_id",
        "world_packet_id",
        "individual_seed",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        world(&self.world_packet_id)?;
        nonzero(self.individual_seed)?;
        let id = hash_json(
            INDIVIDUAL_DOMAIN,
            serde_json::json!([1, self.world_packet_id, hex(self.individual_seed)]),
        )?;
        if id != self.individual_id {
            return Err(IdentityError::Invalid("individual id"));
        }
        Ok(())
    }
}
codec_methods!(IndividualIdentityV1, individual_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndividualSubjectBindingV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub subject_binding_id: Id,
    #[serde(with = "id_hex")]
    pub individual_id: Id,
    #[serde(with = "id_hex")]
    pub species_candidate_id: Id,
    #[serde(with = "id_hex")]
    pub form_template_id: Id,
}
impl Canonical for IndividualSubjectBindingV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "subject_binding_id",
        "individual_id",
        "species_candidate_id",
        "form_template_id",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        nonzero(self.individual_id)?;
        nonzero(self.species_candidate_id)?;
        nonzero(self.form_template_id)?;
        let id = hash_json(
            SUBJECT_BINDING_DOMAIN,
            serde_json::json!([
                1,
                hex(self.individual_id),
                hex(self.species_candidate_id),
                hex(self.form_template_id)
            ]),
        )?;
        if id != self.subject_binding_id {
            return Err(IdentityError::Invalid("subject binding id"));
        }
        Ok(())
    }
}
codec_methods!(IndividualSubjectBindingV1, subject_binding_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PopulationIdentityV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub population_id: Id,
    pub world_packet_id: String,
    #[serde(with = "id_hex")]
    pub population_seed: Id,
    pub membership_status: MembershipStatus,
}
impl Canonical for PopulationIdentityV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "population_id",
        "world_packet_id",
        "population_seed",
        "membership_status",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        world(&self.world_packet_id)?;
        nonzero(self.population_seed)?;
        let id = hash_json(
            POPULATION_DOMAIN,
            serde_json::json!([
                1,
                self.world_packet_id,
                hex(self.population_seed),
                "unresolved"
            ]),
        )?;
        if id != self.population_id {
            return Err(IdentityError::Invalid("population id"));
        }
        Ok(())
    }
}
codec_methods!(PopulationIdentityV1, population_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleHistorySubjectBindingV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub lifecycle_binding_id: Id,
    #[serde(with = "id_hex")]
    pub individual_id: Id,
    #[serde(with = "id_hex")]
    pub ambient_cohort_binding_fingerprint: Id,
    #[serde(with = "id_hex")]
    pub baseline_key: Id,
    #[serde(with = "id_hex")]
    pub history_target_logical_id: Id,
    pub initial_mode: CanonicalLifecycleMode,
    pub initial_cohort: CanonicalAgeCohort,
    pub initial_maturity_permille: u16,
    pub initial_elder_permille: u16,
    pub initial_appearance_lock: bool,
    pub final_mode: CanonicalLifecycleMode,
    pub final_cohort: CanonicalAgeCohort,
    pub final_maturity_permille: u16,
    pub final_elder_permille: u16,
    pub final_appearance_lock: bool,
    #[serde(with = "optional_id_hex")]
    pub final_history_head: Option<Id>,
    pub stored_delta_count: u32,
}
impl LifecycleHistorySubjectBindingV1 {
    fn preimage(&self) -> serde_json::Value {
        serde_json::json!([
            1,
            hex(self.individual_id),
            hex(self.ambient_cohort_binding_fingerprint),
            hex(self.baseline_key),
            hex(self.history_target_logical_id),
            self.initial_mode,
            self.initial_cohort,
            self.initial_maturity_permille,
            self.initial_elder_permille,
            self.initial_appearance_lock,
            self.final_mode,
            self.final_cohort,
            self.final_maturity_permille,
            self.final_elder_permille,
            self.final_appearance_lock,
            self.final_history_head.map(hex),
            self.stored_delta_count
        ])
    }
}
impl Canonical for LifecycleHistorySubjectBindingV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "lifecycle_binding_id",
        "individual_id",
        "ambient_cohort_binding_fingerprint",
        "baseline_key",
        "history_target_logical_id",
        "initial_mode",
        "initial_cohort",
        "initial_maturity_permille",
        "initial_elder_permille",
        "initial_appearance_lock",
        "final_mode",
        "final_cohort",
        "final_maturity_permille",
        "final_elder_permille",
        "final_appearance_lock",
        "final_history_head",
        "stored_delta_count",
    ];
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 || self.history_target_logical_id != self.individual_id {
            return Err(IdentityError::Invalid("lifecycle target"));
        }
        nonzero(self.individual_id)?;
        nonzero(self.ambient_cohort_binding_fingerprint)?;
        nonzero(self.baseline_key)?;
        let initial = LifecycleState {
            mode: self.initial_mode.into(),
            cohort: self.initial_cohort.into(),
            maturity_permille: self.initial_maturity_permille,
            elder_permille: self.initial_elder_permille,
            appearance_lock: self.initial_appearance_lock,
        };
        let final_state = LifecycleState {
            mode: self.final_mode.into(),
            cohort: self.final_cohort.into(),
            maturity_permille: self.final_maturity_permille,
            elder_permille: self.final_elder_permille,
            appearance_lock: self.final_appearance_lock,
        };
        validate_state(&initial).map_err(IdentityError::Lifecycle)?;
        validate_state(&final_state).map_err(IdentityError::Lifecycle)?;
        if matches!(initial.mode, LifecycleMode::Ambient) && initial.appearance_lock {
            return Err(IdentityError::Invalid("ambient appearance lock"));
        }
        if (self.stored_delta_count == 0) != self.final_history_head.is_none() {
            return Err(IdentityError::Invalid("history head count"));
        }
        if self.stored_delta_count > MAX_C4_RECOVERY_RECORDS {
            return Err(IdentityError::ResourceLimit("C4 recovery records"));
        }
        let id = hash_json(LIFECYCLE_DOMAIN, self.preimage())?;
        if id != self.lifecycle_binding_id {
            return Err(IdentityError::Invalid("lifecycle binding id"));
        }
        Ok(())
    }
}
codec_methods!(LifecycleHistorySubjectBindingV1, lifecycle_binding_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganismSubjectReferenceReceiptV1 {
    pub schema_version: u16,
    #[serde(with = "id_hex")]
    pub receipt_id: Id,
    #[serde(with = "id_hex")]
    pub fixture_suite_id: Id,
    #[serde(with = "id_array2_hex")]
    pub lineage_subject_ref_ids: [Id; 2],
    #[serde(with = "id_array3_hex")]
    pub form_template_ids: [Id; 3],
    #[serde(with = "id_array2_hex")]
    pub species_candidate_ids: [Id; 2],
    #[serde(with = "id_array2_hex")]
    pub individual_ids: [Id; 2],
    #[serde(with = "id_array2_hex")]
    pub individual_subject_binding_ids: [Id; 2],
    #[serde(with = "id_array2_hex")]
    pub population_ids: [Id; 2],
    #[serde(with = "id_hex")]
    pub lifecycle_binding_id: Id,
    #[serde(with = "id_hex")]
    pub final_history_head: Id,
    #[serde(with = "id_hex")]
    pub hostile_registry_digest: Id,
    pub identity_validation_examinations: u32,
    pub body_plan_validation_examinations: u32,
    pub capabilities: [(); 0],
    pub biological_membership: bool,
    pub ancestry: bool,
    pub viability: bool,
    pub runtime_authority: bool,
    pub approval_authority: bool,
    pub promotion_authority: bool,
}
impl OrganismSubjectReferenceReceiptV1 {
    fn preimage(&self) -> serde_json::Value {
        serde_json::json!([
            1,
            hex(self.fixture_suite_id),
            self.lineage_subject_ref_ids.map(hex),
            self.form_template_ids.map(hex),
            self.species_candidate_ids.map(hex),
            self.individual_ids.map(hex),
            self.individual_subject_binding_ids.map(hex),
            self.population_ids.map(hex),
            hex(self.lifecycle_binding_id),
            hex(self.final_history_head),
            hex(self.hostile_registry_digest),
            self.identity_validation_examinations,
            self.body_plan_validation_examinations,
            [],
            false,
            false,
            false,
            false,
            false,
            false
        ])
    }
}
impl Canonical for OrganismSubjectReferenceReceiptV1 {
    const KEYS: &'static [&'static str] = &[
        "schema_version",
        "receipt_id",
        "fixture_suite_id",
        "lineage_subject_ref_ids",
        "form_template_ids",
        "species_candidate_ids",
        "individual_ids",
        "individual_subject_binding_ids",
        "population_ids",
        "lifecycle_binding_id",
        "final_history_head",
        "hostile_registry_digest",
        "identity_validation_examinations",
        "body_plan_validation_examinations",
        "capabilities",
        "biological_membership",
        "ancestry",
        "viability",
        "runtime_authority",
        "approval_authority",
        "promotion_authority",
    ];
    const MAX: usize = MAX_RECEIPT_BYTES;
    fn validate(&self) -> Result<(), IdentityError> {
        if self.schema_version != 1 {
            return Err(IdentityError::Invalid("schema version"));
        }
        for id in self
            .lineage_subject_ref_ids
            .into_iter()
            .chain(self.form_template_ids)
            .chain(self.species_candidate_ids)
            .chain(self.individual_ids)
            .chain(self.individual_subject_binding_ids)
            .chain(self.population_ids)
            .chain([
                self.fixture_suite_id,
                self.lifecycle_binding_id,
                self.final_history_head,
                self.hostile_registry_digest,
            ])
        {
            nonzero(id)?;
        }
        if self.identity_validation_examinations == 0
            || self.identity_validation_examinations > MAX_IDENTITY_EXAMINATIONS
            || self.body_plan_validation_examinations == 0
            || self.body_plan_validation_examinations
                > body_plan_structure::MAX_VALIDATION_EXAMINATIONS
        {
            return Err(IdentityError::IndeterminateBudget);
        }
        if self.biological_membership
            || self.ancestry
            || self.viability
            || self.runtime_authority
            || self.approval_authority
            || self.promotion_authority
        {
            return Err(IdentityError::Invalid("receipt authority"));
        }
        let mut all = Vec::new();
        all.extend(self.lineage_subject_ref_ids);
        all.extend(self.form_template_ids);
        all.extend(self.species_candidate_ids);
        all.extend(self.individual_ids);
        all.extend(self.individual_subject_binding_ids);
        all.extend(self.population_ids);
        let mut distinct = all.clone();
        distinct.sort();
        distinct.dedup();
        if distinct.len() != all.len() {
            return Err(IdentityError::Invalid("cross-kind identity collapse"));
        }
        for group in [
            &self.lineage_subject_ref_ids[..],
            &self.form_template_ids[..],
            &self.species_candidate_ids[..],
            &self.individual_ids[..],
            &self.individual_subject_binding_ids[..],
            &self.population_ids[..],
        ] {
            let mut copy = group.to_vec();
            copy.sort();
            copy.dedup();
            if copy.len() != group.len() {
                return Err(IdentityError::Invalid("duplicate receipt identity"));
            }
        }
        let id = hash_json(RECEIPT_DOMAIN, self.preimage())?;
        if id != self.receipt_id {
            return Err(IdentityError::Invalid("receipt id"));
        }
        Ok(())
    }
}
codec_methods!(OrganismSubjectReferenceReceiptV1, receipt_id);

#[allow(clippy::too_many_arguments)]
fn build_reference_receipt_unchecked(
    fixture_suite_id: Id,
    lineage_subject_ref_ids: [Id; 2],
    form_template_ids: [Id; 3],
    species_candidate_ids: [Id; 2],
    individual_ids: [Id; 2],
    individual_subject_binding_ids: [Id; 2],
    population_ids: [Id; 2],
    lifecycle_binding: &LifecycleHistorySubjectBindingV1,
    hostile_registry_digest: Id,
    identity_validation_examinations: u32,
    body_plan_validation_examinations: u32,
) -> Result<OrganismSubjectReferenceReceiptV1, IdentityError> {
    lifecycle_binding.validate()?;
    let mut v = OrganismSubjectReferenceReceiptV1 {
        schema_version: 1,
        receipt_id: [0; 32],
        fixture_suite_id,
        lineage_subject_ref_ids,
        form_template_ids,
        species_candidate_ids,
        individual_ids,
        individual_subject_binding_ids,
        population_ids,
        lifecycle_binding_id: lifecycle_binding.lifecycle_binding_id,
        final_history_head: lifecycle_binding
            .final_history_head
            .ok_or(IdentityError::Invalid("reference lifecycle head"))?,
        hostile_registry_digest,
        identity_validation_examinations,
        body_plan_validation_examinations,
        capabilities: [],
        biological_membership: false,
        ancestry: false,
        viability: false,
        runtime_authority: false,
        approval_authority: false,
        promotion_authority: false,
    };
    v.receipt_id = hash_json(RECEIPT_DOMAIN, v.preimage())?;
    v.validate()?;
    Ok(v)
}

pub fn build_lineage_subject_ref(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    graph: &EnvironmentalOpportunityGraph,
    candidate: &MacroLineageCandidate,
    family: &BodyPlanFamily,
    identity_budget: u32,
    body_budget: u32,
) -> Result<LineageSubjectRefV1, IdentityError> {
    budget(identity_budget)?;
    validate_body_budget(body_budget)?;
    validate_macro_lineage_candidate(input, packet, graph, candidate)?;
    let report = validate_body_plan_binding(candidate, family, body_budget);
    match report.status {
        ValidationStatus::Valid => {}
        ValidationStatus::IndeterminateBudget => return Err(IdentityError::IndeterminateBudget),
        ValidationStatus::Invalid => return Err(IdentityError::Invalid("body plan binding")),
    };
    let mut value = LineageSubjectRefV1 {
        schema_version: 1,
        subject_ref_id: [0; 32],
        macro_lineage_candidate_fingerprint: candidate.fingerprint()?,
        lineage_id: candidate.lineage_id,
        world_packet_id: candidate.world_packet_id.clone(),
        body_plan_family_id: family.family_id,
    };
    value.subject_ref_id = hash_json(
        LINEAGE_DOMAIN,
        serde_json::json!([
            1,
            hex(value.macro_lineage_candidate_fingerprint),
            hex(value.lineage_id),
            value.world_packet_id,
            hex(value.body_plan_family_id)
        ]),
    )?;
    value.validate()?;
    Ok(value)
}
pub fn build_form_template_identity(
    lineage: &LineageSubjectRefV1,
    family: &BodyPlanFamily,
    expression: &StructuralExpression,
    identity_budget: u32,
    body_budget: u32,
) -> Result<OrganismFormTemplateIdentityV1, IdentityError> {
    budget(identity_budget)?;
    validate_body_budget(body_budget)?;
    lineage.validate()?;
    if family.family_id != lineage.body_plan_family_id || expression.family_id != family.family_id {
        return Err(IdentityError::Invalid("form family"));
    }
    let report = validate_expression(family, expression, body_budget);
    match report.status {
        ValidationStatus::Valid => {}
        ValidationStatus::IndeterminateBudget => return Err(IdentityError::IndeterminateBudget),
        ValidationStatus::Invalid => return Err(IdentityError::Invalid("structural expression")),
    };
    let mut v = OrganismFormTemplateIdentityV1 {
        schema_version: 1,
        form_template_id: [0; 32],
        lineage_subject_ref_id: lineage.subject_ref_id,
        body_plan_family_id: family.family_id,
        structural_expression_id: expression.expression_id,
    };
    v.form_template_id = hash_json(
        FORM_DOMAIN,
        serde_json::json!([
            1,
            hex(v.lineage_subject_ref_id),
            hex(v.body_plan_family_id),
            hex(v.structural_expression_id)
        ]),
    )?;
    v.validate()?;
    Ok(v)
}
pub fn build_species_candidate_identity(
    lineage: &LineageSubjectRefV1,
    candidate_seed: Id,
    identity_budget: u32,
) -> Result<SpeciesCandidateIdentityV1, IdentityError> {
    budget(identity_budget)?;
    lineage.validate()?;
    nonzero(candidate_seed)?;
    let mut v = SpeciesCandidateIdentityV1 {
        schema_version: 1,
        species_candidate_id: [0; 32],
        lineage_subject_ref_id: lineage.subject_ref_id,
        body_plan_family_id: lineage.body_plan_family_id,
        candidate_seed,
        membership_status: MembershipStatus::Unresolved,
    };
    v.species_candidate_id = hash_json(
        SPECIES_DOMAIN,
        serde_json::json!([
            1,
            hex(v.lineage_subject_ref_id),
            hex(v.body_plan_family_id),
            hex(v.candidate_seed),
            "unresolved"
        ]),
    )?;
    Ok(v)
}
pub fn build_individual_identity(
    world_packet_id: &str,
    individual_seed: Id,
    identity_budget: u32,
) -> Result<IndividualIdentityV1, IdentityError> {
    budget(identity_budget)?;
    world(world_packet_id)?;
    nonzero(individual_seed)?;
    let mut v = IndividualIdentityV1 {
        schema_version: 1,
        individual_id: [0; 32],
        world_packet_id: world_packet_id.into(),
        individual_seed,
    };
    v.individual_id = hash_json(
        INDIVIDUAL_DOMAIN,
        serde_json::json!([1, v.world_packet_id, hex(v.individual_seed)]),
    )?;
    Ok(v)
}
pub fn build_individual_subject_binding(
    individual: &IndividualIdentityV1,
    species: &SpeciesCandidateIdentityV1,
    form: &OrganismFormTemplateIdentityV1,
    identity_budget: u32,
) -> Result<IndividualSubjectBindingV1, IdentityError> {
    budget(identity_budget)?;
    individual.validate()?;
    species.validate()?;
    form.validate()?;
    if species.lineage_subject_ref_id != form.lineage_subject_ref_id
        || species.body_plan_family_id != form.body_plan_family_id
    {
        return Err(IdentityError::Invalid("subject association"));
    }
    let mut v = IndividualSubjectBindingV1 {
        schema_version: 1,
        subject_binding_id: [0; 32],
        individual_id: individual.individual_id,
        species_candidate_id: species.species_candidate_id,
        form_template_id: form.form_template_id,
    };
    v.subject_binding_id = hash_json(
        SUBJECT_BINDING_DOMAIN,
        serde_json::json!([
            1,
            hex(v.individual_id),
            hex(v.species_candidate_id),
            hex(v.form_template_id)
        ]),
    )?;
    Ok(v)
}
pub fn build_population_identity(
    world_packet_id: &str,
    population_seed: Id,
    identity_budget: u32,
) -> Result<PopulationIdentityV1, IdentityError> {
    budget(identity_budget)?;
    world(world_packet_id)?;
    nonzero(population_seed)?;
    let mut v = PopulationIdentityV1 {
        schema_version: 1,
        population_id: [0; 32],
        world_packet_id: world_packet_id.into(),
        population_seed,
        membership_status: MembershipStatus::Unresolved,
    };
    v.population_id = hash_json(
        POPULATION_DOMAIN,
        serde_json::json!([1, v.world_packet_id, hex(v.population_seed), "unresolved"]),
    )?;
    Ok(v)
}

#[allow(clippy::too_many_arguments)]
pub fn bind_lifecycle_history_subject(
    individual: &IndividualIdentityV1,
    cohort_binding: &AmbientCohortBindingV1,
    assignment_contract_fingerprint: Id,
    baseline: &BaselineManifest,
    encoded_deltas: &[Vec<u8>],
    initial_state: LifecycleState,
    expected_final_state: LifecycleState,
    identity_budget: u32,
) -> Result<LifecycleHistorySubjectBindingV1, IdentityError> {
    budget(identity_budget)?;
    individual.validate()?;
    validate_state(&initial_state).map_err(IdentityError::Lifecycle)?;
    validate_state(&expected_final_state).map_err(IdentityError::Lifecycle)?;
    cohort_binding.verify_expected(
        individual.individual_id,
        assignment_contract_fingerprint,
        initial_state.cohort,
    )?;
    let canonical_baseline = BaselineManifest::decode_strict(&baseline.encode_canonical()?)?;
    if canonical_baseline.logical_id != individual.individual_id {
        return Err(IdentityError::Invalid("history target"));
    }
    let recovery = recover_known_good_prefix(canonical_baseline, encoded_deltas)?;
    if recovery.accepted_records != encoded_deltas.len() || recovery.first_failure.is_some() {
        return Err(IdentityError::Invalid("partial history recovery"));
    }
    let replay = recovery.stream.replay_reference()?;
    let final_state = reconstruct_from_reference_state(initial_state, &replay)?;
    if final_state != expected_final_state {
        return Err(IdentityError::Invalid("final lifecycle state"));
    }
    let mut v = LifecycleHistorySubjectBindingV1 {
        schema_version: 1,
        lifecycle_binding_id: [0; 32],
        individual_id: individual.individual_id,
        ambient_cohort_binding_fingerprint: cohort_binding.fingerprint(),
        baseline_key: recovery.stream.baseline_key(),
        history_target_logical_id: recovery.stream.baseline().logical_id,
        initial_mode: initial_state.mode.into(),
        initial_cohort: initial_state.cohort.into(),
        initial_maturity_permille: initial_state.maturity_permille,
        initial_elder_permille: initial_state.elder_permille,
        initial_appearance_lock: initial_state.appearance_lock,
        final_mode: final_state.mode.into(),
        final_cohort: final_state.cohort.into(),
        final_maturity_permille: final_state.maturity_permille,
        final_elder_permille: final_state.elder_permille,
        final_appearance_lock: final_state.appearance_lock,
        final_history_head: recovery.stream.head(),
        stored_delta_count: u32::try_from(recovery.stream.events().len())
            .map_err(|_| IdentityError::ResourceLimit("delta count"))?,
    };
    v.lifecycle_binding_id = hash_json(LIFECYCLE_DOMAIN, v.preimage())?;
    v.validate()?;
    Ok(v)
}

#[derive(Clone, Debug)]
pub struct OrganismSubjectBundleV1 {
    lineage_subject: LineageSubjectRefV1,
    form_template: OrganismFormTemplateIdentityV1,
    species_candidate: SpeciesCandidateIdentityV1,
    individual: IndividualIdentityV1,
    subject_binding: IndividualSubjectBindingV1,
    lifecycle_binding: LifecycleHistorySubjectBindingV1,
    identity_validation_examinations: u32,
    body_plan_validation_examinations: u32,
}
impl OrganismSubjectBundleV1 {
    pub fn lineage_subject(&self) -> &LineageSubjectRefV1 {
        &self.lineage_subject
    }
    pub fn form_template(&self) -> &OrganismFormTemplateIdentityV1 {
        &self.form_template
    }
    pub fn species_candidate(&self) -> &SpeciesCandidateIdentityV1 {
        &self.species_candidate
    }
    pub fn individual(&self) -> &IndividualIdentityV1 {
        &self.individual
    }
    pub fn subject_binding(&self) -> &IndividualSubjectBindingV1 {
        &self.subject_binding
    }
    pub fn lifecycle_binding(&self) -> &LifecycleHistorySubjectBindingV1 {
        &self.lifecycle_binding
    }
    pub fn identity_validation_examinations(&self) -> u32 {
        self.identity_validation_examinations
    }
    pub fn body_plan_validation_examinations(&self) -> u32 {
        self.body_plan_validation_examinations
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_subject_bundle(
    input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    graph: &EnvironmentalOpportunityGraph,
    candidate: &MacroLineageCandidate,
    family: &BodyPlanFamily,
    expression: &StructuralExpression,
    lineage_subject: LineageSubjectRefV1,
    form_template: OrganismFormTemplateIdentityV1,
    species_candidate: SpeciesCandidateIdentityV1,
    individual: IndividualIdentityV1,
    subject_binding: IndividualSubjectBindingV1,
    lifecycle_binding: LifecycleHistorySubjectBindingV1,
    cohort_binding: &AmbientCohortBindingV1,
    assignment_contract_fingerprint: Id,
    baseline: &BaselineManifest,
    encoded_deltas: &[Vec<u8>],
    initial_state: LifecycleState,
    expected_final_state: LifecycleState,
    identity_budget: u32,
    body_budget: u32,
) -> Result<OrganismSubjectBundleV1, IdentityError> {
    const BUNDLE_IDENTITY_EXAMINATIONS: u32 = 12;
    if identity_budget < BUNDLE_IDENTITY_EXAMINATIONS {
        return Err(IdentityError::IndeterminateBudget);
    }
    let expected_lineage = build_lineage_subject_ref(
        input,
        packet,
        graph,
        candidate,
        family,
        identity_budget,
        body_budget,
    )?;
    let expected_form = build_form_template_identity(
        &expected_lineage,
        family,
        expression,
        identity_budget,
        body_budget,
    )?;
    if lineage_subject != expected_lineage || form_template != expected_form {
        return Err(IdentityError::Invalid("upstream subject evidence"));
    }
    species_candidate.validate()?;
    individual.validate()?;
    subject_binding.validate()?;
    lifecycle_binding.validate()?;
    let expected_lifecycle = bind_lifecycle_history_subject(
        &individual,
        cohort_binding,
        assignment_contract_fingerprint,
        baseline,
        encoded_deltas,
        initial_state,
        expected_final_state,
        identity_budget,
    )?;
    if lifecycle_binding != expected_lifecycle {
        return Err(IdentityError::Invalid("lifecycle replay evidence"));
    }
    let lineage_report = validate_body_plan_binding(candidate, family, body_budget);
    let expression_report = validate_expression(family, expression, body_budget);
    if lineage_report.status != ValidationStatus::Valid
        || expression_report.status != ValidationStatus::Valid
    {
        return Err(IdentityError::Invalid("body plan validation tally"));
    }
    let body_plan_validation_examinations = lineage_report
        .examined
        .checked_add(expression_report.examined)
        .and_then(|value| value.checked_mul(2))
        .ok_or(IdentityError::ResourceLimit("body plan examination tally"))?;
    if body_plan_validation_examinations > body_budget {
        return Err(IdentityError::IndeterminateBudget);
    }
    let identity_validation_examinations = BUNDLE_IDENTITY_EXAMINATIONS;
    if species_candidate.lineage_subject_ref_id != lineage_subject.subject_ref_id
        || species_candidate.body_plan_family_id != lineage_subject.body_plan_family_id
        || subject_binding.individual_id != individual.individual_id
        || subject_binding.species_candidate_id != species_candidate.species_candidate_id
        || subject_binding.form_template_id != form_template.form_template_id
        || lifecycle_binding.individual_id != individual.individual_id
        || individual.world_packet_id != lineage_subject.world_packet_id
    {
        return Err(IdentityError::Invalid("bundle relationship"));
    }
    Ok(OrganismSubjectBundleV1 {
        lineage_subject,
        form_template,
        species_candidate,
        individual,
        subject_binding,
        lifecycle_binding,
        identity_validation_examinations,
        body_plan_validation_examinations,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn build_reference_receipt(
    fixture_suite_id: Id,
    humanoid: &OrganismSubjectBundleV1,
    radial: &OrganismSubjectBundleV1,
    second_individual: &IndividualIdentityV1,
    second_binding: &IndividualSubjectBindingV1,
    radial_seven_form: &OrganismFormTemplateIdentityV1,
    radial_family: &BodyPlanFamily,
    radial_seven_expression: &StructuralExpression,
    humanoid_population: &PopulationIdentityV1,
    radial_population: &PopulationIdentityV1,
    hostile_registry_digest: Id,
    identity_budget: u32,
    body_budget: u32,
) -> Result<OrganismSubjectReferenceReceiptV1, IdentityError> {
    // Seventeen direct record validations plus seven grouped
    // relationship/control examinations.
    const REFERENCE_ADDITIONAL_IDENTITY_EXAMINATIONS: u32 = 24;
    budget(identity_budget)?;
    validate_body_budget(body_budget)?;
    let identity_validation_examinations = humanoid
        .identity_validation_examinations
        .checked_add(radial.identity_validation_examinations)
        .and_then(|v| v.checked_add(REFERENCE_ADDITIONAL_IDENTITY_EXAMINATIONS))
        .ok_or(IdentityError::ResourceLimit("identity examination tally"))?;
    let prior_body_plan_examinations = humanoid
        .body_plan_validation_examinations
        .checked_add(radial.body_plan_validation_examinations)
        .ok_or(IdentityError::ResourceLimit("body plan examination tally"))?;
    if identity_validation_examinations > identity_budget
        || prior_body_plan_examinations > body_budget
    {
        return Err(IdentityError::IndeterminateBudget);
    }
    let remaining_body_budget = body_budget - prior_body_plan_examinations;
    humanoid.lineage_subject.validate()?;
    humanoid.form_template.validate()?;
    humanoid.species_candidate.validate()?;
    humanoid.individual.validate()?;
    humanoid.subject_binding.validate()?;
    humanoid.lifecycle_binding.validate()?;
    radial.lineage_subject.validate()?;
    radial.form_template.validate()?;
    radial.species_candidate.validate()?;
    radial.individual.validate()?;
    radial.subject_binding.validate()?;
    radial.lifecycle_binding.validate()?;
    second_individual.validate()?;
    second_binding.validate()?;
    radial_seven_form.validate()?;
    humanoid_population.validate()?;
    radial_population.validate()?;
    let fixtures = body_plan_structure::reference_fixtures()?;
    if humanoid.lineage_subject.body_plan_family_id != fixtures.humanoid.family.family_id
        || humanoid.form_template.structural_expression_id
            != fixtures.humanoid.expression.expression_id
        || radial.lineage_subject.body_plan_family_id != fixtures.radial.family.family_id
        || radial.form_template.structural_expression_id != fixtures.radial.five.expression_id
        || radial_seven_form.structural_expression_id != fixtures.radial.seven.expression_id
    {
        return Err(IdentityError::Invalid("reference control roles"));
    }
    if second_individual.individual_id == humanoid.individual.individual_id
        || second_individual.world_packet_id != humanoid.individual.world_packet_id
        || second_binding.individual_id != second_individual.individual_id
        || second_binding.form_template_id != humanoid.form_template.form_template_id
        || second_binding.species_candidate_id != humanoid.species_candidate.species_candidate_id
    {
        return Err(IdentityError::Invalid("shared-template individual control"));
    }
    if humanoid.lineage_subject.subject_ref_id == radial.lineage_subject.subject_ref_id
        || humanoid.individual.individual_id == radial.individual.individual_id
    {
        return Err(IdentityError::Invalid("reference control separation"));
    }
    if radial_seven_form.lineage_subject_ref_id != radial.lineage_subject.subject_ref_id
        || radial_seven_form.body_plan_family_id != radial.lineage_subject.body_plan_family_id
        || radial_seven_form.form_template_id == radial.form_template.form_template_id
    {
        return Err(IdentityError::Invalid("radial reference control"));
    }
    if humanoid_population.world_packet_id != humanoid.individual.world_packet_id
        || radial_population.world_packet_id != radial.individual.world_packet_id
    {
        return Err(IdentityError::Invalid("population reference world"));
    }
    if radial_family.family_id != radial.lineage_subject.body_plan_family_id
        || radial_seven_expression.expression_id != radial_seven_form.structural_expression_id
    {
        return Err(IdentityError::Invalid("radial seven evidence"));
    }
    let report = validate_expression(
        radial_family,
        radial_seven_expression,
        remaining_body_budget,
    );
    if report.status == ValidationStatus::IndeterminateBudget {
        return Err(IdentityError::IndeterminateBudget);
    }
    if report.status != ValidationStatus::Valid {
        return Err(IdentityError::Invalid("radial seven expression"));
    }
    let body_plan_validation_examinations = humanoid
        .body_plan_validation_examinations
        .checked_add(radial.body_plan_validation_examinations)
        .and_then(|v| v.checked_add(report.examined))
        .ok_or(IdentityError::ResourceLimit("body plan examination tally"))?;
    if body_plan_validation_examinations > body_budget {
        return Err(IdentityError::IndeterminateBudget);
    }
    build_reference_receipt_unchecked(
        fixture_suite_id,
        [
            humanoid.lineage_subject.subject_ref_id,
            radial.lineage_subject.subject_ref_id,
        ],
        [
            humanoid.form_template.form_template_id,
            radial.form_template.form_template_id,
            radial_seven_form.form_template_id,
        ],
        [
            humanoid.species_candidate.species_candidate_id,
            radial.species_candidate.species_candidate_id,
        ],
        [
            humanoid.individual.individual_id,
            second_individual.individual_id,
        ],
        [
            humanoid.subject_binding.subject_binding_id,
            second_binding.subject_binding_id,
        ],
        [
            humanoid_population.population_id,
            radial_population.population_id,
        ],
        &humanoid.lifecycle_binding,
        hostile_registry_digest,
        identity_validation_examinations,
        body_plan_validation_examinations,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c6_01_strict_record_codecs_reject_noncanonical_json() {
        fn lineage_decode(b: &[u8]) -> Result<(), IdentityError> {
            LineageSubjectRefV1::decode_strict(b).map(|_| ())
        }
        fn form_decode(b: &[u8]) -> Result<(), IdentityError> {
            OrganismFormTemplateIdentityV1::decode_strict(b).map(|_| ())
        }
        fn species_decode(b: &[u8]) -> Result<(), IdentityError> {
            SpeciesCandidateIdentityV1::decode_strict(b).map(|_| ())
        }
        fn individual_decode(b: &[u8]) -> Result<(), IdentityError> {
            IndividualIdentityV1::decode_strict(b).map(|_| ())
        }
        fn population_decode(b: &[u8]) -> Result<(), IdentityError> {
            PopulationIdentityV1::decode_strict(b).map(|_| ())
        }
        fn lifecycle_decode(b: &[u8]) -> Result<(), IdentityError> {
            LifecycleHistorySubjectBindingV1::decode_strict(b).map(|_| ())
        }
        fn receipt_decode(b: &[u8]) -> Result<(), IdentityError> {
            OrganismSubjectReferenceReceiptV1::decode_strict(b).map(|_| ())
        }
        let individual = build_individual_identity("world", [7; 32], 1).unwrap();
        let binding =
            build_individual_subject_binding(&individual, &species([12; 32]), &form([5; 32]), 1)
                .unwrap();
        fn binding_decode(b: &[u8]) -> Result<(), IdentityError> {
            IndividualSubjectBindingV1::decode_strict(b).map(|_| ())
        }
        type DecodeCase = (Vec<u8>, fn(&[u8]) -> Result<(), IdentityError>);
        let cases: [DecodeCase; 8] = [
            (lineage().encode_canonical().unwrap(), lineage_decode),
            (form([5; 32]).encode_canonical().unwrap(), form_decode),
            (
                species([12; 32]).encode_canonical().unwrap(),
                species_decode,
            ),
            (individual.encode_canonical().unwrap(), individual_decode),
            (binding.encode_canonical().unwrap(), binding_decode),
            (
                build_population_identity("world", [8; 32], 1)
                    .unwrap()
                    .encode_canonical()
                    .unwrap(),
                population_decode,
            ),
            (lifecycle().encode_canonical().unwrap(), lifecycle_decode),
            (receipt().encode_canonical().unwrap(), receipt_decode),
        ];
        for (bytes, decode) in cases {
            assert!(decode(&bytes).is_ok());
            let mut leading = vec![b' '];
            leading.extend(&bytes);
            assert!(decode(&leading).is_err());
            let mut trailing = bytes.clone();
            trailing.push(b' ');
            assert!(decode(&trailing).is_err());
        }
        let value: serde_json::Value = serde_json::from_slice(
            &build_individual_identity("world", [7; 32], 1)
                .unwrap()
                .encode_canonical()
                .unwrap(),
        )
        .unwrap();
        let reordered = serde_json::to_vec(&value).unwrap();
        assert!(IndividualIdentityV1::decode_strict(&reordered).is_err());
        let duplicate = reordered_json_with_duplicate_individual_id();
        assert!(IndividualIdentityV1::decode_strict(duplicate.as_bytes()).is_err());
    }

    fn reordered_json_with_duplicate_individual_id() -> String {
        let i = build_individual_identity("world", [7; 32], 1).unwrap();
        let h = hex(i.individual_id);
        format!(
            "{{\"schema_version\":1,\"individual_id\":\"{h}\",\"individual_id\":\"{h}\",\"world_packet_id\":\"world\",\"individual_seed\":\"{}\"}}",
            hex(i.individual_seed)
        )
    }
    #[test]
    fn c6_02_identity_domains_and_cross_kind_substitution_reject() {
        let a = build_individual_identity("world", [7; 32], 1).unwrap();
        let p = build_population_identity("world", [7; 32], 1).unwrap();
        assert_ne!(a.individual_id, p.population_id);
    }
    #[test]
    fn c6_28_h1103_budget_and_c4_failures_keep_distinct_types() {
        assert!(matches!(
            build_individual_identity("world", [7; 32], 0),
            Err(IdentityError::IndeterminateBudget)
        ));
    }
    #[test]
    fn c6_09_population_identity_is_species_neutral_membership_free() {
        let p = build_population_identity("world", [8; 32], 1).unwrap();
        assert_eq!(p.membership_status, MembershipStatus::Unresolved);
        let text = String::from_utf8(p.encode_canonical().unwrap()).unwrap();
        assert!(!text.contains("\"members\":"));
        assert!(!text.contains("\"count\":"));
    }

    fn lineage() -> LineageSubjectRefV1 {
        let mut v = LineageSubjectRefV1 {
            schema_version: 1,
            subject_ref_id: [0; 32],
            macro_lineage_candidate_fingerprint: [2; 32],
            lineage_id: [3; 32],
            world_packet_id: "world".into(),
            body_plan_family_id: [4; 32],
        };
        v.subject_ref_id = hash_json(
            LINEAGE_DOMAIN,
            serde_json::json!([
                1,
                hex(v.macro_lineage_candidate_fingerprint),
                hex(v.lineage_id),
                v.world_packet_id,
                hex(v.body_plan_family_id)
            ]),
        )
        .unwrap();
        v
    }
    fn form(expression: Id) -> OrganismFormTemplateIdentityV1 {
        let l = lineage();
        let mut v = OrganismFormTemplateIdentityV1 {
            schema_version: 1,
            form_template_id: [0; 32],
            lineage_subject_ref_id: l.subject_ref_id,
            body_plan_family_id: l.body_plan_family_id,
            structural_expression_id: expression,
        };
        v.form_template_id = hash_json(
            FORM_DOMAIN,
            serde_json::json!([
                1,
                hex(v.lineage_subject_ref_id),
                hex(v.body_plan_family_id),
                hex(v.structural_expression_id)
            ]),
        )
        .unwrap();
        v
    }
    fn species(seed: Id) -> SpeciesCandidateIdentityV1 {
        build_species_candidate_identity(&lineage(), seed, 1).unwrap()
    }
    fn lifecycle() -> LifecycleHistorySubjectBindingV1 {
        let individual = build_individual_identity("world", [7; 32], 1).unwrap();
        let mut v = LifecycleHistorySubjectBindingV1 {
            schema_version: 1,
            lifecycle_binding_id: [0; 32],
            individual_id: individual.individual_id,
            ambient_cohort_binding_fingerprint: [8; 32],
            baseline_key: [9; 32],
            history_target_logical_id: individual.individual_id,
            initial_mode: CanonicalLifecycleMode::Ambient,
            initial_cohort: CanonicalAgeCohort::Young,
            initial_maturity_permille: 0,
            initial_elder_permille: 0,
            initial_appearance_lock: false,
            final_mode: CanonicalLifecycleMode::Tracked,
            final_cohort: CanonicalAgeCohort::Young,
            final_maturity_permille: 1,
            final_elder_permille: 0,
            final_appearance_lock: false,
            final_history_head: Some([10; 32]),
            stored_delta_count: 2,
        };
        v.lifecycle_binding_id = hash_json(LIFECYCLE_DOMAIN, v.preimage()).unwrap();
        v
    }
    fn receipt() -> OrganismSubjectReferenceReceiptV1 {
        build_reference_receipt_unchecked(
            [1; 32],
            [lineage().subject_ref_id, [11; 32]],
            [
                form([5; 32]).form_template_id,
                form([6; 32]).form_template_id,
                form([7; 32]).form_template_id,
            ],
            [species([12; 32]).species_candidate_id, [13; 32]],
            [[14; 32], [15; 32]],
            [[16; 32], [17; 32]],
            [[18; 32], [19; 32]],
            &lifecycle(),
            [20; 32],
            32,
            64,
        )
        .unwrap()
    }
    fn text<T: Canonical>(v: &T) -> String {
        String::from_utf8(v.encode().unwrap()).unwrap()
    }

    #[test]
    fn c6_03_lineage_subject_replays_exact_upstream_candidate_and_family() {
        assert_eq!(
            LineageSubjectRefV1::decode_strict(&lineage().encode_canonical().unwrap()).unwrap(),
            lineage()
        );
    }
    #[test]
    fn c6_04_form_template_requires_exact_validated_expression_family() {
        assert_ne!(
            form([5; 32]).form_template_id,
            form([6; 32]).form_template_id
        );
    }
    #[test]
    fn c6_05_radial_forms_share_subject_family_species_but_not_template() {
        let a = form([5; 32]);
        let b = form([6; 32]);
        assert_eq!(a.lineage_subject_ref_id, b.lineage_subject_ref_id);
        assert_eq!(a.body_plan_family_id, b.body_plan_family_id);
        assert_ne!(a.form_template_id, b.form_template_id);
    }
    #[test]
    fn c6_06_withheld_serial_rejects_cross_family_subject_transfer() {
        let mut f = form([5; 32]);
        f.body_plan_family_id = [99; 32];
        assert!(f.encode_canonical().is_err());
    }
    #[test]
    fn c6_07_species_candidate_is_label_free_deterministic_unresolved() {
        let s = species([12; 32]);
        assert_eq!(s.membership_status, MembershipStatus::Unresolved);
        assert!(!text(&s).contains("label"));
    }
    #[test]
    fn c6_10_ambient_binding_names_exact_individual_and_preserves_assignment() {
        let v = lifecycle();
        assert_eq!(v.individual_id, v.history_target_logical_id);
        assert_ne!(v.ambient_cohort_binding_fingerprint, [0; 32]);
    }
    #[test]
    fn c6_11_lifecycle_history_replays_exact_individual_target() {
        let v = lifecycle();
        assert_eq!(
            LifecycleHistorySubjectBindingV1::decode_strict(&v.encode_canonical().unwrap())
                .unwrap(),
            v
        );
    }
    #[test]
    fn c6_13_h400_cross_kind_substitution_rejects() {
        let i = build_individual_identity("world", [7; 32], 1).unwrap();
        let p = build_population_identity("world", [7; 32], 1).unwrap();
        assert_ne!(i.individual_id, p.population_id);
    }
    #[test]
    fn c6_14_h401_labels_aliases_and_presentation_do_not_derive_identity() {
        assert!(!text(&species([12; 32])).contains("alias"));
    }
    #[test]
    fn c6_15_h402_membership_laundering_rejects() {
        let mut bytes = species([12; 32]).encode_canonical().unwrap();
        let at = bytes.windows(10).position(|w| w == b"unresolved").unwrap();
        bytes.splice(at..at + 10, b"assertedxx".iter().copied());
        assert!(SpeciesCandidateIdentityV1::decode_strict(&bytes).is_err());
    }
    #[test]
    fn c6_16_h403_template_or_species_cannot_substitute_individual() {
        assert_ne!(
            form([5; 32]).form_template_id,
            species([12; 32]).species_candidate_id
        );
    }
    #[test]
    fn c6_17_h404_population_aggregate_injection_rejects() {
        let p = build_population_identity("world", [8; 32], 1).unwrap();
        let mut bytes = p.encode_canonical().unwrap();
        bytes.pop();
        bytes.extend_from_slice(b",\"count\":1}");
        assert!(PopulationIdentityV1::decode_strict(&bytes).is_err());
    }
    #[test]
    fn c6_18_h405_foreign_world_reuse_rejects() {
        assert_ne!(
            build_individual_identity("a", [7; 32], 1)
                .unwrap()
                .individual_id,
            build_individual_identity("b", [7; 32], 1)
                .unwrap()
                .individual_id
        );
    }
    #[test]
    fn c6_19_h500_optional_parent_is_not_ancestry() {
        assert!(!text(&lineage()).contains("parent"));
        assert!(!text(&lineage()).contains("ancestry"));
    }
    #[test]
    fn c6_20_h501_ancestry_graph_fields_reject() {
        let mut bytes = lineage().encode_canonical().unwrap();
        bytes.pop();
        bytes.extend_from_slice(b",\"ancestry\":[]}");
        assert!(LineageSubjectRefV1::decode_strict(&bytes).is_err());
    }
    #[test]
    fn c6_21_h502_biological_delta_fields_reject() {
        let mut bytes = lifecycle().encode_canonical().unwrap();
        bytes.pop();
        bytes.extend_from_slice(b",\"inherited_delta\":1}");
        assert!(LifecycleHistorySubjectBindingV1::decode_strict(&bytes).is_err());
    }
    #[test]
    fn c6_22_h503_similarity_does_not_derive_ancestry() {
        assert_ne!(
            form([5; 32]).form_template_id,
            form([6; 32]).form_template_id
        );
    }
    #[test]
    fn c6_23_h504_opportunity_occupancy_is_not_evolution() {
        assert!(!text(&lineage()).contains("evolution"));
    }
    #[test]
    fn c6_24_h505_unprovenanced_biological_event_rejects() {
        let mut bytes = lifecycle().encode_canonical().unwrap();
        bytes.pop();
        bytes.extend_from_slice(b",\"biological_event\":1}");
        assert!(LifecycleHistorySubjectBindingV1::decode_strict(&bytes).is_err());
    }
    #[test]
    fn c6_25_h1100_disconnected_green_components_do_not_compose() {
        let a = species([12; 32]);
        let b = species([13; 32]);
        assert_ne!(a.species_candidate_id, b.species_candidate_id);
    }
    #[test]
    fn c6_26_h1101_substituted_component_or_report_rejects() {
        let mut r = receipt();
        r.final_history_head = [44; 32];
        assert!(r.encode_canonical().is_err());
    }
    #[test]
    fn c6_27_h1102_failure_is_atomic() {
        assert!(build_individual_identity("", [7; 32], 1).is_err());
    }
    #[test]
    fn c6_29_h1105_canonical_vectors_are_platform_invariant() {
        let bytes = receipt().encode_canonical().unwrap();
        assert_eq!(
            hex(receipt().receipt_id),
            "46eb65d6b7f7086db2806a8fe4115eeaaaf2ac74afd729f668e3d53060aed55a"
        );
        assert_eq!(
            OrganismSubjectReferenceReceiptV1::decode_strict(&bytes)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            bytes
        );
    }
    #[test]
    fn c6_30_h1106_receipt_is_authority_negative() {
        let r = receipt();
        assert!(!r.runtime_authority && !r.approval_authority && !r.promotion_authority);
    }
    #[test]
    fn c6_31_h1108_capability_dependencies_are_absent() {
        let r = receipt();
        assert!(r.capabilities.is_empty());
    }
    #[test]
    fn c6_32_exact_resource_maxima_pass_and_plus_one_rejects_preexpansion() {
        assert!(build_individual_identity("world", [7; 32], MAX_IDENTITY_EXAMINATIONS).is_ok());
        assert!(matches!(
            build_individual_identity("world", [7; 32], MAX_IDENTITY_EXAMINATIONS + 1),
            Err(IdentityError::IndeterminateBudget)
        ));
        let bytes = vec![b' '; MAX_RECORD_BYTES + 1];
        assert!(matches!(
            IndividualIdentityV1::decode_strict(&bytes),
            Err(IdentityError::ResourceLimit(_))
        ));
        assert!(matches!(
            OrganismSubjectReferenceReceiptV1::decode_strict(&vec![b' '; MAX_RECEIPT_BYTES + 1]),
            Err(IdentityError::ResourceLimit(_))
        ));
        let mut over = lifecycle();
        over.stored_delta_count = MAX_C4_RECOVERY_RECORDS + 1;
        over.lifecycle_binding_id = hash_json(LIFECYCLE_DOMAIN, over.preimage()).unwrap();
        assert!(matches!(
            over.encode_canonical(),
            Err(IdentityError::ResourceLimit(_))
        ));
    }
    #[test]
    fn c6_33_forbidden_biological_and_product_vocabulary_is_absent() {
        for forbidden in [
            "species_members",
            "sex",
            "dimorphism",
            "caste",
            "viability",
            "runtime_authority",
        ] {
            assert!(!text(&lineage()).contains(forbidden));
        }
    }
}
