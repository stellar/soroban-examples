#![cfg(feature = "external")]

use stellar_contract_sdk::{Binary, Env, VariableLengthBinary};
use stellar_token_contract::external::{Identifier, U256};
use stellar_xdr::HostFunction;

pub fn register_test_contract(e: &Env, contract_id: &U256) {
    let mut bin = Binary::new(e);
    for b in contract_id {
        bin.push(*b);
    }

    e.register_contract(bin.into(), crate::SingleOffer {});
}

pub fn initialize(
    e: &mut Env,
    contract_id: &U256,
    admin: Identifier,
    token_a: &U256,
    token_b: &U256,
    n: u32,
    d: u32,
) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "initialize", &admin, token_a, token_b, n, d)
            .try_into()
            .unwrap(),
    );
}

pub fn trade(e: &mut Env, contract_id: &U256, to: Identifier, min: u32) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "trade", &to, min).try_into().unwrap(),
    );
}

pub fn withdraw(e: &mut Env, contract_id: &U256) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "withdraw").try_into().unwrap(),
    );
}
