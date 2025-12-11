//! Telemetry Collection
//!
//! Collects and aggregates metrics from all system components.

use axiom_core::SystemHealth;
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Telemetry collector
pub struct TelemetryCollector {
    health: Arc<RwLock<Option<SystemHealth>>>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self {
            health: Arc::new(RwLock::new(None)),
        }
    }

    /// Update system health
    pub async fn update_health(&self, health: SystemHealth) {
        let mut h = self.health.write().await;
        *h = Some(health);
        
        // Log critical metrics
        info!("System Health - Consistency Error: {}, Entropy: {}, Circuit Breaker: {:?}",
            health.consistency_error.value,
            health.entropy_count.value,
            health.circuit_breaker
        );
    }

    /// Get current health
    pub async fn get_health(&self) -> Option<SystemHealth> {
        self.health.read().await.clone()
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

