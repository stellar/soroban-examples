#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CustomTypesContract);
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
