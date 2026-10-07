# x402 Hardware-Attested Execution Verifier (`x402_tee_verifier`)

This example demonstrates how to build an **x402 Execution Evidence Verifier** smart contract on Stellar using Soroban SDK v28.

## Overview
When autonomous AI agents pay for compute execution performed inside confidential hardware enclaves (Intel TDX, AMD SEV-SNP, or Apple Secure Enclave), the enclave emits a cryptographically signed execution receipt (`x402ev/1:sha256:...`).

This contract:
1. Verifies the enclave's Ed25519 signature directly on-chain using Soroban's native host crypto primitive `env.crypto().ed25519_verify()`.
2. Enforces replay protection by recording the canonical receipt fingerprint into persistent storage.
3. Transfers settlement tokens (Circle USDC or native XLM) from the agent payer to the compute provider vault upon verification.
4. Emits a verifiable event (`x402_pay`) containing the `evidence_ref`.

## Building
```bash
cargo build --target wasm32-unknown-unknown --release
```
