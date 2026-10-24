//! This is an example contract demonstrating secp256r1 (NIST P-256) signature verification.
//!
//! Secp256r1 is widely used in WebAuthn, Passkeys, Apple Secure Enclave, and hardware TPMs.
//! This contract demonstrates both message verification and custom account authentication.
#![no_std]

use soroban_sdk::{
    auth::Context, contract, contracterror, contractimpl, contracttype, crypto::Hash, Bytes,
    BytesN, Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OwnerAlreadySet = 1,
    OwnerNotSet = 2,
}

#[contract]
pub struct Secp256r1Contract;

#[contractimpl]
impl Secp256r1Contract {
    /// Initializes the contract with an account owner's 65-byte uncompressed secp256r1 public key (0x04 || X || Y).
    pub fn __constructor(env: Env, owner_public_key: BytesN<65>) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("owner is already set");
        }
        env.storage()
            .instance()
            .set(&DataKey::Owner, &owner_public_key);
    }

    /// Verifies a secp256r1 signature over a message using an arbitrary public key.
    ///
    /// The message is hashed using SHA-256 before verification.
    /// - `public_key`: 65 bytes uncompressed SEC1 format (`0x04 || X || Y`)
    /// - `message`: arbitrary bytes payload
    /// - `signature`: 64 bytes IEEE P1363 raw signature (`r || s`)
    pub fn verify(env: Env, public_key: BytesN<65>, message: Bytes, signature: BytesN<64>) {
        let digest = env.crypto().sha256(&message);
        env.crypto()
            .secp256r1_verify(&public_key, &digest, &signature);
    }

    /// Verifies a secp256r1 signature over a message using the configured owner's public key.
    pub fn verify_with_owner(env: Env, message: Bytes, signature: BytesN<64>) -> Result<(), Error> {
        let public_key: BytesN<65> = env
            .storage()
            .instance()
            .get::<_, BytesN<65>>(&DataKey::Owner)
            .ok_or(Error::OwnerNotSet)?;

        let digest = env.crypto().sha256(&message);
        env.crypto()
            .secp256r1_verify(&public_key, &digest, &signature);
        Ok(())
    }

    /// Retrieves the current owner's 65-byte public key.
    pub fn get_owner(env: Env) -> Result<BytesN<65>, Error> {
        env.storage()
            .instance()
            .get::<_, BytesN<65>>(&DataKey::Owner)
            .ok_or(Error::OwnerNotSet)
    }

    /// Custom Account authentication entry point.
    ///
    /// When `require_auth` is called on this contract's address, the Soroban host
    /// automatically calls `__check_auth` with the signature payload and credentials.
    #[allow(non_snake_case)]
    pub fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        _auth_context: Vec<Context>,
    ) {
        let public_key: BytesN<65> = env
            .storage()
            .instance()
            .get::<_, BytesN<65>>(&DataKey::Owner)
            .unwrap();

        env.crypto()
            .secp256r1_verify(&public_key, &signature_payload, &signature);
    }
}

#[cfg(test)]
mod test;
