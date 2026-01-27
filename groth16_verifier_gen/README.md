# Soroban Groth16 Verifier

Soroban smart contract verifier for **Groth16** zero-knowledge proofs.  
It supports the **snarkjs-compatible JSON** format (e.g. `verification_key.json`) produced by:

- [Circom](https://docs.circom.io/)
- [Arkworks](https://github.com/arkworks-rs) via [ark-snarkjs](https://github.com/mysteryon88/ark-snarkjs)
- [Gnark](https://github.com/Consensys/gnark) via [gnark-to-snarkjs](https://github.com/mysteryon88/gnark-to-snarkjs)
- [Noname](https://github.com/zksecurity/noname) - [Article about integration with SnarkJS](https://blog.zksecurity.xyz/posts/noname-r1cs/)

Provide `verification_key.json` (and optionally `proof.json` / `public.json`), then generate and deploy the Soroban verifier contract.

The proof and verification key used in tests are taken from the [Groth16 verifier example](https://github.com/stellar/soroban-examples/tree/main/groth16_verifier).

For more examples of different ZK schemes and usage patterns on Soroban, see **[zk-soroban-examples](https://github.com/zk-examples/zk-soroban-examples)**.

---

## Usage

```sh
cd groth16_verifier_gen

cargo install soroban-verifier-gen

soroban-verifier-gen --vk data/circom/verification_key.json --out contracts/verifier
soroban-verifier-gen --vk data/gnark/verification_key.json --out contracts/gnark_verifier --crate-name gnark_verifier
```

---

## ⚠️ WARNING: Demonstration Use Only

**This project is for demonstration purposes only.**

- It has **not** undergone security auditing  
- It is **not** safe for use in production environments  

**Use at your own risk.**
