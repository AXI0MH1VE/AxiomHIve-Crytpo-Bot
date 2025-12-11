//! Smart Order Routing
//!
//! Routes orders across multiple venues to minimize impact and slippage.

use axiom_core::{VerifiedOrder, Venue};
use tracing::info;

/// Smart order router
pub struct OrderRouter;

impl OrderRouter {
    /// Route order across venues
    ///
    /// In production, would split orders across venues based on:
    /// - Liquidity depth
    /// - Fee structure
    /// - Latency
    pub fn route_order(&self, order: &VerifiedOrder) -> Vec<(Venue, VerifiedOrder)> {
        // Simplified: route to primary venue
        // In production, would implement TWAP/VWAP algorithms
        info!("Routing order to venue: {}", order.signal.venue.0);
        vec![(order.signal.venue.clone(), order.clone())]
    }
}

