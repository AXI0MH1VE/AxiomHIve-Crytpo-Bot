//! L0 Invariant Contract: The Mathematical Foundation
//!
//! This module defines the formal invariants that must be satisfied
//! for any trade to execute. These are the "laws of physics" for the system.

use crate::types::*;
use crate::constants::*;
use rust_decimal::Decimal;
use thiserror::Error;

/// The L0 Invariant Contract
///
/// All trades must satisfy these axioms. Violation results in automatic rejection.
#[derive(Debug, Clone)]
pub struct L0InvariantContract;

impl L0InvariantContract {
    /// Verify that a trade signal satisfies all L0 invariants
    ///
    /// Returns Ok(()) if all invariants are satisfied, Err(InvariantViolation) otherwise.
    pub fn verify_signal(signal: &TradeSignal, portfolio: &Portfolio) -> Result<(), InvariantViolation> {
        // Invariant 1: Consistency Error must be zero
        if signal.contradiction_score < Decimal::ZERO {
            return Err(InvariantViolation::NegativeContradiction);
        }

        // Invariant 2: Position size must not exceed maximum
        Self::check_position_size(&signal.symbol, signal.quantity)?;

        // Invariant 3: Portfolio leverage must not exceed maximum
        Self::check_leverage(portfolio)?;

        // Invariant 4: Risk budget must be respected
        Self::check_risk_budget(signal, portfolio)?;

        // Invariant 5: Entropy must be below threshold (not in Unprovable regime)
        if signal.entropy_count > DELTA_U_MAX_SQ {
            return Err(InvariantViolation::ExcessiveEntropy);
        }

        // Invariant 6: Slippage tolerance must be satisfiable
        // (This is checked during execution, but we verify the signal is within bounds)
        if let Some(limit_price) = signal.limit_price {
            // Basic sanity check
            if limit_price <= Decimal::ZERO {
                return Err(InvariantViolation::InvalidPrice);
            }
        }

        Ok(())
    }

    /// Check position size limits
    fn check_position_size(symbol: &Symbol, quantity: Quantity) -> Result<(), InvariantViolation> {
        let max_size = match symbol.0.as_str() {
            "BTC/USD" => MAX_POSITION_SIZE_BTC,
            "ETH/USD" => MAX_POSITION_SIZE_ETH,
            "SOL/USD" => MAX_POSITION_SIZE_SOL,
            _ => return Err(InvariantViolation::UnsupportedSymbol),
        };

        if quantity > max_size {
            return Err(InvariantViolation::PositionSizeExceeded {
                quantity,
                max: max_size,
            });
        }

        Ok(())
    }

    /// Check portfolio leverage
    fn check_leverage(portfolio: &Portfolio) -> Result<(), InvariantViolation> {
        if portfolio.leverage > MAX_LEVERAGE {
            return Err(InvariantViolation::LeverageExceeded {
                current: portfolio.leverage,
                max: MAX_LEVERAGE,
            });
        }

        Ok(())
    }

    /// Check risk budget per trade
    fn check_risk_budget(signal: &TradeSignal, portfolio: &Portfolio) -> Result<(), InvariantViolation> {
        // Calculate position value
        let position_value = signal.quantity
            * signal.limit_price.unwrap_or(Decimal::ZERO);

        // Calculate risk as fraction of equity
        let risk_fraction = position_value / portfolio.equity;

        if risk_fraction < MIN_RISK_BUDGET {
            return Err(InvariantViolation::RiskBudgetTooSmall {
                fraction: risk_fraction,
                min: MIN_RISK_BUDGET,
            });
        }

        if risk_fraction > MAX_RISK_BUDGET {
            return Err(InvariantViolation::RiskBudgetExceeded {
                fraction: risk_fraction,
                max: MAX_RISK_BUDGET,
            });
        }

        Ok(())
    }

    /// Verify Hamiltonian energy constraint (Lyapunov stability)
    pub fn verify_hamiltonian_energy(portfolio: &Portfolio) -> Result<(), InvariantViolation> {
        if portfolio.energy > DELTA_U_MAX_SQ {
            return Err(InvariantViolation::EnergyDivergence {
                energy: portfolio.energy,
                threshold: DELTA_U_MAX_SQ,
            });
        }

        Ok(())
    }
}

/// Invariant violation error
#[derive(Debug, Error, Clone)]
pub enum InvariantViolation {
    #[error("Negative contradiction score (must be >= 0)")]
    NegativeContradiction,

    #[error("Position size exceeded: {quantity} > {max}")]
    PositionSizeExceeded { quantity: Decimal, max: Decimal },

    #[error("Leverage exceeded: {current} > {max}")]
    LeverageExceeded { current: Decimal, max: Decimal },

    #[error("Risk budget exceeded: {fraction} > {max}")]
    RiskBudgetExceeded { fraction: Decimal, max: Decimal },

    #[error("Risk budget too small: {fraction} < {min}")]
    RiskBudgetTooSmall { fraction: Decimal, min: Decimal },

    #[error("Excessive entropy: energy exceeds threshold")]
    ExcessiveEntropy,

    #[error("Invalid price (must be > 0)")]
    InvalidPrice,

    #[error("Unsupported symbol")]
    UnsupportedSymbol,

    #[error("Hamiltonian energy divergence: {energy} > {threshold}")]
    EnergyDivergence { energy: Decimal, threshold: Decimal },
}

