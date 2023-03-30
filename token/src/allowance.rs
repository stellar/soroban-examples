use crate::storage_types::{AllowanceDataKey, DataKey};
use soroban_sdk::{Address, Env};

pub fn read(e: &Env, from: Address, spender: Address) -> i128 {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.storage().get(&key) {
        allowance.unwrap()
    } else {
        0
    }
}

pub fn write(e: &Env, from: Address, spender: Address, amount: i128) {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().set(&key, &amount);
}

pub fn spend(e: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read(e, from.clone(), spender.clone());
    assert!(allowance >= amount, "insufficient allowance");
    write(e, from, spender, allowance - amount);
}
