//! Error Types: Comprehensive Error Handling
//!
//! All errors in the Axiom Hive system are strongly typed and deterministic.

use thiserror::Error;

/// Top-level error type
#[derive(Debug, Error)]
pub enum AxiomError {
    #[error("Invariant violation: {0}")]
    InvariantViolation(#[from] crate::invariants::InvariantViolation),

    #[error("Signature error: {0}")]
    SignatureError(#[from] crate::signature::SignatureError),

    #[error("Data ingestion error: {0}")]
    DataError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Risk management error: {0}")]
    RiskError(String),

    #[error("Verification error: {0}")]
    VerificationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, AxiomError>;

