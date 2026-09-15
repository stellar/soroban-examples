//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

/// Contract implementing atomic multi-party token swap without mutual trust.
#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    /// Swaps token A for token B atomically between parties `a` and `b`.
    ///
    /// Each party authorizes transferring up to their offered amount (`amount_a` and `amount_b`)
    /// conditional on receiving at least their requested threshold (`min_b_for_a` and `min_a_for_b`).
    /// The contract transfers the requested minimum to each counterparty and refunds any remaining
    /// balance to the respective sender.
    ///
    /// # Arguments
    /// * `a` - Address of first party trading `token_a` for `token_b`.
    /// * `b` - Address of second party trading `token_b` for `token_a`.
    /// * `token_a` - Address of the first token contract.
    /// * `token_b` - Address of the second token contract.
    /// * `amount_a` - Maximum amount of `token_a` offered by party `a`.
    /// * `min_b_for_a` - Minimum acceptable threshold of `token_b` expected by party `a`.
    /// * `amount_b` - Maximum amount of `token_b` offered by party `b`.
    /// * `min_a_for_b` - Minimum acceptable threshold of `token_a` expected by party `b`.
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
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

        // Perform the swap by moving tokens from a to b and from b to a.
        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    // This call needs to be authorized by `from` address. It transfers the
    // maximum spend amount to the swap contract's address in order to decouple
    // the signature from `to` address (so that parties don't need to know each
    // other).
    token.transfer(from, &contract_address, &max_spend_amount);
    // Transfer the necessary amount to `to`.
    token.transfer(&contract_address, to, &transfer_amount);
    // Refund the remaining balance to `from`.
    token.transfer(
        &contract_address,
        from,
        &(max_spend_amount - transfer_amount),
    );
}

mod test;
