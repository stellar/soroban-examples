//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, IntoVal};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice to demonstrate the usage of
    // allowance; full amounts could be swapped as well).
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        // Verify preconditions on the minimum price for both parties.
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }
        // Require authorization for a subset of arguments specific to a party.
        // Notice, that arguments are symmetric - there is no difference between
        // `a` and `b` in the call and hence their signatures can be used
        // either for `a` or for `b` role.
        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env),
        );

        // Perform the swap via two token transfers.
        move_token(&env, token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: BytesN<32>,
    from: &Address,
    to: &Address,
    approve_amount: i128,
    xfer_amount: i128,
) {
    let token = token::Client::new(&env, &token);
    let contract_address = env.current_contract_address();
    // This call needs to be authorized by `from` address. Since it increases
    // the allowance on behalf of the contract, `from` doesn't need to know `to`
    // at the signature time.
    token.incr_allow(&from, &contract_address, &approve_amount);
    token.xfer_from(&contract_address, &from, to, &xfer_amount);
}

mod test;
