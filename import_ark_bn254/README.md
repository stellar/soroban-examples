# Soroban ark-bn254 Contract Example

This example demonstrates how to import and use external cryptographic libraries in a Soroban smart contract. Specifically, it imports the [ark-bn254](https://crates.io/crates/ark-bn254) library and performs the pairing operation.

The main goal is to enable contract developers to experiment with computationally expensive features (e.g., cryptography) that aren't supported in the Soroban host.

## Features

- Imports and uses the `ark-bn254` library for cryptographic operations in a user contract
- Demonstrates testing the contract in both native and WASM modes
- Shows budget for WASM execution

## How to Run

```bash
# Build the contract
make build

# Run the tests
make test
```