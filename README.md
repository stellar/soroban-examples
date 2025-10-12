# Soroban Examples <!-- omit in toc -->

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/stellar/soroban-examples)

This repository contains example smart contracts for key Soroban features and concepts. The examples illustrate how to use the features, in their simplest form. 

> [!WARNING]  
> These implementations are educational examples, and have not been tested or audited. They are likely to have significant errors and security vulnerabilities. They should not be relied on for any purpose. Please refer to the license for more information.

The examples in this repository:

- **account**: This a basic multi-sig account contract that with a customizable per-token authorization policy
- **alloc**: Allocates a temporary vector holding values (0..count), then computes and returns their sum
atomic_multiswap**: This contract performs a batch of atomic token swaps between multiple parties and does a simple price matching
- **atomic_swap**: This contract performs an atomic token swap between two parties that don't need to know each other 
- **auth**: This contract demonstrates how to implement authorization using Soroban-managed auth framework for a simple case
- **bls_signature**: This is a basic custom account contract that implements the FastAggregateVerify function in BLS Signatures
- **cross_contract**: Demonstrates how to make cross contract calls
- **custom_types**: A basic increment contract that implements a custom type
- **deep_contract_auth**: This example demonstrates how a contract can authorize deep subcontract calls on its behalf
- **deployer**: This contract deploys another contract Wasm and after deployment it invokes the initialization function of the contract
- **errors**: This contract demonstrates how to define and generate errors in a contract that invokers of the contract can understand and handle
- **eth_abi**: Demonstrates how to decode contract specs in the Application Binary Interface format
- **events**: This contract demonstrates how to publish events from a contract 
- **fuzzing**: This is the 'timelock' example modified slightly to demonstrate Soroban's fuzzing capabilities.
- **hello_world**: The simplest smart contract, it takes a parameter value and add it to a vector and returns it
- **increment**: Demonstrates how to increment a stored value and returning the updated value
- **liquidity_pool**: A minimalistic implementation of a liquidity pool and token swap
- **logging**: A basic example of how to use the standard Soroban terminal logging
- **merkle_distribution**: A Merkle distribution contract that verifies Merkle proofs to distribute tokens efficiently to eligible recipients
- **mint-lock**: Demonstrates token minting, including minting authorization
- **other_custom_types**: The smart contract implements types, including custom types
- **simple_account**: A minimal example of an account contract, owned by a single ed25519 public key
- **single_offer**: This contract implements trading of one token pair between one seller and multiple buyers
- **time_lock**: This contract demonstrates how to write a timelock and implements a greatly simplified claimable balance
- **token**: This contract demonstrates how to write a token contract that implements the Token Interface.
- **ttl**: The contract demonstrates how TTL can be extended for stored keys
- **upgradeable_contract**: This contract demonstrates how to upgrade the Wasm bytecode using example contracts
- **workspace**: This contract demonstrates how multiple smart contracts can be developed, tested, and built side-by-side in the same Rust workspace

## Get Started
The easiest way to get started experimenting with the example smart contracts, is to use Devcontainers. Run the smart 
contracts directly in a browser-based IDE or using a Devcontainer as your local VSCode backend, without any config
or DevOps overhead.

<div style="text-align: center;" align="center">
<strong>Devcontainers</strong>
</div><br/>

<div align="center">
<a href="https://github.com/codespaces/new?repo=stellar/soroban-examples&editor=web">
  <img src="https://github.com/codespaces/badge.svg" alt="Open in Codespaces">
</a>
</div>
<div align="center">
<a href="https://app.codeanywhere.com/#https://github.com/stellar/soroban-examples">
  <img src="https://codeanywhere.com/img/open-in-codeanywhere-btn.svg" alt="Open in Codeanywhere">
</a>
</div>

**Learn more about how Devcontainers are used in this repo:**
- Running [Devcontainers Locally](./devcontainer.md)
- Check out the [Devcontainer config](./.devcontainer/devcontainer.json)

## Installation
Stellar smart contracts are written in the [Rust](https://www.rust-lang.org/) programming language and can be deployed to the testnet or mainnet. 

### Prerequisites
To build and develop contracts you need only a couple prerequisites:

- A [Rust](https://www.rust-lang.org/) toolchain
- An editor that supports Rust
- [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#install-the-stellar-cli)

See the [documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) for more prerequisites installation instructions. 

#### Create Identity
If an identity for signing transactions has already been created, this part can be skipped. 

When deploying a smart contract to a network, an identity that will be used to sign the transactions must be specified. Let's configure an identity called alice. Any name can be used, but it might be convenient to have some named identities for testing, such as alice, bob, and carol. Notice that the account will be funded using [Friendbot](https://developers.stellar.org/docs/learn/fundamentals/networks#friendbot). 

```
stellar keys generate --global alice --network testnet --fund
```

Get the public key of alice with this command: 

```
stellar keys address alice
```

See the [documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#configure-an-identity) for more information about identities.

### Clone Contracts
The example smart contracts don’t need installation, simply clone the repo:

```
git clone https://github.com/stellar/soroban-examples
```

*Note all smart contract examples are cloned, not individual contracts.*

### Run Smart Contracts
*Note: The `increment` contract is used in these instructions, but the instructions are similar for the other contracts, except for how to invoke the contracts.*

The smart contracts can easily be run by deploying them to testnet. Choose a contract and follow these instructions. 

#### Build
First the smart contract must be built with this command from the `increment` contract’s root folder:

```
cd increment
stellar contract build
```

A `.wasm` file will be outputted in the target directory, at `target/wasm32v1-none/release/soroban_increment_contract.wasm`. The `.wasm` file is the built contract.

#### Deploy
The WASM file can now be deployed to the testnet by running this command:

```
stellar contract deploy \
  --wasm target/wasm32v1-none/release/soroban_increment_contract.wasm \
  --source alice \
  --network testnet \
  --alias increment_contract
```

When the smart contract has been successfully deployed, the command will return the contract’s ID (e.g. CACDYF3CYMJEJTIVFESQYZTN67GO2R5D5IUABTCUG3HXQSRXCSOROBAN). This ID can be used to invoke the contract, but since an alias is added, the alias can be used for invoking the contract as well.

#### Invoke
Now the contract is on testnet, it can be invoked. For the increment contract there’s only one function to invoke, and that’s the increment() function. Look at the code for the other contracts to see which function to invoke as every example contract is different.

Run this command to invoke the increment contract (the added alias is used as the contract ID):

```
stellar contract invoke \
  --id increment_contract \
  --source alice \
  --network testnet \
  -- \
  increment 
```

The contract will return 1 the first time it’s run, run it again and see the returned value is being incremented.

## Testing
Each of the example smart contracts also has a test file that has test cases for each of the features of the smart contracts. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. 

From the root of the contract (e.g. `increment`) run this command:

```
cargo test
```

Some examples may contain multiple contracts and require contracts to be built before the test can be run. See the individual example contracts for details.

## Licence
The example smart contracts are licensed under the Apache 2.0 license. See the LICENSE file for details.

## Contributions
Contributions are welcome, please create a pull request with the following information: 

- Explain the changes/additions you made
- Why are these changes/additions needed or relevant?
- How did you solve the problem, or created the suggested feature?
- Have your changes/additions been thoroughly tested?

## Relevant Links:
- [Smart Contract Documentation](https://developers.stellar.org/docs/build)
- [Getting Started Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
- [Example descriptions in the documentation](https://developers.stellar.org/docs/build/smart-contracts/example-contracts)
- [Stellar Developers Discord server](https://discord.gg/stellardev)

