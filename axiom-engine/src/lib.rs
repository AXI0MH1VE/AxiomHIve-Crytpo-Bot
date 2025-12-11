//! Axiom Engine: Hybrid Signal Generation
//!
//! Implements the Proposer-Verifier architecture:
//! - Proposer: Neural network (Mamba-2) suggests trades
//! - Verifier: SMT Solver proves trades satisfy L0 invariants

pub mod proposer;
pub mod verifier;
pub mod signals;
pub mod features;

pub use proposer::*;
pub use verifier::*;
pub use signals::*;
pub use features::*;

