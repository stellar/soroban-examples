#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.total_supply(), 0);

    let alice = Address::generate(&e);
    let bob = Address::generate(&e);

    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 0);

    client.mint(&alice, &5);

    assert_eq!(client.total_supply(), 1);
    assert_eq!(client.token_by_index(&0), 5);
    assert_eq!(client.balance(&alice), 1);
    assert_eq!(client.token_of_owner_by_index(&alice, &0), 5);
    assert_eq!(client.balance(&bob), 0);

    client.mint(&bob, &25);

    assert_eq!(client.total_supply(), 2);
    assert_eq!(client.token_by_index(&0), 5);
    assert_eq!(client.token_by_index(&1), 25);
    assert_eq!(client.balance(&alice), 1);
    assert_eq!(client.token_of_owner_by_index(&alice, &0), 5);
    assert_eq!(client.balance(&bob), 1);
    assert_eq!(client.token_of_owner_by_index(&bob, &0), 25);

    client.transfer(&alice, &bob, &5);

    assert_eq!(client.total_supply(), 2);
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 2);
    assert_eq!(client.token_of_owner_by_index(&bob, &0), 25);
    assert_eq!(client.token_of_owner_by_index(&bob, &1), 5);
}
