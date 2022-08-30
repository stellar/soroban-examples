#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;

use soroban_sdk::{contractimpl, contracttype, BigInt, Env, IntoVal, Symbol};
use soroban_sdk_auth::{
    check_auth, NonceAuth, {Identifier, Signature},
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Acc(Identifier),
    Nonce(Identifier),
    Admin,
}

fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    if let Some(nonce) = e.contract_data().get(key) {
        nonce.unwrap()
    } else {
        BigInt::zero(e)
    }
}
struct WrappedAuth(Signature);

impl NonceAuth for WrappedAuth {
    fn read_nonce(e: &Env, id: Identifier) -> BigInt {
        read_nonce(e, id)
    }

    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
        let key = DataKey::Nonce(id.clone());
        let nonce = Self::read_nonce(e, id);
        e.contract_data()
            .set(key, nonce.clone() + BigInt::from_u32(e, 1));
        nonce
    }

    fn signature(&self) -> &Signature {
        &self.0
    }
}

pub struct AuthContract;

#[cfg_attr(feature = "export", contractimpl)]
#[cfg_attr(not(feature = "export"), contractimpl(export = false))]
impl AuthContract {
    // Sets the admin identifier
    pub fn set_admin(e: Env, admin: Identifier) {
        if e.contract_data().has(DataKey::Admin) {
            panic!("admin is already set")
        }

        e.contract_data().set(DataKey::Admin, admin);
    }

    // Saves data that corresponds to an Identifier, with that Identifiers authorization
    pub fn save_data(e: Env, auth: Signature, nonce: BigInt, num: BigInt) {
        let auth_id = auth.get_identifier(&e);

        check_auth(
            &e,
            &WrappedAuth(auth),
            nonce.clone(),
            Symbol::from_str("save_data"),
            (auth_id.clone(), nonce, num.clone()).into_val(&e),
        );

        e.contract_data().set(DataKey::Acc(auth_id), num);
    }

    // The admin can write data for any Identifier
    pub fn overwrite(e: Env, auth: Signature, nonce: BigInt, id: Identifier, num: BigInt) {
        let auth_id = auth.get_identifier(&e);
        if auth_id != e.contract_data().get_unchecked(DataKey::Admin).unwrap() {
            panic!("not authorized by admin")
        }

        check_auth(
            &e,
            &WrappedAuth(auth),
            nonce.clone(),
            Symbol::from_str("overwrite"),
            (auth_id, nonce, id.clone(), num.clone()).into_val(&e),
        );

        e.contract_data().set(DataKey::Acc(id), num);
    }

    pub fn nonce(e: Env, to: Identifier) -> BigInt {
        read_nonce(&e, to)
    }
}
