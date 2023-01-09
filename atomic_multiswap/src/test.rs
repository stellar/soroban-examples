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
    AtomicMultiSwapContractClient::new(e, e.register_contract(None, AtomicMultiSwapContract {}))
}

#[test]
fn test_atomic_multi_swap() {
    let env: Env = Default::default();
    let accounts_a = [
        AccountSwap {
            account: Account::random(&env),
            amount: 2000,
            min_recv: 290,
        },
        AccountSwap {
            account: Account::random(&env),
            amount: 3000,
            min_recv: 350,
        },
        AccountSwap {
            account: Account::random(&env),
            amount: 4000,
            min_recv: 301,
        },
    ];
    let accounts_b = [
        AccountSwap {
            account: Account::random(&env),
            amount: 300,
            min_recv: 2100,
        },
        AccountSwap {
            account: Account::random(&env),
            amount: 295,
            min_recv: 1950,
        },
        AccountSwap {
            account: Account::random(&env),
            amount: 400,
            min_recv: 2900,
        },
    ];

    let token_admin = Account::random(&env);

    let token_a = create_token_contract(&env, &token_admin.address());
    let token_b = create_token_contract(&env, &token_admin.address());
    token_a.mint(&token_admin, &accounts_a[0].account.address(), &2000);
    token_a.mint(&token_admin, &accounts_a[1].account.address(), &3000);
    token_a.mint(&token_admin, &accounts_a[2].account.address(), &4000);

    token_b.mint(&token_admin, &accounts_b[0].account.address(), &300);
    token_b.mint(&token_admin, &accounts_b[1].account.address(), &295);
    token_b.mint(&token_admin, &accounts_b[2].account.address(), &400);

    let contract = create_atomic_multiswap_contract(&env);

    let swap_contract_id = env.register_contract_wasm(None, atomic_swap::WASM);
    // let swap_contract_id = env.register_contract(None, AtomicSwapContract {});

    contract.swap(
        &swap_contract_id,
        &token_a.contract_id,
        &token_b.contract_id,
        &Vec::from_array(&env, accounts_a.clone()),
        &Vec::from_array(&env, accounts_b.clone()),
    );

    // check that only 4 swaps are authorized and accounts A[0] and B[1] didn't
    // authorize anything (and hence the signature can be reused).
    assert!(env.verify_account_authorization(
        &accounts_a[0].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            2000_i128,
            290_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &accounts_a[1].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            3000_i128,
            350_i128
        )
            .into_val(&env),
    ));

    assert!(env.verify_account_authorization(
        &accounts_b[1].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            295_i128,
            1950_i128,
        )
            .into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &accounts_b[2].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            400_i128,
            2900_i128,
        )
            .into_val(&env),
    ));

    // no swaps happen for these two accounts:
    assert!(!env.verify_account_authorization(
        &accounts_a[2].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            4000_i128,
            301_i128
        )
            .into_val(&env),
    ));
    assert!(!env.verify_account_authorization(
        &accounts_b[0].account,
        &[(&swap_contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            300_i128,
            2100_i128
        )
            .into_val(&env),
    ));

    // Smoke test one approve
    let swap_contract_address = Address::from_contract_id(&env, &swap_contract_id);
    assert!(env.verify_account_authorization(
        &accounts_a[0].account,
        &[
            (&swap_contract_id, "swap"),
            (&token_a.contract_id, "incr_allow"),
        ],
        (&swap_contract_address, 2000_i128,).into_val(&env),
    ));

    // Balance has to be checked after the auth checks because auth is only
    // stored for the last invocation currently.
    assert_eq!(token_a.balance(&accounts_a[0].account.address()), 50);
    assert_eq!(token_a.balance(&accounts_a[1].account.address()), 100);
    assert_eq!(token_a.balance(&accounts_a[2].account.address()), 4000);

    assert_eq!(token_a.balance(&accounts_b[0].account.address()), 0);
    assert_eq!(token_a.balance(&accounts_b[1].account.address()), 1950);
    assert_eq!(token_a.balance(&accounts_b[2].account.address()), 2900);

    assert_eq!(token_b.balance(&accounts_a[0].account.address()), 290);
    assert_eq!(token_b.balance(&accounts_a[1].account.address()), 350);
    assert_eq!(token_b.balance(&accounts_a[2].account.address()), 0);

    assert_eq!(token_b.balance(&accounts_b[0].account.address()), 300);
    assert_eq!(token_b.balance(&accounts_b[1].account.address()), 5);
    assert_eq!(token_b.balance(&accounts_b[2].account.address()), 50);
}

#[test]
fn test_multi_swap_with_duplicate_account() {
    let env: Env = Default::default();
    let acc_a = Account::random(&env);
    let acc_b = Account::random(&env);
    let accounts_a = [
        AccountSwap {
            account: acc_a.clone(),
            amount: 1000,
            min_recv: 100,
        },
        AccountSwap {
            account: acc_a.clone(),
            amount: 2000,
            min_recv: 190,
        },
    ];
    let accounts_b = [
        AccountSwap {
            account: acc_b.clone(),
            amount: 101,
            min_recv: 1000,
        },
        AccountSwap {
            account: acc_b.clone(),
            amount: 190,
            min_recv: 2000,
        },
    ];

    let token_admin = Account::random(&env);

    let token_a = create_token_contract(&env, &token_admin.address());
    let token_b = create_token_contract(&env, &token_admin.address());
    token_a.mint(&token_admin, &acc_a.address(), &3000);
    token_b.mint(&token_admin, &acc_b.address(), &291);

    let contract = create_atomic_multiswap_contract(&env);

    let swap_contract_id = env.register_contract_wasm(None, atomic_swap::WASM);

    contract.swap(
        &swap_contract_id,
        &token_a.contract_id,
        &token_b.contract_id,
        &Vec::from_array(&env, accounts_a.clone()),
        &Vec::from_array(&env, accounts_b.clone()),
    );

    assert!(env.verify_account_authorization(
        &acc_a,
        &[(&swap_contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            1000_i128,
            100_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &acc_a,
        &[(&swap_contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            2000_i128,
            190_i128
        )
            .into_val(&env),
    ));

    assert!(env.verify_account_authorization(
        &acc_b,
        &[(&swap_contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            101_i128,
            1000_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &acc_b,
        &[(&swap_contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            190_i128,
            2000_i128
        )
            .into_val(&env),
    ));
    // Balance has to be checked after the auth checks because auth is only
    // stored for the last invocation currently.
    assert_eq!(token_a.balance(&acc_a.address()), 0);
    assert_eq!(token_a.balance(&acc_b.address()), 3000);

    assert_eq!(token_b.balance(&acc_a.address()), 290);
    assert_eq!(token_b.balance(&acc_b.address()), 1);
}
