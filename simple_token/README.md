# Simple Token Example <!-- omit in toc -->

This example demonstrates a minimal, easy-to-understand token smart contract on Soroban with a **finite supply** and **direct transfer functionality**, intentionally omitting advanced features like allowances and administrative minting.

## Overview

While the standard [`token`](../token) example provides a complete implementation of the full SEP-41 Token Interface (including allowances, admin roles, and authorization delegation), this contract serves as a foundational learning tool for new developers to understand core Soroban concepts:

- **Constructor Pattern (`__constructor`)**: Setting up token metadata, total supply, and crediting the initial owner once at deployment time.
- **Persistent Storage Management**: Managing balances and token parameters using contract instance storage.
- **Finite Total Supply**: All tokens are minted during deployment to the designated initial owner; the total supply is fixed and immutable thereafter.
- **Caller Authorization (`require_auth`)**: Protecting the sender's balance so that only the legitimate owner can initiate a transfer.
- **Safe Transfers & Event Emission**: Validating positive transfer amounts and sufficient balances, followed by publishing standard transfer events.

## Contract Interface

- `__constructor(initial_owner: Address, initial_supply: i128, name: String, symbol: String, decimal: u32)`: Initializes token parameters and credits the full finite supply to `initial_owner`.
- `balance(id: Address) -> i128`: Returns the balance of the specified address (or `0` if empty).
- `transfer(from: Address, to: Address, amount: i128)`: Transfers `amount` from `from` to `to`, requiring authorization from `from`.
- `total_supply() -> i128`: Returns the immutable total supply.
- `name() -> String`: Returns the human-readable token name.
- `symbol() -> String`: Returns the ticker symbol.
- `decimals() -> u32`: Returns the number of decimal places (max 18).

## How to Run the Tests

To build the contract and run the automated unit test suite:

```bash
cargo test
```
