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
    pub fn verify(key: BytesN<65>, msg: BytesN<32>, sig: BytesN<64>) -> Result<(), Error> {
        let key = key.to_array();
        let msg = msg.to_array();
        let sig = sig.to_array();
        let vk = VerifyingKey::from_sec1_bytes(&key).map_err(|_| Error::PublicKeyParse)?;
        let s = Signature::from_slice(&sig).map_err(|_| Error::SignatureParse)?;
        vk.verify(&msg, &s).map_err(|_| Error::VerifyFailed)
    }
}

mod test;
