#![cfg(test)]
use crate::{Receipt, ReceiptAnchor, ReceiptAnchorClient};
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, IntoVal, Val, Vec};

fn setup(env: &Env) -> (ReceiptAnchorClient<'_>, Address, Address) {
    env.mock_all_auths();
    let contract_id = env.register(ReceiptAnchor, ());
    let client = ReceiptAnchorClient::new(env, &contract_id);
    let caller = Address::generate(env);
    let target = Address::generate(env);
    (client, caller, target)
}

fn call_args(env: &Env, amount: u32, deadline: u32) -> Vec<Val> {
    let mut args = Vec::new(env);
    args.push_back(amount.into_val(env));
    args.push_back(deadline.into_val(env));
    args
}

fn result_val(env: &Env, value: u32) -> Val {
    value.into_val(env)
}

#[test]
fn anchor_stores_the_commitment_under_the_caller() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let args = call_args(&env, 100, 42);
    let result = result_val(&env, 7);

    let commitment = client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result, &99);

    assert_eq!(client.count(&caller), 1);
    let stored = client.receipt(&caller, &0u32).unwrap();
    assert_eq!(stored.commitment, commitment);
    assert_eq!(stored.ledger, 99);
    assert_eq!(
        stored.commitment,
        client.commitment(&target, &symbol_short!("swap"), &args, &result, &99)
    );
}

#[test]
fn verify_accepts_the_invocation_it_was_anchored_for() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let args = call_args(&env, 100, 42);
    let result = result_val(&env, 7);

    client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result, &99);
    let stored = client.receipt(&caller, &0u32).unwrap();

    assert!(client.verify(&stored, &args, &result));
}

#[test]
fn verify_rejects_a_different_result() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let args = call_args(&env, 100, 42);

    client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result_val(&env, 7), &99);
    let stored = client.receipt(&caller, &0u32).unwrap();

    // Same caller's record, one unit of result changed.
    assert!(!client.verify(&stored, &args, &result_val(&env, 8)));
}

#[test]
fn verify_rejects_different_args() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let args = call_args(&env, 100, 42);

    client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result_val(&env, 7), &99);
    let stored = client.receipt(&caller, &0u32).unwrap();

    assert!(!client.verify(&stored, &call_args(&env, 101, 42), &result_val(&env, 7)));
    // Argument order is part of the commitment, not just the argument set.
    assert!(!client.verify(&stored, &call_args(&env, 42, 100), &result_val(&env, 7)));
    // A shorter argument list is not a prefix match.
    let mut shorter = Vec::new(&env);
    shorter.push_back(100u32.into_val(&env));
    assert!(!client.verify(&stored, &shorter, &result_val(&env, 7)));
}

#[test]
fn verify_rejects_a_different_ledger() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let args = call_args(&env, 100, 42);
    let result = result_val(&env, 7);

    client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result, &99);
    let stored = client.receipt(&caller, &0u32).unwrap();

    // A receipt that claims a different ledger no longer describes this
    // invocation, even though every other field is untouched.
    let restamped = Receipt {
        ledger: 100,
        ..stored.clone()
    };
    assert!(!client.verify(&restamped, &args, &result));
}

#[test]
fn verify_rejects_a_different_target() {
    let env = Env::default();
    let (client, caller, _target) = setup(&env);
    let args = call_args(&env, 100, 42);
    let result = result_val(&env, 7);
    let target = Address::generate(&env);

    client.anchor(&caller, &target, &symbol_short!("swap"), &args, &result, &99);
    let stored = client.receipt(&caller, &0u32).unwrap();

    // Same args, same result, same ledger: only the contract that was called
    // differs, and the commitment has to notice.
    let elsewhere = Receipt {
        target: Address::generate(&env),
        ..stored.clone()
    };
    assert!(!client.verify(&elsewhere, &args, &result));

    // The function name is committed too.
    let renamed = Receipt {
        fn_name: symbol_short!("mint"),
        ..stored
    };
    assert!(!client.verify(&renamed, &args, &result));
}

#[test]
fn the_same_invocation_commits_to_the_same_bytes_for_any_caller() {
    let env = Env::default();
    let (client, alice, target) = setup(&env);
    let bob = Address::generate(&env);
    let args = call_args(&env, 100, 42);
    let result = result_val(&env, 7);

    let a = client.anchor(&alice, &target, &symbol_short!("swap"), &args, &result, &99);
    let b = client.anchor(&bob, &target, &symbol_short!("swap"), &args, &result, &99);

    // The commitment is content-addressed: who anchored it is not part of it,
    // and anchoring the same facts twice is not a conflict.
    assert_eq!(a, b);
    assert_eq!(client.count(&alice), 1);
    assert_eq!(client.count(&bob), 1);
}

#[test]
fn receipts_are_indexed_per_caller_in_order() {
    let env = Env::default();
    let (client, caller, target) = setup(&env);
    let stranger = Address::generate(&env);

    let first = client.anchor(
        &caller,
        &target,
        &symbol_short!("swap"),
        &call_args(&env, 1, 1),
        &result_val(&env, 1),
        &10,
    );
    let second = client.anchor(
        &caller,
        &target,
        &symbol_short!("swap"),
        &call_args(&env, 2, 2),
        &result_val(&env, 2),
        &11,
    );

    assert_ne!(first, second);
    assert_eq!(client.count(&caller), 2);
    assert_eq!(client.receipt(&caller, &0u32).unwrap().commitment, first);
    assert_eq!(client.receipt(&caller, &1u32).unwrap().commitment, second);

    // Nothing landed in the stranger's list, and an unwritten index is None
    // rather than an empty receipt.
    assert_eq!(client.count(&stranger), 0);
    assert!(client.receipt(&caller, &2u32).is_none());
    assert!(client.receipt(&stranger, &0u32).is_none());
}

#[test]
#[should_panic]
fn anchor_requires_the_callers_authorization() {
    // No mock_all_auths here: a receipt is a statement about what the caller
    // did, so anchoring one without the caller's signature must not succeed.
    let env = Env::default();
    let contract_id = env.register(ReceiptAnchor, ());
    let client = ReceiptAnchorClient::new(&env, &contract_id);
    let caller = Address::generate(&env);
    let target = Address::generate(&env);

    client.anchor(
        &caller,
        &target,
        &symbol_short!("swap"),
        &call_args(&env, 1, 1),
        &result_val(&env, 1),
        &10,
    );
}
