#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logger, Env, Symbol};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&Symbol::short("Dev"));

    let logs = env.logger().all();
    assert_eq!(logs, std::vec!["Hello Symbol(Dev)"]);
    std::println!("{}", logs.join("\n"));
}
