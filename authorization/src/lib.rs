#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod cryptography;
mod public_types;
mod test;

use public_types::{Identifier, KeyedAuthorization};
use soroban_sdk::{contractimpl, contracttype, BigInt, Env, IntoVal, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Acc(Identifier),
    Nonce(Identifier),
}

pub struct AuthContract;

#[contractimpl(export_if = "export")]
impl AuthContract {
    // Saves data that corresponds to an Identifier, with that Identifiers authorization
    pub fn save_data(e: Env, auth: KeyedAuthorization, nonce: BigInt, num: BigInt) {
        cryptography::check_auth(
            &e,
            &auth,
            nonce.clone(),
            Symbol::from_str("save_data"),
            (nonce, num.clone()).into_val(&e),
        );

        let auth_id = auth.get_identifier(&e);
        e.contract_data().set(DataKey::Acc(auth_id), num);
    }

    pub fn nonce(e: Env, to: Identifier) -> BigInt {
        cryptography::read_nonce(&e, to)
    }
}
