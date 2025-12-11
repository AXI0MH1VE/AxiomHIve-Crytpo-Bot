//! Signal Generation: Proposer + Verifier Pipeline
//!
//! Orchestrates the hybrid signal generation system.

use axiom_core::{TradeSignal, VerifiedOrder, OrderBook, Portfolio, Symbol, Venue};
use axiom_engine::proposer::Proposer;
use axiom_engine::verifier::Verifier;
use tracing::{info, warn};

/// Signal generator combining proposer and verifier
pub struct SignalGenerator {
    proposer: Proposer,
    verifier: Verifier,
}

impl SignalGenerator {
    pub fn new() -> Self {
        Self {
            proposer: Proposer::new(),
            verifier: Verifier::new(),
        }
    }

    /// Generate a verified trade signal
    ///
    /// Returns Some(VerifiedOrder) if a valid signal is generated,
    /// None if no opportunity is found or verification fails.
    pub fn generate_signal(
        &mut self,
        symbol: &Symbol,
        venue: &Venue,
        book: &OrderBook,
        portfolio: &Portfolio,
    ) -> Option<VerifiedOrder> {
        // Step 1: Proposer suggests a trade
        let signal = self.proposer.propose_trade(symbol, venue, book, portfolio)?;

        // Step 2: Verifier checks and proves
        match self.verifier.verify_signal(&signal, portfolio) {
            Ok(verified) => {
                info!("Signal generated and verified");
                Some(verified)
            }
            Err(e) => {
                warn!("Signal rejected by verifier: {:?}", e);
                self.proposer.record_rejection();
                None
            }
        }
    }

    /// Get current hallucination rate
    pub fn hallucination_rate(&self) -> rust_decimal::Decimal {
        self.proposer.hallucination_rate()
    }
}

impl Default for SignalGenerator {
    fn default() -> Self {
        Self::new()
    }
}

