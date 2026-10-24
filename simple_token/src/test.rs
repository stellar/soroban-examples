#![cfg(test)]
extern crate alloc;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, String,
};

fn create_token<'a>(env: &Env, owner: &Address, supply: i128) -> (SimpleTokenClient<'a>, Address) {
    let contract_id = env.register(
        SimpleToken,
        (
            owner,
            supply,
            String::from_str(env, "Simple Token"),
            String::from_str(env, "SMPL"),
            7u32,
        ),
    );
    (SimpleTokenClient::new(env, &contract_id), contract_id)
}

#[test]
fn test_initial_state_and_metadata() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let initial_supply = 1_000_000_0000000i128; // 1,000,000 SMPL

    let (client, _) = create_token(&env, &owner, initial_supply);

    // Verify metadata
    assert_eq!(client.name(), String::from_str(&env, "Simple Token"));
    assert_eq!(client.symbol(), String::from_str(&env, "SMPL"));
    assert_eq!(client.decimals(), 7);

    // Verify supply and balance
    assert_eq!(client.total_supply(), initial_supply);
    assert_eq!(client.balance(&owner), initial_supply);

    // Non-owner should have zero balance
    let user = Address::generate(&env);
    assert_eq!(client.balance(&user), 0);
}

#[test]
fn test_transfer_success() {
    let env = Env::default();
    env.mock_all_auths();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let initial_supply = 500_000i128;

    let (client, _) = create_token(&env, &alice, initial_supply);

    // Alice transfers 150_000 to Bob
    client.transfer(&alice, &bob, &150_000);

    assert_eq!(client.balance(&alice), 350_000);
    assert_eq!(client.balance(&bob), 150_000);

    // Total supply remains strictly constant
    assert_eq!(client.total_supply(), initial_supply);

    // Bob transfers 50_000 to Alice
    client.transfer(&bob, &alice, &50_000);

    assert_eq!(client.balance(&alice), 400_000);
    assert_eq!(client.balance(&bob), 100_000);
    assert_eq!(client.total_supply(), initial_supply);
}

#[test]
fn test_transfer_requires_sender_auth() {
    let env = Env::default();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let initial_supply = 1000i128;

    let (client, contract_id) = create_token(&env, &alice, initial_supply);

    env.mock_all_auths();
    client.transfer(&alice, &bob, &300);

    // Verify that the required authorization specifically matches Alice transferring 300
    assert_eq!(
        env.auths(),
        [(
            alice.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id,
                    soroban_sdk::symbol_short!("transfer"),
                    (alice, bob, 300i128).into_val(&env)
                )),
                sub_invocations: alloc::vec::Vec::new(),
            }
        )]
    );
}

#[test]
fn test_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let initial_supply = 100i128;

    let (client, _) = create_token(&env, &alice, initial_supply);

    // Alice only has 100, attempts to transfer 101
    let err = client.try_transfer(&alice, &bob, &101).unwrap_err();
    assert_eq!(err, Ok(Error::InsufficientBalance));

    // Balances remain untouched
    assert_eq!(client.balance(&alice), 100);
    assert_eq!(client.balance(&bob), 0);
}

#[test]
fn test_non_positive_transfer_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let (client, _) = create_token(&env, &alice, 1000);

    // Transferring 0 should fail
    let err_zero = client.try_transfer(&alice, &bob, &0).unwrap_err();
    assert_eq!(err_zero, Ok(Error::NonPositiveAmount));

    // Transferring negative amount should fail
    let err_neg = client.try_transfer(&alice, &bob, &-50).unwrap_err();
    assert_eq!(err_neg, Ok(Error::NonPositiveAmount));
}

#[test]
#[should_panic]
fn test_constructor_negative_supply_panics() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let _ = env.register(
        SimpleToken,
        (
            &owner,
            0i128,
            String::from_str(&env, "Token"),
            String::from_str(&env, "TKN"),
            7u32,
        ),
    );
}

#[test]
#[should_panic]
fn test_constructor_decimal_too_large_panics() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let _ = env.register(
        SimpleToken,
        (
            &owner,
            1000i128,
            String::from_str(&env, "Token"),
            String::from_str(&env, "TKN"),
            19u32,
        ),
    );
}
