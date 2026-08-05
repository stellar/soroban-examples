# Gnark example

The proof and verification key are generated using [gnark](https://github.com/consensys/gnark) (a zk-SNARK library written in Go) with the BLS12-381 curve.

The computation demonstrates a polynomial circuit: `x^3 + x + 5 = y`, where:

- `x` is a private input (secret witness)
- `y` is the public output

In this example, `x = 3` and `y = 35` (since `3^3 + 3 + 5 = 27 + 3 + 5 = 35`).

The [contract implementation](../../../src/lib.rs) stores the verification key supplied during initialization or via `set_verification_key`. The [integration test](../../gnark.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

Exports proof and verification key to snarkjs-compatible JSON format using [gnark-to-snarkjs](https://github.com/mysteryon88/gnark-to-snarkjs).

---

Step-by-step gnark circuit setup and generation of keys/proof for the `bls12_381_verifier` fixture. Run the commands in order.

```sh
cd contracts/bls12_381_verifier/tests/data/gnark/auxiliary

# Install dependencies
go mod tidy

# Run the setup: compiles circuit, generates keys, creates proof, and exports to JSON
# Outputs: ../proof.json and ../verification_key.json
go run .

cd ../../../../../..
```
