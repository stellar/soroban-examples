//! This is a modular account contract that delegates (part of) its
//! authentication to a set of registered signer addresses.
//!
//! Instead of performing all of the signature verification itself, the
//! `ModularAccount` contract forwards its `__check_auth` context to one or more
//! registered delegate signers. Each delegate then runs its own `__check_auth`
//! independently. The user chooses which of the registered signers to
//! authenticate with by attaching them to the transaction's authorization
//! payload as delegated signers.
//!
//! Auth delegation was introduced in soroban-sdk v27 via CAP-71:
//! <https://github.com/stellar/stellar-protocol/blob/master/core/cap-0071.md>.
//!
//! For a single-key account see the `simple_account` example, and for a
//! multi-sig account with custom authorization policies see the `account`
//! example.
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
pub enum ModularAccountError {
    UnknownDelegate = 1,
}

#[contracttype]
pub enum DataKey {
    // The set of addresses allowed to act as delegates for this account.
    Signers,
    // The authorization contexts this account has approved, used by the tests
    // to verify what was authorized.
    AuthorizedContexts,
}

// A custom account that can delegate authentication to a set of registered
// signer addresses.
#[contract]
pub struct ModularAccount;

#[contractimpl]
impl ModularAccount {
    pub fn __constructor(env: Env, signers: Vec<Address>) {
        env.storage().instance().set(&DataKey::Signers, &signers);
    }
}

#[contractimpl]
impl CustomAccountInterface for ModularAccount {
    // This account holds no key of its own; it authenticates purely by
    // delegating, so it carries no signature.
    type Signature = ();
    type Error = ModularAccountError;

    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signature: (),
        auth_contexts: Vec<Context>,
    ) -> Result<(), ModularAccountError> {
        record_authorized_contexts(&env, &auth_contexts);

        // The signers the user attached to the auth entry for this account's
        // authorization. These are unsanitized user input, so the account must
        // verify each one against its own registered signers below.
        let delegates = env.custom_account().get_delegated_signers();

        let signers: Vec<Address> = env.storage().instance().get(&DataKey::Signers).unwrap();
        for delegate in delegates.iter() {
            // The host can not validate the delegates, so the account has to
            // check that each one is actually a registered signer.
            if !signers.contains(&delegate) {
                return Err(ModularAccountError::UnknownDelegate);
            }
            // Forward the current authentication context to the delegate. Unlike
            // `require_auth`, this does not start a new contract invocation and
            // does not require a separate auth entry for the delegate in the
            // transaction. Delegation is nestable: a delegate may further
            // delegate.
            env.custom_account().delegate_auth(&delegate);
        }
        Ok(())
    }
}

// Stores the full set of authorization contexts this account approved in
// instance storage so the tests can assert on the entire auth context.
fn record_authorized_contexts(env: &Env, auth_contexts: &Vec<Context>) {
    env.storage()
        .instance()
        .set(&DataKey::AuthorizedContexts, auth_contexts);
}

mod test;
