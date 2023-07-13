#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Logs, Address, BytesN, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();

    let id_bytes = BytesN::from_array(&env, &[8; 32]);
    let contract_id = env.register_contract(&Address::from_contract_id(&id_bytes), Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol_short!("Dev"));

    let logs = env.logs().all();
    assert_eq!(logs, std::vec!["[Diagnostic Event] contract:0808080808080808080808080808080808080808080808080808080808080808, topics:[log], data:[\"Hello {}\", Dev]"]);
    std::println!("{}", logs.join("\n"));
}
