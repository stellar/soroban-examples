#![cfg(test)]

use super::{a::ContractA, b::add_with, b::ContractB};
use soroban_sdk::{Env, FixedBinary};

#[test]
fn test() {
    let env = Env::default();

    let contract_a = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_a, ContractA);

    let contract_b = FixedBinary::from_array(&env, [1; 32]);
    env.register_contract(&contract_b, ContractB);

    assert_eq!(
        add_with::invoke(
            &env,
            // Invoke 'add_with' on contract B.
            &contract_b,
            // Value X.
            &5,
            // Value Y.
            &7,
            // Tell contract B to call contract A.
            &contract_a,
        ),
        // Expect result 12.
        12
    );
}
