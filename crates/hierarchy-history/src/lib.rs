//! Capability-free hierarchy and sparse-history reference harness.

mod hierarchy;
mod history;
mod proof;

pub use hierarchy::*;
pub use history::*;
pub use proof::*;

use sha2::{Digest, Sha256};
use std::fmt;

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyHistoryError {
    Invalid(&'static str),
    Codec(String),
    CorruptContent,
    StaleCursor,
    Cancelled,
    WrongBaseline,
    WrongTarget,
    Gap,
    StaleHead,
    ForkConflict,
    CommandConflict,
    UnknownOperationSchema,
    UnsupportedCrossTarget,
    SnapshotMismatch,
    UnsupportedMigration,
    MissingDependency(u16),
    DependencyFingerprintMismatch(u16),
    InvalidDependencyAvailability,
    RecoveryBoundExceeded,
    UnsupportedTopology,
    MigrationChainInvalid,
}

impl fmt::Display for HierarchyHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HierarchyHistoryError {}

pub(crate) fn codec<E: fmt::Display>(error: E) -> HierarchyHistoryError {
    HierarchyHistoryError::Codec(error.to_string())
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
