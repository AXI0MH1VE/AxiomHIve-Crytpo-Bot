//! Data Ingestion Errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NormalizationError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid type: {0}")]
    InvalidType(String),
}

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("Normalization error: {0}")]
    Normalization(#[from] NormalizationError),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Exchange API error: {0}")]
    ExchangeApi(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
}

