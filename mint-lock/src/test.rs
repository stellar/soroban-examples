#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::TokenClient,
    Address, Env, IntoVal,
};

#[test]
fn test() {
    let env = Env::default();
    let mint_lock = env.register_contract(None, Contract);
    let mint_lock_client = ContractClient::new(&env, &mint_lock);

    let admin = Address::generate(&env);

    mint_lock_client.set_admin(&admin);

    let token = env.register_stellar_asset_contract(mint_lock.clone());
    let token_client = TokenClient::new(&env, &token);

    // Admin can always mint.
    let user = Address::generate(&env);
    mint_lock_client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &mint_lock,
                fn_name: "mint",
                args: (&token, &user, 123i128).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .mint(&admin, &token, &user, &123);
    assert_eq!(token_client.balance(&user), 123);

    // Authorized Minter can mint.
    let minter = Address::generate(&env);
    mint_lock_client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &mint_lock,
                fn_name: "set_minter",
                args: (
                    &minter,
                    MinterConfig {
                        limit: 100,
                        limit_ledger_count: 17820,
                    },
                )
                    .into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_minter(
            &minter,
            &MinterConfig {
                limit: 100,
                limit_ledger_count: 17820,
            },
        );
    let user = Address::generate(&env);
    mint_lock_client
        .mock_auths(&[MockAuth {
            address: &minter,
            invoke: &MockAuthInvoke {
                contract: &mint_lock,
                fn_name: "mint",
                args: (&token, &user, 97i128).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .mint(&minter, &token, &user, &97i128);
    assert_eq!(token_client.balance(&user), 97);
    assert_eq!(
        mint_lock_client.minter(&minter),
        (
            MinterConfig {
                limit: 100,
                limit_ledger_count: 17820
            },
            0,
            MinterStats { consumed_limit: 97 }
        )
    );
}
