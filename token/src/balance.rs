use crate::storage_types::DataKey;
use soroban_auth::Identifier;
use soroban_sdk::Env;

pub fn read_balance(e: &Env, id: Identifier) -> i128 {
    let key = DataKey::Balance(id);
    if let Some(balance) = e.storage().get(key) {
        balance.unwrap()
    } else {
        0
    }
}

fn write_balance(e: &Env, id: Identifier, amount: i128) {
    let key = DataKey::Balance(id);
    e.storage().set(key, amount);
}

pub fn receive_balance(e: &Env, id: Identifier, amount: i128) {
    let balance = read_balance(e, id.clone());
    if !is_authorized(e, id.clone()) {
        panic!("can't receive when deauthorized");
    }
    write_balance(e, id, balance + amount);
}

pub fn spend_balance(e: &Env, id: Identifier, amount: i128) {
    let balance = read_balance(e, id.clone());
    if !is_authorized(e, id.clone()) {
        panic!("can't spend when deauthorized");
    }
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, id, balance - amount);
}

pub fn is_authorized(e: &Env, id: Identifier) -> bool {
    let key = DataKey::State(id);
    if let Some(state) = e.storage().get(key) {
        state.unwrap()
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, id: Identifier, is_authorized: bool) {
    let key = DataKey::State(id);
    e.storage().set(key, is_authorized);
}
