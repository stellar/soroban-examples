#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, Symbol};
mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
    pub type TokenClient = Client;
}

use token::TokenClient;

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_atomic_swap_contract(e: &Env) -> AtomicSwapContractClient {
    AtomicSwapContractClient::new(e, &e.register_contract(None, AtomicSwapContract {}))
}

#[test]
fn test_atomic_swap() {
    let env: Env = Default::default();
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
        &token_a.contract_id,
        &token_b.contract_id,
        &1000,
        &4500,
        &5000,
        &950,
    );

    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![
            (
                a.clone(),
                contract.contract_id.clone(),
                Symbol::short("swap"),
                (
                    token_a.contract_id.clone(),
                    token_b.contract_id.clone(),
                    1000_i128,
                    4500_i128
                )
                    .into_val(&env),
            ),
            (
                b.clone(),
                contract.contract_id.clone(),
                Symbol::short("swap"),
                (
                    token_b.contract_id.clone(),
                    token_a.contract_id.clone(),
                    5000_i128,
                    950_i128
                )
                    .into_val(&env),
            ),
        ]
    );

    assert_eq!(token_a.balance(&a), 50);
    assert_eq!(token_a.balance(&b), 950);

    assert_eq!(token_b.balance(&a), 4500);
    assert_eq!(token_b.balance(&b), 500);
}
