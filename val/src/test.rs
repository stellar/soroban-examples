#![cfg(test)]
use crate::{Contract, ContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal,
};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let owner_a = Address::generate(&env);
    let owner_b = Address::generate(&env);

    let token_a = client.mint(&owner_a);
    let token_b = client.mint(&owner_a);
    assert_eq!(client.owner(&token_a), owner_a);
    assert_eq!(client.owner(&token_b), owner_a);

    client.mock_all_auths().transfer(&token_a, &owner_b);
    assert_eq!(
        env.auths(),
        [(
            owner_a.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id.clone(),
                    symbol_short!("transfer"),
                    (&token_a, &owner_b).into_val(&env)
                )),
                sub_invocations: [].into(),
            }
        )]
    );
    assert_eq!(client.owner(&token_a), owner_b);
    assert_eq!(client.owner(&token_b), owner_a);
}
