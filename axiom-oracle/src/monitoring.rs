//! System Monitoring: Real-Time Health Tracking
//!
//! Tracks system health, latency, and performance metrics.

use axiom_core::{SystemHealth, ConsistencyError, EntropyCount, CircuitBreakerState};
use axiom_risk::circuit_breaker::CircuitBreaker;
use rust_decimal::Decimal;
use chrono::Utc;
use std::collections::VecDeque;

/// System monitor
pub struct SystemMonitor {
    health_history: VecDeque<SystemHealth>,
    latency_samples: VecDeque<u64>,
    max_history: usize,
}

impl SystemMonitor {
    pub fn new(max_history: usize) -> Self {
        Self {
            health_history: VecDeque::with_capacity(max_history),
            latency_samples: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Record a latency sample
    pub fn record_latency(&mut self, latency_ms: u64) {
        self.latency_samples.push_back(latency_ms);
        if self.latency_samples.len() > self.max_history {
            self.latency_samples.pop_front();
        }
    }

    /// Calculate latency percentiles
    pub fn latency_percentiles(&self) -> (u64, u64, u64) {
        if self.latency_samples.is_empty() {
            return (0, 0, 0);
        }

        let mut sorted: Vec<u64> = self.latency_samples.iter().cloned().collect();
        sorted.sort();

        let p50_idx = (sorted.len() as f64 * 0.5) as usize;
        let p99_idx = (sorted.len() as f64 * 0.99) as usize;
        let p999_idx = (sorted.len() as f64 * 0.999) as usize.min(sorted.len() - 1);

        (
            sorted.get(p50_idx).copied().unwrap_or(0),
            sorted.get(p99_idx).copied().unwrap_or(0),
            sorted.get(p999_idx).copied().unwrap_or(0),
        )
    }

    /// Generate system health snapshot
    pub fn generate_health_snapshot(
        &mut self,
        consistency_error: Decimal,
        entropy_count: Decimal,
        circuit_breaker: CircuitBreakerState,
        hallucination_rate: Decimal,
    ) -> SystemHealth {
        let (p50, p99, p999) = self.latency_percentiles();

        let health = SystemHealth {
            consistency_error: ConsistencyError {
                value: consistency_error,
                source: "Verifier".to_string(),
                timestamp: Utc::now(),
            },
            entropy_count: EntropyCount {
                value: entropy_count,
                threshold: axiom_core::constants::DELTA_U_MAX_SQ,
                regime: if entropy_count > axiom_core::constants::DELTA_U_MAX_SQ {
                    axiom_core::MarketRegime::Unprovable
                } else {
                    axiom_core::MarketRegime::Normal
                },
                timestamp: Utc::now(),
            },
            circuit_breaker,
            hallucination_rate,
            latency_p50: p50,
            latency_p99: p99,
            latency_p999: p999,
            timestamp: Utc::now(),
        };

        self.health_history.push_back(health.clone());
        if self.health_history.len() > self.max_history {
            self.health_history.pop_front();
        }

        health
    }

    /// Get current health
    pub fn current_health(&self) -> Option<&SystemHealth> {
        self.health_history.back()
    }
}

