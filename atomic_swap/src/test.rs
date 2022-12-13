#![cfg(test)]

use super::*;
use soroban_sdk::{Address, Env, IntoVal};
use token::{Client as TokenClient, TokenMetadata};

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    let id = e.register_contract_token();
    let token = TokenClient::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.init(
        admin,
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    token
}

fn create_atomic_swap_contract(e: &Env) -> AtomicSwapContractClient {
    AtomicSwapContractClient::new(e, e.register_contract(None, AtomicSwapContract {}))
}

#[test]
fn test_atomic_swap() {
    let env: Env = Default::default();
    let account_a = Account::random(&env);
    let account_b = Account::random(&env);

    let token_admin = Account::random(&env);

    let token_a = create_token_contract(&env, &token_admin.address());
    let token_b = create_token_contract(&env, &token_admin.address());
    token_a.mint(&token_admin, &account_a.address(), &1000);
    token_b.mint(&token_admin, &account_b.address(), &5000);

    let contract = create_atomic_swap_contract(&env);

    contract.swap(
        &account_a,
        &account_b,
        &token_a.contract_id,
        &token_b.contract_id,
        &1000,
        &4500,
        &5000,
        &950,
    );

    assert!(env.verify_account_authorization(
        &account_a,
        &[(&contract.contract_id, "swap"),],
        (
            &token_a.contract_id,
            &token_b.contract_id,
            1000_i128,
            4500_i128
        )
            .into_val(&env),
    ));
    let contract_address = Address::from_contract_id(&env, &contract.contract_id);
    assert!(env.verify_account_authorization(
        &account_a,
        &[
            (&contract.contract_id, "swap"),
            (&token_a.contract_id, "approve"),
        ],
        (&contract_address, 1000_i128,).into_val(&env),
    ));

    assert!(env.verify_account_authorization(
        &account_b,
        &[(&contract.contract_id, "swap"),],
        (
            &token_b.contract_id,
            &token_a.contract_id,
            5000_i128,
            950_i128
        )
            .into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &account_b,
        &[
            (&contract.contract_id, "swap"),
            (&token_b.contract_id, "approve"),
        ],
        (&contract_address, 5000_i128,).into_val(&env),
    ));

    assert_eq!(token_a.balance(&account_a.address()), 50);
    assert_eq!(token_a.balance(&account_b.address()), 950);

    assert_eq!(token_b.balance(&account_a.address()), 4500);
    assert_eq!(token_b.balance(&account_b.address()), 500);
}
