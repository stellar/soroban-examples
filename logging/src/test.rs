#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol_short!("Dev"));

    let logs = env.logs().all();
    assert_eq!(logs, std::vec!["Hello Symbol(Dev)"]);
    std::println!("{}", logs.join("\n"));
}
