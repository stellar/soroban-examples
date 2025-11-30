# Lean Incremental Merkle Tree (LeanIMT)

A Rust implementation of a Lean Incremental Merkle Tree designed for use with Soroban smart contracts and compatible with the `merkleProof.circom` circuit.

## Overview

LeanIMT is a specialized merkle tree implementation that follows specific design principles for efficient incremental updates and compatibility with zero-knowledge proof circuits. It's designed to work seamlessly with the privacy pools contract and the circom circuit implementation.

## Design Principles

LeanIMT follows these key design principles:

1. **Every node with two children is the hash of its left and right nodes**
2. **Every node with one child has the same value as its child node**
3. **Tree is always built from leaves to root**
4. **Tree is always balanced by construction**
5. **Tree depth is dynamic and can increase with insertion of new leaves**

## Features

- **Incremental Updates**: Efficiently add new leaves without rebuilding the entire tree
- **Dynamic Depth**: Tree depth automatically adjusts based on the number of leaves
- **Proof Generation**: Generate merkle inclusion proofs for any leaf
- **Soroban Compatible**: Designed specifically for Stellar's Soroban smart contract platform
- **Circuit Compatible**: Matches the behavior expected by `merkleProof.circom`

## Usage

### Basic Usage

```rust
use lean_imt::LeanIMT;
use soroban_sdk::{Env, BytesN};

// Create a new tree
let env = Env::default();
let mut tree = LeanIMT::new(env.clone());

// Insert leaves
let leaf1 = BytesN::from_array(&env, &[1u8; 32]);
let leaf2 = BytesN::from_array(&env, &[2u8; 32]);

tree.insert(leaf1);
tree.insert(leaf2);

// Get tree information
let root = tree.get_root();
let depth = tree.get_depth();
let leaf_count = tree.get_leaf_count();

// Generate a proof
let proof = tree.generate_proof(0);
```

### In a Soroban Contract

```rust
use lean_imt::{LeanIMT, TREE_ROOT_KEY, TREE_DEPTH_KEY, TREE_LEAVES_KEY};

// Store tree state
let (leaves, depth, root) = tree.to_storage();
env.storage().instance().set(&TREE_LEAVES_KEY, &leaves);
env.storage().instance().set(&TREE_DEPTH_KEY, &depth);
env.storage().instance().set(&TREE_ROOT_KEY, &root);

// Restore tree state
let leaves: Vec<BytesN<32>> = env.storage().instance().get(&TREE_LEAVES_KEY)
    .unwrap_or(vec![&env]);
let depth: u32 = env.storage().instance().get(&TREE_DEPTH_KEY)
    .unwrap_or(0);
let root: BytesN<32> = env.storage().instance().get(&TREE_ROOT_KEY)
    .unwrap_or(BytesN::from_array(&env, &[0u8; 32]));

let tree = LeanIMT::from_storage(env.clone(), leaves, depth, root);
```

## API Reference

### Core Methods

- `new(env: Env) -> Self`: Create a new empty tree
- `insert(leaf: BytesN<32>)`: Insert a new leaf
- `get_root() -> BytesN<32>`: Get the current merkle root
- `get_depth() -> u32`: Get the current tree depth
- `get_leaf_count() -> u32`: Get the number of leaves
- `generate_proof(leaf_index: u32) -> Option<(Vec<BlsScalar>, u32)>`: Generate inclusion proof

### Storage Methods

- `to_storage() -> (Vec<BytesN<32>>, u32, BytesN<32>)`: Serialize tree for storage
- `from_storage(env: Env, leaves: Vec<BytesN<32>>, depth: u32, root: BytesN<32>) -> Self`: Deserialize from storage

### Utility Methods

- `get_leaves() -> &Vec<BytesN<32>>`: Get reference to all leaves
- `is_empty() -> bool`: Check if tree is empty
- `get_leaf(index: usize) -> Option<&BytesN<32>>`: Get leaf at specific index

## Performance Optimizations

LeanIMT implements several key optimizations to achieve efficient incremental updates and minimal storage overhead:

### Dynamic Programming for Empty Trees

When building an empty tree (all leaves are zero), LeanIMT uses a dynamic programming optimization that reduces complexity from **O(2^depth)** to **O(depth)**:

**Traditional approach**: O(2^depth) - computes every node individually
- For depth 20: 2^20 = 1,048,576 hash operations

**Optimized approach**: O(depth) - leverages identical subtrees  
- For depth 20: only 20 hash operations

**How it works:**
1. **Identical Subtrees**: In an empty tree, all subtrees at the same level are identical (all zeros)
2. **Level-by-Level Computation**: Instead of computing each node individually, we compute one hash per level
3. **Hash Reuse**: `hash(level_n, level_n) = level_n+1` - each level's hash becomes the input for the next level
4. **Caching**: The computed hash for each level is cached in `subtree_cache`

**Example for depth 3:**
```
Level 0: hash(0, 0) = H0
Level 1: hash(H0, H0) = H1  
Level 2: hash(H1, H1) = H2
Level 3: hash(H2, H2) = H3 (root)
```

### Sparse Storage Structure

LeanIMT uses a hybrid caching system to minimize storage while maintaining fast access:

#### 1. Subtree Cache (`subtree_cache`)
- **Purpose**: Stores hashes for entire levels of empty subtrees
- **Key**: `level` → **Value**: `hash` for that level
- **Use Case**: Empty tree construction and level-based optimizations
- **Storage**: Minimal - only one entry per level

#### 2. Sparse Cache (`sparse_cache`) 
- **Purpose**: Stores only the nodes that have been updated due to leaf insertions
- **Key**: `(level, node_index)` → **Value**: `computed_hash` for that specific node
- **Use Case**: Incremental updates after leaf insertions
- **Storage**: Sparse - only stores nodes on the path from new leaf to root

#### Cache Lookup Strategy
The system uses a two-tier lookup approach:
1. **Primary Check**: Look in sparse cache for specific node updates (most recent changes)
2. **Fallback**: If not found, check subtree cache for level-based hashes (empty tree optimization)

### Incremental Update Optimization

When inserting a new leaf, LeanIMT implements "Clever Shortcut 2" from Tornado Cash:

1. **Path-Only Recomputation**: Only recomputes the path from the new leaf to the root
2. **Sibling Reuse**: Siblings to the left of the insertion path can be cached and reused
3. **Cache Updates**: Updates the sparse cache as it recomputes the path
4. **Minimal Work**: Avoids recomputing the entire tree structure

**Example insertion path:**
```
New leaf at index 5 (binary: 101)
Path: 5 → 2 → 1 → 0 (root)
Only these 4 nodes need recomputation
All other nodes remain cached and unchanged
```

## Hash Function

LeanIMT uses **Poseidon2** as its hash function, which provides:

- **Consistency**: Same hash function used in the contract and circuit
- **Security**: Cryptographically secure hash function
- **Efficiency**: Fast computation suitable for smart contracts
- **Standardization**: Widely adopted hash function in blockchain systems

The hash function is used to combine pairs of nodes when building the tree structure, ensuring the integrity and uniqueness of each merkle root.

## Compatibility with merkleProof.circom

The LeanIMT implementation is designed to be fully compatible with the `merkleProof.circom` circuit:

- **Proof Format**: The `generate_proof` method returns siblings and depth in the exact format expected by the circuit
- **Tree Structure**: The tree construction follows the same logic as the circuit
- **Hash Consistency**: Both use Poseidon for hashing, ensuring identical behavior

## Testing

Run the test suite:

```bash
cd lean-imt
cargo test
```

The tests cover:
- Tree creation and initialization
- Leaf insertion and tree growth
- Proof generation
- Storage serialization/deserialization
- Edge cases and error conditions

## Integration

This crate is designed to integrate with:

- **Privacy Pools Contract**: Stores commitments in the merkle tree
- **Zero-Knowledge Proofs**: Provides merkle proofs for circuit verification
- **Soroban Platform**: Native integration with Stellar's smart contract platform

## License

This project is part of the Soroban Privacy Pools implementation and follows the same licensing terms.
