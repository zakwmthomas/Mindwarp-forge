//! Capability-free significance and scheduling reference harness.

mod closure;
mod proof;
mod scheduler;
mod significance;

pub use proof::*;
pub use scheduler::*;
pub use significance::*;

use sha2::{Digest, Sha256};
use std::fmt;

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignificanceSchedulerError {
    Invalid(&'static str),
    Codec(String),
    NonCanonical,
    DuplicateTicket,
    UnknownDependency,
    DependencyCycle,
    UnknownFallback,
    InvalidFallback,
    InvalidCancellationTree,
    AdmissionRejected,
    UnknownTicket,
    InvalidTransition,
    StaleEpoch,
}

impl fmt::Display for SignificanceSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SignificanceSchedulerError {}

pub(crate) fn codec<E: fmt::Display>(error: E) -> SignificanceSchedulerError {
    SignificanceSchedulerError::Codec(error.to_string())
}

pub(crate) fn hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn bytes32(bytes: &[u8]) -> Result<[u8; 32], SignificanceSchedulerError> {
    bytes
        .try_into()
        .map_err(|_| SignificanceSchedulerError::Invalid("expected 32 bytes"))
}
pub use closure::*;
