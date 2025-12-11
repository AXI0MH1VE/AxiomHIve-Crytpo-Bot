//! Data Ingestion: Streaming Market Data
//!
//! Handles real-time data ingestion from exchanges with deterministic
//! normalization and latency monitoring.

use axiom_core::{Symbol, Venue, Tick, OrderBook};
use crate::normalization::*;
use crate::errors::*;
use tokio::sync::mpsc;
use tracing::{info, error};
use std::time::Instant;

/// Data ingestion manager
pub struct DataIngestionManager {
    tick_sender: mpsc::UnboundedSender<Tick>,
    book_sender: mpsc::UnboundedSender<OrderBook>,
}

impl DataIngestionManager {
    pub fn new(
        tick_sender: mpsc::UnboundedSender<Tick>,
        book_sender: mpsc::UnboundedSender<OrderBook>,
    ) -> Self {
        Self {
            tick_sender,
            book_sender,
        }
    }

    /// Start ingesting data from a venue
    pub async fn start_ingestion(
        &self,
        venue: Venue,
        symbols: Vec<Symbol>,
    ) -> Result<(), IngestionError> {
        info!("Starting data ingestion for venue: {}", venue.0);
        
        // For now, this is a placeholder. In production, this would:
        // 1. Connect to exchange WebSocket/REST API
        // 2. Subscribe to tick and order book streams
        // 3. Normalize all incoming data
        // 4. Send to channels with latency tracking
        
        Ok(())
    }

    /// Process a raw tick from exchange
    pub fn process_tick(&self, raw: &[u8], venue: &Venue) -> Result<(), IngestionError> {
        let start = Instant::now();
        
        // Parse JSON
        let json: serde_json::Value = serde_json::from_slice(raw)
            .map_err(|e| IngestionError::InvalidFormat(format!("JSON parse: {}", e)))?;

        // Normalize to Tick
        let tick = self.normalize_tick(&json, venue)?;

        // Check latency
        let latency_ms = start.elapsed().as_millis() as u64;
        if latency_ms > 10 {
            error!("High ingestion latency: {}ms", latency_ms);
        }

        // Send to channel
        self.tick_sender.send(tick)
            .map_err(|e| IngestionError::Network(format!("Channel send: {}", e)))?;

        Ok(())
    }

    /// Normalize raw JSON to Tick
    fn normalize_tick(&self, json: &serde_json::Value, venue: &Venue) -> Result<Tick, IngestionError> {
        let symbol = Symbol(
            json.get("symbol")
                .and_then(|v| v.as_str())
                .ok_or_else(|| IngestionError::InvalidFormat("Missing symbol".to_string()))?
                .to_string()
        );

        let price = normalize_price(
            json.get("price")
                .ok_or_else(|| IngestionError::InvalidFormat("Missing price".to_string()))?
        )?;

        let quantity = normalize_quantity(
            json.get("quantity")
                .ok_or_else(|| IngestionError::InvalidFormat("Missing quantity".to_string()))?
        )?;

        let timestamp = normalize_timestamp(
            json.get("timestamp")
                .ok_or_else(|| IngestionError::InvalidFormat("Missing timestamp".to_string()))?
        )?;

        let side_str = json.get("side")
            .and_then(|v| v.as_str())
            .ok_or_else(|| IngestionError::InvalidFormat("Missing side".to_string()))?;
        let side = normalize_side(side_str)?;

        Ok(Tick {
            symbol,
            venue: venue.clone(),
            price,
            quantity,
            timestamp,
            side,
        })
    }
}

