# Gnark example

The proof and verification key are generated using [gnark](https://github.com/consensys/gnark) (a zk-SNARK library written in Go) with the BN254 curve.

The computation demonstrates a polynomial circuit: `x³ + x + 5 = y`, where:

- `x` is a private input (secret witness)
- `y` is the public output

In this example, `x = 3` and `y = 35` (since 3³ + 3 + 5 = 27 + 3 + 5 = 35).

The [contract implementation](../../contracts/gnark_bn254_verifier/src/lib.rs) is **auto-generated** from `verification_key.json` using [soroban-verifier-gen](https://github.com/mysteryon88/soroban-verifier-gen) (Groth16 verifier generator for Soroban). The [test suite](../../contracts/gnark_bn254_verifier/src/test.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

Exports proof and verification key to snarkjs-compatible JSON format using [gnark-to-snarkjs](https://github.com/mysteryon88/gnark-to-snarkjs)

---

Step-by-step gnark circuit setup and generation of keys/proof for the `gnark_bn254_verifier` contract. Run the commands in order.

```sh
cd data/gnark_bn254/auxiliary

# Install dependencies
go mod tidy

# Run the setup: compiles circuit, generates keys, creates proof, and exports to JSON
# Outputs: ../proof.json and ../verification_key.json
go run .

cd ../../..
```
