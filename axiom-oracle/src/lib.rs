//! Axiom Oracle: The God-View Dashboard
//!
//! Monitors the entire system, collecting telemetry and detecting anomalies.

pub mod monitoring;
pub mod telemetry;
pub mod alerts;

pub use monitoring::*;
pub use telemetry::*;
pub use alerts::*;

