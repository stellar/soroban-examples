#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.hello(&symbol!("Dev"));
}
