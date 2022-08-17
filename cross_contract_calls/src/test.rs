#![cfg(test)]

use super::{a::ContractA, b::add_with, b::ContractB};
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();

    let contract_a = BytesN::from_array(&env, [0; 32]);
    env.register_contract(&contract_a, ContractA);

    let contract_b = BytesN::from_array(&env, [1; 32]);
    env.register_contract(&contract_b, ContractB);

    // Invoke 'add_with' on contract B.
    let sum = add_with::invoke(
        &env,
        &contract_b,
        // Value X.
        &5,
        // Value Y.
        &7,
        // Tell contract B to call contract A.
        &contract_a,
    );

    assert_eq!(sum, 12);
}
