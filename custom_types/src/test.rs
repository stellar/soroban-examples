#![cfg(test)]

use super::*;
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, CustomTypesContract);
    let client = CustomTypesContractClient::new(&env, &contract_id);

    assert_eq!(client.retrieve(), Name::None);

    client.store(&Name::FirstLast(FirstLast {
        first: symbol!("first"),
        last: symbol!("last"),
    }));

    assert_eq!(
        client.retrieve(),
        Name::FirstLast(FirstLast {
            first: symbol!("first"),
            last: symbol!("last"),
        }),
    );
}
