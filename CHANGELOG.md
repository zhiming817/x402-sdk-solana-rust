# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2025-01-XX

### Changed
- **BREAKING**: Upgraded Solana SDK from 1.18 to 3.0
  - Updated `solana-sdk` to version 3.0
  - Updated `solana-client` to version 3.0
  - Updated `solana-program` to version 3.0
- **BREAKING**: Upgraded SPL Token dependencies to support Solana 3.0
  - Updated `spl-token` from 4.0 to 9.0
  - Updated `spl-associated-token-account` from 6.0 to 8.0
- Added `solana-system-interface` 3.0 with bincode feature for system instructions

### Fixed
- Updated Keypair API calls to use `from_base58_string()` instead of deprecated `from_bytes()`
- Fixed system instruction imports to use `solana-system-interface` crate
- Resolved type compatibility between Pubkey and Address types through spl-associated-token-account 8.0

### Dependencies
- solana-sdk: 1.18 → 3.0
- solana-client: 1.18 → 3.0
- solana-program: 1.18 → 3.0
- spl-token: 4.0 → 9.0
- spl-associated-token-account: 6.0 → 8.0
- Added: solana-system-interface: 3.0 (with bincode feature)

### Notes
- This release maintains compatibility with existing X402 payment protocol
- All core functionality (SOL transfer, SPL Token transfer) has been verified
- See `UPGRADE_SOLANA_3.0.md` for detailed upgrade documentation

## [0.1.3] - Previous Release

Initial release with Solana 1.18 support.
