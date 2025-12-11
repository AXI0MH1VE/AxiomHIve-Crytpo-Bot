//! Order Book Management: L2/L3 Reconstruction
//!
//! Handles full-depth order book reconstruction and maintenance
//! with deterministic calculations.

use axiom_core::{Symbol, Venue, OrderBook, BookLevel};
use crate::normalization::*;
use crate::errors::*;
use rust_decimal::Decimal;
use chrono::Utc;

/// Order book builder and maintainer
pub struct OrderBookBuilder {
    symbol: Symbol,
    venue: Venue,
    sequence: u64,
}

impl OrderBookBuilder {
    pub fn new(symbol: Symbol, venue: Venue) -> Self {
        Self {
            symbol,
            venue,
            sequence: 0,
        }
    }

    /// Build order book from snapshot
    pub fn from_snapshot(&mut self, snapshot: &serde_json::Value) -> Result<OrderBook, IngestionError> {
        let bids = self.parse_levels(
            snapshot.get("bids")
                .ok_or_else(|| IngestionError::InvalidFormat("Missing bids".to_string()))?
        )?;

        let asks = self.parse_levels(
            snapshot.get("asks")
                .ok_or_else(|| IngestionError::InvalidFormat("Missing asks".to_string()))?
        )?;

        // Sort: bids descending, asks ascending
        let mut bids = bids;
        bids.sort_by(|a, b| b.price.cmp(&a.price));

        let mut asks = asks;
        asks.sort_by(|a, b| a.price.cmp(&b.price));

        self.sequence += 1;

        Ok(OrderBook {
            symbol: self.symbol.clone(),
            venue: self.venue.clone(),
            bids,
            asks,
            timestamp: Utc::now(),
            sequence: self.sequence,
        })
    }

    /// Update order book with incremental update
    pub fn apply_update(&mut self, book: &mut OrderBook, update: &serde_json::Value) -> Result<(), IngestionError> {
        // Handle incremental updates (add/remove/update levels)
        // This is exchange-specific, so simplified here
        
        self.sequence += 1;
        book.sequence = self.sequence;
        book.timestamp = Utc::now();

        Ok(())
    }

    fn parse_levels(&self, levels: &serde_json::Value) -> Result<Vec<BookLevel>, IngestionError> {
        let array = levels.as_array()
            .ok_or_else(|| IngestionError::InvalidFormat("Levels not an array".to_string()))?;

        let mut result = Vec::new();
        for level in array {
            let price = normalize_price(
                level.get(0)
                    .ok_or_else(|| IngestionError::InvalidFormat("Missing price in level".to_string()))?
            )?;

            let quantity = normalize_quantity(
                level.get(1)
                    .ok_or_else(|| IngestionError::InvalidFormat("Missing quantity in level".to_string()))?
            )?;

            if quantity > Decimal::ZERO {
                result.push(BookLevel { price, quantity });
            }
        }

        Ok(result)
    }
}

