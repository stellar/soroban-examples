# CoinUtils - Privacy Pool Coin Utilities

A modular Rust application for managing privacy pool coins, including generation, withdrawal, and association set management.

## Features

- **Coin Generation**: Create new privacy pool coins with cryptographic commitments
- **Coin Withdrawal**: Generate SNARK inputs for coin withdrawals with merkle proofs
- **Association Set Management**: Manage association sets for privacy pool operations
- **Modular Architecture**: Clean separation of concerns with well-defined modules
- **Comprehensive Testing**: Unit tests and integration tests
- **Logging Support**: Configurable logging with different levels

## Architecture

The application is organized into the following modules:

### Core Modules

- **`types/`**: Data structures for coins, state files, and SNARK inputs
- **`crypto/`**: Cryptographic operations including Poseidon hashing and conversions
- **`merkle/`**: Merkle tree operations for withdrawals and association sets
- **`io/`**: File I/O operations and serialization
- **`cli/`**: Command-line interface and argument parsing
- **`error/`**: Custom error types and error handling

### Configuration

- **`config.rs`**: Application constants and configuration values

## Usage

### Generate a Coin

```bash
stellar-coinutils generate my_pool_scope coin.json
```

### Withdraw a Coin

```bash
stellar-coinutils withdraw coin.json state.json association.json withdrawal.json
```

### Update Association Set

```bash
stellar-coinutils updateAssociation association.json "1234567890..."
```

## File Formats

### Coin File Format

```json
{
  "coin": {
    "value": "1000000000",
    "nullifier": "...",
    "secret": "...",
    "label": "...",
    "commitment": "..."
  },
  "commitment_hex": "0x..."
}
```

### State File Format

```json
{
  "commitments": ["commitment1", "commitment2", ...],
  "scope": "pool_scope"
}
```

### Association Set File Format

```json
{
  "labels": ["label1", "label2", "label3", "label4"],
  "scope": "pool_scope",
  "root": "merkle_tree_root"
}
```

## Development

### Running Tests

```bash
cargo test
```

### Running Integration Tests

```bash
cargo test --test integration_tests
```

### Building

```bash
cargo build --release
```

## Logging

The application supports configurable logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
RUST_LOG=debug stellar-coinutils generate my_scope coin.json
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

## Dependencies

- **Soroban SDK**: Stellar smart contract development
- **Poseidon**: Cryptographic hash function
- **Lean IMT**: Incremental Merkle Tree implementation
- **Clap**: Command-line argument parsing
- **Serde**: Serialization/deserialization
- **ThisError**: Error handling
- **Log/Env Logger**: Logging support

## Error Handling

The application uses a custom error type hierarchy with proper error propagation and user-friendly error messages. All operations return `Result<T, CoinUtilsError>` for consistent error handling.

## Testing

The project includes:

- **Unit Tests**: In each module for testing individual components
- **Integration Tests**: End-to-end testing of complete workflows
- **Test Data**: Sample files for testing different scenarios
