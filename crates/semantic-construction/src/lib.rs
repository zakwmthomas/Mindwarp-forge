//! Capability-free semantic and construction reference harness.

mod model;
mod proof;
mod validate;

pub use model::*;
pub use proof::*;
pub use validate::*;

use sha2::{Digest, Sha256};
use std::fmt;

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticConstructionError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
    ValidationFailed,
    StalePrecondition,
    UnknownOperation,
}

impl fmt::Display for SemanticConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SemanticConstructionError {}

pub(crate) fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

pub(crate) fn canonical_json<T>(value: &T) -> Result<Vec<u8>, SemanticConstructionError>
where
    T: serde::Serialize,
{
    serde_json::to_vec(value).map_err(|error| SemanticConstructionError::Codec(error.to_string()))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
