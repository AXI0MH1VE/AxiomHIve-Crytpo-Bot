//! System Constants: The Immutable Foundation
//!
//! All constants that define the operational parameters of Axiom Hive.
//! These are compiled into the binary to prevent runtime modification.

use rust_decimal::Decimal;

/// The deterministic seed for all random number generation
pub const DETERMINISTIC_SEED: u64 = 42;

/// Maximum acceptable consistency error (C=0 mandate)
pub const MAX_CONSISTENCY_ERROR: Decimal = Decimal::ZERO;

use rust_decimal_macros::dec;

/// Maximum position size per symbol (in base currency)
pub const MAX_POSITION_SIZE_BTC: Decimal = dec!(10.0);
pub const MAX_POSITION_SIZE_ETH: Decimal = dec!(100.0);
pub const MAX_POSITION_SIZE_SOL: Decimal = dec!(1000.0);

/// Maximum order size per symbol
pub const MAX_ORDER_SIZE_BTC: Decimal = dec!(1.0);
pub const MAX_ORDER_SIZE_ETH: Decimal = dec!(10.0);
pub const MAX_ORDER_SIZE_SOL: Decimal = dec!(100.0);

/// Maximum portfolio leverage (gross)
pub const MAX_LEVERAGE: Decimal = dec!(3.0);

/// Per-trade risk budget (as fraction of equity)
pub const MIN_RISK_BUDGET: Decimal = dec!(0.0025); // 0.25%
pub const MAX_RISK_BUDGET: Decimal = dec!(0.01); // 1.0%

/// Daily maximum drawdown threshold (triggers circuit breaker)
pub const MAX_DAILY_DRAWDOWN: Decimal = dec!(0.03); // 3%

/// Lyapunov stability threshold (Hamiltonian energy)
pub const DELTA_U_MAX_SQ: Decimal = dec!(0.000000000001); // 1e-12

/// Maximum acceptable slippage (as fraction of mid price)
pub const MAX_SLIPPAGE_TOLERANCE: Decimal = dec!(0.001); // 0.1%

/// Minimum liquidity requirement (in quote currency)
pub const MIN_LIQUIDITY_USD: Decimal = dec!(10000.0);

/// Hallucination rate threshold (triggers model hot-swap)
pub const MAX_HALLUCINATION_RATE: Decimal = dec!(0.0001); // 0.01%

/// Data ingestion latency targets (milliseconds)
pub const TARGET_LATENCY_ORDERBOOK: u64 = 5;
pub const TARGET_LATENCY_ONCHAIN: u64 = 50;
pub const TARGET_LATENCY_ONTOLOGY: u64 = 200;

/// Supported trading pairs
pub const SUPPORTED_PAIRS: &[&str] = &["BTC/USD", "ETH/USD", "SOL/USD"];

/// Supported venues
pub const SUPPORTED_VENUES: &[&str] = &["binance", "bybit", "hyperliquid"];

