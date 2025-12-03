# Soroban Privacy Pools

This project is currently a work in progress and is a merely a prototype for research and educational purposes only. It does not yet constitute a secure or functional system. Features and APIs may change. It also contains known limitations and security gaps and some jurisdictions may prohibit or limit features. Any implementation requires comprehensive legal and regulatory analysis by qualified counsel. 

**Warning regarding Poseidon hash implementation**: This repository contains an implementation of the Poseidon hash. This proprietary implementation was put in place to serve this proof of concept and have a hash that is consistent between the Circom circuit and Rust code. It is currently not audited for security and should not be used as a reference code for secure Poseidon implementations.

A privacy-preserving transaction system built on Stellar using Soroban smart contracts and zero-knowledge proofs (zkSNARKs). This project implements privacy pools that allow users to deposit and withdraw tokens while maintaining transaction privacy through cryptographic commitments and Merkle tree inclusion proofs. It includes the incorporation of Association Set Providers (ASPs) that define inclusion criteria intended to help mitigate illicit use. ASPs enable participants to selectively associate with sets of other participants who meet their chosen compliance standards

> **Note**: The commitments Merkle-tree is very small in the current version of the code. It will be possible to increase the depth of the commitments tree once Poseidon hashing is added as a host function (CAP-75)[https://github.com/stellar/stellar-protocol/blob/master/core/cap-0075.md], more hashing operations to fit within a function's budget.

## Features

- **Privacy-preserving transactions**: Deposit and withdraw tokens without revealing transaction history
- **Zero-knowledge proofs**: Uses Groth16 zkSNARKs with BLS12-381 curve for cryptographic verification
- **Commitment scheme**: Cryptographic commitments using Poseidon hashing
- **Merkle tree inclusion**: Efficient proof of commitment inclusion in the pool state
- **Association sets**: A compliance feature that lets users, at withdrawal, prove membership in the group of deposits defined by a specific Association Set Provider's (ASP) policy.
- **Soroban integration**: Native integration with Stellar's smart contract platform

## Project Structure

This repository demonstrates a complete privacy pool implementation for Soroban, organized into on-chain components, reusable libraries, and command-line tools.

```
.
├── contract/                 # Soroban smart contract
│   ├── src/
│   │   ├── lib.rs            # Main contract logic
│   │   └── test.rs           # Contract tests
│   ├── Cargo.toml
│   └── Makefile
├── libs/                     # Reusable libraries (may be extracted to separate crates)
│   ├── lean-imt/             # Lean Incremental Merkle Tree implementation
│   │   ├── src/
│   │   │   ├── lib.rs        # LeanIMT core implementation
│   │   │   └── tests.rs      # LeanIMT tests
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── poseidon/             # Poseidon hash implementation (⚠️ not audited)
│   │   ├── src/
│   │   │   ├── lib.rs        # Poseidon hash functions
│   │   │   └── test.rs       # Poseidon hash tests
│   │   └── Cargo.toml
│   └── zk/                   # Zero-knowledge proof verification using BLS12-381
│       ├── src/
│       │   ├── lib.rs        # ZK proof verification logic
│       │   └── test.rs       # ZK verification tests
│       └── Cargo.toml
├── cli/                      # Command-line tools for off-chain operations
│   ├── circom2soroban/       # Converts circom artifacts to Soroban format
│   │   ├── src/main.rs       # Converts VK/proofs/public signals
│   │   └── Cargo.toml
│   └── coinutils/            # Coin generation and management CLI
│       ├── src/main.rs       # CLI for generating coins and withdrawal inputs
│       ├── README.md
│       └── Cargo.toml
├── circuits/                 # Circom circuits for zero-knowledge proofs
│   ├── commitment.circom     # Commitment hashing logic
│   ├── main.circom           # Main withdrawal verification circuit
│   ├── merkleProof.circom    # Merkle tree inclusion proof
│   ├── poseidon255.circom    # Poseidon255 hash implementation
│   ├── poseidon255_constants.circom # Poseidon255 constants
│   ├── dummy.circom          # Simplified circuit for testing
│   └── test/                 # Test circuits and utilities
│       ├── lean-imt-test/    # LeanIMT integration tests
│       ├── poseidon-test/    # Poseidon hash tests
│       └── test_*.circom     # Test circuit files
├── Cargo.toml                # Workspace configuration
├── Makefile                  # Circuit compilation commands
├── demo.sh                   # Complete demo script
└── README.md
```

## Prerequisites

- **Rust** (latest stable version)
- **Soroban CLI** for contract development
- **Node.js** and **npm** for circuit tools
- **Circom** circuit compiler
- **snarkjs** for proof generation and BLS12-381 support

### Install Dependencies

```bash
# Install circom
npm install -g circom

# Install snarkjs (with BLS12-381 support)
npm install -g snarkjs

# Install Soroban CLI
cargo install --locked soroban-cli
```

## Quick Start

### 1. Compile Circuits

```bash
# Compile all circuits with BLS12-381 curve
make .circuits
```

### 2. Generate Trusted Setup (BLS12-381)

The project uses BLS12-381 curve to match Soroban's native cryptographic primitives:

```bash
cd circuits

# Generate powers of tau for BLS12-381 (if not already available)
# Note: Use existing ceremony files or generate new ones
snarkjs powersoftau new bls12-381 20 output/pot20_0000.ptau -v
snarkjs powersoftau contribute output/pot20_0000.ptau output/pot20_0001.ptau --name="First contribution" -v
snarkjs powersoftau prepare phase2 output/pot20_0001.ptau output/pot20_final.ptau -v

# Generate circuit-specific setup for main circuit
snarkjs groth16 setup build/main.r1cs output/pot20_final.ptau output/main_0000.zkey

# Contribute to ceremony
snarkjs zkey contribute output/main_0000.zkey output/main_final.zkey --name="Your contribution" -e="random entropy"

# Export verification key
snarkjs zkey export verificationkey output/main_final.zkey output/main_verification_key.json
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific contract tests
cargo test -p privacy-pools

# Run ZK verification tests
cargo test test_coin_ownership
```

## Usage

### Circuit Development

The project includes several Circom circuits:

- **`commitment.circom`**: Implements the commitment scheme using Poseidon hashing
- **`main.circom`**: Full withdrawal circuit with Merkle tree inclusion proof
- **`merkleProof.circom`**: Lean Incremental Merkle Tree (LeanIMT) verification
- **`dummy.circom`**: Simplified circuit for testing without Merkle tree verification

### Smart Contract

The Soroban contract (`contract/`) implements:

- Deposit functionality with commitment generation
- Withdrawal with zero-knowledge proof verification using BLS12-381
- Nullifier tracking to prevent double-spending
- Association set management for membership verification
- Integration with Soroban's native BLS12-381 curve operations

### Proof Generation and Conversion

```bash
# Generate witness
node build/main_js/generate_witness.js build/main_js/main.wasm input.json witness.wtns

# Generate proof
snarkjs groth16 prove output/main_final.zkey witness.wtns proof.json public.json

# Verify proof (snarkjs)
snarkjs groth16 verify output/main_verification_key.json public.json proof.json

# Convert verification key for Soroban
cargo run --bin stellar-circom2soroban vk output/main_verification_key.json

# Convert proof for Soroban
cargo run --bin stellar-circom2soroban proof proof.json

# Convert public outputs for Soroban
cargo run --bin stellar-circom2soroban public public.json
```

### coinutils Tool

The `coinutils` utility helps generate and manage privacy pool coins with proper cryptographic commitments:

```bash
# Generate a new coin for a privacy pool
cargo run --bin stellar-coinutils generate <scope> [output_file]

# Create withdrawal inputs from an existing coin (requires state file and association set file)
cargo run --bin stellar-coinutils withdraw <coin_file> <state_file> <association_set_file> [output_file]

# Add a label to an association set
cargo run --bin stellar-coinutils updateAssociation <association_set_file> <label>
```

**Features:**
- **Coin Generation**: Creates new coins with random nullifiers and secrets
- **Commitment Calculation**: Implements the same commitment scheme as the circuits
- **Merkle Tree Integration**: Uses lean-imt for consistent merkle tree operations
- **Association Set Management**: Creates and manages association sets to verify membership
- **Withdrawal Preparation**: Generates circuit inputs with merkle tree and association set proofs
- **BLS12-381 Field Operations**: Uses arkworks for proper field arithmetic
- **Poseidon Hashing**: Matches the circuit's hash implementation

**Examples:**
```bash
# Generate a coin for "my_pool" scope
cargo run --bin stellar-coinutils generate my_pool coin.json

# Create state file with commitments
echo '{"commitments": ["commitment1", "commitment2", "..."], "scope": "my_pool"}' > state.json

# Create association set with coin's label
LABEL=$(cat coin.json | jq -r '.coin.label')
cargo run --bin stellar-coinutils updateAssociation association.json "$LABEL"

# Create withdrawal from existing coin with state file and association set
cargo run --bin stellar-coinutils withdraw coin.json state.json association.json withdrawal.json
```

**Generated Coin Structure:**
```json
{
  "coin": {
    "value": "1000000000",         
    "nullifier": "12345...",     
    "secret": "67890...",       
    "label": "24680...",       
    "commitment": "13579..."
  },
  "commitment_hex": "0xabcd..."
}
```

**State File Structure:**
```json
{
  "commitments": [
    "commitment1_hash",
    "commitment2_hash",
    "commitment3_hash"
  ],
  "scope": "pool_scope"
}
```

**Association Set File Structure:**
```json
{
  "labels": [
    "label1_hash",
    "label2_hash",
    "label3_hash",
    "label4_hash"
  ],
  "scope": "pool_scope"
}
```

**Withdrawal Input Structure:**
```json
{
  "withdrawnValue": "1000000000",
  "label": "24680...",
  "value": "1000000000",
  "nullifier": "12345...",
  "secret": "67890...",
  "stateRoot": "merkle_root_hash",
  "stateIndex": "1",
  "stateSiblings": [
    "sibling1_hash",
    "sibling2_hash",
    "0", "0", "0", "0", "0", "0", "0", "0",
    "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"
  ],
  "associationRoot": "association_merkle_root_hash",
  "labelIndex": "0",
  "labelSiblings": [
    "label_sibling1_hash",
    "label_sibling2_hash"
  ]
}
```

### circom2soroban Tool

The `circom2soroban` utility converts snarkjs artifacts to Soroban-compatible format:

```bash
# Convert verification key
cargo run --bin stellar-circom2soroban vk <verification_key.json>
# Outputs: Rust code with verification key coordinates

# Convert proof
cargo run --bin stellar-circom2soroban proof <proof.json>
# Outputs: Rust code with proof coordinates

# Convert public outputs
cargo run --bin stellar-circom2soroban public <public.json>
# Outputs: Rust code with public inputs as U256 and Fr conversion
```

Example output for public conversion:
```rust
// Public output signals:
let public_0 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x07, 0xf5, ...]));
let public_1 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x00, 0x09, ...]));

// Create output vector for verification:
let output = Vec::from_array(&env, [Fr::from_u256(public_0), Fr::from_u256(public_1)]);
```

## Development Workflow

1. **Modify circuits** in `circuits/` directory
2. **Recompile circuits** using `make .circuits`
3. **Regenerate trusted setup** if circuit structure changed
4. **Generate test coins** using `coinutils generate`
5. **Create withdrawal inputs** using `coinutils withdraw`
6. **Generate new proofs** with circuit inputs
7. **Convert artifacts** using `circom2soroban`
8. **Update tests** with new verification keys/proofs/public outputs
9. **Run tests** to verify everything works

### Complete Example Workflow

```bash
# 1. Generate a test coin
cargo run --bin stellar-coinutils generate test_pool_scope test_coin.json

# 2. Create state file with commitments (including the generated coin's commitment)
COMMITMENT=$(cat test_coin.json | jq -r '.coin.commitment')
echo "{
  \"commitments\": [
    \"$COMMITMENT\"
  ],
  \"scope\": \"test_pool_scope\"
}" > state.json

# 3. Create association set with the coin's label
LABEL=$(cat test_coin.json | jq -r '.coin.label')
cargo run --bin stellar-coinutils updateAssociation test_association.json "$LABEL"

# 4. Create withdrawal input from the coin with state file and association set
cargo run --bin stellar-coinutils withdraw test_coin.json state.json test_association.json withdrawal_input.json

# 5. Generate witness and proof
cd circuits
node build/main_js/generate_witness.js build/main_js/main.wasm ../withdrawal_input.json witness.wtns
snarkjs groth16 prove output/main_final.zkey witness.wtns proof.json public.json

# 6. Convert for Soroban
cd ..
cargo run --bin stellar-circom2soroban vk circuits/output/main_verification_key.json
cargo run --bin stellar-circom2soroban proof circuits/proof.json
cargo run --bin stellar-circom2soroban public circuits/public.json

# 7. Update test with new values and run
cargo test test_coin_ownership
```

## Testing

The project includes comprehensive tests:

- **Circuit tests**: Generate and verify proofs using snarkjs with BLS12-381
- **Contract tests**: Test deposit/withdrawal functionality
- **ZK verification tests**: Test proof verification in Soroban environment
- **Integration tests**: End-to-end privacy pool functionality

## Admin Role

The privacy pools contract implements an admin role system for secure management of association sets:

- **Admin Assignment**: The contract deployer automatically becomes the admin
- **Admin Privileges**: Only the admin can set association roots for compliance verification
- **Admin Transparency**: Anyone can query the current admin address using `get_admin()`
- **Security**: Admin functions require proper authentication and validation

### Admin Functions

```bash
# Get the current admin address
soroban contract invoke --id <CONTRACT_ID> --source <USER> --network <NETWORK> -- get_admin

# Set association root (admin only)
soroban contract invoke --id <CONTRACT_ID> --source <ADMIN> --network <NETWORK> -- set_association_root --caller <ADMIN> --association_root <ROOT_HEX>
```

## Association Sets

A compliance feature that lets users, at withdrawal, prove membership in the group of deposits defined by a specific Association Set Provider's (ASP) policy.

### How Association Sets Work

1. **Label Generation**: Each coin has a unique label derived from its scope and nonce
2. **Association Tree**: Labels are organized in a Merkle tree (depth 2, max 4 labels)
3. **Compliance Verification**: The withdrawal proof must include a valid membership proof showing that the coin's label is contained in the on-chain association root published by an Association Set Provider (ASP).

### Association Set Management

```bash
# Create or update an association set
cargo run --bin stellar-coinutils updateAssociation <association_file> <label>

# Check association set contents
cat <association_file>

# Withdraw with association set verification
cargo run --bin stellar-coinutils withdraw <coin_file> <state_file> <association_file> <output_file>
```

### Contract Integration

The contract provides methods to manage association sets:

```bash
# Set association root (admin only)
soroban contract invoke --id <CONTRACT_ID> --source <ADMIN> --network <NETWORK> -- set_association_root --caller <ADMIN> --association_root <ROOT_HEX>

# Check if association set is configured
soroban contract invoke --id <CONTRACT_ID> --source <USER> --network <NETWORK> -- has_association_set

# Get current association root
soroban contract invoke --id <CONTRACT_ID> --source <USER> --network <NETWORK> -- get_association_root

# Get the contract admin address
soroban contract invoke --id <CONTRACT_ID> --source <USER> --network <NETWORK> -- get_admin
```

## Security Considerations

- **Trusted Setup**: The project uses Groth16 which requires a trusted setup ceremony for BLS12-381
- **Curve Consistency**: All components use BLS12-381 to ensure compatibility with Soroban
- **Circuit Auditing**: Circuits should be audited before production use
- **Key Management**: Verification keys must be properly validated
- **Nullifier Uniqueness**: Contract ensures nullifiers cannot be reused
- **Association Set Security**: Association roots must be properly validated and managed

## BLS12-381 Integration

This project is specifically designed to work with Soroban's BLS12-381 implementation:

- **Circuit compilation**: Uses `--prime bls12381` flag in circom
- **Trusted setup**: Powers of tau ceremony for BLS12-381 curve
- **Proof verification**: Native BLS12-381 operations in Soroban contracts
- **Field arithmetic**: All field operations use BLS12-381 scalar field
- **Point serialization**: Consistent coordinate representation between snarkjs and arkworks

## Building and Deploying the Privacy-Pools contract

```bash
# Generate a key for the deployer
soroban keys generate alice --network <NETWORK>

# Fund the deployer address
soroban keys fund alice --network <NETWORK>
```

In the workspace directory run

```bash
# Build the contract
cargo build --target wasm32v1-none --release -p privacy-pools

# Optimize the WASM for Soroban
soroban contract optimize --wasm target/wasm32v1-none/release/privacy_pools.wasm --wasm-out target/wasm32v1-none/release/privacy_pools.optimized.wasm

# Deploy the contract to the testnet passing verification key, token address, and admin address to the constructor
soroban contract deploy --wasm target/wasm32v1-none/release/privacy_pools.optimized.wasm --source alice --network <NETWORK> -- --vk_bytes <VK_BYTES_HEX> --token_address <TOKEN_ADDRESS> --admin <ADMIN_ADDRESS>
```

**Note:** The constructor requires three parameters:
- `vk_bytes`: Hex-encoded verification key (without `0x` prefix)
- `token_address`: Address of the token contract to use for deposits/withdrawals
- `admin`: Address of the contract administrator (typically the deployer)

To deposit into the contract run

```bash
soroban contract invoke --id <CONTRACT_ID> --source alice --network <NETWORK> -- deposit --from alice --commitment <COMMITMENT_HEX>
```

and to withdraw

```bash
soroban contract invoke --id <CONTRACT_ID> --source alice --network <NETWORK> -- withdraw --to alice --proof_bytes <PROOF_BYTES_HEX> --pub_signals_bytes <PUBLIC_OUTPUT_HEX>
```

## Demo: Complete Privacy Pool Workflow

This demo walks through the complete lifecycle of a privacy pool transaction, from coin generation to withdrawal with zero-knowledge proofs.

### Prerequisites

Before running the demo, ensure you have:

1. **Compiled circuits** (see Quick Start section)
2. **Generated trusted setup** for the main circuit
3. **Soroban CLI** configured with a testnet account
4. **Built the contract** and utilities

```bash
# Ensure circuits are compiled
make .circuits

# Build all utilities
cargo build --release

# Set up Soroban testnet account
soroban keys generate demo_user --network testnet
soroban keys fund demo_user --network testnet
```

### Step 1: Deploy the Privacy-Pools Contract

First, we need to deploy the contract with the verification key:

```bash
# Build the contract
cargo build --target wasm32v1-none --release -p privacy-pools

# Optimize the WASM
soroban contract optimize --wasm target/wasm32v1-none/release/privacy_pools.wasm --wasm-out target/wasm32v1-none/release/privacy_pools.optimized.wasm

# Convert verification key to hex format and extract it
cargo run --bin stellar-circom2soroban vk circuits/output/main_verification_key.json > vk_hex.txt
VK_HEX=$(cat vk_hex.txt | grep -o '[0-9a-f]*$')

# Deploy the contract (replace TOKEN_ADDRESS with actual token contract address)
soroban contract deploy --wasm target/wasm32v1-none/release/privacy_pools.optimized.wasm --source demo_user --network testnet -- --vk_bytes $VK_HEX --token_address <TOKEN_ADDRESS> --admin demo_user

# Save the contract ID for later use
export CONTRACT_ID=<CONTRACT_ID_FROM_DEPLOYMENT>

# Verify the admin was set correctly
soroban contract invoke --id $CONTRACT_ID --source demo_user --network testnet -- get_admin
```

### Step 2: Generate a Coin

Create a new coin with a random nullifier and secret:

```bash
# Generate a coin for the demo pool
cargo run --bin stellar-coinutils generate demo_pool demo_coin.json

# View the generated coin
cat demo_coin.json
```

The generated coin contains:
- `value`: The coin's value (e.g., 1000000000 for 1 token)
- `nullifier`: Unique identifier to prevent double-spending
- `secret`: Random secret for commitment generation
- `label`: Additional random data
- `commitment`: Cryptographic commitment hash

### Step 3: Deposit the Coin into the Contract

Use the commitment from the generated coin to deposit:

```bash
# Extract the commitment hex from the coin file (remove 0x prefix)
COMMITMENT_HEX=$(cat demo_coin.json | jq -r '.commitment_hex' | sed 's/^0x//')

# Deposit the coin into the contract
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- deposit --from demo_user --commitment $COMMITMENT_HEX
```

### Step 4: Check the Balance

Verify the deposit was successful by checking the contract state:

```bash
# Check the contract balance
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_balance

# Check the list of commitments
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_commitments
```

### Step 5: Create State File and Association Set

First, create a state file with the commitment from the generated coin, then create an association set with the coin's label:

```bash
# Create state file with the coin's commitment
COMMITMENT=$(cat demo_coin.json | jq -r '.coin.commitment')
echo "{
  \"commitments\": [
    \"$COMMITMENT\"
  ],
  \"scope\": \"demo_pool\"
}" > demo_state.json

# Create association set with the coin's label
LABEL=$(cat demo_coin.json | jq -r '.coin.label')
cargo run --bin stellar-coinutils updateAssociation demo_association.json "$LABEL"

# Create withdrawal inputs from the coin with state file and association set
cargo run --bin stellar-coinutils withdraw demo_coin.json demo_state.json demo_association.json withdrawal_input.json

# Generate witness and proof using the main circuit
cd circuits
node build/main_js/generate_witness.js build/main_js/main.wasm ../withdrawal_input.json witness.wtns
snarkjs groth16 prove ../output/main_final.zkey witness.wtns proof.json public.json

# Convert proof and public signals for Soroban
cd ..
cargo run --bin stellar-circom2soroban proof circuits/proof.json > proof_hex.txt
cargo run --bin stellar-circom2soroban public circuits/public.json > public_hex.txt

# Extract the hex strings (without 0x prefix)
PROOF_HEX=$(sed -n '/^Proof Hex encoding:/{n;p;}' proof_hex.txt | tr -d '[:space:]' | sed -E 's/^0x//i')
PUBLIC_HEX=$(cat public_hex.txt | grep -o '[0-9a-f]*$')

# Extract association root from public signals and set it in the contract
ASSOCIATION_ROOT_HEX=$(echo "$PUBLIC_HEX" | tail -c 65) # Last 64 hex chars (32 bytes) for association root
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- set_association_root --caller demo_user --association_root $ASSOCIATION_ROOT_HEX

echo "Generated proof: $PROOF_HEX"
echo "Public signals: $PUBLIC_HEX"
echo "Association root set: $ASSOCIATION_ROOT_HEX"
```

### Step 6: Withdraw from the Contract

Use the proof to withdraw the coin:

```bash
# Withdraw using the proof
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- withdraw --to demo_user --proof_bytes $PROOF_HEX --pub_signals_bytes $PUBLIC_HEX

echo "Successfully withdrew coin"
```

### Step 7: Verify the Withdrawal

Check that the withdrawal was successful and the nullifier is recorded:

```bash
# Check the list of used nullifiers
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_nullifiers

# Check the updated contract balance
soroban contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_balance

# The nullifier should now appear in the list, indicating it has been spent
```

### Complete Demo Script

`demo.sh` contains a complete script that automates the entire demo.
When running the demo, you should see output similar to:

```bash
🚀 Starting Privacy Pool Demo...
📦 Deploying contract...
Contract deployed: CABC123...
🪙 Generating coin...
Generated coin with commitment: abcd1234...
💰 Depositing coin...
Deposit successful!
📊 Checking balance...
1000000000
📋 Creating state file...
🏷️  Creating association set...
Added label 'xyz789...' to association set. Total labels: 1
🔐 Creating withdrawal proof...
📝 Generating witness and proof...
🔄 Converting proof for Soroban...
🔗 Setting association root in contract...
💸 Withdrawing coin...
Withdrawal successful!
✅ Verifying withdrawal...
[nullifier_list]
0
🎉 Demo completed successfully!
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is open source. Please see the license file for details.
