#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, testutils::Logger, BytesN, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol!("Dev"));

    let logs = env.logger().all();
    assert_eq!(logs, std::vec!["Hello Symbol(Dev)"]);
    std::println!("{}", logs.join("\n"));
}
