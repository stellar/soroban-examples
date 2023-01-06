#![cfg(test)]

use super::{AllocContract, AllocContractClient};
use soroban_sdk::{testutils::Logger, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllocContract);
    let client = AllocContractClient::new(&env, &contract_id);
    client.init();

    assert_eq!(client.grow(&1), 1);
    assert_eq!(client.grow(&2), 3);
    assert_eq!(client.grow(&5), 8);

    std::println!("{}", env.logger().all().join("\n"));
}
