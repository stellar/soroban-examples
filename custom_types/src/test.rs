#![cfg(test)]

use super::*;
use soroban_sdk::{Env, FixedBinary};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, CustomTypesContract);

    assert_eq!(retrieve::invoke(&env, &contract_id), Name::None);

    store::invoke(
        &env,
        &contract_id,
        &Name::First(First {
            first: Symbol::from_str("firstonly"),
        }),
    );

    assert_eq!(
        retrieve::invoke(&env, &contract_id),
        Name::First(First {
            first: Symbol::from_str("firstonly"),
        }),
    );

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
