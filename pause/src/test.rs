#![cfg(test)]
use crate::{Pause, PauseClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Pause, ());
    let client = PauseClient::new(&env, &contract_id);

    assert_eq!(client.paused(), false);
    client.set(&true);
    assert_eq!(client.paused(), true);
}
