#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logger, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.try_increment(), Ok(Ok(1)));
    assert_eq!(client.try_increment(), Ok(Ok(2)));
    assert_eq!(client.try_increment(), Ok(Ok(3)));
    assert_eq!(client.try_increment(), Ok(Ok(4)));
    assert_eq!(client.try_increment(), Ok(Ok(5)));
    assert_eq!(client.try_increment(), Err(Ok(Error::LimitReached)));

    std::println!("{}", env.logger().all().join("\n"));
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_panic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.increment(), 4);
    assert_eq!(client.increment(), 5);
    client.increment();
}
