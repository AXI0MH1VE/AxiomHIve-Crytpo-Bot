//! Axiom Data: Zero-Entropy Ingestion Engine
//!
//! This module handles deterministic data ingestion from exchanges,
//! normalizing all data into fixed-point representations to ensure
//! bitwise reproducibility.

pub mod ingestion;
pub mod normalization;
pub mod orderbook;
pub mod onchain;
pub mod errors;

pub use ingestion::*;
pub use normalization::*;
pub use orderbook::*;
pub use onchain::*;
pub use errors::*;

