#![cfg(test)]
use crate::{Error, IncrementContract, IncrementContractArgs, IncrementContractClient, Pause};
use soroban_sdk::{contract, contractimpl, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Pause, ());
    let client = PauseClient::new(&env, &contract_id);

    assert_eq!(client.pause(), false);
    client.set(true);
    assert_eq!(client.pause(), true);
}
