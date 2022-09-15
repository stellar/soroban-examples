#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;

use soroban_auth::{
    check_auth, {Identifier, Signature},
};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, Env, IntoVal};

#[contracttype]
pub enum DataKey {
    SavedNum(Identifier),
    Nonce(Identifier),
    Admin,
}

fn read_nonce(e: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    e.contract_data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

fn verify_and_consume_nonce(e: &Env, id: &Identifier, expected_nonce: &BigInt) {
    match id {
        Identifier::Contract(_) => {
            if BigInt::zero(&e) != expected_nonce {
                panic!("nonce should be zero for Contract")
            }
            return;
        }
        _ => {}
    }

    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, id);

    if nonce != expected_nonce {
        panic!("incorrect nonce")
    }
    e.contract_data().set(key, &nonce + 1);
}

pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Set the admin identifier. May be called only once.
    pub fn set_admin(e: Env, admin: Identifier) {
        if e.contract_data().has(DataKey::Admin) {
            panic!("admin is already set")
        }

        e.contract_data().set(DataKey::Admin, admin);
    }

    /// Save the number for an authenticated [Identifier].
    pub fn save_num(e: Env, sig: Signature, nonce: BigInt, num: BigInt) {
        let auth_id = sig.get_identifier(&e);

        verify_and_consume_nonce(&e, &auth_id, &nonce);

        check_auth(
            &e,
            &sig,
            symbol!("save_num"),
            (&auth_id, nonce, &num).into_val(&e),
        );

        e.contract_data().set(DataKey::SavedNum(auth_id), num);
    }

    // The admin can write data for any Identifier
    pub fn overwrite(e: Env, sig: Signature, nonce: BigInt, id: Identifier, num: BigInt) {
        let auth_id = sig.get_identifier(&e);
        if auth_id != e.contract_data().get_unchecked(DataKey::Admin).unwrap() {
            panic!("not authorized by admin")
        }

        verify_and_consume_nonce(&e, &auth_id, &nonce);

        check_auth(
            &e,
            &sig,
            symbol!("overwrite"),
            (auth_id, nonce, &id, &num).into_val(&e),
        );

        e.contract_data().set(DataKey::SavedNum(id), num);
    }

    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }
}
