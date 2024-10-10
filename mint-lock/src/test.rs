#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{storage::Temporary as _, Address as _, Ledger, MockAuth, MockAuthInvoke},
    token::TokenClient,
    Address, Env, IntoVal,
};

#[test]
fn test() {
    const EPOCH_LENGTH: u32 = 10000;
    const LEDGER_EPOCH: u32 = 150;

    let env = Env::default();

    env.ledger().with_mut(|ledger_info| {
        ledger_info.sequence_number = EPOCH_LENGTH * LEDGER_EPOCH + EPOCH_LENGTH / 2;
    });

    let admin = Address::generate(&env);

    let mint_lock = env.register(Contract, (&admin,));
    let mint_lock_client = ContractClient::new(&env, &mint_lock);

    let token = env
        .register_stellar_asset_contract_v2(mint_lock.clone())
        .address();
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
                        epoch_length: EPOCH_LENGTH,
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
                epoch_length: EPOCH_LENGTH,
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
                epoch_length: EPOCH_LENGTH
            },
            LEDGER_EPOCH,
            MinterStats { consumed_limit: 97 }
        )
    );
    env.as_contract(&mint_lock, || {
        assert_eq!(
            env.storage().temporary().get_ttl(&StorageKey::MinterStats(
                token.clone(),
                minter.clone(),
                EPOCH_LENGTH,
                LEDGER_EPOCH,
            )),
            EPOCH_LENGTH * (LEDGER_EPOCH + 1) - 1
        );
    });
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
    let admin = Address::generate(&env);
    let mint_lock = env.register(Contract, (&admin,));
    let mint_lock_client = ContractClient::new(&env, &mint_lock);

    let token = env.register(NoopMintContract, ());

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
