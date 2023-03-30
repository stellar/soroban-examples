use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn read(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    e.storage().get(&key).transpose().unwrap().unwrap_or_default()
}

fn write(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);
    e.storage().set(&key, &amount);
}

pub fn receive(e: &Env, addr: Address, amount: i128) {
    let balance = read(e, addr.clone());
    assert!(is_authorized(e, addr.clone()), "can't receive when deauthorized");
    write(e, addr, balance + amount);
}

pub fn spend(e: &Env, addr: Address, amount: i128) {
    let balance = read(e, addr.clone());

    assert!(
        is_authorized(e, addr.clone()),
        "can't spend when deauthorized"
    );

    assert!(amount <= balance, "insufficient balance");

    write(e, addr, balance - amount);
}

pub fn is_authorized(e: &Env, addr: Address) -> bool {
    let key = DataKey::State(addr);
    if let Some(state) = e.storage().get(&key) {
        state.unwrap()
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
    let key = DataKey::State(addr);
    e.storage().set(&key, &is_authorized);
}
