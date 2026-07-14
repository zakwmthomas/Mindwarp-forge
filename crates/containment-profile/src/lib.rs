//! Capability-free P7b-1a containment-policy and readiness-receipt reference.

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
pub enum ContainmentProfileError {
    Codec(String),
    NonCanonical,
    ValidationFailed,
}

impl fmt::Display for ContainmentProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ContainmentProfileError {}

pub(crate) fn canonical_json<T: serde::Serialize>(
    value: &T,
) -> Result<Vec<u8>, ContainmentProfileError> {
    serde_json::to_vec(value).map_err(|error| ContainmentProfileError::Codec(error.to_string()))
}

pub(crate) fn hash(domain: &[u8], bytes: &[u8]) -> Id {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
