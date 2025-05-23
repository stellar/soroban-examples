//! This a basic 1-of-n multisig account contract where any of the
//! configured signatures may sign.
#![no_std]

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype,
    crypto::Hash,
    BytesN, Env, Vec,
};
#[contract]
struct Contract;

#[contracttype]
#[derive(Clone)]
pub struct Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Signer(BytesN<32>),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UnknownSigner = 1,
}

#[contractimpl]
impl Contract {
    // Initialize the contract with a list of ed25519 public key ('signers').
    pub fn __constructor(env: Env, signers: Vec<BytesN<32>>) {
        for signer in signers.iter() {
            env.storage().instance().set(&DataKey::Signer(signer), &());
        }
    }
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Signature = Signature;
    type Error = Error;

    // This is the 'entry point' of the account contract and every account
    // contract has to implement it. `require_auth` calls for the Address of
    // this contract will result in calling this `__check_auth` function with
    // the appropriate arguments.
    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: Self::Signature,
        _auth_context: Vec<Context>,
    ) -> Result<(), Error> {
        if !env
            .storage()
            .instance()
            .has(&DataKey::Signer(signature.public_key.clone()))
        {
            return Err(Error::UnknownSigner);
        }
        env.crypto().ed25519_verify(
            &signature.public_key,
            &signature_payload.clone().into(),
            &signature.signature,
        );
        Ok(())
    }
}

mod test;
