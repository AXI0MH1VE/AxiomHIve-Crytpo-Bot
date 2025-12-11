//! Position Sizing: Kelly Criterion with Certainty Weighting
//!
//! Calculates optimal position size based on risk budget and certainty score.

use axiom_core::{TradeSignal, Portfolio, Decimal};
use axiom_core::constants::*;
use rust_decimal::Decimal;

/// Calculate position size using Kelly Criterion
///
/// Position size is adjusted by the certainty score (1 - P(Hallucination))
pub fn calculate_position_size(
    signal: &TradeSignal,
    portfolio: &Portfolio,
    certainty_score: Decimal,
) -> Decimal {
    // Base risk budget (0.25% - 1% of equity)
    let base_risk = portfolio.equity * MAX_RISK_BUDGET;
    
    // Adjust by certainty score
    let adjusted_risk = base_risk * certainty_score;
    
    // Calculate position size from risk and stop distance
    // Simplified: assume 2% stop loss
    let stop_distance = Decimal::from(2) / Decimal::from(100);
    
    if stop_distance == Decimal::ZERO {
        return Decimal::ZERO;
    }
    
    let position_value = adjusted_risk / stop_distance;
    
    // Get price
    let price = signal.limit_price.unwrap_or(Decimal::ZERO);
    if price == Decimal::ZERO {
        return Decimal::ZERO;
    }
    
    // Position size in base currency
    let size = position_value / price;
    
    // Enforce maximum position size
    let max_size = match signal.symbol.0.as_str() {
        "BTC/USD" => MAX_POSITION_SIZE_BTC,
        "ETH/USD" => MAX_POSITION_SIZE_ETH,
        "SOL/USD" => MAX_POSITION_SIZE_SOL,
        _ => Decimal::from(1),
    };
    
    size.min(max_size)
}

