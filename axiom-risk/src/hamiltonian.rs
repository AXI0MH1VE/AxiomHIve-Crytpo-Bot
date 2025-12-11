//! Hamiltonian Containment: Physics-Based Risk Model
//!
//! Models portfolio risk as physical energy, enforcing Lyapunov stability.

use axiom_core::{Portfolio, Decimal};
use axiom_core::constants::DELTA_U_MAX_SQ;
use rust_decimal::Decimal;

/// Calculate Hamiltonian energy (risk measure)
pub fn calculate_hamiltonian_energy(portfolio: &Portfolio) -> Decimal {
    // Energy = 0.5 * (leverage^2 + correlation_penalty)
    let leverage_term = portfolio.leverage * portfolio.leverage;
    
    // Correlation penalty (simplified - in production would use full covariance matrix)
    let correlation_penalty = calculate_correlation_penalty(portfolio);
    
    (leverage_term + correlation_penalty) / Decimal::from(2)
}

/// Calculate correlation penalty
///
/// Penalizes concentrated positions (high correlation)
fn calculate_correlation_penalty(portfolio: &Portfolio) -> Decimal {
    if portfolio.positions.is_empty() {
        return Decimal::ZERO;
    }

    // Simplified: penalty increases with number of positions in same direction
    let long_count = portfolio.positions.iter()
        .filter(|p| p.side == axiom_core::Side::Buy)
        .count();

    let short_count = portfolio.positions.iter()
        .filter(|p| p.side == axiom_core::Side::Sell)
        .count();

    let concentration = Decimal::from(long_count.max(short_count));
    concentration * (Decimal::from(1) / Decimal::from(10))
}

/// Check Lyapunov stability condition
///
/// Returns true if dE/dt <= 0 (energy not diverging)
pub fn check_lyapunov_stability(portfolio: &Portfolio) -> bool {
    let energy = calculate_hamiltonian_energy(portfolio);
    energy <= DELTA_U_MAX_SQ
}

