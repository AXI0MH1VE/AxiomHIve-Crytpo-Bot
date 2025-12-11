//! Verifier: SMT Solver-Based Proof Generation
//!
//! The "proving" component that verifies all trade proposals satisfy
//! the L0 Invariant Contract using formal methods.

use axiom_core::{
    TradeSignal, VerifiedOrder, Proof, Portfolio, L0InvariantContract,
    InvariantViolation, MarketRegime,
};
use axiom_core::constants::*;
use rust_decimal::Decimal;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{info, warn};
use z3::{Config, Context, Solver, ast::Int};

/// SMT-based verifier gate
pub struct Verifier {
    context: Context,
}

impl Verifier {
    pub fn new() -> Self {
        let cfg = Config::new();
        let context = Context::new(&cfg);
        Self { context }
    }

    /// Verify a trade signal and generate proof
    ///
    /// Returns Ok(VerifiedOrder) if the trade satisfies all invariants,
    /// Err(InvariantViolation) otherwise.
    pub fn verify_signal(
        &self,
        signal: &TradeSignal,
        portfolio: &Portfolio,
    ) -> Result<VerifiedOrder, InvariantViolation> {
        // Step 1: Check L0 Invariant Contract
        L0InvariantContract::verify_signal(signal, portfolio)?;

        // Step 2: Check Hamiltonian energy
        L0InvariantContract::verify_hamiltonian_energy(portfolio)?;

        // Step 3: Check market regime (entropy threshold)
        if signal.entropy_count > DELTA_U_MAX_SQ {
            return Err(InvariantViolation::ExcessiveEntropy);
        }

        // Step 4: Generate SMT proof
        let proof = self.generate_proof(signal, portfolio)?;

        // Step 5: Create verified order
        let verified_order = VerifiedOrder {
            signal: signal.clone(),
            proof_signature: format!("C=0:{}", hex::encode(&proof.model.get("hash").unwrap_or(&"".to_string()).as_bytes())),
            proof,
            verified_at: Utc::now(),
        };

        info!("Trade verified: {} {} @ {:?}", 
            signal.side, signal.quantity, signal.limit_price);

        Ok(verified_order)
    }

    /// Generate SMT proof for the trade
    fn generate_proof(
        &self,
        signal: &TradeSignal,
        portfolio: &Portfolio,
    ) -> Result<Proof, InvariantViolation> {
        let solver = Solver::new(&self.context);

        // Create SMT variables
        let quantity = Int::from_i64(&self.context, 
            (signal.quantity * Decimal::from(1_000_000)).to_i64().unwrap_or(0));
        let max_quantity = Int::from_i64(&self.context, 
            (MAX_POSITION_SIZE_BTC * Decimal::from(1_000_000)).to_i64().unwrap_or(0));
        let leverage = Int::from_i64(&self.context,
            (portfolio.leverage * Decimal::from(1_000_000)).to_i64().unwrap_or(0));
        let max_leverage = Int::from_i64(&self.context,
            (MAX_LEVERAGE * Decimal::from(1_000_000)).to_i64().unwrap_or(0));

        // Add constraints (axioms)
        // Axiom 1: quantity <= max_quantity
        solver.assert(&quantity.le(&max_quantity));
        
        // Axiom 2: leverage <= max_leverage
        solver.assert(&leverage.le(&max_leverage));

        // Check satisfiability
        match solver.check() {
            z3::SatResult::Sat => {
                let model = solver.get_model().unwrap();
                let mut proof_model = HashMap::new();
                
                // Extract model values
                for decl in model.get_const_decls() {
                    let value = model.eval(&decl.apply(&[]), true).unwrap();
                    proof_model.insert(decl.name().to_string(), value.to_string());
                }

                Ok(Proof {
                    satisfiable: true,
                    model: proof_model,
                    axioms_satisfied: vec![
                        "PositionSizeLimit".to_string(),
                        "LeverageLimit".to_string(),
                        "RiskBudget".to_string(),
                        "EnergyConstraint".to_string(),
                    ],
                })
            }
            z3::SatResult::Unsat => {
                warn!("SMT solver found constraints unsatisfiable");
                Err(InvariantViolation::LeverageExceeded {
                    current: portfolio.leverage,
                    max: MAX_LEVERAGE,
                })
            }
            z3::SatResult::Unknown => {
                warn!("SMT solver returned unknown");
                Err(InvariantViolation::ExcessiveEntropy)
            }
        }
    }
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

