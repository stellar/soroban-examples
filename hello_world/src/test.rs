#![cfg(test)]

use super::*;
use soroban_sdk::{vec, BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, HelloContract);

    let words = hello::invoke(&env, &contract_id, &Symbol::from_str("SourBun"));
    assert_eq!(
        words,
        vec![&env, Symbol::from_str("Hello"), Symbol::from_str("SourBun"),]
    );
}
