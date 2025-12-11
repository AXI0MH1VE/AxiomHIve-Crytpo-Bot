//! Data Normalization: Deterministic Type Conversion
//!
//! All external data (JSON, floats) is normalized into Decimal types
//! to ensure bitwise determinism.

use axiom_core::{Symbol, Venue, Price, Quantity, Tick, OrderBook, BookLevel, Side};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Normalize price from external format to Decimal
pub fn normalize_price(value: &Value) -> Result<Price, NormalizationError> {
    match value {
        Value::String(s) => {
            Decimal::from_str_exact(s)
                .or_else(|_| s.parse::<f64>()
                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    .map_err(|_| NormalizationError::ParseError("Failed to parse price".to_string())))
                .map_err(|e| NormalizationError::ParseError(format!("Price: {}", e)))
        }
        Value::Number(n) => {
            n.as_f64()
                .ok_or_else(|| NormalizationError::InvalidType("Number not convertible to f64".to_string()))
                .and_then(|f| {
                    Decimal::try_from(f)
                        .map_err(|e| NormalizationError::ParseError(format!("Decimal conversion: {}", e)))
                })
        }
        _ => Err(NormalizationError::InvalidType("Expected string or number".to_string())),
    }
}

/// Normalize quantity from external format to Decimal
pub fn normalize_quantity(value: &Value) -> Result<Quantity, NormalizationError> {
    normalize_price(value) // Same logic as price
}

/// Normalize timestamp from various formats
pub fn normalize_timestamp(value: &Value) -> Result<DateTime<Utc>, NormalizationError> {
    match value {
        Value::Number(n) => {
            let ts = n.as_i64()
                .ok_or_else(|| NormalizationError::InvalidType("Timestamp not i64".to_string()))?;
            DateTime::from_timestamp(ts / 1000, ((ts % 1000) * 1_000_000) as u32)
                .ok_or_else(|| NormalizationError::InvalidType("Invalid timestamp".to_string()))
        }
        Value::String(s) => {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| NormalizationError::ParseError(format!("RFC3339 parse: {}", e)))
        }
        _ => Err(NormalizationError::InvalidType("Expected number or RFC3339 string".to_string())),
    }
}

/// Normalize side from string
pub fn normalize_side(value: &str) -> Result<Side, NormalizationError> {
    match value.to_uppercase().as_str() {
        "BUY" | "B" | "1" => Ok(Side::Buy),
        "SELL" | "S" | "2" => Ok(Side::Sell),
        _ => Err(NormalizationError::InvalidType(format!("Unknown side: {}", value))),
    }
}

/// Calculate depth imbalance (deterministic)
pub fn calculate_depth_imbalance(book: &OrderBook) -> Decimal {
    let bid_volume: Decimal = book.bids.iter()
        .map(|level| level.quantity)
        .sum();
    
    let ask_volume: Decimal = book.asks.iter()
        .map(|level| level.quantity)
        .sum();

    if bid_volume + ask_volume == Decimal::ZERO {
        return Decimal::ZERO;
    }

    (bid_volume - ask_volume) / (bid_volume + ask_volume)
}

/// Calculate mid price (deterministic)
pub fn calculate_mid_price(book: &OrderBook) -> Option<Price> {
    let best_bid = book.bids.first()?;
    let best_ask = book.asks.first()?;
    
    Some((best_bid.price + best_ask.price) / Decimal::from(2))
}

/// Calculate spread (deterministic)
pub fn calculate_spread(book: &OrderBook) -> Option<Decimal> {
    let best_bid = book.bids.first()?;
    let best_ask = book.asks.first()?;
    
    Some(best_ask.price - best_bid.price)
}

/// Calculate spread percentage (deterministic)
pub fn calculate_spread_pct(book: &OrderBook) -> Option<Decimal> {
    let spread = calculate_spread(book)?;
    let mid = calculate_mid_price(book)?;
    
    if mid == Decimal::ZERO {
        return None;
    }
    
    Some((spread / mid) * Decimal::from(100))
}

