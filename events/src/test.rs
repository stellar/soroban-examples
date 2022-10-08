#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    env.events().all().iter().map(Result::unwrap).for_each(|e| {
        std::println!(
            "event:\n - contract: {:?}\n - topics: {:?}\n - value: {:?}",
            e.0,
            e.1,
            e.2
        );
    });
}
