# Groth16 Verifier Contract (BN254)

A Groth16 verifier for the BN254 curve implemented using Soroban SDK crypto types.

The computation demonstrates a simple multiplication circuit: `a * b = c`, where:
- `a` and `b` are private inputs
- `c` is the public output

The `./data` directory contains all input files (circuit definition, inputs) and generated outputs. For proof verification, three key files are required:
- [proof.json](data/proof.json) - Contains the zero-knowledge proof
- [verification_key.json](data/verification_key.json) - Contains the verification key
- [public.json](data/public.json) - Contains the public inputs/outputs

Other intermediate artifacts, including witness generation code and outputs from the "Powers of Tau" ceremony, are included in [data/auxiliary](data/auxiliary) for reproducibility.

The contract implementation in [src/lib.rs](src/lib.rs) is translated from the auto-generated Solidity contract produced by snarkjs. The test suite in [src/test.rs](src/test.rs) demonstrates off-chain parsing of the proof and verification key, along with successful contract execution.

## ⚠️ WARNING: Demonstration Use Only

**This project is for demonstration purposes only.**
- It has **not** undergone security auditing
- It is **not** safe for use in production environments

**Use at your own risk.**
