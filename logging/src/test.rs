#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Logs, Address, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();

    let addr = Address::from_str(
        &env,
        "CAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQMCJ",
    );
    let contract_id = env.register_at(&addr, Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol_short!("Dev"));

    let logs = env.logs().all();
    assert_eq!(logs, std::vec!["[Diagnostic Event] contract:CAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQMCJ, topics:[log], data:[\"Hello {}\", Dev]"]);
    std::println!("{}", logs.join("\n"));
}
