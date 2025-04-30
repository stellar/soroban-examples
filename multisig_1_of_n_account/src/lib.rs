//! This a basic multi-sig account contract where any signature may sign.
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
    //
    // This should return `()` if authentication and authorization checks have
    // been passed and return an error (or panic) otherwise.
    //
    // `__check_auth` takes the payload that needed to be signed, arbitrarily
    // typed signatures (`Vec<Signature>` contract type here) and authorization
    // context that contains all the invocations that this call tries to verify.
    // The authorization contexts are unused in this case.
    //
    // `__check_auth` has to authenticate the signatures. It also may use
    // `auth_context` to implement additional authorization policies (like token
    // spend limits here).
    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: Self::Signature,
        _auth_context: Vec<Context>,
    ) -> Result<(), Error> {
        env.crypto().ed25519_verify(
            &signature.public_key,
            &signature_payload.clone().into(),
            &signature.signature,
        );
        if !env
            .storage()
            .instance()
            .has(&DataKey::Signer(signature.public_key.clone()))
        {
            return Err(Error::UnknownSigner);
        }
        Ok(())
    }
}

mod test;
