use crate::storage_types::DataKey;
use soroban_sdk::{Bytes, Env};

pub fn read_decimal(e: &Env) -> u32 {
    let key = DataKey::Decimals;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_decimal(e: &Env, d: u8) {
    let key = DataKey::Decimals;
    e.storage().set(&key, &u32::from(d))
}

pub fn read_name(e: &Env) -> Bytes {
    let key = DataKey::Name;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_name(e: &Env, d: Bytes) {
    let key = DataKey::Name;
    e.storage().set(&key, &d)
}

pub fn read_symbol(e: &Env) -> Bytes {
    let key = DataKey::Symbol;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_symbol(e: &Env, d: Bytes) {
    let key = DataKey::Symbol;
    e.storage().set(&key, &d)
}
