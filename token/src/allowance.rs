use crate::storage_types::{AllowanceDataKey, DataKey};
use soroban_auth::Identifier;
use soroban_sdk::{BigInt, Env};

pub fn read_allowance(e: &Env, from: Identifier, spender: Identifier) -> BigInt {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.data().get(key) {
        allowance.unwrap()
    } else {
        BigInt::zero(e)
    }
}

pub fn write_allowance(e: &Env, from: Identifier, spender: Identifier, amount: BigInt) {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.data().set(key, amount);
}

pub fn spend_allowance(e: &Env, from: Identifier, spender: Identifier, amount: BigInt) {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance < amount {
        panic!("insufficient allowance");
    }
    write_allowance(e, from, spender, allowance - amount);
}
