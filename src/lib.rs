// This file serves as the entry point for the x402 SDK for Solana.
// It defines the public interface and modules for the library.

pub mod client;
pub mod server;
pub mod facilitator;
pub mod solana;
pub mod types;
pub mod error;
pub mod utils;

// Re-export commonly used items
pub use error::X402Error;
pub use types::{
    Network, PaymentPayload, PaymentRequirements, PaymentScheme, X402Config, 
    FacilitatorConfig, RouteConfig, SvmConfig, TokenConfig,
};
pub use client::{Fetcher, create_payment_header};
pub use server::{check_payment, settle_payment, PaymentMiddlewareConfig};
pub use facilitator::Handler;
pub use solana::{Wallet, TransactionBuilder, create_signer};