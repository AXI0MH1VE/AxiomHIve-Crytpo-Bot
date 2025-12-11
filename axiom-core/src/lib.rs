//! Axiom Core: The Zero Entropy Foundation
//!
//! This module defines the fundamental types, constants, and invariants
//! that govern the entire Axiom Hive system. The L0 Invariant Contract
//! is enforced at compile time where possible, and at runtime through
//! formal verification.

pub mod constants;
pub mod invariants;
pub mod types;
pub mod signature;
pub mod errors;

pub use constants::*;
pub use invariants::*;
pub use types::*;
pub use signature::*;
pub use errors::*;

