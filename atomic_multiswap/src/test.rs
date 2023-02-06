#![cfg(test)]

use super::*;
use soroban_sdk::{Address, Env, IntoVal};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
    pub type TokenClient = Client;
}

use token::TokenClient;

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(
        e,
        &e.register_stellar_asset_contract_with_admin(admin.clone()),
    )
}

fn create_atomic_multiswap_contract(e: &Env) -> AtomicMultiSwapContractClient {
    AtomicMultiSwapContractClient::new(e, &e.register_contract(None, AtomicMultiSwapContract {}))
}

#[test]
fn test_atomic_multi_swap() {
    let env: Env = Default::default();
    let swaps_a = [
        SwapSpec {
            address: Address::random(&env),
            amount: 2000,
            min_recv: 290,
        },
        SwapSpec {
            address: Address::random(&env),
            amount: 3000,
            min_recv: 350,
        },
        SwapSpec {
            address: Address::random(&env),
            amount: 4000,
            min_recv: 301,
        },
    ];
    let swaps_b = [
        SwapSpec {
            address: Address::random(&env),
            amount: 300,
            min_recv: 2100,
        },
        SwapSpec {
            address: Address::random(&env),
            amount: 295,
            min_recv: 1950,
        },
        SwapSpec {
            address: Address::random(&env),
            amount: 400,
            min_recv: 2900,
        },
    ];

    let token_admin = Address::random(&env);

    let token_a = create_token_contract(&env, &token_admin);
    let token_b = create_token_contract(&env, &token_admin);
    token_a.mint(&token_admin, &swaps_a[0].address, &2000);
    token_a.mint(&token_admin, &swaps_a[1].address, &3000);
    token_a.mint(&token_admin, &swaps_a[2].address, &4000);

    token_b.mint(&token_admin, &swaps_b[0].address, &300);
    token_b.mint(&token_admin, &swaps_b[1].address, &295);
    token_b.mint(&token_admin, &swaps_b[2].address, &400);

    let contract = create_atomic_multiswap_contract(&env);

    let swap_contract_id = env.register_contract_wasm(None, atomic_swap::WASM);
    // let swap_contract_id = env.register_contract(None, AtomicSwapContract {});

    contract.swap(
        &swap_contract_id,
        &token_a.contract_id,
        &token_b.contract_id,
        &Vec::from_array(&env, swaps_a.clone()),
        &Vec::from_array(&env, swaps_b.clone()),
    );

    // check that only 4 swaps are authorized and accounts A[0] and B[1] didn't
    // authorize anything (and hence the signature can be reused).
    assert!(env.verify_top_authorization(
        &swaps_a[0].address,
        &swap_contract_id,
        "swap",
        (
            token_a.contract_id.clone(),
            token_b.contract_id.clone(),
            2000_i128,
            290_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_top_authorization(
        &swaps_a[1].address,
        &swap_contract_id,
        "swap",
        (
            token_a.contract_id.clone(),
            token_b.contract_id.clone(),
            3000_i128,
            350_i128
        )
            .into_val(&env),
    ));

    assert!(env.verify_top_authorization(
        &swaps_b[1].address,
        &swap_contract_id,
        "swap",
        (
            token_b.contract_id.clone(),
            token_a.contract_id.clone(),
            295_i128,
            1950_i128,
        )
            .into_val(&env),
    ));
    assert!(env.verify_top_authorization(
        &swaps_b[2].address,
        &swap_contract_id,
        "swap",
        (
            token_b.contract_id.clone(),
            token_a.contract_id.clone(),
            400_i128,
            2900_i128,
        )
            .into_val(&env),
    ));

    // no swaps happen for these two accounts:
    assert!(!env.verify_top_authorization(
        &swaps_a[2].address,
        &swap_contract_id,
        "swap",
        (
            token_a.contract_id.clone(),
            token_b.contract_id.clone(),
            4000_i128,
            301_i128
        )
            .into_val(&env),
    ));
    assert!(!env.verify_top_authorization(
        &swaps_b[0].address,
        &swap_contract_id,
        "swap",
        (
            token_b.contract_id.clone(),
            token_a.contract_id.clone(),
            300_i128,
            2100_i128
        )
            .into_val(&env),
    ));

    // Balance has to be checked after the auth checks because auth is only
    // stored for the last invocation currently.
    assert_eq!(token_a.balance(&swaps_a[0].address), 50);
    assert_eq!(token_a.balance(&swaps_a[1].address), 100);
    assert_eq!(token_a.balance(&swaps_a[2].address), 4000);

    assert_eq!(token_a.balance(&swaps_b[0].address), 0);
    assert_eq!(token_a.balance(&swaps_b[1].address), 1950);
    assert_eq!(token_a.balance(&swaps_b[2].address), 2900);

    assert_eq!(token_b.balance(&swaps_a[0].address), 290);
    assert_eq!(token_b.balance(&swaps_a[1].address), 350);
    assert_eq!(token_b.balance(&swaps_a[2].address), 0);

    assert_eq!(token_b.balance(&swaps_b[0].address), 300);
    assert_eq!(token_b.balance(&swaps_b[1].address), 5);
    assert_eq!(token_b.balance(&swaps_b[2].address), 50);
}

#[test]
fn test_multi_swap_with_duplicate_account() {
    let env: Env = Default::default();
    let address_a = Address::random(&env);
    let address_b = Address::random(&env);
    let swaps_a = [
        SwapSpec {
            address: address_a.clone(),
            amount: 1000,
            min_recv: 100,
        },
        SwapSpec {
            address: address_a.clone(),
            amount: 2000,
            min_recv: 190,
        },
    ];
    let swaps_b = [
        SwapSpec {
            address: address_b.clone(),
            amount: 101,
            min_recv: 1000,
        },
        SwapSpec {
            address: address_b.clone(),
            amount: 190,
            min_recv: 2000,
        },
    ];

    let token_admin = Address::random(&env);

    let token_a = create_token_contract(&env, &token_admin);
    let token_b = create_token_contract(&env, &token_admin);
    token_a.mint(&token_admin, &address_a, &3000);
    token_b.mint(&token_admin, &address_b, &291);

    let contract = create_atomic_multiswap_contract(&env);

    let swap_contract_id = env.register_contract_wasm(None, atomic_swap::WASM);

    contract.swap(
        &swap_contract_id,
        &token_a.contract_id,
        &token_b.contract_id,
        &Vec::from_array(&env, swaps_a.clone()),
        &Vec::from_array(&env, swaps_b.clone()),
    );

    assert!(env.verify_top_authorization(
        &address_a,
        &swap_contract_id,
        "swap",
        (
            token_a.contract_id.clone(),
            token_b.contract_id.clone(),
            1000_i128,
            100_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_top_authorization(
        &address_a,
        &swap_contract_id,
        "swap",
        (
            token_a.contract_id.clone(),
            token_b.contract_id.clone(),
            2000_i128,
            190_i128
        )
            .into_val(&env),
    ));

    assert!(env.verify_top_authorization(
        &address_b,
        &swap_contract_id,
        "swap",
        (
            token_b.contract_id.clone(),
            token_a.contract_id.clone(),
            101_i128,
            1000_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_top_authorization(
        &address_b,
        &swap_contract_id,
        "swap",
        (
            token_b.contract_id.clone(),
            token_a.contract_id.clone(),
            190_i128,
            2000_i128
        )
            .into_val(&env),
    ));
    // Balance has to be checked after the auth checks because auth is only
    // stored for the last invocation currently.
    assert_eq!(token_a.balance(&address_a), 0);
    assert_eq!(token_a.balance(&address_b), 3000);

    assert_eq!(token_b.balance(&address_a), 290);
    assert_eq!(token_b.balance(&address_b), 1);
}
