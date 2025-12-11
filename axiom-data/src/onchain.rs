//! On-Chain Data: Direct RPC Access
//!
//! Handles direct RPC connections to blockchain nodes for minimal
//! latency on-chain data access.

use axiom_core::{Symbol, Venue};
use crate::errors::*;
use tracing::info;

/// On-chain data fetcher
pub struct OnChainFetcher {
    rpc_endpoints: Vec<String>,
}

impl OnChainFetcher {
    pub fn new(rpc_endpoints: Vec<String>) -> Self {
        Self { rpc_endpoints }
    }

    /// Fetch liquidity data from on-chain sources
    pub async fn fetch_liquidity(&self, symbol: &Symbol) -> Result<serde_json::Value, IngestionError> {
        info!("Fetching on-chain liquidity for {}", symbol.0);
        
        // Placeholder: In production, this would:
        // 1. Connect to Solana/Hyperliquid RPC
        // 2. Query liquidity pools
        // 3. Return normalized data
        
        Ok(serde_json::json!({
            "symbol": symbol.0,
            "liquidity": "0",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

