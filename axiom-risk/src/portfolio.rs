//! Portfolio Management: Real-Time State Tracking
//!
//! Maintains the portfolio state with Hamiltonian energy calculations.

use axiom_core::{Portfolio, Position, Symbol, Side, Price, Amount, Decimal};
use axiom_core::constants::*;
use rust_decimal::Decimal;
use chrono::Utc;
use std::collections::HashMap;

/// Portfolio manager
pub struct PortfolioManager {
    portfolio: Portfolio,
    position_map: HashMap<Symbol, Position>,
}

impl PortfolioManager {
    pub fn new(initial_equity: Amount) -> Self {
        Self {
            portfolio: Portfolio {
                equity: initial_equity,
                positions: Vec::new(),
                total_exposure: Decimal::ZERO,
                net_exposure: Decimal::ZERO,
                leverage: Decimal::ZERO,
                energy: Decimal::ZERO,
                correlation_matrix: Vec::new(),
            },
            position_map: HashMap::new(),
        }
    }

    /// Update portfolio with new position
    pub fn update_position(
        &mut self,
        symbol: Symbol,
        side: Side,
        quantity: Decimal,
        price: Price,
    ) {
        let position_value = quantity * price;
        
        // Update or create position
        let position = self.position_map.entry(symbol.clone())
            .and_modify(|p| {
                match (p.side, side) {
                    (Side::Buy, Side::Buy) | (Side::Sell, Side::Sell) => {
                        // Add to position
                        let total_value = p.quantity * p.entry_price + position_value;
                        let total_quantity = p.quantity + quantity;
                        p.entry_price = total_value / total_quantity;
                        p.quantity = total_quantity;
                    }
                    (Side::Buy, Side::Sell) | (Side::Sell, Side::Buy) => {
                        // Reduce or close position
                        if quantity >= p.quantity {
                            // Position closed
                            p.quantity = Decimal::ZERO;
                        } else {
                            p.quantity -= quantity;
                        }
                    }
                }
                p.current_price = price;
            })
            .or_insert_with(|| Position {
                symbol: symbol.clone(),
                venue: axiom_core::Venue("unknown".to_string()),
                side,
                quantity,
                entry_price: price,
                current_price: price,
                unrealized_pnl: Decimal::ZERO,
                realized_pnl: Decimal::ZERO,
            });

        // Recalculate portfolio metrics
        self.recalculate_metrics();
    }

    /// Update position prices (mark-to-market)
    pub fn update_prices(&mut self, prices: &HashMap<Symbol, Price>) {
        for (symbol, price) in prices {
            if let Some(position) = self.position_map.get_mut(symbol) {
                position.current_price = *price;
                position.unrealized_pnl = match position.side {
                    Side::Buy => (price - position.entry_price) * position.quantity,
                    Side::Sell => (position.entry_price - price) * position.quantity,
                };
            }
        }

        self.recalculate_metrics();
    }

    /// Recalculate all portfolio metrics
    fn recalculate_metrics(&mut self) {
        // Update positions vector
        self.portfolio.positions = self.position_map.values()
            .filter(|p| p.quantity > Decimal::ZERO)
            .cloned()
            .collect();

        // Calculate total exposure
        self.portfolio.total_exposure = self.portfolio.positions.iter()
            .map(|p| p.quantity * p.current_price)
            .sum();

        // Calculate net exposure
        let long_exposure: Decimal = self.portfolio.positions.iter()
            .filter(|p| p.side == Side::Buy)
            .map(|p| p.quantity * p.current_price)
            .sum();

        let short_exposure: Decimal = self.portfolio.positions.iter()
            .filter(|p| p.side == Side::Sell)
            .map(|p| p.quantity * p.current_price)
            .sum();

        self.portfolio.net_exposure = long_exposure - short_exposure;

        // Calculate leverage
        if self.portfolio.equity > Decimal::ZERO {
            self.portfolio.leverage = self.portfolio.total_exposure / self.portfolio.equity;
        } else {
            self.portfolio.leverage = Decimal::ZERO;
        }

        // Update equity (including unrealized PnL)
        let total_unrealized: Decimal = self.portfolio.positions.iter()
            .map(|p| p.unrealized_pnl)
            .sum();

        self.portfolio.equity = self.portfolio.equity + total_unrealized;
    }

    /// Get current portfolio
    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }

    /// Get position for symbol
    pub fn get_position(&self, symbol: &Symbol) -> Option<&Position> {
        self.position_map.get(symbol)
    }
}

