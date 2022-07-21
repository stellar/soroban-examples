#![cfg(feature = "external")]

use num_bigint::BigInt;
use stellar_contract_sdk::{Binary, Env, VariableLengthBinary};
use stellar_token_contract::external::{Identifier, U256};
use stellar_xdr::HostFunction;

pub fn register_test_contract(e: &Env, contract_id: &U256) {
    let mut bin = Binary::new(e);
    for b in contract_id {
        bin.push(*b);
    }

    e.register_contract(bin.into(), crate::LiquidityPool {});
}

pub fn initialize(e: &mut Env, contract_id: &U256, token_a: &U256, token_b: &U256) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "initialize", token_a, token_b)
            .try_into()
            .unwrap(),
    );
}

pub fn share_id(e: &mut Env, contract_id: &U256) -> U256 {
    use stellar_xdr::{ScObject, ScVal};
    let id = e.invoke_contract(
        HostFunction::Call,
        (contract_id, "share_id").try_into().unwrap(),
    );
    if let ScVal::Object(Some(ScObject::Binary(bin))) = id {
        bin.as_slice().try_into().unwrap()
    } else {
        panic!()
    }
}

pub fn deposit(e: &mut Env, contract_id: &U256, to: &Identifier) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "deposit", to).try_into().unwrap(),
    );
}

pub fn swap(e: &mut Env, contract_id: &U256, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "swap", to, out_a, out_b).try_into().unwrap(),
    );
}

pub fn withdraw(e: &mut Env, contract_id: &U256, to: &Identifier) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "withdraw", to).try_into().unwrap(),
    );
}
