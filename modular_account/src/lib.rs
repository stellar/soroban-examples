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
    Address, BytesN, Env, Symbol, Vec,
};

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ModularAccountError {
    UnknownDelegate = 1,
}

#[contracttype]
pub enum DataKey {
    // The account's own ed25519 public key.
    PublicKey,
    // The set of addresses allowed to act as delegates for this account.
    Signers,
    // A log of the function names this account has approved, used by the tests
    // to verify what was authorized.
    AuthorizedCalls,
}

// A custom account that can delegate authentication to a set of registered
// signer addresses.
#[contract]
pub struct ModularAccount;

#[contractimpl]
impl ModularAccount {
    pub fn __constructor(env: Env, public_key: BytesN<32>, signers: Vec<Address>) {
        env.storage()
            .instance()
            .set(&DataKey::PublicKey, &public_key);
        env.storage().instance().set(&DataKey::Signers, &signers);
    }
}

#[contractimpl]
impl CustomAccountInterface for ModularAccount {
    type Signature = BytesN<64>;
    type Error = ModularAccountError;

    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), ModularAccountError> {
        // Even though we use delegated authentication, the account can still
        // perform the regular verification if necessary.
        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::PublicKey).unwrap();
        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
        record_authorized_calls(&env, &auth_contexts);

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

// Appends the function name from each contract-call context to a per-account
// log in instance storage so the tests can verify what the account approved.
fn record_authorized_calls(env: &Env, auth_contexts: &Vec<Context>) {
    let mut calls: Vec<Symbol> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorizedCalls)
        .unwrap_or_else(|| Vec::new(env));
    for ctx in auth_contexts.iter() {
        if let Context::Contract(c) = ctx {
            calls.push_back(c.fn_name);
        }
    }
    env.storage()
        .instance()
        .set(&DataKey::AuthorizedCalls, &calls);
}

mod test;
