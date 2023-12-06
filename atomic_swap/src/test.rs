#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal,
};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

fn create_atomic_swap_contract(e: &Env) -> AtomicSwapContractClient {
    AtomicSwapContractClient::new(e, &e.register_contract(None, AtomicSwapContract {}))
}

#[test]
fn test_atomic_swap() {
    let env = Env::default();
    env.mock_all_auths();

    let a = Address::generate(&env);
    let b = Address::generate(&env);

    let token_admin = Address::generate(&env);

    let (token_a, token_a_admin) = create_token_contract(&env, &token_admin);
    let (token_b, token_b_admin) = create_token_contract(&env, &token_admin);
    token_a_admin.mint(&a, &1000);
    token_b_admin.mint(&b, &5000);

    let contract = create_atomic_swap_contract(&env);

    contract.swap(
        &a,
        &b,
        &token_a.address,
        &token_b.address,
        &1000,
        &4500,
        &5000,
        &950,
    );

    assert_eq!(
        env.auths(),
        std::vec![
            (
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract.address.clone(),
                        symbol_short!("swap"),
                        (
                            token_a.address.clone(),
                            token_b.address.clone(),
                            1000_i128,
                            4500_i128
                        )
                            .into_val(&env),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            token_a.address.clone(),
                            symbol_short!("transfer"),
                            (a.clone(), contract.address.clone(), 1000_i128,).into_val(&env),
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            ),
            (
                b.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract.address.clone(),
                        symbol_short!("swap"),
                        (
                            token_b.address.clone(),
                            token_a.address.clone(),
                            5000_i128,
                            950_i128
                        )
                            .into_val(&env),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            token_b.address.clone(),
                            symbol_short!("transfer"),
                            (b.clone(), contract.address.clone(), 5000_i128,).into_val(&env),
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            ),
        ]
    );

    assert_eq!(token_a.balance(&a), 50);
    assert_eq!(token_a.balance(&b), 950);

    assert_eq!(token_b.balance(&a), 4500);
    assert_eq!(token_b.balance(&b), 500);
}
