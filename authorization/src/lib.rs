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
    Admin,
}

pub struct AuthContract;

#[contractimpl(export_if = "export")]
impl AuthContract {
    // Sets the admin identifier
    pub fn set_admin(e: Env, admin: Identifier) {
        if e.contract_data().has(DataKey::Admin) {
            panic!("admin is already set")
        }

        e.contract_data().set(DataKey::Admin, admin);
    }

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

    // The admin can write data for any Identifier
    pub fn overwrite(e: Env, auth: KeyedAuthorization, nonce: BigInt, id: Identifier, num: BigInt) {
        let auth_id = auth.get_identifier(&e);
        if auth_id != e.contract_data().get_unchecked(DataKey::Admin).unwrap() {
            panic!("not authorized by admin")
        }

        cryptography::check_auth(
            &e,
            &auth,
            nonce.clone(),
            Symbol::from_str("overwrite"),
            (nonce, id.clone(), num.clone()).into_val(&e),
        );

        e.contract_data().set(DataKey::Acc(id), num);
    }

    pub fn nonce(e: Env, to: Identifier) -> BigInt {
        cryptography::read_nonce(&e, to)
    }
}
