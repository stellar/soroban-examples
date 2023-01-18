use crate::storage_types::{AllowanceDataKey, DataKey};
use soroban_auth::Identifier;
use soroban_sdk::Env;

pub fn read_allowance(e: &Env, from: Identifier, spender: Identifier) -> i128 {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.storage().get(&key) {
        allowance.unwrap()
    } else {
        0
    }
}

pub fn write_allowance(e: &Env, from: Identifier, spender: Identifier, amount: i128) {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().set(&key, &amount);
}

pub fn spend_allowance(e: &Env, from: Identifier, spender: Identifier, amount: i128) {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance < amount {
        panic!("insufficient allowance");
    }
    write_allowance(e, from, spender, allowance - amount);
}
