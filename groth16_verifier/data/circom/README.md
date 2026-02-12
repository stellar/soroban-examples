# Circom example

The proof and verification key are generated following the Circom (circom compiler 2.2.2) [getting-started guide](https://docs.circom.io/getting-started/installation/).

The computation demonstrates a simple multiplication circuit: `a * b = c`, where:

- `a` and `b` are private inputs
- `c` is the public output

The [contract implementation](../../contracts/circom_verifier/src/lib.rs) is **auto-generated** from `verification_key.json` using [soroban-verifier-gen](https://github.com/mysteryon88/soroban-verifier-gen) (Groth16 verifier generator for Soroban). The [test suite](../../contracts/circom_verifier/src/test.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

---

Step-by-step Circom circuit setup and generation of keys/proof for the `circom_verifier` contract. Run the commands in order.

```sh
cd ./data/circom/auxiliary/

# Compile the circuit: R1CS, WASM build, and debug symbols; BLS12-381 curve (same as Soroban)
circom multiplier2.circom --r1cs --wasm --prime bls12381

# --- Trusted setup (Phase 1: Powers of Tau) ---
snarkjs powersoftau new bls12-381 5 pot5_0000.ptau -v
snarkjs powersoftau contribute pot5_0000.ptau pot5_0001.ptau --name="First contribution" -v -e="some random text"
snarkjs powersoftau prepare phase2 pot5_0001.ptau pot5_final.ptau -v

# Phase 2: circuit + ptau → initial zkey, then contribution
snarkjs groth16 setup Multiplier2.r1cs pot5_final.ptau Multiplier2_0000.zkey
snarkjs zkey contribute Multiplier2_0000.zkey Multiplier2_final.zkey --name="1st Contributor Name" -v -e="some random text"

# Export verification key (used by the contract)
snarkjs zkey export verificationkey Multiplier2_final.zkey ../verification_key.json

# Generate proof: inputs → proof + public signals (output in ../proof.json and ../public.json)
snarkjs groth16 fullprove input.json Multiplier2_js/Multiplier2.wasm Multiplier2_final.zkey ../proof.json ../public.json

cd ../../..
```

After running, `data/circom/` will contain `verification_key.json`, `proof.json`, and `public.json` — use them to update the contract and tests.
