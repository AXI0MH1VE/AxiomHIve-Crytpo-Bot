//! Axiom Execution: Order Routing and Safety
//!
//! Handles order execution with built-in safety checks and C=0 signature
//! verification before any order is sent to an exchange.

pub mod executor;
pub mod safety;
pub mod routing;

pub use executor::*;
pub use safety::*;
pub use routing::*;

