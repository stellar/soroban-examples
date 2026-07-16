#![no_std]

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype,
    crypto::Hash,
    Address, Env, Vec,
};

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UnknownDelegate = 1,
    InsufficientDelegates = 2,
}

#[contracttype]
enum ModularAccountDataKey {
    // Marks an address as a signer allowed to authenticate for the
    // modular account.
    Signer(Address),
}

#[contract]
pub struct ModularAccount;

#[contractimpl]
impl ModularAccount {
    // Registers the addresses allowed to authenticate for this account.
    pub fn __constructor(env: Env, signers: Vec<Address>) {
        for signer in signers.iter() {
            env.storage()
                .persistent()
                .set(&ModularAccountDataKey::Signer(signer), &());
        }
    }
}

#[contractimpl]
impl CustomAccountInterface for ModularAccount {
    // The account verifies no signature of its own, so it carries no
    // signature to check.
    type Signature = ();
    type Error = Error;

    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signatures: (),
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        // The signers the user attached to the auth entry for this
        // account's authorization.
        let delegates = env.custom_account().get_delegated_signers();

        // With no delegates to forward to, the account would authenticate
        // nothing and be effectively unauthenticated, so reject it. A real
        // account might require more than one delegate to meet a threshold.
        if delegates.is_empty() {
            return Err(Error::InsufficientDelegates);
        }

        // Check if the delegates are accepted by the modular account.
        for delegate in delegates.iter() {
            if !env
                .storage()
                .persistent()
                .has(&ModularAccountDataKey::Signer(delegate.clone()))
            {
                return Err(Error::UnknownDelegate);
            }
        }

        // Forward the current authorization to each delegate.
        for delegate in delegates.iter() {
            env.custom_account().delegate_auth(&delegate);
        }

        Ok(())
    }
}

mod test;
