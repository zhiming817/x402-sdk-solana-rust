# x402-sdk-for-solana Rust SDK

This project is a Rust SDK for interacting with the Solana blockchain, designed to facilitate the development of applications that require blockchain functionality.

## Features

- **Client Module**: Provides functionality to fetch data from the Solana network.
- **Server Module**: Implements middleware for handling requests and responses.
- **Facilitator Module**: Manages the processing of requests.
- **Solana Module**: Contains structures and methods for transactions and wallet management.
- **Types Module**: Defines custom types used throughout the SDK.
- **Error Handling**: Custom error types for better error management.
- **Utilities**: Helper functions for common tasks.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).

### Installation

Clone the repository:

```bash
git clone <repository-url>
cd x402-sdk-solana-rust
```

Install the dependencies:

```bash
cargo build
```

### Usage

To use the SDK in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
x402-sdk-solana = { path = "path/to/x402-sdk-solana-rust" }
```

### Examples

Refer to the `examples` directory for usage examples of the client, server, and facilitator modules.

### Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the ISC License. See the LICENSE file for more details.