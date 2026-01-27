//! Error types for the Makoto SDK.

use thiserror::Error;

/// Errors that can occur when working with Makoto attestations.
#[derive(Error, Debug)]
pub enum MakotoError {
    /// JSON serialization or deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Cryptographic signature error.
    #[error("Signature error: {0}")]
    Signature(String),

    /// Hash verification failed.
    #[error("Hash verification failed: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    /// Invalid attestation structure.
    #[error("Invalid attestation: {0}")]
    InvalidAttestation(String),

    /// Missing required field.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid predicate type.
    #[error("Invalid predicate type: expected {expected}, got {actual}")]
    InvalidPredicateType { expected: String, actual: String },

    /// Key parsing error.
    #[error("Key error: {0}")]
    KeyError(String),

    /// Merkle tree error.
    #[error("Merkle tree error: {0}")]
    MerkleError(String),

    /// Chain verification error.
    #[error("Chain verification error: {0}")]
    ChainError(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for Makoto operations.
pub type Result<T> = std::result::Result<T, MakotoError>;
