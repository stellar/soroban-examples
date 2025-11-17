# Circuits Test Suite

This directory contains compatiblity tests for Poseidon hash and Merkle-Tree implementations in Rust and Circom.

## Lean-IMT Compatibility Test

This directory contains a compatibility test that verifies the `merkleProof.circom` circuit works correctly with the Lean-IMT (Lean Incremental Merkle Tree) implementation from the Rust codebase.

### Overview

The Lean-IMT compatibility test ensures consistency between:
1. **Rust Implementation**: `lean-imt` crate with LeanIMT implementation
2. **Circom Implementation**: `merkleProof.circom` circuit

This is crucial for maintaining consistency between the Rust implementation (used in the main application) and the Circom implementation (used in zero-knowledge proofs).

### Test Files

#### 1. `test/test_merkleProof.circom`
The main test circuit that includes:
- `TestMerkleProof`: Tests merkle proof verification with configurable depth
- **Input signals**: `leaf`, `leafIndex`, `siblings`, `actualDepth`
- **Output signals**: `out` (computed merkle root)
- **Components**: 
  - `MerkleProof(maxDepth)` for merkle proof verification
  - Uses depth 2 for the main test component

#### 2. `test/lean-imt-test/`
Rust test implementation that:
- Uses the `lean-imt` crate with LeanIMT implementation
- Generates test data and merkle proofs
- Outputs a `circuit_input.json` file with test data

### Running the Lean-IMT Compatibility Test

#### Prerequisites
- Rust toolchain (cargo)
- circom compiler
- snarkjs

#### Step 1: Generate Test Data from Rust Implementation
```bash
cd circuits/test
cargo run --bin lean-imt-test --manifest-path lean-imt-test/Cargo.toml
```

This will generate a `circuit_input.json` file with test data including:
- `leaf`: The leaf value to prove inclusion of
- `leafIndex`: Index of the leaf in the Merkle tree
- `siblings`: Array of sibling values along the path to root
- `actualDepth`: Current tree depth

#### Step 2: Compile the Test Circuit
```bash
cd ../../
circom circuits/test/test_merkleProof.circom -l $CIRCOMLIB --wasm --prime bls12381 -o circuits/build/
```

#### Step 3: Generate Witness from Circom Circuit
```bash
cd circuits/build/test_merkleProof_js
node generate_witness.js test_merkleProof.wasm ../../test/circuit_input.json ../../test/test_merkleProof.wtns
```

#### Step 4: Extract Witness Outputs
```bash
cd ../../test
snarkjs wtns export json test_merkleProof.wtns
```

#### Step 5: Verify Compatibility
Compare the computed root from the circuit with the expected root from the Rust implementation. The outputs should match, confirming that both implementations produce identical merkle proof verification results.

### Test Circuit Structure

The `test_merkleProof.circom` circuit:
- **Input signals**: `leaf`, `leafIndex`, `siblings`, `actualDepth`
- **Output signals**: `out` (computed merkle root)
- **Public signals**: All inputs and outputs are public for testing purposes
- **Merkle proof verification**: Uses the same hashing and verification logic as the Rust implementation

### Key Benefits

1. **Cross-Implementation Verification**: Ensures Rust and Circom implementations produce identical merkle proof verification results
2. **Cryptographic Consistency**: Validates that both implementations use the same hashing algorithms and tree construction
3. **Integration Testing**: Provides confidence that the two codebases can work together seamlessly
4. **Regression Prevention**: Catches any divergence between implementations during development

### Technical Details

- **Hash Function**: Both implementations use Poseidon255 for hashing
- **Tree Structure**: Both follow the LeanIMT design principles
- **Proof Format**: Both handle the same merkle proof structure and validation
- **Output Format**: Both produce field elements in the same representation

### Troubleshooting Lean-IMT Tests

#### Common Issues
1. **Compilation Errors**: Ensure circom and circomlib are properly installed
2. **Witness Generation Errors**: Check that the generated `circuit_input.json` matches the expected format
3. **Mismatched Outputs**: Verify that both implementations use the same hashing algorithms

#### Debugging
- Use the generated witness files to verify outputs manually
- Check that both implementations use the same field arithmetic
- Verify that the merkle proof structure is consistent between implementations

## Poseidon Compatibility Test

This directory also contains a compatibility test for the `poseidon255.circom` circuit, which verifies that the Rust `poseidon` crate produces identical hash outputs to the Circom implementation.

### Overview

The Poseidon compatibility test ensures consistency between:
1. **Rust Implementation**: `poseidon` crate with `Poseidon255` implementation
2. **Circom Implementation**: `poseidon255.circom` circuit

This is crucial for maintaining consistency between the Rust implementation (used in the main application) and the Circom implementation (used in zero-knowledge proofs).

### Test Files

#### 1. `test/test_poseidon.circom`
The main test circuit that includes:
- `TestPoseidon`: Tests both single and two-input hashing
- **Input signals**: `in1`, `in2`
- **Output signals**: `out1` (single input hash), `out2` (two input hash)
- **Components**: 
  - `Poseidon255(1)` for single input hashing
  - `Poseidon255(2)` for two input hashing

#### 2. `test/poseidon-test/`
Rust test implementation that:
- Uses the `poseidon` crate with `Poseidon255` implementation
- Takes JSON input with two values: `in1` and `in2`
- Computes the same hashes as the circuit

#### 3. `test/test_poseidon_input.json`
Test input data:
```json
{
    "in1": "123456789",
    "in2": "0"
}
```

### Running the Poseidon Compatibility Test

#### Prerequisites
- Rust toolchain (cargo)
- Node.js and npm
- circom compiler
- snarkjs

#### Step 1: Generate Witness from Rust Implementation
```bash
cd circuits/test
cat test_poseidon_input.json | cargo run --bin test_poseidon --manifest-path poseidon-test/Cargo.toml
```

**Expected Output:**
```
49771379518533783451081444171936304251693849153677701053778138403868110038125
2595333311380081774082696984545715941782212075692277571540746075566179600420
```

#### Step 2: Generate Witness from Circom Circuit
```bash
cd circuits/test
circom test_poseidon.circom -l $CIRCOMLIB --prime bls12381 --wasm -o ../build
node ../build/test_poseidon_js/generate_witness.js ../build/test_poseidon_js/test_poseidon.wasm test_poseidon_input.json test_poseidon_new.wtns
```

#### Step 3: Extract Witness Outputs
```bash
cd ../../test
snarkjs wtns export json test_poseidon_new.wtns
```

#### Step 4: Verify Compatibility
Compare the outputs from both implementations:

| Hash Type | Rust Output | Circom Output | Status |
|-----------|-------------|---------------|---------|
| Single Input (`in1`) | `49771379518533783451081444171936304251693849153677701053778138403868110038125` | `49771379518533783451081444171936304251693849153677701053778138403868110038125` | ✅ **MATCH** |
| Two Inputs (`in1`, `in2`) | `2595333311380081774082696984545715941782212075692277571540746075566179600420` | `2595333311380081774082696984545715941782212075692277571540746075566179600420` | ✅ **MATCH** |

### Test Circuit Structure

The `test_poseidon.circom` circuit:
- **Input signals**: `in1`, `in2`
- **Output signals**: `out1`, `out2`
- **Public signals**: All inputs and outputs are public for testing purposes
- **Hash functions**: Uses the same Poseidon255 parameters as the Rust implementation

### Key Benefits

1. **Cross-Implementation Verification**: Ensures Rust and Circom implementations produce identical results
2. **Cryptographic Consistency**: Validates that both implementations use the same parameters and algorithms
3. **Integration Testing**: Provides confidence that the two codebases can work together seamlessly
4. **Regression Prevention**: Catches any divergence between implementations during development

### Technical Details

- **Field**: Both implementations use the BLS12-381 scalar field
- **Hash Function**: Poseidon255 with identical constants and parameters
- **Input Processing**: Both handle the same input format and validation
- **Output Format**: Both produce field elements in the same representation

### Troubleshooting Poseidon Tests

#### Common Issues
1. **Compilation Errors**: Ensure circom and circomlib are properly installed
2. **Witness Generation Errors**: Check that input JSON matches the expected format
3. **Mismatched Outputs**: Verify that both implementations use the same Poseidon parameters

#### Debugging
- Use the generated witness files to verify outputs manually
- Check that both implementations use the same field arithmetic
- Verify Poseidon constants are identical between implementations
