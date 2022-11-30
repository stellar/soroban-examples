#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}
