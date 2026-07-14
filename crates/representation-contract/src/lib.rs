//! Capability-free P7a representation contract and lineage reference harness.

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
pub enum RepresentationContractError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
    ValidationFailed,
}

impl fmt::Display for RepresentationContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RepresentationContractError {}

pub(crate) fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

pub(crate) fn canonical_json<T: serde::Serialize>(
    value: &T,
) -> Result<Vec<u8>, RepresentationContractError> {
    serde_json::to_vec(value).map_err(|error| RepresentationContractError::Codec(error.to_string()))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
