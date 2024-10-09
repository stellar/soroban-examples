#![cfg(test)]

use soroban_sdk::Env;

use crate::{
    contract_a::{ContractA, ContractAClient},
    contract_b::ContractB,
    contract_c::ContractC,
};
extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let a_address = env.register(ContractA, ());
    let b_address = env.register(ContractB, ());
    let c_address = env.register(ContractC, ());
    let client = ContractAClient::new(&env, &a_address);
    client.call_b(&b_address, &c_address);
}
