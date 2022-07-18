#![cfg(feature = "external")]

use num_bigint::BigInt;
use stellar_contract_sdk::{Binary, Env, VariableLengthBinary};
use stellar_token_contract::external::{Identifier, ToScVec, U256};
use stellar_xdr::HostFunction;

macro_rules! contract_fn {
    ($name:ident, $f:ident, $n:tt, $($i:tt),*) => {
        pub fn $name(e: Env, args: &[RawVal]) -> RawVal {
            if args.len() != $n {
                panic!()
            } else {
                crate::$f(e, $(args[$i]),*)
            }
        }
    }
}

mod contract_fns {
    use stellar_contract_sdk::{Env, RawVal};

    contract_fn!(initialize, __initialize, 2, 0, 1);
    contract_fn!(share_id, __share_id, 0,);
    contract_fn!(deposit, __deposit, 1, 0);
    contract_fn!(swap, __swap, 3, 0, 1, 2);
    contract_fn!(withdraw, __withdraw, 1, 0);
}

pub fn register_test_contract(e: &Env, contract_id: &U256) {
    let mut bin = Binary::new(e);
    for b in contract_id {
        bin.push(*b);
    }

    let mut tc = stellar_contract_sdk::TestContract::new();
    tc.add_function("initialize", &contract_fns::initialize);
    tc.add_function("share_id", &contract_fns::share_id);
    tc.add_function("deposit", &contract_fns::deposit);
    tc.add_function("swap", &contract_fns::swap);
    tc.add_function("withdraw", &contract_fns::withdraw);
    e.register_contract(bin.into(), tc);
}

pub fn initialize(e: &mut Env, contract_id: &U256, token_a: &U256, token_b: &U256) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "initialize", token_a, token_b)
            .to_scvec()
            .unwrap(),
    );
}

pub fn share_id(e: &mut Env, contract_id: &U256) -> U256 {
    use stellar_xdr::{ScObject, ScVal};
    let id = e.invoke_contract(
        HostFunction::Call,
        (contract_id, "share_id").to_scvec().unwrap(),
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
        (contract_id, "deposit", to).to_scvec().unwrap(),
    );
}

pub fn swap(e: &mut Env, contract_id: &U256, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "swap", to, out_a, out_b).to_scvec().unwrap(),
    );
}

pub fn withdraw(e: &mut Env, contract_id: &U256, to: &Identifier) {
    e.invoke_contract(
        HostFunction::Call,
        (contract_id, "withdraw", to).to_scvec().unwrap(),
    );
}
