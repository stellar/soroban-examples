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
        .mint(&token, &admin, &user, &123);
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
                    &token,
                    &minter,
                    MinterConfig {
                        limit: 100,
                        epoch_length: 17820,
                    },
                )
                    .into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_minter(
            &token,
            &minter,
            &MinterConfig {
                limit: 100,
                epoch_length: 17820,
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
        .mint(&token, &minter, &user, &97i128);
    assert_eq!(token_client.balance(&user), 97);
    assert_eq!(
        mint_lock_client.minter(&token, &minter),
        (
            MinterConfig {
                limit: 100,
                epoch_length: 17820
            },
            0,
            MinterStats { consumed_limit: 97 }
        )
    );
}

#[contract]
struct NoopMintContract;

#[contractimpl]
impl MintInterface for NoopMintContract {
    fn mint(_env: Env, _to: Address, _amount: i128) {}
}

#[test]
fn test_disallow_negative() {
    let env = Env::default();
    let mint_lock = env.register_contract(None, Contract);
    let mint_lock_client = ContractClient::new(&env, &mint_lock);

    let admin = Address::generate(&env);

    mint_lock_client.set_admin(&admin);

    let token = env.register_contract(None, NoopMintContract);

    // Admin can always mint.
    let user = Address::generate(&env);
    assert_eq!(
        mint_lock_client
            .mock_auths(&[MockAuth {
                address: &admin,
                invoke: &MockAuthInvoke {
                    contract: &mint_lock,
                    fn_name: "mint",
                    args: (&token, &user, -123i128).into_val(&env),
                    sub_invokes: &[],
                },
            }])
            .try_mint(&token, &admin, &user, &-123),
        Err(Ok(Error::NegativeAmount)),
    );

    // Authorized Minter can mint.
    let minter = Address::generate(&env);
    mint_lock_client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &mint_lock,
                fn_name: "set_minter",
                args: (
                    &token,
                    &minter,
                    MinterConfig {
                        limit: 100,
                        epoch_length: 17820,
                    },
                )
                    .into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_minter(
            &token,
            &minter,
            &MinterConfig {
                limit: 100,
                epoch_length: 17820,
            },
        );
    let user = Address::generate(&env);
    assert_eq!(
        mint_lock_client
            .mock_auths(&[MockAuth {
                address: &minter,
                invoke: &MockAuthInvoke {
                    contract: &mint_lock,
                    fn_name: "mint",
                    args: (&token, &user, -1000i128).into_val(&env),
                    sub_invokes: &[],
                },
            }])
            .try_mint(&token, &minter, &user, &-1000i128),
        Err(Ok(Error::NegativeAmount)),
    );
    assert_eq!(
        mint_lock_client.minter(&token, &minter),
        (
            MinterConfig {
                limit: 100,
                epoch_length: 17820
            },
            0,
            MinterStats { consumed_limit: 0 }
        )
    );
}
