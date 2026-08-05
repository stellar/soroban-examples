# Arkworks example

The proof and verification key are generated using [arkworks](https://github.com/arkworks-rs) (a Rust ecosystem for zero-knowledge cryptography) with the BLS12-381 curve.

The computation demonstrates a Merkle tree membership verification circuit: given a Merkle tree root and an authentication path, prove that a specific leaf value exists in the tree without revealing the entire tree structure.

In this example:

- A Merkle tree is constructed from 8 leaf values: `[1, 2, 3, 10, 9, 17, 70, 45]`
- The proof demonstrates membership of leaf `9` (at index 4)
- The circuit verifies the authentication path against the public root

The [contract implementation](../../../src/lib.rs) stores the verification key supplied during initialization or via `set_verification_key`. The [integration test](../../arkworks.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

---

Step-by-step arkworks circuit setup and generation of keys/proof for the `bls12_381_verifier` fixture. Run the commands in order.

```sh
cd contracts/bls12_381_verifier/tests/data/arkworks/auxiliary

# Run the integration test: compiles circuit, generates keys, creates proof, and exports to JSON
# Outputs: ../proof.json and ../verification_key.json
cargo test merkle_tree -- --nocapture
```
