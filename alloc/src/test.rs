#![cfg(test)]

use super::{AllocContract, AllocContractClient};
use soroban_sdk::{testutils::Logger, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllocContract);
    let client = AllocContractClient::new(&env, &contract_id);
    assert_eq!(client.sum(&1), 0);
    assert_eq!(client.sum(&2), 1);
    assert_eq!(client.sum(&5), 10);

    std::println!("{}", env.logger().all().join("\n"));
}
