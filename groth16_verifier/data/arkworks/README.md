# Arkworks example

The proof and verification key are generated using [arkworks](https://github.com/arkworks-rs) (a Rust ecosystem for zero-knowledge cryptography) with the BLS12-381 curve.

The computation demonstrates a **Merkle tree membership verification** circuit: given a Merkle tree root and an authentication path, prove that a specific leaf value exists in the tree without revealing the entire tree structure.

In this example:

- A Merkle tree is constructed from 8 leaf values: `[1, 2, 3, 10, 9, 17, 70, 45]`
- The proof demonstrates membership of leaf `9` (at index 4)
- The circuit verifies the authentication path against the public root

The [contract implementation](../../contracts/ark_verifier/src/lib.rs) is **auto-generated** from `verification_key.json` using [soroban-verifier-gen](https://github.com/mysteryon88/soroban-verifier-gen) (Groth16 verifier generator for Soroban). The [test suite](../../contracts/ark_verifier/src/test.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

---

Step-by-step arkworks circuit setup and generation of keys/proof for the `ark_verifier` contract. Run the commands in order.

```sh
cd data/arkworks/auxiliary

# Run the integration test: compiles circuit, generates keys, creates proof, and exports to JSON
# Outputs: ../proof.json and ../verification_key.json
cargo test merkle_tree -- --nocapture
```
