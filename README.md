# x402-sdk-solana-rust

[![Crates.io](https://img.shields.io/crates/v/x402-sdk-solana-rust.svg)](https://crates.io/crates/x402-sdk-solana-rust)
[![Documentation](https://docs.rs/x402-sdk-solana-rust/badge.svg)](https://docs.rs/x402-sdk-solana-rust)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A Rust SDK for the X402 payment protocol on Solana, enabling pay-per-use APIs with automatic blockchain-based payment handling.

## Overview

X402 is a payment protocol that allows API providers to charge users on a per-request basis using blockchain payments. This SDK provides three main components:

- **Client**: HTTP client with automatic payment handling (402 Payment Required)
- **Server**: Payment-protected API server with middleware
- **Facilitator**: Payment verification and settlement service

## Features

- 🔐 Automatic payment handling for HTTP 402 responses
- ⚡ Async/await support with Tokio
- 🔗 Solana blockchain integration
- 💰 Support for SOL transfers (SPL Token support planned)
- 🛡️ Built-in signature verification
- 📝 Comprehensive error handling
- 🔧 Easy configuration via environment variables

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
x402-sdk-solana-rust = "0.1.0"
```

## Quick Start

### Client Example

```rust
use x402_sdk_solana_rust::{PaymentClient, solana::Wallet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize wallet from private key
    let wallet = Wallet::from_private_key("your-private-key")?;
    
    // Create payment client
    let client = PaymentClient::new(
        wallet,
        "your-receiver-address",
        100000, // max payment (0.1 USDC in atomic units)
        "https://api.mainnet-beta.solana.com",
        Some("http://localhost:8081/api/settle"),
    );
    
    // Make request with automatic payment handling
    let response = client.get("http://localhost:8080/api/data").await?;
    println!("Response: {}", response.text().await?);
    
    Ok(())
}
```

### Server Example

```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use x402_sdk_solana_rust::server::PaymentMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(PaymentMiddleware::new(
                "your-public-key",
                "http://localhost:8081/api/verify",
                "1800", // price in atomic units
            ))
            .route("/api/data", web::get().to(|| async {
                HttpResponse::Ok().json(serde_json::json!({"data": "value"}))
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Facilitator Example

See `examples/facilitator_example.rs` for a complete payment verification and settlement service.

## Configuration

All examples support `.env` files for configuration:

### Client (.env)
```env
WALLET_PRIVATE_KEY=your_private_key_base58
RECEIVER_ADDRESS=receiver_public_key
MAX_PAYMENT=100000
RPC_URL=https://api.mainnet-beta.solana.com
FACILITATOR_URL=http://localhost:8081/api/settle
TARGET_URL=http://localhost:8080/api/data
```

### Server (.env)
```env
SERVER_PORT=8080
PUBLIC_KEY=your_public_key
FACILITATOR_URL=http://localhost:8081/api/verify
PRICE=1800
```

### Facilitator (.env)
```env
FACILITATOR_PORT=8081
RPC_URL=https://api.mainnet-beta.solana.com
EXPECTED_RECEIVER=receiver_public_key
WALLET_PRIVATE_KEY=facilitator_private_key_base58
TOKEN_DECIMALS=6
```

## Examples

Run the examples:

```bash
# Start facilitator
cargo run --example facilitator_example

# Start server (in another terminal)
cargo run --example server_example

# Run client (in another terminal)
cargo run --example client_example
```

## Architecture

```
┌─────────┐         ┌─────────┐         ┌──────────────┐
│ Client  │ ──402──▶│ Server  │ ──────▶ │ Facilitator  │
│         │ ◀─sig──┤         │ ◀verify─┤              │
│         │         └─────────┘         │              │
│         │                              │              │
│         │ ──tx──▶ Solana Chain ◀─────┤              │
└─────────┘                              └──────────────┘
```

1. Client requests API endpoint
2. Server returns 402 with payment challenge
3. Client creates and signs payment transaction
4. Client sends transaction to facilitator for settlement
5. Facilitator verifies and submits to Solana
6. Client retries request with payment proof

## Current Limitations

- ⚠️ Currently supports SOL transfers only (not SPL Tokens/USDC)
- To use USDC/SPL Tokens, modify client to call `create_token_transfer_transaction`
- Server configured for USDC decimals (6) but client uses SOL transfers (9 decimals)

## Testing

Run unit tests:
```bash
cargo test --lib
```

Run integration tests:
```bash
cargo test --test integration_tests
```

## Documentation

Full API documentation is available at [docs.rs/x402-sdk-solana-rust](https://docs.rs/x402-sdk-solana-rust)

For detailed guides, see:
- [API Reference](docs/API.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Configuration Guide](docs/CONFIGURATION.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

This SDK is inspired by the TypeScript implementation at [x402-sdk-for-solana](https://github.com/xilibi2003/x402-sdk-for-solana).

## Support

- 📖 [Documentation](https://docs.rs/x402-sdk-solana-rust)
- 🐛 [Issue Tracker](https://github.com/zhiming817/x402-sdk-solana-rust/issues)
- 💬 [Discussions](https://github.com/zhiming817/x402-sdk-solana-rust/discussions)