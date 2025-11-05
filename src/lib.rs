//! # X402 SDK for Solana (Rust)
//!
//! A Rust SDK for the X402 payment protocol on Solana, enabling pay-per-use APIs 
//! with automatic blockchain-based payment handling.
//!
//! ## Overview
//!
//! X402 is a payment protocol that allows API providers to charge users on a per-request 
//! basis using blockchain payments. This SDK provides three main components:
//!
//! - **Client**: HTTP client with automatic payment handling (402 Payment Required)
//! - **Server**: Payment-protected API server with middleware
//! - **Facilitator**: Payment verification and settlement service
//!
//! ## Features
//!
//! - 🔐 Automatic payment handling for HTTP 402 responses
//! - ⚡ Async/await support with Tokio
//! - 🔗 Solana blockchain integration
//! - 💰 Support for SOL transfers (SPL Token support planned)
//! - 🛡️ Built-in signature verification
//! - 📝 Comprehensive error handling
//! - 🔧 Easy configuration via environment variables
//!
//! ## Quick Start
//!
//! ### Client Example
//!
//! ```rust,no_run
//! use x402_sdk_solana_rust::{client::PaymentClient, solana::Wallet};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize wallet from private key
//!     let wallet = Wallet::from_private_key("your-private-key")?;
//!     
//!     // Create payment client
//!     let client = PaymentClient::new(
//!         wallet,
//!         "receiver-address",
//!         100000, // max payment in atomic units
//!         "https://api.mainnet-beta.solana.com",
//!         Some("http://localhost:8081/api/settle"),
//!     );
//!     
//!     // Make request with automatic payment handling
//!     let response = client.get("http://localhost:8080/api/data").await?;
//!     println!("Response: {}", response.text().await?);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Server Example
//!
//! ```rust,no_run
//! use actix_web::{web, App, HttpServer, HttpResponse};
//! use x402_sdk_solana_rust::server::PaymentMiddleware;
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| {
//!         App::new()
//!             .wrap(PaymentMiddleware::new(
//!                 "your-public-key",
//!                 "http://localhost:8081/api/verify",
//!                 "1800", // price in atomic units
//!             ))
//!             .route("/api/data", web::get().to(|| async {
//!                 HttpResponse::Ok().json(serde_json::json!({"data": "value"}))
//!             }))
//!     })
//!     .bind("127.0.0.1:8080")?
//!     .run()
//!     .await
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────┐         ┌─────────┐         ┌──────────────┐
//! │ Client  │ ──402──▶│ Server  │ ──────▶ │ Facilitator  │
//! │         │ ◀─sig──┤         │ ◀verify─┤              │
//! │         │         └─────────┘         │              │
//! │         │                              │              │
//! │         │ ──tx──▶ Solana Chain ◀─────┤              │
//! └─────────┘                              └──────────────┘
//! ```
//!
//! ## Configuration
//!
//! All examples support `.env` files. See the `examples/` directory for complete 
//! configuration templates.
//!
//! ## Current Limitations
//!
//! - Currently supports SOL transfers only (not SPL Tokens/USDC)
//! - To use USDC/SPL Tokens, modify client to call `create_token_transfer_transaction`
//!
//! ## Examples
//!
//! See the `examples/` directory for complete working examples:
//! - `client_example.rs` - HTTP client with automatic payments
//! - `server_example.rs` - Payment-protected API server
//! - `facilitator_example.rs` - Payment verification service

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