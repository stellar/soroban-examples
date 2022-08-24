#![cfg(test)]

use super::*;
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, CustomTypesContract);

    assert_eq!(retrieve::invoke(&env, &contract_id), Name::None);

    store::invoke(
        &env,
        &contract_id,
        &Name::FirstLast(FirstLast {
            first: Symbol::from_str("first"),
            last: Symbol::from_str("last"),
        }),
    );

    assert_eq!(
        retrieve::invoke(&env, &contract_id),
        Name::FirstLast(FirstLast {
            first: Symbol::from_str("first"),
            last: Symbol::from_str("last"),
        }),
    );
}
