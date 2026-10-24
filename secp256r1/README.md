# Secp256r1 (NIST P-256) Signature Verification Example

This example demonstrates how to use the built-in `env.crypto().secp256r1_verify` host function in Soroban smart contracts.

`secp256r1` (also known as NIST P-256 or prime256v1) is the standard elliptic curve used in modern client authentication systems:
- **WebAuthn / Passkeys** (Apple Touch ID / Face ID, Windows Hello, Android Biometrics)
- **FIDO2 Security Keys** (YubiKey)
- **Apple Secure Enclave** and hardware TPMs

This contract illustrates two practical patterns:
1. **Message Verification**: Verifying a signature over an arbitrary message (hashed with SHA-256).
2. **Custom Account Authentication**: Implementing the `__check_auth` entry point to create a smart account controlled by a secp256r1 public key (e.g. a Passkey-backed account).

## Cryptographic Parameters

- **Public Key**: 65 bytes uncompressed SEC1 format (`0x04 || X (32 bytes) || Y (32 bytes)`).
- **Signature**: 64 bytes IEEE P1363 format (`r (32 bytes) || s (32 bytes)`).
  - *Note*: Soroban enforces low-`s` normalization ($s \le n/2$) to prevent signature malleability.
- **Message Digest**: 32 bytes SHA-256 digest (`Hash<32>`).

## Contract Functions

- `__constructor(env: Env, owner_public_key: BytesN<65>)`: Initializes the contract storage with the account owner's 65-byte uncompressed public key.
- `verify(env: Env, public_key: BytesN<65>, message: Bytes, signature: BytesN<64>)`: Hashes `message` with SHA-256 and verifies the signature against the provided public key.
- `verify_with_owner(env: Env, message: Bytes, signature: BytesN<64>) -> Result<(), Error>`: Hashes `message` with SHA-256 and verifies the signature against the stored owner public key.
- `get_owner(env: Env) -> Result<BytesN<65>, Error>`: Returns the stored 65-byte owner public key.
- `__check_auth(env: Env, signature_payload: Hash<32>, signature: BytesN<64>, _auth_context: Vec<Context>)`: Custom Account authorization entry point called automatically by the Soroban host during `require_auth` checks.

## Building and Testing

```bash
# Run unit tests
cargo test

# Build WASM binary
cargo build --target wasm32v1-none --release
```
