//! Alert System: Anomaly Detection
//!
//! Detects anomalies and triggers alerts when system deviates from expected behavior.

use axiom_core::{SystemHealth, CircuitBreakerState};
use axiom_core::constants::*;
use tracing::{warn, error};

/// Alert manager
pub struct AlertManager;

impl AlertManager {
    /// Check for anomalies and trigger alerts
    pub fn check_anomalies(&self, health: &SystemHealth) {
        // Check consistency error
        if health.consistency_error.value > MAX_CONSISTENCY_ERROR {
            error!("CONTRADICTION DETECTED: Consistency error = {}", health.consistency_error.value);
        }

        // Check entropy
        if health.entropy_count.value > DELTA_U_MAX_SQ {
            warn!("HIGH ENTROPY: Market disorder detected. Value: {}", health.entropy_count.value);
        }

        // Check circuit breaker
        if matches!(health.circuit_breaker, CircuitBreakerState::Tripped) {
            error!("CIRCUIT BREAKER TRIPPED: System halted");
        }

        // Check hallucination rate
        if health.hallucination_rate > MAX_HALLUCINATION_RATE {
            warn!("HIGH HALLUCINATION RATE: {} (threshold: {})",
                health.hallucination_rate, MAX_HALLUCINATION_RATE);
        }

        // Check latency
        if health.latency_p99 > 100 {
            warn!("HIGH LATENCY: P99 = {}ms", health.latency_p99);
        }
    }
}

