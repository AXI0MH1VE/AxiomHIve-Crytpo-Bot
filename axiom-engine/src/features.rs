//! Feature Engineering: Deterministic State Construction
//!
//! All features are calculated using fixed-point arithmetic to ensure
//! bitwise determinism across all execution environments.

use axiom_core::{OrderBook, Tick, Price, Decimal};
use axiom_data::normalization::*;
use std::collections::VecDeque;

/// Feature calculator with deterministic arithmetic
pub struct FeatureCalculator {
    price_history: VecDeque<Price>,
    max_history: usize,
}

impl FeatureCalculator {
    pub fn new(max_history: usize) -> Self {
        Self {
            price_history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Calculate contradiction score
    ///
    /// Measures divergence between implied and realized volatility,
    /// or between on-chain and CEX liquidity.
    pub fn calculate_contradiction_score(
        &self,
        book: &OrderBook,
        onchain_liquidity: Decimal,
    ) -> Decimal {
        let cex_liquidity = self.calculate_cex_liquidity(book);
        
        if cex_liquidity == Decimal::ZERO {
            return Decimal::ZERO;
        }

        // Contradiction = |onchain - cex| / cex
        let diff = (onchain_liquidity - cex_liquidity).abs();
        diff / cex_liquidity
    }

    /// Calculate entropy count (market disorder)
    pub fn calculate_entropy(&self, book: &OrderBook) -> Decimal {
        let spread_pct = calculate_spread_pct(book)
            .unwrap_or(Decimal::ZERO);
        
        let imbalance = calculate_depth_imbalance(book).abs();

        // Entropy = spread_pct * (1 + imbalance)
        spread_pct * (Decimal::ONE + imbalance)
    }

    /// Calculate CEX liquidity from order book
    fn calculate_cex_liquidity(&self, book: &OrderBook) -> Decimal {
        let bid_volume: Decimal = book.bids.iter()
            .take(10) // Top 10 levels
            .map(|level| level.price * level.quantity)
            .sum();

        let ask_volume: Decimal = book.asks.iter()
            .take(10)
            .map(|level| level.price * level.quantity)
            .sum();

        (bid_volume + ask_volume) / Decimal::from(2)
    }

    /// Calculate realized volatility (deterministic)
    pub fn calculate_volatility(&mut self, current_price: Price) -> Decimal {
        self.price_history.push_back(current_price);
        
        if self.price_history.len() > self.max_history {
            self.price_history.pop_front();
        }

        if self.price_history.len() < 2 {
            return Decimal::ZERO;
        }

        // Calculate returns
        let returns: Vec<Decimal> = self.price_history
            .iter()
            .zip(self.price_history.iter().skip(1))
            .map(|(prev, curr)| (curr - prev) / prev)
            .collect();

        // Calculate variance
        let mean: Decimal = returns.iter().sum::<Decimal>() / Decimal::from(returns.len());
        let variance: Decimal = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<Decimal>() / Decimal::from(returns.len());

        // Volatility = sqrt(variance) * sqrt(periods_per_year)
        // Assuming 1-minute bars, 525600 periods per year
        variance.sqrt() * Decimal::from(724) // sqrt(525600) ≈ 724
    }

    /// Calculate RSI (Relative Strength Index) - deterministic
    pub fn calculate_rsi(&self, prices: &[Price], period: usize) -> Option<Decimal> {
        if prices.len() < period + 1 {
            return None;
        }

        let changes: Vec<Decimal> = prices.windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        let gains: Decimal = changes.iter()
            .filter(|&&c| c > Decimal::ZERO)
            .sum();

        let losses: Decimal = changes.iter()
            .filter(|&&c| c < Decimal::ZERO)
            .map(|c| -c)
            .sum();

        if losses == Decimal::ZERO {
            return Some(Decimal::from(100));
        }

        let rs = gains / losses;
        Some(Decimal::from(100) - (Decimal::from(100) / (Decimal::ONE + rs)))
    }
}

