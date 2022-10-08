#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Accounts, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    assert_eq!(client.with_source_account(&user_1).increment(), 1);
    assert_eq!(client.with_source_account(&user_1).increment(), 2);
    assert_eq!(client.with_source_account(&user_2).increment(), 1);
    assert_eq!(client.with_source_account(&user_1).increment(), 3);
}
