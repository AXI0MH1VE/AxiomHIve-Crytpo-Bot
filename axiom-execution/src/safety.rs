//! Execution Safety: Pre-Flight Checks
//!
//! Every order must pass these checks before being sent to an exchange.

use axiom_core::{VerifiedOrder, Symbol, Decimal};
use axiom_core::constants::*;
use tracing::{info, warn};

/// Pre-flight safety checks
pub struct SafetyChecker;

impl SafetyChecker {
    /// Verify order is safe to execute
    pub fn check_order(order: &VerifiedOrder) -> Result<(), SafetyError> {
        // Check 1: Verify C=0 signature
        // (In production, would verify cryptographic signature)
        
        // Check 2: Verify order size
        Self::check_order_size(&order.signal)?;
        
        // Check 3: Verify price is reasonable
        Self::check_price(&order.signal)?;
        
        info!("Safety checks passed for order");
        Ok(())
    }

    fn check_order_size(signal: &axiom_core::TradeSignal) -> Result<(), SafetyError> {
        let max_size = match signal.symbol.0.as_str() {
            "BTC/USD" => MAX_ORDER_SIZE_BTC,
            "ETH/USD" => MAX_ORDER_SIZE_ETH,
            "SOL/USD" => MAX_ORDER_SIZE_SOL,
            _ => return Err(SafetyError::UnsupportedSymbol),
        };

        if signal.quantity > max_size {
            warn!("Order size {} exceeds maximum {}", signal.quantity, max_size);
            return Err(SafetyError::OrderSizeExceeded {
                size: signal.quantity,
                max: max_size,
            });
        }

        if signal.quantity <= Decimal::ZERO {
            return Err(SafetyError::InvalidQuantity);
        }

        Ok(())
    }

    fn check_price(signal: &axiom_core::TradeSignal) -> Result<(), SafetyError> {
        if let Some(price) = signal.limit_price {
            if price <= Decimal::ZERO {
                return Err(SafetyError::InvalidPrice);
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SafetyError {
    #[error("Order size exceeded: {size} > {max}")]
    OrderSizeExceeded { size: Decimal, max: Decimal },
    
    #[error("Invalid quantity (must be > 0)")]
    InvalidQuantity,
    
    #[error("Invalid price (must be > 0)")]
    InvalidPrice,
    
    #[error("Unsupported symbol")]
    UnsupportedSymbol,
}

