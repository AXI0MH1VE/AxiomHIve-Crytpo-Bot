//! Circuit Breakers: The Dead Man Switch
//!
//! Hard limits that trigger automatic shutdown or risk reduction.

use axiom_core::{Portfolio, CircuitBreakerState, Decimal};
use axiom_core::constants::*;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use std::collections::VecDeque;

/// Circuit breaker manager
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    daily_pnl_history: VecDeque<(DateTime<Utc>, Decimal)>,
    max_daily_drawdown: Decimal,
    last_reset: DateTime<Utc>,
}

impl CircuitBreaker {
    pub fn new(max_daily_drawdown: Decimal) -> Self {
        Self {
            state: CircuitBreakerState::Normal,
            daily_pnl_history: VecDeque::new(),
            max_daily_drawdown,
            last_reset: Utc::now(),
        }
    }

    /// Check circuit breaker conditions
    pub fn check(&mut self, portfolio: &Portfolio) -> CircuitBreakerState {
        // Check daily drawdown
        let daily_drawdown = self.calculate_daily_drawdown(portfolio);
        
        if daily_drawdown.abs() > self.max_daily_drawdown {
            self.state = CircuitBreakerState::Tripped;
            tracing::error!("Circuit breaker TRIPPED: Daily drawdown {} exceeds limit {}", 
                daily_drawdown, self.max_daily_drawdown);
            return self.state;
        }

        // Check leverage
        if portfolio.leverage > MAX_LEVERAGE {
            self.state = CircuitBreakerState::Tripped;
            tracing::error!("Circuit breaker TRIPPED: Leverage {} exceeds limit {}", 
                portfolio.leverage, MAX_LEVERAGE);
            return self.state;
        }

        // Check Hamiltonian energy
        let energy = axiom_risk::hamiltonian::calculate_hamiltonian_energy(portfolio);
        if energy > DELTA_U_MAX_SQ {
            self.state = CircuitBreakerState::Warning;
            tracing::warn!("Circuit breaker WARNING: Energy {} exceeds threshold", energy);
        } else {
            self.state = CircuitBreakerState::Normal;
        }

        self.state
    }

    /// Calculate daily drawdown
    fn calculate_daily_drawdown(&self, portfolio: &Portfolio) -> Decimal {
        // Simplified: compare current equity to equity at start of day
        // In production, would track peak equity throughout the day
        if self.daily_pnl_history.is_empty() {
            return Decimal::ZERO;
        }

        let start_equity = self.daily_pnl_history.front()
            .map(|(_, equity)| *equity)
            .unwrap_or(portfolio.equity);

        (portfolio.equity - start_equity) / start_equity
    }

    /// Record daily PnL snapshot
    pub fn record_snapshot(&mut self, portfolio: &Portfolio) {
        self.daily_pnl_history.push_back((Utc::now(), portfolio.equity));
        
        // Keep only last 24 hours
        let cutoff = Utc::now() - Duration::hours(24);
        while let Some(&(time, _)) = self.daily_pnl_history.front() {
            if time < cutoff {
                self.daily_pnl_history.pop_front();
            } else {
                break;
            }
        }
    }

    /// Reset circuit breaker (start of new day)
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Normal;
        self.daily_pnl_history.clear();
        self.last_reset = Utc::now();
    }

    /// Get current state
    pub fn state(&self) -> CircuitBreakerState {
        self.state
    }
}

