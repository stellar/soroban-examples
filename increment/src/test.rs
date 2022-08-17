#![cfg(test)]

use super::{increment, IncrementContract};
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, IncrementContract);

    let count = increment::invoke(&env, &contract_id);
    assert_eq!(count, 1);

    let count = increment::invoke(&env, &contract_id);
    assert_eq!(count, 2);

    let count = increment::invoke(&env, &contract_id);
    assert_eq!(count, 3);
}
