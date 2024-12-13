#![cfg(test)]
use crate::{Error, IncrementContract, IncrementContractArgs, IncrementContractClient};
use soroban_sdk::Env;

mod pause {
    soroban_sdk::contractimport!(
        file = "../pause/target/wasm32-unknown-unknown/release/soroban_pause_contract.wasm"
    );
}

#[test]
fn test_notpaused() {
    let env = Env::default();

    let pause_id = env.register(pause::WASM, ());
    let pause_client = pause::Client::new(&env, &pause_id);

    let contract_id = env.register(
        IncrementContract,
        IncrementContractArgs::__constructor(&pause_id),
    );
    let client = IncrementContractClient::new(&env, &contract_id);

    pause_client.set(&false);
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

#[test]
fn test_paused() {
    let env = Env::default();

    let pause_id = env.register(pause::WASM, ());
    let pause_client = pause::Client::new(&env, &pause_id);
    pause_client.set(&true);

    let contract_id = env.register(
        IncrementContract,
        IncrementContractArgs::__constructor(&pause_id),
    );
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.try_increment(), Err(Ok(Error::Paused)));
}
