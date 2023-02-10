use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn has(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(&key)
}

fn read(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().set(&key, id);
}

pub fn check(e: &Env, admin: &Address) {
    assert!(admin == &read(e), "not authorized by admin");
}
