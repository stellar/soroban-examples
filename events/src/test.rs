#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.hello(&symbol!("Dev"));
}
