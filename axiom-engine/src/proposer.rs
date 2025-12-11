//! Proposer: Neural Network Signal Generation
//!
//! The "creative" component that proposes trades based on pattern matching.
//! This is allowed to be probabilistic, but its outputs are verified.

use axiom_core::{TradeSignal, Symbol, Venue, Side, OrderType, OrderBook, Portfolio};
use axiom_engine::features::FeatureCalculator;
use rust_decimal::Decimal;
use chrono::Utc;
use tracing::{info, warn};

/// Trade proposer (simplified - in production would use Mamba-2)
pub struct Proposer {
    feature_calc: FeatureCalculator,
    hallucination_count: u64,
    total_proposals: u64,
}

impl Proposer {
    pub fn new() -> Self {
        Self {
            feature_calc: FeatureCalculator::new(1000),
            hallucination_count: 0,
            total_proposals: 0,
        }
    }

    /// Propose a trade signal based on market state
    ///
    /// This is the "thinking" component - it can be creative and probabilistic.
    /// The verifier will ensure it's safe.
    pub fn propose_trade(
        &mut self,
        symbol: &Symbol,
        venue: &Venue,
        book: &OrderBook,
        portfolio: &Portfolio,
    ) -> Option<TradeSignal> {
        self.total_proposals += 1;

        // Calculate features
        let contradiction_score = self.feature_calc.calculate_contradiction_score(book, Decimal::ZERO);
        let entropy = self.feature_calc.calculate_entropy(book);
        
        // Simple rule-based proposer (in production, this would be a neural network)
        // Look for arbitrage opportunities (crossed spreads, mispricing)
        let mid_price = axiom_data::normalization::calculate_mid_price(book)?;
        let spread = axiom_data::normalization::calculate_spread(book)?;
        let spread_pct = axiom_data::normalization::calculate_spread_pct(book)?;

        // Propose trade if contradiction is high (market inefficiency detected)
        let threshold = Decimal::from_str_exact("0.05").unwrap_or(Decimal::from(5) / Decimal::from(100));
        let spread_threshold = Decimal::from_str_exact("0.001").unwrap_or(Decimal::from(1) / Decimal::from(1000));
        
        if contradiction_score > threshold && spread_pct > spread_threshold {
            
            // Determine side based on depth imbalance
            let imbalance = axiom_data::normalization::calculate_depth_imbalance(book);
            let side = if imbalance > Decimal::ZERO {
                Side::Buy  // More bid volume, expect upward pressure
            } else {
                Side::Sell // More ask volume, expect downward pressure
            };

            // Calculate position size (simplified - verifier will check)
            let base_quantity = Decimal::from_str_exact("0.1").unwrap_or(Decimal::from(1) / Decimal::from(10));
            
            let signal = TradeSignal {
                symbol: symbol.clone(),
                venue: venue.clone(),
                side,
                order_type: OrderType::Limit,
                quantity: base_quantity,
                limit_price: Some(mid_price),
                stop_price: None,
                timestamp: Utc::now(),
                contradiction_score,
                entropy_count: entropy,
            };

            info!("Proposed trade: {:?} @ {}", side, mid_price);
            Some(signal)
        } else {
            None
        }
    }

    /// Get hallucination rate (rejection rate from verifier)
    pub fn hallucination_rate(&self) -> Decimal {
        if self.total_proposals == 0 {
            return Decimal::ZERO;
        }
        
        Decimal::from(self.hallucination_count) / Decimal::from(self.total_proposals)
    }

    /// Record a rejection (hallucination)
    pub fn record_rejection(&mut self) {
        self.hallucination_count += 1;
        warn!("Proposer rejection recorded. Rate: {}", self.hallucination_rate());
    }
}

impl Default for Proposer {
    fn default() -> Self {
        Self::new()
    }
}

