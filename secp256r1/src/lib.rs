#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, BytesN};

use p256::ecdsa::{signature::Verifier as _, Signature, VerifyingKey};

#[contract]
pub struct Contract;

#[contracterror]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    PublicKeyParse = 1,
    SignatureParse = 2,
    VerifyFailed = 3,
}

#[contractimpl]
impl Contract {
    /// Verifies an ecdsa secp256r1 signature.
    ///
    /// The signature is verified as a valid signature of the message by the
    /// ecdsa public key.
    ///
    /// The public key msut be sec1 encoded.
    ///
    /// While the ecdsa signature verification process does not prescribe a size
    /// limit for the message, this contract requires the message to be 32-bytes
    /// as a memory allocation optimization.
    ///
    /// The key should be uncompressed SEC1 encoded.
    ///
    /// ### Errors
    ///
    /// If the signature verification fails.
    ///
    /// If the public key cannot be parsed as a sec1 encoded key.
    pub fn secp256r1_verify(
        key: BytesN<65>,
        msg: BytesN<32>,
        sig: BytesN<64>,
    ) -> Result<(), Error> {
        let key = key.to_array();
        let msg = msg.to_array();
        let sig = sig.to_array();
        let vk = VerifyingKey::from_sec1_bytes(&key).map_err(|_| Error::PublicKeyParse)?;
        let s = Signature::from_slice(&sig).map_err(|_| Error::SignatureParse)?;
        vk.verify(&msg, &s).map_err(|_| Error::VerifyFailed)
    }
}

mod test;
