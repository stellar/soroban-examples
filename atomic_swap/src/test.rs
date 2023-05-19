#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env, IntoVal, Symbol};
use token::Client as TokenClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_atomic_swap_contract(e: &Env) -> AtomicSwapContractClient {
    AtomicSwapContractClient::new(e, &e.register_contract(None, AtomicSwapContract {}))
}

#[test]
fn test_atomic_swap() {
    let env = Env::default();
    env.mock_all_auths();

    let a = Address::random(&env);
    let b = Address::random(&env);

    let token_admin = Address::random(&env);

    let token_a = create_token_contract(&env, &token_admin);
    let token_b = create_token_contract(&env, &token_admin);
    token_a.mint(&a, &1000);
    token_b.mint(&b, &5000);

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
                contract.address.clone(),
                Symbol::short("swap"),
                (
                    token_a.address.clone(),
                    token_b.address.clone(),
                    1000_i128,
                    4500_i128
                )
                    .into_val(&env),
            ),
            (
                a.clone(),
                token_a.address.clone(),
                Symbol::new(&env, "increase_allowance"),
                (a.clone(), &contract.address, 1000_i128).into_val(&env),
            ),
            (
                b.clone(),
                contract.address.clone(),
                Symbol::short("swap"),
                (
                    token_b.address.clone(),
                    token_a.address.clone(),
                    5000_i128,
                    950_i128
                )
                    .into_val(&env),
            ),
            (
                b.clone(),
                token_b.address.clone(),
                Symbol::new(&env, "increase_allowance"),
                (b.clone(), &contract.address, 5000_i128).into_val(&env),
            ),
        ]
    );

    assert_eq!(token_a.balance(&a), 50);
    assert_eq!(token_a.balance(&b), 950);

    assert_eq!(token_b.balance(&a), 4500);
    assert_eq!(token_b.balance(&b), 500);
}
